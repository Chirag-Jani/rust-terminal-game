use std::{
    io::{stdout, Stdout, Write},
    process::exit,
    time::Duration,
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, KeyCode},
    style::{Color, Print, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};
use rand::Rng;

use crate::{
    painting::{draw_playing_area, paint_screen},
    player::initialize_player,
    score_handler::{load_high_score, save_high_score},
    structs::{Food, Player, World, DIRECTION},
};

fn clear_at_line(screen: &mut Stdout, row: u16, col: u16) -> std::io::Result<()> {
    screen.queue(MoveTo(col, row))?;
    screen.queue(Print(" "))?;
    Ok(())
}

pub fn spawn_food(screen: &mut Stdout, row: u16, col: u16, food: &mut Food) -> std::io::Result<()> {
    let r_cor = rand::thread_rng().gen_range(3..row - 1);
    let c_cor = rand::thread_rng().gen_range(3..col - 1);
    screen.queue(MoveTo(c_cor, r_cor))?;
    screen.queue(Print("*"))?;
    screen.flush()?;
    food.f_col = c_cor;
    food.f_row = r_cor;
    Ok(())
}

pub fn play() -> std::io::Result<()> {
    // drawing initial screen
    let mut screen = paint_screen().unwrap_or(stdout());

    // hids the cursor while playing
    screen.execute(Hide)?;

    // forgot it's use case
    enable_raw_mode()?;

    // get the size of the terminal
    let (columns, rows) = size().unwrap();

    // world shit
    let mut world: World = World {
        player_column: 1,
        player_row: 1,
        high_score: load_high_score(),
    };

    // player info
    let player_data: Player = initialize_player().unwrap();
    let player: String = player_data.body;
    let mut player_direction: DIRECTION = player_data.curr_direction;
    let mut scored: u64 = 0;
    let mut player_speed: u64 = 80;
    let mut current_level: u64 = 1;

    // food shit
    let mut food = Food {
        f_col: 0,
        f_row: 0,
        f_size: 0,
        f_points: 0,
    };

    // drawing initial player area (border shit)
    draw_playing_area(&mut screen, rows, columns).expect("Error drawing Player Area");

    spawn_food(&mut screen, rows, columns, &mut food).unwrap();

    let mut player_score = 0;
    'game: loop {
        if poll(Duration::from_millis(player_speed))? {
            let reading = read().unwrap();
            match reading {
                crossterm::event::Event::Key(key_event) => match key_event.code {
                    KeyCode::Char('q') => {
                        screen.queue(MoveTo(columns / 3, rows / 2))?;
                        screen.queue(Print("\nThank you. Game Over.\n"))?;
                        screen.queue(MoveTo(columns / 3, rows / 2 + 1))?;
                        screen.queue(Print("\nYour Score: "))?;
                        screen.queue(Print(player_score))?;
                        screen.queue(MoveTo(0, 0))?;
                        break 'game;
                    }
                    KeyCode::Char('w') => {
                        player_direction = DIRECTION::UP;
                    }
                    KeyCode::Char('s') => {
                        player_direction = DIRECTION::DOWN;
                    }
                    KeyCode::Char('a') => {
                        player_direction = DIRECTION::LEFT;
                    }
                    KeyCode::Char('d') => {
                        player_direction = DIRECTION::RIGHT;
                    }
                    KeyCode::Enter => {
                        break 'game;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        screen.queue(MoveTo(columns - 11, 1))?;
        screen.queue(Print("Score:"))?;
        screen.queue(Print(player_score))?;
        screen.queue(MoveTo(columns - 11, 2))?;
        screen.queue(Print("Level:"))?;
        screen.queue(Print(current_level))?;
        screen.queue(MoveTo(columns - 16, 3))?;
        screen.queue(Print("High Score:"))?;
        screen.queue(Print(world.high_score))?;

        eat(
            &mut screen,
            &world,
            rows,
            columns,
            &mut food,
            &mut player_score,
            &mut scored,
            &mut player_speed,
            &mut current_level,
        )?;

        update_screen(
            &mut screen,
            &mut world,
            player.clone(),
            rows,
            columns,
            player_direction.clone(),
            player_score,
        )?;
    }

    Ok(())
}

fn eat(
    screen: &mut Stdout,
    world: &World,
    row: u16,
    col: u16,
    food: &mut Food,
    user_score: &mut u16,
    scored: &mut u64,
    player_speed: &mut u64,
    level: &mut u64,
) -> std::io::Result<()> {
    if world.player_column == food.f_col && world.player_row == food.f_row {
        spawn_food(screen, row, col, food)?;
        *user_score += 1;
        screen.flush()?;

        if *scored == 3 && *player_speed > 10 {
            screen.queue(MoveTo(col / 3, row / 2))?;
            *player_speed -= 10;
            *scored = 0;
            *level += 1;
        } else {
            *scored += 1;
        }
    }
    Ok(())
}

pub fn update_screen(
    screen: &mut Stdout,
    world: &mut World,
    player: String,
    rows: u16,
    columns: u16,
    to_where: DIRECTION,
    player_score: u16,
) -> std::io::Result<()> {
    clear_at_line(screen, world.player_row, world.player_column)?;
    match to_where {
        DIRECTION::UP => {
            if world.player_row > 1 {
                world.player_row -= 1;
            } else {
                if player_score > world.high_score {
                    world.high_score = player_score;
                }
                game_over(screen, columns, rows, player_score)?;
            }
            clear_at_line(screen, world.player_row, world.player_column)?;
        }
        DIRECTION::DOWN => {
            if world.player_row < rows - 2 {
                world.player_row += 1;
            } else {
                if player_score > world.high_score {
                    world.high_score = player_score;
                }
                game_over(screen, columns, rows, player_score)?;
            }
        }
        DIRECTION::RIGHT => {
            if world.player_column < columns - 2 {
                world.player_column += 1;
            } else {
                if player_score > world.high_score {
                    world.high_score = player_score;
                }
                game_over(screen, columns, rows, player_score)?;
            }
        }
        DIRECTION::LEFT => {
            if world.player_column > 2 {
                world.player_column -= 1;
            } else {
                if player_score > world.high_score {
                    world.high_score = player_score;
                }
                game_over(screen, columns, rows, player_score)?;
            }
        }
    }
    screen.queue(MoveTo(world.player_column, world.player_row))?;
    screen.queue(Print(player.with(Color::Red)))?;

    screen.flush()?;
    Ok(())
}

pub fn game_over(
    screen: &mut Stdout,
    columns: u16,
    rows: u16,
    player_score: u16,
) -> std::io::Result<()> {
    screen.queue(Clear(ClearType::All))?;
    screen.queue(MoveTo(columns / 3, rows / 2))?;
    screen.queue(Print("\nThank you. Game Over.\n"))?;
    screen.queue(MoveTo(columns / 3 + 3, rows / 2 + 1))?;
    screen.queue(Print("\nYour Score: "))?;
    screen.queue(Print(player_score))?;
    screen.queue(MoveTo(0, 0))?;
    screen.execute(Show)?;
    disable_raw_mode()?;
    save_high_score(player_score)?;
    exit(0);
}
