use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};
use std::collections::HashMap;

/// Colors for each frame in the sequence
pub const FRAME_COLORS: &[Color] = &[
    Color::Yellow,
    Color::Green,
    Color::Cyan,
    Color::Magenta,
    Color::Red,
    Color::Blue,
    Color::LightYellow,
    Color::LightGreen,
];

/// Keyboard layout with ASCII art and key mappings
pub struct Keyboard {
}

impl Default for Keyboard {
    fn default() -> Self {
        Self::new()
    }
}

impl Keyboard {
    pub fn new() -> Self {
        Self {}
    }

    /// Get the base keyboard layout as lines (lowercase, shift_active toggles to uppercase)
    pub fn get_layout_lines(&self, shift_active: bool) -> Vec<&'static str> {
        if shift_active {
            vec![
                "┌───┬──┬──┬──┬──┬──┬──┬──┬──┬──┬────┬───┬────┐",
                "│Esc│F1│F2│F3│F4│F5│F6│F7│F8│F9│ F10│F11│ F12│",
                "├───┴┬─┴┬─┴┬─┴┬─┴┬─┴┬──┬─┴┬─┴┬─┴┬──┬┴─┬─┴┬───┤",
                "│ ~  │! │@ │# │$ │% │^ │& │* │( │) │_ │+ │Bsp│",
                "├────┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬──┤",
                "│Tab  │Q │W │E │R │T │Y │U │I │O │P │{ │} │| │",
                "├─────┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴──┤",
                "│Caps  │A │S │D │F │G │H │J │K │L │: │\" │Ent │",
                "├──────┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴────┤",
                "│Shift  │Z │X │C │V │B │N │M │< │> │? │Shift │",
                "├────┬──┴┬─┴─┬┴──┴──┴──┴──┴──┴┬─┴─┬┴──┬───┬──┤",
                "│Ctrl│Sup│Alt│      Space     │Alt│Fn │Mnu│Ct│",
                "└────┴───┴───┴────────────────┴───┴───┴───┴──┘",
            ]
        } else {
            vec![
                "┌───┬──┬──┬──┬──┬──┬──┬──┬──┬──┬────┬───┬────┐",
                "│Esc│F1│F2│F3│F4│F5│F6│F7│F8│F9│ F10│F11│ F12│",
                "├───┴┬─┴┬─┴┬─┴┬─┴┬─┴┬──┬─┴┬─┴┬─┴┬──┬┴─┬─┴┬───┤",
                "│ `  │1 │2 │3 │4 │5 │6 │7 │8 │9 │0 │- │= │Bsp│",
                "├────┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬──┤",
                "│Tab  │q │w │e │r │t │y │u │i │o │p │[ │] │\\ │",
                "├─────┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴──┤",
                "│Caps  │a │s │d │f │g │h │j │k │l │; │' │Ent │",
                "├──────┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴┬─┴────┤",
                "│Shift  │z │x │c │v │b │n │m │, │. │/ │Shift │",
                "├────┬──┴┬─┴─┬┴──┴──┴──┴──┴──┴┬─┴─┬┴──┬───┬──┤",
                "│Ctrl│Sup│Alt│      Space     │Alt│Fn │Mnu│Ct│",
                "└────┴───┴───┴────────────────┴───┴───┴───┴──┘",
            ]
        }
    }

    /// Render keyboard with highlighted keys
    pub fn render<'a>(&self, highlighted_keys: &[&str]) -> Vec<Line<'a>> {
        // Check if shift is in highlighted keys
        let shift_active = highlighted_keys
            .iter()
            .any(|k| k.to_lowercase() == "shift");
        let layout = self.get_layout_lines(shift_active);
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

    /// Render keyboard with all frames shown simultaneously, each with different color
    pub fn render_legend<'a>(&self, frames: &[Vec<&str>]) -> Vec<Line<'a>> {
        // Check if any frame contains shift
        let shift_active = frames
            .iter()
            .any(|f| f.iter().any(|k| k.to_lowercase() == "shift"));
        let layout = self.get_layout_lines(shift_active);
        let mut result = Vec::new();
        let normal_style = Style::default().fg(Color::Gray);

        // Build map: key -> frame index (for coloring)
        let mut key_to_frame: HashMap<String, usize> = HashMap::new();
        for (frame_idx, frame_keys) in frames.iter().enumerate() {
            for key in frame_keys {
                key_to_frame.insert(key.to_lowercase(), frame_idx);
            }
        }

        for line in layout {
            let mut spans = Vec::new();
            let mut current_pos = 0;
            let chars: Vec<char> = line.chars().collect();

            while current_pos < chars.len() {
                let c = chars[current_pos];

                if c.is_alphanumeric() || c == '`' || c == '-' || c == '=' || c == '[' || c == ']' || c == '\\' || c == ';' || c == '\'' || c == ',' || c == '.' || c == '/' {
                    let start = current_pos;
                    let mut end = current_pos;
                    while end < chars.len() && !['│', '┌', '┐', '└', '┘', '├', '┤', '┬', '┴', '┼', '─'].contains(&chars[end]) {
                        end += 1;
                    }

                    let key_str: String = chars[start..end].iter().collect();
                    let key_trimmed = key_str.trim();

                    let style = self.find_frame_style(key_trimmed, &key_to_frame)
                        .unwrap_or(normal_style);

                    spans.push(Span::styled(key_str.clone(), style));
                    current_pos = end;
                } else {
                    spans.push(Span::styled(c.to_string(), normal_style));
                    current_pos += 1;
                }
            }

            result.push(Line::from(spans));
        }

        result
    }

    fn find_frame_style(&self, key: &str, key_to_frame: &HashMap<String, usize>) -> Option<Style> {
        let key_lower = key.to_lowercase();

        // Direct match
        if let Some(&frame_idx) = key_to_frame.get(&key_lower) {
            let color = FRAME_COLORS[frame_idx % FRAME_COLORS.len()];
            return Some(Style::default().fg(Color::Black).bg(color));
        }

        // Check for partial matches
        let key_mappings = [
            ("bsp", "backsp"),
            ("ent", "enter"),
            ("ct", "ctrl"),
            ("mnu", "menu"),
            ("sup", "super"),
        ];

        for (short, full) in key_mappings {
            if key_lower == short || key_lower.starts_with(short) {
                if let Some(&frame_idx) = key_to_frame.get(full) {
                    let color = FRAME_COLORS[frame_idx % FRAME_COLORS.len()];
                    return Some(Style::default().fg(Color::Black).bg(color));
                }
            }
        }

        // Single letter
        if key_lower.len() == 1 {
            if let Some(&frame_idx) = key_to_frame.get(&key_lower) {
                let color = FRAME_COLORS[frame_idx % FRAME_COLORS.len()];
                return Some(Style::default().fg(Color::Black).bg(color));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_keyboard() {
        let kb = Keyboard::new();
        let lines = kb.render(&["f", "f"]);
        assert!(!lines.is_empty());
    }
}
