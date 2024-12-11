use regex::Regex;

advent_of_code::solution!(3);

pub fn parse_input_one(input: &str) -> Vec<(usize, usize)> {
    // We need to go through and match regexes to get the data we need
    // The regex should match on mul(num,num)
    // We can then split on the comma and parse the two numbers
    // We can then return a tuple of the two numbers
    // mul\((\d+),(\d+)\)
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut result = Vec::new();
    let caps = re.captures_iter(input);
    for cap in caps {
        let x = cap[1].parse().unwrap();
        let y = cap[2].parse().unwrap();
        result.push((x, y));
    }
    result
}

enum Instructions {
    // This enables future multiplications
    Do,
    // This disables future multiplications
    Dont,
    // This multiplies the two numbers
    Multiply(usize, usize),
}

struct Machine {
    instructions: Vec<Instructions>,
    multiplication_enabled: bool,
    accumulator: usize,
    index: usize,
}

impl Machine {
    fn parse_instructions(input: &str) -> Vec<Instructions> {
        let mut result = Vec::new();
        // This needs to be a regex that matches on mul(num,num)
        // as well as do() and don't()
        let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
        let caps = re.captures_iter(input);
        for cap in caps {
            if cap.get(1).is_some() {
                let x = cap[1].parse().unwrap();
                let y = cap[2].parse().unwrap();
                result.push(Instructions::Multiply(x, y));
            } else if cap.get(0).unwrap().as_str() == "do()" {
                result.push(Instructions::Do);
            } else if cap.get(0).unwrap().as_str() == "don't()" {
                result.push(Instructions::Dont);
            }
        }
        result
    }
    fn new(instructions: Vec<Instructions>) -> Self {
        Self {
            instructions,
            accumulator: 0,
            index: 0,
            multiplication_enabled: true,
        }
    }

    fn run(&mut self) -> usize {
        while self.index < self.instructions.len() {
            match self.instructions[self.index] {
                Instructions::Do => {
                    self.multiplication_enabled = true;
                }
                Instructions::Dont => {
                    self.multiplication_enabled = false;
                }
                Instructions::Multiply(x, y) => {
                    if !self.multiplication_enabled {
                    } else {
                        self.accumulator += x * y;
                    }
                }
            }
            self.index += 1;
        }
        self.accumulator
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let parsed_tuples = parse_input_one(input);
    Some(parsed_tuples.iter().map(|(x, y)| x * y).sum::<usize>())
}

pub fn part_two(input: &str) -> Option<u32> {
    let instructions = Machine::parse_instructions(input);
    let mut machine = Machine::new(instructions);
    Some(machine.run() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
