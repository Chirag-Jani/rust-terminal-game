use std::io::Stdout;

// HAS ALL THE STRUCTURES AND OUTLINES OF THE COMPONENTS IN THE GAME
//
pub struct ScreenShit {
    pub screen: Stdout,
    pub rows: u16,
    pub columns: u16,
}

pub struct World {
    pub player_row: u16,
    pub player_column: u16,
    pub high_score: u64,
}
pub struct Food {
    pub f_row: u16,
    pub f_col: u16,
    pub f_points: u64,
}

pub struct Player {
    pub body: String,
    pub curr_score: u64,
    pub curr_level: u64,
    pub high_score: u64,
    pub curr_direction: DIRECTION,
}

#[derive(Clone)]
pub enum DIRECTION {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
