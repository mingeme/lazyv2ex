use std::time::Duration;

use api::Crawler;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction},
    prelude::*,
    style::{palette::tailwind, Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Row, Table, TableState},
    Frame,
};

mod api;
mod models;
mod time;

use models::Topic;

struct TableColors {
    buffer_bg: Color,
    header_bg: Color,
    header_fg: Color,
    row_fg: Color,
    selected_row_style_fg: Color,
    selected_column_style_fg: Color,
    selected_cell_style_fg: Color,
    normal_row_color: Color,
    alt_row_color: Color,
    footer_border_color: Color,
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

struct App {
    crawler: Crawler,
    running: bool,
    state: TableState,
    items: Vec<Topic>,
    colors: TableColors,
}

#[derive(PartialEq)]
enum Message {
    PreviousRow,
    NextRow,
    Top,
    Bottom,
    Enter,
    Reload,
    Quit,
}

impl App {
    fn new() -> Self {
        Self {
            crawler: Crawler::new(),
            running: true,
            state: TableState::default(),
            colors: TableColors::new(&tailwind::BLUE),
            items: vec![],
        }
    }
}

fn update(app: &mut App, msg: Message) -> Option<Message> {
    match msg {
        Message::PreviousRow => app.state.select_previous(),
        Message::NextRow => app.state.select_next(),
        Message::Reload => {
            app.state.select(Some(0));
            app.items = app.crawler.fetch_topics().unwrap();
        }
        Message::Top => app.state.select_first(),
        Message::Bottom => app.state.select_last(),
        Message::Enter => {}
        Message::Quit => app.running = false,
    }
    None
}

fn view(app: &mut App, frame: &mut Frame) {
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
        .fg(app.colors.selected_row_style_fg);

    // init rows from self.items
    let rows: Vec<Row> = app
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

    frame.render_stateful_widget(table, main_layout[1], &mut app.state);

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

fn handle_event() -> Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(Some(Message::Quit)),
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Ok(Some(Message::Quit))
                }
                KeyCode::Char('r') => return Ok(Some(Message::Reload)),
                KeyCode::Char('t') => return Ok(Some(Message::Top)),
                KeyCode::Char('b') => return Ok(Some(Message::Bottom)),
                KeyCode::Up | KeyCode::Char('k') => {
                    return Ok(Some(Message::PreviousRow));
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    return Ok(Some(Message::NextRow));
                }
                KeyCode::Enter => {
                    return Ok(Some(Message::Enter));
                }
                _ => {}
            }
        }
    }
    Ok(None)
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let mut app = App::new();
    let mut first_load = true;

    while app.running {
        terminal.draw(|f| view(&mut app, f))?;

        if first_load {
            first_load = false;
            update(&mut app, Message::Reload);
        }

        let mut msg = handle_event()?;

        while msg.is_some() {
            msg = update(&mut app, msg.unwrap());
        }
    }

    ratatui::restore();
    Ok(())
}
