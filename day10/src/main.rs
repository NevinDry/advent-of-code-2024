use std::fs::File;
use std::io::{BufRead, BufReader};

// Puzzle at : https://adventofcode.com/2024/day/10

fn main() {
    let path = "./src/data.txt";
    let file = File::open(path).expect("Error opening file");
    let input: Vec<Vec<usize>> = get_input_from_file(&file);

    // first star
    let trailheads = test_get_trailheads(&input);
    let answers = find_trails_score(&input, trailheads);
    println!("First Star Answer: {}", answers);

    // second star
    let trailheads = test_get_trailheads(&input);
    let answers = find_all_trails_score(&input, trailheads);
    println!("Second Star Answer: {}", answers);
}

fn find_trails_score(input: &Vec<Vec<usize>>, trails_head: Vec<(usize, usize)>) -> i32 {
    let mut score = 0;
    for (x, y) in trails_head {
        let mut path_completed: Vec<(usize, usize)> = vec![];
        find_head_score(input, y, x, &mut path_completed);
        let head_score = path_completed.len() as i32;
        score += head_score;
    }
    score
}

fn find_head_score(
    input: &Vec<Vec<usize>>,
    y: usize,
    x: usize,
    path_completed: &mut Vec<(usize, usize)>,
) {
    let current = input[y][x];

    if current == 9 && !path_completed.contains(&(x, y)) {
        path_completed.push((x, y));
        return;
    }

    if x < input[y].len() - 1 {
        if let Some(next_right) = input[y].get(x + 1) {
            if *next_right == current + 1 {
                find_head_score(input, y, x + 1, path_completed);
            }
        } else {
            return;
        }
    }
    if y < input.len() - 1 {
        if let Some(next_down) = input[y + 1].get(x) {
            if *next_down == current + 1 {
                find_head_score(input, y + 1, x, path_completed);
            }
        } else {
            return;
        }
    }

    if x > 0 {
        if let Some(next_left) = input[y].get(x - 1) {
            if *next_left == current + 1 {
                find_head_score(input, y, x - 1, path_completed);
            }
        } else {
            return;
        }
    }
    if y > 0 {
        if let Some(row_above) = input.get(y - 1) {
            if let Some(next_up) = row_above.get(x) {
                if *next_up == current + 1 {
                    find_head_score(input, y - 1, x, path_completed);
                }
            }
        }
    }
}

fn find_all_trails_score(input: &Vec<Vec<usize>>, trails_head: Vec<(usize, usize)>) -> i32 {
    let mut score = 0;
    for (x, y) in trails_head {
        let trails_score = find_all_head_score(input, y, x);
        score += trails_score;
    }
    score
}

fn find_all_head_score(input: &Vec<Vec<usize>>, y: usize, x: usize) -> i32 {
    let current = input[y][x];
    let mut score: i32 = 0;

    if current == 9 {
        score += 1;
        return score;
    }

    if x < input[y].len() - 1 {
        if let Some(next_right) = input[y].get(x + 1) {
            if *next_right == current + 1 {
                score += find_all_head_score(input, y, x + 1);
            }
        } else {
            return score;
        }
    }
    if y < input.len() - 1 {
        if let Some(next_down) = input[y + 1].get(x) {
            if *next_down == current + 1 {
                score += find_all_head_score(input, y + 1, x);
            }
        } else {
            return score;
        }
    }

    if x > 0 {
        if let Some(next_left) = input[y].get(x - 1) {
            if *next_left == current + 1 {
                score += find_all_head_score(input, y, x - 1);
            }
        } else {
            return score;
        }
    }
    if y > 0 {
        if let Some(row_above) = input.get(y - 1) {
            if let Some(next_up) = row_above.get(x) {
                if *next_up == current + 1 {
                    score += find_all_head_score(input, y - 1, x);
                }
            }
        } else {
            return score;
        }
    }
    score
}

fn test_get_trailheads(input: &[Vec<usize>]) -> Vec<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, digit)| if *digit == 0 { Some((x, y)) } else { None })
        })
        .collect()
}

fn get_input_from_file(file: &File) -> Vec<Vec<usize>> {
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect::<Vec<Vec<usize>>>()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_basic_trail() {
        let input = vec![
            vec![8, 8, 8, 0, 8, 8, 8],
            vec![8, 8, 8, 1, 8, 8, 8],
            vec![8, 8, 8, 2, 8, 8, 8],
            vec![6, 5, 4, 3, 4, 5, 6],
            vec![7, 1, 1, 1, 1, 1, 7],
            vec![8, 1, 1, 1, 1, 1, 8],
            vec![9, 1, 1, 1, 1, 1, 9],
        ];

        let trails_head = super::test_get_trailheads(&input);
        assert_eq!(trails_head.len(), 1);
        let count = super::find_trails_score(&input, trails_head);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_basic_trai_2() {
        let input = vec![
            vec![2, 2, 9, 0, 2, 2, 9],
            vec![5, 5, 5, 1, 2, 9, 8],
            vec![8, 8, 8, 2, 6, 2, 7],
            vec![6, 5, 4, 3, 4, 5, 6],
            vec![7, 6, 5, 5, 9, 8, 7],
            vec![8, 7, 6, 2, 2, 2, 2],
            vec![9, 8, 7, 2, 2, 2, 2],
        ];

        let trails_head = super::test_get_trailheads(&input);
        assert_eq!(trails_head.len(), 1);
        let count = super::find_trails_score(&input, trails_head);
        assert_eq!(count, 4);
    }

    #[test]
    fn test_get_trailhead_count() {
        let input = vec![
            vec![8, 9, 0, 1, 0, 1, 2, 3],
            vec![7, 8, 1, 2, 1, 8, 7, 4],
            vec![8, 7, 4, 3, 0, 9, 6, 5],
            vec![9, 6, 5, 4, 9, 8, 7, 4],
            vec![4, 5, 6, 7, 8, 9, 0, 3],
            vec![3, 2, 0, 1, 9, 0, 1, 2],
            vec![0, 1, 3, 2, 9, 8, 0, 1],
            vec![1, 0, 4, 5, 6, 7, 3, 2],
        ];
        let trail_heads = super::test_get_trailheads(&input);
        assert_eq!(trail_heads.len(), 9);
        let score = super::find_trails_score(&input, trail_heads);
        assert_eq!(score, 36);
    }

    #[test]
    fn test_get_trailhead_all_count() {
        let input = vec![
            vec![8, 9, 0, 1, 0, 1, 2, 3],
            vec![7, 8, 1, 2, 1, 8, 7, 4],
            vec![8, 7, 4, 3, 0, 9, 6, 5],
            vec![9, 6, 5, 4, 9, 8, 7, 4],
            vec![4, 5, 6, 7, 8, 9, 0, 3],
            vec![3, 2, 0, 1, 9, 0, 1, 2],
            vec![0, 1, 3, 2, 9, 8, 0, 1],
            vec![1, 0, 4, 5, 6, 7, 3, 2],
        ];
        let trail_heads = super::test_get_trailheads(&input);
        assert_eq!(trail_heads.len(), 9);
        let score = super::find_all_trails_score(&input, trail_heads);
        assert_eq!(score, 81);
    }
}
