use std::collections::HashSet;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
    value: u32,
}

fn get_neighbors(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> Vec<Point> {
    let mut valid_neighbors = Vec::new();

    // TODO: simplify this
    let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    for direction in directions {
        let neighbor_x = x as i32 + direction.0;
        let neighbor_y = y as i32 + direction.1;

        if neighbor_y < 0 || neighbor_y >= grid.len() as i32 {
            continue;
        }
        if neighbor_x < 0
            || neighbor_x >= grid[y].len() as i32
            || neighbor_x == x as i32 && neighbor_y == y as i32
        {
            continue;
        }

        let neighbor_value = grid[neighbor_y as usize][neighbor_x as usize];
        valid_neighbors.push(Point {
            x: neighbor_x as usize,
            y: neighbor_y as usize,
            value: neighbor_value,
        })
    }
    valid_neighbors
}

fn get_to_visit(grid: &Vec<Vec<u32>>, x: usize, y: usize, to_visit: &mut Vec<Point>) {
    let neighbors = get_neighbors(&grid, x, y);
    for neighbor in neighbors {
        if neighbor.value == 9 {
            continue;
        }
        to_visit.push(neighbor)
    }
}

fn is_low_point(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let current_value = grid[y][x];
    let neighbors = get_neighbors(&grid, x, y);
    for neighbor in neighbors {
        if current_value >= neighbor.value {
            return false;
        }
    }
    true
}

fn build_grid(input: &str) -> Vec<Vec<u32>> {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut grid = Vec::new();
    for line in lines {
        let line = line
            .chars()
            .map(|n| n.to_string().parse().unwrap())
            .collect::<Vec<u32>>();
        grid.push(line);
    }
    grid
}

fn part_one(input: &str) -> u32 {
    let grid = build_grid(&input);

    let mut low_points = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            if is_low_point(&grid, x, y) {
                low_points.push(height);
            }
        }
    }
    low_points.into_iter().map(|n| n + 1).sum()
}

fn find_basin_size(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> usize {
    let mut seen = HashSet::new();
    let mut to_visit: Vec<Point> = vec![Point {
        x,
        y,
        value: grid[y][x],
    }];
    while to_visit.len() > 0 {
        if let Some(point) = to_visit.pop() {
            if seen.contains(&point) {
                continue;
            }
            seen.insert(Point { ..point });
            get_to_visit(grid, point.x, point.y, &mut to_visit);
        }
    }
    seen.len()
}

fn part_two(input: &str) -> usize {
    let grid = build_grid(&input);
    // find basins
    let mut basin_sizes = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if is_low_point(&grid, x, y) {
                basin_sizes.push(find_basin_size(&grid, x, y))
            }
        }
    }
    basin_sizes.sort_by(|a, b| b.cmp(a));
    &basin_sizes[0] * &basin_sizes[1] * &basin_sizes[2]
}

fn main() {
    let input = include_str!("day9.txt");
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("day9-example.txt");
        assert_eq!(part_one(input), 15);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("day9-example.txt");
        assert_eq!(part_two(input), 1134);
    }
}
