use std::collections::{HashMap, HashSet};
enum LineResult {
    Valid { incomplete: Vec<String> },
    Invalid { invalid_with: String },
}

fn parse_line(line: &str) -> LineResult {
    let pairs = HashMap::from([("}", "{"), (")", "("), ("]", "["), (">", "<")]);
    let opening_symbols: HashSet<&str> = pairs.values().map(|n| *n).collect();
    let mut stack = Vec::new();
    for c in line.chars() {
        let c = c.to_string();
        if opening_symbols.contains(&*c) {
            stack.push(c);
        } else {
            if let Some(last) = stack.last() {
                if last == pairs[&*c] {
                    stack.pop();
                } else {
                    return LineResult::Invalid { invalid_with: c };
                }
            }
        }
    }
    LineResult::Valid { incomplete: stack }
}

fn part_one(input: &str) -> u32 {
    let lines: Vec<&str> = input.split("\n").collect();
    let scores = HashMap::from([(")", 3), ("]", 57), ("}", 1197), (">", 25137)]);

    let mut total = 0;
    for line in lines {
        match parse_line(&line) {
            LineResult::Invalid { invalid_with } => total += scores[&*invalid_with],
            _ => (),
        }
    }
    total
}

fn part_two(input: &str) -> u64 {
    let lines: Vec<&str> = input.split("\n").collect();
    let scores = HashMap::from([(")", 1), ("]", 2), ("}", 3), (">", 4)]);
    let pairs = HashMap::from([("(", ")"), ("{", "}"), ("[", "]"), ("<", ">")]);

    let mut calculated_results = Vec::new();
    for line in lines {
        let mut total = 0;
        match parse_line(&line) {
            LineResult::Valid { incomplete } => {
                let mut to_match = incomplete;
                to_match.reverse();
                for symbol in to_match {
                    total *= 5;
                    total += scores[pairs[&*symbol]];
                }
                calculated_results.push(total);
            }
            _ => (),
        }
    }
    calculated_results.sort();
    calculated_results[calculated_results.len() / 2]
}

fn main() {
    let input = include_str!("day10.txt");
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("day10-example.txt");
        assert_eq!(part_one(input), 26397);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("day10-example.txt");
        assert_eq!(part_two(input), 288957);
    }
}
