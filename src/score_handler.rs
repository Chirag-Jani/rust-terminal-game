use std::io::Write;
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader},
};

pub fn save_high_score(score: u16) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("high_score.txt")?;
    writeln!(file, "{}", score)?;
    Ok(())
}

pub fn load_high_score() -> u16 {
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
