use std::vec::Vec;

fn count_increases(input: &str) -> u32 {
    let mut previous_number = u32::MAX;
    let mut num_increase = 0;
    for line in input.split("\n") {
        let num = line.parse().unwrap();
        if num > previous_number {
            num_increase += 1;
        }
        previous_number = num;
    }
    num_increase
}

fn count_window_increases(input: &str) -> u32 {
    let lines: Vec<u32> = input.split("\n").map(|x| x.parse().unwrap()).collect();
    let mut previous_sum = u32::MAX;
    let mut num_increase = 0;

    const WINDOW_SIZE: usize = 3;
    for (i, line) in lines.iter().enumerate() {
        if i + 1 < WINDOW_SIZE {
            continue;
        }
        let sum = lines[i] + lines[i-1] + lines[i-2];
        if sum > previous_sum {
            num_increase += 1;
        }
        previous_sum = sum;
    }
    num_increase
}

fn main() {
    let input = include_str!("day1.txt");
    let answer = count_window_increases(input);
    println!("ANSWER: {}", answer)
}

mod tests {
    use super::*;

    #[test]
    fn test_count_increases() {
        let input = include_str!("day1-example.txt");
        assert_eq!(count_increases(input), 7);
    }

    #[test]
    fn test_count_window_increases() {
        let input = include_str!("day1-example.txt");
        assert_eq!(count_window_increases(input), 5);
    }
}
