use std::{
    io::{stdout, Stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, KeyCode},
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};

struct World {
    player_row: u16,
    player_column: u16,
}

fn draw_screen(screen: &mut Stdout, world: &World) -> std::io::Result<()> {
    screen.queue(MoveTo(world.player_column, world.player_row))?;
    screen.queue(Print("P"))?;
    screen.flush()?;
    Ok(())
}
fn clear_at_line(screen: &mut Stdout, row: u16, col: u16) -> std::io::Result<()> {
    screen.queue(MoveTo(col, row))?;
    screen.queue(Print("Hi"))?;
    screen.queue(Clear(ClearType::CurrentLine))?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    // setup screen
    let mut screen = stdout();

    // hide cursor
    screen.execute(Hide)?;

    // read about this shit
    enable_raw_mode()?;

    // getting the screensize
    let (columns, rows) = size().unwrap();

    // setup world
    let mut world = World {
        player_column: columns / 5,
        player_row: rows - 1,
    };

    // game loop
    'game: loop {
        // read input
        if poll(Duration::from_millis(500))? {
            let reading = read().unwrap();
            match reading {
                crossterm::event::Event::Key(key_event) => match key_event.code {
                    KeyCode::Char('q') => {
                        break 'game;
                    }
                    KeyCode::Char('w') => {
                        clear_at_line(&mut screen, world.player_row, world.player_column)?;
                        world.player_row -= 1;
                    }
                    KeyCode::Char('s') => {
                        clear_at_line(&mut screen, world.player_row, world.player_column)?;
                        world.player_row += 1;
                    }
                    KeyCode::Char('a') => {
                        clear_at_line(&mut screen, world.player_row, world.player_column)?;
                        world.player_column -= 1;
                    }
                    KeyCode::Char('d') => {
                        clear_at_line(&mut screen, world.player_row, world.player_column)?;
                        world.player_column += 1;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        // do physics / calculation
        // draw screen
        draw_screen(&mut screen, &world)?;
    }
    screen.execute(Show)?;
    disable_raw_mode()?;
    Ok(())
}
