use std::collections::HashSet;

advent_of_code::solution!(8);

type Antenna = char;

struct Location {
    antenna: Option<Antenna>,
    antinodes: Vec<Antenna>,
}

struct CityMap {
    antennas: HashSet<char>,
    locations: Vec<Vec<Location>>,
    width: usize,
    height: usize,
}

impl CityMap {
    fn from_input(input: &str) -> Self {
        let mut antennas = HashSet::new();
        let locations: Vec<Vec<Location>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        let antenna = if c != '.' {
                            antennas.insert(c);
                            Some(c)
                        } else {
                            None
                        };
                        Location {
                            antenna,
                            antinodes: vec![],
                        }
                    })
                    .collect()
            })
            .collect();
        let width = locations[0].len();
        let height = locations.len();

        println!("CityMap: {}x{}", width, height);
        CityMap {
            locations,
            antennas,
            width,
            height,
        }
    }

    fn get_location_mut(&mut self, x: usize, y: usize) -> Option<&mut Location> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(&mut self.locations[y][x])
    }

    fn set_antinode(&mut self, x: usize, y: usize, antenna: Antenna) {
        let location = self.get_location_mut(x, y);
        match location {
            None => (),
            Some(location) => {
                if !location.antinodes.contains(&antenna) {
                    location.antinodes.push(antenna);
                }
            }
        }
    }

    fn find_distance(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> (usize, usize) {
        let x = if x1 > x2 { x1 - x2 } else { x2 - x1 };
        let y = if y1 > y2 { y1 - y2 } else { y2 - y1 };
        (x, y)
    }

    fn set_antinodes_distanced(&mut self) {
        // To find where an antinode is, we have to find two antenna of the same type
        // and then an antinode will be created at the point where one antenna is twice as far as the other
        // from the antinode
        for antenna in &self.antennas.clone() {
            let mut locations: Vec<(usize, usize)> = vec![];
            for (y, row) in self.locations.iter().enumerate() {
                for (x, location) in row.iter().enumerate() {
                    if location.antenna == Some(*antenna) {
                        locations.push((x, y));
                    }
                }
            }
            for (x1, y1) in &locations {
                for (x2, y2) in &locations {
                    if x1 == x2 && y1 == y2 {
                        continue;
                    }
                    let (dx, dy) = self.find_distance(*x1, *y1, *x2, *y2);

                    let x1_node = if x1 > x2 {
                        x1.checked_add(dx)
                    } else {
                        x1.checked_sub(dx)
                    };
                    let y1_node = if y1 > y2 {
                        y1.checked_add(dy)
                    } else {
                        y1.checked_sub(dy)
                    };
                    let x2_node = if x1 > x2 {
                        x2.checked_sub(dx)
                    } else {
                        x2.checked_add(dx)
                    };
                    let y2_node = if y1 > y2 {
                        y2.checked_sub(dy)
                    } else {
                        y2.checked_add(dy)
                    };

                    if let (Some(x1_node), Some(y1_node)) = (x1_node, y1_node) {
                        self.set_antinode(x1_node, y1_node, *antenna);
                    }
                    if let (Some(x2_node), Some(y2_node)) = (x2_node, y2_node) {
                        self.set_antinode(x2_node, y2_node, *antenna);
                    }
                }
            }
        }
    }
    fn is_in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0
            && x < self.width.try_into().unwrap()
            && y >= 0
            && y < self.height.try_into().unwrap()
    }

    fn set_antinodes_stepwise(&mut self) {
        // This one should set antinodes wherever the line through two antennas passes through a location
        // with or without an antenna.

        for antenna in &self.antennas.clone() {
            let mut locations: Vec<(isize, isize)> = vec![];
            for (y, row) in self.locations.iter().enumerate() {
                for (x, location) in row.iter().enumerate() {
                    if location.antenna == Some(*antenna) {
                        locations.push((x as isize, y as isize));
                    }
                }
            }
            for (x1, y1) in locations.clone() {
                for (x2, y2) in locations.clone() {
                    // If they are the same node, skip it
                    if x1 == x2 && y1 == y2 {
                        continue;
                    }
                    // Otherwise we set the x and y step size to be the distance between the two points
                    let x_step = x2.checked_sub(x1);
                    let y_step = y2.checked_sub(y1);
                    if x_step.is_none() || y_step.is_none() {
                        continue;
                    }
                    let x_step = x_step.unwrap();
                    let y_step = y_step.unwrap();

                    let mut next_step = (x1, y1);
                    while self.is_in_bounds(next_step.0, next_step.1) {
                        let x = next_step.0;
                        let y = next_step.1;
                        self.set_antinode(x.try_into().unwrap(), y.try_into().unwrap(), *antenna);
                        if x < x_step || y < y_step {
                            break;
                        }
                        next_step = (next_step.0 - x_step, next_step.1 - y_step);
                    }
                    next_step = (x2, y2);
                    while self.is_in_bounds(next_step.0, next_step.1) {
                        let x = next_step.0;
                        let y = next_step.1;
                        self.set_antinode(x.try_into().unwrap(), y.try_into().unwrap(), *antenna);
                        next_step = (next_step.0 + x_step, next_step.1 + y_step);
                    }
                }
            }
        }
    }

    fn count_antinodes(&self) -> usize {
        self.locations
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|location| !location.antinodes.is_empty())
                    .count()
            })
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut city_map = CityMap::from_input(input);
    city_map.set_antinodes_distanced();
    Some(city_map.count_antinodes())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut city_map = CityMap::from_input(input);
    city_map.set_antinodes_stepwise();
    Some(city_map.count_antinodes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
