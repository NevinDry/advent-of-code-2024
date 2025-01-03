use std::fs::File;
use std::io::{self, BufRead};

// Puzzle at : https://adventofcode.com/2024/day/2

fn main() {
    // Setup
    let path = "./src/data.txt";
    let file = File::open(path).expect("Error opening file");

    let reports: Vec<Vec<i32>> = get_reports_from_file(&file);

    // first star
    let safe_reports = get_safe_reports(&reports);

    println!("First Star Answer: {:?}", safe_reports.len());

    // second star
    let safe_reports_with_dampener = get_safe_reports_with_dampener(&reports);

    println!("Second Star Answer: {:?}", safe_reports_with_dampener.len());
}

fn get_reports_from_file(file: &File) -> Vec<Vec<i32>> {
    let reader = io::BufReader::new(file);
    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Cannot get line");
        let values: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        reports.push(values);
    }
    reports
}

fn get_safe_reports(reports: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut safe_reports: Vec<Vec<i32>> = Vec::new();
    for report in reports {
        if report_is_safe(report) {
            safe_reports.push(report.to_owned())
        }
    }
    safe_reports
}

fn get_safe_reports_with_dampener(reports: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut safe_reports_with_dampener: Vec<Vec<i32>> = Vec::new();
    for report in reports {
        let mut is_safe = false;
        for i in 0..report.len() {
            let mut report_with_removed_level = report.to_owned();
            report_with_removed_level.remove(i);
            if report_is_safe(&report_with_removed_level) {
                is_safe = true;
                break;
            }
        }
        if is_safe {
            safe_reports_with_dampener.push(report.to_owned());
        }
    }
    safe_reports_with_dampener
}

fn report_is_safe(report: &[i32]) -> bool {
    if report.len() < 2 {
        return true;
    }

    let ascending = report[0] < report[1];

    let mut iter = report.iter().peekable();

    while let Some(&current) = iter.next() {
        if let Some(&next) = iter.peek() {
            if (current - *next).abs() > 3 || current == *next {
                return false;
            }

            if ascending && current > *next || !ascending && current < *next {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_first_star_sample() {
        let reports = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        let safe_reports = super::get_safe_reports(&reports);
        assert_eq!(safe_reports.len(), 2);
    }

    #[test]
    fn test_second_star_sample() {
        let reports = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        let safe_reports_with_dampener = super::get_safe_reports_with_dampener(&reports);

        assert_eq!(safe_reports_with_dampener.len(), 4);
    }
}
