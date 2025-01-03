use std::fs::File;
use std::io::{BufRead, BufReader};

// Puzzle at : https://adventofcode.com/2024/day/13

fn main() {
    let path = "./src/data.txt";
    let file = File::open(path).expect("Error opening file");
    let mut input = get_input_from_file(&file);

    // first star
    let answer = get_tokens_price(&mut input, false);
    println!("First star: {}", answer);

    // second star
    let answer = get_tokens_price(&mut input, true);
    println!("Second star: {}", answer);
}

fn get_tokens_price(input: &mut Vec<Vec<(i128, i128)>>, is_large: bool) -> i128 {
    let mut tokens_price: i128 = 0;
    for game in input {
        if is_large {
            tokens_price += get_tokens_price_for_large_game(game);
        } else {
            tokens_price += get_tokens_price_for_game(game);
        }
    }

    tokens_price
}

fn get_tokens_price_for_game(game: &[(i128, i128)]) -> i128 {
    let a = game[0];
    let b = game[1];
    let target = game[2];

    let mut min_y = None;
    let mut min_x = 0;
    let mut y = 0;

    loop {
        let mut potential_x = None;
        let remaining = target.0 - b.0 * y;
        if remaining % a.0 == 0 {
            let x = remaining / a.0;
            if x >= 0 {
                potential_x = Some(x);
            }
        }
        if potential_x.is_some() && a.1 * potential_x.unwrap() + b.1 * y == target.1 {
            min_y = Some(y);
            min_x = potential_x.unwrap();
            break;
        }

        y += 1;

        if y > 100 {
            break;
        }
    }

    if let Some(y) = min_y {
        y + (min_x * 3)
    } else {
        0
    }
}

fn get_tokens_price_for_large_game(game: &[(i128, i128)]) -> i128 {
    let (a, b, mut target) = (game[0], game[1], game[2]);

    target.0 += 10000000000000;
    target.1 += 10000000000000;

    let determinant = a.0 * b.1 - a.1 * b.0;
    if determinant == 0 {
        return 0;
    }
    let (mut a_coeff, mut b_coeff) = (
        b.1 * target.0 - b.0 * target.1,
        a.0 * target.1 - a.1 * target.0,
    );
    if a_coeff % determinant != 0 || b_coeff % determinant != 0 {
        return 0;
    }
    a_coeff /= determinant;
    b_coeff /= determinant;

    3 * a_coeff + b_coeff
}

fn get_input_from_file(file: &File) -> Vec<Vec<(i128, i128)>> {
    let mut input: Vec<Vec<(i128, i128)>> = Vec::new();
    let mut game: Vec<(i128, i128)> = Vec::new();

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line_content = line.unwrap();
        if line_content.is_empty() {
            input.push(game);
            game = Vec::new();
            continue;
        } else {
            let line = line_content;
            let coor_line = line.split(":").collect::<Vec<&str>>();
            let coors: Vec<&str> = coor_line[1].split(",").collect::<Vec<&str>>();
            game.push((
                coors[0].to_string()[3..].parse::<i128>().unwrap(),
                coors[1].to_string()[3..].parse::<i128>().unwrap(),
            ));
        }
    }

    input
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_basic() {
        let mut input = vec![vec![(94, 34), (22, 67), (8400, 5400)]];
        let answer = super::get_tokens_price(&mut input, false);
        assert_eq!(answer, 280);
    }

    #[test]
    fn test_basic_none() {
        let mut input = vec![vec![(26, 66), (67, 21), (12748, 12176)]];
        let answer = super::get_tokens_price(&mut input, false);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_basic_sample() {
        let mut input = vec![
            vec![(94, 34), (22, 67), (8400, 5400)],
            vec![(26, 66), (67, 21), (12748, 12176)],
            vec![(17, 86), (84, 37), (7870, 6450)],
            vec![(69, 23), (27, 71), (18641, 10279)],
        ];
        let answer = super::get_tokens_price(&mut input, false);
        assert_eq!(answer, 480);
    }

    #[test]
    fn test_basic_large() {
        let mut input: Vec<Vec<(i128, i128)>> = vec![
            vec![(94, 34), (22, 67), (8400, 5400)],
            vec![(26, 66), (67, 21), (12748, 12176)],
        ];
        let answer = super::get_tokens_price(&mut input, true);
        assert_eq!(answer, 459236326669);
    }
}
