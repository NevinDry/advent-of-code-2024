use std::fs::File;
use std::io::{self, BufRead};
use std::vec;

fn main() {
    let path = "./src/data.txt";
    let file = File::open(path).expect("Error opening file");
    let initial_bytes_fallen = get_input_from_file(&file);

    // first star
    let mut bytes_fallen = initial_bytes_fallen
        .clone()
        .into_iter()
        .take(1024)
        .collect::<Vec<_>>();
    let grid = create_grid(&bytes_fallen, 71);
    let answer = find_way_out(&grid);
    println!("Answer 1: {:?}", answer.0);

    // second star
    let mut breaker = (0, 0);
    for i in 12..initial_bytes_fallen.len() {
        bytes_fallen.push(initial_bytes_fallen[i]);
        let grid = create_grid(&bytes_fallen, 71);
        let answer = find_way_out(&grid);
        if answer.1 == false {
            breaker = initial_bytes_fallen[i];
            break;
        }
    }

    println!("Answer 2: {:?}", breaker);
}

fn find_way_out(grid: &Vec<Vec<char>>) -> (usize, bool) {
    let mut cache = vec![vec![false; grid.len()]; grid.len()];
    let mut queue = vec![];
    let mut moves = 0;

    queue.push((0, 0));
    cache[0][0] = true;

    while !queue.is_empty() {
        let mut new_queue = vec![];
        for (x, y) in queue {
            if grid[y][x] == '#' {
                continue;
            }

            if x == grid.len() - 1 && y == grid.len() - 1 {
                return (moves, true);
            }

            if x > 0 && !cache[y][x - 1] {
                new_queue.push((x - 1, y));
                cache[y][x - 1] = true;
            }

            if x < grid.len() - 1 && !cache[y][x + 1] {
                new_queue.push((x + 1, y));
                cache[y][x + 1] = true;
            }

            if y > 0 && !cache[y - 1][x] {
                new_queue.push((x, y - 1));
                cache[y - 1][x] = true;
            }

            if y < grid.len() - 1 && !cache[y + 1][x] {
                new_queue.push((x, y + 1));
                cache[y + 1][x] = true;
            }
        }
        moves += 1;
        queue = new_queue;
    }
    (moves, false)
}

fn create_grid(bytes_fallen: &Vec<(usize, usize)>, size: usize) -> Vec<Vec<char>> {
    let mut grid = vec![vec!['.'; size]; size];

    for (x, y) in bytes_fallen {
        grid[*y][*x] = '#';
    }
    grid
}

fn get_input_from_file(file: &File) -> Vec<(usize, usize)> {
    let mut bytes_fallen = vec![];
    let reader: io::BufReader<&File> = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let coor = line.split(",").collect::<Vec<&str>>();
        bytes_fallen.push((
            coor[0].parse::<usize>().unwrap(),
            coor[1].parse::<usize>().unwrap(),
        ));
    }
    bytes_fallen
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maze() {
        let bytes_fallen = vec![
            (5, 4),
            (4, 2),
            (4, 5),
            (3, 0),
            (2, 1),
            (6, 3),
            (2, 4),
            (1, 5),
            (0, 6),
            (3, 3),
            (2, 6),
            (5, 1),
            (1, 2),
            (5, 5),
            (2, 5),
            (6, 5),
            (1, 4),
            (0, 4),
            (6, 4),
            (1, 1),
            (6, 1),
            (1, 0),
            (0, 5),
            (1, 6),
            (2, 0),
        ];

        let bytes_fallen: Vec<(usize, usize)> =
            bytes_fallen.into_iter().take(12).collect::<Vec<_>>();

        let grid = create_grid(&bytes_fallen, 7);
        let answer = find_way_out(&grid);

        println!("{:?}", answer);
        assert_eq!(answer.0, 22);
    }

    #[test]
    fn test_find_breaker() {
        let initial_bytes_fallen = vec![
            (5, 4),
            (4, 2),
            (4, 5),
            (3, 0),
            (2, 1),
            (6, 3),
            (2, 4),
            (1, 5),
            (0, 6),
            (3, 3),
            (2, 6),
            (5, 1),
            (1, 2),
            (5, 5),
            (2, 5),
            (6, 5),
            (1, 4),
            (0, 4),
            (6, 4),
            (1, 1),
            (6, 1),
            (1, 0),
            (0, 5),
            (1, 6),
            (2, 0),
        ];

        let mut bytes_fallen: Vec<(usize, usize)> = initial_bytes_fallen
            .clone()
            .into_iter()
            .take(12)
            .collect::<Vec<_>>();

        let mut breaker = (0, 0);
        for i in 12..initial_bytes_fallen.len() {
            bytes_fallen.push(initial_bytes_fallen[i]);
            let grid = create_grid(&bytes_fallen, 7);
            let answer = find_way_out(&grid);
            if answer.1 == false {
                breaker = initial_bytes_fallen[i];
                break;
            }
        }

        println!("{:?}", breaker);
        assert_eq!(breaker, (6, 1));
    }
}
