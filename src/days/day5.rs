use crate::puzzle::{io, File, Puzzle};
use std::string::String;
pub struct Day5;

impl Day5 {
    fn solve(&self, mem: &Vec<i64>, input: i64) -> i64 {
        let mut c = crate::computer::Computer::new(mem.clone());
        c.run_program(vec![input])
    }
}

impl Puzzle for Day5 {
    fn solve(&self, lines: io::Result<io::Lines<io::BufReader<File>>>) -> (String, String) {
        let mem: Vec<i64> = lines
            .expect("No input file")
            .map(|l| l.unwrap())
            .collect::<String>()
            .split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        return (
            self.solve(&mem, 1).to_string(),
            self.solve(&mem, 5).to_string(),
        );
    }
}
