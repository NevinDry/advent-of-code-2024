use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, Error};
fn main() {
    let path = "./src/data.txt";
    let file = File::open(path).expect("Error opening file");
    let mut lines_vec = get_lines_from_file(&file);
    let mut lines_vec_clone = lines_vec.clone();

    // first star
    let answer = get_guard_duty(&mut lines_vec);
    println!("Answer 1: {:?}", answer);

    // second star
    let answer = get_guard_duty_infini_loop(&mut lines_vec_clone);
    println!("Answer 2: {:?}", answer);
}

/// First Star (Perform the guard duty)
/// Move depending on obstacle and direction
/// If we reach the end of the board, we calculate the number of X
fn get_guard_duty(lines_vec: &mut Vec<Vec<char>>) -> i32 {
    let starting_point = lines_vec
        .iter()
        .enumerate()
        .find(|(_, line)| line.iter().any(|&c| c == '^'))
        .map(|(i, line)| (i, line.iter().position(|&c| c == '^').unwrap()))
        .unwrap();

    let mut saved_positions = HashSet::new();
    let guard_duty = perform_duty(lines_vec, starting_point, &mut saved_positions).unwrap();
    guard_duty.iter().flatten().filter(|&&c| c == 'X').count() as i32
}

fn perform_duty(
    lines_vec: &mut Vec<Vec<char>>,
    position: (usize, usize),
    saved_positions: &mut HashSet<String>,
) -> Result<Vec<Vec<char>>, Error> {
    let mut i = position.0;
    let mut j = position.1;

    match lines_vec[i][j] {
        '>' => {
            if can_right_move(lines_vec, i, j) {
                lines_vec[i][j] = 'X';
                lines_vec[i][j + 1] = '>';
                j += 1;
            } else {
                lines_vec[i][j] = 'v';
            }
        }
        '<' => {
            if can_left_move(lines_vec, i, j) {
                lines_vec[i][j] = 'X';
                lines_vec[i][j - 1] = '<';
                j -= 1;
            } else {
                lines_vec[i][j] = '^';
            }
        }
        '^' => {
            if can_up_move(lines_vec, i, j) {
                lines_vec[i][j] = 'X';
                lines_vec[i - 1][j] = '^';
                i -= 1;
            } else {
                lines_vec[i][j] = '>';
            }
        }
        'v' => {
            if can_down_move(lines_vec, i, j) {
                lines_vec[i][j] = 'X';
                lines_vec[i + 1][j] = 'v';
                i += 1;
            } else {
                lines_vec[i][j] = '<';
            }
        }
        _ => {
            println!("should not end up here")
        }
    }

    if i == lines_vec.len() - 1 || i == 0 || j == lines_vec[i - 1].len() - 1 || j == 0 {
        lines_vec[i][j] = 'X';
        Ok(lines_vec.clone())
    } else {
        // Check if we are stuck in an infinite loop
        let current_position = format!("{}-{}{}", i, j, lines_vec[i][j]);
        if saved_positions.contains(&current_position) {
            return Err(Error::new(io::ErrorKind::Other, "Infinite loop detected"));
        }
        saved_positions.insert(current_position);

        // Recursive call to perform all the duty
        perform_duty(lines_vec, (i, j), saved_positions)
    }
}

/// Second Start (Infinite loop) - reusing the same function as the first star
/// Ugly solution, but it works, we try a guard duty with every new obstacle possible.
/// If we detect an infinite loop wuth the given obstacle, we increment the infinite_guard_count
fn get_guard_duty_infini_loop(lines_vec: &mut Vec<Vec<char>>) -> i32 {
    let mut infinite_guard_count = 0;
    let starting_point = lines_vec
        .iter()
        .enumerate()
        .find(|(_, line)| line.iter().any(|&c| c == '^'))
        .map(|(i, line)| (i, line.iter().position(|&c| c == '^').unwrap()))
        .unwrap();
    let original_lines_vec = lines_vec.clone();
    let mut saved_positions = HashSet::new();
    saved_positions.insert(format!("{}-{}^", starting_point.0, starting_point.1));
    println!("Starting exploring possibilities....");

    let mut i = 0;
    while i < lines_vec.len() {
        let mut j = 0;
        while j < lines_vec[i].len() {
            if lines_vec[i][j] == '.' {
                lines_vec[i][j] = '#';

                // Perform the duty and catch infinite loop
                if perform_duty(lines_vec, starting_point, &mut saved_positions).is_err() {
                    infinite_guard_count += 1;
                    println!("loop detected: {:?}", infinite_guard_count);
                }

                // Reset the board
                lines_vec[i][j] = '.';
                saved_positions = HashSet::new();
                saved_positions.insert(format!("{}-{}^", starting_point.0, starting_point.1));
                *lines_vec = original_lines_vec.clone();
            }
            j += 1;
        }
        i += 1;
    }

    infinite_guard_count
}

// Directional functions
fn can_right_move(lines_vec: &[Vec<char>], i: usize, j: usize) -> bool {
    if lines_vec[i][j + 1] == '.' || lines_vec[i][j + 1] == 'X' {
        return true;
    } else if lines_vec[i][j + 1] == '#' {
        return false;
    }
    false
}

fn can_left_move(lines_vec: &[Vec<char>], i: usize, j: usize) -> bool {
    if lines_vec[i][j - 1] == '.' || lines_vec[i][j - 1] == 'X' {
        return true;
    } else if lines_vec[i][j - 1] == '#' {
        return false;
    }
    false
}

fn can_up_move(lines_vec: &[Vec<char>], i: usize, j: usize) -> bool {
    if lines_vec[i - 1][j] == '.' || lines_vec[i - 1][j] == 'X' {
        return true;
    } else if lines_vec[i - 1][j] == '#' {
        return false;
    }
    false
}

fn can_down_move(lines_vec: &[Vec<char>], i: usize, j: usize) -> bool {
    if lines_vec[i + 1][j] == '.' || lines_vec[i + 1][j] == 'X' {
        return true;
    } else if lines_vec[i + 1][j] == '#' {
        return false;
    }
    false
}

fn get_lines_from_file(file: &File) -> Vec<Vec<char>> {
    let reader = io::BufReader::new(file);
    let mut lines_vec: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Cannot get line");
        let char_vec: Vec<char> = line.chars().collect();
        lines_vec.push(char_vec);
    }
    lines_vec
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_first_star_sample() {
        let input = "....#.....
                        .........#
                        ..........
                        ..#.......
                        .......#..
                        ..........
                        .#..^.....
                        ........#.
                        #.........
                        ......#...";

        let mut input: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect();
        let answer = super::get_guard_duty(&mut input);
        assert_eq!(answer, 41);
    }

    #[test]
    fn test_second_star_sample() {
        let input = "....#.....
                        .........#
                        ..........
                        ..#.......
                        .......#..
                        ..........
                        .#..^.....
                        ........#.
                        #.........
                        ......#...";

        let mut input: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect();
        let answer = super::get_guard_duty_infini_loop(&mut input);
        assert_eq!(answer, 6);
    }
}
