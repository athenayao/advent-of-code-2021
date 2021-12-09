fn is_low_point(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let current_value = grid[y][x];

    // what is a better way to do this? just cast to signed + use min?
    let y_low;
    if y == 0 {
        y_low = 0;
    } else {
        y_low = y - 1;
    }
    for neighbor_y in y_low..=y + 1 {
        if neighbor_y >= grid.len() {
            continue;
        }

        let x_low;
        if x == 0 {
            x_low = 0;
        } else {
            x_low = x - 1;
        }

        for neighbor_x in x_low..=x + 1 {
            if neighbor_x >= grid[y].len() || neighbor_x == x && neighbor_y == y {
                continue;
            }
            if current_value >= grid[neighbor_y as usize][neighbor_x as usize] {
                return false;
            }
        }
    }
    true
}

fn part_one(input: &str) -> u32 {
    let lines = input.split("\n").collect::<Vec<&str>>();

    let mut grid = Vec::new();
    for line in lines {
        let line = line
            .chars()
            .map(|n| n.to_string().parse().unwrap())
            .collect::<Vec<u32>>();
        grid.push(line);
    }

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

fn part_two(input: &str) -> u32 {
    0
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
        assert_eq!(part_two(input), 0);
    }
}
