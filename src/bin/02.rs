advent_of_code::solution!(2);

pub fn parse_input(input: &str) -> Vec<Vec<usize>> {
    // First, we split the input into lines
    let lines = input.lines();
    // Then we split each line into a list of usize
    let split = lines.map(|line| {
        line.split_whitespace()
            .map(|value| value.parse().unwrap())
            .collect()
    });
    split.collect()
}

pub fn all_increasing(report: &Vec<usize>) -> bool {
    // We need to iterate through the report and make sure that
    // They are all increasing
    let mut iter = report.iter();
    let mut last = iter.next().unwrap();

    for value in iter {
        if value <= last {
            return false;
        }
        last = value;
    }
    true
}

pub fn all_decreasing(report: &Vec<usize>) -> bool {
    // We need to iterate through the report and make sure that
    // They are all decreasing
    let mut iter = report.iter();
    let mut last = iter.next().unwrap();

    for value in iter {
        if value >= last {
            return false;
        }
        last = value;
    }
    true
}

pub fn all_diff_less_than_three(report: &Vec<usize>) -> bool {
    // We need to iterate through the report and make sure that
    // The difference between each value is less than or equal to 3
    let mut iter = report.iter();
    let mut last = iter.next().unwrap();

    for value in iter {
        if value > last && value - last > 3 {
            return false;
        }
        if value < last && last - value > 3 {
            return false;
        }
        last = value;
    }
    true
}

pub fn report_is_safe(report: &Vec<usize>) -> bool {
    // We need to iterate through the report and make sure that
    // They are either all increasing or all decreasing
    // And that the difference between each value is less than or equal to 3
    all_diff_less_than_three(report) && (all_increasing(report) || all_decreasing(report))
}

pub fn report_is_safe_with_removal(report: &Vec<usize>) -> bool {
    // We need to iterate through the report and make sure that
    // They are either all increasing or all decreasing
    // And that the difference between each value is less than or equal to 3
    // And that removing any value will still result in a safe report
    if report_is_safe(report) {
        return true;
    }
    // Else we need to make new reports with each value removed
    // So we can loop through the indexes and remove the value at that index
    for (index, _) in report.iter().enumerate() {
        let mut new_report = report.clone();
        new_report.remove(index);
        if report_is_safe(&new_report) {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = parse_input(input);
    let safe_reports = reports.iter().filter(|report| report_is_safe(report));
    let count = safe_reports.count();
    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse_input(input);
    let safe_reports = reports
        .iter()
        .filter(|report| report_is_safe_with_removal(report));
    let count = safe_reports.count();
    Some(count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
