advent_of_code::solution!(1);

pub fn parse_list(input: &str) -> (Vec<usize>, Vec<usize>) {
    // First, we split the input into lines
    let lines = input.lines();
    // For each line, we have a left and a right, separated by some whitespace
    let split = lines.map(|line| {
        let mut split = line.split_whitespace();
        let left: usize = split.next().unwrap().parse().unwrap();
        let right: usize = split.next().unwrap().parse().unwrap();
        // These should both be numbers, so we can parse them
        (left, right)
    });
    // We then need the left and right values as separate lists
    split.clone().unzip()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (left, right): (Vec<usize>, Vec<usize>) = parse_list(input);
    // Then we sort each list
    let mut left = left;
    left.sort();
    let mut right = right;
    right.sort();
    // Then we zip them back up
    let zipped = left.iter().zip(right.iter());
    // And find the absolute distance between the two values
    let distances = zipped.map(|(left, right)| {
        // Since we are dealing with unsigned integers, we find which one is largest and subtract the other
        if left > right {
            left - right
        } else {
            right - left
        }
    });
    // Finally, we sum all the distances
    let sum: usize = distances.sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right): (Vec<usize>, Vec<usize>) = parse_list(input);
    // We need to turn the right side into a map of the values with the count of times that they appear
    let right_map: std::collections::HashMap<usize, usize> =
        right
            .iter()
            .fold(std::collections::HashMap::new(), |mut map, &value| {
                *map.entry(value).or_insert(0) += 1;
                map
            });
    // Then we need to run through the left list and multiply the value by the count of times it appears in the right list
    let sum: u32 = left
        .iter()
        .map(|value| {
            let count = right_map.get(value).unwrap_or(&0);
            (*value as u32) * (*count as u32)
        })
        .sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
