use std::collections::{HashMap, HashSet};

fn parse_digits(input: &str) -> Vec<&str> {
    return input.split_whitespace().collect();
}

fn process_line_part_one(input: &str) -> u32 {
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
        total += process_line_part_one(&row);
    }
    total
}

fn make_set(string: &str) -> HashSet<String> {
    let mut set = HashSet::new();
    for char in string.chars() {
        set.insert(char.to_string());
    }
    set
}

fn analyze_digits(all_digits: Vec<&str>) -> Vec<HashSet<String>> {
    let mut raw_to_set: HashMap<&str, HashSet<_>> = HashMap::new();
    let mut raw_to_num: HashMap<&str, u32> = HashMap::new();
    let mut num_to_raw: HashMap<String, &str> = HashMap::new();

    for raw_digit in all_digits {
        let mut set = HashSet::new();
        for char in raw_digit.chars() {
            set.insert(char.to_string());
        }
        raw_to_set.insert(raw_digit, set);
    }
    // println!("raw_to_set {:?}", raw_to_set);

    for raw_digit in raw_to_set.keys() {
        if let Some(actual_digit) = match raw_digit.len() {
            2 => Some(1),
            4 => Some(4),
            3 => Some(7),
            7 => Some(8),
            _ => None,
        } {
            raw_to_num.insert(raw_digit, actual_digit);
            num_to_raw.insert(actual_digit.to_string(), raw_digit);
        }
    }

    // having three loops is silly, btw
    for raw_digit in raw_to_set.keys() {
        let digit_as_set = &raw_to_set[raw_digit];
        if raw_digit.len() == 6 {
            let contains_four = digit_as_set.is_superset(&raw_to_set[num_to_raw["4"]]);
            let contains_one = digit_as_set.is_superset(&raw_to_set[num_to_raw["1"]]);

            let actual_digit;
            if contains_four && contains_one {
                actual_digit = 9;
            } else if contains_one {
                actual_digit = 0;
            } else {
                actual_digit = 6;
            }

            raw_to_num.insert(raw_digit, actual_digit);
            num_to_raw.insert(actual_digit.to_string(), raw_digit);
        }
        // 0 = len 6 - 1, not 4
        // 6 = len 6
        // 9 = len 6 = 1 + 4
    }

    for raw_digit in raw_to_set.keys() {
        let digit_as_set = &raw_to_set[raw_digit];
        if raw_digit.len() == 5 {
            let contains_one = digit_as_set.is_superset(&raw_to_set[num_to_raw["1"]]);
            let contains_seven = digit_as_set.is_superset(&raw_to_set[num_to_raw["7"]]);
            let is_contained_by_six = digit_as_set.is_subset(&raw_to_set[num_to_raw["6"]]);

            let actual_digit;
            if contains_one && contains_seven {
                actual_digit = 3;
            } else if is_contained_by_six {
                actual_digit = 5
            } else {
                actual_digit = 2;
            }
            // 2 = len 5 - none
            // 3 = len 5 - 1, 7
            // 5 = len 5 - 6

            raw_to_num.insert(raw_digit, actual_digit);
            num_to_raw.insert(actual_digit.to_string(), raw_digit);
        }
    }

    let mut sets: Vec<HashSet<String>> = Vec::new();
    let mut sorted: Vec<&String> = num_to_raw.keys().collect();
    sorted.sort();
    for i_key in sorted {
        let i: usize = i_key.parse().unwrap();
        sets.push(make_set(&num_to_raw[i_key]));
    }
    sets
}

fn process_line_part_two(input: &str) -> u32 {
    let input_line: Vec<&str> = input.split(" | ").collect();
    let mapped_digits = analyze_digits(parse_digits(input_line[0]));
    let mut display_digits = parse_digits(input_line[1]);

    display_digits.reverse();

    let mut current_total = 0;
    let mut current_place = 1;
    for raw_digit in display_digits {
        let digit_as_set = make_set(raw_digit);
        for (mapped_value, mapped_set) in mapped_digits.iter().enumerate() {
            if mapped_set.eq(&digit_as_set) {
                current_total += mapped_value * current_place;
                current_place *= 10;
            }
        }
    }
    current_total as u32
}

fn part_two(input: &str) -> u32 {
    let mut total = 0;
    for row in input.split("\n") {
        total += process_line_part_two(&row);
    }
    total
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
        assert_eq!(part_two("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"), 5353);
        assert_eq!(part_two(input), 61229);
    }
}
