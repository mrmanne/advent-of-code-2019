use crate::puzzle::Puzzle;
mod day1;
mod day2;

pub fn puzzle_factory(day: u8) -> Option<Box<dyn Puzzle>> {
    match day {
        1 => Some(Box::new(day1::Day1 {})),
        2 => Some(Box::new(day2::Day2 {})),
        _ => None,
    }
}
