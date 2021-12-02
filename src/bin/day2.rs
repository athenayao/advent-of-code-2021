use std::vec::Vec;

fn calculate_depth(input: &str) -> u32 {
    let lines: Vec<_> = input.split("\n").collect();
    let mut horizontal = 0;
    let mut depth = 0;

    for &line in &lines {
        let split: Vec<_> = line.split(" ").collect();
        let direction = split[0];
        let number: u32 = split[1].parse().unwrap();
        match direction {
            "forward" => horizontal += number,
            "down" => depth += number,
            "up" => depth -= number,
            _ => {}
        }
    }
    horizontal * depth
}

fn calculate_depth_with_aim(input: &str) -> u32 {
    let lines: Vec<_> = input.split("\n").collect();
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for &line in &lines {
        let split: Vec<_> = line.split(" ").collect();
        let direction = split[0];
        let number: u32 = split[1].parse().unwrap();
        match direction {
            "forward" => {
                horizontal += number;
                depth += aim * number

            },
            "down" => aim += number,
            "up" => aim -= number,
            _ => {}
        }

    }
    horizontal * depth
}


fn main() {
    let input = include_str!("day2.txt");
    let answer = calculate_depth_with_aim(input);
    println!("ANSWER: {}", answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_depth() {
        let input = include_str!("day2-example.txt");
        assert_eq!(calculate_depth(input), 150);
    }

    #[test]
    fn test_calculate_depth_with_aim() {
        let input = include_str!("day2-example.txt");
        assert_eq!(calculate_depth_with_aim(input), 900);
    }
}
