use std::collections::HashSet;

advent_of_code::solution!(8);

type Antenna = char;

struct Location {
    x: usize,
    y: usize,
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
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let antenna = if c != '.' {
                            antennas.insert(c);
                            Some(c)
                        } else {
                            None
                        };
                        Location {
                            x,
                            y,
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
            None => return,
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

    fn set_antinodes_undistanced(&mut self) {
        // This one should set antinodes wherever the line through two antennas passes through a location
        // with or without an antenna.
        let mut passed_through_both = 0;
        let mut total = 0;

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
                    // Essentially we are drawing a line through here and looking for where it passes through whole x and y values
                    // and setting antinodes there
                    let slope = (*y2 as f64 - *y1 as f64) / (*x2 as f64 - *x1 as f64);
                    let y_intercept = *y1 as f64 - slope * *x1 as f64;
                    let mut times_crossed = 0;
                    for x in 0..self.width {
                        let y = slope * x as f64 + y_intercept;
                        // If this is a whole number
                        let fudge_factor = 0.0000003;
                        let close_enough =
                            y.fract() < fudge_factor || y.fract() > (1.0 - fudge_factor);
                        if close_enough && y >= 0.0 {
                            let y = if y.fract() < fudge_factor {
                                y.floor() as usize
                            } else {
                                y.ceil() as usize
                            };
                            self.set_antinode(x, y, *antenna);
                            if x == *x1 && y == *y1 {
                                times_crossed += 1;
                            }
                            if x == *x2 && y == *y2 {
                                times_crossed += 1;
                            }
                        }
                    }
                    if times_crossed != 2 {
                        println!(
                            "Times crossed: {}, x1: {}, y1: {}, x2: {}, y2: {}",
                            times_crossed, *x1, *y1, *x2, *y2
                        );
                        println!("y = {}x + {}", slope, y_intercept);
                        println!(
                            "Y: {}, should be: {}",
                            slope * *x1 as f64 + y_intercept,
                            *y1
                        );
                        println!(
                            "Y: {}, should be: {}",
                            slope * *x2 as f64 + y_intercept,
                            *y2
                        );
                    } else {
                        passed_through_both += 1;
                    }
                    total += 1;
                }
            }
        }
        println!("Passed through both: {} / {}", passed_through_both, total);
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

    fn to_string(&self) -> String {
        let mut result = String::new();
        for row in &self.locations {
            for location in row {
                match location.antenna {
                    Some(antenna) => result.push(antenna),
                    None => {
                        if location.antinodes.is_empty() {
                            result.push('.')
                        } else {
                            result.push('#')
                        }
                    }
                }
            }
            result.push('\n');
        }
        result
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
        assert_eq!(result, Some(240));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(955));
    }
}
