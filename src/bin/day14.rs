use std::collections::HashMap;

fn insert(template: &Vec<String>, rules: &HashMap<String, String>) -> Vec<String> {
    let mut modified: Vec<String> = Vec::new();
    modified.push(template[0].to_owned());
    for i in 0..template.len() - 1 {
        let first = &template[i];
        let second = &template[i + 1];
        if let Some(insert) = rules.get(&*format!("{}{}", &first, &second).to_owned()) {
            modified.push(insert.to_owned());
        }
        modified.push(second.to_owned());
    }
    modified
}
fn part_one(input: &str) -> u32 {
    let mut lines = input.split("\n");

    let mut start: Vec<String> = Vec::new();
    if let Some(line) = lines.next() {
        start = line.chars().map(|c| c.to_string()).collect();
    }

    let _ = lines.next();

    let mut rules = HashMap::new();
    loop {
        if let Some(line) = lines.next() {
            let line: Vec<&str> = line.split(" -> ").collect();
            let from = line[0];
            let to = line[1];
            rules.insert(from.to_owned(), to.to_owned());
        } else {
            break;
        }
    }

    let mut polymer = start;
    for _ in 0..40 {
        polymer = insert(&polymer, &rules);
    }

    let mut counts: HashMap<String, u32> = HashMap::new();
    for element in polymer {
        *counts.entry(element.to_owned()).or_insert(0) += 1;
    }
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

fn part_two(input: &str) -> u32 {
    0
}

fn main() {
    let input = include_str!("day14.txt");
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("day14-example.txt");
        assert_eq!(part_one(input), 1588);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("day14-example.txt");
        assert_eq!(part_two(input), 0);
    }
}
