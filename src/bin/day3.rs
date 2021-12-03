fn convert_to_decimal(mut vec: Vec<i32>) -> usize {
    vec.reverse();
    let mut total = 0;
    dbg!("{}", &vec);
    for (i, &v) in vec.iter().enumerate() {
        if v == 1 {
            // TODO: better this
            total += (2 << i) / 2;
        }
    }
    total
}

fn part_one(input: &str) -> usize {
    let lines: Vec<_> = input.split("\n").collect();

    let mut grid = Vec::with_capacity(5);
    let first_line = &lines[0];
    for c in first_line.chars() {
        grid.push(Vec::<i32>::new())
    }

    for &line in &lines {
        for (i, c) in line.chars().enumerate() {
            grid[i].push(c.to_string().parse().unwrap())
        }
    }


    let mut gamma = Vec::new();
    let mut epsilon = Vec::new();
    // most common bit
    for col in grid {
        let mut c0 = 0;
        let mut c1 = 0;

        for item in col {
            if item == 0 {
                c0 += 1
            } else if item == 1 {
                c1 += 1
            }
        }

        if c0 > c1 {
            gamma.push(0);
            epsilon.push(1);
        } else {
            gamma.push(1);
            epsilon.push(0);
        }
    }
    convert_to_decimal(gamma) * convert_to_decimal(epsilon)
}

fn part_two(input: &str) -> u32 {
    0
}

fn main() {
    let input = include_str!("day3.txt");
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("day3-example.txt");
        assert_eq!(part_one(input), 198);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("day3-example.txt");
        assert_eq!(part_two(input), 0);
    }
}
