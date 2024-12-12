use std::{collections::HashSet, fmt::Debug, fs::File, io::Write};

advent_of_code::solution!(12);

#[derive(Clone, Copy, PartialEq, Debug)]
struct Plant {
    x: usize,
    y: usize,
    kind: char,
    borders: Borders,
}

#[derive(Clone, Debug, PartialEq)]
struct Side {
    plants: Vec<Plant>,
    border: Border,
}

impl Side {
    fn new(border: Border, plant: &Plant) -> Self {
        Self {
            plants: vec![*plant],
            border,
        }
    }

    fn try_connect_side(&mut self, side: &mut Side) -> bool {
        if self.border != side.border {
            return false;
        }
        // If any of the plants touch, then we can connect the sides
        let connected = self
            .plants
            .iter()
            .any(|p| side.plants.iter().any(|p2| p.touches(p2)));
        if connected {
            self.plants.append(&mut side.plants);
            return true;
        }
        false
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Borders(u8);

#[allow(dead_code)]
impl Borders {
    fn new() -> Self {
        Borders(1 | 1 << 1 | 1 << 2 | 1 << 3)
    }

    fn set(&mut self, border: Border) {
        self.0 |= match border {
            Border::Top => 1 << 0,
            Border::Right => 1 << 1,
            Border::Bottom => 1 << 2,
            Border::Left => 1 << 3,
        };
    }

    fn unset(&mut self, border: Border) {
        self.0 &= !(match border {
            Border::Top => 1 << 0,
            Border::Right => 1 << 1,
            Border::Bottom => 1 << 2,
            Border::Left => 1 << 3,
        });
    }

    fn has(&self, border: Border) -> bool {
        (self.0
            & match border {
                Border::Top => 1 << 0,
                Border::Right => 1 << 1,
                Border::Bottom => 1 << 2,
                Border::Left => 1 << 3,
            })
            != 0
    }

    fn get_num_borders(&self) -> usize {
        let mut num_borders = 0;
        for border in Border::iter() {
            if self.has(border) {
                num_borders += 1;
            }
        }
        num_borders
    }

    fn iter(&self) -> impl Iterator<Item = Border> + '_ {
        Border::iter().filter(move |b| self.has(*b))
    }
}

impl Debug for Borders {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut borders = Vec::new();
        for border in Border::iter() {
            if self.has(border) {
                match border {
                    Border::Top => borders.push("U"),
                    Border::Right => borders.push("R"),
                    Border::Bottom => borders.push("D"),
                    Border::Left => borders.push("L"),
                }
            }
        }
        write!(f, "{:?}", borders)
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
enum Border {
    Top,
    Right,
    Bottom,
    Left,
}

impl Border {
    fn iter() -> impl Iterator<Item = Border> {
        [Border::Top, Border::Right, Border::Bottom, Border::Left]
            .iter()
            .copied()
    }

    fn direction(&self) -> (i32, i32) {
        match self {
            Border::Top => (0, -1),
            Border::Right => (1, 0),
            Border::Bottom => (0, 1),
            Border::Left => (-1, 0),
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Border::Top => Border::Bottom,
            Border::Right => Border::Left,
            Border::Bottom => Border::Top,
            Border::Left => Border::Right,
        }
    }
}

#[allow(dead_code)]
impl Plant {
    fn from_char(x: usize, y: usize, c: char) -> Self {
        Self {
            x,
            y,
            kind: c,
            borders: Borders::new(),
        }
    }

    fn is_same_kind(&self, other: &Self) -> bool {
        self.kind == other.kind
    }

    fn get_hash(&self) -> u64 {
        let mut hash: u64 = 0;
        hash |= (self.x as u64) << 32;
        hash |= (self.y as u64) << 16;
        hash |= self.kind as u64;
        hash
    }

    fn touches(&self, other: &Self) -> bool {
        // Check if the plants are touching
        let dx = (self.x as i32 - other.x as i32).abs();
        let dy = (self.y as i32 - other.y as i32).abs();
        match (dx, dy) {
            (0, 0) => false,
            (0, dy) if dy <= 1 => true,
            (dx, 0) if dx <= 1 => true,
            _ => false,
        }
    }

    fn has_border(&self, border: Border) -> bool {
        self.borders.has(border)
    }

    fn set_border(&mut self, border: Border) {
        self.borders.set(border);
    }

    fn unset_border(&mut self, border: Border) {
        self.borders.unset(border);
    }

    fn shares_border(&self, other: &Self, border: Border) -> bool {
        self.has_border(border) && other.has_border(border)
    }

    fn set_border_between(&mut self, other: &mut Self, border: Border) {
        self.unset_border(border);
        other.unset_border(border.opposite());
    }

    fn set_borders_between(&mut self, other: &mut Self) {
        for border in Border::iter() {
            let (dx, dy) = border.direction();
            let x = self.x as i32 + dx;
            let y = self.y as i32 + dy;
            if x < 0 || y < 0 {
                continue;
            }
            let x: usize = x.try_into().unwrap();
            let y: usize = y.try_into().unwrap();
            if x == other.x && y == other.y {
                self.set_border_between(other, border);
            }
        }
    }

    fn get_perimeter(&self) -> usize {
        self.borders.get_num_borders()
    }
}

#[derive(Clone, Debug)]
struct Plot {
    kind: char,
    plants: Vec<Plant>,
    plant_set: HashSet<u64>,
}

impl Plot {
    fn new(kind: char) -> Self {
        Self {
            plants: Vec::new(),
            plant_set: HashSet::new(),
            kind,
        }
    }

    fn get_area(&self) -> usize {
        self.plants.len()
    }

    fn get_plant_mut(&mut self, x: i32, y: i32) -> Option<&mut Plant> {
        if x < 0 || y < 0 {
            return None;
        }
        if !self.contains_plant_at(x, y) {
            return None;
        }
        let x: usize = x.try_into().unwrap();
        let y: usize = y.try_into().unwrap();
        self.plants.iter_mut().find(|p| p.x == x && p.y == y)
    }

    fn contains(&self, plant: &Plant) -> bool {
        self.plant_set.contains(&plant.get_hash())
    }

    fn add_plant(&mut self, plant: &mut Plant) {
        self.plants.push(*plant);
        self.plant_set.insert(plant.get_hash());
        for border in Border::iter() {
            let (dx, dy) = border.direction();
            let x = plant.x as i32 + dx;
            let y = plant.y as i32 + dy;
            if let Some(neighbor) = self.get_plant_mut(x, y) {
                plant.set_border_between(neighbor, border);
            }
        }
    }

    fn try_add_plant(&mut self, plant: &mut Plant) -> bool {
        if self.contains(plant) {
            return false;
        }
        self.add_plant(plant);
        true
    }

    fn get_price_one(&mut self) -> usize {
        self.get_area() * self.calculate_perimeter()
    }

    fn get_price_two(&mut self) -> usize {
        self.get_area() * self.calculate_num_sides()
    }

    fn contains_plant_at(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 {
            return false;
        }
        let x: usize = x.try_into().unwrap();
        let y: usize = y.try_into().unwrap();
        // Create a plant with the same x and y coordinates
        // and check if it is in the plot
        let test_plant = Plant::from_char(x, y, self.kind);
        self.contains(&test_plant)
    }

    fn calculate_perimeter(&mut self) -> usize {
        self.set_perimeters();
        let mut perimeter = 0;
        for plant in self.plants.iter() {
            perimeter += plant.get_perimeter();
        }
        perimeter
    }

    fn set_perimeters(&mut self) {
        let mut next_plants = Vec::new();
        for mut plant in self.plants.clone() {
            for border in Border::iter() {
                let (dx, dy) = border.direction();
                let x = plant.x as i32 + dx;
                let y = plant.y as i32 + dy;
                if self.contains_plant_at(x, y) {
                    plant.unset_border(border);
                }
            }
            next_plants.push(plant);
        }
        self.plants = next_plants;
    }

    fn calculate_num_sides(&mut self) -> usize {
        self.set_perimeters();

        // Eliminate all the plants that have a perimeter of 0
        let mut plants = self.plants.clone();
        // Then start with the first plant, and try to go as far along the perimeter as possible
        let mut sides: Vec<Side> = Vec::new();

        while !plants.is_empty() {
            let plant = plants.remove(0);
            for border in plant.borders.iter() {
                let side = Side::new(border, &plant);
                sides.push(side);
            }
        }

        let mut merged: Vec<Side> = sides.clone();
        loop {
            let mut finished = true;
            let mut next_sides = merged.clone();
            merged = Vec::new();
            for side in next_sides.iter_mut() {
                let mut found = false;
                for m in merged.iter_mut() {
                    if m.try_connect_side(side) {
                        found = true;
                        finished = false;
                        break;
                    }
                }
                if !found {
                    merged.push(side.clone());
                }
            }
            merged = merged.clone();
            if finished {
                break;
            }
        }

        merged.len()
    }

    #[allow(dead_code)]
    fn print_garden_plot_perim(&mut self, width: usize, height: usize) {
        let kind = self.kind;
        let area = self.get_area();
        println!(
            "Plot of kind {} has an area of {} and perimeter of {}",
            kind,
            area,
            self.calculate_perimeter()
        );
        let mut garden = vec![vec!['.'; width]; height];
        for plant in &self.plants {
            garden[plant.y][plant.x] = plant.kind;
        }
        for row in garden {
            println!("{}", row.iter().collect::<String>());
        }
    }
    #[allow(dead_code)]
    fn print_garden_plot_sides(&mut self, width: usize, height: usize) {
        let kind = self.kind;
        let area = self.get_area();
        println!(
            "Plot of kind {} has an area of {} and {} sides",
            kind,
            area,
            self.calculate_num_sides()
        );
        let mut garden = vec![vec!['.'; width * 3]; height * 3];
        for plant in &self.plants {
            let fixed_x = plant.x * 3 + 1;
            let fixed_y = plant.y * 3 + 1;
            garden[fixed_y][fixed_x] = plant.kind;
            if plant.borders.has(Border::Top) {
                garden[fixed_y - 1][fixed_x] = '_';
            }
            if plant.borders.has(Border::Right) {
                garden[fixed_y][fixed_x + 1] = '|';
            }
            if plant.borders.has(Border::Bottom) {
                garden[fixed_y + 1][fixed_x] = '-';
            }
            if plant.borders.has(Border::Left) {
                garden[fixed_y][fixed_x - 1] = '|';
            }
        }
        for row in garden {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

struct Garden {
    width: usize,
    height: usize,
    plants: Vec<Vec<Plant>>,
}

impl Garden {
    fn from_str(input: &str) -> Self {
        let mut plants = Vec::new();
        for (y, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                row.push(Plant::from_char(x, y, c));
            }
            plants.push(row);
        }
        let width = plants[0].len();
        let height = plants.len();
        Self {
            plants,
            width,
            height,
        }
    }

    fn get_plant(&self, x: i32, y: i32) -> Option<&Plant> {
        if x < 0 || y < 0 {
            return None;
        }
        let x: usize = x.try_into().unwrap();
        let y: usize = y.try_into().unwrap();
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(&self.plants[y][x])
    }

    fn get_neighbor_plants(&self, plant: &Plant) -> Vec<Plant> {
        let mut neighbors = Vec::new();
        for dir in Border::iter() {
            let (dx, dy) = dir.direction();

            let x: i32 = plant.x as i32 + dx;
            let y: i32 = plant.y as i32 + dy;

            if let Some(neighbor) = self.get_plant(x, y) {
                neighbors.push(*neighbor);
            }
        }
        neighbors
    }

    fn get_matching_neighbors(&self, plant: &Plant) -> Vec<Plant> {
        self.get_neighbor_plants(plant)
            .into_iter()
            .filter(|p| p.is_same_kind(plant))
            .collect()
    }

    fn spread(&self, plot: &mut Plot) {
        let mut has_added_new_plants = false;
        for plant in plot.plants.clone() {
            let mut neighbors = self.get_matching_neighbors(&plant);
            for neighbor in neighbors.iter_mut() {
                if plot.contains(neighbor) {
                    continue;
                }
                if plot.try_add_plant(neighbor) {
                    has_added_new_plants = true;
                }
            }
        }
        if has_added_new_plants {
            self.spread(plot);
        }
    }

    fn create_plots(&mut self) -> Vec<Plot> {
        let mut plots: Vec<Plot> = Vec::new();
        for row in self.plants.clone() {
            for mut plant in row.clone() {
                // Check if the plant is already in a plot
                let found = plots.iter().any(|plot| plot.contains(&plant));
                if found {
                    continue;
                }
                let mut plot = Plot::new(plant.kind);
                plot.add_plant(&mut plant);
                self.spread(&mut plot);
                plots.push(plot);
            }
        }

        plots
    }

    fn get_price_one(&mut self) -> usize {
        let mut plots = self.create_plots();
        plots.iter_mut().map(|p| p.get_price_one()).sum()
    }

    fn get_price_two(&mut self) -> usize {
        let mut plots = self.create_plots();
        plots.iter_mut().map(|p| p.get_price_two()).sum()
    }

    #[allow(dead_code)]
    fn print_perim(&mut self) {
        for plot in self.create_plots().iter_mut() {
            let kind = plot.kind;
            println!("Plot {} has a price of {}", kind, plot.get_price_one());
            plot.print_garden_plot_perim(self.width, self.height);
        }
    }
    #[allow(dead_code)]
    fn print_sides(&mut self) {
        for plot in self.create_plots().iter_mut() {
            let kind = plot.kind;
            println!("Plot {} has a price of {}", kind, plot.get_price_two());
            plot.print_garden_plot_sides(self.width, self.height);
        }
    }

    #[allow(dead_code)]
    fn output_graph_with_sides(&mut self) {
        let mut plots = self.create_plots();
        for plot in plots.iter_mut() {
            plot.get_price_two();
        }
        let mut garden = vec![vec![' '; self.width * 2 + 1]; self.height * 2 + 1];
        // Start at one and go around by 2s to add the initial ! to the garden
        // First, the bottom
        for x in 1..self.width * 2 {
            if x % 2 == 0 {
                continue;
            }
            garden[0][x] = '!';
            garden[self.height * 2][x] = '!';
        }
        // Then the sides
        for y in garden.iter_mut().take(self.height * 2).skip(1).step_by(2) {
            y[0] = '!';
            y[self.width * 2] = '!';
        }

        for plot in plots.iter() {
            for plant in plot.plants.iter() {
                let fixed_x = plant.x * 2 + 1;
                let fixed_y = plant.y * 2 + 1;
                garden[fixed_y][fixed_x] = plant.kind;
                if plant.borders.has(Border::Top) {
                    let ch = garden[fixed_y - 1][fixed_x];
                    if ch != '!' {
                        garden[fixed_y - 1][fixed_x] = '!';
                    } else {
                        garden[fixed_y - 1][fixed_x] = '-';
                    }
                } else {
                    garden[fixed_y - 1][fixed_x] = ' ';
                }
                if plant.borders.has(Border::Right) {
                    let ch = garden[fixed_y][fixed_x + 1];
                    if ch != '!' {
                        garden[fixed_y][fixed_x + 1] = '!';
                    } else {
                        garden[fixed_y][fixed_x + 1] = '|';
                    }
                } else {
                    garden[fixed_y][fixed_x + 1] = ' ';
                }
                if plant.borders.has(Border::Bottom) {
                    let ch = garden[fixed_y + 1][fixed_x];
                    if ch != '!' {
                        garden[fixed_y + 1][fixed_x] = '!';
                    } else {
                        garden[fixed_y + 1][fixed_x] = '-';
                    }
                } else {
                    garden[fixed_y + 1][fixed_x] = ' ';
                }
                if plant.borders.has(Border::Left) {
                    let ch = garden[fixed_y][fixed_x - 1];
                    if ch != '!' {
                        garden[fixed_y][fixed_x - 1] = '!';
                    } else {
                        garden[fixed_y][fixed_x - 1] = '|';
                    }
                } else {
                    garden[fixed_y][fixed_x - 1] = ' ';
                }
            }
        }
        // Write this to a file
        let mut file = File::create("garden.txt").unwrap();
        let output_string = garden
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");
        file.write_all(output_string.as_bytes()).unwrap();
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut garden = Garden::from_str(input);
    // garden.print_perim();
    Some(garden.get_price_one())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut garden = Garden::from_str(input);
    // garden.print_sides();
    // garden.output_graph_with_sides();
    Some(garden.get_price_two())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two_one() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(1206));
    }
    #[test]
    fn test_part_two_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(80));
    }
    #[test]
    fn test_part_two_three() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(436));
    }
    #[test]
    fn test_part_two_four() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(236));
    }

    #[test]
    fn test_part_two_five() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(368));
    }
}
