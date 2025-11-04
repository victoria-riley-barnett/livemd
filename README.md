# livemd

A Markdown streaming tool for terminals. Streams Markdown content as it's generated, with basic formatting powered by [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) and [termimad](https://github.com/Canop/termimad).

## Features

If you use a terminal AI tool, you're probably reading a lot of raw Markdown. 
livemd streams Markdown content as it's generated, rendering it (kind of) nicely in your terminal.

- **AI Queries**: Stream responses from LLM tools
- **Multi-word queries**: No quotes needed - `livemd explain rust ownership`
- **Terminal formatting**: Headers, lists, code blocks, tables via pulldown-cmark, termimad, and Crossterm
- **Individual colors for each header level (H1-H6)** with full hex color support for customization
- **Command presets**: Configure different commands for different use cases

## Important Notes

⚠️ **This is a tool with limitations:**
- Not all Markdown features are supported, especially extended syntax
- Compatibility has not been extensively tested.
- Streaming may not work perfectly with all content, especially complex layouts: formatting may break, might flush imperfectly if it's not seeing the right boundaries.

## Supported Markdown Elements

Support for:
- ### Headers (with color coding)
- **Bold** and *italic* text
- `Inline code` and
```c
printf("code blocks");
```
- Bulleted and numbered lists
  - Sometimes, nested lists :)
- Tables
- Links and blockquotes

## Installation

### Option 1: Pre-built Binaries (Recommended)

Download pre-built binaries for your platform from the [GitHub Releases](https://github.com/victoria-riley-barnett/livemd/releases) page.

**Automatic Installation:**
```bash
curl -fsSL https://raw.githubusercontent.com/victoria-riley-barnett/livemd/main/install.sh | bash
```
*By default, installs to `~/.local/bin`. To install to a different directory, set `INSTALL_DIR`:*
```bash
INSTALL_DIR=/usr/local/bin curl -fsSL https://raw.githubusercontent.com/victoria-riley-barnett/livemd/main/install.sh | bash
```
*Note: Ensure `~/.local/bin` (or your chosen directory) is in your `PATH`. Add to your shell config if needed: `export PATH="$HOME/.local/bin:$PATH"`*

**Manual Installation:**
```bash
# Download the appropriate binary for your platform
# macOS Intel: livemd-macos-x64.tar.gz
# macOS Apple Silicon: livemd-macos-arm64.tar.gz
# Linux x64: livemd-linux-x64.tar.gz
# Linux ARM64: livemd-linux-arm64.tar.gz
# Windows: livemd-windows-x64.zip

# Extract and install
tar xzf livemd-*.tar.gz  # or unzip on Windows
mv livemd ~/.local/bin/  # or sudo mv livemd /usr/local/bin/
```

### Option 2: Homebrew
(for later, todo: PR homebrew)

### Option 3: Cargo Install (Rust)

```bash
cargo install --git https://github.com/victoria-riley-barnett/livemd.git
```

### Option 4: Build from Source (Rust)

```bash
git clone https://github.com/victoria-riley-barnett/livemd.git
cd livemd
cargo build --release
sudo cp target/release/livemd /usr/local/bin/
```

## AI Tool Setup

`livemd --query` requires an AI/LLM tool to process queries. You must specify the command using `--llm-cmd` or by defining it in the config file.

### Example with aichat

1. Install [aichat](https://github.com/sigoden/aichat) & go through its setup instructions/API key configuration. Same deal for any other AI tool you choose.

2. Use with livemd:
   ```bash
   livemd explain rust ownership
   # or
   livemd "explain rust ownership"
   ```

   **Note:** If your query contains shell glob characters (`?`, `*`, `[`, `]`), you may need to quote the query or use `noglob`:
   ```bash
   livemd "what is gnosticism?"  # Use quotes
   noglob livemd what is gnosticism?  # Or disable globbing
   ```

For convenience, add this function to your `~/.zshrc` for easy queries with punctuation:
```bash
ai() {
    noglob livemd "$@"
}
```

You can configure multiple commands in your config file for different modes:

```json
{
  "llm-cmd": {
    "default": "aichat",
    "dev": "aichat -s dev --save-session",
    "fast": "aichat --model gpt-4o-mini"
  }
}
```

Then use: `livemd "query" --llm-cmd dev`

## Usage

```bash
# Query AI and stream response (default mode)
livemd explain rust ownership system

# Or use quotes if you prefer
livemd "explain rust ownership system"

# Stream a Markdown file
livemd --file README.md

# Stream command output
livemd --cmd "cat large_file.md"

# Stream from stdin
livemd --stdin < file.md
cat file.md | livemd
```

## Options

Powered by [clap](https://github.com/clap-rs/clap) for command-line parsing:

- `QUERY` (positional, multiple): AI query to process and stream (default mode)
- `--file <PATH>`: Markdown file to stream
- `--cmd <COMMAND>`: Shell command to run and stream output
- `--speed <SECONDS>`: Delay between chunks (default: 0.005)
- `--chunk-size <BYTES>`: Max chunk size before flush (default: 3200)
- `--strip-boxes`: Convert ASCII boxes to Markdown headers
- `--llm-cmd <CMD>`: AI tool to invoke (supports complex commands like `aichat -s dev --save-session`)
- `--theme <THEME>`: Color theme (dark/light/mono, default: dark)
- `--theme-file <PATH>`: Path to custom theme JSON file
- `--stdin`: Force reading from stdin
- `--no-inject`: Skip Markdown instruction injection

See [CONFIG.md](CONFIG.md) for detailed configuration options and setup.

## Contributing

Feel free to submit issues or PRs for improvements.
