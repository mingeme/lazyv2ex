use crate::{action::Action, api::Crawler, model::TopicDetail};

use super::{Page, PageType};
use crossterm::event::{Event, KeyCode, MouseEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
    Frame,
};

pub struct DetailPage {
    loading: bool,
    topic_detail: Option<TopicDetail>,
    crawler: Crawler,
}

impl DetailPage {
    pub fn new() -> Self {
        DetailPage {
            loading: true,
            topic_detail: None,
            crawler: Crawler::new(),
        }
    }
}

impl Page for DetailPage {
    fn page_type(&self) -> PageType {
        PageType::Detail
    }

    fn init(&mut self) -> Option<Action> {
        self.loading = true;
        None
    }

    fn render(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(frame.area());

        if self.loading {
            let loading_text = "Loading...";
            let paragraph = Paragraph::new(loading_text);
            frame.render_widget(paragraph, chunks[0]);
            return;
        }

        let mut lines = vec![];
        lines.push(Line::from(vec![
            Span::styled("话题：", Style::default().fg(Color::Gray)),
            Span::styled(
                &self.topic_detail.as_ref().unwrap().title,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled("楼主：", Style::default().fg(Color::Gray)),
            Span::styled(
                &self.topic_detail.as_ref().unwrap().author,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));

        lines.push(Line::from(vec![
            Span::styled("活跃时间：", Style::default().fg(Color::Gray)),
            Span::styled(
                &self.topic_detail.as_ref().unwrap().updated,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
        lines.push(Line::from(vec![Span::styled(
            "内容：",
            Style::default().fg(Color::Gray),
        )]));
        lines.push(Line::from(vec![Span::styled(
            &self.topic_detail.as_ref().unwrap().content,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]));

        lines.push(Line::from(vec![Span::styled(
            "评论：",
            Style::default().fg(Color::Gray),
        )]));

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

        let paragraph = Paragraph::new(lines).wrap(Wrap { trim: true });
        frame.render_widget(paragraph, chunks[0]);
    }

    fn handle_event(&mut self, event: Event) -> Option<Action> {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Esc | KeyCode::Backspace => Some(Action::GoHome),
                KeyCode::Char('o') => {
                    if let Some(detail) = self.topic_detail.as_ref() {
                        Some(Action::OpenBrowser(detail.link.clone()))
                    } else {
                        None
                    }
                }
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
            Action::GoHome => {
                self.loading = true;
                self.topic_detail = None;
                None
            }
            Action::FetchTopicDetail(url) => {
                self.loading = false;
                self.topic_detail = Some(self.crawler.fetch_topic_detail(&url).unwrap());
                None
            }
            Action::LineUp(count) => None,
            Action::LineDown(count) => None,
            _ => None,
        }
    }
}
