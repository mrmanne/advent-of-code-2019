use crate::puzzle::Puzzle;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

pub fn puzzle_factory(day: u8) -> Option<Box<dyn Puzzle>> {
    match day {
        1 => Some(Box::new(day1::Day1 {})),
        2 => Some(Box::new(day2::Day2 {})),
        3 => Some(Box::new(day3::Day3 {})),
        4 => Some(Box::new(day4::Day4 {})),
        5 => Some(Box::new(day5::Day5 {})),
        _ => None,
    }
}
