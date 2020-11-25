use crate::puzzle::{io, File, Puzzle};
use std::string::String;
pub struct Day2;

impl Day2 {
    fn solve_part1(&self, mem: &Vec<i64>) -> Vec<i64> {
        let mut c = crate::computer::Computer::new(mem.clone());
        c.run_program(vec![0]);
        c.mem().clone()
    }

    fn solve_part2(&self, mem: &Vec<i64>) -> Result<String, &'static str> {
        for noun in 0..100 {
            for verb in 0..100 {
                let mut mem = mem.clone();
                mem[1] = noun;
                mem[2] = verb;
                let mut c = crate::computer::Computer::new(mem);
                c.run_program(vec![0]);
                if c.mem()[0] == 19690720 {
                    return Ok((100 * noun + verb).to_string());
                }
            }
        }
        Err("No solution found")
    }
}

impl Puzzle for Day2 {
    fn solve(&self, lines: io::Result<io::Lines<io::BufReader<File>>>) -> (String, String) {
        let mut mem: Vec<i64> = lines
            .expect("No input file")
            .map(|l| l.unwrap())
            .collect::<String>()
            .split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        mem[1] = 12;
        mem[2] = 2;
        return (
            self.solve_part1(&mem)[0].to_string(),
            self.solve_part2(&mem).unwrap(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(
            Day2 {}.solve_part1(&vec!(1, 0, 0, 0, 99)),
            vec!(2, 0, 0, 0, 99)
        );
    }

    #[test]
    fn part1_example2() {
        assert_eq!(
            Day2 {}.solve_part1(&vec!(2, 3, 0, 3, 99)),
            vec!(2, 3, 0, 6, 99)
        );
    }

    #[test]
    fn part1_example3() {
        assert_eq!(
            Day2 {}.solve_part1(&vec!(2, 4, 4, 5, 99, 0)),
            vec!(2, 4, 4, 5, 99, 9801)
        );
    }

    #[test]
    fn part1_example4() {
        assert_eq!(
            Day2 {}.solve_part1(&vec!(1, 1, 1, 4, 99, 5, 6, 0, 99)),
            vec!(30, 1, 1, 4, 2, 5, 6, 0, 99)
        );
    }
}
