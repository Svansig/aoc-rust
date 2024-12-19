use std::{
    collections::{HashSet, VecDeque},
    fmt::{Debug, Formatter},
    hash::{Hash, Hasher},
};

advent_of_code::solution!(16);

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Facing {
    North,
    East,
    South,
    West,
}

impl Hash for Facing {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Facing::North => 0.hash(state),
            Facing::East => 1.hash(state),
            Facing::South => 2.hash(state),
            Facing::West => 3.hash(state),
        }
    }
}

impl Facing {
    fn get_all_facings() -> Vec<Self> {
        vec![Facing::North, Facing::East, Facing::South, Facing::West]
    }
    fn turn_counterclockwise(&self) -> Self {
        match self {
            Facing::North => Facing::West,
            Facing::East => Facing::North,
            Facing::South => Facing::East,
            Facing::West => Facing::South,
        }
    }

    fn turn_clockwise(&self) -> Self {
        match self {
            Facing::North => Facing::East,
            Facing::East => Facing::South,
            Facing::South => Facing::West,
            Facing::West => Facing::North,
        }
    }

    fn get_opposite(&self) -> Self {
        match self {
            Facing::North => Facing::South,
            Facing::East => Facing::West,
            Facing::South => Facing::North,
            Facing::West => Facing::East,
        }
    }

    fn get_next_position(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Facing::North => (x, y - 1),
            Facing::East => (x + 1, y),
            Facing::South => (x, y + 1),
            Facing::West => (x - 1, y),
        }
    }

    fn get_cost_to_change_facing(&self, exit_facing: &Facing) -> usize {
        match (self, exit_facing) {
            (Facing::North, Facing::North) => usize::MAX,
            (Facing::North, Facing::East) => 1000,
            (Facing::North, Facing::South) => 0,
            (Facing::North, Facing::West) => 1000,
            (Facing::East, Facing::North) => 1000,
            (Facing::East, Facing::East) => usize::MAX,
            (Facing::East, Facing::South) => 1000,
            (Facing::East, Facing::West) => 0,
            (Facing::South, Facing::North) => 0,
            (Facing::South, Facing::East) => 1000,
            (Facing::South, Facing::South) => usize::MAX,
            (Facing::South, Facing::West) => 1000,
            (Facing::West, Facing::North) => 1000,
            (Facing::West, Facing::East) => 0,
            (Facing::West, Facing::South) => 1000,
            (Facing::West, Facing::West) => usize::MAX,
        }
    }
}

struct Cost {
    cost: [usize; 4],
}

impl Debug for Cost {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cost: N: {}, E: {}, S: {}, W: {}",
            self.cost[0], self.cost[1], self.cost[2], self.cost[3]
        )
    }
}

impl Cost {
    fn new() -> Self {
        let cost = [usize::MAX; 4];
        Self { cost }
    }

    fn get_cost_from_entry(&self, facing: &Facing) -> usize {
        match facing {
            Facing::North => self.cost[0],
            Facing::East => self.cost[1],
            Facing::South => self.cost[2],
            Facing::West => self.cost[3],
        }
    }

    fn set_cost_from_entry(&mut self, facing: &Facing, cost: usize) -> bool {
        // Only override if the cost is less than the current cost
        let current_cost = self.get_cost_from_entry(facing);

        if cost >= current_cost {
            return false;
        }
        match facing {
            Facing::North => self.cost[0] = cost,
            Facing::East => self.cost[1] = cost,
            Facing::South => self.cost[2] = cost,
            Facing::West => self.cost[3] = cost,
        };
        true
    }
}

#[derive(Debug)]
enum Cell {
    Empty(Cost),
    Wall,
    End(Cost),
    Reindeer(Cost),
}

impl Cell {
    fn get_cost_from_entry(&self, facing: &Facing) -> usize {
        match self {
            Cell::Empty(cost) => cost.get_cost_from_entry(facing),
            Cell::Wall => usize::MAX,
            Cell::End(cost) => cost.get_cost_from_entry(facing),
            Cell::Reindeer(cost) => cost.get_cost_from_entry(facing),
        }
    }

    fn set_cost_from_entry(&mut self, facing: &Facing, new_cost: usize) -> bool {
        match self {
            Cell::Empty(cost) => cost.set_cost_from_entry(facing, new_cost),
            Cell::Wall => false,
            Cell::End(cost) => cost.set_cost_from_entry(facing, new_cost),
            Cell::Reindeer(cost) => cost.set_cost_from_entry(facing, new_cost),
        }
    }

    fn get_minimum_cost(&self) -> usize {
        match self {
            Cell::Empty(cost) => {
                let mut min_cost = usize::MAX;
                for c in cost.cost.iter() {
                    if *c < min_cost {
                        min_cost = *c;
                    }
                }
                min_cost
            }
            Cell::End(cost) => {
                let mut min_cost = usize::MAX;
                for c in cost.cost.iter() {
                    if *c < min_cost {
                        min_cost = *c;
                    }
                }
                min_cost
            }
            Cell::Reindeer(cost) => {
                let mut min_cost = usize::MAX;
                for c in cost.cost.iter() {
                    if *c < min_cost {
                        min_cost = *c;
                    }
                }
                min_cost
            }
            _ => usize::MAX,
        }
    }

    fn get_minimum_cost_to_exit_facing(&self, facing: &Facing) -> usize {
        let mut min_cost = usize::MAX;
        for f in Facing::get_all_facings() {
            let cost = self.get_cost_from_facing_to_exit(&f, facing);
            if cost < min_cost {
                min_cost = cost;
            }
        }
        min_cost
    }

    fn get_cost_from_facing_to_exit(&self, enter: &Facing, exit: &Facing) -> usize {
        self.get_cost_from_entry(enter)
            .saturating_add(enter.get_cost_to_change_facing(exit))
    }

    fn is_passable(&self) -> bool {
        match self {
            Cell::Empty(_) => true,
            Cell::Wall => false,
            Cell::End(_) => true,
            Cell::Reindeer(_) => true,
        }
    }

    fn get_minimum_cost_entries_to_exit_facing(&self, exit_facing: &Facing) -> Vec<Facing> {
        let mut entries = Vec::new();
        match self {
            Cell::Empty(cost) => {
                let min_cost = self.get_minimum_cost_to_exit_facing(exit_facing);
                for f in Facing::get_all_facings() {
                    let cost = self.get_cost_from_facing_to_exit(&f, exit_facing);
                    if cost == min_cost {
                        entries.push(f);
                    }
                }
            }
            Cell::End(cost) => {
                let min_cost = self.get_minimum_cost();
                for f in Facing::get_all_facings() {
                    let cost = self.get_cost_from_entry(&f);
                    if cost == min_cost {
                        entries.push(f);
                    }
                }
            }
            Cell::Reindeer(cost) => {}
            _ => {}
        }
        entries
    }

    fn has_multiple_entries(&self, facing: &Facing) -> bool {
        self.get_minimum_cost_entries_to_exit_facing(facing).len() > 1
    }

    fn is_end(&self) -> bool {
        match self {
            Cell::End(_) => true,
            _ => false,
        }
    }
}

struct Maze {
    cells: Vec<Vec<Cell>>,
}

impl Maze {
    fn from_input(input: &str) -> Self {
        let mut cells = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                let cell = match c {
                    '.' => Cell::Empty(Cost::new()),
                    '#' => Cell::Wall,
                    'E' => Cell::End(Cost::new()),
                    'S' => Cell::Reindeer(Cost::new()),
                    _ => panic!("Invalid cell: {}", c),
                };
                row.push(cell);
            }
            cells.push(row);
        }
        Self { cells }
    }

    fn get_cell(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y][x]
    }

    fn get_cell_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        &mut self.cells[y][x]
    }

    fn get_reindeer_cell_position(&self) -> (usize, usize) {
        self.cells
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(|(x, cell)| match cell {
                    Cell::Reindeer(_) => Some((x, y)),
                    _ => None,
                })
            })
            .unwrap()
    }

    fn get_end_cell_position(&self) -> (usize, usize) {
        self.cells
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(|(x, cell)| match cell {
                    Cell::End(_) => Some((x, y)),
                    _ => None,
                })
            })
            .unwrap()
    }

    fn calculate_minimum_cost(&mut self) -> usize {
        // Start from the Reindeer cell
        let current_cell = self.get_reindeer_cell_position();
        let mut queue = VecDeque::new();
        queue.push_back((current_cell, Facing::East, 0));
        while !queue.is_empty() {
            let (current_cell, current_facing, current_cost) = queue.pop_front().unwrap();
            let (x, y) = current_cell;
            let current_cell = self.get_cell_mut(x, y);
            if current_cell.is_end() || !current_cell.is_passable() {
                continue;
            }

            // Find the next move
            let next_position = current_facing.get_next_position(x, y);
            let next_cell = self.get_cell_mut(next_position.0, next_position.1);
            let next_cost = current_cost + 1;
            if next_cell.set_cost_from_entry(&current_facing.get_opposite(), next_cost) {
                queue.push_front((next_position, current_facing, next_cost));
            }
            let cw_facing = current_facing.turn_clockwise();
            let cw_position = cw_facing.get_next_position(x, y);
            let cw_cell = self.get_cell_mut(cw_position.0, cw_position.1);
            let cw_cost = current_cost + 1001;
            if cw_cell.set_cost_from_entry(&cw_facing.get_opposite(), cw_cost) {
                queue.push_back((cw_position, cw_facing, cw_cost));
            }
            let ccw_facing = current_facing.turn_counterclockwise();
            let ccw_position = ccw_facing.get_next_position(x, y);
            let ccw_cell = self.get_cell_mut(ccw_position.0, ccw_position.1);
            let ccw_cost = current_cost + 1001;
            if ccw_cell.set_cost_from_entry(&ccw_facing.get_opposite(), ccw_cost) {
                queue.push_back((ccw_position, ccw_facing, ccw_cost));
            }
            // let rev_facing = current_facing.get_opposite();
            // let rev_position = rev_facing.get_next_position(x, y);
            // let rev_cell = self.get_cell_mut(rev_position.0, rev_position.1);
            // let rev_cost = current_cost + 2001;
            // if rev_cell.set_cost_from_entry(&rev_facing.get_opposite(), rev_cost) {
            //     queue.push_back((rev_position, rev_facing, rev_cost));
            // }
        }
        self.get_cell(
            self.get_end_cell_position().0,
            self.get_end_cell_position().1,
        )
        .get_minimum_cost()
    }

    fn count_cells_along_path(&self) -> usize {
        // This should start at the end cell and follow the path backwards, taking all of the cells that have a minimum cost
        let mut visited_dir = HashSet::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let end_cell = self.get_end_cell_position();
        let reindeer_cell = self.get_reindeer_cell_position();
        visited_dir.insert((reindeer_cell, Facing::East));
        visited.insert(reindeer_cell);
        queue.push_back((end_cell, Facing::East));
        queue.push_back((end_cell, Facing::North));
        queue.push_back((end_cell, Facing::South));
        queue.push_back((end_cell, Facing::West));
        while !queue.is_empty() {
            let current_cell = queue.pop_front().unwrap();
            if visited_dir.contains(&current_cell) {
                continue;
            }
            visited_dir.insert(current_cell);

            let ((x, y), facing) = current_cell;
            visited.insert((x, y));
            let current_cell = self.get_cell(x, y);

            let current_facings: Vec<Facing> =
                current_cell.get_minimum_cost_entries_to_exit_facing(&facing);
            if current_cell.has_multiple_entries(&facing) {
                // If there are multiple entries, we need to add the current cell to the queue
                println!("Multiple entries: ({}, {})", x, y);
                println!("Current Facings: {:?}", current_facings);
            }

            for current_facing in current_facings {
                let (nx, ny) = current_facing.get_next_position(x, y);
                queue.push_back(((nx, ny), current_facing.get_opposite()));
            }
        }

        visited.len() // Add one to account for the reindeer cell
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut maze = Maze::from_input(input);
    Some(maze.calculate_minimum_cost())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut maze = Maze::from_input(input);
    maze.calculate_minimum_cost();
    Some(maze.count_cells_along_path())
}

#[cfg(test)]
mod tests_16 {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11048));
    }
    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(45));
    }
    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(64));
    }
}
