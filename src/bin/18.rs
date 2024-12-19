use std::{collections::VecDeque, usize};

advent_of_code::solution!(18);

#[derive(Clone)]
enum Cell {
    Empty(usize),
    Corrupted,
}

struct Grid {
    width: usize,
    height: usize,
    falling_bytes: Vec<(usize, usize)>,
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    fn from_input(input: &str, width: usize, height: usize) -> Self {
        let cells = vec![vec![Cell::Empty(usize::MAX); width]; height];
        let mut falling_bytes = Vec::new();
        // Each line is a comma-separated coordinate pair that represents a falling byte.
        for line in input.lines() {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            falling_bytes.push((x, y));
        }
        Grid {
            width,
            height,
            falling_bytes,
            cells,
        }
    }

    fn reset_empty_cells(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Cell::Empty(steps) = self.cells[y][x] {
                    self.cells[y][x] = Cell::Empty(usize::MAX);
                }
            }
        }
    }

    fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        self.cells[y][x] = cell;
    }

    fn cell_is_passable(&self, x: usize, y: usize) -> bool {
        match self.cells[y][x] {
            Cell::Empty(_) => true,
            Cell::Corrupted => false,
        }
    }

    fn simulate_falling_bytes(&mut self, num_bytes: usize) {
        for index in 0..num_bytes {
            let (x, y) = *self.falling_bytes.get(index).unwrap();
            self.set_cell(x, y, Cell::Corrupted);
        }
    }

    fn find_min_steps_to_coord(&mut self, end_x: usize, end_y: usize) -> Option<usize> {
        if end_x >= self.width || end_y >= self.height {
            return None;
        }
        let mut min_steps = usize::MAX;
        let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
        queue.push_back((0, 0, 0));
        while let Some((x, y, steps)) = queue.pop_front() {
            if x == end_x && y == end_y {
                if steps < min_steps {
                    min_steps = steps;
                }
                continue;
            }

            if self.cell_is_passable(x, y) {
                let cell_cost = match self.cells[y][x] {
                    Cell::Empty(cost) => cost,
                    Cell::Corrupted => usize::MAX,
                };

                if cell_cost <= steps {
                    continue;
                } else {
                    self.set_cell(x, y, Cell::Empty(steps));
                }

                if x > 0 {
                    queue.push_back((x - 1, y, steps + 1));
                }
                if x < self.width - 1 {
                    queue.push_back((x + 1, y, steps + 1));
                }
                if y > 0 {
                    queue.push_back((x, y - 1, steps + 1));
                }
                if y < self.height - 1 {
                    queue.push_back((x, y + 1, steps + 1));
                }
            }
        }

        if min_steps == usize::MAX {
            None
        } else {
            Some(min_steps)
        }
    }

    fn find_first_blocking_byte(&mut self) -> Option<(usize, usize)> {
        let cloned_bytes = self.falling_bytes.clone();
        for (x, y) in cloned_bytes {
            self.set_cell(x, y, Cell::Corrupted);
            self.reset_empty_cells();
            if self
                .find_min_steps_to_coord(self.width - 1, self.height - 1)
                .is_none()
            {
                return Some((x, y));
            }
        }
        None
    }
}

pub fn part_one_internal(
    input: &str,
    width: usize,
    height: usize,
    num_bytes: usize,
) -> Option<usize> {
    let mut grid = Grid::from_input(input, width + 1, height + 1);
    grid.simulate_falling_bytes(num_bytes);
    grid.find_min_steps_to_coord(width, height)
}

pub fn part_two_internal(input: &str, width: usize, height: usize) -> Option<String> {
    let mut grid = Grid::from_input(input, width + 1, height + 1);
    let blocking_byte = grid.find_first_blocking_byte();
    if let Some((x, y)) = blocking_byte {
        Some(format!("{},{}", x, y))
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    part_one_internal(input, 70, 70, 1024)
}

pub fn part_two(input: &str) -> Option<String> {
    part_two_internal(input, 70, 70)
}

#[cfg(test)]
mod tests_18 {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one_internal(&input, 6, 6, 12);
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two_internal(&input, 6, 6);
        assert_eq!(result, Some("6,1".to_string()));
    }
}
