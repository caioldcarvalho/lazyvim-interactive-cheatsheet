use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub keys: String,
    pub description: String,
    pub category: Category,
    #[serde(default)]
    pub mode: Mode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    #[default]
    Normal,
    Insert,
    Visual,
    Command,
}

impl Mode {
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    General,
    Navigation,
    Search,
    Lsp,
    Git,
    Buffer,
    Window,
    Tab,
    Code,
    Debug,
    Terminal,
    Ui,
    Plugin,
}

impl Category {
    pub fn as_str(&self) -> &'static str {
        match self {
            Category::General => "General",
            Category::Navigation => "Navigation",
            Category::Search => "Search",
            Category::Lsp => "LSP",
            Category::Git => "Git",
            Category::Buffer => "Buffer",
            Category::Window => "Window",
            Category::Tab => "Tab",
            Category::Code => "Code",
            Category::Debug => "Debug",
            Category::Terminal => "Terminal",
            Category::Ui => "UI",
            Category::Plugin => "Plugin",
        }
    }
}

/// A single key in a keypress
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Key {
    pub key: String,
    pub is_modifier: bool,
    pub is_leader: bool,
}

/// A frame represents keys pressed simultaneously (e.g., Shift+D)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyFrame {
    pub keys: Vec<Key>,
}

impl KeyFrame {
    pub fn new(keys: Vec<Key>) -> Self {
        Self { keys }
    }

    pub fn single(key: Key) -> Self {
        Self { keys: vec![key] }
    }
}

impl Command {
    /// Parse keys into animation frames
    /// Each frame = keys pressed at the same time
    /// Example: "gD" -> [Frame{g}, Frame{Shift, d}]
    /// Example: "<C-w>v" -> [Frame{Ctrl, w}, Frame{v}]
    pub fn parse_keys(&self) -> Vec<KeyFrame> {
        let mut frames = Vec::new();
        let keys = &self.keys;
        let mut chars = keys.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '<' {
                // Parse special key like <leader>, <C-w>, <S-Tab>, etc.
                let mut special = String::new();
                while let Some(&next) = chars.peek() {
                    chars.next();
                    if next == '>' {
                        break;
                    }
                    special.push(next);
                }

                let frame = Self::parse_special_key(&special);
                frames.push(frame);
            } else if c != '-' && c != '+' {
                // Regular character
                let frame = if c.is_ascii_uppercase() {
                    // Uppercase letter needs Shift
                    KeyFrame::new(vec![
                        Key {
                            key: "Shift".to_string(),
                            is_modifier: true,
                            is_leader: false,
                        },
                        Key {
                            key: c.to_lowercase().to_string(),
                            is_modifier: false,
                            is_leader: false,
                        },
                    ])
                } else {
                    KeyFrame::single(Key {
                        key: c.to_string(),
                        is_modifier: false,
                        is_leader: false,
                    })
                };
                frames.push(frame);
            }
        }

        frames
    }

    fn parse_special_key(special: &str) -> KeyFrame {
        // Handle combinations like C-w, S-Tab, A-j
        let parts: Vec<&str> = special.split('-').collect();

        if parts.len() == 1 {
            // Simple special key like <leader>, <CR>, <Esc>
            let key_lower = special.to_lowercase();
            let (display_key, is_leader) = match key_lower.as_str() {
                "leader" | "space" => ("Space".to_string(), true),
                "cr" | "enter" | "return" => ("Enter".to_string(), false),
                "esc" | "escape" => ("Esc".to_string(), false),
                "bs" | "backspace" => ("Backsp".to_string(), false),
                "tab" => ("Tab".to_string(), false),
                _ => (special.to_string(), false),
            };

            KeyFrame::single(Key {
                key: display_key,
                is_modifier: false,
                is_leader,
            })
        } else {
            // Combination like C-w, S-Tab, A-j
            let mut keys = Vec::new();

            for (i, part) in parts.iter().enumerate() {
                let part_lower = part.to_lowercase();
                let is_last = i == parts.len() - 1;

                if !is_last {
                    // Modifier
                    let modifier = match part_lower.as_str() {
                        "c" | "ctrl" | "control" => "Ctrl",
                        "s" | "shift" => "Shift",
                        "a" | "alt" | "m" | "meta" => "Alt",
                        _ => continue,
                    };
                    keys.push(Key {
                        key: modifier.to_string(),
                        is_modifier: true,
                        is_leader: false,
                    });
                } else {
                    // Target key
                    let display_key = match part_lower.as_str() {
                        "cr" | "enter" | "return" => "Enter".to_string(),
                        "esc" | "escape" => "Esc".to_string(),
                        "bs" | "backspace" => "Backsp".to_string(),
                        "tab" => "Tab".to_string(),
                        "space" => "Space".to_string(),
                        "up" => "Up".to_string(),
                        "down" => "Down".to_string(),
                        "left" => "Left".to_string(),
                        "right" => "Right".to_string(),
                        _ => part.to_lowercase(),
                    };
                    keys.push(Key {
                        key: display_key,
                        is_modifier: false,
                        is_leader: false,
                    });
                }
            }

            KeyFrame::new(keys)
        }
    }
}

pub fn load_commands() -> anyhow::Result<Vec<Command>> {
    let json_data = include_str!("../data/commands.json");
    let commands: Vec<Command> = serde_json::from_str(json_data)?;
    Ok(commands)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_leader_key() {
        let cmd = Command {
            keys: "<leader>ff".to_string(),
            description: "Find files".to_string(),
            category: Category::Search,
            mode: Mode::Normal,
        };

        let frames = cmd.parse_keys();
        assert_eq!(frames.len(), 3);
        // Frame 1: Space (leader)
        assert_eq!(frames[0].keys.len(), 1);
        assert!(frames[0].keys[0].is_leader);
        assert_eq!(frames[0].keys[0].key, "Space");
        // Frame 2: f
        assert_eq!(frames[1].keys[0].key, "f");
        // Frame 3: f
        assert_eq!(frames[2].keys[0].key, "f");
    }

    #[test]
    fn test_parse_ctrl_combo() {
        let cmd = Command {
            keys: "<C-w>v".to_string(),
            description: "Split vertical".to_string(),
            category: Category::Window,
            mode: Mode::Normal,
        };

        let frames = cmd.parse_keys();
        assert_eq!(frames.len(), 2);
        // Frame 1: Ctrl + w (simultaneous)
        assert_eq!(frames[0].keys.len(), 2);
        assert_eq!(frames[0].keys[0].key, "Ctrl");
        assert!(frames[0].keys[0].is_modifier);
        assert_eq!(frames[0].keys[1].key, "w");
        // Frame 2: v
        assert_eq!(frames[1].keys.len(), 1);
        assert_eq!(frames[1].keys[0].key, "v");
    }

    #[test]
    fn test_parse_uppercase_with_shift() {
        let cmd = Command {
            keys: "gD".to_string(),
            description: "Go to declaration".to_string(),
            category: Category::Lsp,
            mode: Mode::Normal,
        };

        let frames = cmd.parse_keys();
        assert_eq!(frames.len(), 2);
        // Frame 1: g (lowercase, no shift)
        assert_eq!(frames[0].keys.len(), 1);
        assert_eq!(frames[0].keys[0].key, "g");
        // Frame 2: Shift + d (uppercase D)
        assert_eq!(frames[1].keys.len(), 2);
        assert_eq!(frames[1].keys[0].key, "Shift");
        assert!(frames[1].keys[0].is_modifier);
        assert_eq!(frames[1].keys[1].key, "d");
    }

    #[test]
    fn test_parse_shift_combo() {
        let cmd = Command {
            keys: "<S-h>".to_string(),
            description: "Previous buffer".to_string(),
            category: Category::Buffer,
            mode: Mode::Normal,
        };

        let frames = cmd.parse_keys();
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0].keys.len(), 2);
        assert_eq!(frames[0].keys[0].key, "Shift");
        assert_eq!(frames[0].keys[1].key, "h");
    }
}
