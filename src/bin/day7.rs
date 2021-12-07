fn find_min(vec: &Vec<i32>) -> i32 {
    let mut min = &vec[0];
    for x in vec.iter() {
        if x < min {
            min = x;
        }
    }
    *min
}

fn find_max(vec: &Vec<i32>) -> i32 {
    let mut max = &vec[0];
    for x in vec.iter() {
        if x > max {
            max = x;
        }
    }
    *max
}

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
    let min_range = find_min(&crabs);
    let max_range = find_max(&crabs);

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
    0
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
        assert_eq!(part_two(input), 0);
    }
}
