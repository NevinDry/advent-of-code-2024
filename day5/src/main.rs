use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

fn main() {
    let path = "./src/data.txt";
    let file = File::open(path).expect("Error opening file");

    let (ordering_lines, updates_lines) = get_lines_from_file(&file);
    let mut correct_ordered_lines = Vec::new();
    let mut incorrected_ordered_lines = Vec::new();

    for line in updates_lines {
        if is_in_correct_order(&line, &ordering_lines) {
            correct_ordered_lines.push(line);
        } else {
            incorrected_ordered_lines.push(line);
        }
    }
    let answer = get_middle_page_number(&correct_ordered_lines);
    println!("Answer 1: {:?}", answer);

    let incorrected_ordered_lines =
        reorder_incorrect_lines(&incorrected_ordered_lines, &ordering_lines);
    let answer = get_middle_page_number(&incorrected_ordered_lines);
    println!("Answer 2: {:?}", answer);
}

fn reorder_incorrect_lines(
    incorrected_ordered_lines: &[String],
    ordering_lines: &[String],
) -> Vec<String> {
    let mut correct_ordered_lines = Vec::new();
    for line in incorrected_ordered_lines {
        let numbers_to_check = line.split(',').collect::<Vec<&str>>();
        let pair_count_number = numbers_to_check
            .iter()
            .map(|number_checked| {
                let position: i32 = ordering_lines
                    .iter()
                    .filter(|line| line.split('|').all(|num| numbers_to_check.contains(&num)))
                    .collect_vec()
                    .iter()
                    .map(|line: &&String| -> i32 {
                        let first = line.split('|').next().unwrap();
                        if number_checked == &first {
                            1
                        } else {
                            0
                        }
                    })
                    .sum();
                (position, number_checked.parse::<i32>().unwrap())
            })
            .sorted_by_key(|&(x, _)| x)
            .map(|(_, y)| y.to_string())
            .collect::<Vec<String>>();

        correct_ordered_lines.push(pair_count_number.join(","));
    }
    correct_ordered_lines
}

fn get_middle_page_number(ordered_lines: &[String]) -> i32 {
    ordered_lines
        .iter()
        .map(|line| {
            let line = line.split(',').collect::<Vec<&str>>();
            let middle_index = line.len() / 2;
            line[middle_index].parse::<i32>().unwrap()
        })
        .sum()
}

fn is_in_correct_order(line: &str, ordering_lines: &[String]) -> bool {
    let numbers_to_check = line.split(',').collect::<Vec<&str>>();

    let filtered_lines: Vec<&String> = ordering_lines
        .iter()
        .filter(|line| line.split('|').all(|num| numbers_to_check.contains(&num)))
        .collect();

    numbers_to_check
        .iter()
        .enumerate()
        .all(|(number_index, number_checked)| {
            let position = filtered_lines
                .iter()
                .filter(|line| {
                    line.split('|')
                        .nth(1)
                        .map_or(false, |num| num == *number_checked)
                })
                .count();
            position == number_index
        })
}

fn get_lines_from_file(file: &File) -> (Vec<String>, Vec<String>) {
    let mut ordering_lines = Vec::new();
    let mut updates_lines = Vec::new();
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Cannot get line");
        if line.contains('|') {
            ordering_lines.push(line);
        } else if line.contains(',') {
            updates_lines.push(line);
        }
    }
    (ordering_lines, updates_lines)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_first_star_sample() {
        let ordering_lines: Vec<String> = vec![
            "47|53".to_string(),
            "97|13".to_string(),
            "97|61".to_string(),
            "97|47".to_string(),
            "75|29".to_string(),
            "61|13".to_string(),
            "75|53".to_string(),
            "29|13".to_string(),
            "97|29".to_string(),
            "53|29".to_string(),
            "61|53".to_string(),
            "97|53".to_string(),
            "61|29".to_string(),
            "47|13".to_string(),
            "75|47".to_string(),
            "97|75".to_string(),
            "47|61".to_string(),
            "75|61".to_string(),
            "47|29".to_string(),
            "75|13".to_string(),
            "53|13".to_string(),
        ];

        let update_lines = vec![
            "75,47,61,53,29".to_string(),
            "97,61,53,29,13".to_string(),
            "75,29,13".to_string(),
            "75,97,47,61,53".to_string(),
            "61,13,29".to_string(),
            "97,13,75,29,47".to_string(),
        ];

        let mut correct_ordered_lines = Vec::new();

        for line in update_lines {
            if super::is_in_correct_order(&line, &ordering_lines) {
                correct_ordered_lines.push(line);
            }
        }

        let answer = super::get_middle_page_number(&correct_ordered_lines);
        assert_eq!(answer, 143);
    }

    #[test]
    fn test_second_star_sample() {
        let ordering_lines: Vec<String> = vec![
            "47|53".to_string(),
            "97|13".to_string(),
            "97|61".to_string(),
            "97|47".to_string(),
            "75|29".to_string(),
            "61|13".to_string(),
            "75|53".to_string(),
            "29|13".to_string(),
            "97|29".to_string(),
            "53|29".to_string(),
            "61|53".to_string(),
            "97|53".to_string(),
            "61|29".to_string(),
            "47|13".to_string(),
            "75|47".to_string(),
            "97|75".to_string(),
            "47|61".to_string(),
            "75|61".to_string(),
            "47|29".to_string(),
            "75|13".to_string(),
            "53|13".to_string(),
        ];

        let updates_lines = vec![
            "75,47,61,53,29".to_string(),
            "97,61,53,29,13".to_string(),
            "75,29,13".to_string(),
            "75,97,47,61,53".to_string(),
            "61,13,29".to_string(),
            "97,13,75,29,47".to_string(),
        ];

        let mut incorrected_ordered_lines = Vec::new();

        for line in updates_lines {
            if !super::is_in_correct_order(&line, &ordering_lines) {
                incorrected_ordered_lines.push(line);
            }
        }
        let correct_ordered_lines =
            super::reorder_incorrect_lines(&incorrected_ordered_lines, &ordering_lines);
        let answer = super::get_middle_page_number(&correct_ordered_lines);
        assert_eq!(answer, 123);
    }
}
