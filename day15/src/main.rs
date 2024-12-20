use std::fs::File;
use std::io::{self, BufRead};
use std::thread;

fn main() {
    let path = "./src/data.txt";
    let file = File::open(path).expect("Error opening file");
    let (mut frame, moves) = get_input_from_file(&file);

    // first star
    let answer = perform_fish_duty(&mut frame, &moves);
    println!("Answer 1: {:?}", answer);

    // second star
    let file = File::open(path).expect("Error opening file");
    let (mut frame, moves) = get_input_doubled_from_file(&file);
    let answer = perform_fish_duty_doubled(&mut frame, &moves);
    println!("Answer 2: {:?}", answer);
}

fn perform_fish_duty(frame: &mut Vec<Vec<char>>, moves: &Vec<char>) -> i32 {
    let mut fish_position = get_fish_initial_position(frame);

    for movement in moves {
        move_fish(frame, &mut fish_position, movement);
        // print_frame(&frame);
        // thread::sleep(std::time::Duration::from_millis(50));
    }

    let mut sum = 0;
    for y in 0..frame.len() {
        for x in 0..frame[y].len() {
            if frame[y][x] == 'O' {
                sum += 100 * y as i32 + x as i32;
            }
        }
    }
    sum
}

fn get_fish_initial_position(frame: &Vec<Vec<char>>) -> (usize, usize) {
    frame
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|&c| c == '@').map(|j| (i, j)))
        .unwrap_or((0, 0))
}

fn print_frame(frame: &Vec<Vec<char>>) -> () {
    print!("{}[2J", 27 as char);
    for row in frame {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

fn move_fish(
    frame: &mut Vec<Vec<char>>,
    fish_position: &mut (usize, usize),
    movement: &char,
) -> () {
    let (y, x) = *fish_position;
    let mut moved = false;
    match movement {
        '>' => {
            if frame[y][x + 1] == '#' {
                return;
            } else if frame[y][x + 1] == 'O' {
                let mut line_char = x + 2;
                loop {
                    if frame[y][line_char] == '#' {
                        break;
                    }
                    if frame[y][line_char] == '.' {
                        moved = true;
                        for j in x..line_char {
                            frame[y][j + 1] = 'O';
                        }
                        fish_position.1 += 1;
                        break;
                    }
                    line_char += 1;
                }
            } else {
                moved = true;
                fish_position.1 += 1;
            }
        }
        '<' => {
            if frame[y][x - 1] == '#' {
                return;
            } else if frame[y][x - 1] == 'O' {
                let mut line_char = x - 2;
                loop {
                    if frame[y][line_char] == '#' {
                        break;
                    }
                    if frame[y][line_char] == '.' {
                        moved = true;
                        for j in (line_char + 1)..=x {
                            frame[y][j - 1] = 'O';
                        }
                        fish_position.1 -= 1;

                        break;
                    }
                    line_char -= 1;
                }
            } else {
                moved = true;
                fish_position.1 -= 1;
            }
        }
        '^' => {
            if frame[y - 1][x] == '#' {
                return;
            } else if frame[y - 1][x] == 'O' {
                let mut line_char = y - 2;
                loop {
                    if frame[line_char][x] == '#' {
                        break;
                    }
                    if frame[line_char][x] == '.' {
                        moved = true;
                        for i in (line_char + 1)..=y {
                            frame[i - 1][x] = 'O';
                        }
                        fish_position.0 -= 1;
                        break;
                    }
                    line_char -= 1;
                }
            } else {
                moved = true;
                fish_position.0 -= 1;
            }
        }
        'v' => {
            if frame[y + 1][x] == '#' {
                return;
            } else if frame[y + 1][x] == 'O' {
                let mut line_char = y + 2;
                loop {
                    if frame[line_char][x] == '#' {
                        break;
                    }
                    if frame[line_char][x] == '.' {
                        moved = true;
                        for i in y..line_char {
                            frame[i + 1][x] = 'O';
                        }
                        fish_position.0 += 1;
                        break;
                    }
                    line_char += 1;
                }
            } else {
                moved = true;
                fish_position.0 += 1;
            }
        }

        _ => {
            panic!("Invalid movement");
        }
    };

    if moved {
        frame[fish_position.0][fish_position.1] = '@';
        frame[y][x] = '.';
    }
}

fn perform_fish_duty_doubled(frame: &mut Vec<Vec<char>>, moves: &Vec<char>) -> i32 {
    let mut fish_position = get_fish_initial_position(frame);

    for movement in moves {
        move_fish_doubled(frame, &mut fish_position, movement);
        // print_frame(&frame);
        // thread::sleep(std::time::Duration::from_millis(5));
    }

    let mut sum = 0;
    for y in 0..frame.len() {
        for x in 0..frame[y].len() {
            if frame[y][x] == '[' {
                sum += 100 * y as i32 + x as i32;
            }
        }
    }
    sum
}

fn move_fish_doubled(
    frame: &mut Vec<Vec<char>>,
    fish_position: &mut (usize, usize),
    movement: &char,
) -> () {
    let (y, x) = *fish_position;
    let mut moved = false;
    match movement {
        '>' => {
            if frame[y][x + 1] == '#' {
                return;
            } else if frame[y][x + 1] == '[' {
                let mut line_char: usize = x + 3;
                loop {
                    if frame[y][line_char] == '#' {
                        break;
                    }
                    if frame[y][line_char] == '.' {
                        moved = true;
                        for j in (x + 2..line_char).step_by(2) {
                            frame[y][j] = '[';
                            frame[y][j + 1] = ']';
                        }
                        fish_position.1 += 1;
                        break;
                    }
                    line_char += 2;
                }
            } else {
                moved = true;
                fish_position.1 += 1;
            }
        }
        '<' => {
            if frame[y][x - 1] == '#' {
                return;
            } else if frame[y][x - 1] == ']' {
                let mut line_char = x - 3;
                loop {
                    if frame[y][line_char] == '#' {
                        break;
                    }
                    if frame[y][line_char] == '.' {
                        moved = true;
                        for j in (line_char + 2..x).rev().step_by(2) {
                            frame[y][j - 1] = ']';
                            frame[y][j - 2] = '[';
                        }
                        fish_position.1 -= 1;
                        break;
                    }
                    line_char -= 2;
                }
            } else {
                moved = true;
                fish_position.1 -= 1;
            }
        }

        '^' => {
            if frame[y - 1][x] == '#' {
                return;
            } else if frame[y - 1][x] == '[' || frame[y - 1][x] == ']' {
                let boxes_possiibilities: &mut Vec<(bool, usize, (usize, usize))> = &mut vec![];
                let mut boxes_possiibilities = find_boxes_possibilities(
                    -1,
                    frame,
                    &(fish_position.0 as usize, fish_position.1 as usize),
                    boxes_possiibilities,
                );

                let not_possible = boxes_possiibilities
                    .iter()
                    .any(|(is_possible, _, (_, _))| *is_possible == false);

                if not_possible {
                    return;
                } else {
                    boxes_possiibilities.sort();
                    boxes_possiibilities.dedup();

                    for boxe in boxes_possiibilities.iter() {
                        frame[boxe.1][boxe.2 .0] = '.';
                        frame[boxe.1][boxe.2 .1] = '.';
                        frame[boxe.1 - 1][boxe.2 .0] = '[';
                        frame[boxe.1 - 1][boxe.2 .1] = ']';
                    }
                    moved = true;
                    fish_position.0 -= 1;
                }
            } else {
                moved = true;
                fish_position.0 -= 1;
            }
        }
        'v' => {
            if frame[y as usize + 1][x as usize] == '#' {
                return;
            } else if frame[y as usize + 1][x as usize] == '['
                || frame[y as usize + 1][x as usize] == ']'
            {
                let boxes_possiibilities: &mut Vec<(bool, usize, (usize, usize))> = &mut vec![];
                let mut boxes_possiibilities = find_boxes_possibilities(
                    1,
                    frame,
                    &(fish_position.0 as usize, fish_position.1 as usize),
                    boxes_possiibilities,
                );

                let not_possible = boxes_possiibilities
                    .iter()
                    .any(|(is_possible, _, (_, _))| *is_possible == false);

                if not_possible {
                    return;
                } else {
                    boxes_possiibilities.sort();
                    boxes_possiibilities.dedup();

                    for boxe in boxes_possiibilities.iter().rev() {
                        frame[boxe.1][boxe.2 .0] = '.';
                        frame[boxe.1][boxe.2 .1] = '.';
                        frame[boxe.1 + 1][boxe.2 .0] = '[';
                        frame[boxe.1 + 1][boxe.2 .1] = ']';
                    }
                    moved = true;
                    fish_position.0 += 1;
                }
            } else {
                moved = true;
                fish_position.0 += 1;
            }
        }
        _ => {
            panic!("Invalid movement");
        }
    };

    if moved {
        frame[fish_position.0 as usize][fish_position.1 as usize] = '@';
        frame[y][x] = '.';
    }
}

fn find_boxes_possibilities(
    direction: i32,
    frame: &Vec<Vec<char>>,
    cube_position: &(usize, usize),
    mut boxes_possibilities: &mut Vec<(bool, usize, (usize, usize))>,
) -> Vec<(bool, usize, (usize, usize))> {
    if frame[(cube_position.0 as i32 + direction) as usize][cube_position.1] == '#' {
        boxes_possibilities.push((false, cube_position.0 - 1, (0, 0)));
    } else if frame[(cube_position.0 as i32 + direction) as usize][cube_position.1] == '[' {
        boxes_possibilities.push((
            true,
            (cube_position.0 as i32 + direction) as usize,
            (cube_position.1, cube_position.1 + 1),
        ));
        find_boxes_possibilities(
            direction,
            frame,
            &(
                (cube_position.0 as i32 + direction) as usize,
                cube_position.1,
            ),
            &mut boxes_possibilities,
        );
        find_boxes_possibilities(
            direction,
            frame,
            &(
                (cube_position.0 as i32 + direction) as usize,
                cube_position.1 + 1,
            ),
            &mut boxes_possibilities,
        );
    } else if frame[(cube_position.0 as i32 + direction) as usize][cube_position.1 as usize] == ']'
    {
        boxes_possibilities.push((
            true,
            (cube_position.0 as i32 + direction) as usize,
            (cube_position.1 - 1, cube_position.1),
        ));
        find_boxes_possibilities(
            direction,
            frame,
            &(
                (cube_position.0 as i32 + direction) as usize,
                cube_position.1,
            ),
            &mut boxes_possibilities,
        );
        find_boxes_possibilities(
            direction,
            frame,
            &(
                (cube_position.0 as i32 + direction) as usize,
                cube_position.1 - 1,
            ),
            &mut boxes_possibilities,
        );
    };
    boxes_possibilities.to_vec()
}

fn get_input_from_file(file: &File) -> (Vec<Vec<char>>, Vec<char>) {
    let mut frame: Vec<Vec<char>> = Vec::new();
    let mut moves: Vec<char> = Vec::new();

    let reader: io::BufReader<&File> = io::BufReader::new(file);
    let mut first_part = true;
    for line in reader.lines() {
        let line = line.expect("Cannot get line");
        if first_part {
            frame.push(line.chars().collect());
        } else {
            moves.extend(line.chars());
        }

        if line == "" {
            first_part = false;
        }
    }
    (frame, moves)
}

fn get_input_doubled_from_file(file: &File) -> (Vec<Vec<char>>, Vec<char>) {
    let mut frame: Vec<Vec<char>> = Vec::new();
    let mut moves: Vec<char> = Vec::new();

    let reader: io::BufReader<&File> = io::BufReader::new(file);
    let mut first_part = true;
    for line in reader.lines() {
        let line = line.expect("Cannot get line");
        if first_part {
            let mut line_vec = vec![];
            for c in line.chars() {
                match c {
                    '#' => {
                        line_vec.push('#');
                        line_vec.push('#');
                    }
                    '.' => {
                        line_vec.push('.');
                        line_vec.push('.');
                    }
                    'O' => {
                        line_vec.push('[');
                        line_vec.push(']');
                    }
                    '@' => {
                        line_vec.push('@');
                        line_vec.push('.');
                    }
                    _ => {
                        panic!("Invalid character");
                    }
                }
            }
            frame.push(line_vec);
        } else {
            moves.extend(line.chars());
        }

        if line == "" {
            first_part = false;
        }
    }
    (frame, moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_fish() {
        let mut frame = vec![
            vec!['#', '#', '#', '#', '#', '#', '#', '#'],
            vec!['#', '.', '.', 'O', '.', 'O', '.', '#'],
            vec!['#', '#', '@', '.', 'O', '.', '.', '#'],
            vec!['#', '.', '.', '.', 'O', '.', '.', '#'],
            vec!['#', '.', '#', '.', 'O', '.', '.', '#'],
            vec!['#', '.', '.', '.', 'O', '.', '.', '#'],
            vec!['#', '.', '.', '.', '.', '.', '.', '#'],
            vec!['#', '#', '#', '#', '#', '#', '#', '#'],
        ];

        // let moves = vec!['<', '<', 'v', '>'];

        let moves = vec![
            '<', '^', '^', '>', '>', '>', 'v', 'v', '<', 'v', '>', '>', 'v', '<', '<',
        ];

        let answer = perform_fish_duty(&mut frame, &moves);
        assert_eq!(answer, 2028);
    }

    #[test]
    fn test_more_complex_fish() {
        let mut frame = vec![
            vec!['#', '#', '#', '#', '#', '#', '#', '#', '#', '#'],
            vec!['#', '.', '.', 'O', '.', '.', 'O', '.', 'O', '#'],
            vec!['#', '.', '.', '.', '.', '.', '.', 'O', '.', '#'],
            vec!['#', '.', 'O', 'O', '.', '.', 'O', '.', 'O', '#'],
            vec!['#', '.', '.', 'O', '@', '.', '.', 'O', '.', '#'],
            vec!['#', 'O', '#', '.', '.', 'O', '.', '.', '.', '#'],
            vec!['#', 'O', '.', '.', 'O', '.', '.', 'O', '.', '#'],
            vec!['#', '.', 'O', 'O', '.', 'O', '.', 'O', 'O', '#'],
            vec!['#', '.', '.', '.', '.', 'O', '.', '.', '.', '#'],
            vec!['#', '#', '#', '#', '#', '#', '#', '#', '#', '#'],
        ];

        let moves = vec![
            '<', 'v', 'v', '>', '^', '<', 'v', '^', '>', 'v', '>', '^', 'v', 'v', '^', 'v', '>',
            'v', '<', '>', 'v', '^', 'v', '<', 'v', '<', '^', 'v', 'v', '<', '<', '<', '^', '>',
            '<', '<', '>', '<', '>', '>', 'v', '<', 'v', 'v', 'v', '<', '>', '^', 'v', '^', '>',
            '^', '<', '<', '<', '>', '<', '<', 'v', '<', '<', '<', 'v', '^', 'v', 'v', '^', 'v',
            '>', '^', 'v', 'v', 'v', '<', '<', '^', '>', '^', 'v', '^', '^', '>', '<', '<', '>',
            '>', '>', '<', '>', '^', '<', '<', '>', '<', '^', 'v', 'v', '^', '^', '<', '>', 'v',
            'v', 'v', '<', '>', '>', '<', '^', '^', 'v', '>', '^', '>', 'v', 'v', '<', '>', 'v',
            '<', '<', '<', '<', 'v', '<', '^', 'v', '>', '^', '<', '^', '^', '>', '>', '>', '^',
            '<', 'v', '<', 'v', '>', '<', '>', 'v', 'v', '>', 'v', '^', 'v', '^', '<', '>', '>',
            '<', '>', '>', '>', '>', '<', '^', '^', '>', 'v', 'v', '>', 'v', '<', '^', '^', '^',
            '>', '>', 'v', '^', 'v', '^', '<', '^', '^', '>', 'v', '^', '^', '>', 'v', '^', '<',
            '^', 'v', '>', 'v', '<', '>', '>', 'v', '^', 'v', '^', '<', 'v', '>', 'v', '^', '^',
            '<', '^', '^', 'v', 'v', '<', '<', '<', 'v', '<', '^', '>', '>', '^', '^', '^', '^',
            '>', '>', '>', 'v', '^', '<', '>', 'v', 'v', 'v', '^', '>', '<', 'v', '<', '<', '<',
            '>', '^', '^', '^', 'v', 'v', '^', '<', 'v', 'v', 'v', '>', '^', '>', 'v', '<', '^',
            '^', '^', '^', 'v', '<', '>', '^', '>', 'v', 'v', 'v', 'v', '>', '<', '>', '>', 'v',
            '^', '<', '<', '^', '^', '^', '^', '^', '^', '>', '<', '^', '>', '<', '>', '>', '>',
            '<', '>', '^', '^', '<', '<', '^', '^', 'v', '>', '>', '>', '<', '^', '<', 'v', '>',
            '^', '<', 'v', 'v', '>', '>', 'v', '>', '>', '>', '^', 'v', '>', '<', '>', '^', 'v',
            '>', '<', '<', '<', '<', 'v', '>', '>', 'v', '<', 'v', '<', 'v', '>', 'v', 'v', 'v',
            '>', '^', '<', '>', '<', '<', '>', '^', '>', '<', '^', '>', '>', '<', '>', '^', 'v',
            '<', '>', '<', '^', 'v', 'v', 'v', '<', '^', '^', '<', '>', '<', 'v', '<', '<', '<',
            '<', '<', '>', '<', '^', 'v', '<', '<', '<', '>', '<', '<', '<', '^', '^', '<', 'v',
            '<', '^', '^', '^', '>', '<', '^', '>', '>', '^', '<', 'v', '^', '>', '<', '<', '<',
            '^', '>', '>', '^', 'v', '<', 'v', '^', 'v', '<', 'v', '^', '>', '^', '>', '>', '^',
            'v', '>', 'v', 'v', '>', '^', '<', '<', '^', 'v', '<', '>', '>', '<', '<', '>', '<',
            '<', 'v', '<', '<', 'v', '>', '<', '>', 'v', '<', '^', 'v', 'v', '<', '<', '<', '>',
            '^', '^', 'v', '^', '>', '^', '^', '>', '>', '>', '<', '<', '^', 'v', '>', '>', 'v',
            '^', 'v', '>', '<', '^', '^', '>', '>', '^', '<', '>', 'v', 'v', '^', '<', '>', '<',
            '^', '^', '>', '^', '^', '^', '<', '>', '<', 'v', 'v', 'v', 'v', 'v', '^', 'v', '<',
            'v', '<', '<', '>', '^', 'v', '<', 'v', '>', 'v', '<', '<', '^', '>', '<', '<', '>',
            '<', '<', '>', '<', '<', '<', '^', '^', '<', '<', '<', '^', '<', '<', '>', '>', '<',
            '<', '>', '<', '^', '^', '^', '>', '^', '^', '<', '>', '^', '>', 'v', '<', '>', '^',
            '^', '>', 'v', 'v', '<', '^', 'v', '^', 'v', '<', 'v', 'v', '>', '^', '<', '>', '<',
            'v', '<', '^', 'v', '>', '^', '^', '^', '>', '>', '>', '^', '^', 'v', 'v', 'v', '^',
            '>', 'v', 'v', 'v', '<', '>', '>', '>', '^', '<', '^', '>', '>', '>', '>', '>', '^',
            '<', '<', '^', 'v', '>', '^', 'v', 'v', 'v', '<', '>', '^', '<', '>', '<', '<', 'v',
            '>', 'v', '^', '^', '>', '>', '>', '<', '<', '^', '^', '<', '>', '>', '^', 'v', '^',
            '<', 'v', '^', 'v', 'v', '<', '>', 'v', '^', '<', '<', '>', '^', '<', '^', 'v', '^',
            'v', '>', '<', '^', '<', '<', '<', '>', '<', '<', '^', '<', 'v', '>', '<', 'v', '<',
            '>', 'v', 'v', '>', '>', 'v', '>', '<', 'v', '^', '<', 'v', 'v', '<', '>', 'v', '^',
            '<', '<', '^',
        ];

        let answer = perform_fish_duty(&mut frame, &moves);
        assert_eq!(answer, 10092);
    }
}
