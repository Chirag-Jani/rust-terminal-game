use std::{
    fs::{File, OpenOptions},
    io::{stdout, BufRead, BufReader, Stdout, Write},
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

struct World {
    player_row: u16,
    player_column: u16,
    high_score: u16,
}
struct Food {
    f_row: u16,
    f_col: u16,
}

#[derive(Clone)]
enum DIRECTION {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn init_game(screen: &mut Stdout, row: u16, col: u16) -> std::io::Result<()> {
    screen.queue(MoveTo(0, 0))?;
    screen.queue(Clear(ClearType::All))?;
    screen.queue(Print("Starting..."))?;
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

fn draw_screen(
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

fn clear_at_line(screen: &mut Stdout, row: u16, col: u16) -> std::io::Result<()> {
    screen.queue(MoveTo(col, row))?;
    screen.queue(Print(" "))?;
    Ok(())
}

fn game_over(
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

fn spawn_food(screen: &mut Stdout, row: u16, col: u16, food: &mut Food) -> std::io::Result<()> {
    let r_cor = rand::thread_rng().gen_range(3..row - 1);
    let c_cor = rand::thread_rng().gen_range(3..col - 1);
    screen.queue(MoveTo(c_cor, r_cor))?;
    screen.queue(Print("*"))?;
    screen.flush()?;
    food.f_col = c_cor;
    food.f_row = r_cor;
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

fn save_high_score(score: u16) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("high_score.txt")?;
    writeln!(file, "{}", score)?;
    Ok(())
}

fn load_high_score() -> u16 {
    if let Ok(file) = File::open("high_score.txt") {
        let reader = BufReader::new(file);
        if let Some(Ok(line)) = reader.lines().next() {
            if let Ok(score) = line.parse::<u16>() {
                return score;
            }
        }
    }
    0
}

fn main() -> std::io::Result<()> {
    let mut screen = stdout();

    screen.execute(Hide)?;

    enable_raw_mode()?;

    let (columns, rows) = size().unwrap();

    let mut world = World {
        player_column: 1,
        player_row: 1,
        high_score: load_high_score(),
    };

    let mut scored = 0;
    let mut player_speed = 80;
    let mut current_level = 1;

    let player = String::from("8");
    let mut player_direction = DIRECTION::RIGHT;

    let mut food = Food { f_col: 0, f_row: 0 };

    init_game(&mut screen, rows, columns)?;

    spawn_food(&mut screen, rows, columns, &mut food).unwrap();

    let mut player_score = 0;
    screen.queue(MoveTo(columns - 11, 1))?;
    screen.queue(Print("Score:"))?;
    screen.queue(Print(player_score))?;
    screen.queue(MoveTo(columns - 16, 3))?;
    screen.queue(Print("High Score:"))?;
    screen.queue(Print(world.high_score))?;

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

        draw_screen(
            &mut screen,
            &mut world,
            player.clone(),
            rows,
            columns,
            player_direction.clone(),
            player_score,
        )?;
    }

    screen.execute(Show)?;
    disable_raw_mode()?;

    Ok(())
}
