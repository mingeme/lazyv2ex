use crate::action::Action;

// pub mod detail;
pub mod home;

pub trait Page {
    fn page_type(&self) -> PageType;
    fn init(&mut self) -> Option<Action>;
    fn render(&mut self, frame: &mut ratatui::Frame);
    fn handle_event(&mut self, key_event: crossterm::event::KeyEvent) -> Option<Action>;
    fn update(&mut self, action: Action) -> Option<Action>;
}

#[derive(PartialEq, Eq)]
pub enum PageType {
    Home,
    Detail,
}
