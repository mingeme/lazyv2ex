use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{palette::tailwind, Color, Modifier, Style, Stylize},
    symbols,
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, Row, Table, TableState},
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

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

struct Data {
    topic: String,
    author: String,
    comment: String,
    time: String,
}

struct App {
    state: TableState,
    items: Vec<Data>,
    colors: TableColors,
}

impl App {
    fn new() -> Self {
        let items = vec![
            Data {
                topic: "如何学习Rust语言？新手求指导".into(),
                author: "rustacean".into(),
                comment: "42".into(),
                time: "2小时前".into(),
            },
            Data {
                topic: "推荐一个好用的代码编辑器".into(),
                author: "vimer".into(),
                comment: "156".into(),
                time: "5小时前".into(),
            },
            Data {
                topic: "分享一个开源项目：TUI框架".into(),
                author: "opensource".into(),
                comment: "89".into(),
                time: "1天前".into(),
            },
        ];
        Self {
            state: TableState::default().with_selected(0),
            colors: TableColors::new(&tailwind::BLUE),
            items,
        }
    }

    pub fn next_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    i
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn prev_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    0
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|f| self.render(f))?;
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Up | KeyCode::Char('k') => self.prev_row(),
                    KeyCode::Down | KeyCode::Char('j') => self.next_row(),
                    _ => {}
                }
            }
        }
    }

    fn render(&mut self, frame: &mut Frame) {
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
            .fg(self.colors.selected_row_style_fg);

        // init rows from self.items
        let rows: Vec<Row> = self
            .items
            .iter()
            .map(|item| {
                Row::new(vec![
                    Span::raw(item.topic.clone()),
                    Span::raw(item.author.clone()).green(),
                    Span::raw(item.comment.clone()).cyan(),
                    Span::raw(item.time.clone()).dark_gray(),
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
                .title(" 最新话题 ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan)),
        )
        .widths(&[
            Constraint::Percentage(45),
            Constraint::Percentage(20),
            Constraint::Percentage(15),
            Constraint::Percentage(20),
        ])
        .column_spacing(1)
        .style(Style::default())
        .row_highlight_style(selected_row_style);

        frame.render_stateful_widget(table, main_layout[1], &mut self.state);

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
}
