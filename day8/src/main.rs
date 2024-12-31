use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let path = "./src/data.txt";
    let file = File::open(path).expect("Error opening file");
    let input = get_input_from_file(&file);

    // first star
    let antinodes_count = get_antenas_antinodes_count(&input, false);
    println!("Answer 1: {:?}", antinodes_count);

    // second star
    let antinodes_count = get_antenas_antinodes_count(&input, true);
    println!("Answer 2: {:?}", antinodes_count);
}

fn get_antenas_antinodes_count(input: &Vec<Vec<char>>, find_resonant: bool) -> u64 {
    let antenas = get_existing_antenas(input);

    antenas
        .iter()
        .flat_map(|antena| get_antena_antinodes_positions(input, *antena, find_resonant))
        .collect::<HashSet<_>>()
        .len() as u64
}

fn get_antena_antinodes_positions(
    input: &[Vec<char>],
    antena: char,
    find_resonant: bool,
) -> Vec<(i32, i32)> {
    let mut antinode_positions = vec![];

    let antena_positions = get_antena_positions(input, antena);

    antena_positions.iter().for_each(|(a, b)| {
        antena_positions.iter().for_each(|(x, y)| {
            if (x, y) != (a, b) {
                let mut antinodes = vec![];
                match find_resonant {
                    true => {
                        antinodes.push((*x, *y));
                        antinodes.extend(get_resonant_antinodes(input, (*x, *y), (*a, *b)));
                    }
                    false => {
                        antinodes.push((x + (x - a), y + (y - b)));
                    }
                }
                for antinode in antinodes {
                    if !antinode_positions.contains(&antinode)
                        && antinode.0 >= 0
                        && antinode.1 >= 0
                        && antinode.0 < input.len() as i32
                        && antinode.1 < input[0].len() as i32
                    {
                        antinode_positions.push(antinode);
                    }
                }
            }
        });
    });
    antinode_positions
}

fn get_resonant_antinodes(
    input: &[Vec<char>],
    first_antena_pos: (i32, i32),
    second_antena_pos: (i32, i32),
) -> Vec<(i32, i32)> {
    let distance_move = (
        first_antena_pos.0 - second_antena_pos.0,
        first_antena_pos.1 - second_antena_pos.1,
    );
    let mut antinode = (
        first_antena_pos.0 + distance_move.0,
        first_antena_pos.1 + distance_move.1,
    );
    let mut resonant_antinodes = Vec::new();

    while antinode.0 >= 0
        && antinode.0 < input.len() as i32
        && antinode.1 >= 0
        && antinode.1 < input[0].len() as i32
    {
        resonant_antinodes.push(antinode);
        antinode = (antinode.0 + distance_move.0, antinode.1 + distance_move.1);
    }

    resonant_antinodes
}
fn get_existing_antenas(input: &[Vec<char>]) -> Vec<char> {
    input
        .iter()
        .flat_map(|line| line.iter().filter(|&&c| c != '.'))
        .collect::<HashSet<_>>()
        .into_iter()
        .cloned()
        .collect()
}
fn get_antena_positions(input: &[Vec<char>], antena: char) -> Vec<(i32, i32)> {
    input
        .iter()
        .enumerate()
        .flat_map(|(x, line)| {
            line.iter().enumerate().filter_map(move |(y, &c)| {
                if c == antena {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect()
}
fn get_input_from_file(file: &File) -> Vec<Vec<char>> {
    let reader = BufReader::new(file);
    let input: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    extract_lines(input)
}
fn extract_lines(input: Vec<String>) -> Vec<Vec<char>> {
    input.iter().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_first_star_easy_sample() {
        let input = vec![
            "..........".to_string(),
            "..........".to_string(),
            "..........".to_string(),
            "....a.....".to_string(),
            "..........".to_string(),
            ".....a....".to_string(),
            "..........".to_string(),
            "..........".to_string(),
            "..........".to_string(),
            "..........".to_string(),
        ];
        let input = super::extract_lines(input);
        let answer = super::get_antenas_antinodes_count(&input, false);
        assert_eq!(answer, 2);
    }

    #[test]
    fn test_first_star_multi_sample() {
        let input = vec![
            "..........".to_string(),
            "..........".to_string(),
            "..........".to_string(),
            "....a.....".to_string(),
            "........a.".to_string(),
            ".....a....".to_string(),
            "..........".to_string(),
            "..........".to_string(),
            "..........".to_string(),
            "..........".to_string(),
        ];
        let input = super::extract_lines(input);
        let answer = super::get_antenas_antinodes_count(&input, false);
        assert_eq!(answer, 4);
    }

    #[test]
    fn test_first_star_more_antenas_sample() {
        let input = vec![
            "..........".to_string(),
            "..........".to_string(),
            "..........".to_string(),
            "....a.....".to_string(),
            "........a.".to_string(),
            ".....a....".to_string(),
            "..........".to_string(),
            "......A...".to_string(),
            "..........".to_string(),
            "..........".to_string(),
        ];
        let input = super::extract_lines(input);
        let answer = super::get_antenas_antinodes_count(&input, false);
        assert_eq!(answer, 4);
    }

    #[test]
    fn test_first_star_complete_sample() {
        let input = vec![
            "............".to_string(),
            "........0...".to_string(),
            ".....0......".to_string(),
            ".......0....".to_string(),
            "....0.......".to_string(),
            "......A.....".to_string(),
            "............".to_string(),
            "............".to_string(),
            "........A...".to_string(),
            ".........A..".to_string(),
            "............".to_string(),
            "............".to_string(),
        ];
        let input = super::extract_lines(input);
        let answer = super::get_antenas_antinodes_count(&input, false);
        assert_eq!(answer, 14);
    }

    #[test]
    fn test_second_star_easy_sample() {
        let input = vec![
            "T.........".to_string(),
            "...T......".to_string(),
            ".T........".to_string(),
            "..........".to_string(),
            "..........".to_string(),
            "..........".to_string(),
            "..........".to_string(),
            "..........".to_string(),
            "..........".to_string(),
            "..........".to_string(),
        ];
        let input = super::extract_lines(input);
        let answer = super::get_antenas_antinodes_count(&input, true);
        assert_eq!(answer, 9);
    }
}
