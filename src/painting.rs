use crossterm::{
    cursor::MoveTo,
    style::Print,
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use std::io::{stdout, Result, Stdout};

pub fn paint_screen() -> std::io::Result<Stdout> {
    let screen = stdout();
    Ok(screen)
}

pub fn draw_playing_area(screen: &mut Stdout, row: u16, col: u16) -> Result<()> {
    screen.queue(MoveTo(0, 0))?;
    screen.queue(Clear(ClearType::All))?;
    for i in 0..row * 10 {
        screen.queue(Print("-"))?;
        screen.queue(MoveTo(i, row))?;
    }
    for i in 0..row * 10 {
        screen.queue(Print("-"))?;
        screen.queue(MoveTo(i, 0))?;
    }
    for i in 0..col {
        screen.queue(Print("|"))?;
        screen.queue(MoveTo(0, i))?;
    }
    for i in 0..row * 10 {
        screen.queue(Print("|"))?;
        screen.queue(MoveTo(col, i))?;
    }
    Ok(())
}