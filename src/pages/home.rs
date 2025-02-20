use crate::{action::Action, api::Crawler, model::Topic};

use super::{Page, PageType};
use crossterm::event::{Event, KeyCode};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{palette::tailwind, Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Row, Table, TableState},
    Frame,
};

pub struct TableColors {
    pub selected_row_style_fg: Color,
}

impl TableColors {
    const fn new(color: &tailwind::Palette) -> Self {
        Self {
            selected_row_style_fg: color.c400,
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

    fn init(&mut self) -> Option<Action> {
        Some(Action::FetchTopics)
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
                Constraint::Percentage(70),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(15),
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
        let footer_text = Line::from(vec![
            "退出：q｜滚动：↑↓jk｜移到顶部：t｜移到底部：b｜查看：Enter"
                .cyan()
                .bold(),
        ]);
        let footer = Paragraph::new(footer_text)
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Cyan)),
            );
        frame.render_widget(footer, main_layout[2]);
    }

    fn handle_event(&mut self, event: Event) -> Option<Action> {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('r') => Some(Action::Reload),
                KeyCode::Char('t') => Some(Action::Top),
                KeyCode::Char('b') => Some(Action::Bottom),
                KeyCode::Up | KeyCode::Char('k') => Some(Action::PreviousRow),
                KeyCode::Down | KeyCode::Char('j') => Some(Action::NextRow),
                KeyCode::Enter => Some(Action::Enter),
                _ => None,
            },
            _ => None,
        }
    }

    fn update(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::Reload => {
                self.loading = true;
                self.state.select_first();
                Some(Action::FetchTopics)
            }
            Action::FetchTopics => {
                self.loading = false;
                self.state.select_first();
                self.items = self.crawler.fetch_topics().unwrap();
                None
            }
            Action::Top => {
                self.state.select_first();
                None
            }
            Action::Bottom => {
                self.state.select_last();
                None
            }
            Action::PreviousRow => {
                self.state.select_previous();
                None
            }
            Action::NextRow => {
                self.state.select_next();
                None
            }
            Action::Enter => {
                if let Some(index) = self.state.selected() {
                    if let Some(item) = self.items.get(index) {
                        return Some(Action::FetchTopicDetail(item.link.clone()));
                    }
                }
                None
            }
            _ => None,
        }
    }
}
