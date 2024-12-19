use std::collections::HashMap;

advent_of_code::solution!(13);

#[derive(Debug, Clone)]
struct Button {
    dx: usize,
    dy: usize,
    cost: usize,
}

#[derive(Debug)]
struct Prize {
    x: usize,
    y: usize,
}

impl Prize {
    fn get_y_for_slope(&self, x: usize) -> usize {
        let slope = self.y as f64 / self.x as f64;
        (slope * x as f64).round() as usize
    }

    fn get_distance(&self, x: usize, y: usize) -> usize {
        if x > self.x || y > self.y {
            return usize::MAX;
        }
        // Manhattan distance
        // But we should also weight it towards the centerline between the prize and 0,0
        let x_remaining = self.x - x;
        let y_remaining = self.y - y;
        // let distance_from_slope = if self.get_y_for_slope(x) > y {
        //     self.get_y_for_slope(x) - y
        // } else {
        //     y - self.get_y_for_slope(x)
        // };

        // x_remaining + y_remaining + distance_from_slope
        x_remaining + y_remaining
    }

    fn is_runnable(&self, visit: &Visited, button: &Button) -> Option<usize> {
        if visit.x > self.x || visit.y > self.y {
            return None;
        }
        // This should determine if it is possible to just keep pushing the button until you reach the prize
        let x_remaining = self.x - visit.x;
        let y_remaining = self.y - visit.y;
        // Check to see if they are divisible by the button dx and dy with no remainder
        let x_divisible = x_remaining % button.dx == 0;
        let y_divisible = y_remaining % button.dy == 0;
        if x_divisible && y_divisible {
            // Then they have to be divisible by the same number
            let x_presses = x_remaining / button.dx;
            let y_presses = y_remaining / button.dy;

            if x_presses == y_presses {
                return Some(x_presses);
            } else {
                return None;
            }
        };
        None
    }

    fn overshoot(&self, visit: &Visited) -> bool {
        visit.x > self.x || visit.y > self.y
    }

    fn caught(&self, visit: &Visited) -> bool {
        visit.x == self.x && visit.y == self.y
    }
}

#[derive(Debug, Clone)]
struct Visited {
    x: usize,
    y: usize,
    cost: usize,
    distance: usize,
}

impl Visited {
    fn new(x: usize, y: usize, cost: usize, prize: &Prize) -> Self {
        let distance = prize.get_distance(x, y);
        Self {
            x,
            y,
            cost,
            distance,
        }
    }

    fn is_coincident(&self, other: &Visited) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn get_hash(&self) -> u64 {
        (self.x as u64) << 32 | self.y as u64
    }

    fn get_next_visited(&self, button: &Button, prize: &Prize) -> Visited {
        let x = self.x + button.dx;
        let y = self.y + button.dy;
        let cost = self.cost + button.cost;
        let distance = prize.get_distance(x, y);
        Visited {
            x,
            y,
            cost,
            distance,
        }
    }
}

impl PartialEq for Visited {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}
impl Eq for Visited {}
impl PartialOrd for Visited {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cost.cmp(&other.cost))
    }
}
impl Ord for Visited {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

struct MinHeap<'a> {
    target: &'a Prize,
    size: usize,
    // distances: Vec<usize>,
    min_distance: Option<usize>,
    heap: HashMap<usize, Vec<Visited>>,
}

impl<'a> MinHeap<'a> {
    fn new(target: &'a Prize) -> Self {
        Self {
            target,
            // distances: Vec::new(),
            min_distance: None,
            heap: HashMap::new(),
            size: 0,
        }
    }

    fn insert(&mut self, visited: Visited) {
        let distance = visited.distance;
        if distance == usize::MAX {
            return;
        }
        if let Some(visited_list) = self.heap.get_mut(&distance) {
            // See if there is a visited at the same x and y
            if let Some(visited_index) = visited_list.iter().position(|v| v.is_coincident(&visited))
            {
                // let previous = visited_list.get_mut(visited_index).unwrap();
                // if previous.cost > visited.cost {
                //     println!("Found previous: {:?}, current: {:?}", previous, visited);
                //     previous.cost = visited.cost;
                // } else {
                return;
                // }
            } else {
                visited_list.push(visited);
                self.size += 1;
            }
            // visited_list.sort();

            // println!(
            //     "Found {} item list at distance: {}: {:?}",
            //     visited_list.len(),
            //     distance,
            //     visited_list
            // );
            // visited_list.sort_by(|a, b| b.cost.cmp(&a.cost));
        } else {
            self.heap.insert(distance, vec![visited]);
            // self.distances.push(distance);
            // self.distances.sort_by(|a, b| b.cmp(a));
            // self.distances.sort();
            self.size += 1;
        }
    }

    fn get_min_distance(&self) -> Option<usize> {
        if self.min_distance.is_some() {
            return self.min_distance;
        };
        self.heap.keys().min().copied()
    }

    fn get_next(&mut self) -> Option<Visited> {
        // let distance = self.distances.last()?;
        // let distance = self.get_min_distance()?;
        let distance = self.get_min_distance()?;
        let visited_list = self.heap.get_mut(&distance)?;
        let visited = visited_list.pop()?;

        if visited_list.is_empty() {
            self.heap.remove(&distance);
            // self.distances.pop();
            self.min_distance = None;
            self.min_distance = self.get_min_distance();
        }
        // println!("Visited: {:?}, Target: {:?}", visited, self.target);
        self.size -= 1;
        Some(visited)
    }
}

struct ClawMachine {
    button_a: Button,
    button_b: Button,
    seen: HashMap<u64, Visited>,
    min_cost: Option<usize>,
}

impl ClawMachine {
    fn new(button_a: Button, button_b: Button) -> Self {
        Self {
            button_a,
            button_b,
            seen: HashMap::new(),
            min_cost: None,
        }
    }

    fn has_seen(&self, visit: &Visited) -> bool {
        self.seen.contains_key(&visit.get_hash())
    }

    fn get_min_cost(&self) -> Option<usize> {
        self.min_cost
    }

    fn should_visit(&self, visit: &Visited) -> bool {
        if visit.distance == usize::MAX {
            return false;
        }
        if let Some(visited) = self.seen.get(&visit.get_hash()) {
            visited.cost > visit.cost
        } else {
            true
        }
    }

    fn set_min_cost(&mut self, visit: &Visited) -> usize {
        let hash = visit.get_hash();
        if let Some(visited) = self.seen.get_mut(&hash) {
            if visited.cost > visit.cost {
                visited.cost = visit.cost;
            }
            visited.cost
        } else {
            self.seen.insert(hash, visit.clone());
            visit.cost
        }
    }

    fn try_runnable(&mut self, visit: &Visited, prize: &Prize) -> Option<usize> {
        if let Some(presses) = prize.is_runnable(visit, &self.button_a) {
            let button_a = self.button_a.clone();
            self.run_button(prize, visit.clone(), &button_a);
            return Some(presses * self.button_a.cost);
        }
        if let Some(presses) = prize.is_runnable(visit, &self.button_b) {
            let button_b = self.button_b.clone();
            self.run_button(prize, visit.clone(), &button_b);
            return Some(presses * self.button_b.cost);
        }
        None
    }

    fn run_button(&mut self, prize: &Prize, visit: Visited, button: &Button) {
        let mut visit = visit;
        while !prize.overshoot(&visit) {
            visit = visit.get_next_visited(button, prize);
            self.set_min_cost(&visit);
        }
    }

    fn find_min_cost_to_prize(&mut self, prize: &Prize) -> Option<usize> {
        self.seen.clear();
        let mut min_heap = MinHeap::new(prize);
        let start = Visited::new(0, 0, 0, prize);
        min_heap.insert(start);
        while let Some(visited) = min_heap.get_next() {
            // println!(
            //     "({},{}): {} - {} Left in heap. Min Cost: {}",
            //     visited.x,
            //     visited.y,
            //     visited.cost,
            //     min_heap.size(),
            //     self.min_cost.unwrap_or(usize::MAX)
            // );

            if self.has_seen(&visited) {
                continue;
            }

            // If we are already over the min cost, then skip
            if self.min_cost.is_some() && visited.cost >= self.min_cost.unwrap() {
                continue;
            }

            // If we have overshot the prize, then skip
            if prize.overshoot(&visited) {
                continue;
            }

            self.set_min_cost(&visited);
            // Check if we have reached the prize
            if prize.caught(&visited) {
                self.min_cost = Some(self.set_min_cost(&visited));
                // println!("Found prize: {}", self.min_cost.unwrap());
                continue;
            }

            // Try to see if we can just run the button until we reach the prize
            if let Some(runnable_cost) = self.try_runnable(&visited, prize) {
                let next_cost = runnable_cost + visited.cost;
                if self.min_cost.is_none() || next_cost < self.min_cost.unwrap() {
                    self.min_cost = Some(next_cost);
                    // println!("Found runnable: {}", self.min_cost.unwrap());
                    continue;
                }
            }

            // Then enqueue the next possible moves
            let next_visited_a = visited.get_next_visited(&self.button_a, prize);
            let next_visited_b = visited.get_next_visited(&self.button_b, prize);

            if self.should_visit(&next_visited_a) {
                min_heap.insert(next_visited_a);
            }
            if self.should_visit(&next_visited_b) {
                min_heap.insert(next_visited_b);
            }
        }

        // println!("Cached {} positions", self.seen.keys().len());

        self.min_cost
    }
}

fn parse_button_line(input: &str) -> Button {
    // Button A: X+94, Y+34
    // Regex for this is: Button (A|B): X+<number>, Y+<number>
    let nums_re = regex::Regex::new(r"Button (A|B): X\+(\d+), Y\+(\d+)").unwrap();
    let nums = nums_re.captures(input).unwrap();
    let button_name: String = nums[1].parse().unwrap();
    let cost = if button_name == "A" { 3 } else { 1 };
    Button {
        dx: nums[2].parse().unwrap(),
        dy: nums[3].parse().unwrap(),
        cost,
    }
}

fn parse_prize_line(input: &str) -> Prize {
    // Prize: X=8400, Y=5400
    // Regex for this is: Prize: X=<number>, Y=<number>
    let nums_re = regex::Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    let nums = nums_re.captures(input).unwrap();
    Prize {
        x: nums[1].parse().unwrap(),
        y: nums[2].parse().unwrap(),
    }
}

fn parse_input(input: &str, offset: usize) -> Vec<ClawMachine> {
    /*
    Button A: X+94, Y+34
    Button B: X+22, Y+67
    Prize: X=8400, Y=5400
     */

    let mut claw_machines = Vec::new();
    let mut lines = input.lines();
    let mut iterations = 0;
    println!("Parsing input");
    while let Some(button_line) = lines.next() {
        let start_time = std::time::Instant::now();
        let button_a = parse_button_line(button_line);
        let button_b = parse_button_line(lines.next().unwrap());
        let mut prize = parse_prize_line(lines.next().unwrap());
        prize.x += offset;
        prize.y += offset;
        let mut claw_machine = ClawMachine::new(button_a, button_b);
        iterations += 1;
        // println!("Starting iteration: {}", iterations);
        claw_machine.find_min_cost_to_prize(&prize);
        // if let Some(min_cost) = claw_machine.get_min_cost() {
        //     println!("[{}] Min cost: {}", iterations, min_cost);
        // } else {
        //     println!("[{}] No min cost found", iterations);
        // }
        println!("[{}] Time: {:?}", iterations, start_time.elapsed());
        claw_machines.push(claw_machine);
        lines.next();
    }
    claw_machines
}

fn parse_input_math(input: &str, offset: usize) -> usize {
    /*
    Button A: X+94, Y+34
    Button B: X+22, Y+67
    Prize: X=8400, Y=5400
     */

    let mut lines = input.lines();
    let mut cost = 0;
    let mut iterations = 0;
    while let Some(button_line) = lines.next() {
        let start_time = std::time::Instant::now();
        let button_a = parse_button_line(button_line);
        let button_b = parse_button_line(lines.next().unwrap());
        let mut prize = parse_prize_line(lines.next().unwrap());
        prize.x += offset;
        prize.y += offset;
        iterations += 1;
        // Instead of that, we're going to do math
        // There should be some amount of button pushes where the prize is reachable
        // A_Pushes * A_DX + B_Pushes * B_DX = Prize_X
        // A_Pushes * A_DY + B_Pushes * B_DY = Prize_Y
        // A_Pushes * A_Cost + B_Pushes * B_Cost = Min_Cost
        // A_Pushes = (Prize_X - B_Pushes * B_DX) / A_DX
        // B_Pushes = (Prize_Y - A_Pushes * A_DY) / B_DY
        let x1 = button_a.dx;
        let y1 = button_a.dy;
        let x2 = button_b.dx;
        let y2 = button_b.dy;
        let z1 = prize.x;
        let z2 = prize.y;
        let b = (z2 * x1 - z1 * x2) / (y2 * x1 - y1 * x2);
        let a = (z1 - b * y1) / x1;
        // let check_x = (a * x1) + (b * y1);
        // let check_y = (a * x2) + (b * y2);
        if (x1 * a + y1 * b, x2 * a + y2 * b) == (z1, z2) {
            let next_cost = (a * button_a.cost) + (b * button_b.cost);

            cost += next_cost;
        }
        // if check_x == z1 && check_y == z2 {
        // println!("Found a: {}, b: {}", a, b);
        // println!("Cost: {}", cost);
        // } else {
        // println!("Failed to find a: {}, b: {}", a, b);
        // }
        // println!("[{}] Time: {:?}", iterations, start_time.elapsed());
        lines.next();
    }
    cost as usize
}

// pub fn part_one(input: &str) -> Option<usize> {
//     let claw_machines = parse_input(input, 0);
//     let total_cost = claw_machines
//         .iter()
//         .map(|claw_machine| claw_machine.get_min_cost().unwrap_or(0))
//         .sum();
//     Some(total_cost)
// }

// pub fn part_two(input: &str) -> Option<usize> {
//     let claw_machines = parse_input(input, 10000000000000);
//     let total_cost = claw_machines
//         .iter()
//         .map(|claw_machine| claw_machine.get_min_cost().unwrap_or(0))
//         .sum();
//     Some(total_cost)
//     // None
// }

use itertools::Itertools;

fn solve(x1: i64, x2: i64, y1: i64, y2: i64, z1: i64, z2: i64) -> i64 {
    let b = (z2 * x1 - z1 * x2) / (y2 * x1 - y1 * x2);
    let a = (z1 - b * y1) / x1;
    if (x1 * a + y1 * b, x2 * a + y2 * b) != (z1, z2) {
        return 0;
    }
    a * 3 + b
}

fn stolen(input: &str, offset: i64) -> i64 {
    let xs = input
        .split(|c: char| !c.is_ascii_digit())
        .filter(|w| !w.is_empty())
        .map(|w| w.parse().unwrap())
        .tuples();
    let mut p = 0;
    for (x1, x2, y1, y2, z1, z2) in xs {
        p += solve(x1, x2, y1, y2, z1 + offset, z2 + offset);
    }
    p
}

// This is the math version
pub fn part_one(input: &str) -> Option<usize> {
    let cost = stolen(input, 0);
    Some(cost as usize)
}
pub fn part_two(input: &str) -> Option<usize> {
    let cost = stolen(input, 10000000000000);
    Some(cost as usize)
}

#[cfg(test)]
mod tests_13 {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
