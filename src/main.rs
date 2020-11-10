use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;
use std::time::Instant;

mod days;
mod puzzle;

#[macro_export]
macro_rules! err_exit {
    ($($arg:tt)*) => ({
        std::eprintln!($($arg)*);
        process::exit(1);
    })
}

#[macro_export]
macro_rules! usage_exit {
    ($($arg:tt)*) => ({
        std::eprintln!($($arg)*);
        std::eprintln!("usage: {} day", env::current_exe().unwrap().file_name().unwrap().to_str().unwrap());
        process::exit(1);
    })
}

fn get_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage_exit!("Missing mandatory argument 'day'");
    }
    let day = args[1]
        .parse::<u8>()
        .unwrap_or_else(|e| usage_exit!("Incorrect 'day' argument. {}", e));
    let input_filename = format!("input/day{}.txt", day);
    let puzzle =
        days::puzzle_factory(day).unwrap_or_else(|| err_exit!("No solution found for day {}", day));
    let lines = get_input(&input_filename)
        .unwrap_or_else(|e| err_exit!("Failed to open file: {}, {}", &input_filename, e));

    let now = Instant::now();
    let answer = puzzle.solve(lines);
    let elapsed = now.elapsed().as_millis();
    println!("First answer found: {}", answer.0);
    println!("Second answer found: {}", answer.1);
    println!("Execution time: {}", elapsed);
}
