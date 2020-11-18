use crate::puzzle::{io, File, Puzzle};
use std::cmp;
use std::string::String;
pub struct Day6;

#[derive(Debug)]
struct Node {
    name: String,
    nodes: Vec<Box<Node>>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            nodes: Vec::new(),
        }
    }
    pub fn insert(&mut self, node: Box<Node>) {
        self.nodes.push(node);
    }
    pub fn find(&mut self, name: &str) -> Option<&mut Node> {
        if self.name == name {
            return Some(self);
        } else {
            for node in &mut self.nodes {
                if let Some(found) = node.find(&name) {
                    return Some(found);
                }
            }
        }
        return None;
    }
    fn checksum_helper(&self, distance: u64) -> u64 {
        let mut total = distance;
        for node in &self.nodes {
            total += node.checksum_helper(distance + 1);
        }
        total
    }
    pub fn checksum(&self) -> u64 {
        self.checksum_helper(0)
    }
    pub fn path(&mut self, name: &str) -> Option<Vec<String>> {
        if self.name == name {
            return Some(vec![name.to_string()]);
        } else {
            for node in &mut self.nodes {
                if let Some(mut path) = node.path(&name) {
                    path.insert(0, self.name.clone());
                    return Some(path);
                }
            }
        }
        return None;
    }
}

impl Day6 {
    fn solve_part1(&self, orbits: &Vec<String>) -> u64 {
        let mut com = Node::new("COM");
        let mut orbits: Vec<(String, String)> = orbits
            .iter()
            .map(|x| {
                let bodies: Vec<String> = x.split(")").map(|x| x.to_string()).collect();
                (bodies[0].clone(), bodies[1].clone())
            })
            .collect();
        while orbits.len() > 0 {
            let (inner, outer) = orbits.remove(0);
            if let Some(node) = com.find(&inner) {
                node.insert(Box::new(Node::new(&outer)));
            } else {
                orbits.push((inner, outer));
            }
        }
        com.checksum()
    }

    fn solve_part2(&self, orbits: &Vec<String>) -> u64 {
        let mut com = Node::new("COM");
        let mut orbits: Vec<(String, String)> = orbits
            .iter()
            .map(|x| {
                let bodies: Vec<String> = x.split(")").map(|x| x.to_string()).collect();
                (bodies[0].clone(), bodies[1].clone())
            })
            .collect();
        while orbits.len() > 0 {
            let (inner, outer) = orbits.remove(0);
            if let Some(node) = com.find(&inner) {
                node.insert(Box::new(Node::new(&outer)));
            } else {
                orbits.push((inner, outer));
            }
        }
        let path_me = com.path("YOU").unwrap();
        let path_santa = com.path("SAN").unwrap();
        for i in 0..cmp::min(path_me.len(), path_santa.len()) {
            if path_me[i] != path_santa[i] {
                return (path_me.len() - i - 1 + path_santa.len() - i - 1) as u64;
            }
        }
        0
    }
}

impl Puzzle for Day6 {
    fn solve(&self, lines: io::Result<io::Lines<io::BufReader<File>>>) -> (String, String) {
        let orbits: Vec<String> = lines.expect("No input file").map(|l| l.unwrap()).collect();
        return (
            self.solve_part1(&orbits).to_string(),
            self.solve_part2(&orbits).to_string(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let orbits = (vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
        ]
        .iter()
        .map(|x| x.to_string()))
        .collect();
        assert_eq!(Day6 {}.solve_part1(&orbits), 42);
    }

    #[test]
    fn part2_example1() {
        let orbits = (vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
            "I)SAN",
        ]
        .iter()
        .map(|x| x.to_string()))
        .collect();
        assert_eq!(Day6 {}.solve_part2(&orbits), 4);
    }
}
