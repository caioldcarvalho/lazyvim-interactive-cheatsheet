use crate::commands::{Command, KeyFrame};
use crate::keyboard::{Keyboard, FRAME_COLORS};
use crate::search::SearchEngine;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use std::time::{Duration, Instant};

const FRAME_DURATION_MS: u64 = 500; // Animation speed

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ViewMode {
    #[default]
    Animation,
    Legend,
}

impl ViewMode {
    pub fn toggle(&mut self) {
        *self = match self {
            ViewMode::Animation => ViewMode::Legend,
            ViewMode::Legend => ViewMode::Animation,
        };
    }
}

pub struct App {
    pub query: String,
    pub commands: Vec<Command>,
    pub filtered_results: Vec<usize>,
    pub selected_index: usize,
    pub search_engine: SearchEngine,
    pub keyboard: Keyboard,
    pub should_quit: bool,
    // Animation state
    pub current_frame: usize,
    pub last_frame_time: Instant,
    pub cached_frames: Vec<KeyFrame>,
    pub last_selected: Option<usize>,
    // View mode
    pub view_mode: ViewMode,
}

impl App {
    pub fn new(commands: Vec<Command>) -> Self {
        let filtered_results: Vec<usize> = (0..commands.len()).collect();
        Self {
            query: String::new(),
            commands,
            filtered_results,
            selected_index: 0,
            search_engine: SearchEngine::new(),
            keyboard: Keyboard::new(),
            should_quit: false,
            current_frame: 0,
            last_frame_time: Instant::now(),
            cached_frames: Vec::new(),
            last_selected: None,
            view_mode: ViewMode::default(),
        }
    }

    pub fn update_search(&mut self) {
        let results = self.search_engine.search(&self.commands, &self.query);
        self.filtered_results = results
            .into_iter()
            .map(|(cmd, _)| {
                self.commands
                    .iter()
                    .position(|c| std::ptr::eq(c, cmd))
                    .unwrap()
            })
            .collect();
        self.selected_index = 0;
        self.reset_animation();
    }

    pub fn selected_command(&self) -> Option<&Command> {
        self.filtered_results
            .get(self.selected_index)
            .and_then(|&idx| self.commands.get(idx))
    }

    fn reset_animation(&mut self) {
        self.current_frame = 0;
        self.last_frame_time = Instant::now();
        self.cached_frames = self
            .selected_command()
            .map(|cmd| cmd.parse_keys())
            .unwrap_or_default();
        self.last_selected = self.filtered_results.get(self.selected_index).copied();
    }

    pub fn tick(&mut self) {
        // Check if selection changed
        let current_selected = self.filtered_results.get(self.selected_index).copied();
        if current_selected != self.last_selected {
            self.reset_animation();
        }

        // Advance animation frame
        if !self.cached_frames.is_empty()
            && self.last_frame_time.elapsed() >= Duration::from_millis(FRAME_DURATION_MS)
        {
            self.current_frame = (self.current_frame + 1) % self.cached_frames.len();
            self.last_frame_time = Instant::now();
        }
    }

    pub fn handle_input(&mut self) -> anyhow::Result<()> {
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => {
                        if self.query.is_empty() {
                            self.should_quit = true;
                        } else {
                            self.query.clear();
                            self.update_search();
                        }
                    }
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.should_quit = true;
                    }
                    KeyCode::Char('v') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.view_mode.toggle();
                    }
                    KeyCode::Char(c) => {
                        self.query.push(c);
                        self.update_search();
                    }
                    KeyCode::Backspace => {
                        self.query.pop();
                        self.update_search();
                    }
                    KeyCode::Down | KeyCode::Tab => {
                        if !self.filtered_results.is_empty() {
                            self.selected_index =
                                (self.selected_index + 1) % self.filtered_results.len();
                        }
                    }
                    KeyCode::Up | KeyCode::BackTab => {
                        if !self.filtered_results.is_empty() {
                            self.selected_index = if self.selected_index == 0 {
                                self.filtered_results.len() - 1
                            } else {
                                self.selected_index - 1
                            };
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),  // Search input
                Constraint::Min(8),     // Results list
                Constraint::Length(15), // Keyboard
            ])
            .split(frame.area());

        self.draw_search_input(frame, chunks[0]);
        self.draw_results_list(frame, chunks[1]);
        self.draw_keyboard(frame, chunks[2]);
    }

    fn draw_search_input(&self, frame: &mut Frame, area: Rect) {
        let input = Paragraph::new(Line::from(vec![
            Span::styled("Search: ", Style::default().fg(Color::Yellow)),
            Span::raw(&self.query),
            Span::styled(
                "_",
                Style::default()
                    .fg(Color::Gray)
                    .add_modifier(Modifier::SLOW_BLINK),
            ),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("LazyVim Helper (Esc to quit)"),
        );
        frame.render_widget(input, area);
    }

    fn draw_results_list(&self, frame: &mut Frame, area: Rect) {
        let results_count = self.filtered_results.len();
        let title = format!("Commands ({} results)", results_count);
        let list_height = area.height.saturating_sub(2) as usize;
        let mut start = 0usize;

        if list_height > 0 && results_count > list_height {
            let half = list_height / 2;
            if self.selected_index > half {
                start = self.selected_index - half;
            }
            let max_start = results_count - list_height;
            if start > max_start {
                start = max_start;
            }
        }

        let end = if list_height == 0 {
            start
        } else {
            (start + list_height).min(results_count)
        };

        let items: Vec<ListItem> = (start..end)
            .map(|i| {
                let cmd_idx = self.filtered_results[i];
                let cmd = &self.commands[cmd_idx];
                let style = if i == self.selected_index {
                    Style::default()
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };

                let content = Line::from(vec![
                    Span::styled(format!("{:16}", cmd.keys), style.fg(Color::Cyan)),
                    Span::styled(" │ ", style.fg(Color::DarkGray)),
                    Span::styled(&cmd.description, style),
                    Span::styled(" │ ", style.fg(Color::DarkGray)),
                    Span::styled(
                        format!("[{}]", cmd.category.as_str()),
                        style.fg(Color::Yellow),
                    ),
                ]);

                ListItem::new(content)
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(title))
            .highlight_style(Style::default().bg(Color::DarkGray));

        let mut state = ListState::default();
        if results_count > 0 && list_height > 0 {
            state.select(Some(self.selected_index.saturating_sub(start)));
        }

        frame.render_stateful_widget(list, area, &mut state);
    }

    fn draw_keyboard(&self, frame: &mut Frame, area: Rect) {
        match self.view_mode {
            ViewMode::Animation => self.draw_keyboard_animation(frame, area),
            ViewMode::Legend => self.draw_keyboard_legend(frame, area),
        }
    }

    fn draw_keyboard_animation(&self, frame: &mut Frame, area: Rect) {
        let highlighted_keys = self.get_current_frame_keys();
        let kb_lines = self.keyboard.render(&highlighted_keys);

        let title = if let Some(cmd) = self.selected_command() {
            let total_frames = self.cached_frames.len();
            if total_frames > 1 {
                format!(
                    " {} [frame {}/{}] ",
                    cmd.keys,
                    self.current_frame + 1,
                    total_frames
                )
            } else {
                format!(" {} ", cmd.keys)
            }
        } else {
            String::new()
        };

        let kb_widget = Paragraph::new(kb_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Keyboard{} (Ctrl+V: Legend)", title)),
        );

        frame.render_widget(kb_widget, area);
    }

    fn draw_keyboard_legend(&self, frame: &mut Frame, area: Rect) {
        // Split area for keyboard and legend bar
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(13), Constraint::Length(1)])
            .split(area);

        // Get all frames as key lists
        let all_frames: Vec<Vec<&str>> = self
            .cached_frames
            .iter()
            .map(|kf| {
                kf.keys
                    .iter()
                    .filter_map(|k| Self::key_to_static(&k.key))
                    .collect()
            })
            .collect();

        let kb_lines = self.keyboard.render_legend(&all_frames);

        let title = self
            .selected_command()
            .map(|cmd| format!(" {} ", cmd.keys))
            .unwrap_or_default();

        let kb_widget = Paragraph::new(kb_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Keyboard{} (Ctrl+V: Animation)", title)),
        );

        frame.render_widget(kb_widget, chunks[0]);

        // Draw legend bar showing sequence
        let legend_spans = self.build_legend_bar();
        let legend = Paragraph::new(Line::from(legend_spans));
        frame.render_widget(legend, chunks[1]);
    }

    fn build_legend_bar(&self) -> Vec<Span<'static>> {
        let mut spans = Vec::new();
        spans.push(Span::styled("Sequence: ", Style::default().fg(Color::Gray)));

        for (i, kf) in self.cached_frames.iter().enumerate() {
            let color = FRAME_COLORS[i % FRAME_COLORS.len()];

            // Build key representation for this frame
            let keys_str: String = kf
                .keys
                .iter()
                .map(|k| {
                    if k.key == "Space" {
                        "␣".to_string()
                    } else if k.key.len() > 1 {
                        k.key.clone()
                    } else {
                        k.key.to_uppercase()
                    }
                })
                .collect::<Vec<_>>()
                .join("+");

            spans.push(Span::styled(
                format!(" {} ", keys_str),
                Style::default().fg(Color::Black).bg(color),
            ));

            if i < self.cached_frames.len() - 1 {
                spans.push(Span::styled(" → ", Style::default().fg(Color::DarkGray)));
            }
        }

        spans
    }

    fn get_current_frame_keys(&self) -> Vec<&'static str> {
        if self.cached_frames.is_empty() {
            return Vec::new();
        }

        let current = &self.cached_frames[self.current_frame];
        let mut result = Vec::new();

        for key in &current.keys {
            if let Some(static_key) = Self::key_to_static(&key.key) {
                result.push(static_key);
            }
        }

        result
    }

    fn key_to_static(key: &str) -> Option<&'static str> {
        match key.to_lowercase().as_str() {
            "space" => Some("Space"),
            "ctrl" => Some("Ctrl"),
            "alt" => Some("Alt"),
            "shift" => Some("Shift"),
            "enter" => Some("Enter"),
            "esc" => Some("Esc"),
            "tab" => Some("Tab"),
            "backsp" => Some("Backsp"),
            "a" => Some("a"),
            "b" => Some("b"),
            "c" => Some("c"),
            "d" => Some("d"),
            "e" => Some("e"),
            "f" => Some("f"),
            "g" => Some("g"),
            "h" => Some("h"),
            "i" => Some("i"),
            "j" => Some("j"),
            "k" => Some("k"),
            "l" => Some("l"),
            "m" => Some("m"),
            "n" => Some("n"),
            "o" => Some("o"),
            "p" => Some("p"),
            "q" => Some("q"),
            "r" => Some("r"),
            "s" => Some("s"),
            "t" => Some("t"),
            "u" => Some("u"),
            "v" => Some("v"),
            "w" => Some("w"),
            "x" => Some("x"),
            "y" => Some("y"),
            "z" => Some("z"),
            "0" => Some("0"),
            "1" => Some("1"),
            "2" => Some("2"),
            "3" => Some("3"),
            "4" => Some("4"),
            "5" => Some("5"),
            "6" => Some("6"),
            "7" => Some("7"),
            "8" => Some("8"),
            "9" => Some("9"),
            "/" => Some("/"),
            "." => Some("."),
            "," => Some(","),
            ";" => Some(";"),
            "'" => Some("'"),
            "[" => Some("["),
            "]" => Some("]"),
            "\\" => Some("\\"),
            "-" => Some("-"),
            "=" => Some("="),
            "`" => Some("`"),
            _ => None,
        }
    }
}
