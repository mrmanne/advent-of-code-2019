use crate::computer::Computer;
use crate::puzzle::{io, File, Puzzle};
use std::string::String;
pub struct Day7;

fn max_thrust(settings_left: Vec<i64>, selected_settings: Vec<i64>, program: &Vec<i64>) -> i64 {
    if settings_left.is_empty() {
        let mut computers: Vec<Computer> = vec![];
        let mut input = 0;
        let mut done = false;
        for phase in &selected_settings {
            computers.push(Computer::new(program.clone()));
            let c = computers.last_mut().unwrap();
            let (halted, output) = c.run_until_output(vec![*phase, input]);
            input = output;
            done = halted;
        }
        while !done {
            for c in &mut computers {
                let (halted, output) = c.run_until_output(vec![input]);
                input = output;
                done = halted;
            }
        }
        input
    } else {
        let mut max = std::i64::MIN;
        for phase in &settings_left {
            let mut new_settings_left = settings_left.clone();
            new_settings_left.remove(new_settings_left.iter().position(|x| *x == *phase).unwrap());
            let mut new_selected_settings = selected_settings.clone();
            new_selected_settings.push(*phase);
            let thrust = max_thrust(new_settings_left, new_selected_settings, program);
            if thrust > max {
                max = thrust;
            }
        }
        max
    }
}

impl Day7 {
    fn solve_part1(&self, program: &Vec<i64>) -> i64 {
        max_thrust(vec![0, 1, 2, 3, 4], vec![], &program)
    }
    fn solve_part2(&self, program: &Vec<i64>) -> i64 {
        max_thrust(vec![5, 6, 7, 8, 9], vec![], &program)
    }
}

impl Puzzle for Day7 {
    fn solve(&self, lines: io::Result<io::Lines<io::BufReader<File>>>) -> (String, String) {
        let program: Vec<i64> = lines
            .expect("No input file")
            .map(|l| l.unwrap())
            .collect::<String>()
            .split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        return (
            self.solve_part1(&program).to_string(),
            self.solve_part2(&program).to_string(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let program = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(Day7 {}.solve_part1(&program), 43210);
    }

    #[test]
    fn part1_example2() {
        let program = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        assert_eq!(Day7 {}.solve_part1(&program), 54321);
    }

    #[test]
    fn part1_example3() {
        let program = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        assert_eq!(Day7 {}.solve_part1(&program), 65210);
    }

    #[test]
    fn part2_example1() {
        let program = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        assert_eq!(Day7 {}.solve_part2(&program), 139629729);
    }

    #[test]
    fn part2_example2() {
        let program = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        assert_eq!(Day7 {}.solve_part2(&program), 18216);
    }
}
