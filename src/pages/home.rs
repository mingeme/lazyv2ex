use crate::{action::Action, api::Crawler, model::Topic};

use super::{Page, PageType};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{palette::tailwind, Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Row, Table, TableState},
    Frame,
};

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

pub struct HomePage {
    pub items: Vec<Topic>,
    pub state: TableState,
    pub colors: TableColors,
    loading: bool,
    crawler: Crawler,
}

impl HomePage {
    pub fn new() -> Self {
        HomePage {
            items: vec![],
            state: TableState::default(),
            loading: true,
            colors: TableColors::new(&tailwind::BLUE),
            crawler: Crawler::new(),
        }
    }
}

impl Page for HomePage {
    fn page_type(&self) -> PageType {
        PageType::Home
    }

    fn init(&mut self) -> Action {
        Action::FetchTopics
    }

    fn render(&mut self, frame: &mut Frame) {
        let main_layout: std::rc::Rc<[ratatui::prelude::Rect]> = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Table
                Constraint::Length(3), // Footer
            ])
            .margin(1)
            .split(frame.area());

        // Render header with centered text
        let title = Line::from(vec![
            Span::styled("欢迎来到 ", Style::default().fg(Color::Gray)),
            Span::styled(
                "V2EX",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]);
        let header = Paragraph::new(title).alignment(Alignment::Center).block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan)),
        );
        frame.render_widget(header, main_layout[0]);

        // Create table with styled mock data
        let header_cells = ["话题", "楼主", "评论数", "活跃时间"]
            .iter()
            .map(|h| h.bold().yellow());
        let header = Row::new(header_cells).height(1).bottom_margin(1);
        let selected_row_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_row_style_fg);

        // init rows from self.items
        let rows: Vec<Row> = self
            .items
            .iter()
            .map(|item| {
                Row::new(vec![
                    item.title.as_str().white(),
                    item.author.as_str().green(),
                    item.comment.as_str().cyan(),
                    item.updated.as_str().dark_gray(),
                ])
            })
            .collect();

        let table = Table::new(
            rows,
            [
                Constraint::Percentage(45),
                Constraint::Percentage(20),
                Constraint::Percentage(15),
                Constraint::Percentage(20),
            ],
        )
        .header(header)
        .block(
            Block::default()
                .title(" 全部话题 ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan)),
        )
        .widths([
            Constraint::Percentage(45),
            Constraint::Percentage(20),
            Constraint::Percentage(15),
            Constraint::Percentage(20),
        ])
        .column_spacing(1)
        .style(Style::default())
        .row_highlight_style(selected_row_style);

        if self.loading {
            let loading_text = Line::from(vec!["Loading...".yellow()]);
            let loading = Paragraph::new(loading_text)
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::Cyan)),
                );
            frame.render_widget(loading, main_layout[1]);
        } else {
            frame.render_stateful_widget(table, main_layout[1], &mut self.state);
        }
        // Render footer with help text
        let footer_text = Line::from(vec!["Press ".gray(), "q".cyan().bold(), " to quit".gray()]);
        let footer = Paragraph::new(footer_text)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Cyan)),
            );
        frame.render_widget(footer, main_layout[2]);
    }

    fn handle_event(&mut self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Char('r') => Action::FetchTopics,
            KeyCode::Char('t') => Action::Top,
            KeyCode::Char('b') => Action::Bottom,
            KeyCode::Up | KeyCode::Char('k') => Action::PreviousRow,
            KeyCode::Down | KeyCode::Char('j') => Action::NextRow,
            KeyCode::Enter => Action::Enter,
            _ => Action::Noop,
        }
    }

    fn update(&mut self, action: Action) {
        match action {
            Action::FetchTopics => {
                self.state.select_first();
                self.items = self.crawler.fetch_topics().unwrap();
            }
            Action::Top => self.state.select_first(),
            Action::Bottom => self.state.select_last(),
            Action::PreviousRow => self.state.select_previous(),
            Action::NextRow => self.state.select_next(),
            _ => {}
        }
    }
}
