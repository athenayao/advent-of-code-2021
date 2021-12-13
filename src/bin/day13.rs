use std::cmp::max;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Mark {
    Dot,
    Hash,
}

#[derive(Debug)]
enum Fold {
    Left,
    Up,
    Unknown,
}

#[derive(Debug)]
struct Grid {
    grid: Vec<String>,
    width: usize,
    height: usize,
}

impl Grid {
    fn index(&self, point: Point) -> usize {
        (self.width * point.y) + point.x
    }

    fn get(&self, x: usize, y: usize) -> &str {
        self.grid.get(self.index(Point { x, y })).unwrap()
    }

    fn overlay(&self, v1: &str, v2: &str) -> String {
        if v1 == "#" || v2 == "#" {
            return "#".to_owned();
        }
        ".".to_owned()
    }

    fn replace(&mut self, point: Point, value: String) {
        self.grid.push(value);
        self.grid.swap_remove(self.index(point));
    }

    fn mark(&mut self, point: Point) {
        self.grid.push("#".to_owned());
        self.grid.swap_remove(self.index(point));
    }

    fn fold(&mut self, num: usize, direction: Fold) {
        if matches!(direction, Fold::Up) {
            for from_y in (num + 1)..self.height {
                let to_y = 2 * num - from_y;
                for x in 0..self.width {
                    let from_value = &self.get(x, from_y);
                    let to_value = &self.get(x, to_y);
                    &self.replace(Point { x, y: to_y }, self.overlay(&from_value, &to_value));
                }
            }
            self.height = num;
        } else if matches!(direction, Fold::Left) {
            for from_x in (num + 1)..self.width {
                let to_x = 2 * num - from_x;
                for y in 0..self.height {
                    let from_value = &self.get(from_x, y);
                    let to_value = &self.get(to_x, y);
                    &self.replace(Point { x: to_x, y }, self.overlay(&from_value, &to_value));
                }
            }

            let mut new_grid = Vec::new();
            for y in 0..self.height {
                let from_index = self.index(Point { y, x: 0 });
                let to_index = self.index(Point { y, x: num });
                for item in &self.grid[from_index..to_index] {
                    new_grid.push(item.to_owned());
                }
            }
            self.width = num;
            self.grid = new_grid;
        }
    }

    fn print(&self) {
        for i in 0..self.width * self.height {
            print!("{}", self.grid[i]);
            if i % self.width == self.width - 1 {
                println!("");
            }
        }
        println!("");
    }

    fn count_marks(&self) -> u32 {
        let mut total = 0;
        for i in 0..self.width * self.height {
            let value = &self.grid[i];
            if value == "#" {
                total += 1;
            }
        }
        total
    }
}

fn part_one(input: &str) -> u32 {
    let mut lines = input.split("\n");

    let mut point_inputs: Vec<Point> = Vec::new();
    let mut max_width: usize = 0;
    let mut max_height: usize = 0;
    // Coordinate loop
    loop {
        if let Some(line) = lines.next() {
            if line == "" {
                break;
            }

            let line: Vec<usize> = line
                .split(",")
                .map(|n| n.parse().expect("Expected an int"))
                .collect();
            point_inputs.push(Point {
                x: line[0],
                y: line[1],
            });
            max_width = max(max_width, line[0] + 1);
            max_height = max(max_height, line[1] + 1);
        }
    }

    let mut paper = Grid {
        grid: vec![".".to_owned(); max_width * max_height + 1],
        width: max_width,
        height: max_height,
    };

    for point in point_inputs {
        paper.mark(point);
    }
    paper.print();

    // Fold loop (but we only want the first)
    if let Some(line) = lines.next() {
        let line: Vec<&str> = line[11..].split("=").collect();
        let fold = match line[0] {
            "x" => Fold::Left,
            "y" => Fold::Up,
            _ => Fold::Unknown,
        };
        paper.fold(line[1].parse().unwrap(), fold);
        paper.print();

        // println!("Step 2");
        // paper.fold(5, Fold::Left);
        // paper.print();
    }

    paper.count_marks()
    // loop {
    //     if let Some(line) = lines.next() {
    //         if line == "" {
    //             break;
    //         }
    //     }
    // }
}
// 0, 14 == 0, 0

fn part_two(input: &str) -> u32 {
    0
}

fn main() {
    let input = include_str!("day13.txt");
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("day13-example.txt");
        assert_eq!(part_one(input), 17);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("day13-example.txt");
        assert_eq!(part_two(input), 0);
    }
}
