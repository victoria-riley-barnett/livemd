//! Core streaming functionality

use crate::config::StreamerConfig;
use crate::table::TableRenderer;
use crate::theme::Theme;
use pulldown_cmark::{Parser as MarkdownParser, Options, Event, Tag, TagEnd, CodeBlockKind};
use regex::Regex;
use std::io::{Read, BufReader, Write, stdout};
use std::path::PathBuf;
use std::process::Stdio;
use termimad::crossterm::{
    style::{Print, ResetColor, SetForegroundColor, SetAttribute, Attribute},
    terminal::size,
    QueueableCommand,
};
use termimad::MadSkin;
use tokio::time::{sleep, Duration};

/// Core Markdown streaming implementation
pub struct MinimalStreamer {
    config: StreamerConfig,
    theme: Theme,
    mad_skin: MadSkin,
}

impl MinimalStreamer {
    /// Create a new streamer instance
    pub fn new(config: StreamerConfig) -> Self {
        let theme = if let Some(ref theme_file) = config.theme_file {
            match Theme::from_file(theme_file) {
                Ok(theme) => theme,
                Err(e) => {
                    eprintln!("Warning: Failed to load theme from {:?}: {}", theme_file, e);
                    eprintln!("Falling back to built-in theme: {}", config.theme_name);
                    match config.theme_name.as_str() {
                        "light" => Theme::light(),
                        "mono" => Theme::mono(),
                        _ => Theme::dark(),
                    }
                }
            }
        } else {
            match config.theme_name.as_str() {
                "light" => Theme::light(),
                "mono" => Theme::mono(),
                _ => Theme::dark(),
            }
        };

        // Create termimad skin for rich text rendering
        let mut mad_skin = MadSkin::default();
        mad_skin.set_fg(termimad::crossterm::style::Color::AnsiValue(15)); // White text
        mad_skin.set_bg(termimad::crossterm::style::Color::AnsiValue(0));  // Black background

        // Left-align paragraphs and headers
        mad_skin.paragraph.align = termimad::Alignment::Left;
        mad_skin.paragraph.set_bg(termimad::crossterm::style::Color::Reset);

        for header in &mut mad_skin.headers {
            header.align = termimad::Alignment::Left;
            header.set_bg(termimad::crossterm::style::Color::Reset); // No background
        }

        // Remove backgrounds from other elements
        mad_skin.bold.set_bg(termimad::crossterm::style::Color::Reset);
        mad_skin.italic.set_bg(termimad::crossterm::style::Color::Reset);
        mad_skin.strikeout.set_bg(termimad::crossterm::style::Color::Reset);
        mad_skin.inline_code.set_bg(termimad::crossterm::style::Color::Reset);

        // Configure header colors (termimad handles the sizing automatically)
        // Set header colors from theme
        for (i, header) in mad_skin.headers.iter_mut().enumerate() {
            let color = theme.get_heading_color(i + 1); // 1-indexed levels
            header.set_fg(color);
        }

        Self { config, theme, mad_skin }
    }

    /// Render math expressions with special formatting
    fn render_math(&self, math_text: &str) -> String {
        // For now, just return the math with special markers
        // Could render LaTeX to ASCII art or similar?
        format!("[Math: {}]", math_text.trim())
    }

    /// Process text for math expressions before markdown parsing
    fn preprocess_math(&self, text: &str) -> String {
        let math_re = Regex::new(r"\$\$([^$]+)\$\$").unwrap();
        math_re.replace_all(text, |caps: &regex::Captures| {
            let math_content = &caps[1];
            self.render_math(math_content)
        }).to_string()
    }

    /// Find the optimal boundary for flushing content during streaming
    /// Prioritizes code fences, table boundaries, then paragraph boundaries, then size thresholds
    fn find_flush_boundary(&self, buffer: &str) -> usize {
        // 1. Prioritize code fences
        if let Some(mat) = Regex::new(r"```").unwrap().find_iter(buffer).nth(1) {
            let mut flush_at = mat.start() + 3;
            // include following newline if present
            if flush_at < buffer.len() && buffer.chars().nth(flush_at) == Some('\n') {
                flush_at += 1;
            }
            return flush_at;
        }
        // 2. Don't break inside table rows
        if let Some(table_row_start) = buffer.find("|") {
            // Look for the end of the current table row
            if let Some(row_end) = buffer[table_row_start..].find('\n') {
                let potential_flush = table_row_start + row_end + 1;
                if potential_flush < buffer.len() && buffer.chars().nth(potential_flush) != Some('|') {
                    // Not in the middle of a table, safe to flush after this row
                    return potential_flush;
                }
            }
        }
        // 3. Paragraph boundaries - preserve consecutive newlines
        if let Some(idx) = buffer.find("\n\n") {
            let mut flush_at = idx + 2;
            // find the end of consecutive newlines
            while flush_at < buffer.len() && buffer.chars().nth(flush_at) == Some('\n') {
                flush_at += 1;
            }
            return flush_at;
        }
        // 4. Size threshold - prefer sentence boundaries over word boundaries
        if buffer.len() >= self.config.chunk_size {
            // First, try to find a sentence boundary (period + space)
            if let Some(sentence_end) = buffer[..self.config.chunk_size].rfind(". ") {
                return sentence_end + 2; // Include period and space
            }
            // Then try to find a sentence boundary with other punctuation
            if let Some(sentence_end) = buffer[..self.config.chunk_size].rfind(|c: char| ".!?:".contains(c)) {
                if sentence_end + 1 < buffer.len() && buffer.chars().nth(sentence_end + 1).unwrap_or(' ').is_whitespace() {
                    return sentence_end + 2; // Include punctuation and following whitespace
                }
            }
            // Try to find a comma boundary
            if let Some(comma_end) = buffer[..self.config.chunk_size].rfind(", ") {
                return comma_end + 2; // Include comma and space
            }
            // Try to find a dash boundary
            if let Some(dash_end) = buffer[..self.config.chunk_size].rfind(" - ") {
                return dash_end + 3; // Include dash and spaces
            }
            // Fall back to word boundary, but prefer larger chunks
            if let Some(last_space) = buffer[..self.config.chunk_size].rfind(|c: char| c.is_whitespace()) {
                // Only break if we're at least 75% through the chunk to avoid tiny fragments
                if last_space > self.config.chunk_size * 3 / 4 {
                    let mut flush_at = last_space + 1;
                    // Skip any trailing newlines to avoid double newlines
                    while flush_at < buffer.len() && buffer.chars().nth(flush_at) == Some('\n') {
                        flush_at += 1;
                    }
                    return flush_at;
                }
            }
            // No good boundary found, flush at chunk_size
            return self.config.chunk_size;
        }
        0
    }

    /// Parse and render Markdown with terminal styling using Crossterm
    fn print_styled_markdown(&self, text: &str) {
        let mut stdout = stdout();
        // Preprocess math expressions
        let processed_text = self.preprocess_math(text);
        let parser = MarkdownParser::new_ext(&processed_text, Options::all());
        let mut list_depth = 0;
        let mut table_buffer = String::new();
        let mut in_table = false;
        let mut table_cell_count = 0;
        let mut header_buffer = String::new();
        let mut in_header = false;
        let mut list_buffer = String::new();
        let mut in_list = false;
        let mut list_indent_level = 0;
        let mut list_types: Vec<Option<u64>> = Vec::new();
        let mut item_numbers: Vec<usize> = Vec::new();
        let mut code_block_buffer = String::new();
        let mut in_code_block = false;
        let mut in_paragraph = false;

        for event in parser {
            if in_table {
                match event {
                    Event::End(TagEnd::Table) => {
                        in_table = false;
                        // Remove trailing separator and render table with borders
                        let table_md = table_buffer.trim_end_matches(" | ").trim_end_matches("| ");
                        if !table_md.is_empty() {
                            TableRenderer::render_table(table_md);
                        }
                        table_buffer.clear();
                    }
                    Event::Start(Tag::TableHead) | Event::Start(Tag::TableRow) => {
                        table_cell_count = 0;
                    }
                    Event::End(TagEnd::TableHead) | Event::End(TagEnd::TableRow) => {
                        if table_cell_count > 0 {
                            table_buffer.push('\n');
                        }
                    }
                    Event::Start(Tag::TableCell) => {
                        if table_cell_count > 0 {
                            table_buffer.push_str(" | ");
                        }
                        table_cell_count += 1;
                    }
                    Event::Text(text) => {
                        table_buffer.push_str(&text);
                    }
                    _ => {}
                }
            } else {
                match event {
                    Event::Start(Tag::Table(_)) => {
                        // Flush any pending header or list before starting table
                        if in_header && !header_buffer.is_empty() {
                            let _ = self.mad_skin.print_text(&header_buffer);
                            header_buffer.clear();
                            in_header = false;
                        }
                        if in_list && !list_buffer.is_empty() {
                            let _ = self.mad_skin.print_text(&list_buffer);
                            list_buffer.clear();
                            in_list = false;
                        }
                        in_table = true;
                        table_buffer.clear();
                        table_cell_count = 0;
                    }
                    Event::Start(Tag::Heading { level, .. }) => {
                        // Flush any pending content
                        if in_list && !list_buffer.is_empty() {
                            let _ = self.mad_skin.print_text(&list_buffer);
                            list_buffer.clear();
                            in_list = false;
                        }
                        in_header = true;
                        header_buffer.clear();
                        // Add markdown header prefix
                        header_buffer.push_str(&"#".repeat(level as usize));
                        header_buffer.push(' ');
                    }
                    Event::End(TagEnd::Heading(_)) => {
                        if in_header {
                            let _ = self.mad_skin.print_text(&header_buffer);
                            header_buffer.clear();
                            in_header = false;
                        }
                    }
                    Event::Start(Tag::List(list_type)) => {
                        if !in_list {
                            // Flush any pending header
                            if in_header && !header_buffer.is_empty() {
                                let _ = self.mad_skin.print_text(&header_buffer);
                                header_buffer.clear();
                                in_header = false;
                            }
                            in_list = true;
                            list_buffer.clear();
                            list_depth = 0;
                            list_types.clear();
                            item_numbers.clear();
                        }
                        list_depth += 1;
                        list_types.push(list_type);
                        item_numbers.push(0);
                        if list_depth > 1 {
                            list_buffer.push('\n');
                        }
                        list_indent_level = list_depth - 1;
                    }
                    Event::End(TagEnd::List(_)) => {
                        list_depth -= 1;
                        list_types.pop();
                        item_numbers.pop();
                        if list_depth == 0 && in_list {
                            let _ = self.mad_skin.print_text(&list_buffer);
                            list_buffer.clear();
                            in_list = false;
                        } else if list_depth > 0 {
                            list_indent_level = list_depth - 1;
                        }
                    }
                    Event::Start(Tag::Item) => {
                        if in_list {
                            let indent_len = 2 * list_indent_level;
                            let indent = " ".repeat(indent_len.min(3));
                            list_buffer.push_str(&indent);
                            let level = list_depth - 1;
                            let item_num = item_numbers[level];
                            item_numbers[level] = item_num + 1;
                            if let Some(start) = list_types[level] {
                                // ordered list
                                list_buffer.push_str(&format!("{}. ", start + item_num as u64));
                            } else {
                                // unordered list
                                list_buffer.push_str("- ");
                            }
                        }
                    }
                    Event::End(TagEnd::Item) => {
                        if in_list {
                            list_buffer.push('\n');
                        }
                    }
                    Event::Start(Tag::CodeBlock(kind)) => {
                        // Flush any pending content before code block
                        if in_header && !header_buffer.is_empty() {
                            let _ = self.mad_skin.print_text(&header_buffer);
                            header_buffer.clear();
                            in_header = false;
                        }
                        if in_list && !list_buffer.is_empty() {
                            let _ = self.mad_skin.print_text(&list_buffer);
                            list_buffer.clear();
                            in_list = false;
                        }
                        in_code_block = true;
                        code_block_buffer.clear();
                        // Add opening backticks and language
                        code_block_buffer.push_str("```");
                        match kind {
                            CodeBlockKind::Fenced(lang) => {
                                if !lang.is_empty() {
                                    code_block_buffer.push_str(&lang);
                                }
                            }
                            _ => {}
                        }
                        code_block_buffer.push('\n');
                    }
                    Event::End(TagEnd::CodeBlock) => {
                        code_block_buffer.push_str("\n```");
                        let _ = self.mad_skin.print_text(&code_block_buffer);
                        code_block_buffer.clear();
                        in_code_block = false;
                    }
                    Event::Start(Tag::Emphasis) => {
                        if in_header {
                            header_buffer.push_str("*");
                        } else if in_list {
                            list_buffer.push_str("*");
                        } else {
                            let _ = stdout.queue(SetAttribute(Attribute::Italic));
                            let _ = stdout.queue(SetForegroundColor(self.theme.get_color("italic")));
                        }
                    }
                    Event::End(TagEnd::Emphasis) => {
                        if in_header {
                            header_buffer.push_str("*");
                        } else if in_list {
                            list_buffer.push_str("*");
                        } else {
                            let _ = stdout.queue(ResetColor);
                        }
                    }
                    Event::Start(Tag::Strong) => {
                        if in_header {
                            header_buffer.push_str("**");
                        } else if in_list {
                            list_buffer.push_str("**");
                        } else {
                            let _ = stdout.queue(SetAttribute(Attribute::Bold));
                            let _ = stdout.queue(SetForegroundColor(self.theme.get_color("bold")));
                        }
                    }
                    Event::End(TagEnd::Strong) => {
                        if in_header {
                            header_buffer.push_str("**");
                        } else if in_list {
                            list_buffer.push_str("**");
                        } else {
                            let _ = stdout.queue(ResetColor);
                        }
                    }
                    Event::Text(text) => {
                        if in_header {
                            header_buffer.push_str(&text);
                        } else if in_list {
                            list_buffer.push_str(&text);
                        } else if in_code_block {
                            code_block_buffer.push_str(&text);
                        } else {
                            let _ = stdout.queue(Print(text));
                        }
                    }
                    Event::SoftBreak => {
                        if in_list {
                            // For lists, soft breaks should create new lines
                            list_buffer.push('\n');
                        } else {
                            let _ = stdout.queue(Print("\n"));
                        }
                    }
                    Event::HardBreak => {
                        if in_list {
                            list_buffer.push_str("\n\n");
                        } else {
                            let _ = stdout.queue(Print("\n\n"));
                        }
                    }
                    Event::Rule => {
                        // Flush any pending content before rule
                        if in_header && !header_buffer.is_empty() {
                            let _ = self.mad_skin.print_text(&header_buffer);
                            header_buffer.clear();
                            in_header = false;
                        }
                        if in_list && !list_buffer.is_empty() {
                            let _ = self.mad_skin.print_text(&list_buffer);
                            list_buffer.clear();
                            in_list = false;
                        }
                        if let Ok((width, _)) = size() {
                            let rule = "─".repeat(width as usize);
                            let _ = stdout.queue(Print(format!("\n{}\n", rule)));
                        } else {
                            let _ = stdout.queue(Print("\n─────────────────────────────────────────────────────────────────────────────────────────────────────\n"));
                        }
                    }
                    Event::Start(Tag::BlockQuote(_)) => {
                        if in_list {
                            list_buffer.push_str("> ");
                        } else {
                            let _ = stdout.queue(SetForegroundColor(self.theme.get_color("italic")));
                            let _ = stdout.queue(Print("│ "));
                        }
                    }
                    Event::End(TagEnd::BlockQuote(_)) => {
                        if in_list {
                            list_buffer.push('\n');
                        } else {
                            let _ = stdout.queue(ResetColor);
                            let _ = stdout.queue(Print("\n"));
                        }
                    }
                    Event::Start(Tag::Paragraph) => {
                        in_paragraph = true;
                    }
                    Event::End(TagEnd::Paragraph) => {
                        if in_list {
                            // In lists, paragraphs are handled differently
                        } else if in_paragraph {
                            // Only add paragraph spacing if we actually had paragraph content
                            let _ = stdout.queue(Print("\n\n"));
                        }
                        in_paragraph = false;
                    }
                    _ => {}
                }
            }
        }
        let _ = stdout.flush();
    }

    fn strip_ansi(&self, text: &str) -> String {
        let ansi_re = Regex::new(r"\x1B\[[0-?]*[ -/]*[@-~]").unwrap();
        ansi_re.replace_all(text, "").to_string()
    }

    fn sanitize_boxes(&self, text: &str) -> String {
        let lines: Vec<&str> = text.lines().collect();
        let mut out_lines = Vec::new();
        let mut i = 0;

        while i < lines.len() {
            if i + 2 < lines.len() {
                let a = lines[i];
                let b = lines[i + 1];
                let c = lines[i + 2];

                // Check for box pattern
                if (a.contains('┏') || a.contains('┌') || a.contains('╔') || a.chars().any(|ch| "━──═-+".contains(ch))) &&
                   (b.contains('┃') || b.contains('│') || b.contains('|')) &&
                   (c.contains('┗') || c.contains('└') || c.contains('╚') || c.chars().any(|ch| "━─═-+".contains(ch))) {

                    // Extract inner text from middle line
                    let inner = b
                        .replace('┃', "")
                        .replace('│', "")
                        .replace('|', "")
                        .trim()
                        .to_string();

                    if !inner.is_empty() {
                        out_lines.push("".to_string());
                        out_lines.push(format!("### {}", inner));
                        out_lines.push("".to_string());
                    }
                    i += 3;
                    continue;
                }
            }

            // Remove box drawing characters from regular lines
            let clean_line = lines[i]
                .chars()
                .filter(|&ch| !"┏┓┗┛┃━─┌┐└┘│╔╗╚╝═".contains(ch))
                .collect::<String>();
            out_lines.push(clean_line);
            i += 1;
        }

        out_lines.join("\n")
    }

    /// Stream text content
    pub async fn stream_text(&self, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut pos = 0;
        let mut buffer = String::new();
        let step = 240; // Increased chunk size for better throughput
        let text_bytes = text.as_bytes();

        while pos < text_bytes.len() {
            let end = std::cmp::min(pos + step, text_bytes.len());
            let chunk = std::str::from_utf8(&text_bytes[pos..end]).unwrap_or("");
            pos = end;

            buffer.push_str(chunk);
            buffer = self.strip_ansi(&buffer);

            if self.config.strip_boxes {
                buffer = self.sanitize_boxes(&buffer);
            }

            let mut flush_pos;
            let mut chunks_processed = 0;
            while {
                flush_pos = self.find_flush_boundary(&buffer);
                flush_pos > 0
            } {
                let to_print = buffer.drain(..flush_pos).collect::<String>();
                self.print_styled_markdown(&to_print);
                chunks_processed += 1;

                // Only sleep after processing a few chunks to reduce latency
                if chunks_processed % 5 == 0 {
                    sleep(Duration::from_secs_f64(self.config.speed)).await;
                }
            }
        }

        if !buffer.trim().is_empty() {
            self.print_styled_markdown(&buffer);
        }
        Ok(())
    }

    /// Stream content from a file
    pub async fn stream_file(&self, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = std::fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        self.stream_text(&contents).await?;
        Ok(())
    }

    /// Stream output from a command
    pub async fn stream_command(&self, cmd: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut child = std::process::Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .stdout(Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take().expect("Failed to capture stdout.");
        let mut reader = BufReader::new(stdout);
        let mut buffer = String::new();

        loop {
            let mut chunk = vec![0; 4096]; // Increased buffer size for better throughput
            match reader.read(&mut chunk) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    let chunk_str = String::from_utf8_lossy(&chunk[..n]);
                    buffer.push_str(&chunk_str);
                    buffer = self.strip_ansi(&buffer);

                    if self.config.strip_boxes {
                        buffer = self.sanitize_boxes(&buffer);
                    }

                    let mut flush_pos;
                    let mut chunks_processed = 0;
                    while {
                        flush_pos = self.find_flush_boundary(&buffer);
                        flush_pos > 0
                    } {
                        let to_print = buffer.drain(..flush_pos).collect::<String>();
                        self.print_styled_markdown(&to_print);
                        chunks_processed += 1;

                        // Only sleep after processing a few chunks to reduce latency
                        if chunks_processed % 3 == 0 {
                            sleep(Duration::from_secs_f64(self.config.speed)).await;
                        }
                    }
                }
                Err(e) => return Err(e.into()),
            }
        }

        if !buffer.trim().is_empty() {
            self.print_styled_markdown(&buffer);
        }
        Ok(())
    }

    /// Stream output from an LLM query
    pub async fn stream_query(&self, query: &str) -> Result<(), Box<dyn std::error::Error>> {
        let llm_cmd = self.config.llm_cmd.as_ref().ok_or("Error: --llm_cmd is required when using --query. Set it with --llm_cmd 'your-ai-tool'")?;

        let mut query_str = query.to_string();
        if self.config.inject_md_instruction {
            query_str = format!("Please respond only in Markdown.\n{}", query);
        }

        // Build the full command string with query
        let full_cmd = format!("{} {}", llm_cmd, query_str);

        let mut child = std::process::Command::new("sh")
            .arg("-c")
            .arg(&full_cmd)
            .stdout(Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take().expect("Failed to capture stdout.");
        let mut reader = BufReader::new(stdout);
        let mut buffer = String::new();

        loop {
            let mut chunk = vec![0; 4096]; // Increased buffer size for better throughput
            match reader.read(&mut chunk) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    let chunk_str = String::from_utf8_lossy(&chunk[..n]);
                    buffer.push_str(&chunk_str);
                    buffer = self.strip_ansi(&buffer);

                    if self.config.strip_boxes {
                        buffer = self.sanitize_boxes(&buffer);
                    }

                    let mut flush_pos;
                    let mut chunks_processed = 0;
                    while {
                        flush_pos = self.find_flush_boundary(&buffer);
                        flush_pos > 0
                    } {
                        let to_print = buffer.drain(..flush_pos).collect::<String>();
                        self.print_styled_markdown(&to_print);
                        chunks_processed += 1;

                        // Only sleep after processing a few chunks to reduce latency
                        if chunks_processed % 3 == 0 {
                            sleep(Duration::from_secs_f64(self.config.speed)).await;
                        }
                    }
                }
                Err(e) => return Err(e.into()),
            }
        }

        if !buffer.trim().is_empty() {
            self.print_styled_markdown(&buffer);
        }
        Ok(())
    }

    /// Stream content from stdin
    pub async fn stream_stdin(&self) -> Result<(), Box<dyn std::error::Error>> {
        use tokio::io::{AsyncReadExt, stdin};
        let mut reader = stdin();
        let mut buffer = String::new();
        let mut chunk = vec![0; 4096];

        loop {
            match reader.read(&mut chunk).await {
                Ok(0) => break, // EOF
                Ok(n) => {
                    let chunk_str = String::from_utf8_lossy(&chunk[..n]);
                    buffer.push_str(&chunk_str);

                    if self.config.strip_boxes {
                        buffer = self.sanitize_boxes(&buffer);
                    }

                    let mut flush_pos;
                    let mut chunks_processed = 0;
                    while {
                        flush_pos = self.find_flush_boundary(&buffer);
                        flush_pos > 0
                    } {
                        let to_print = buffer.drain(..flush_pos).collect::<String>();
                        self.print_styled_markdown(&to_print);
                        chunks_processed += 1;

                        // Only sleep after processing a few chunks to reduce latency
                        if chunks_processed % 3 == 0 {
                            sleep(Duration::from_secs_f64(self.config.speed)).await;
                        }
                    }
                }
                Err(e) => return Err(e.into()),
            }
        }

        if !buffer.trim().is_empty() {
            self.print_styled_markdown(&buffer);
        }
        Ok(())
    }
}