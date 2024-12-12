use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(5);

struct SafetyManual {
    updates: Vec<Vec<usize>>,
    graph: HashMap<usize, HashSet<usize>>,
}

impl SafetyManual {
    fn parse_input(input: &str) -> Self {
        // The two sections are separated with a blank line
        let mut sections = input.split("\n\n");
        let adj_list: Vec<(usize, usize)> = sections
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let mut parts = line.split("|");
                let from = parts.next().unwrap().parse().unwrap();
                let to = parts.next().unwrap().parse().unwrap();
                (from, to)
            })
            .collect();
        let updates = sections
            .next()
            .unwrap()
            .lines()
            .map(|line| line.split(",").map(|part| part.parse().unwrap()).collect())
            .collect();
        let graph = SafetyManual::create_graph(adj_list.clone());
        SafetyManual { updates, graph }
    }

    fn is_sorted(&self, update: &[usize]) -> bool {
        // We need to loop through every item in the update list
        let mut queue: VecDeque<usize> = update.iter().cloned().collect();
        while let Some(item) = queue.pop_front() {
            // If the item isn't in the graph, it doesn't depend on any other item
            if !self.graph.contains_key(&item) {
                continue;
            }
            // If it does not depend on any other item in the list, add it to the sorted list
            for remaining_item in &queue.clone() {
                if !self.graph.contains_key(remaining_item) {
                    println!("Remaining item missing in graph: {:?}", remaining_item);
                    continue;
                }
                if self.graph[remaining_item].contains(&item) {
                    return false;
                }
            }
        }
        true
    }

    fn create_graph(adj_list: Vec<(usize, usize)>) -> HashMap<usize, HashSet<usize>> {
        let mut graph = HashMap::new();
        for &(from, to) in &adj_list {
            graph.entry(from).or_insert_with(HashSet::new).insert(to);
            // If the to node is not in the graph, add it
            graph.entry(to).or_insert_with(HashSet::new);
        }
        graph
    }

    fn sort(&mut self, update: &[usize]) -> Vec<usize> {
        let mut sorted = Vec::new();
        // For each item, if it does not depend on any other item in the list, add it to the queue
        let mut queue: VecDeque<usize> = update.iter().cloned().collect();
        while let Some(item) = queue.pop_front() {
            // If it does not depend on any other item in the list, add it to the sorted list
            let mut is_in_order = true;
            for remaining_item in &queue.clone() {
                if !self.graph.contains_key(remaining_item) {
                    self.graph.insert(*remaining_item, HashSet::new());
                }
                if self.graph[remaining_item].contains(&item) {
                    queue.push_back(item);
                    queue.retain(|&x| x != *remaining_item);
                    queue.push_front(*remaining_item);
                    is_in_order = false;
                    break;
                }
            }
            if is_in_order {
                sorted.push(item);
            }
        }
        // Sorted should be the same length as the update list
        if sorted.len() != update.len() {
            panic!("Could not sort the list");
        }
        sorted
    }

    fn get_unordered_updates(&self) -> Vec<Vec<usize>> {
        let mut unordered_updates = Vec::new();
        for update in &self.updates {
            if !self.is_sorted(update) {
                unordered_updates.push(update.clone());
            }
        }
        unordered_updates
    }

    fn get_ordered_updates(&self) -> Vec<Vec<usize>> {
        let mut ordered_updates = Vec::new();
        for update in &self.updates {
            if self.is_sorted(update) {
                ordered_updates.push(update.clone());
            }
        }
        ordered_updates
    }
}

pub fn get_middle_item<T>(list: &[T]) -> &T {
    let middle = list.len() / 2;
    &list[middle]
}

pub fn part_one(input: &str) -> Option<usize> {
    // Get all the sorted updates
    // Find the middle item
    // Return the sum of the middle item
    let manual = SafetyManual::parse_input(input);
    let ordered_updates = manual.get_ordered_updates();
    let middle_item = ordered_updates
        .iter()
        .map(|update| *get_middle_item(update))
        .collect::<Vec<usize>>();
    let sum: usize = middle_item.iter().sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    // Get all the sorted updates
    // Find the middle item
    // Return the sum of the middle item
    let mut manual = SafetyManual::parse_input(input);
    let ordered_updates = manual.get_unordered_updates();
    let middle_item = ordered_updates
        .iter()
        .map(|update| manual.sort(update))
        .map(|update| *get_middle_item(&update))
        .collect::<Vec<usize>>();
    let sum: usize = middle_item.iter().sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
