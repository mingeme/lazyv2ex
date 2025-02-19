use crate::action::Action;

use super::{Page, PageType};
use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct DetailPage {}

impl DetailPage {
    pub fn new() -> Self {
        DetailPage {}
    }
}

impl Page for DetailPage {
    fn page_type(&self) -> PageType {
        PageType::Detail
    }

    fn init(&mut self) -> Action {
        Action::Noop
    }

    fn render(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(frame.area());

        let block = Block::default()
            .title("Settings Page")
            .borders(Borders::ALL);
        let paragraph =
            Paragraph::new("This is the Settings Page. Press 'h' to go back to Home.").block(block);

        frame.render_widget(paragraph, chunks[0]);
    }

    fn handle_event(&mut self, key_event: KeyEvent) -> Action {
        if key_event.code == crossterm::event::KeyCode::Char('h') {
            println!("Switching to Home Page...");
        }
        Action::Noop
    }

    fn update(&mut self, action: Action) -> Action {
        match action {
            _ => Action::Noop,
        }
    }
}
