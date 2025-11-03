# livemd Configuration

Configuration options and setup for livemd.

## Configuration File

livemd supports a JSON configuration file for default settings. The config file is located at:

- **Linux/macOS**: `~/.config/livemd/config.json`
- **Windows**: `%APPDATA%\livemd\config.json`

### Example Configuration

```json
{
  "theme_file": "themes/my_theme.json"
}
```

## Configuration Options

### theme_file
Path to a custom theme JSON file, relative to the config directory.

```json
{
  "theme_file": "themes/dracula.json"
}
```

## Command Line Options

All configuration can be overridden with command-line flags:

### Streaming Options
- `--speed <SECONDS>`: Delay between chunks (default: 0.005)
  - Lower values = faster streaming
  - Higher values = slower, more readable streaming
- `--chunk-size <BYTES>`: Max chunk size before flush (default: 3200)

### Content Processing
- `--strip-boxes`: Convert ASCII box drawings to Markdown headers
- `--no-inject`: Skip Markdown instruction injection for LLM queries

### Theming
- `--theme <THEME>`: Built-in theme (dark/light/mono)
- `--theme-file <PATH>`: Path to custom theme JSON file

### Input Sources
- `--file <PATH>`: Markdown file to stream
- `--cmd <COMMAND>`: Shell command to run and stream output
- `--query <TEXT>`: AI query to process and stream

## Environment Variables

livemd respects these environment variables:

- `LIVEMD_CONFIG_DIR`: Override config directory location
- `LIVEMD_THEME`: Default theme (same as `--theme`)

## Priority Order

Configuration values are applied in this priority order (highest to lowest):

1. Command-line flags
2. Environment variables
3. Configuration file
4. Built-in defaults

## Directory Structure

livemd expects this directory structure in your config directory:

```
~/.config/livemd/
├── config.json          # Main configuration
└── themes/              # Custom theme files
    ├── dracula.json
    ├── solarized.json
    └── my_theme.json
```

## Example Setup

1. Create the config directory:
   ```bash
   mkdir -p ~/.config/livemd/themes
   ```

2. Create a custom theme (see [THEMES.md](THEMES.md) for details)

3. Set up config.json:
   ```json
   {
     "theme_file": "themes/my_custom_theme.json"
   }
   ```

4. Use livemd with your custom configuration:
   ```bash
   livemd --file document.md  # Uses your custom theme automatically
   ```