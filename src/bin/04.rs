advent_of_code::solution!(4);

struct WordSearch {
    words: Vec<String>,
    grid: Vec<Vec<char>>,
}

#[derive(Debug, Clone, Copy)]
enum WordDirections {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl WordSearch {
    fn parse_input(input: &str) -> Vec<Vec<char>> {
        input.lines().map(|line| line.chars().collect()).collect()
    }
    fn new(words: Vec<String>, grid: Vec<Vec<char>>) -> Self {
        Self { words, grid }
    }

    fn get_next_char_in_direction(
        &self,
        x: usize,
        y: usize,
        direction: WordDirections,
        mut prefix: Vec<char>,
        remaining: usize,
    ) -> Vec<char> {
        if remaining == 0 {
            return prefix;
        }
        let current_char = self.grid[y][x];
        prefix.push(current_char);
        match direction {
            WordDirections::Up => {
                if y == 0 {
                    return prefix;
                }
                self.get_next_char_in_direction(x, y - 1, direction, prefix, remaining - 1)
            }
            WordDirections::Down => {
                if y == self.grid.len() - 1 {
                    return prefix;
                }
                self.get_next_char_in_direction(x, y + 1, direction, prefix, remaining - 1)
            }
            WordDirections::Left => {
                if x == 0 {
                    return prefix;
                }
                self.get_next_char_in_direction(x - 1, y, direction, prefix, remaining - 1)
            }
            WordDirections::Right => {
                if x == self.grid[y].len() - 1 {
                    return prefix;
                }
                self.get_next_char_in_direction(x + 1, y, direction, prefix, remaining - 1)
            }
            WordDirections::UpLeft => {
                if x == 0 || y == 0 {
                    return prefix;
                }
                self.get_next_char_in_direction(x - 1, y - 1, direction, prefix, remaining - 1)
            }
            WordDirections::UpRight => {
                if x == self.grid[y].len() - 1 || y == 0 {
                    return prefix;
                }
                self.get_next_char_in_direction(x + 1, y - 1, direction, prefix, remaining - 1)
            }
            WordDirections::DownLeft => {
                if x == 0 || y == self.grid.len() - 1 {
                    return prefix;
                }
                self.get_next_char_in_direction(x - 1, y + 1, direction, prefix, remaining - 1)
            }
            WordDirections::DownRight => {
                if x == self.grid[y].len() - 1 || y == self.grid.len() - 1 {
                    return prefix;
                }
                self.get_next_char_in_direction(x + 1, y + 1, direction, prefix, remaining - 1)
            }
        }
    }

    fn get_words_in_directions(&self, x: usize, y: usize, length: usize) -> Vec<String> {
        let mut result = Vec::new();
        for direction in &[
            WordDirections::Up,
            WordDirections::Down,
            WordDirections::Left,
            WordDirections::Right,
            WordDirections::UpLeft,
            WordDirections::UpRight,
            WordDirections::DownLeft,
            WordDirections::DownRight,
        ] {
            let word = self.get_next_char_in_direction(x, y, *direction, Vec::new(), length);
            result.push(word.iter().collect());
        }
        result
    }

    fn get_x_shape(&self, x: usize, y: usize) -> Option<Vec<String>> {
        let mut result = Vec::new();
        let prefix = Vec::new();
        if x == 0 || y == 0 || x == self.grid[y].len() - 1 || y == self.grid.len() - 1 {
            return None;
        }
        // Start top left (x-1)(y-1) and go to bottom right (x+1)(y+1)
        let tl: String = self
            .get_next_char_in_direction(x - 1, y - 1, WordDirections::DownRight, prefix.clone(), 3)
            .iter()
            .collect();
        let tr: String = self
            .get_next_char_in_direction(x + 1, y - 1, WordDirections::DownLeft, prefix.clone(), 3)
            .iter()
            .collect();
        let bl: String = self
            .get_next_char_in_direction(x - 1, y + 1, WordDirections::UpRight, prefix.clone(), 3)
            .iter()
            .collect();
        let br: String = self
            .get_next_char_in_direction(x + 1, y + 1, WordDirections::UpLeft, prefix.clone(), 3)
            .iter()
            .collect();
        result.push(tl);
        result.push(tr);
        result.push(bl);
        result.push(br);

        Some(result)
    }

    fn find_count_one(&self) -> u32 {
        // This needs to go through each cell in the grid
        // and check against the words provided
        // If that cell contains the first letter of a word
        // then we need to check the surrounding cells
        // to see if the word is there
        let mut count: u32 = 0;
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                for word in &self.words {
                    if self.grid[y][x] == word.chars().next().unwrap() {
                        let words = self.get_words_in_directions(x, y, word.len());
                        for word in words {
                            if self.words.contains(&word) {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
        count
    }

    fn find_count_two(&self) -> u32 {
        // This needs to go through each cell in the grid
        // and check against the words provided
        // to see if the word is there
        let mut count: u32 = 0;
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                for word in &self.words {
                    let words = self.get_x_shape(x, y);
                    match words {
                        Some(words) => {
                            // The word needs to be in there twice
                            let matches = words.iter().filter(|w| *w == word).count();
                            if matches > 1 {
                                count += 1;
                            }
                        }
                        None => {}
                    }
                }
            }
        }
        count
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let words = vec!["XMAS".to_string()];
    let grid = WordSearch::parse_input(input);
    let word_search = WordSearch::new(words, grid);
    Some(word_search.find_count_one())
}

pub fn part_two(input: &str) -> Option<u32> {
    let words = vec!["MAS".to_string()];
    let grid = WordSearch::parse_input(input);
    let word_search = WordSearch::new(words, grid);

    Some(word_search.find_count_two())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2447));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1868));
    }
}
