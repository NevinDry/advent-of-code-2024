use std::fs::File;
use std::io::{self, BufRead};

// Puzzle at : https://adventofcode.com/2024/day/1

fn main() {
    // Setup
    let path = "./src/data.txt";
    let file = File::open(path).expect("Error opening file");

    let (mut column1, mut column2) = get_columns_from_file(&file);

    column1.sort_unstable();
    column2.sort_unstable();

    // first star
    let columns_gap = get_columns_elements_gap(&column1, &column2);
    println!("First Star Answer: {:?}", columns_gap);

    // second star
    let column_similarity = get_columns_elements_similarity(&column1, &column2);
    println!("Second Star Answer: {:?}", column_similarity);
}

fn get_columns_from_file(file: &File) -> (Vec<i32>, Vec<i32>) {
    let reader = io::BufReader::new(file);
    let mut column1: Vec<i32> = Vec::new();
    let mut column2: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Cannot get line");
        let columns: Vec<&str> = line.split_whitespace().collect();

        if columns.len() == 2 {
            if let (Ok(val1), Ok(val2)) = (columns[0].parse::<i32>(), columns[1].parse::<i32>()) {
                column1.push(val1);
                column2.push(val2);
            } else {
                eprintln!("Cannot parse line: {}", line);
            }
        } else {
            eprintln!("Line invalid: {}", line);
        }
    }
    (column1, column2)
}

fn get_columns_elements_gap(column1: &[i32], column2: &[i32]) -> i32 {
    column1
        .iter()
        .zip(column2.iter())
        .map(|(&a, &b)| (a - b).abs())
        .sum()
}

fn get_columns_elements_similarity(column1: &[i32], column2: &[i32]) -> i32 {
    column1
        .iter()
        .map(|&x| column2.iter().filter(|&&y| y == x).count() as i32 * x)
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_first_star_sample() {
        let column1 = vec![3, 4, 2, 1, 3, 3];
        let column2 = vec![3, 4, 5, 3, 9, 3];
        let gap = super::get_columns_elements_gap(&column1, &column2);
        assert_eq!(gap, 11);
    }

    #[test]
    fn test_second_star_sample() {
        let column1 = vec![3, 4, 2, 1, 3, 3];
        let column2 = vec![3, 4, 5, 3, 9, 3];
        let similarity = super::get_columns_elements_similarity(&column1, &column2);
        assert_eq!(similarity, 31);
    }
}
