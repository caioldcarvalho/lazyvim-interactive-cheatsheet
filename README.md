# LazyVim Interactive Cheatsheet

Interactive TUI to search LazyVim keybindings and visualize key sequences on a keyboard layout.

Built with Rust and the `ratatui` framework.

## 🎯 What It Does

Provides a searchable, interactive reference for LazyVim shortcuts and commands - perfect for learning or quick lookups during development. LazyVim has hundreds of keybindings; instead of constantly checking documentation, this tool lets you quickly search and reference commands while staying in the terminal workflow.

## 🎬 Demos

Search and filter keybindings:
![Search demo](assets/demos/search.gif)

Keyboard animation for a selected command:
![Keyboard animation demo](assets/demos/keyboard-animation.gif)

Toggle Animation/Legend view:
![Legend toggle demo](assets/demos/legend-toggle.gif)

## 🛠️ Tech Stack

- **Language**: Rust
- **TUI Framework**: ratatui
- **Target**: LazyVim users
- **Purpose**: Developer productivity tool

## 🚀 Quick Start

```bash
cargo run
```

## 📖 How to Use

1. Start typing to filter keybindings by keys, description, or category.
2. Use Up/Down (or Tab/Shift-Tab) to move the selection.
3. Watch the keyboard animation to learn the sequence.
4. Press Ctrl+V to toggle between Animation and Legend views.
5. Press Esc to clear the search; Esc again (on empty search) quits.

## ⌨️ Controls

| Key | Action |
|-----|--------|
| Type | Search |
| Backspace | Remove character from query |
| Up/Down or Tab/Shift-Tab | Move selection |
| Esc | Clear query (or quit if empty) |
| Ctrl+C | Quit |
| Ctrl+V | Toggle Animation/Legend view |

## 🔧 Customize the Keybindings

Commands live in `data/commands.json` and are embedded at compile time. If you build from source, update this file and rebuild to see changes.

Each entry uses this shape:
```json
{
  "keys": "<leader>ff",
  "description": "Find files",
  "category": "search",
  "mode": "normal"
}
```

Modes default to `normal` when omitted.

## 🔮 Future Work

- User-defined command list (external file or plugin config)
- LazyVim plugin for easier integration
- More keyboard layouts (ABNT2, AZERTY, QWERTZ)

## 🏗️ Build

```bash
cargo build
./target/debug/lazyvim-interactive-cheatsheet
```

## 🙏 Credits

Cheatsheet data is based on "LazyVim (neovim) Cheat Sheet" by thesujit on Cheatography:
https://cheatography.com/thesujit/cheat-sheets/lazyvim-neovim/

## 📝 License

MIT

## 👤 Author

Caio Carvalho - [@caioldcarvalho](https://github.com/caioldcarvalho)
