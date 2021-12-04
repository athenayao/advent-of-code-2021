use std::collections::HashMap;

#[derive(Debug)]
struct BingoCard {
    grid: Vec<Vec<i32>>,
    checked: HashMap<String, i32>,
}

impl BingoCard {
    fn build() -> BingoCard {
        BingoCard {
            grid: Vec::new(),
            checked: HashMap::new(),
        }
    }

    fn add_line(&mut self, line: &str) {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse().expect("Expected number"))
            .collect();
        self.grid.push(numbers);
    }

    fn mark_and_check(&mut self, number: i32, x: usize, y: usize) -> bool {
        if number != self.grid[y][x] {
            return false;
        }
        self.grid[y][x] = -1;

        let x_val = self.checked.entry(format!("x{}", x)).or_insert(0);
        *x_val += 1;
        if *x_val == 5 {
            return true;
        }

        let y_val = self.checked.entry(format!("y{}", y)).or_insert(0);
        *y_val += 1;
        if *y_val == 5 {
            return true;
        }

        false
    }

    fn count_unmarked(&self) -> i32 {
        // sum of all unmarked numbers
        let mut sum = 0;
        for row in &self.grid {
            for item in row {
                if *item > 0 {
                    sum += item;
                }
            }
        }
        sum
    }
}

fn part_one(input: &str) -> i32 {
    let lines: Vec<&str> = input.split("\n").collect();

    let drawn_numbers: Vec<i32> = lines[0]
        .split(",")
        .map(|n| n.parse().expect("Expected number"))
        .collect();

    let mut bingo_cards: Vec<BingoCard> = Vec::new();
    let mut current_card: BingoCard = BingoCard::build();
    for &line in &lines[1..] {
        match line {
            "" => {
                if current_card.grid.len() > 0 {
                    bingo_cards.push(current_card)
                }
                current_card = BingoCard::build()
            }
            line => current_card.add_line(line),
        }
    }

    for number in drawn_numbers {
        for bingo_card in &mut bingo_cards {
            for y in 0..5 {
                for x in 0..5 {
                    if bingo_card.mark_and_check(number, x, y) {
                        return bingo_card.count_unmarked() * number;
                    }
                }
            }
        }
    }
    0
}

fn part_two(_input: &str) -> i32 {
    0
}

fn main() {
    let input = include_str!("day4.txt");
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("day4-example.txt");
        assert_eq!(part_one(input), 4512);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("day4-example.txt");
        assert_eq!(part_two(input), 0);
    }
}
