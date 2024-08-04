mod game;
mod painting;
pub mod player;
mod score_handler;
mod structs;
use crossterm::terminal::disable_raw_mode;
use game::play;

fn main() -> std::io::Result<()> {
    play()?;
    disable_raw_mode()?;
    Ok(())
}
