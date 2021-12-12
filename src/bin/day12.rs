use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Map {
    caves: HashMap<String, Cave>,
}
impl Map {
    fn connect(&mut self, cave_1: &str, cave_2: &str) {
        self.caves
            .entry(cave_1.to_owned())
            .or_insert(Cave::build(cave_1))
            .add_connection(cave_2);
        self.caves
            .entry(cave_2.to_owned())
            .or_insert(Cave::build(cave_2))
            .add_connection(cave_1);
    }
}

#[derive(Debug)]
struct Path {
    visited: Vec<String>,
    seen_small: HashSet<String>,
}
impl Path {
    fn visit(&mut self, cave: Cave) {
        self.visited.push(cave.id.to_owned());
        match &cave.size {
            CaveSize::Small => {
                self.seen_small.insert(cave.id.to_owned());
            }
            _ => (),
        };
    }
}

#[derive(Debug)]
enum CaveSize {
    Small,
    Large,
}

#[derive(Debug)]
struct Cave {
    id: String,
    connections: Vec<String>,
    size: CaveSize,
}

impl Cave {
    fn build(id: &str) -> Cave {
        let cave_size;
        if id.to_lowercase() == id {
            cave_size = CaveSize::Small;
        } else {
            cave_size = CaveSize::Large;
        }

        Cave {
            id: id.to_owned(),
            connections: Vec::new(),
            size: cave_size,
        }
    }

    fn add_connection(&mut self, cave: &str) {
        self.connections.push(cave.to_owned());
    }
}

fn find_path(
    cave_name: &str,
    map: &Map,
    current_path: &Vec<String>,
    seen_small: &HashSet<String>,
) -> Vec<Vec<String>> {
    if cave_name == "end" {
        return vec![current_path.clone()];
    }
    let mut current_path = current_path.clone();
    current_path.push(cave_name.to_owned());

    let mut seen_small = seen_small.clone();

    let mut paths: Vec<Vec<String>> = Vec::new();
    if let Some(cave) = map.caves.get(cave_name) {
        if matches!(cave.size, CaveSize::Small) {
            seen_small.insert(cave.id.clone());
        }
        for connection in &cave.connections {
            if seen_small.contains(connection) {
                continue;
            }
            // but have to return multiple
            let found_paths = find_path(&connection, map, &current_path, &seen_small);
            for path in &found_paths {
                let mut new_path = path.clone();
                new_path.push(cave.id.to_owned());
                paths.push(new_path);
            }
        }
    }
    paths
}

fn part_one(input: &str) -> u32 {
    let mut map = Map {
        caves: HashMap::new(),
    };

    let lines: Vec<&str> = input.split("\n").collect();
    for line in lines {
        let caves: Vec<&str> = line.split("-").collect();
        map.connect(&caves[0], &caves[1]);
    }

    // start at start, end at end, don't visit small caves more than once
    // DFS

    let paths = find_path("start", &map, &Vec::new(), &HashSet::new());
    paths.len() as u32
}

fn part_two(input: &str) -> u32 {
    0
}

fn main() {
    let input = include_str!("day12.txt");
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("day12-example.txt");
        assert_eq!(part_one(input), 10);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("day12-example.txt");
        assert_eq!(part_two(input), 0);
    }
}
