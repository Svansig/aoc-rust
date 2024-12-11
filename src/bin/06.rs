advent_of_code::solution!(6);

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Clone)]
enum Position {
    Occupied,
    Empty,
    Visited(Vec<Direction>),
}

#[derive(Debug, PartialEq, Clone)]
struct BuildingMap {
    positions: Vec<Vec<Position>>,
}

impl BuildingMap {
    fn from_input(input: &str) -> Self {
        let mut map = BuildingMap::new();

        for (y, line) in input.lines().enumerate() {
            if y == map.positions.len() {
                map.positions.push(Vec::with_capacity(line.len()));
            }
            if map.positions[y].len() < line.len() {
                map.positions[y].resize(line.len(), Position::Empty);
            }
            for (x, c) in line.chars().enumerate() {
                let position = match c {
                    '#' => Position::Occupied,
                    '.' => Position::Empty,
                    '^' => Position::Visited(vec![Direction::North]),
                    _ => panic!("Invalid character in input: {}", c),
                };

                map.set(x, y, position);
            }
        }

        map
    }

    fn new() -> Self {
        BuildingMap {
            positions: Vec::new(),
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<&Position> {
        self.positions.get(y).and_then(|row| row.get(x))
    }

    fn set(&mut self, x: usize, y: usize, position: Position) {
        let row = self.positions.get_mut(y).unwrap();
        row[x] = position;
    }

    fn get_possible_obstructable_positions(&self) -> Vec<(usize, usize)> {
        let mut obstructable_positions = Vec::new();
        for (y, row) in self.positions.iter().enumerate() {
            for (x, position) in row.iter().enumerate() {
                match position {
                    Position::Empty => {
                        obstructable_positions.push((x, y));
                    }
                    _ => {}
                }
            }
        }
        obstructable_positions
    }

    fn get_visited_count(&self) -> usize {
        self.positions
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|position| matches!(position, Position::Visited(_)))
                    .count()
            })
            .sum()
    }
}

struct Guard {
    position: (usize, usize),
    facing: Direction,
    map: BuildingMap,
    steps_taken: usize,
}

impl Guard {
    fn new(input: &str) -> Self {
        let map = BuildingMap::from_input(input);

        let position = map
            .positions
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(|(x, position)| {
                    if let Position::Visited(_) = position {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .unwrap();

        Guard {
            position,
            facing: Direction::North,
            map,
            steps_taken: 0,
        }
    }

    fn clone(&self) -> Self {
        Guard {
            position: self.position.clone(),
            facing: self.facing.clone(),
            map: self.map.clone(),
            steps_taken: self.steps_taken,
        }
    }

    fn set_obstruction(&mut self, x: usize, y: usize) -> bool {
        match self.map.get(x, y) {
            Some(Position::Empty) => {
                self.map.set(x, y, Position::Occupied);
                return true;
            }
            Some(Position::Occupied) | Some(Position::Visited(_)) => {
                return false;
            }
            None => {
                return false;
            }
        };
    }

    fn get_possible_obstructable_positions(&self) -> Vec<(usize, usize)> {
        self.map.get_possible_obstructable_positions()
    }

    fn get_next_position(&mut self) -> Option<(usize, usize)> {
        let (x, y) = self.position;

        match self.facing {
            Direction::North => {
                if y == 0 {
                    return None;
                }
                Some((x, y - 1))
            }
            Direction::East => {
                if x == self.map.positions[y].len() - 1 {
                    return None;
                }
                Some((x + 1, y))
            }
            Direction::South => {
                if y == self.map.positions.len() - 1 {
                    return None;
                }
                Some((x, y + 1))
            }
            Direction::West => {
                if x == 0 {
                    return None;
                }
                Some((x - 1, y))
            }
        }
    }

    fn turn_right(&mut self) {
        self.facing = match self.facing {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
    }

    fn move_next(&mut self) {
        if let Some((x, y)) = self.get_next_position() {
            match self.map.get(x, y) {
                Some(Position::Empty) => {
                    self.map
                        .set(x, y, Position::Visited(vec![self.facing.clone()]));
                    self.position = (x, y);
                    self.steps_taken += 1;
                }
                Some(Position::Visited(dirs)) => {
                    let mut new_dirs = dirs.clone();
                    new_dirs.push(self.facing.clone());
                    self.map.set(x, y, Position::Visited(new_dirs));
                    self.position = (x, y);
                }
                Some(Position::Occupied) => {
                    self.turn_right();
                    self.move_next();
                }
                None => {
                    return;
                }
            }
            self.move_next();
        };
    }

    fn check_walk_loop(&mut self) -> bool {
        if let Some((x, y)) = self.get_next_position() {
            match self.map.get(x, y) {
                Some(Position::Visited(dirs)) => {
                    if dirs.contains(&self.facing) {
                        return true;
                    } else {
                        let mut new_dirs = dirs.clone();
                        new_dirs.push(self.facing.clone());
                        self.map.set(x, y, Position::Visited(new_dirs));
                        self.position = (x, y);
                        self.steps_taken += 1;
                        return self.check_walk_loop();
                    }
                }
                Some(Position::Empty) => {
                    self.map
                        .set(x, y, Position::Visited(vec![self.facing.clone()]));
                    self.position = (x, y);
                    self.steps_taken += 1;
                    return self.check_walk_loop();
                }
                Some(Position::Occupied) => {
                    self.turn_right();
                    return self.check_walk_loop();
                }
                None => {
                    return false;
                }
            }
        };
        return false;
    }

    fn get_visited_count(&self) -> usize {
        self.map.get_visited_count()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut guard = Guard::new(input);
    guard.move_next();
    Some(guard.get_visited_count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut possible_obstruction_spots_count = 0;
    let guard = Guard::new(input);

    for (x, y) in guard.get_possible_obstructable_positions() {
        let mut guard_clone = guard.clone();
        if guard_clone.set_obstruction(x, y) {
            if guard_clone.check_walk_loop() {
                possible_obstruction_spots_count += 1;
            }
        }
    }

    Some(possible_obstruction_spots_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
