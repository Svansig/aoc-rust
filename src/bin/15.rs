use core::panic;

advent_of_code::solution!(15);

#[derive(Clone, Copy)]
enum MoveDir {
    Up,
    Down,
    Left,
    Right,
}

impl MoveDir {
    fn get_next_move(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            MoveDir::Up => (x, y - 1),
            MoveDir::Down => (x, y + 1),
            MoveDir::Left => (x - 1, y),
            MoveDir::Right => (x + 1, y),
        }
    }
    fn from_char(c: char) -> MoveDir {
        match c {
            '^' => MoveDir::Up,
            'v' => MoveDir::Down,
            '<' => MoveDir::Left,
            '>' => MoveDir::Right,
            _ => panic!("Invalid move: {}", c),
        }
    }
}

#[derive(Clone)]
enum Cell {
    Empty,
    Wall,
    WarehouseBox(WarehouseBox),
    DoubleSizeBoxLeft(WarehouseBox),
    DoubleSizeBoxRight(WarehouseBox),
    Robot(Robot),
}

impl Cell {
    fn from_char(x: usize, y: usize, c: char) -> Cell {
        match c {
            '.' => Cell::Empty,
            '#' => Cell::Wall,
            'O' => Cell::WarehouseBox(WarehouseBox { x, y }),
            '[' => Cell::DoubleSizeBoxLeft(WarehouseBox { x, y }),
            ']' => Cell::DoubleSizeBoxRight(WarehouseBox { x, y }),
            '@' => Cell::Robot(Robot {
                x,
                y,
                move_index: 0,
                remaining_moves: Vec::new(),
            }),
            _ => panic!("Invalid cell: {}", c),
        }
    }

    fn get_gps(&self) -> usize {
        match self {
            Cell::WarehouseBox(b) => b.x + 100 * b.y,
            Cell::DoubleSizeBoxLeft(b) => b.x + 100 * b.y,
            _ => 0,
        }
    }

    fn move_to(&mut self, x: usize, y: usize) {
        match self {
            Cell::WarehouseBox(b) => b.move_to(x, y),
            Cell::DoubleSizeBoxLeft(b) => b.move_to(x, y),
            Cell::DoubleSizeBoxRight(b) => b.move_to(x, y),
            Cell::Robot(r) => r.move_to(x, y),
            _ => (),
        }
    }

    fn get_name(&self) -> &str {
        match self {
            Cell::Empty => "Empty",
            Cell::Wall => "Wall",
            Cell::WarehouseBox(_) => "WarehouseBox",
            Cell::DoubleSizeBoxLeft(_) => "DoubleSizeBoxLeft",
            Cell::DoubleSizeBoxRight(_) => "DoubleSizeBoxRight",
            Cell::Robot(_) => "Robot",
        }
    }
}

#[derive(Clone)]
struct Robot {
    x: usize,
    y: usize,
    move_index: usize,
    remaining_moves: Vec<MoveDir>,
}

impl Robot {
    fn get_next_move(&mut self) -> Option<&MoveDir> {
        let next_move = self.remaining_moves.get(self.move_index);
        self.move_index += 1;
        next_move
    }

    fn move_to(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
}

#[derive(Clone)]
struct WarehouseBox {
    x: usize,
    y: usize,
}

impl WarehouseBox {
    fn move_to(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
}

struct Warehouse {
    cells: Vec<Vec<Cell>>,
    robot: Robot,
}

impl Warehouse {
    fn from_str(input: &str) -> Warehouse {
        // First, break by a double newline
        let mut cells = Vec::new();
        let mut split = input.split("\n\n");
        let grid = split.next().unwrap().lines();
        let directions = split.next().unwrap();

        for (y, line) in grid.enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                let cell = Cell::from_char(x, y, c);
                if let Cell::Robot(robot) = cell {
                    let mut remaining_moves = Vec::new();
                    for c in directions.chars() {
                        // Skip newlines
                        if c == '\n' {
                            continue;
                        }
                        remaining_moves.push(MoveDir::from_char(c));
                    }
                    row.push(Cell::Robot(Robot {
                        x: robot.x,
                        y: robot.y,
                        remaining_moves,
                        move_index: 0,
                    }));
                } else {
                    row.push(cell);
                }
            }
            cells.push(row);
        }

        // Find the robot
        let robot = cells
            .iter()
            .flat_map(|row| row.iter())
            .find_map(|cell| match cell {
                Cell::Robot(robot) => Some(robot.clone()),
                _ => None,
            })
            .unwrap();

        // Replace the robot cell with an empty cell
        cells[robot.y][robot.x] = Cell::Empty;

        Warehouse { cells, robot }
    }

    fn get_cell(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y][x]
    }

    fn move_cells(&mut self, x: usize, y: usize, dir: MoveDir, other_half_checked: bool) {
        let cell = self.get_cell(x, y).clone();
        if let Cell::Empty = cell {
            return;
        }
        if let Cell::Wall = cell {
            return;
        }
        if !other_half_checked {
            if let Cell::DoubleSizeBoxLeft(_) = cell {
                self.move_cells(x + 1, y, dir, true);
            }
            if let Cell::DoubleSizeBoxRight(_) = cell {
                self.move_cells(x - 1, y, dir, true);
            }
        }
        let (next_x, next_y) = dir.get_next_move(x, y);
        self.cells[y][x] = Cell::Empty;
        self.move_cells(next_x, next_y, dir, false);
        self.cells[next_y][next_x] = cell.clone();
        self.cells[next_y][next_x].move_to(next_x, next_y);
    }

    fn try_move_cell_in_dir(
        &mut self,
        x: usize,
        y: usize,
        dir: MoveDir,
        other_half_checked: bool,
    ) -> bool {
        let cell = self.get_cell(x, y).clone();
        // If it's empty, return true
        if let Cell::Empty = cell {
            return true;
        }
        if let Cell::Wall = cell {
            return false;
        }
        let (next_x, next_y) = dir.get_next_move(x, y);
        // If it's a right side of a double box moving left, or a left side of a double box moving right
        // Then we should set that the other half has been checked
        let is_pushing_in_dir = match (cell.clone(), dir) {
            (Cell::DoubleSizeBoxLeft(_), MoveDir::Right) => true,
            (Cell::DoubleSizeBoxRight(_), MoveDir::Left) => true,
            _ => false,
        };
        let mut next_can_move = self.try_move_cell_in_dir(next_x, next_y, dir, is_pushing_in_dir);
        if !is_pushing_in_dir && !other_half_checked {
            if let Cell::DoubleSizeBoxLeft(_) = cell {
                next_can_move = next_can_move && self.try_move_cell_in_dir(x + 1, y, dir, true);
            }
            if let Cell::DoubleSizeBoxRight(_) = cell {
                next_can_move = next_can_move && self.try_move_cell_in_dir(x - 1, y, dir, true);
            }
        }
        if next_can_move {
            return true;
        }
        false
    }

    fn try_move(&mut self) {
        // Get the next move from the robot
        // Then check in the direction of the move if it's possible
        // Boxes can be pushed
        // If the move is possible, update the robot's position
        // If the move is not possible, do nothing
        let (robot_x, robot_y) = (self.robot.x, self.robot.y);
        let next_move = self.robot.get_next_move();
        if next_move.is_none() {
            return;
        }
        let next_move = *next_move.unwrap();
        let (x, y) = next_move.get_next_move(robot_x, robot_y);
        if self.try_move_cell_in_dir(x, y, next_move, false) {
            self.move_cells(x, y, next_move, false);
            self.robot.move_to(x, y);
        };

        self.try_move();
    }

    fn get_gps(&self) -> usize {
        self.cells
            .iter()
            .flat_map(|row| row.iter())
            .map(|cell| cell.get_gps())
            .sum()
    }

    fn print(&self) {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if x == self.robot.x && y == self.robot.y {
                    print!("@");
                    continue;
                }
                match cell {
                    Cell::Empty => print!("."),
                    Cell::Wall => print!("#"),
                    Cell::WarehouseBox(_) => print!("O"),
                    Cell::Robot(_) => print!("@"),
                    Cell::DoubleSizeBoxLeft(_) => print!("["),
                    Cell::DoubleSizeBoxRight(_) => print!("]"),
                }
            }
            println!();
        }
    }
}

fn double_size_string(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        match c {
            '#' => {
                result.push('#');
                result.push('#');
            }
            'O' => {
                result.push('[');
                result.push(']');
            }
            '@' => {
                result.push('@');
                result.push('.');
            }
            '.' => {
                result.push('.');
                result.push('.');
            }
            '\n' => {
                result.push('\n');
            }
            _ => panic!("Invalid char: {}", c),
        }
    }
    result
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut warehouse = Warehouse::from_str(input);
    warehouse.try_move();
    warehouse.print();
    Some(warehouse.get_gps())
}

pub fn part_two(input: &str) -> Option<usize> {
    // Split at the double newline
    let mut split = input.split("\n\n");
    let doubled = double_size_string(split.next().unwrap());
    let directions = split.next().unwrap();
    let input = format!("{}\n\n{}", doubled, directions);
    let mut warehouse = Warehouse::from_str(&input);
    // warehouse.print();
    warehouse.try_move();
    warehouse.print();
    Some(warehouse.get_gps())
}

#[cfg(test)]
mod tests_15 {
    use super::*;

    #[test]
    fn test_part_one_big() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(10092));
    }
    #[test]
    fn test_part_one_small() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_two_big() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(9021));
    }

    #[test]
    fn test_part_two_small() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(618));
    }
}
