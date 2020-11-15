use crate::puzzle::{io, File, Puzzle};
use std::cmp;
use std::string::String;

pub struct Day3;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Copy, Clone)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn intersection(&self, l2: &Line) -> Option<Point> {
        if self.horizontal() == l2.horizontal() {
            return None;
        }
        let horizontal_line = if self.horizontal() { self } else { l2 };
        let vertical_line = if self.horizontal() { l2 } else { self };
        if vertical_line.max_x() < horizontal_line.min_x()
            || vertical_line.max_x() > horizontal_line.max_x()
        {
            None
        } else if horizontal_line.max_y() < vertical_line.min_y()
            || horizontal_line.max_y() > vertical_line.max_y()
        {
            None
        } else {
            Some(Point {
                x: vertical_line.max_x(),
                y: horizontal_line.max_y(),
            })
        }
    }

    fn max_x(&self) -> i64 {
        cmp::max(self.start.x, self.end.x)
    }

    fn min_x(&self) -> i64 {
        cmp::min(self.start.x, self.end.x)
    }

    fn max_y(&self) -> i64 {
        cmp::max(self.start.y, self.end.y)
    }

    fn min_y(&self) -> i64 {
        cmp::min(self.start.y, self.end.y)
    }

    fn horizontal(&self) -> bool {
        self.start.x != self.end.x
    }

    fn contains(&self, point: &Point) -> bool {
        point.x >= self.min_x()
            && point.x <= self.max_x()
            && point.y >= self.min_y()
            && point.y <= self.max_y()
    }

    fn length(&self) -> i64 {
        manhattan_distance(&self.start, &self.end)
    }
}

fn get_lines(cmds: &Vec<String>) -> Vec<Line> {
    let points: Vec<Point> = cmds
        .iter()
        .scan(Point { x: 0, y: 0 }, |prev, cmd| {
            let point: Point = match cmd.chars().nth(0).unwrap() {
                'R' => Point {
                    x: prev.x + &cmd[1..].parse::<i64>().unwrap(),
                    y: prev.y,
                },
                'D' => Point {
                    x: prev.x,
                    y: prev.y - &cmd[1..].parse::<i64>().unwrap(),
                },
                'L' => Point {
                    x: prev.x - &cmd[1..].parse::<i64>().unwrap(),
                    y: prev.y,
                },
                'U' => Point {
                    x: prev.x,
                    y: prev.y + &cmd[1..].parse::<i64>().unwrap(),
                },
                _ => panic!("Illegal command"),
            };
            *prev = point;
            Some(point)
        })
        .collect();
    let lines: Vec<Line> = points
        .iter()
        .scan(Point { x: 0, y: 0 }, |prev, &point| {
            let line = Line {
                start: *prev,
                end: point,
            };
            *prev = point;
            Some(line)
        })
        .collect();
    lines
}

fn manhattan_distance(p1: &Point, p2: &Point) -> i64 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn step_distance(path: &Vec<Line>, destination: &Point) -> Result<i64, &'static str> {
    let mut distance: i64 = 0;
    for line in path {
        if line.contains(destination) {
            distance += manhattan_distance(&line.start, &destination);
            return Ok(distance);
        } else {
            distance += line.length()
        }
    }
    Err("Point not on line")
}

impl Day3 {
    fn solve_part1(&self, wire1_input: &Vec<String>, wire2_input: &Vec<String>) -> i64 {
        let mut min_distance = std::i64::MAX;
        let wire1_lines = get_lines(wire1_input);
        let wire2_lines = get_lines(wire2_input);
        for line1 in &wire1_lines {
            for line2 in &wire2_lines {
                if let Some(point) = line1.intersection(&line2) {
                    let central = Point { x: 0, y: 0 };
                    if point != central {
                        min_distance = cmp::min(min_distance, manhattan_distance(&central, &point));
                    }
                }
            }
        }
        min_distance
    }

    fn solve_part2(&self, wire1_input: &Vec<String>, wire2_input: &Vec<String>) -> i64 {
        let mut min_distance = std::i64::MAX;
        let wire1_lines = get_lines(wire1_input);
        let wire2_lines = get_lines(wire2_input);
        for line1 in &wire1_lines {
            for line2 in &wire2_lines {
                if let Some(point) = line1.intersection(&line2) {
                    let central = Point { x: 0, y: 0 };
                    if point != central {
                        let distance = step_distance(&wire1_lines, &point).unwrap()
                            + step_distance(&wire2_lines, &point).unwrap();
                        min_distance = cmp::min(min_distance, distance);
                    }
                }
            }
        }
        min_distance
    }
}

impl Puzzle for Day3 {
    fn solve(&self, lines: io::Result<io::Lines<io::BufReader<File>>>) -> (String, String) {
        let mut lines = lines.expect("No input file").map(|l| l.unwrap());
        let wire1_input = lines
            .nth(0)
            .unwrap()
            .split(",")
            .map(|x| String::from(x))
            .collect();
        let wire2_input = lines
            .nth(0)
            .unwrap()
            .split(",")
            .map(|x| String::from(x))
            .collect();

        (
            self.solve_part1(&wire1_input, &wire2_input).to_string(),
            self.solve_part2(&wire1_input, &wire2_input).to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_intersect1() {
        let l1 = Line {
            start: Point { x: 0, y: 0 },
            end: Point { x: 0, y: 5 },
        };
        let l2 = Line {
            start: Point { x: 0, y: 0 },
            end: Point { x: 0, y: 5 },
        };
        assert!(l1.intersection(&l2).is_none());
    }

    #[test]
    fn line_intersect2() {
        let l1 = Line {
            start: Point { x: 0, y: 0 },
            end: Point { x: 0, y: 5 },
        };
        let l2 = Line {
            start: Point { x: 0, y: 0 },
            end: Point { x: 5, y: 0 },
        };
        assert_eq!(l1.intersection(&l2).unwrap(), Point { x: 0, y: 0 });
    }

    #[test]
    fn part1_example1() {
        assert_eq!(
            Day3 {}.solve_part1(
                &vec!("R8", "U5", "L5", "D3")
                    .iter()
                    .map(|x| String::from(*x))
                    .collect(),
                &vec!("U7", "R6", "D4", "L4")
                    .iter()
                    .map(|x| String::from(*x))
                    .collect()
            ),
            6
        );
    }

    #[test]
    fn part1_example2() {
        assert_eq!(
            Day3 {}.solve_part1(
                &vec!("R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72")
                    .iter()
                    .map(|x| String::from(*x))
                    .collect(),
                &vec!("U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83")
                    .iter()
                    .map(|x| String::from(*x))
                    .collect()
            ),
            159
        );
    }

    #[test]
    fn part1_example3() {
        assert_eq!(
            Day3 {}.solve_part1(
                &vec!("R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51")
                    .iter()
                    .map(|x| String::from(*x))
                    .collect(),
                &vec!("U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7")
                    .iter()
                    .map(|x| String::from(*x))
                    .collect()
            ),
            135
        );
    }

    #[test]
    fn part2_example1() {
        assert_eq!(
            Day3 {}.solve_part2(
                &vec!("R8", "U5", "L5", "D3")
                    .iter()
                    .map(|x| String::from(*x))
                    .collect(),
                &vec!("U7", "R6", "D4", "L4")
                    .iter()
                    .map(|x| String::from(*x))
                    .collect()
            ),
            30
        );
    }

    #[test]
    fn part2_example2() {
        assert_eq!(
            Day3 {}.solve_part2(
                &vec!("R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72")
                    .iter()
                    .map(|x| String::from(*x))
                    .collect(),
                &vec!("U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83")
                    .iter()
                    .map(|x| String::from(*x))
                    .collect()
            ),
            610
        );
    }

    #[test]
    fn part2_example3() {
        assert_eq!(
            Day3 {}.solve_part2(
                &vec!("R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51")
                    .iter()
                    .map(|x| String::from(*x))
                    .collect(),
                &vec!("U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7")
                    .iter()
                    .map(|x| String::from(*x))
                    .collect()
            ),
            410
        );
    }
}
