//! # livemd
//!
//! A Markdown streaming tool for terminals.
//!
//! This tool streams Markdown content with basic formatting, supporting files,
//! command output, and AI chat responses.

use clap::Parser;
use std::path::PathBuf;

mod config;
mod streamer;
mod table;
mod theme;

use config::{ConfigFile, StreamerConfig};
use streamer::MinimalStreamer;
use atty::{is, Stream};

#[derive(Parser)]
#[command(name = "livemd")]
#[command(about = "Live Markdown streaming tool", version, author)]
#[command(after_help = "EXAMPLES:
  livemd explain rust ownership    # Query AI (no quotes needed!)
  livemd --file README.md          # Stream a markdown file  
  livemd --cmd 'ls -la'            # Stream command output
  livemd --stdin < file.md         # Stream from stdin
  cat file.md | livemd             # Pipe content to livemd

NOTE: For queries with shell glob characters (?, *, [, ]), use quotes:
  livemd \"what is gnosticism?\"   # With quotes
  noglob livemd what is gnosticism?  # Or disable globbing (zsh)

For zsh users, add this to ~/.zshrc: ai() { noglob livemd \"$@\" }

CONFIG: ~/.config/livemd/config.json
THEMES: ~/.config/livemd/themes/")]
struct Cli {
    #[arg(trailing_var_arg = true, help = "Query to run with configured LLM command (default mode)")]
    query: Vec<String>,

    #[arg(short, long, help = "Markdown file to stream")]
    file: Option<PathBuf>,

    #[arg(short, long, help = "Command to run and stream")]
    cmd: Option<String>,

    #[arg(long, help = "Delay between chunks in seconds (smaller = faster)")]
    speed: Option<f64>,

    #[arg(long, help = "Max chunk size before flush")]
    chunk_size: Option<usize>,

    #[arg(long, help = "Convert simple boxed headings into Markdown headers")]
    strip_boxes: bool,

    #[arg(long, help = "Command to invoke the LLM")]
    llm_cmd: Option<String>,

    #[arg(long, help = "Color theme: dark, light, mono")]
    theme: Option<String>,

    #[arg(long, help = "Path to custom theme JSON file")]
    theme_file: Option<PathBuf>,

    #[arg(long, help = "Force reading from stdin (overrides other modes)")]
    stdin: bool,

    #[arg(long, help = "Do not inject the default 'respond only in Markdown' instruction")]
    no_inject: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Load configuration file if it exists
    let config_file = ConfigFile::load();

    // Apply defaults from config file, CLI args take precedence
    let theme_name = cli.theme.or_else(|| config_file.as_ref().and_then(|c| c.theme.as_ref()).cloned()).unwrap_or_else(|| "dark".to_string());
    let theme_file = cli.theme_file.or_else(|| {
        config_file.as_ref().and_then(|c| c.theme_file.as_ref()).map(|tf| {
            dirs::home_dir()
                .map(|h| h.join(".config").join("livemd").join(tf))
                .unwrap_or_default()
        })
    }).or_else(|| {
        // Check for default theme file
        let default_theme = dirs::home_dir()
            .map(|h| h.join(".config").join("livemd").join("themes").join("default.json"))
            .unwrap_or_default();
        if default_theme.exists() {
            Some(default_theme)
        } else {
            None
        }
    });
    let speed = cli.speed.or_else(|| config_file.as_ref().and_then(|c| c.speed)).unwrap_or(0.001);
    let chunk_size = cli.chunk_size.or_else(|| config_file.as_ref().and_then(|c| c.chunk_size)).unwrap_or(150);
    let strip_boxes = cli.strip_boxes || config_file.as_ref().and_then(|c| c.strip_boxes).unwrap_or(false);
    let llm_cmd = config_file.as_ref().and_then(|c| c.resolve_llm_cmd(cli.llm_cmd.as_deref())).or_else(|| cli.llm_cmd);
    let inject_md_instruction = !cli.no_inject && config_file.as_ref().and_then(|c| c.inject_md_instruction).unwrap_or(true);

    let config = StreamerConfig {
        chunk_size,
        speed,
        strip_boxes,
        llm_cmd,
        inject_md_instruction,
        theme_name,
        theme_file,
    };

    let streamer = MinimalStreamer::new(config);


    let result: Result<(), Box<dyn std::error::Error>> = async {
        if cli.stdin {
            // Explicit --stdin flag
            streamer.stream_stdin().await?;
        } else if let Some(file_path) = cli.file {
            streamer.stream_file(file_path).await?;
        } else if let Some(cmd) = cli.cmd {
            streamer.stream_command(&cmd).await?;
        } else if !cli.query.is_empty() {
            let query = cli.query.join(" ");
            streamer.stream_query(&query).await?;
        } else if !is(Stream::Stdin) {
            // If no other mode specified and stdin is available (piped)
            streamer.stream_stdin().await?;
        } else {
            eprintln!("Error: Must specify a query, --file, --cmd, --stdin, or pipe input to stdin");
            std::process::exit(1);
        }
        Ok(())
    }.await;

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}