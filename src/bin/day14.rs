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

fn f(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}
fn insert_with_counters(pair_counters: &mut HashMap<String, u64>, rules: &HashMap<String, String>) {
    let mut to_add = HashMap::new();
    let mut to_remove = HashMap::new();
    for (pair, insert) in rules {
        if let Some(count) = pair_counters.get(pair) {
            *to_add.entry(f(&pair[..1], insert)).or_insert(0) += count;
            *to_add.entry(f(insert, &pair[1..])).or_insert(0) += count;
            *to_remove.entry(pair.clone()).or_insert(0) += count;
        }
    }

    for (pair, count) in to_add {
        *pair_counters.entry(pair).or_insert(0) += count;
    }
    for (pair, count) in to_remove {
        *pair_counters.entry(pair).or_insert(0) -= count;
    }
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

fn part_two(input: &str) -> u64 {
    let mut lines = input.split("\n");

    let mut pair_counters = HashMap::new();
    let mut last_letter = String::from("");
    if let Some(line) = lines.next() {
        let start: Vec<String> = line.chars().map(|c| c.to_string()).collect();
        last_letter = start.last().unwrap().to_owned();
        for i in 0..start.len() - 1 {
            *pair_counters
                .entry(f(&start[i], &start[i + 1]))
                .or_insert(0) += 1;
        }
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

    for _ in 0..40 {
        insert_with_counters(&mut pair_counters, &rules);
    }

    let mut element_counters = HashMap::new();
    element_counters.insert(last_letter.to_owned(), 1);
    for (pair, count) in pair_counters {
        *element_counters
            .entry(pair[0..1].to_owned()) // first character of every pair
            .or_insert(0) += count;
    }

    // dbg!("{}", &element_counters);
    element_counters.values().max().unwrap() - element_counters.values().min().unwrap()
}

fn main() {
    let input = include_str!("day14.txt");
    // println!("Part 1: {}", part_one(input));
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
        assert_eq!(part_two(input), 1588);
    }
}

fn count_pairs(line: &str) {
    let mut pair_counters = HashMap::new();
    let start: Vec<String> = line.chars().map(|c| c.to_string()).collect();
    for i in 0..start.len() - 1 {
        *pair_counters
            .entry(f(&start[i], &start[i + 1]))
            .or_insert(0) += 1;
    }
    // dbg!("COUNT_PAIRS", pair_counters);
}
