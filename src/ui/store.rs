use ratatui::{
    style::{palette::tailwind, Color},
    widgets::TableState,
};

use crate::{api::Crawler, model::Topic};

use super::action::Action;

pub struct Store {
    pub home: Home,
    pub running: bool,
    crawler: Crawler,
}

impl Store {
    pub fn new() -> Self {
        Self {
            home: Home::new(),
            running: true,
            crawler: Crawler::new(),
        }
    }
    pub fn update(&mut self, action: Action) {
        match action {
            Action::PreviousRow => self.home.state.select_previous(),
            Action::NextRow => self.home.state.select_next(),
            Action::Reload => {
                self.home.state.select(Some(0));
                self.home.items = self.crawler.fetch_topics().unwrap();
            }
            Action::Top => self.home.state.select_first(),
            Action::Bottom => self.home.state.select_last(),
            Action::Enter => {}
            Action::Quit => self.running = false,
        }
    }
}

pub struct TableColors {
    pub buffer_bg: Color,
    pub header_bg: Color,
    pub header_fg: Color,
    pub row_fg: Color,
    pub selected_row_style_fg: Color,
    pub selected_column_style_fg: Color,
    pub selected_cell_style_fg: Color,
    pub normal_row_color: Color,
    pub alt_row_color: Color,
    pub footer_border_color: Color,
}

impl TableColors {
    const fn new(color: &tailwind::Palette) -> Self {
        Self {
            buffer_bg: tailwind::SLATE.c950,
            header_bg: color.c900,
            header_fg: tailwind::SLATE.c200,
            row_fg: tailwind::SLATE.c200,
            selected_row_style_fg: color.c400,
            selected_column_style_fg: color.c400,
            selected_cell_style_fg: color.c600,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
            footer_border_color: color.c400,
        }
    }
}

pub struct Home {
    pub items: Vec<Topic>,
    pub state: TableState,
    pub colors: TableColors,
}

impl Home {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            state: TableState::default(),
            colors: TableColors::new(&tailwind::BLUE),
        }
    }
}
