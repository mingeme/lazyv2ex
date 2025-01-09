use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    symbols,
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, Row, Table},
    Frame,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(
    mut terminal: ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
) -> Result<()> {
    let quit = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(key) if key == quit) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Table
            Constraint::Length(3), // Footer
        ])
        .margin(1)
        .split(frame.size());

    // Render header with centered text
    let title = Line::from(vec![
        Span::styled("Welcome to ", Style::default().fg(Color::Gray)),
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
        .map(|h| h.clone().bold().yellow());
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = vec![
        Row::new(vec![
            "如何学习Rust语言？新手求指导".into(),
            "rustacean".green(),
            "42".cyan(),
            "2小时前".dark_gray(),
        ]),
        Row::new(vec![
            "推荐一个好用的代码编辑器".into(),
            "vimer".green(),
            "156".cyan(),
            "5小时前".dark_gray(),
        ]),
        Row::new(vec![
            "分享一个开源项目：TUI框架".into(),
            "opensource".green(),
            "89".cyan(),
            "1天前".dark_gray(),
        ]),
    ];

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
            .title(" 最新话题 ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Blue)),
    )
    .widths(&[
        Constraint::Percentage(45),
        Constraint::Percentage(20),
        Constraint::Percentage(15),
        Constraint::Percentage(20),
    ])
    .column_spacing(1)
    .style(Style::default())
    .highlight_style(Style::default().bold());

    frame.render_widget(table, main_layout[1]);

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
