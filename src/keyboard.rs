use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};
use std::collections::HashMap;

/// Key position and size on the keyboard layout
#[derive(Debug, Clone, Copy)]
pub struct KeyPosition {
    pub row: usize,
    pub col: usize,
    pub width: usize,
}

/// Keyboard layout with ASCII art and key mappings
pub struct Keyboard {
    key_positions: HashMap<String, KeyPosition>,
}

impl Default for Keyboard {
    fn default() -> Self {
        Self::new()
    }
}

impl Keyboard {
    pub fn new() -> Self {
        let mut key_positions = HashMap::new();

        // Row 0: Esc, F1-F12
        key_positions.insert("Esc".to_string(), KeyPosition { row: 0, col: 1, width: 3 });
        key_positions.insert("F1".to_string(), KeyPosition { row: 0, col: 6, width: 2 });
        key_positions.insert("F2".to_string(), KeyPosition { row: 0, col: 9, width: 2 });
        key_positions.insert("F3".to_string(), KeyPosition { row: 0, col: 12, width: 2 });
        key_positions.insert("F4".to_string(), KeyPosition { row: 0, col: 15, width: 2 });
        key_positions.insert("F5".to_string(), KeyPosition { row: 0, col: 19, width: 2 });
        key_positions.insert("F6".to_string(), KeyPosition { row: 0, col: 22, width: 2 });
        key_positions.insert("F7".to_string(), KeyPosition { row: 0, col: 25, width: 2 });
        key_positions.insert("F8".to_string(), KeyPosition { row: 0, col: 28, width: 2 });
        key_positions.insert("F9".to_string(), KeyPosition { row: 0, col: 32, width: 2 });
        key_positions.insert("F10".to_string(), KeyPosition { row: 0, col: 35, width: 3 });
        key_positions.insert("F11".to_string(), KeyPosition { row: 0, col: 39, width: 3 });
        key_positions.insert("F12".to_string(), KeyPosition { row: 0, col: 43, width: 3 });

        // Row 1: Number row
        key_positions.insert("`".to_string(), KeyPosition { row: 1, col: 1, width: 1 });
        key_positions.insert("1".to_string(), KeyPosition { row: 1, col: 5, width: 1 });
        key_positions.insert("2".to_string(), KeyPosition { row: 1, col: 9, width: 1 });
        key_positions.insert("3".to_string(), KeyPosition { row: 1, col: 13, width: 1 });
        key_positions.insert("4".to_string(), KeyPosition { row: 1, col: 17, width: 1 });
        key_positions.insert("5".to_string(), KeyPosition { row: 1, col: 21, width: 1 });
        key_positions.insert("6".to_string(), KeyPosition { row: 1, col: 25, width: 1 });
        key_positions.insert("7".to_string(), KeyPosition { row: 1, col: 29, width: 1 });
        key_positions.insert("8".to_string(), KeyPosition { row: 1, col: 33, width: 1 });
        key_positions.insert("9".to_string(), KeyPosition { row: 1, col: 37, width: 1 });
        key_positions.insert("0".to_string(), KeyPosition { row: 1, col: 41, width: 1 });
        key_positions.insert("-".to_string(), KeyPosition { row: 1, col: 45, width: 1 });
        key_positions.insert("=".to_string(), KeyPosition { row: 1, col: 49, width: 1 });
        key_positions.insert("Backsp".to_string(), KeyPosition { row: 1, col: 53, width: 6 });

        // Row 2: QWERTY row
        key_positions.insert("Tab".to_string(), KeyPosition { row: 2, col: 1, width: 3 });
        key_positions.insert("q".to_string(), KeyPosition { row: 2, col: 7, width: 1 });
        key_positions.insert("w".to_string(), KeyPosition { row: 2, col: 11, width: 1 });
        key_positions.insert("e".to_string(), KeyPosition { row: 2, col: 15, width: 1 });
        key_positions.insert("r".to_string(), KeyPosition { row: 2, col: 19, width: 1 });
        key_positions.insert("t".to_string(), KeyPosition { row: 2, col: 23, width: 1 });
        key_positions.insert("y".to_string(), KeyPosition { row: 2, col: 27, width: 1 });
        key_positions.insert("u".to_string(), KeyPosition { row: 2, col: 31, width: 1 });
        key_positions.insert("i".to_string(), KeyPosition { row: 2, col: 35, width: 1 });
        key_positions.insert("o".to_string(), KeyPosition { row: 2, col: 39, width: 1 });
        key_positions.insert("p".to_string(), KeyPosition { row: 2, col: 43, width: 1 });
        key_positions.insert("[".to_string(), KeyPosition { row: 2, col: 47, width: 1 });
        key_positions.insert("]".to_string(), KeyPosition { row: 2, col: 51, width: 1 });
        key_positions.insert("\\".to_string(), KeyPosition { row: 2, col: 55, width: 1 });

        // Row 3: Home row (ASDF)
        key_positions.insert("Ctrl".to_string(), KeyPosition { row: 3, col: 1, width: 4 });
        key_positions.insert("a".to_string(), KeyPosition { row: 3, col: 8, width: 1 });
        key_positions.insert("s".to_string(), KeyPosition { row: 3, col: 12, width: 1 });
        key_positions.insert("d".to_string(), KeyPosition { row: 3, col: 16, width: 1 });
        key_positions.insert("f".to_string(), KeyPosition { row: 3, col: 20, width: 1 });
        key_positions.insert("g".to_string(), KeyPosition { row: 3, col: 24, width: 1 });
        key_positions.insert("h".to_string(), KeyPosition { row: 3, col: 28, width: 1 });
        key_positions.insert("j".to_string(), KeyPosition { row: 3, col: 32, width: 1 });
        key_positions.insert("k".to_string(), KeyPosition { row: 3, col: 36, width: 1 });
        key_positions.insert("l".to_string(), KeyPosition { row: 3, col: 40, width: 1 });
        key_positions.insert(";".to_string(), KeyPosition { row: 3, col: 44, width: 1 });
        key_positions.insert("'".to_string(), KeyPosition { row: 3, col: 48, width: 1 });
        key_positions.insert("Enter".to_string(), KeyPosition { row: 3, col: 52, width: 5 });

        // Row 4: Shift row (ZXCV)
        key_positions.insert("Shift".to_string(), KeyPosition { row: 4, col: 1, width: 5 });
        key_positions.insert("z".to_string(), KeyPosition { row: 4, col: 9, width: 1 });
        key_positions.insert("x".to_string(), KeyPosition { row: 4, col: 13, width: 1 });
        key_positions.insert("c".to_string(), KeyPosition { row: 4, col: 17, width: 1 });
        key_positions.insert("v".to_string(), KeyPosition { row: 4, col: 21, width: 1 });
        key_positions.insert("b".to_string(), KeyPosition { row: 4, col: 25, width: 1 });
        key_positions.insert("n".to_string(), KeyPosition { row: 4, col: 29, width: 1 });
        key_positions.insert("m".to_string(), KeyPosition { row: 4, col: 33, width: 1 });
        key_positions.insert(",".to_string(), KeyPosition { row: 4, col: 37, width: 1 });
        key_positions.insert(".".to_string(), KeyPosition { row: 4, col: 41, width: 1 });
        key_positions.insert("/".to_string(), KeyPosition { row: 4, col: 45, width: 1 });
        key_positions.insert("RShift".to_string(), KeyPosition { row: 4, col: 49, width: 6 });

        // Row 5: Bottom row
        key_positions.insert("LCtrl".to_string(), KeyPosition { row: 5, col: 1, width: 4 });
        key_positions.insert("Super".to_string(), KeyPosition { row: 5, col: 6, width: 3 });
        key_positions.insert("Alt".to_string(), KeyPosition { row: 5, col: 10, width: 3 });
        key_positions.insert("Space".to_string(), KeyPosition { row: 5, col: 14, width: 23 });
        key_positions.insert("RAlt".to_string(), KeyPosition { row: 5, col: 38, width: 3 });
        key_positions.insert("Fn".to_string(), KeyPosition { row: 5, col: 42, width: 2 });
        key_positions.insert("Menu".to_string(), KeyPosition { row: 5, col: 45, width: 4 });
        key_positions.insert("RCtrl".to_string(), KeyPosition { row: 5, col: 50, width: 4 });

        Self { key_positions }
    }

    /// Get the base keyboard layout as lines
    pub fn get_layout_lines(&self) -> Vec<&'static str> {
        vec![
            "┌───┬──┬──┬──┬──┬───┬──┬──┬──┬───┬───┬───┬───┐",
            "│Esc│F1│F2│F3│F4│ F5│F6│F7│F8│ F9│F10│F11│F12│",
            "├───┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬───┤",
            "│ `  │1 │2 │3 │4 │5 │6 │7 │8 │9 │0 │- │= │Bsp│",
            "├────┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬──┤",
            "│Tab  │Q │W │E │R │T │Y │U │I │O │P │[ │] │\\ │",
            "├─────┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴──┤",
            "│Ctrl  │A │S │D │F │G │H │J │K │L │; │' │Ent │",
            "├──────┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴────┤",
            "│Shift  │Z │X │C │V │B │N │M │, │. │/ │Shift │",
            "├────┬──┴┬─┴──┴──┴──┴──┴──┴──┴┬─┴─┬┴──┬───┬──┤",
            "│Ctrl│Sup│Alt │     Space     │Alt│Fn │Mnu│Ct│",
            "└────┴───┴────┴───────────────┴───┴───┴───┴──┘",
        ]
    }

    /// Render keyboard with highlighted keys
    pub fn render<'a>(&self, highlighted_keys: &[&str]) -> Vec<Line<'a>> {
        let layout = self.get_layout_lines();
        let mut result = Vec::new();

        // Colors for highlighting
        let highlight_style = Style::default().fg(Color::Black).bg(Color::Yellow);
        let leader_style = Style::default().fg(Color::Black).bg(Color::Cyan);
        let modifier_style = Style::default().fg(Color::Black).bg(Color::Magenta);
        let normal_style = Style::default().fg(Color::Gray);

        // Build a set of keys to highlight with their types
        let mut highlight_map: HashMap<String, Style> = HashMap::new();
        for key in highlighted_keys {
            let key_lower = key.to_lowercase();
            let style = if key_lower == "space" || *key == "Space" {
                leader_style
            } else if ["ctrl", "alt", "shift", "super"].contains(&key_lower.as_str()) {
                modifier_style
            } else {
                highlight_style
            };
            highlight_map.insert(key_lower, style);
            // Also add uppercase version for matching
            highlight_map.insert(key.to_uppercase(), style);
        }

        for line in layout {
            let mut spans = Vec::new();
            let mut current_pos = 0;
            let chars: Vec<char> = line.chars().collect();

            while current_pos < chars.len() {
                let c = chars[current_pos];

                // Check if this is the start of a key label
                if c.is_alphanumeric() || c == '`' || c == '-' || c == '=' || c == '[' || c == ']' || c == '\\' || c == ';' || c == '\'' || c == ',' || c == '.' || c == '/' {
                    // Extract the key label
                    let start = current_pos;
                    let mut end = current_pos;
                    while end < chars.len() && !['│', '┌', '┐', '└', '┘', '├', '┤', '┬', '┴', '┼', '─'].contains(&chars[end]) {
                        end += 1;
                    }

                    let key_str: String = chars[start..end].iter().collect();
                    let key_trimmed = key_str.trim();

                    // Check if this key should be highlighted
                    let style = self.find_key_style(key_trimmed, &highlight_map).unwrap_or(normal_style);

                    spans.push(Span::styled(key_str.clone(), style));
                    current_pos = end;
                } else {
                    // Regular character (borders, spaces)
                    spans.push(Span::styled(c.to_string(), normal_style));
                    current_pos += 1;
                }
            }

            result.push(Line::from(spans));
        }

        result
    }

    fn find_key_style(&self, key: &str, highlight_map: &HashMap<String, Style>) -> Option<Style> {
        let key_lower = key.to_lowercase();

        // Direct match
        if let Some(&style) = highlight_map.get(&key_lower) {
            return Some(style);
        }

        // Check for partial matches (e.g., "Bsp" for "Backsp")
        let key_mappings = [
            ("bsp", "backsp"),
            ("ent", "enter"),
            ("ct", "ctrl"),
            ("mnu", "menu"),
            ("sup", "super"),
        ];

        for (short, full) in key_mappings {
            if key_lower == short || key_lower.starts_with(short) {
                if let Some(&style) = highlight_map.get(full) {
                    return Some(style);
                }
            }
        }

        // Check for single letter keys
        if key_lower.len() == 1 {
            if let Some(&style) = highlight_map.get(&key_lower) {
                return Some(style);
            }
        }

        None
    }

    /// Get position of a key by name
    pub fn get_key_position(&self, key: &str) -> Option<&KeyPosition> {
        let key_lower = key.to_lowercase();
        self.key_positions.get(&key_lower)
            .or_else(|| self.key_positions.get(key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyboard_creation() {
        let kb = Keyboard::new();
        assert!(kb.get_key_position("Space").is_some());
        assert!(kb.get_key_position("a").is_some());
        assert!(kb.get_key_position("Ctrl").is_some());
    }

    #[test]
    fn test_render_keyboard() {
        let kb = Keyboard::new();
        let lines = kb.render(&["f", "f"]);
        assert!(!lines.is_empty());
    }
}
