use std::time::Duration;

use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Row, Table},
    Frame,
};

use crate::ui::{Dispatcher, Store};

use super::{action::Action, store};

pub fn render() -> Result<()> {
    let mut store = Store::new();
    let dispatcher = Dispatcher::new();
    let mut terminal = ratatui::init();
    let mut first_load = true;

    while store.running {
        terminal.draw(|f| render_home(&mut store, f))?;

        if first_load {
            first_load = false;
            store.update(Action::Reload);
        }

        handle_event(&mut store)?;
    }
    ratatui::restore();
    Ok(())
}

pub fn render_home(store: &mut Store, frame: &mut Frame) {
    let main_layout = Layout::default()
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
        .fg(store.home.colors.selected_row_style_fg);

    // init rows from self.items
    let rows: Vec<Row> = store
        .home
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

    frame.render_stateful_widget(table, main_layout[1], &mut store.home.state);

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

fn handle_event(store: &mut Store) -> Result<()> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => store.update(Action::Quit),
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    store.update(Action::Quit);
                }
                KeyCode::Char('r') => store.update(Action::Reload),
                KeyCode::Char('t') => store.update(Action::Top),
                KeyCode::Char('b') => store.update(Action::Bottom),
                KeyCode::Up | KeyCode::Char('k') => {
                    store.update(Action::PreviousRow);
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    store.update(Action::NextRow);
                }
                KeyCode::Enter => {
                    store.update(Action::Enter);
                }
                _ => {}
            }
        }
    }
    Ok(())
}
