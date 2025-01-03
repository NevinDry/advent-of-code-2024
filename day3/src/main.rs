use std::fs::File;
use std::io::Read;

use regex::Regex;

// Puzzle at : https://adventofcode.com/2024/day/3

const MUL_REGEX: &str = r"mul\(\d+,\d+\)";
const MUL_REGEX_WITH_STATEMENTS: &str = r"mul\(\d+,\d+\)|do\(\)|don't\(\)";

fn main() {
    let path = "./src/data.txt";
    let mut file = File::open(path).expect("Error opening file");
    let mut input = String::new();

    file.read_to_string(&mut input).expect("Error reading file");

    // first star
    let regex = regex::Regex::new(MUL_REGEX).unwrap();
    let cleaned_input = get_vec_cleaned_input(&input, regex);
    let answer: i32 = multiply_cleaned_input(cleaned_input);
    println!("First Star Answer: {:?}", answer);

    // second star
    let regex = regex::Regex::new(MUL_REGEX_WITH_STATEMENTS).unwrap();
    let cleaned_input = get_vec_cleaned_input(&input, regex);
    let answer = multiply_cleaned_input_with_statements(cleaned_input);
    println!("Second Star Answer: {:?}", answer);
}

fn get_vec_cleaned_input(input: &str, regex: Regex) -> Vec<&str> {
    let cleaned_inputs = regex.find_iter(input).map(|m| m.as_str()).collect();
    cleaned_inputs
}

fn multiply_cleaned_input(cleaned_input: Vec<&str>) -> i32 {
    cleaned_input
        .iter()
        .map(|multiply_str| {
            multiply_str
                .trim_start_matches("mul(")
                .trim_end_matches(")")
                .split(",")
                .collect()
        })
        .map(|numbers: Vec<&str>| {
            let num1 = numbers[0].parse::<i32>().unwrap();
            let num2 = numbers[1].parse::<i32>().unwrap();
            num1 * num2
        })
        .sum()
}

fn multiply_cleaned_input_with_statements(cleaned_input: Vec<&str>) -> i32 {
    let mut should_take = true;
    cleaned_input
        .iter()
        .map(|cleaned_str| {
            match (
                cleaned_str.contains("don't()"),
                cleaned_str.contains("do()"),
            ) {
                (true, _) => {
                    should_take = false;
                    None
                }
                (_, true) => {
                    should_take = true;
                    None
                }
                (_, _) if should_take => Some(
                    cleaned_str
                        .trim_start_matches("mul(")
                        .trim_end_matches(")")
                        .split(',')
                        .collect(),
                ),
                (_, _) => None,
            }
        })
        .map(|numbers: Option<Vec<&str>>| match numbers {
            Some(numbers) => {
                let num1 = numbers[0].parse::<i32>().unwrap();
                let num2 = numbers[1].parse::<i32>().unwrap();
                num1 * num2
            }
            None => 0,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::MUL_REGEX;
    use crate::MUL_REGEX_WITH_STATEMENTS;

    #[test]
    fn test_first_star_sample() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let regex = regex::Regex::new(MUL_REGEX).unwrap();
        let cleaned_input = super::get_vec_cleaned_input(input, regex);
        let answer = super::multiply_cleaned_input(cleaned_input);
        assert_eq!(answer, 161);
    }

    #[test]
    fn test_second_star_sample() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let regex = regex::Regex::new(MUL_REGEX_WITH_STATEMENTS).unwrap();
        let cleaned_input = super::get_vec_cleaned_input(input, regex);
        let answer = super::multiply_cleaned_input_with_statements(cleaned_input);
        assert_eq!(answer, 48);
    }
}
