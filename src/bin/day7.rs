fn sum_distance(vec: &Vec<i32>, position: i32) -> i32 {
    let mut total = 0;
    for x in vec.iter() {
        let distance = (x - position).abs();
        total += distance;
    }
    total
}

fn part_one(input: &str) -> i32 {
    let crabs: Vec<i32> = input.split(",").map(|n| n.parse().unwrap()).collect();
    let min_range = *crabs.iter().min().unwrap();
    let max_range = *crabs.iter().max().unwrap();

    let mut min_distance = sum_distance(&crabs, max_range);
    for i in min_range..max_range {
        let current_sum = sum_distance(&crabs, i);
        if current_sum < min_distance {
            min_distance = current_sum;
        }
    }
    min_distance
}

fn part_two(input: &str) -> i32 {
    let crabs: Vec<i32> = input.split(",").map(|n| n.parse().unwrap()).collect();
    let min_range = *crabs.iter().min().unwrap();
    let max_range = *crabs.iter().max().unwrap();

    let mut min_distance = i32::MAX;
    let mut memoize = vec![0; (max_range + 1) as usize];
    memoize[0] = 0;
    memoize[1] = 1;
    for i in 2..=max_range {
        memoize[i as usize] = i + memoize[(i - 1) as usize];
    }

    for i in min_range..=max_range {
        let mut current_sum = 0;
        for x in &crabs {
            let distance = (x - i).abs();
            current_sum += memoize[distance as usize];
        }
        if current_sum < min_distance {
            min_distance = current_sum;
        }
    }
    min_distance
}

fn main() {
    let input = include_str!("day7.txt");
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("day7-example.txt");
        assert_eq!(part_one(input), 37);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("day7-example.txt");
        assert_eq!(part_two(input), 168);
    }
}
