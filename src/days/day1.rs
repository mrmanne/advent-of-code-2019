use crate::puzzle::{io, File, Puzzle};
pub struct Day1;

fn calc_fuel(mass: &u64) -> u64 {
    let tmp: u64 = mass / 3;
    if tmp > 2 {
        tmp - 2
    } else {
        0
    }
}

impl Day1 {
    fn solve_part1(&self, input: &Vec<u64>) -> String {
        let mut fuel: u64 = 0;
        for mass in input {
            fuel += calc_fuel(mass);
        }
        fuel.to_string()
    }

    fn solve_part2(&self, input: &Vec<u64>) -> String {
        let mut fuel: u64 = 0;
        for mass in input {
            let mut module_fuel = calc_fuel(mass);
            while module_fuel > 0 {
                fuel += module_fuel;
                module_fuel = calc_fuel(&module_fuel);
            }
        }
        fuel.to_string()
    }
}

impl Puzzle for Day1 {
    fn solve(&self, lines: io::Lines<io::BufReader<File>>) -> (String, String) {
        let numbers: Vec<u64> = lines.map(|l| l.unwrap().parse::<u64>().unwrap()).collect();
        return (self.solve_part1(&numbers), self.solve_part2(&numbers));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(Day1 {}.solve_part1(&vec!(12)), "2");
    }

    #[test]
    fn part1_example2() {
        assert_eq!(Day1 {}.solve_part1(&vec!(14)), "2");
    }

    #[test]
    fn part1_example3() {
        assert_eq!(Day1 {}.solve_part1(&vec!(1969)), "654");
    }

    #[test]
    fn part1_example4() {
        assert_eq!(Day1 {}.solve_part1(&vec!(100756)), "33583");
    }

    #[test]
    fn part2_example1() {
        assert_eq!(Day1 {}.solve_part2(&vec!(14)), "2");
    }

    #[test]
    fn part2_example2() {
        assert_eq!(Day1 {}.solve_part2(&vec!(1969)), "966");
    }

    #[test]
    fn part2_example3() {
        assert_eq!(Day1 {}.solve_part2(&vec!(100756)), "50346");
    }
}
