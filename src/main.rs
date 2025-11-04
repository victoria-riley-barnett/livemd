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
struct Cli {
    #[arg(short, long, help = "Markdown file to stream")]
    file: Option<PathBuf>,

    #[arg(short, long, help = "Command to run and stream")]
    cmd: Option<String>,

    #[arg(short, long, help = "Run configured LLM command with this query and stream output")]
    query: Option<String>,

    #[arg(long, default_value = "0.001", help = "Delay between chunks in seconds (smaller = faster)")]
    speed: f64,

    #[arg(long, default_value = "150", help = "Max chunk size before flush")]
    chunk_size: usize,

    #[arg(long, help = "Convert simple boxed headings into Markdown headers")]
    strip_boxes: bool,

    #[arg(long, default_value = "aichat", help = "Command to invoke the LLM")]
    llm_cmd: String,

    #[arg(long, default_value = "dark", help = "Color theme: dark, light, mono")]
    theme: String,

    #[arg(long, help = "Path to custom theme JSON file")]
    theme_file: Option<PathBuf>,

    #[arg(long, help = "Do not inject the default 'respond only in Markdown' instruction")]
    no_inject: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Load configuration file if it exists
    let config_file = ConfigFile::load();

    // Apply defaults from config file, CLI args take precedence
    let theme_name = cli.theme;
    let theme_file = cli.theme_file.or_else(|| {
        config_file.as_ref().and_then(|c| c.theme_file.as_ref()).map(|tf| {
            let project_dirs = directories::ProjectDirs::from("com", "livemd", "livemd").unwrap();
            project_dirs.config_dir().join(tf)
        })
    });

    let config = StreamerConfig {
        chunk_size: cli.chunk_size,
        speed: cli.speed,
        strip_boxes: cli.strip_boxes,
        llm_cmd: cli.llm_cmd,
        inject_md_instruction: !cli.no_inject,
        theme_name,
        theme_file,
    };

    let streamer = MinimalStreamer::new(config);


    let result: Result<(), Box<dyn std::error::Error>> = async {
        if let Some(file_path) = cli.file {
            streamer.stream_file(file_path).await?;
        } else if let Some(cmd) = cli.cmd {
            streamer.stream_command(&cmd).await?;
        } else if let Some(query) = cli.query {
            streamer.stream_query(&query).await?;
        } else if !is(Stream::Stdin) {
            streamer.stream_stdin().await?;
        } else {
            eprintln!("Error: Must specify one of --file, --cmd, or --query, or pipe input to stdin");
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