use std::collections::HashSet;
struct Octopus {
    energy_level: u32,
    point: Point,
}

#[derive(Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Octopus {
    fn tick(&mut self) {
        self.energy_level += 1
    }

    fn get_adjacent_deltas(&self, grid_size: usize) -> Vec<Point> {
        let mut adjacent = Vec::new();
        for y in self.point.y as i32 - 1..=self.point.y as i32 + 1 {
            if y < 0 || y >= grid_size as i32 {
                continue;
            }
            for x in self.point.x as i32 - 1..=self.point.x as i32 + 1 {
                if x == self.point.x as i32 && y == self.point.y as i32 {
                    continue;
                }
                if x < 0 || x >= grid_size as i32 {
                    continue;
                }

                adjacent.push(Point {
                    x: x as usize,
                    y: y as usize,
                })
            }
        }
        adjacent
    }
}

impl std::fmt::Display for Octopus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.energy_level)
    }
}

fn tick(grid: &mut Vec<Vec<Octopus>>) -> u32 {
    let grid_len = grid.len();
    let mut to_increment = Vec::new();
    let mut just_flashed = HashSet::new();

    for y in 0..grid_len {
        for x in 0..grid_len {
            to_increment.push(Point { x, y })
        }
    }

    while to_increment.len() > 0 {
        if let Some(point) = to_increment.pop() {
            let octopus = &mut grid[point.y][point.x];
            octopus.tick();

            if octopus.energy_level > 9 && !just_flashed.contains(&point) {
                just_flashed.insert(Point {
                    x: point.x,
                    y: point.y,
                });
                to_increment.append(&mut octopus.get_adjacent_deltas(grid_len));
            }
        }
    }

    for point in &just_flashed {
        let octopus = &mut grid[point.y][point.x];
        octopus.energy_level = 0;
    }
    just_flashed.len() as u32
}

fn print_grid(grid: &Vec<Vec<Octopus>>) {
    for line in grid {
        for octopus in line {
            print!("{}", octopus);
        }
        println!("");
    }
    println!("");
}

fn part_one(input: &str) -> u32 {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut grid = Vec::new();
    for (y, &row) in lines.iter().enumerate() {
        grid.push(Vec::new());
        for (x, cell) in row.chars().enumerate() {
            grid[y].push(Octopus {
                energy_level: cell.to_string().parse().unwrap(),
                point: Point { x, y },
            })
        }
    }

    let mut flashes = 0;
    for _ in 0..100 {
        flashes += tick(&mut grid);
    }
    flashes
}

fn part_two(input: &str) -> u32 {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut grid = Vec::new();
    for (y, &row) in lines.iter().enumerate() {
        grid.push(Vec::new());
        for (x, cell) in row.chars().enumerate() {
            grid[y].push(Octopus {
                energy_level: cell.to_string().parse().unwrap(),
                point: Point { x, y },
            })
        }
    }

    let mut i = 0;
    loop {
        i += 1;
        let flashes = tick(&mut grid);
        if flashes == 100 {
            return i;
        }
    }
}

fn main() {
    let input = include_str!("day11.txt");
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("day11-example.txt");
        assert_eq!(part_one(input), 1656);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("day11-example.txt");
        assert_eq!(part_two(input), 195);
    }
}
