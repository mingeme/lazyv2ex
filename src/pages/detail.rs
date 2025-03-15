use crate::{action::Action, api::Crawler, model::TopicDetail};

use super::{Page, PageType};
use crossterm::event::{Event, KeyCode, MouseEventKind};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub struct DetailPage {
    loading: bool,
    topic_detail: Option<TopicDetail>,
    crawler: Crawler,
    max_scroll: u16,
    scroll: u16,
}

impl DetailPage {
    pub fn new() -> Self {
        DetailPage {
            loading: true,
            topic_detail: None,
            crawler: Crawler::new(),
            max_scroll: 0,
            scroll: 0,
        }
    }
}

impl Page for DetailPage {
    fn page_type(&self) -> PageType {
        PageType::Detail
    }

    fn init(&mut self) -> Option<Action> {
        self.loading = true;
        self.topic_detail = None;
        self.scroll = 0;
        None
    }

    fn render(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(frame.area());

        if self.loading {
            let loading_text = "Loading...";
            let paragraph = Paragraph::new(loading_text);
            frame.render_widget(paragraph, chunks[0]);
            return;
        }

        let detail = self.topic_detail.as_ref().unwrap();
        let bold_cyan = Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD);
        let gray = Style::default().fg(Color::Gray);
        
        let mut lines = vec![
            Line::from(vec![
                Span::styled("话题：", gray),
                Span::styled(&detail.title, bold_cyan),
            ]),
            Line::from(vec![
                Span::styled("楼主：", gray),
                Span::styled(&detail.author, bold_cyan),
            ]),
            Line::from(vec![
                Span::styled("活跃时间：", gray),
                Span::styled(&detail.updated, bold_cyan),
            ]),
            Line::from(vec![Span::styled("内容：", gray)]),
            Line::from(vec![Span::styled(&detail.content, bold_cyan)]),
            Line::from(vec![Span::styled("评论：", gray)]),
        ];

        self.topic_detail
            .as_ref()
            .unwrap()
            .replies
            .iter()
            .for_each(|reply| {
                lines.push(Line::from(vec![Span::styled(
                    "",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )]));
                lines.push(Line::from(vec![Span::styled(
                    format!(">>> {} 回复于 {}:", reply.author, reply.time),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )]));
                lines.push(Line::from(vec![Span::styled(
                    &reply.content,
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )]));
            });

        let area = chunks[0];
        let text = ratatui::text::Text::from(lines);

        let paragraph = Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .scroll((self.scroll, 0));
        let line_count = paragraph.line_count(area.width) as u16;
        self.max_scroll = line_count.saturating_sub(area.height);
        frame.render_widget(paragraph, area);

        // Render footer with help text
        let footer_text = Line::from(vec![
            "退出：q｜返回：Esc/Backspace｜滚动：↑↓jk｜移到顶部：t｜移到底部：b｜浏览器打开：o"
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
        frame.render_widget(footer, chunks[1]);
    }

    fn handle_event(&mut self, event: Event) -> Option<Action> {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Esc | KeyCode::Backspace => Some(Action::GoHome),
                KeyCode::Char('o') => self
                    .topic_detail
                    .as_ref()
                    .map(|detail| Action::OpenBrowser(detail.link.clone())),
                KeyCode::Char('t') => Some(Action::Top),
                KeyCode::Char('b') => Some(Action::Bottom),
                KeyCode::Up | KeyCode::Char('k') => Some(Action::LineUp(3)),
                KeyCode::Down | KeyCode::Char('j') => Some(Action::LineDown(3)),

                _ => None,
            },
            Event::Mouse(mouse_event) => match mouse_event.kind {
                MouseEventKind::ScrollUp => Some(Action::LineUp(3)),
                MouseEventKind::ScrollDown => Some(Action::LineDown(3)),
                _ => None,
            },
            _ => None,
        }
    }

    fn update(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::GoHome => self.init(),
            Action::FetchTopicDetail(url) => {
                self.loading = false;
                self.topic_detail = Some(self.crawler.fetch_topic_detail(&url).unwrap());
                None
            }
            Action::Top => {
                self.scroll = 0;
                None
            }
            Action::Bottom => {
                self.scroll = self.max_scroll;
                None
            }
            Action::LineUp(count) => {
                self.scroll = self.scroll.saturating_sub(count);
                None
            }
            Action::LineDown(count) => {
                self.scroll = self.scroll.saturating_add(count).min(self.max_scroll);
                None
            }
            _ => None,
        }
    }
}
