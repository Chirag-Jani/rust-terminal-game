use dialoguer::Select;
use std::io::{Result, Stdout};

use crate::game::play;

pub fn render_options(screen: Stdout, row: u16, col: u16) -> Result<()> {
    let items = vec!["1. Play", "2. Exit"];

    let selection = Select::new()
        .with_prompt("Choose Option:")
        .items(&items)
        .default(0)
        .interact()
        .unwrap();

    if selection == 0 {
        play(screen, row, col)?;
    }
    println!("You chose: {}", items[selection]);
    Ok(())
}
