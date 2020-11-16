use std::convert::TryInto;

const PARAM_MODE_POSITION: usize = 0;
const PARAM_MODE_IMMEDIATE: usize = 1;

enum OpCode {
    Add { a: i64, b: i64, r: usize },
    Multiply { a: i64, b: i64, r: usize },
    Input { r: usize },
    Output { a: i64 },
    JumpIfTrue { a: i64, d: usize },
    JumpIfFalse { a: i64, d: usize },
    LessThan { a: i64, b: i64, r: usize },
    Equals { a: i64, b: i64, r: usize },
    Halt,
}

pub struct Computer {
    pc: usize,
    mem: Vec<i64>,
    input: i64,
    output: i64,
}

impl Computer {
    pub fn new(mem: Vec<i64>) -> Self {
        Self {
            pc: 0,
            mem: mem,
            input: 0,
            output: 0,
        }
    }

    fn decode_instruction(&self) -> Result<(OpCode, usize), &'static str> {
        let opcode = self.mem[self.pc] % 100;
        match opcode {
            1 => Ok((
                OpCode::Add {
                    a: self.get_param(0, false),
                    b: self.get_param(1, false),
                    r: self.get_param(2, true) as usize,
                },
                4,
            )),
            2 => Ok((
                OpCode::Multiply {
                    a: self.get_param(0, false),
                    b: self.get_param(1, false),
                    r: self.get_param(2, true) as usize,
                },
                4,
            )),
            3 => Ok((
                OpCode::Input {
                    r: self.get_param(0, true) as usize,
                },
                2,
            )),
            4 => Ok((
                OpCode::Output {
                    a: self.get_param(0, false),
                },
                2,
            )),
            5 => Ok((
                OpCode::JumpIfTrue {
                    a: self.get_param(0, false),
                    d: self.get_param(1, false) as usize,
                },
                3,
            )),
            6 => Ok((
                OpCode::JumpIfFalse {
                    a: self.get_param(0, false),
                    d: self.get_param(1, false) as usize,
                },
                3,
            )),
            7 => Ok((
                OpCode::LessThan {
                    a: self.get_param(0, false),
                    b: self.get_param(1, false),
                    r: self.get_param(2, true) as usize,
                },
                4,
            )),
            8 => Ok((
                OpCode::Equals {
                    a: self.get_param(0, false),
                    b: self.get_param(1, false),
                    r: self.get_param(2, true) as usize,
                },
                4,
            )),
            99 => Ok((OpCode::Halt, 1)),
            _ => Err("Illegal opcode"),
        }
    }
    fn execute_instruction(&mut self) -> bool {
        let pc_start = self.pc;
        let (opcode, size) = self.decode_instruction().unwrap();
        match opcode {
            OpCode::Add { a, b, r } => {
                self.mem[r] = a + b;
            }
            OpCode::Multiply { a, b, r } => {
                self.mem[r] = a * b;
            }
            OpCode::Input { r } => {
                self.mem[r] = self.input;
            }
            OpCode::Output { a } => {
                println!("output: {}", a);
                self.output = a;
            }
            OpCode::JumpIfTrue { a, d } => {
                if a != 0 {
                    self.pc = d;
                }
            }
            OpCode::JumpIfFalse { a, d } => {
                if a == 0 {
                    self.pc = d;
                }
            }
            OpCode::LessThan { a, b, r } => {
                self.mem[r] = if a < b { 1 } else { 0 };
            }
            OpCode::Equals { a, b, r } => {
                self.mem[r] = if a == b { 1 } else { 0 };
            }
            OpCode::Halt => return true,
        }
        // Don't increment PC for jump instructions that modify PC by them selves.
        if self.pc == pc_start {
            self.pc += size;
        }
        return false;
    }
    pub fn run_program(&mut self, input: i64) -> i64 {
        self.input = input;
        while !self.execute_instruction() {}
        self.output
    }
    fn get_param(&self, index: usize, raw: bool) -> i64 {
        let flag =
            (((self.mem[self.pc] / 100) / i64::pow(10, index.try_into().unwrap())) % 10) as usize;
        let val = self.mem[self.pc + index + 1];
        if raw || flag == PARAM_MODE_IMMEDIATE {
            val
        } else if flag == PARAM_MODE_POSITION {
            self.mem[val as usize]
        } else {
            panic!("Illegal param mode {}", flag)
        }
    }
    pub fn mem(&self) -> &Vec<i64> {
        &self.mem
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_output() {
        let program = vec![3, 0, 4, 0, 99];
        assert_eq!(Computer::new(program.clone()).run_program(333), 333);
    }

    #[test]
    fn mul_immediate() {
        let program = vec![1002, 7, 3, 0, 4, 0, 99, 33];
        assert_eq!(Computer::new(program.clone()).run_program(1), 99);
    }

    #[test]
    fn add_immediate() {
        let program = vec![1101, 100, -5, 0, 4, 0, 99];
        assert_eq!(Computer::new(program.clone()).run_program(1), 95);
    }

    #[test]
    fn equal_to_8_position() {
        let program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(Computer::new(program.clone()).run_program(8), 1);
        assert_eq!(Computer::new(program.clone()).run_program(7), 0);
    }

    #[test]
    fn less_than_8_position() {
        let program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(Computer::new(program.clone()).run_program(7), 1);
        assert_eq!(Computer::new(program.clone()).run_program(8), 0);
    }

    #[test]
    fn equal_to_8_immediate() {
        let program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        assert_eq!(Computer::new(program.clone()).run_program(8), 1);
        assert_eq!(Computer::new(program.clone()).run_program(7), 0);
    }

    #[test]
    fn less_than_8_immediate() {
        let program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        assert_eq!(Computer::new(program.clone()).run_program(7), 1);
        assert_eq!(Computer::new(program.clone()).run_program(8), 0);
    }

    #[test]
    fn jump_if_true_position() {
        let program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        assert_eq!(Computer::new(program.clone()).run_program(0), 0);
        assert_eq!(Computer::new(program.clone()).run_program(3), 1);
    }

    #[test]
    fn jump_if_true_immediate() {
        let program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        assert_eq!(Computer::new(program.clone()).run_program(0), 0);
        assert_eq!(Computer::new(program.clone()).run_program(3), 1);
    }

    #[test]
    fn day5_larger_example() {
        let program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        assert_eq!(Computer::new(program.clone()).run_program(7), 999);
        assert_eq!(Computer::new(program.clone()).run_program(8), 1000);
        assert_eq!(Computer::new(program.clone()).run_program(9), 1001);
    }
}
