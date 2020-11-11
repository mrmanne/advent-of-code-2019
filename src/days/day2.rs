use crate::puzzle::{io, File, Puzzle};
use std::string::String;
pub struct Day2;

enum OpCode {
    ADD { a: usize, b: usize, c: usize },
    MUL { a: usize, b: usize, c: usize },
    HALT,
}

struct Computer {
    pc: usize,
    mem: Vec<usize>,
}

impl Computer {
    pub fn new(mem: Vec<usize>) -> Self {
        Self { pc: 0, mem: mem }
    }
    fn decode_instruction(&self) -> Result<OpCode, &'static str> {
        match self.mem[self.pc] {
            1 => Ok(OpCode::ADD {
                a: self.mem[self.pc + 1],
                b: self.mem[self.pc + 2],
                c: self.mem[self.pc + 3],
            }),
            2 => Ok(OpCode::MUL {
                a: self.mem[self.pc + 1],
                b: self.mem[self.pc + 2],
                c: self.mem[self.pc + 3],
            }),
            99 => Ok(OpCode::HALT),
            _ => Err("Illegal opcode"),
        }
    }
    fn execute_instruction(&mut self) -> bool {
        let opcode = self.decode_instruction().unwrap();
        match opcode {
            OpCode::ADD { a, b, c } => {
                self.mem[c] = self.mem[a] + self.mem[b];
                self.pc += 4;
                false
            }
            OpCode::MUL { a, b, c } => {
                self.mem[c] = self.mem[a] * self.mem[b];
                self.pc += 4;
                false
            }
            OpCode::HALT => true,
        }
    }
    fn run_program(&mut self) {
        while !self.execute_instruction() {}
    }
}

impl Day2 {
    fn solve_part1(&self, mem: &Vec<usize>) -> Vec<usize> {
        let mut c = Computer::new(mem.clone());
        c.run_program();
        c.mem.clone()
    }

    fn solve_part2(&self, mem: &Vec<usize>) -> Result<String, &'static str> {
        for noun in 0..100 {
            for verb in 0..100 {
                let mut c = Computer::new(mem.clone());
                c.mem[1] = noun;
                c.mem[2] = verb;
                c.run_program();
                if c.mem[0] == 19690720 {
                    return Ok((100 * noun + verb).to_string());
                }
            }
        }
        Err("No solution found")
    }
}

impl Puzzle for Day2 {
    fn solve(&self, lines: io::Lines<io::BufReader<File>>) -> (String, String) {
        let mut mem: Vec<usize> = lines
            .map(|l| l.unwrap())
            .collect::<String>()
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
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
