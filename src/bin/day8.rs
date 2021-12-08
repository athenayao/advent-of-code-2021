fn parse_digits(input: &str) -> Vec<&str> {
    return input.split_whitespace().collect();
}

fn process_line(input: &str) -> u32 {
    let input_line: Vec<&str> = input.split(" | ").collect();
    let _all_digits = input_line[0];
    let display_digits = parse_digits(input_line[1]);
    let mut digit_counts = vec![0; 10];

    for raw_digit in display_digits {
        if let Some(actual_digit) = match raw_digit.len() {
            2 => Some(1),
            4 => Some(4),
            3 => Some(7),
            7 => Some(8),
            _ => None,
        } {
            digit_counts[actual_digit] += 1;
        }
    }
    digit_counts.iter().sum()
}

fn part_one(input: &str) -> u32 {
    let mut total = 0;
    for row in input.split("\n") {
        total += process_line(&row);
    }
    total
}

fn part_two(input: &str) -> u32 {
    0
}

fn main() {
    let input = include_str!("day8.txt");
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("day8-example.txt");
        assert_eq!(part_one(input), 26);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("day8-example.txt");
        assert_eq!(part_two(input), 0);
    }
}
