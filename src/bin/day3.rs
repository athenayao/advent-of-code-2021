fn convert_to_decimal(mut vec: Vec<i32>) -> usize {
    vec.reverse();
    let mut total = 0;
    for (i, &v) in vec.iter().enumerate() {
        if v == 1 {
            total += (2 << i) / 2;
        }
    }
    total
}

fn part_one(input: &str) -> usize {
    let lines: Vec<_> = input.split("\n").collect();

    let first_line = &lines[0];
    let mut grid = Vec::with_capacity(first_line.len());
    for _ in first_line.chars() {
        grid.push(Vec::<i32>::new())
    }

    for &line in &lines {
        for (i, c) in line.chars().enumerate() {
            grid[i].push(c.to_string().parse().unwrap())
        }
    }


    let mut gamma = Vec::new();
    let mut epsilon = Vec::new();
    // most common bit
    for col in grid {
        let (c0, c1) = count_bits(&col);

        if c0 > c1 {
            gamma.push(0);
            epsilon.push(1);
        } else {
            gamma.push(1);
            epsilon.push(0);
        }
    }
    convert_to_decimal(gamma) * convert_to_decimal(epsilon)
}

fn count_bits(vec: &Vec<i32>) -> (i32, i32) {
    let mut c0 = 0;
    let mut c1 = 0;

    for &item in vec {
        if item == 0 {
            c0 += 1
        } else if item == 1 {
            c1 += 1
        }
    }
    return (c0, c1)
}

fn recurse_oxygen<'a>(by_columns: &Vec<Vec<i32>>, column: usize, numbers: Vec<Vec<i32>>) -> Vec<i32> {
    if numbers.len() == 1 {
        return numbers[0].clone();
    }

    let (c0, c1) = count_bits(&by_columns[column]);
    let keep_digit;
    if c1 == c0 {
        keep_digit = 1;
    } else if c1 > c0 {
        keep_digit = 1;
    } else {
        keep_digit = 0;
    }

    let mut new_numbers = Vec::new();
    for number in numbers {
        if number[column] == keep_digit {
            new_numbers.push(number.clone());
        }
    }

    let first_line = &new_numbers[0];
    let mut by_columns = Vec::with_capacity(first_line.len());
    for _ in first_line {
        by_columns.push(Vec::<i32>::new())
    }
    for number in &new_numbers {
        for (i, &digit) in number.iter().enumerate() {
            by_columns[i].push(digit);
        }
    }

    println!("o2 {}, c1 {}", c0, c1);
    dbg!("new_numbers {}", &new_numbers);
    recurse_oxygen(&by_columns, column + 1, new_numbers)
}

fn recurse_co2<'a>(by_columns: &Vec<Vec<i32>>, column: usize, numbers: Vec<Vec<i32>>) -> Vec<i32> {
    if numbers.len() == 1 {
        return numbers[0].clone();
    }

    let (c0, c1) = count_bits(&by_columns[column]);
    let keep_digit;
    if c1 == c0 {
        keep_digit = 0;
    } else if c1 > c0 {
        keep_digit = 0;
    } else {
        keep_digit = 1;
    }

    let mut new_numbers = Vec::new();
    for number in numbers {
        if number[column] == keep_digit {
            new_numbers.push(number.clone());
        }
    }

    let first_line = &new_numbers[0];
    let mut by_columns = Vec::with_capacity(first_line.len());
    for _ in first_line {
        by_columns.push(Vec::<i32>::new())
    }
    for number in &new_numbers {
        for (i, &digit) in number.iter().enumerate() {
            by_columns[i].push(digit);
        }
    }

    println!("c0 {}, c1 {}", c0, c1);
    dbg!("new_numbers {}", &new_numbers);
    recurse_co2(&by_columns, column + 1, new_numbers)
}

fn part_two(input: &str) -> usize {
    let lines: Vec<_> = input.split("\n").collect();

    let first_line = &lines[0];
    let mut grid = Vec::with_capacity(first_line.len());
    for _ in first_line.chars() {
        grid.push(Vec::<i32>::new())
    }

    let mut numbers = Vec::new();
    for &line in &lines {
        let mut num = Vec::new();
        for (i, c) in line.chars().enumerate() {
            let digit = c.to_string().parse().unwrap();
            grid[i].push(digit);
            num.push(digit);
        }
        numbers.push(num);
    }

    let o2 = recurse_oxygen(&grid, 0, numbers.clone());
    let co2 = recurse_co2(&grid, 0, numbers.clone());

    convert_to_decimal(o2) * convert_to_decimal(co2)
}

fn main() {
    let input = include_str!("day3.txt");
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("day3-example.txt");
        assert_eq!(part_one(input), 198);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("day3-example.txt");
        println!("recurse co2");
        assert_eq!(part_two(input), 230);
    }
}
