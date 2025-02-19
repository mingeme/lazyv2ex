use crossterm::event::{KeyCode, KeyModifiers};

use crate::{
    action::Action,
    pages::{Page, PageType},
};

pub struct App {
    current_page: PageType,
    pages: Vec<Box<dyn Page>>,
}

impl App {
    pub fn new() -> Self {
        let mut pages: Vec<Box<dyn Page>> = Vec::new();
        pages.push(Box::new(crate::pages::home::HomePage::new()));
        // pages.push(Box::new(crate::pages::detail::DetailPage::new()));

        App {
            current_page: PageType::Home,
            pages,
        }
    }

    pub fn render(&mut self, frame: &mut ratatui::Frame) {
        for page in &mut self.pages {
            if page.page_type() == self.current_page {
                page.render(frame);
                break;
            }
        }
    }

    pub fn handle_event(&mut self, key_event: crossterm::event::KeyEvent) -> Option<Action> {
        if key_event.code == KeyCode::Char('q') {
            return Some(Action::Quit);
        }
        if key_event.code == KeyCode::Char('c') && key_event.modifiers == KeyModifiers::CONTROL {
            return Some(Action::Quit);
        }
        for page in &mut self.pages {
            if page.page_type() == self.current_page {
                return page.handle_event(key_event);
            }
        }
        None
    }

    pub fn update(&mut self, action: Action) -> Option<Action> {
        for page in &mut self.pages {
            if page.page_type() == self.current_page {
                return page.update(action);
            }
        }
        None
    }

    pub fn switch_page(&mut self, page_type: PageType) -> Option<Action> {
        self.current_page = page_type;
        for page in &mut self.pages {
            if page.page_type() == self.current_page {
                return page.init();
            }
        }
        None
    }
}
