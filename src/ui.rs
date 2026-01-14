use crate::commands::Command;
use crate::keyboard::Keyboard;
use crate::search::SearchEngine;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

pub struct App {
    pub query: String,
    pub commands: Vec<Command>,
    pub filtered_results: Vec<usize>, // Indices into commands
    pub selected_index: usize,
    pub search_engine: SearchEngine,
    pub keyboard: Keyboard,
    pub should_quit: bool,
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
    }

    pub fn selected_command(&self) -> Option<&Command> {
        self.filtered_results
            .get(self.selected_index)
            .and_then(|&idx| self.commands.get(idx))
    }

    pub fn handle_input(&mut self) -> anyhow::Result<()> {
        if event::poll(std::time::Duration::from_millis(50))? {
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
            Span::styled("_", Style::default().fg(Color::Gray).add_modifier(Modifier::SLOW_BLINK)),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("LazyVim Helper (Esc to quit)"),
        );
        frame.render_widget(input, area);
    }

    fn draw_results_list(&self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .filtered_results
            .iter()
            .enumerate()
            .take(50) // Limit displayed results
            .map(|(i, &cmd_idx)| {
                let cmd = &self.commands[cmd_idx];
                let style = if i == self.selected_index {
                    Style::default()
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };

                let content = Line::from(vec![
                    Span::styled(
                        format!("{:16}", cmd.keys),
                        style.fg(Color::Cyan),
                    ),
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

        let results_count = self.filtered_results.len();
        let title = format!("Commands ({} results)", results_count);

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(title))
            .highlight_style(Style::default().bg(Color::DarkGray));

        let mut state = ListState::default();
        state.select(Some(self.selected_index));

        frame.render_stateful_widget(list, area, &mut state);
    }

    fn draw_keyboard(&self, frame: &mut Frame, area: Rect) {
        let highlighted_refs: Vec<&str> = self
            .selected_command()
            .map(|cmd| self.get_highlight_keys(cmd))
            .unwrap_or_default();

        let kb_lines = self.keyboard.render(&highlighted_refs);

        let selected_info = self
            .selected_command()
            .map(|cmd| format!(" {} ", cmd.keys))
            .unwrap_or_default();

        let kb_widget = Paragraph::new(kb_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Keyboard{}", selected_info)),
        );

        frame.render_widget(kb_widget, area);
    }

    fn get_highlight_keys(&self, cmd: &Command) -> Vec<&'static str> {
        // Map parsed keys to static strings for the keyboard
        let keys = cmd.parse_keys();
        let mut result = Vec::new();

        for key in &keys {
            let static_key: &'static str = match key.key.to_lowercase().as_str() {
                "space" => "Space",
                "ctrl" => "Ctrl",
                "alt" => "Alt",
                "shift" => "Shift",
                "enter" => "Enter",
                "esc" => "Esc",
                "tab" => "Tab",
                "backsp" => "Backsp",
                "a" => "a",
                "b" => "b",
                "c" => "c",
                "d" => "d",
                "e" => "e",
                "f" => "f",
                "g" => "g",
                "h" => "h",
                "i" => "i",
                "j" => "j",
                "k" => "k",
                "l" => "l",
                "m" => "m",
                "n" => "n",
                "o" => "o",
                "p" => "p",
                "q" => "q",
                "r" => "r",
                "s" => "s",
                "t" => "t",
                "u" => "u",
                "v" => "v",
                "w" => "w",
                "x" => "x",
                "y" => "y",
                "z" => "z",
                "0" => "0",
                "1" => "1",
                "2" => "2",
                "3" => "3",
                "4" => "4",
                "5" => "5",
                "6" => "6",
                "7" => "7",
                "8" => "8",
                "9" => "9",
                "/" => "/",
                "." => ".",
                "," => ",",
                ";" => ";",
                "'" => "'",
                "[" => "[",
                "]" => "]",
                "\\" => "\\",
                "-" => "-",
                "=" => "=",
                "`" => "`",
                _ => continue,
            };
            result.push(static_key);
        }

        result
    }
}
