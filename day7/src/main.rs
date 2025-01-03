use itertools::Itertools;
use std::fmt::Error;
use std::fs::File;
use std::io::{self, BufRead};

// Puzzle at : https://adventofcode.com/2024/day/7

fn main() {
    let path = "./src/data.txt";
    let file = File::open(path).expect("Error opening file");
    let result_and_numbers: Vec<(u64, Vec<u16>)> = get_operations_from_file(&file);

    // first star
    let anwser = get_valid_operations_sum(&result_and_numbers, vec!["+", "*"]);
    println!("First Star Answer: {:?}", anwser);

    // second star
    let anwser = get_valid_operations_sum(&result_and_numbers, vec!["+", "*", "||"]);
    println!("Second Star Answer: {:?}", anwser);
}

fn get_valid_operations_sum(result_and_numbers: &[(u64, Vec<u16>)], operators: Vec<&str>) -> u64 {
    let mut valid_operation_sum = 0;

    result_and_numbers.iter().for_each(|(result, numbers)| {
        match operation_is_valid(result, numbers, operators.clone()) {
            Ok(true) => {
                valid_operation_sum += result;
            }
            Ok(false) => {}
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    });

    valid_operation_sum
}

fn operation_is_valid(result: &u64, numbers: &[u16], operators: Vec<&str>) -> Result<bool, Error> {
    if (0..numbers.len() - 1)
        .map(|_| operators.clone())
        .multi_cartesian_product()
        .any(|operators_permutation| {
            let mut operation_result: u64 = numbers[0].into();
            for (i, operator) in operators_permutation.iter().enumerate() {
                let next_value = numbers[i + 1] as u64;
                operation_result = match *operator {
                    "+" => operation_result + next_value,
                    "*" => operation_result * next_value,
                    "||" => format!("{}{}", operation_result, next_value)
                        .parse::<u64>()
                        .unwrap(),
                    _ => {
                        return false;
                    }
                };
            }
            operation_result == *result
        })
    {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn get_operations_from_file(file: &File) -> Vec<(u64, Vec<u16>)> {
    let mut operations_results: Vec<(u64, Vec<u16>)> = Vec::new();
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        if let Some((before_colon, after_colon)) = line.unwrap().split_once(':') {
            let second_numbers = after_colon
                .split_whitespace()
                .filter_map(|num| num.parse::<u16>().ok())
                .collect::<Vec<u16>>();
            operations_results.push((before_colon.trim().parse::<u64>().unwrap(), second_numbers));
        }
    }
    operations_results
}
#[cfg(test)]
mod tests {

    #[test]
    fn test_first_star_sample() {
        let input = vec![
            (190, vec![10, 19]),
            (3267, vec![81, 40, 27]),
            (83, vec![17, 5]),
            (156, vec![15, 6]),
            (7290, vec![6, 8, 6, 15]),
            (161011, vec![16, 10, 13]),
            (192, vec![17, 8, 14]),
            (21037, vec![9, 7, 18, 13]),
            (292, vec![11, 6, 16, 20]),
        ];
        let answer = super::get_valid_operations_sum(&input, vec!["+", "*"]);
        assert_eq!(answer, 3749);
    }

    #[test]
    fn test_second_star_sample() {
        let input = vec![
            (190, vec![10, 19]),
            (3267, vec![81, 40, 27]),
            (83, vec![17, 5]),
            (156, vec![15, 6]),
            (7290, vec![6, 8, 6, 15]),
            (161011, vec![16, 10, 13]),
            (192, vec![17, 8, 14]),
            (21037, vec![9, 7, 18, 13]),
            (292, vec![11, 6, 16, 20]),
        ];
        let answer = super::get_valid_operations_sum(&input, vec!["+", "*", "||"]);
        assert_eq!(answer, 11387);
    }
}
