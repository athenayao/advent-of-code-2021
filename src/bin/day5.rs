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

    fn list_points(&self, with_diagonal: bool) -> Vec<Point> {
        let mut points = Vec::new();
        if &self.start.x == &self.end.x {
            // horizontal
            let start = min(self.start.y, self.end.y);
            let end = max(self.start.y, self.end.y);
            for y in start..=end {
                points.push(Point { x: self.start.x, y })
            }
        } else if &self.start.y == &self.end.y {
            // vertical
            let start = min(self.start.x, self.end.x);
            let end = max(self.start.x, self.end.x);
            for x in start..=end {
                points.push(Point { x, y: self.start.y })
            }
        } else if with_diagonal {
            // println!("DIAGONAL {:?}", &self);
            // diagonal
            let x_diff: i32 = self.start.x as i32 - self.end.x as i32;
            let y_diff: i32 = self.start.y as i32 - self.end.y as i32;
            if x_diff.abs() == y_diff.abs() {
                let incr_x: i32;
                if self.start.x < self.end.x {
                    incr_x = 1;
                } else {
                    incr_x = -1;
                }
                let incr_y: i32;
                if self.start.y < self.end.y {
                    incr_y = 1;
                } else {
                    incr_y = -1;
                }

                let mut x = self.start.x as i32;
                let mut y = self.start.y as i32;
                while x != self.end.x as i32 {
                    points.push(Point {
                        x: x as u32,
                        y: y as u32,
                    });
                    x += incr_x;
                    y += incr_y;
                    // println!("Pushing diagonal - ({}, {})", x, y)
                }
                points.push(Point {
                    x: x as u32,
                    y: y as u32,
                });
                // println!("Pushing diagonal - ({}, {})", x, y)
            }
        }
        // diagonal
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
        for point in vent.list_points(false) {
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
    let lines: Vec<&str> = input.split("\n").collect();

    let mut vents: Vec<Line> = Vec::new();
    for line in &lines {
        vents.push(Line::build(line));
    }

    let mut grid = Grid::build();
    for vent in &vents {
        for point in vent.list_points(true) {
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
        assert_eq!(part_two(input), 12);
    }
}
