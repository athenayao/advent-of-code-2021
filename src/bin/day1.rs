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

fn main() {
    let input = include_str!("day1.txt");
    let answer = count_increases(input);
    println!("ANSWER: {}", answer)
}

mod tests {
    use super::*;

    #[test]
    fn test_count_increases() {
        let input = include_str!("day1-example.txt");
        assert_eq!(count_increases(input), 7);
    }
}
