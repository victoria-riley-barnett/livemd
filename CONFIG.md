# livemd Configuration

livemd supports a JSON configuration file at `~/.config/livemd/config.json` for default settings.

## Example Configuration

```json
{
  // Path to custom theme file (relative to config dir)
  "theme-file": "themes/my_theme.json",

  // LLM command - single string or object with presets
  "llm-cmd": {
    "default": "aichat",
    "dev": "aichat -s dev --save-session",
    "fast": "aichat --model gpt-4o-mini"
  },

  // Streaming speed (seconds between chunks, lower = faster)
  "speed": 0.005,

  // Max chunk size before flush
  "chunk-size": 3200,

  // Built-in theme (dark/light/mono)
  "theme": "dark",

  // Convert ASCII boxes to headers
  "strip-boxes": false,

  // Inject "respond in Markdown" instruction
  "inject-md-instruction": true
}
```

## Configuration Options

All options can be overridden with command-line flags. Priority order: CLI flags > config file > defaults.

### LLM Commands
- Single command: `"llm-cmd": "aichat"`
- Multiple presets: `"llm-cmd": {"preset": "command"}`
- Use presets with: `--llm-cmd preset`

### Theme Files
- Automatic loading: `~/.config/livemd/themes/default.json`
- Custom path: `--theme-file path/to/theme.json`
- Config: `"theme-file": "themes/my_theme.json"`

### Environment Variables
- `LIVEMD_CONFIG_DIR`: Override config directory
- `LIVEMD_THEME`: Default theme

## Directory Structure

```
~/.config/livemd/
├── config.json
└── themes/
    ├── default.json    # Auto-loaded
    ├── dracula.json
    └── solarized.json
```

See [THEMES.md](THEMES.md) for theme customization.