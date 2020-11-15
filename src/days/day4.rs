use crate::puzzle::{io, File, Puzzle};
use std::string::String;

pub struct Day4;

fn is_valid_password_part1(password: usize, password_len: usize) -> bool {
    let mut prev: u32 = 0;
    let mut duplicate: bool = false;
    for (i, c) in format!("{num:0>width$}", num = password, width = password_len)
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .enumerate()
    {
        if i > 0 {
            if c < prev {
                return false;
            } else if c == prev {
                duplicate = true;
            }
        }
        prev = c;
    }
    duplicate
}

fn is_valid_password_part2(password: usize, password_len: usize) -> bool {
    let mut prev: u32 = 0;
    let mut duplicate: bool = false;
    let mut duplicate_count: usize = 0;
    for (i, c) in format!("{num:0>width$}", num = password, width = password_len)
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .enumerate()
    {
        if i > 0 {
            if c < prev {
                return false;
            } else if c == prev {
                if duplicate_count == 0 {
                    duplicate_count = 2;
                } else {
                    duplicate_count += 1;
                }
            } else {
                if duplicate_count == 2 {
                    duplicate = true;
                }
                duplicate_count = 0;
            }
        }
        prev = c;
    }
    if duplicate_count == 2 {
        duplicate = true;
    }
    duplicate
}

fn get_combinations(min: usize, max: usize, is_valid_pwd_fn: fn(usize, usize) -> bool) -> usize {
    let mut nof_passwords: usize = 0;
    for i in min..max + 1 {
        if is_valid_pwd_fn(i, max.to_string().len()) {
            nof_passwords += 1;
        }
    }
    nof_passwords
}

impl Day4 {
    fn solve_part1(&self, min: usize, max: usize) -> usize {
        get_combinations(min, max, is_valid_password_part1)
    }

    fn solve_part2(&self, min: usize, max: usize) -> usize {
        get_combinations(min, max, is_valid_password_part2)
    }
}

impl Puzzle for Day4 {
    fn solve(&self, _lines: io::Result<io::Lines<io::BufReader<File>>>) -> (String, String) {
        (
            self.solve_part1(108457, 562041).to_string(),
            self.solve_part2(108457, 562041).to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(Day4 {}.solve_part1(0, 99), 10);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(Day4 {}.solve_part1(0, 999), 100);
    }

    #[test]
    fn part1_example3() {
        assert_eq!(Day4 {}.solve_part1(111111, 111111), 1);
    }

    #[test]
    fn part1_example4() {
        assert_eq!(Day4 {}.solve_part1(223450, 223450), 0);
    }

    #[test]
    fn part1_example5() {
        assert_eq!(Day4 {}.solve_part1(123789, 123789), 0);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(Day4 {}.solve_part2(112233, 112233), 1);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(Day4 {}.solve_part2(123444, 123444), 0);
    }

    #[test]
    fn part2_example3() {
        assert_eq!(Day4 {}.solve_part2(111122, 111122), 1);
    }
}
