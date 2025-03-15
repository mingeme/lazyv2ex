use crossterm::event::{Event, KeyCode, KeyModifiers};

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
        let pages: Vec<Box<dyn Page>> = vec![
            Box::new(crate::pages::home::HomePage::new()),
            Box::new(crate::pages::detail::DetailPage::new()),
        ];

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

    pub fn handle_event(&mut self, event: Event) -> Option<Action> {
        if let Event::Key(key_event) = event {
            if key_event.code == KeyCode::Char('q') {
                return Some(Action::Quit);
            }
            if key_event.code == KeyCode::Char('c') && key_event.modifiers == KeyModifiers::CONTROL
            {
                return Some(Action::Quit);
            }
        }
        for page in &mut self.pages {
            if page.page_type() == self.current_page {
                return page.handle_event(event);
            }
        }
        None
    }

    pub fn update(&mut self, action: Action) -> Option<Action> {
        if let Action::OpenBrowser(url) = action {
            if let Err(e) = open::that(&url) {
                eprintln!("Failed to open URL: {}", e);
            }
            return None;
        }
        for page in &mut self.pages {
            if page.page_type() == self.current_page {
                match action {
                    Action::Enter => {
                        self.current_page = PageType::Detail;
                    }
                    Action::GoHome => {
                        self.current_page = PageType::Home;
                    }
                    _ => {}
                }
                let next_action = page.update(action);
                return next_action;
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
