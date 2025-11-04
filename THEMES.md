# livemd Themes

livemd supports full color customization with hex colors and named colors.

## Theme Structure

```json
{
  "heading": "#ff6b6b",
  "code": "#4ecdc4",
  "bold": "#ffd93d",
  "italic": "#6bcf7f",
  "link": "#4d96ff",
  "list": "#f368e0"
}
```

## Individual Header Colors

Use an array for different colors per header level (H1-H6):

```json
{
  "heading": [
    "#ff6b6b",  // H1
    "#4ecdc4",  // H2
    "#ffd93d",  // H3
    "#6bcf7f",  // H4
    "#4d96ff",  // H5
    "#f368e0"   // H6
  ],
  "code": "#2d3748",
  "bold": "#ffd93d",
  "italic": "#6bcf7f",
  "link": "#4d96ff",
  "list": "#f368e0"
}
```

## Color Formats

- **Hex**: `#RRGGBB` (e.g., `#ff6b6b`)
- **Named**: `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`, `grey`, `dark_grey`

## Built-in Themes

- **`dark`** (default): For dark terminals
- **`light`**: For light terminals
- **`mono`**: Monochrome

## Custom Themes

### Solarized Dark
```json
{
  "heading": "#268bd2",
  "code": "#073642",
  "bold": "#d33682",
  "italic": "#2aa198",
  "link": "#268bd2",
  "list": "#859900"
}
```

### Dracula
```json
{
  "heading": "#ff79c6",
  "code": "#44475a",
  "bold": "#50fa7b",
  "italic": "#ffb86c",
  "link": "#8be9fd",
  "list": "#bd93f9"
}
```

### Gruvbox
```json
{
  "heading": "#fb4934",
  "code": "#3c3836",
  "bold": "#fabd2f",
  "italic": "#83a598",
  "link": "#83a598",
  "list": "#b8bb26"
}
```

## Theme Locations

livemd loads themes in this order:
1. `--theme-file` option
2. `~/.config/livemd/themes/default.json` (auto-loaded)
3. `~/.config/livemd/themes/` directory
4. Built-in themes

Invalid colors fall back to white. Check terminal output for parsing errors.

## Example: My catppuccin-mocha variant
```json
{
  "heading": [
    "#cba6f7",
    "#89b4fa", 
    "#a6e3a1",
    "#fab387",
    "#f38ba8",
    "#89dceb"
  ],
  "code": "#313244",
  "bold": "#f9e2af",
  "italic": "#94e2d5",
  "link": "#74c7ec",
  "list": "#a6adc8"
}
```
