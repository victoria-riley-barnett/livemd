# livemd Themes

Detailed guide to customizing colors and themes in livemd.

## Theme Structure

Themes are JSON files that define colors for different Markdown elements:

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
  "code": "#2d3748",
  "bold": "#ffd93d",
  "italic": "#6bcf7f",
  "link": "#4d96ff",
  "list": "#f368e0"
}
```

## Color Formats

### Hex Colors
- Full format: `#RRGGBB` (e.g., `#ff6b6b`, `#4ecdc4`)
- 6-digit hexadecimal RGB values

### Named Colors
- `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`
- `grey` / `gray`, `dark_grey` / `dark_gray`

## Built-in Themes

### Dark Theme (default)
Optimized for dark terminal backgrounds.

### Light Theme
Optimized for light terminal backgrounds.

### Mono Theme
Monochrome theme for accessibility or limited color support.

## Custom Theme Locations

livemd looks for theme files in these locations (in order):

1. Path specified with `--theme-file` option
2. `~/.config/livemd/themes/` directory
3. Current working directory

## Example Themes

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

## Theme Validation

livemd validates theme files at startup. Invalid colors will fall back to white. Check the terminal output for any theme parsing errors.