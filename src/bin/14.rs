use gif::{Encoder, Frame, Repeat};
use std::fs::File;

advent_of_code::solution!(14);

const COLOR_MAP: &[u8; 33] = &[
    0xFF, 0xEB, 0xEB, 0xEC, 0xFF, 0xEB, 0x5C, 0x00, 0x00, 0x75, 0x17, 0x17, 0xBA, 0x0C, 0x0C, 0xFF,
    0x00, 0x00, 0x27, 0xA3, 0x00, 0x2A, 0x85, 0x0E, 0x2D, 0x66, 0x1B, 0x00, 0x5C, 0x00, 0xFF, 0xC2,
    0xC2,
];

struct Robot {
    x: i8,
    y: i8,
    dx: i8,
    dy: i8,
    color_index: u8,
}

impl Robot {
    fn parse_from_line(input: &str) -> Self {
        // p=0,4 v=3,-3
        // Regex for the position is "p=(\d+),(\d+)"
        // Regex for the velocity is "v=(-?\d+),(-?\d+)"
        let re = regex::Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
        let caps = re.captures(input).unwrap();
        Robot {
            x: caps[1].parse().unwrap(),
            y: caps[2].parse().unwrap(),
            dx: caps[3].parse().unwrap(),
            dy: caps[4].parse().unwrap(),
            color_index: caps[1].parse::<u8>().unwrap() % 8 + 2,
        }
    }

    fn simulate_seconds(&mut self, seconds: isize, wrapping_width: isize, wrapping_height: isize) {
        let next_x = (self.x as isize + (self.dx as isize * seconds)) % wrapping_width;
        let next_y = (self.y as isize + (self.dy as isize * seconds)) % wrapping_height;
        if next_x < 0 {
            self.x = (wrapping_width + next_x).try_into().unwrap();
        } else {
            self.x = next_x as i8;
        }
        if next_y < 0 {
            self.y = (wrapping_height + next_y).try_into().unwrap();
        } else {
            self.y = next_y as i8;
        }
    }

    fn is_touching(&self, other: &Robot) -> bool {
        let x_diff = (self.x - other.x).abs();
        let y_diff = (self.y - other.y).abs();
        x_diff <= 1 && y_diff <= 1
    }

    fn increment_color_index(&mut self) {
        self.color_index = (self.color_index + 1) % 8 + 2;
    }
}

struct Grid {
    width: i8,
    height: i8,
    robots: Vec<Robot>,
}

impl Grid {
    fn from_input(width: i8, height: i8, input: &str) -> Self {
        let robots = input.lines().map(Robot::parse_from_line).collect();
        Grid {
            width,
            height,
            robots,
        }
    }

    fn simulate_seconds(&mut self, seconds: i8) {
        for robot in self.robots.iter_mut() {
            robot.simulate_seconds(seconds as isize, self.width as isize, self.height as isize);
        }
    }

    fn count_robots_in_quadrant(&self, x_min: i8, x_max: i8, y_min: i8, y_max: i8) -> usize {
        self.robots
            .iter()
            .filter(|robot| {
                robot.x >= x_min && robot.x < x_max && robot.y >= y_min && robot.y < y_max
            })
            .count()
    }

    fn get_safety_factor(&self) -> usize {
        // This needs to separate the robots into four quadrants
        // count the number of robots in each quadrant
        // and then multiply these together
        let x_mid = self.width / 2;
        let y_mid = self.height / 2;
        let top_left = self.count_robots_in_quadrant(0, x_mid, 0, y_mid);
        let top_right = self.count_robots_in_quadrant(x_mid + 1, self.width, 0, y_mid);
        let bottom_left = self.count_robots_in_quadrant(0, x_mid, y_mid + 1, self.height);
        let bottom_right =
            self.count_robots_in_quadrant(x_mid + 1, self.width, y_mid + 1, self.height);

        top_left * top_right * bottom_left * bottom_right
    }

    fn print(&self) {
        let mut grid = vec![vec!['.'; self.width as usize]; self.height as usize];
        // This should print the count of robots at each position
        for robot in self.robots.iter() {
            // If we can parse the count as a digit, then we need to increment it
            // Otherwise, we need to set the count to 1
            if grid[robot.y as usize][robot.x as usize].is_ascii_digit() {
                let count = grid[robot.y as usize][robot.x as usize]
                    .to_digit(10)
                    .unwrap();
                grid[robot.y as usize][robot.x as usize] =
                    std::char::from_digit(count + 1, 10).unwrap();
            } else {
                grid[robot.y as usize][robot.x as usize] = '1';
            }
        }
        for row in grid.iter() {
            println!("{}", row.iter().collect::<String>());
        }
    }

    fn is_fully_connected(&self) -> bool {
        let mut touching_count = 0;
        let mut non_touching_count = 0;
        for i in 0..self.robots.len() {
            let mut has_match = false;
            for j in i + 1..self.robots.len() {
                if self.robots[i].is_touching(&self.robots[j]) {
                    has_match = true;
                    continue;
                }
            }
            if has_match {
                touching_count += 1;
            } else {
                non_touching_count += 1;
            }
        }

        touching_count > non_touching_count
    }

    // fn generate_frame_stamp(&self, &mut buffer: Vec<u8>, frame_index: usize, width: usize, height: usize) {
    //     // This should generate a frame stamp for the current grid
    //     // A frame stamp is outputting the current frame_index into the bottom left corner of the buffer
    //     let frame_index_str = frame_index.to_string();
    //     let frame_index_len = frame_index_str.len();
    //     let frame_index_offset = width * (height - 1) + 1;
    //     for (i, c) in frame_index_str.chars().enumerate() {
    //         buffer[frame_index_offset + i] = c as u8;
    //     }

    // }

    fn generate_frame(&self, width: usize, height: usize) -> Frame {
        let frame_width = width * 4;
        let frame_height = height * 4;
        let mut buffer = vec![0; frame_width * frame_height];
        for robot in self.robots.iter() {
            // This should create a 3x3 pixel square for each robot
            // if there is nothing to the right and down, it should write a shadow
            let tl = robot.y as usize * 4 * frame_width + robot.x as usize * 4;
            let ml = tl + frame_width;
            let bl = tl + 2 * frame_width;
            let sl = tl + 3 * frame_width;
            buffer[tl] = robot.color_index;
            buffer[tl + 1] = robot.color_index;
            buffer[tl + 2] = robot.color_index;
            if buffer[tl + 3] == 0 {
                buffer[tl + 3] = 11;
            }
            // Look Right
            if buffer[tl + 4] != 0 {
                buffer[tl + 3] = robot.color_index;
            }
            // Look Left
            if tl != 0 && buffer[tl - 1] != 0 {
                buffer[tl - 1] = buffer[tl - 2];
            }

            buffer[ml] = robot.color_index;
            buffer[ml + 1] = robot.color_index;
            buffer[ml + 2] = robot.color_index;
            if buffer[ml + 3] == 0 {
                buffer[ml + 3] = 11;
            }
            // Look Right
            if buffer[ml + 4] != 0 {
                buffer[ml + 3] = robot.color_index;
            }
            // Look Left
            if ml != 0 && buffer[ml - 1] != 0 {
                buffer[ml - 1] = buffer[ml - 2];
            }

            buffer[bl] = robot.color_index;
            buffer[bl + 1] = robot.color_index;
            buffer[bl + 2] = robot.color_index;
            if buffer[bl + 3] == 0 {
                buffer[bl + 3] = 11;
            }
            // Look Right
            if buffer[bl + 4] != 0 {
                buffer[bl + 3] = robot.color_index;
            }
            // Look Left
            if bl != 0 && buffer[bl - 1] != 0 {
                buffer[bl - 1] = buffer[bl - 2];
            }

            // Look down
            if sl + frame_width < buffer.len() && buffer[sl + frame_width] != 0 {
                buffer[sl] = robot.color_index;
                buffer[sl + 1] = robot.color_index;
                buffer[sl + 2] = robot.color_index;
                buffer[sl + 3] = robot.color_index;
            } else {
                buffer[sl] = 11;
                buffer[sl + 1] = 11;
                buffer[sl + 2] = 11;
                buffer[sl + 3] = 11;
            }
        }
        let mut frame = Frame::default();
        frame.width = frame_width.try_into().unwrap();
        frame.height = frame_height.try_into().unwrap();
        frame.buffer = buffer.into();
        frame
    }
}

fn part_one_internal(input: &str, width: i8, height: i8, seconds: i8) -> Option<usize> {
    let mut grid = Grid::from_input(width, height, input);
    grid.simulate_seconds(seconds);
    grid.print();
    Some(grid.get_safety_factor())
}

fn part_two_internal_gif(input: &str, width: i8, height: i8, seconds: usize) -> Option<usize> {
    let mut grid = Grid::from_input(width, height, input);
    let mut image = File::create("target/bots.gif").unwrap();
    let mut encoder = Encoder::new(
        &mut image,
        width.try_into().unwrap(),
        height.try_into().unwrap(),
        COLOR_MAP,
    )
    .unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();
    let mut i = 0;
    while !grid.is_fully_connected() {
        grid.simulate_seconds(1);
        i += 1;
        if i % 20 == 0 {
            let frame = grid.generate_frame(width as usize, height as usize);
            encoder.write_frame(&frame).unwrap();
        }
    }
    for _ in 0..seconds {
        for robot in grid.robots.iter_mut() {
            robot.increment_color_index();
        }
        let frame = grid.generate_frame(width as usize, height as usize);
        encoder.write_frame(&frame).unwrap();
        encoder.write_frame(&frame).unwrap();
    }
    Some(i)
}

fn part_two_internal(input: &str, width: i8, height: i8, seconds: usize) -> Option<usize> {
    let mut grid = Grid::from_input(width, height, input);
    let mut i = 0;
    while !grid.is_fully_connected() {
        grid.simulate_seconds(1);
        i += 1;
        // println!("Seconds: {}", i);
    }
    // grid.print();
    Some(i)
}

pub fn part_one(input: &str) -> Option<usize> {
    part_one_internal(input, 101, 103, 100)
}

pub fn part_two(input: &str) -> Option<usize> {
    part_two_internal(input, 101, 103, 60)
}

#[cfg(test)]
mod tests_14 {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_internal(
            &advent_of_code::template::read_file("examples", DAY),
            11,
            7,
            100,
        );
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        // Skipping
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }
}
