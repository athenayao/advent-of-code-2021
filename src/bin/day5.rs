use std::cmp::{max, min};
use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}
impl Point {
    fn build(string: &str) -> Point {
        let string: Vec<&str> = string.split(",").collect();
        Point {
            x: string[0].trim().parse().unwrap(),
            y: string[1].trim().parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn build(string: &str) -> Line {
        let string: Vec<&str> = string.split("->").collect();
        Line {
            start: Point::build(&string[0]),
            end: Point::build(&string[1]),
        }
    }

    fn list_points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        // horizontal
        if &self.start.x == &self.end.x {
            let start = min(self.start.y, self.end.y);
            let end = max(self.start.y, self.end.y);
            for y in start..=end {
                points.push(Point { x: self.start.x, y })
            }
        }
        // vertical
        if &self.start.y == &self.end.y {
            let start = min(self.start.x, self.end.x);
            let end = max(self.start.x, self.end.x);
            for x in start..=end {
                points.push(Point { x, y: self.start.y })
            }
        }
        // println!("{:?} -> [{:?}]", &self, &points);
        points
    }
}

#[derive(Debug)]
struct Grid {
    points: HashMap<Point, u32>,
}

impl Grid {
    fn build() -> Grid {
        Grid {
            points: HashMap::new(),
        }
    }
}
impl Grid {
    fn mark(&mut self, point: Point) {
        *self.points.entry(point).or_insert(0) += 1;
    }
}

fn part_one(input: &str) -> u32 {
    let lines: Vec<&str> = input.split("\n").collect();

    let mut vents: Vec<Line> = Vec::new();
    for line in &lines {
        vents.push(Line::build(line));
    }

    let mut grid = Grid::build();
    for vent in &vents {
        for point in vent.list_points() {
            grid.mark(point);
        }
    }
    let mut total = 0;
    for (key, &points) in &grid.points {
        if points >= 2 {
            total += 1;
        }
    }
    total
}

fn part_two(input: &str) -> u32 {
    0
}

fn main() {
    let input = include_str!("day5.txt");
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("day5-example.txt");
        assert_eq!(part_one(input), 5);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("day5-example.txt");
        assert_eq!(part_two(input), 0);
    }
}
