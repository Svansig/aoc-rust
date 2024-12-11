use std::collections::HashMap;

advent_of_code::solution!(11);

#[derive(Clone)]
struct Stone {
    value: usize,
    splits: usize,
}

struct Line {
    stones: Vec<Stone>,
    cache: Vec<HashMap<usize, Stone>>,
}

enum Rule {
    Empty,
    Even,
    Default,
}

impl Stone {
    fn new(value: usize, splits: usize) -> Self {
        Self { value, splits }
    }

    fn rule(&self) -> Rule {
        match self.value {
            0 => Rule::Empty,
            _ if self.value.to_string().len() % 2 == 0 => Rule::Even,
            _ => Rule::Default,
        }
    }
}

impl Line {
    fn from_input(input: &str, total_rounds: usize) -> Self {
        let stones = input
            .split_whitespace()
            .map(|s| Stone::new(s.parse().unwrap(), 1))
            .collect();
        // The cache should be a Vec of size total_rounds
        // And each element should be a HashMap of the value to the stone
        let cache = vec![HashMap::new(); total_rounds + 1];
        Self { stones, cache }
    }

    fn apply_stone(&mut self, stone: &Stone, rounds_remaining: usize) -> usize {
        // First check to see if the cache already has the stone
        if let Some(stone) = self.cache[rounds_remaining].get(&stone.value) {
            return stone.splits;
        }

        if rounds_remaining == 0 {
            self.cache[rounds_remaining].insert(stone.value, Stone::new(stone.value, 1));
            return 1;
        }

        let (first, second) = stone.rule().apply(stone);
        let first = self.apply_stone(&first, rounds_remaining - 1);
        if second.is_some() {
            let second = self.apply_stone(&second.unwrap(), rounds_remaining - 1);
            self.cache[rounds_remaining]
                .insert(stone.value, Stone::new(stone.value, first + second));
            first + second
        } else {
            self.cache[rounds_remaining].insert(stone.value, Stone::new(stone.value, first));
            first
        }
    }

    fn blink(&mut self, rounds_remaining: usize) -> usize {
        let mut splits = 0;
        for stone in &self.stones.clone() {
            splits += self.apply_stone(stone, rounds_remaining);
        }
        splits
    }
}

impl Rule {
    fn apply(&self, stone: &Stone) -> (Stone, Option<Stone>) {
        match self {
            Rule::Empty => (Stone::new(1, 1), None),
            Rule::Even => {
                // This needs to split the digits of the number
                // the first half is the first stone, the second half is the second stone
                let value = stone.value.to_string();
                let half = value.len() / 2;
                let first = value[..half].parse().unwrap();
                let second = value[half..].parse().unwrap();
                (Stone::new(first, 1), Some(Stone::new(second, 1)))
            }
            Rule::Default => (Stone::new(stone.value * 2024, 1), None),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let num_rounds = 25;
    let mut line = Line::from_input(input, num_rounds);

    Some(line.blink(num_rounds))
}

pub fn part_two(input: &str) -> Option<usize> {
    let num_rounds = 75;
    let mut line = Line::from_input(input, num_rounds);

    Some(line.blink(num_rounds))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
