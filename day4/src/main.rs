use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let path = "./src/data.txt";
    let file = File::open(path).expect("Error opening file");
    let lines_vec = get_lines_from_file(&file);

    // first star
    let answer = get_xmas_count(&lines_vec);
    println!("Answer 1: {:?}", answer);

    // second star
    println!("{:?}", lines_vec);
    let answer = get_xmas_x_count(&lines_vec);
    println!("Answer 2: {:?}", answer);
}

fn get_xmas_count(lines_vec: &Vec<Vec<char>>) -> i32 {
    lines_vec
        .iter()
        .enumerate()
        .map(|(line_index, line)| {
            line.iter()
                .enumerate()
                .filter(|&(_, &c)| c == 'X')
                .map(|(char_index, _)| {
                    horizontal_xmas_count(line, char_index)
                        + vertical_xmas_count(lines_vec, line_index, char_index)
                        + diagonal_xmas_count(lines_vec, line_index, char_index)
                })
                .sum::<i32>()
        })
        .sum()
}

fn get_xmas_x_count(lines_vec: &Vec<Vec<char>>) -> i32 {
    lines_vec
        .iter()
        .enumerate()
        .map(|(line_index, line)| {
            line.iter()
                .enumerate()
                .filter(|&(_, &c)| c == 'A')
                .map(|(char_index, _)| diagonal_mas_count(lines_vec, line_index, char_index))
                .sum::<i32>()
        })
        .sum()
}

fn horizontal_xmas_count(line: &Vec<char>, char_index: usize) -> i32 {
    let mut xmas_count = 0;
    if char_index + 3 < line.len() && line[char_index + 1] == 'M' && line[char_index + 2] == 'A' && line[char_index + 3] == 'S' {
        xmas_count += 1;
    }

    if char_index >= 3 && line[char_index - 1] == 'M' && line[char_index - 2] == 'A' && line[char_index - 3] == 'S' {
        xmas_count += 1;
    }
    xmas_count
}

fn vertical_xmas_count(line_vec: &Vec<Vec<char>>, line_index: usize, char_index: usize) -> i32 {
    let mut xmas_count = 0;

    if line_index + 3 < line_vec.len() && line_vec[line_index + 1][char_index] == 'M'
            && line_vec[line_index + 2][char_index] == 'A' && line_vec[line_index + 3][char_index] == 'S' {
        xmas_count += 1;
    }

    if line_index >= 3 && line_vec[line_index - 1][char_index] == 'M'
            && line_vec[line_index - 2][char_index] == 'A' && line_vec[line_index - 3][char_index] == 'S' {
        xmas_count += 1;
    }

    xmas_count
}

fn diagonal_xmas_count(line_vec: &Vec<Vec<char>>, line_index: usize, char_index: usize) -> i32 {
    let mut xmas_count = 0;

    if line_index + 3 < line_vec.len() && char_index + 3 < line_vec[line_index].len() && line_vec[line_index + 1][char_index + 1] == 'M'
            && line_vec[line_index + 2][char_index + 2] == 'A' && line_vec[line_index + 3][char_index + 3] == 'S' {
        xmas_count += 1;
    }

    if line_index >= 3 && char_index >= 3 && line_vec[line_index - 1][char_index - 1] == 'M'
            && line_vec[line_index - 2][char_index - 2] == 'A' && line_vec[line_index - 3][char_index - 3] == 'S' {
        xmas_count += 1;
    }

    if line_index + 3 < line_vec.len() && char_index >= 3 && line_vec[line_index + 1][char_index - 1] == 'M'
            && line_vec[line_index + 2][char_index - 2] == 'A' && line_vec[line_index + 3][char_index - 3] == 'S' {
        xmas_count += 1;
    }

    if line_index >= 3 && char_index + 3 < line_vec[line_index].len() && line_vec[line_index - 1][char_index + 1] == 'M'
            && line_vec[line_index - 2][char_index + 2] == 'A' && line_vec[line_index - 3][char_index + 3] == 'S' {
        xmas_count += 1;
    }

    xmas_count
}

fn diagonal_mas_count(line_vec: &Vec<Vec<char>>, line_index: usize, char_index: usize) -> i32 {
    let mut xmas_count: i32 = 0;

    if char_index > 0
        && char_index < line_vec[line_index].len() - 1
        && line_index > 0
        && line_index < line_vec.len() - 1
    {
        if line_vec[line_index + 1][char_index + 1] == 'S'
            && line_vec[line_index - 1][char_index - 1] == 'M'
        {
            if line_vec[line_index - 1][char_index + 1] == 'S'
                && line_vec[line_index + 1][char_index - 1] == 'M'
            {
                xmas_count += 1;
                // M.S
                // .A.
                // M.S
            } else if line_vec[line_index - 1][char_index + 1] == 'M'
                && line_vec[line_index + 1][char_index - 1] == 'S'
            {
                xmas_count += 1;
                // M.M
                // .A.
                // S.S
            }
        } else if line_vec[line_index + 1][char_index + 1] == 'M'
            && line_vec[line_index - 1][char_index - 1] == 'S'
        {
            if line_vec[line_index - 1][char_index + 1] == 'M'
                && line_vec[line_index + 1][char_index - 1] == 'S'
            {
                xmas_count += 1;
                // S.M
                // .A.
                // S.M
            } else if line_vec[line_index - 1][char_index + 1] == 'S'
                && line_vec[line_index + 1][char_index - 1] == 'M'
            {
                xmas_count += 1;
                // S.S
                // .A.
                // M.M
            }
        } else if line_vec[line_index + 1][char_index + 1] == 'M'
            && line_vec[line_index - 1][char_index - 1] == 'M'
        {
            if line_vec[line_index - 1][char_index + 1] == 'S'
                && line_vec[line_index + 1][char_index - 1] == 'S'
            {
                xmas_count += 1;
                // M.S
                // .A.
                // S.M
            }
        } else if line_vec[line_index + 1][char_index + 1] == 'S' && line_vec[line_index - 1][char_index - 1] == 'S' && line_vec[line_index - 1][char_index + 1] == 'M' && line_vec[line_index + 1][char_index - 1] == 'M' {
            xmas_count += 1;
            // S.M
            // .A.
            // M.S
        }
    }
    xmas_count
}

fn get_lines_from_file(file: &File) -> Vec<Vec<char>> {
    let reader = io::BufReader::new(file);
    let mut lines_vec: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Cannot get line");
        let char_vec = line.chars().collect();
        lines_vec.push(char_vec);
    }
    lines_vec
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_first_star_sample() {
        let input = "MMMSXXMASM
                        MSAMXMSMSA
                        AMXSXMAAMM
                        MSAMASMSMX
                        XMASAMXAMM
                        XXAMMXXAMA
                        SMSMSASXSS
                        SAXAMASAAA
                        MAMMMXMMMM
                        MXMXAXMASX";

        let input: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect();
        let answer = super::get_xmas_count(&input);
        assert_eq!(answer, 18);
    }

    #[test]
    fn test_second_star_sample() {
        let input = ".M.S......
                            ..A..MSMS.
                            .M.S.MAA..
                            ..A.ASMSM.
                            .M.S.M....
                            ..........
                            S.S.S.S.S.
                            .A.A.A.A..
                            M.M.M.M.M.
                            ..........";

        let input: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect();
        let answer = super::get_xmas_x_count(&input);
        assert_eq!(answer, 9);
    }
}
