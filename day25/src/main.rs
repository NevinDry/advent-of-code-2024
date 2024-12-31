use std::fs::File;
use std::io::{self, BufRead};
fn main() {
    let path = "./src/data.txt";
    let file = File::open(path).expect("Error opening file");

    let (keys, locks) = get_input_from_file(&file);

    // first star
    let keys = prepare_input(keys);
    let locks = prepare_input(locks);
    let answer = try_keys(&keys, &locks);
    println!("Answer 1 {:?}", answer);
}

fn try_keys(
    keys: &Vec<(usize, usize, usize, usize, usize)>,
    locks: &Vec<(usize, usize, usize, usize, usize)>,
) -> usize {
    let mut result = 0;
    for key in keys {
        for lock in locks {
            if key.0 + lock.0 > 7
                || key.1 + lock.1 > 7
                || key.2 + lock.2 > 7
                || key.3 + lock.3 > 7
                || key.4 + lock.4 > 7
            {
                continue;
            } else {
                result += 1;
            }
        }
    }
    result
}

fn prepare_input(input: Vec<Vec<Vec<char>>>) -> Vec<(usize, usize, usize, usize, usize)> {
    let mut output = Vec::new();
    for element in input {
        let mut sequence = (0, 0, 0, 0, 0);
        for y in 0..element.len() {
            for x in 0..element[y].len() {
                if element[y][x] == '#' {
                    match x {
                        0 => sequence.0 += 1,
                        1 => sequence.1 += 1,
                        2 => sequence.2 += 1,
                        3 => sequence.3 += 1,
                        4 => sequence.4 += 1,
                        _ => panic!("Invalid symbol"),
                    }
                }
            }
        }
        output.push(sequence);
    }
    output
}

fn get_input_from_file(file: &File) -> (Vec<Vec<Vec<char>>>, Vec<Vec<Vec<char>>>) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    let mut current_grid: Vec<Vec<char>> = Vec::new();

    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();

        if line.trim().is_empty() {
            if !current_grid.is_empty() {
                if current_grid[0].iter().all(|&c| c == '#') {
                    locks.push(current_grid.clone());
                } else {
                    keys.push(current_grid.clone());
                }
                current_grid.clear();
            }
        } else {
            current_grid.push(line.chars().collect());
        }
    }
    if !current_grid.is_empty() {
        if current_grid[0].iter().all(|&c| c == '#') {
            locks.push(current_grid);
        } else {
            keys.push(current_grid);
        }
    }

    (keys, locks)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first() {
        let keys = vec![vec![
            vec!['.', '.', '.', '.', '.'],
            vec!['#', '.', '.', '.', '.'],
            vec!['#', '.', '.', '.', '.'],
            vec!['#', '.', '.', '.', '#'],
            vec!['#', '.', '#', '.', '#'],
            vec!['#', '.', '#', '#', '#'],
            vec!['#', '#', '#', '#', '#'],
        ]];

        let locks = vec![vec![
            vec!['#', '#', '#', '#', '#'],
            vec!['.', '#', '#', '#', '#'],
            vec!['.', '#', '#', '#', '#'],
            vec!['.', '#', '#', '#', '#'],
            vec!['.', '#', '.', '#', '.'],
            vec!['.', '#', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
        ]];

        let keys = prepare_input(keys);
        let locks = prepare_input(locks);

        println!("{:?}", keys);
        println!("{:?}", locks);
        let result = try_keys(&keys, &locks);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_sample() {
        let path = "./src/data_sample.txt";
        let file = File::open(path).expect("Error opening file");

        let (keys, locks) = get_input_from_file(&file);

        let keys = prepare_input(keys);
        let locks = prepare_input(locks);

        println!("{:?}", keys);
        println!("{:?}", locks);

        let result = try_keys(&keys, &locks);
        assert_eq!(result, 3);
    }
}
