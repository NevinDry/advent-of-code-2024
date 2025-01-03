use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{self, BufRead};

// Puzzle at : https://adventofcode.com/2024/day/16

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn main() {
    let path = "./src/data.txt";
    let file = File::open(path).expect("Error opening file");
    let frame = get_input_from_file(&file);

    // first star
    let answer = go_over_maze(&frame);
    println!("First Star Answer: {}", answer.0);

    // second star
    let answer = find_best_spots(answer);
    println!("Second Star Answer: {}", answer);
}

#[allow(clippy::type_complexity)]
fn go_over_maze(frame: &[Vec<char>]) -> (i32, Vec<(Vec<(usize, usize, i32)>, i32, usize)>) {
    let start = (frame.len() - 2, 1);
    assert_eq!(frame[start.0][start.1], 'S');
    let end = (1, frame[0].len() - 2);
    assert_eq!(frame[end.0][end.1], 'E');

    let maze_runs = get_maze_runs(frame, start, end);

    (maze_runs.iter().min_by_key(|x| x.1).unwrap().1, maze_runs)
}

#[allow(clippy::type_complexity)]
fn find_best_spots(min_and_maze_runs: (i32, Vec<(Vec<(usize, usize, i32)>, i32, usize)>)) -> i32 {
    let mut best_spots = min_and_maze_runs
        .1
        .iter()
        .filter(|x| x.1 == min_and_maze_runs.0)
        .flat_map(|x| x.0.clone().into_iter().map(|(x, y, _)| (x, y)))
        .collect::<Vec<(usize, usize)>>();

    best_spots.sort();
    best_spots.dedup();

    best_spots.len() as i32
}

#[allow(clippy::type_complexity)]
fn get_maze_runs(
    frame: &[Vec<char>],
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(Vec<(usize, usize, i32)>, i32, usize)> {
    let mut priority_queue = BinaryHeap::new();
    let mut cache: HashMap<((usize, usize), usize), i32> = HashMap::new();
    let mut maze_runs = vec![];
    let mut min_cost = i32::MAX;

    for dir in 0..4 {
        let turn_cost = if dir == 1 { 0 } else { 1000 };
        let initial_path = vec![(start.0, start.1, turn_cost)];
        priority_queue.push(Reverse((turn_cost, start, dir, initial_path)));
    }

    while let Some(Reverse((current_cost, (x, y), direction, current_path))) = priority_queue.pop()
    {
        if current_cost > min_cost {
            continue;
        }
        if (x, y) == end {
            min_cost = min_cost.min(current_cost);
            maze_runs.push((current_path.clone(), current_cost, direction));
            continue;
        }

        if let Some(&cached_cost) = cache.get(&((x, y), direction)) {
            if current_cost > cached_cost {
                continue;
            }
        }
        cache.insert(((x, y), direction), current_cost);

        for (new_dir, &(dx, dy)) in DIRECTIONS.iter().enumerate() {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && ny >= 0 && nx < frame.len() as isize && ny < frame[0].len() as isize {
                let nx = nx as usize;
                let ny = ny as usize;
                if frame[nx][ny] == '.' || frame[nx][ny] == 'E' {
                    let turn_cost = if new_dir == direction { 0 } else { 1000 };
                    let move_cost = 1;
                    let total_cost = current_cost + turn_cost + move_cost;

                    if let Some(&cached_cost) = cache.get(&((nx, ny), new_dir)) {
                        if total_cost > cached_cost {
                            continue;
                        }
                    }

                    let mut new_path = current_path.clone();
                    new_path.push((nx, ny, turn_cost + move_cost));
                    priority_queue.push(Reverse((total_cost, (nx, ny), new_dir, new_path)));
                }
            }
        }
    }

    maze_runs
}

fn get_input_from_file(file: &File) -> Vec<Vec<char>> {
    let mut frame: Vec<Vec<char>> = Vec::new();

    let reader: io::BufReader<&File> = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Cannot get line");
        frame.push(line.chars().collect());
    }
    frame
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_first_exemple() {
        let frame = vec![
            vec![
                '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
            ],
            vec![
                '#', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '.', '.', 'E', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '#', '#', '.', '#', '.', '#', '#', '#', '.', '#',
            ],
            vec![
                '#', '.', '.', '.', '.', '.', '#', '.', '#', '.', '.', '.', '#', '.', '#',
            ],
            vec![
                '#', '.', '#', '#', '#', '.', '#', '#', '#', '#', '#', '.', '#', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '.', '.', '.', '.', '.', '.', '.', '#', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '#', '#', '#', '#', '.', '#', '#', '#', '.', '#',
            ],
            vec![
                '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '#',
            ],
            vec![
                '#', '#', '#', '.', '#', '.', '#', '#', '#', '#', '#', '.', '#', '.', '#',
            ],
            vec![
                '#', '.', '.', '.', '#', '.', '.', '.', '.', '.', '#', '.', '#', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '.', '#', '#', '#', '.', '#', '.', '#', '.', '#',
            ],
            vec![
                '#', '.', '.', '.', '.', '.', '#', '.', '.', '.', '#', '.', '#', '.', '#',
            ],
            vec![
                '#', '.', '#', '#', '#', '.', '#', '.', '#', '.', '#', '.', '#', '.', '#',
            ],
            vec![
                '#', 'S', '.', '.', '#', '.', '.', '.', '.', '.', '#', '.', '.', '.', '#',
            ],
            vec![
                '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
            ],
        ];

        let answer = go_over_maze(&frame);
        assert_eq!(answer.0, 7036);
    }

    #[test]
    fn test_second_exemple() {
        let frame = vec![
            vec![
                '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
            ],
            vec![
                '#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '.', 'E', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '.', '#', '.', '#', '.', '#', '.', '#', '.', '#', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '.', '#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '.', '#', '.', '#', '#', '#', '.', '#', '.', '#', '.', '#',
            ],
            vec![
                '#', '.', '.', '.', '#', '.', '#', '.', '#', '.', '.', '.', '.', '.', '#', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '.', '#', '.', '#', '.', '#', '#', '#', '#', '#', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '.', '.', '#', '.', '#', '.', '#', '.', '.', '.', '.', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '#', '#', '#', '#', '.', '#', '.', '#', '#', '#', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '#', '#', '.', '#', '#', '#', '#', '#', '.', '#', '#', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '.', '.', '.', '#', '.', '.', '.', '.', '.', '#', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '.', '#', '#', '#', '#', '#', '.', '#', '#', '#', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '.', '#', '#', '#', '#', '#', '#', '#', '#', '#', '.', '#',
            ],
            vec![
                '#', 'S', '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#',
            ],
            vec![
                '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
            ],
        ];

        let answer = go_over_maze(&frame);
        assert_eq!(answer.0, 11048);
    }

    #[test]
    fn test_first_exemple_best_spots() {
        let frame = vec![
            vec![
                '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
            ],
            vec![
                '#', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '.', '.', 'E', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '#', '#', '.', '#', '.', '#', '#', '#', '.', '#',
            ],
            vec![
                '#', '.', '.', '.', '.', '.', '#', '.', '#', '.', '.', '.', '#', '.', '#',
            ],
            vec![
                '#', '.', '#', '#', '#', '.', '#', '#', '#', '#', '#', '.', '#', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '.', '.', '.', '.', '.', '.', '.', '#', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '#', '#', '#', '#', '.', '#', '#', '#', '.', '#',
            ],
            vec![
                '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '#',
            ],
            vec![
                '#', '#', '#', '.', '#', '.', '#', '#', '#', '#', '#', '.', '#', '.', '#',
            ],
            vec![
                '#', '.', '.', '.', '#', '.', '.', '.', '.', '.', '#', '.', '#', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '.', '#', '#', '#', '.', '#', '.', '#', '.', '#',
            ],
            vec![
                '#', '.', '.', '.', '.', '.', '#', '.', '.', '.', '#', '.', '#', '.', '#',
            ],
            vec![
                '#', '.', '#', '#', '#', '.', '#', '.', '#', '.', '#', '.', '#', '.', '#',
            ],
            vec![
                '#', 'S', '.', '.', '#', '.', '.', '.', '.', '.', '#', '.', '.', '.', '#',
            ],
            vec![
                '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
            ],
        ];
        let answer = go_over_maze(&frame);
        let answer = find_best_spots(answer);
        assert_eq!(answer, 45);
    }

    #[test]
    fn test_second_exemple_best_spots() {
        let frame = vec![
            vec![
                '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
            ],
            vec![
                '#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '.', 'E', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '.', '#', '.', '#', '.', '#', '.', '#', '.', '#', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '.', '#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '.', '#', '.', '#', '#', '#', '.', '#', '.', '#', '.', '#',
            ],
            vec![
                '#', '.', '.', '.', '#', '.', '#', '.', '#', '.', '.', '.', '.', '.', '#', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '.', '#', '.', '#', '.', '#', '#', '#', '#', '#', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '.', '.', '#', '.', '#', '.', '#', '.', '.', '.', '.', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '#', '#', '#', '#', '.', '#', '.', '#', '#', '#', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '#', '#', '.', '#', '#', '#', '#', '#', '.', '#', '#', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '.', '.', '.', '#', '.', '.', '.', '.', '.', '#', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '.', '#', '#', '#', '#', '#', '.', '#', '#', '#', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '.', '#', '#', '#', '#', '#', '#', '#', '#', '#', '.', '#',
            ],
            vec![
                '#', 'S', '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#',
            ],
            vec![
                '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
            ],
        ];

        let answer = go_over_maze(&frame);
        let answer = find_best_spots(answer);
        assert_eq!(answer, 64);
    }
}
