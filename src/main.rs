mod game;
mod options;
mod painting;
pub mod player;
mod score_handler;
mod structs;
use crossterm::terminal::disable_raw_mode;
use options::render_options;
use painting::paint_screen;

fn main() -> std::io::Result<()> {
    // drawing initial screen
    let screen_shit = paint_screen().unwrap();

    render_options(screen_shit.screen, screen_shit.rows, screen_shit.columns)?;
    disable_raw_mode()?;
    Ok(())
}
