struct LanternFish {
    timer: u32,
}
impl LanternFish {
    fn tick(&mut self) -> bool {
        let old_timer = self.timer;

        self.timer = match &self.timer {
            0 => 6,
            x => x - 1,
        };

        if old_timer == 0 {
            return true;
        }
        false
    }
}

impl std::fmt::Display for LanternFish {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.timer)
    }
}

fn part_one(input: &str, days: u32) -> u64 {
    let mut school = Vec::new();
    for fish in input.split(",") {
        school.push(LanternFish {
            timer: fish.parse().unwrap(),
        })
    }

    for _ in 1..=days {
        let mut to_spawn = 0;
        for fish in &mut school {
            let should_spawn = fish.tick();
            if should_spawn {
                to_spawn += 1;
            }
        }
        for _ in 0..to_spawn {
            school.push(LanternFish { timer: 8 });
        }
    }
    school.len() as u64
}

fn part_two(input: &str, days: u32) -> u64 {
    let mut timers = vec![0; 9];
    for fish in input.split(",") {
        let i: usize = fish.parse().unwrap();
        timers[i] += 1;
    }

    for _ in 1..=days {
        let mut current_timers = vec![0; 9];
        for i in 0..timers.len() {
            current_timers[i] = timers[(i + 1) % timers.len()];
        }
        current_timers[6] += timers[0];
        // current_timers[8] = timers[0]; -- occurs due to the % above
        timers = current_timers;
    }
    timers.iter().sum()
}

fn main() {
    let input = include_str!("day6.txt");
    println!("Part 1: {}", part_one(input, 80));
    println!("Part 2: {}", part_two(input, 256));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("day6-example.txt");
        assert_eq!(part_one(input, 18), 26);
        assert_eq!(part_one(input, 80), 5934);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("day6-example.txt");
        assert_eq!(part_two(input, 18), 26);
        assert_eq!(part_two(input, 80), 5934);
        assert_eq!(part_two(input, 256), 26984457539);
    }
}
