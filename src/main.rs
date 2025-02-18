use color_eyre::Result;

mod api;
mod model;
mod time;
mod ui;

fn main() -> Result<()> {
    color_eyre::install()?;
    ui::render()?;
    Ok(())
}
