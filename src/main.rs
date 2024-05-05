use app::App;
use color_eyre::Result;

mod app;
mod errors;
mod tui;
mod ui;
mod widgets;

fn main() -> Result<()> {
    errors::install_hooks()?;
    let mut terminal = tui::init()?;
    let app = &mut App::new();
    app.run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}
