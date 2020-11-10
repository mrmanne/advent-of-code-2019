pub use std::fs::File;
pub use std::io::{self, BufRead};

pub trait Puzzle {
    fn solve(&self, lines: io::Lines<io::BufReader<File>>) -> (String, String);
}
