fn part_one(input: &str) -> u32 {
    0
}

fn part_two(input: &str) -> u32 {
    0
}

fn main() {
    let input = include_str!("_day.txt");
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("_day-example.txt");
        assert_eq!(part_one(input), 0);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("_day-example.txt");
        assert_eq!(part_two(input), 0);
    }
}
