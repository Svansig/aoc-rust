use std::{cell::RefCell, rc::Rc};

advent_of_code::solution!(10);

enum NextStep {
    Up,
    Down,
    Left,
    Right,
}

impl NextStep {
    fn get_next_steps() -> Vec<NextStep> {
        vec![
            NextStep::Up,
            NextStep::Down,
            NextStep::Left,
            NextStep::Right,
        ]
    }

    fn get_next(&self, y: isize, x: isize) -> (isize, isize) {
        match self {
            NextStep::Up => (y - 1, x),
            NextStep::Down => (y + 1, x),
            NextStep::Left => (y, x - 1),
            NextStep::Right => (y, x + 1),
        }
    }
}

#[derive(Debug, PartialEq)]
struct TrailTile<'a> {
    x: usize,
    y: usize,
    height: u8,
    score: usize,
    rating: usize,
    next_steps: Vec<&'a Rc<RefCell<TrailTile<'a>>>>,
}

impl<'a> TrailTile<'a> {
    fn new(x: usize, y: usize, height: u8) -> Self {
        Self {
            x,
            y,
            height,
            score: usize::MAX,
            rating: usize::MAX,
            next_steps: vec![],
        }
    }

    fn get_coords(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn is_trailhead(&self) -> bool {
        self.height == 0
    }

    fn is_top(&self) -> bool {
        self.height == 9
    }

    fn is_passable(&self, next: &TrailTile) -> bool {
        next.height > self.height && next.height - self.height == 1
    }

    fn add_next_step(&mut self, next_step: &'a Rc<RefCell<TrailTile<'a>>>) {
        self.next_steps.push(next_step);
    }

    fn get_reachable_tops(&self) -> Vec<(usize, usize)> {
        if self.is_top() {
            return vec![self.get_coords()];
        }
        let mut reachable_tops = vec![];
        for next_step in &self.next_steps {
            if next_step.borrow().is_top() {
                let coords = next_step.borrow().get_coords();
                if !reachable_tops.contains(&coords) {
                    reachable_tops.push(next_step.borrow().get_coords());
                }
            } else {
                let next_coords = next_step.borrow().get_reachable_tops();
                for coords in next_coords {
                    if !reachable_tops.contains(&coords) {
                        reachable_tops.push(coords);
                    }
                }
            }
        }
        reachable_tops
    }

    fn get_score(&mut self) -> usize {
        if self.is_top() {
            self.score = 1;
            return self.score;
        }
        if self.score == usize::MAX {
            self.score = self.get_reachable_tops().len();
        };
        self.score
    }

    fn get_rating(&mut self) -> usize {
        if self.is_top() {
            return 1;
        }
        if self.rating == usize::MAX {
            let mut rating = 0;
            for next_step in &self.next_steps {
                rating += next_step.borrow_mut().get_rating();
            }
            self.rating = rating;
        }
        self.rating
    }
}

struct TrailMap<'a> {
    width: usize,
    height: usize,
    map: Vec<Vec<Rc<RefCell<TrailTile<'a>>>>>,
}

impl<'a> TrailMap<'a> {
    fn from_input(input: &str) -> Self {
        let map: Vec<Vec<Rc<RefCell<TrailTile<'a>>>>> = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        Rc::new(RefCell::new(TrailTile::new(
                            x,
                            y,
                            c.to_digit(10).unwrap() as u8,
                        )))
                    })
                    .collect()
            })
            .collect();
        let height = map.len();
        let width = map[0].len();
        Self { map, width, height }
    }

    fn is_in_bounds(&self, y: isize, x: isize) -> bool {
        x >= 0 && y >= 0 && y < self.height as isize && x < self.width as isize
    }

    fn get_tile(&'a self, y: isize, x: isize) -> Option<&'a Rc<RefCell<TrailTile<'a>>>> {
        if !self.is_in_bounds(y, x) {
            return None;
        }
        Some(&self.map[y as usize][x as usize])
    }

    fn connect_tiles(&'a self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let x: isize = x.try_into().unwrap();
                let y: isize = y.try_into().unwrap();
                if let Some(current_tile) = self.get_tile(y, x) {
                    if current_tile.borrow().is_top() {
                        continue;
                    }
                    for next_step in NextStep::get_next_steps() {
                        let (next_y, next_x) = next_step.get_next(y, x);
                        if let Some(next_tile) = self.get_tile(next_y, next_x) {
                            if current_tile.borrow().is_passable(&next_tile.borrow()) {
                                current_tile.borrow_mut().add_next_step(next_tile);
                            }
                        }
                    }
                }
            }
        }
    }

    fn get_trailheads(&self) -> Vec<&Rc<RefCell<TrailTile<'a>>>> {
        self.map
            .iter()
            .flat_map(|row| row.iter())
            .filter(|tile| tile.borrow().is_trailhead())
            .collect()
    }

    fn score_all_trailheads(&self) -> usize {
        let mut score = 0;
        for trailhead in self.get_trailheads() {
            score += trailhead.borrow_mut().get_score();
        }
        score
    }

    fn rate_all_trailheads(&self) -> usize {
        let mut rating = 0;
        for trailhead in self.get_trailheads() {
            rating += trailhead.borrow_mut().get_rating();
        }
        rating
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let trail_map = TrailMap::from_input(input);
    trail_map.connect_tiles();
    let score = trail_map.score_all_trailheads();
    // trail_map.print_trailheads();
    Some(score)
}

pub fn part_two(input: &str) -> Option<usize> {
    let trail_map = TrailMap::from_input(input);
    trail_map.connect_tiles();
    let rating = trail_map.rate_all_trailheads();
    // trail_map.print_trailheads();
    Some(rating)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
