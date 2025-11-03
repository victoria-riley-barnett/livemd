# livemd

A Markdown streaming tool for terminals. Streams Markdown content as it's generated, with basic formatting powered by [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) and [termimad](https://github.com/Canop/termimad).

## Features

If you use a terminal AI tool, you're probably reading a lot of raw Markdown. 
livemd streams Markdown content as it's generated, rendering it (kind of) nicely in your terminal.
- Streams Markdown from AI queries (it can do files and commands too, but it's not as pretty as something like [Glow](https://github.com/charmbracelet/glow))
- Basic terminal formatting for headers, lists, and code blocks via pulldown-cmark, termimad, and Crossterm.
- **Individual colors for each header level (H1-H6)** with full hex color support for customization
- Configurable streaming speeds and chunk sizes

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
sudo mv livemd /usr/local/bin/
```

### Option 2: Homebrew (macOS)

```bash
brew tap victoria-riley-barnett/livemd
brew install livemd
```

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

## Usage

```bash
# Stream a Markdown file
livemd --file README.md

# Stream command output
livemd --cmd "cat large_file.md"

# Query AI and stream response
livemd --query "Explain Rust's ownership system"

# Stream with custom speed and theme
livemd --file document.md --speed 0.01 --theme light
```

## Options

Powered by [clap](https://github.com/clap-rs/clap) for command-line parsing:

- `--file <PATH>`: Markdown file to stream
- `--cmd <COMMAND>`: Shell command to run and stream output
- `--query <TEXT>`: AI query to process and stream
- `--speed <SECONDS>`: Delay between chunks (default: 0.005)
- `--chunk-size <BYTES>`: Max chunk size before flush (default: 3200)
- `--strip-boxes`: Convert ASCII boxes to Markdown headers
- `--aichat-cmd <CMD>`: AI command to use (default: aichat)
- `--theme <THEME>`: Color theme (dark/light/mono, default: dark)
- `--theme-file <PATH>`: Path to custom theme JSON file
- `--no-inject`: Skip Markdown instruction injection

See [CONFIG.md](CONFIG.md) for detailed configuration options and setup.

## Themes

livemd supports color themes with full hex color support for complete customization:

- **`dark`** (default): For dark terminal backgrounds
- **`light`**: For light terminal backgrounds  
- **`mono`**: Monochrome theme

### Individual Header Colors

You can specify different colors for each header level (H1-H6):

```json
{
  "heading": [
    "#ff6b6b",  // H1 - Red
    "#4ecdc4",  // H2 - Teal
    "#ffd93d",  // H3 - Yellow
    "#6bcf7f",  // H4 - Green
    "#4d96ff",  // H5 - Blue
    "#f368e0"   // H6 - Pink
  ],
  "code": "#4ecdc4",
  "bold": "#ffd93d",
  "italic": "#6bcf7f",
  "link": "#4d96ff",
  "list": "#f368e0"
}
```

**Note:** For single color headers, use `"heading": "#ff6b6b"` instead of an array.

See [THEMES.md](THEMES.md) for detailed theming documentation, including example themes and color format reference.


## Contributing

Feel free to submit issues or PRs for improvements.