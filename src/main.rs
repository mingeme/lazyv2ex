use std::time::Duration;

use action::Action;
use app::App;
use color_eyre::Result;
use crossterm::event::{self, Event};
use pages::PageType;

mod action;
mod api;
mod app;
mod model;
mod pages;
mod time;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();

    let mut app = App::new();
    terminal.draw(|f| app.render(f))?;
    let init_action = app.switch_page(PageType::Home);
    app.update(init_action);

    loop {
        terminal.draw(|f| app.render(f))?;
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key_event) = event::read()? {
                let action = app.handle_event(key_event);
                if action == Action::Quit {
                    break;
                }
                app.update(action);
            }
        }
    }
    ratatui::restore();
    Ok(())
}
