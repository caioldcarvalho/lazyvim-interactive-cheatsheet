# LazyVim Interactive Cheatsheet

Interactive TUI to search LazyVim keybindings and visualize key sequences on a keyboard layout.

## Features
- Fuzzy search over keybindings, descriptions, and categories.
- Keyboard animation showing key sequences frame by frame.
- Legend view to see all frames at once with color coding.
- Categories and modes loaded from a JSON file.

## Build
```bash
cargo build
```

## Run
```bash
cargo run
```

## Controls
- Type to search.
- Backspace removes a character from the query.
- Up/Down or Tab/Shift-Tab moves the selection.
- Esc clears the query; Esc on empty query quits.
- Ctrl+C quits.
- Ctrl+V toggles Animation/Legend view.

## Data Format
Commands live in `data/commands.json` and are embedded at compile time.

Each entry uses this shape:
```json
{
  "keys": "<leader>ff",
  "description": "Find files",
  "category": "search",
  "mode": "normal"
}
```

Supported categories are defined in `src/commands.rs`.
Modes default to `normal` when omitted.

## Project Layout
- `src/main.rs`: TUI setup and event loop.
- `src/ui.rs`: Rendering and input handling.
- `src/commands.rs`: Command parsing and keyframe generation.
- `src/keyboard.rs`: Keyboard layout and highlighting.
- `src/search.rs`: Fuzzy search scoring.
- `data/commands.json`: Keybinding data source.

## License
MIT
