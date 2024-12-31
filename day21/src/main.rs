use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec;

fn main() {
    let path = "./src/data.txt";
    let file = File::open(path).expect("Error opening file");

    let codes = get_input_from_file(&file);

    // first star
    let answer = perform_codes(codes.clone(), 2);
    println!("First star answer : {:?}", answer);

    // second star
    let answer = perform_codes(codes, 25);
    println!("Second star answer : {:?}", answer);
}

fn wrap_moves(
    vertical_move: i32,
    horizontal_move: i32,
    depth: usize,
    is_horizontal: bool,
    cache: &mut HashMap<(i32, i32, usize, bool), i64>,
) -> i64 {
    if let Some(&cached_result) = cache.get(&(vertical_move, horizontal_move, depth, is_horizontal))
    {
        return cached_result;
    }

    let mut moves = vec![];

    let direction = if vertical_move > 0 { '^' } else { 'v' };
    let steps = vertical_move.abs();
    for _ in 0..steps {
        moves.push(direction);
    }
    let direction = if horizontal_move > 0 { '<' } else { '>' };
    let steps = horizontal_move.abs();
    for _ in 0..steps {
        moves.push(direction);
    }

    if is_horizontal {
        moves.reverse();
    }

    moves.push('A');
    let result = if depth == 0 {
        moves.len() as i64
    } else {
        let mut result = 0;
        let mut position = get_arrow_key('A');
        for mov in moves {
            let target = get_arrow_key(mov);
            let current_position = position;
            position = target;
            let vertical_move = current_position.0 - target.0;
            let horizontal_move = current_position.1 - target.1;

            if vertical_move == 0 || horizontal_move == 0 {
                result += wrap_moves(vertical_move, horizontal_move, depth - 1, false, cache)
            } else if target == (1, 0) && current_position.0 == 0 {
                result += wrap_moves(vertical_move, horizontal_move, depth - 1, false, cache)
            } else if current_position == (1, 0) && target.0 == 0 {
                result += wrap_moves(vertical_move, horizontal_move, depth - 1, true, cache)
            } else {
                let horizontal = wrap_moves(vertical_move, horizontal_move, depth - 1, true, cache);
                let vertical = wrap_moves(vertical_move, horizontal_move, depth - 1, false, cache);

                if horizontal > vertical {
                    result += vertical;
                } else {
                    result += horizontal;
                }
            }
        }
        result
    };

    cache.insert(
        (vertical_move, horizontal_move, depth, is_horizontal),
        result,
    );
    result
}

fn perform_codes(codes: Vec<Vec<char>>, depth: usize) -> i64 {
    let mut result = 0;
    for mut code in codes {
        let mut position: (i32, i32) = get_num_key('A');
        let mut code_result = 0;
        let mut cache = HashMap::new();

        for c in code.clone() {
            let target = get_num_key(c);
            let current_position = position;
            let vertical_move = position.0 - target.0;
            let horizontal_move = position.1 - target.1;
            position = target;
            if current_position.0 == 3 && target.1 == 0 {
                code_result += wrap_moves(vertical_move, horizontal_move, depth, false, &mut cache)
            } else if current_position.1 == 0 && target.0 == 3 {
                code_result += wrap_moves(vertical_move, horizontal_move, depth, true, &mut cache)
            } else {
                let horizontal =
                    wrap_moves(vertical_move, horizontal_move, depth, true, &mut cache);
                let vertical = wrap_moves(vertical_move, horizontal_move, depth, false, &mut cache);

                if horizontal > vertical {
                    code_result += vertical;
                } else {
                    code_result += horizontal;
                }
            }
        }

        code.remove(3);
        let concatenated_code: String = code.iter().collect();
        result += code_result * concatenated_code.parse::<i64>().unwrap();
    }

    result
}

fn get_input_from_file(file: &File) -> Vec<Vec<char>> {
    let mut codes = vec![];
    let reader: io::BufReader<&File> = io::BufReader::new(file);
    for line in reader.lines() {
        let mut code = vec![];
        let line = line.unwrap();
        for key in line.chars() {
            code.push(key);
        }
        codes.push(code);
    }
    codes
}

fn get_num_key(key: char) -> (i32, i32) {
    match key {
        '7' => (0, 0),
        '8' => (0, 1),
        '9' => (0, 2),
        '4' => (1, 0),
        '5' => (1, 1),
        '6' => (1, 2),
        '1' => (2, 0),
        '2' => (2, 1),
        '3' => (2, 2),
        '0' => (3, 1),
        'A' => (3, 2),
        _ => panic!("Invalid Num key"),
    }
}

fn get_arrow_key(key: char) -> (i32, i32) {
    match key {
        '^' => (0, 1),
        'A' => (0, 2),
        '<' => (1, 0),
        'v' => (1, 1),
        '>' => (1, 2),
        _ => panic!("Invalid Arrow key"),
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    #[test]
    fn test_first_pad() {
        let input = vec![vec!['0', '2', '9', 'A']];
        let answer = super::perform_codes(input, 2);

        assert_eq!(answer, 1972);
    }

    #[test]
    fn test_third_pad() {
        let input = vec![vec!['1', '7', '9', 'A']];
        let answer = super::perform_codes(input, 2);

        assert_eq!(answer, 12172);
    }

    #[test]
    fn test_two_codes() {
        let input = vec![vec!['0', '2', '9', 'A'], vec!['9', '8', '0', 'A']];
        let answer = super::perform_codes(input, 2);

        assert_eq!(answer, 60772);
    }

    #[test]
    fn test_sample() {
        let input = vec![
            vec!['0', '2', '9', 'A'],
            vec!['9', '8', '0', 'A'],
            vec!['1', '7', '9', 'A'],
            vec!['4', '5', '6', 'A'],
            vec!['3', '7', '9', 'A'],
        ];
        let answer = super::perform_codes(input, 2);

        assert_eq!(answer, 126384);
    }
}
