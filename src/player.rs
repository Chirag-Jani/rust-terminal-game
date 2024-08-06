use crate::structs::{self, DIRECTION};

use structs::Player;

pub fn initialize_player() -> std::io::Result<Player> {
    let player = Player {
        body: ">".to_string(),
        curr_level: 1,
        curr_score: 0,
        high_score: 0,
        curr_direction: DIRECTION::RIGHT,
    };

    return Ok(player);
}
