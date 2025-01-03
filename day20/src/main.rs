use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

// Puzzle at : https://adventofcode.com/2024/day/20

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn main() {
    let path = "./src/data.txt";
    let file = File::open(path).expect("Error opening file");
    let frame = get_input_from_file(&file);

    // first star
    let answer = go_over_maze(&frame, 100, 2);
    println!("First Star Answer: {}", answer);

    // second star
    let answer = go_over_maze(&frame, 100, 20);
    println!("Second Star Answer: {}", answer);
}

fn go_over_maze(frame: &Vec<Vec<char>>, diff: i32, max_depth: i32) -> i64 {
    let start = frame
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|&c| c == 'S').map(|j| (i, j)))
        .unwrap_or((0, 0));

    let end = frame
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|&c| c == 'E').map(|j| (i, j)))
        .unwrap_or((0, 0));

    let maze_runs = get_maze_runs(frame, start, end, i32::MAX);
    let reference_run = maze_runs.iter().min_by_key(|x| x.1).unwrap();

    let mut reference_run_map = HashMap::new();
    for &(y, x, d) in &reference_run.0 {
        reference_run_map.insert((y, x), d);
    }

    let mut run_savings: i64 = 0;

    for dot in reference_run.0.iter() {
        let mut cache: HashSet<(isize, isize, i32)> = HashSet::new();
        let mut cheat_done: HashSet<(usize, usize)> = HashSet::new();
        run_savings += find_land(
            frame,
            &reference_run_map,
            dot.0 as isize,
            dot.1 as isize,
            *dot,
            diff,
            1,
            max_depth,
            &mut cache,
            &mut cheat_done,
        );
    }
    run_savings
}

#[allow(clippy::too_many_arguments)]
fn find_land(
    frame: &Vec<Vec<char>>,
    reference_run_map: &HashMap<(usize, usize), i32>,
    ny: isize,
    nx: isize,
    dot: (usize, usize, i32),
    diff: i32,
    depth: i32,
    max_depth: i32,
    cache: &mut HashSet<(isize, isize, i32)>,
    cheat_done: &mut HashSet<(usize, usize)>,
) -> i64 {
    if depth > max_depth || cache.contains(&(ny, nx, depth)) {
        return 0;
    }

    let mut run_savings = 0;

    for &(dy, dx) in &DIRECTIONS {
        let nny = ny + dy;
        let nnx = nx + dx;

        if nny >= 0 && nnx >= 0 && (nny as usize) < frame.len() && (nnx as usize) < frame[0].len() {
            let key = (nny as usize, nnx as usize);

            if let Some(&distance) = reference_run_map.get(&key) {
                if distance - (depth + dot.2) >= diff && !cheat_done.contains(&key) {
                    cheat_done.insert(key);
                    run_savings += 1;
                }
            } else {
                cache.insert((ny, nx, depth));
            }
            run_savings += find_land(
                frame,
                reference_run_map,
                nny,
                nnx,
                dot,
                diff,
                depth + 1,
                max_depth,
                cache,
                cheat_done,
            );
        }
    }

    run_savings
}

#[allow(clippy::type_complexity)]
fn get_maze_runs(
    frame: &[Vec<char>],
    start: (usize, usize),
    end: (usize, usize),
    max_cost: i32,
) -> Vec<(Vec<(usize, usize, i32)>, i32, usize)> {
    let mut maze_runs = vec![];

    let mut curent_runs: Vec<(Vec<(usize, usize, i32)>, i32, usize)> = vec![];
    let mut cache: HashMap<((usize, usize), usize), i32> = HashMap::new();

    for dir in 0..4 {
        curent_runs.push((vec![(start.0, start.1, 0)], 0, dir));
    }

    while let Some((current_path, current_cost, direction)) = curent_runs.pop() {
        let (x, y, _cost) = *current_path.last().unwrap();
        if current_cost > max_cost {
            continue;
        }
        if (x, y) == end {
            maze_runs.push((current_path, current_cost, direction));
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
                    let move_cost = 1;
                    let total_cost = current_cost + move_cost;
                    if let Some(&cached_cost) = cache.get(&((nx, ny), new_dir)) {
                        if total_cost > cached_cost {
                            continue;
                        }
                    }
                    let mut new_path = current_path.clone();
                    new_path.push((nx, ny, total_cost));
                    curent_runs.push((new_path, total_cost, new_dir));
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
                '#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '.', '.', '.', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '.', '#', '.', '#', '.', '#', '#', '#', '.', '#',
            ],
            vec![
                '#', 'S', '#', '.', '.', '.', '#', '.', '#', '.', '#', '.', '.', '.', '#',
            ],
            vec![
                '#', '#', '#', '#', '#', '#', '#', '.', '#', '.', '#', '.', '#', '#', '#',
            ],
            vec![
                '#', '#', '#', '#', '#', '#', '#', '.', '#', '.', '#', '.', '.', '.', '#',
            ],
            vec![
                '#', '#', '#', '#', '#', '#', '#', '.', '#', '.', '#', '#', '#', '.', '#',
            ],
            vec![
                '#', '#', '#', '.', '.', 'E', '#', '.', '.', '.', '#', '.', '.', '.', '#',
            ],
            vec![
                '#', '#', '#', '.', '#', '#', '#', '#', '#', '#', '#', '.', '#', '#', '#',
            ],
            vec![
                '#', '.', '.', '.', '#', '#', '#', '.', '.', '.', '#', '.', '.', '.', '#',
            ],
            vec![
                '#', '.', '#', '#', '#', '#', '#', '.', '#', '.', '#', '#', '#', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '.', '.', '#', '.', '#', '.', '#', '.', '.', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '.', '#', '.', '#', '.', '#', '.', '#', '#', '#',
            ],
            vec![
                '#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '.', '.', '#', '#', '#',
            ],
            vec![
                '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
            ],
        ];

        let answer = go_over_maze(&frame, 8, 2);

        assert_eq!(answer, 14);
    }

    #[test]
    fn test_second_star() {
        let frame = vec![
            vec![
                '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
            ],
            vec![
                '#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '.', '.', '.', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '.', '#', '.', '#', '.', '#', '#', '#', '.', '#',
            ],
            vec![
                '#', 'S', '#', '.', '.', '.', '#', '.', '#', '.', '#', '.', '.', '.', '#',
            ],
            vec![
                '#', '#', '#', '#', '#', '#', '#', '.', '#', '.', '#', '.', '#', '#', '#',
            ],
            vec![
                '#', '#', '#', '#', '#', '#', '#', '.', '#', '.', '#', '.', '.', '.', '#',
            ],
            vec![
                '#', '#', '#', '#', '#', '#', '#', '.', '#', '.', '#', '#', '#', '.', '#',
            ],
            vec![
                '#', '#', '#', '.', '.', 'E', '#', '.', '.', '.', '#', '.', '.', '.', '#',
            ],
            vec![
                '#', '#', '#', '.', '#', '#', '#', '#', '#', '#', '#', '.', '#', '#', '#',
            ],
            vec![
                '#', '.', '.', '.', '#', '#', '#', '.', '.', '.', '#', '.', '.', '.', '#',
            ],
            vec![
                '#', '.', '#', '#', '#', '#', '#', '.', '#', '.', '#', '#', '#', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '.', '.', '#', '.', '#', '.', '#', '.', '.', '.', '#',
            ],
            vec![
                '#', '.', '#', '.', '#', '.', '#', '.', '#', '.', '#', '.', '#', '#', '#',
            ],
            vec![
                '#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '.', '.', '#', '#', '#',
            ],
            vec![
                '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
            ],
        ];

        let answer = go_over_maze(&frame, 50, 20);
        assert_eq!(answer, 285);
    }
}
