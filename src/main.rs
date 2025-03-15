use std::time::Duration;

use action::Action;
use app::App;
use clap::Parser;
use color_eyre::Result;
use crossterm::event::{self};
use pages::PageType;

mod action;
mod api;
mod app;
mod model;
mod pages;
mod time;

/// A command-line client for v2ex
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {}

fn main() -> Result<()> {
    color_eyre::install()?;

    let _ = Args::parse();

    let mut terminal = ratatui::init();

    let mut app = App::new();
    terminal.draw(|f| app.render(f))?;
    let init_action = app.switch_page(PageType::Home);
    app.update(init_action.unwrap());

    loop {
        terminal.draw(|f| app.render(f))?;

        if event::poll(Duration::from_millis(250))? {
            let event = event::read()?;
            let action = app.handle_event(event);
            if let Some(Action::Quit) = action {
                break;
            }
            if action.is_none() {
                continue;
            }

            let mut current_action = action.unwrap();

            while let Some(next_action) = app.update(current_action) {
                terminal.draw(|f| app.render(f))?;
                current_action = next_action;
            }
        }
    }
    ratatui::restore();
    Ok(())
}
