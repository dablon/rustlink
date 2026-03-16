use anyhow::Result;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::io;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::storage::{Message, Storage};

/// TUI Application state
pub struct TuiApp {
    pub storage: Storage,
    pub current_screen: Screen,
    pub selected_friend: Option<String>,
    pub messages: Vec<Message>,
    pub input_buffer: String,
    pub scroll_offset: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    FriendsList,
    Chat,
}

impl TuiApp {
    pub fn new(storage: Storage) -> Self {
        Self {
            storage,
            current_screen: Screen::FriendsList,
            selected_friend: None,
            messages: Vec::new(),
            input_buffer: String::new(),
            scroll_offset: 0,
        }
    }

    pub fn load_messages(&mut self, peer_id: &str) -> Result<()> {
        self.messages = self.storage.get_messages(peer_id)?;
        Ok(())
    }
}

/// Run the TUI application
pub fn run_tui(storage: Storage) -> Result<()> {
    let app = Arc::new(Mutex::new(TuiApp::new(storage)));

    // Initialize terminal
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Render once (simplified version - full event handling would use crossterm events)
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(f.area());

        // Title bar
        let title = Paragraph::new("RustLink - P2P Chat")
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL).title("RustLink"));
        f.render_widget(title, chunks[0]);

        // Main content area
        let app_guard = app.blocking_lock();
        match app_guard.current_screen.clone() {
            Screen::FriendsList => {
                let friends = app_guard.storage.get_friends().unwrap_or_default();
                drop(app_guard);

                let items: Vec<ListItem> = friends
                    .iter()
                    .map(|f| {
                        ListItem::new(Line::from(vec![
                            Span::raw(&f.username),
                            Span::raw(" ("),
                            Span::raw(&f.peer_id[..16.min(f.peer_id.len())]),
                            Span::raw(")"),
                        ]))
                    })
                    .collect();

                let list = List::new(items)
                    .block(Block::default().borders(Borders::ALL).title("Friends"))
                    .style(Style::default().fg(Color::White));

                f.render_widget(list, chunks[1]);
            }
            Screen::Chat => {
                let friend = app_guard.selected_friend.clone().unwrap_or_default();
                let messages = app_guard.messages.clone();
                let input_text = app_guard.input_buffer.clone();
                drop(app_guard);

                let chat_area = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(0), Constraint::Length(3)])
                    .split(chunks[1]);

                // Messages
                let message_text: Vec<Line> = messages
                    .iter()
                    .map(|m| {
                        if m.from == friend {
                            Line::from(vec![
                                Span::styled("> ", Style::default().fg(Color::Green)),
                                Span::raw(&m.content),
                            ])
                        } else {
                            Line::from(vec![
                                Span::styled("< ", Style::default().fg(Color::Blue)),
                                Span::raw(&m.content),
                            ])
                        }
                    })
                    .collect();

                let messages_widget = Paragraph::new(message_text)
                    .block(Block::default().borders(Borders::ALL).title("Chat"))
                    .scroll((0, 0));

                f.render_widget(messages_widget, chat_area[0]);

                // Input
                let input = Paragraph::new(input_text)
                    .block(Block::default().borders(Borders::ALL).title("Message"));

                f.render_widget(input, chat_area[1]);
            }
        }

        // Status bar
        let status = {
            let app_guard = app.blocking_lock();
            match app_guard.current_screen {
                Screen::FriendsList => "↑↓: Select | Enter: Chat | q: Quit",
                Screen::Chat => "↑↓: Scroll | Enter: Send | Esc: Back | q: Quit",
            }
        };
        let status_bar = Paragraph::new(status)
            .style(Style::default().fg(Color::DarkGray))
            .block(Block::default().borders(Borders::ALL));

        f.render_widget(status_bar, chunks[2]);
    })?;

    Ok(())
}
