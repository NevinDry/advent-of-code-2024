use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "./src/data.txt";
    let file = File::open(path).expect("Error opening file");
    let input: Vec<char> = get_input_char_from_file(&file);

    // first star
    let first_input = unravel_disk_map(&input);
    let moved_blocks = move_blocks(first_input);
    let answer = calculate_checksum(moved_blocks);
    println!("First star answer: {}", answer);

    // second star
    let second_input = unravel_disk_map(&input);
    let moved_blocks_compacted: Vec<String> = move_blocks_compacted(second_input);
    let answer = calculate_checksum(moved_blocks_compacted);

    println!("Second star answer: {}", answer);
}

fn get_input_char_from_file(file: &File) -> Vec<char> {
    let reader: BufReader<&File> = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.expect("Error reading line"))
        .collect::<Vec<String>>()[0]
        .chars()
        .collect()
}

fn move_blocks(mut input: Vec<String>) -> Vec<String> {
    for i in 0..input.len() {
        if input[i] == "." && input[i..].iter().any(|x| x != ".") {
            let index_to_replace: usize = input.iter().rposition(|c| c != ".").unwrap();
            input[i] = input[index_to_replace].to_string();
            input[index_to_replace] = ".".to_string();
        }
    }

    input
}

fn move_blocks_compacted(mut input: Vec<String>) -> Vec<String> {
    let files_vec = input
        .iter()
        .enumerate()
        .filter_map(|(i, c)| {
            if c.len() > 1 {
                Some((c.to_string(), i))
            } else if c.chars().all(|ch| ch.is_ascii_digit()) {
                Some((c.chars().next().unwrap().to_string(), i))
            } else {
                None
            }
        })
        .fold(Vec::new(), |mut acc, (file_id, index)| {
            if let Some((last_file_id, count, _)) = acc.last_mut() {
                if last_file_id == &file_id {
                    *count += 1;
                    return acc;
                }
            }
            acc.push((file_id, 1, index));
            acc
        });

    let mut dots_vec = Vec::new();
    let mut i = 0;
    while i < input.len() {
        if input[i] == "." {
            let index = i;
            let mut count = 0;
            while i < input.len() && input[i] == "." {
                count += 1;
                i += 1;
            }
            dots_vec.push((index, count));
        } else {
            i += 1;
        }
    }

    for file in files_vec.iter().rev() {
        let first_match_index = dots_vec
            .iter()
            .position(|dots| file.1 <= dots.1 && file.2 > dots.0);

        match first_match_index {
            Some(index) => {
                let first_match: (usize, usize) = dots_vec[index];
                for ia in input.iter_mut().skip(first_match.0).take(file.1) {
                    *ia = file.0.to_string();
                }
                for ib in input.iter_mut().skip(file.2).take(file.1) {
                    *ib = ".".to_string();
                }
                dots_vec.remove(index);
                if first_match.1 > file.1 {
                    dots_vec.insert(index, (first_match.0 + file.1, first_match.1 - file.1));
                }
            }
            None => {
                continue;
            }
        }
    }
    input
}

fn unravel_disk_map(input: &Vec<char>) -> Vec<String> {
    let mut mapped_disk = Vec::new();
    let mut is_file = true;
    let mut file_id = 0;
    for c in input {
        let c_as_digit = c.to_digit(10).unwrap();
        if is_file {
            for _ in 0..c_as_digit {
                mapped_disk.push(file_id.to_string());
            }
            is_file = false;
            file_id += 1;
        } else {
            if c_as_digit != 0 {
                for _ in 0..c_as_digit {
                    mapped_disk.push(".".to_string());
                }
            }

            is_file = true;
        }
    }
    mapped_disk
}

fn calculate_checksum(input: Vec<String>) -> i128 {
    input
        .iter()
        .enumerate()
        .filter_map(|(i, value)| {
            if value != "." {
                Some(i as i128 * value.parse::<i128>().unwrap())
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_first_star_disk_map_sample() {
        let input = vec!['1', '2', '3', '4', '5'];
        let input = super::unravel_disk_map(&input);
        assert_eq!(
            input,
            vec!["0", ".", ".", "1", "1", "1", ".", ".", ".", ".", "2", "2", "2", "2", "2"]
        );
        let moved_blocks = super::move_blocks(input);
        assert_eq!(
            moved_blocks,
            vec!["0", "2", "2", "1", "1", "1", "2", "2", "2", ".", ".", ".", ".", ".", "."]
        );
    }

    #[test]
    fn test_first_star_complex_disk_map_sample() {
        let input = vec![
            '2', '3', '3', '3', '1', '3', '3', '1', '2', '1', '4', '1', '4', '1', '3', '1', '4',
            '0', '2',
        ];
        let input = super::unravel_disk_map(&input);
        assert_eq!(
            input,
            vec![
                "0", "0", ".", ".", ".", "1", "1", "1", ".", ".", ".", "2", ".", ".", ".", "3",
                "3", "3", ".", "4", "4", ".", "5", "5", "5", "5", ".", "6", "6", "6", "6", ".",
                "7", "7", "7", ".", "8", "8", "8", "8", "9", "9"
            ]
        );
        let moved_blocks = super::move_blocks(input);
        assert_eq!(
            moved_blocks,
            vec![
                "0", "0", "9", "9", "8", "1", "1", "1", "8", "8", "8", "2", "7", "7", "7", "3",
                "3", "3", "6", "4", "4", "6", "5", "5", "5", "5", "6", "6", ".", ".", ".", ".",
                ".", ".", ".", ".", ".", ".", ".", ".", ".", "."
            ]
        );
        let answer = super::calculate_checksum(moved_blocks);
        assert_eq!(answer, 1928);
    }

    #[test]
    fn test_second_star_checksum_sample() {
        let moved_blocks_compacted: Vec<String> = vec![
            "0".to_string(),
            "0".to_string(),
            "9".to_string(),
            "9".to_string(),
            "2".to_string(),
            "1".to_string(),
            "1".to_string(),
            "1".to_string(),
            "7".to_string(),
            "7".to_string(),
            "7".to_string(),
            ".".to_string(),
            "4".to_string(),
            "4".to_string(),
            ".".to_string(),
            "3".to_string(),
            "3".to_string(),
            "3".to_string(),
            ".".to_string(),
            ".".to_string(),
            ".".to_string(),
            ".".to_string(),
            "5".to_string(),
            "5".to_string(),
            "5".to_string(),
            "5".to_string(),
            ".".to_string(),
            "6".to_string(),
            "6".to_string(),
            "6".to_string(),
            "6".to_string(),
            ".".to_string(),
            ".".to_string(),
            ".".to_string(),
            ".".to_string(),
            ".".to_string(),
            "8".to_string(),
            "8".to_string(),
            "8".to_string(),
            "8".to_string(),
            ".".to_string(),
            ".".to_string(),
        ];
        let answer = super::calculate_checksum(moved_blocks_compacted);
        assert_eq!(answer, 2858);
    }

    #[test]
    fn test_second_star_complex_disk_map_sample() {
        let input = vec![
            '2', '3', '3', '3', '1', '3', '3', '1', '2', '1', '4', '1', '4', '1', '3', '1', '4',
            '0', '2', '1', '2', '3', '2',
        ];
        let input = super::unravel_disk_map(&input);
        assert_eq!(
            input,
            vec![
                "0", "0", ".", ".", ".", "1", "1", "1", ".", ".", ".", "2", ".", ".", ".", "3",
                "3", "3", ".", "4", "4", ".", "5", "5", "5", "5", ".", "6", "6", "6", "6", ".",
                "7", "7", "7", ".", "8", "8", "8", "8", "9", "9", ".", "10", "10", ".", ".", ".",
                "11", "11"
            ]
        );
        let moved_blocks_compacted = super::move_blocks_compacted(input);
        assert_eq!(
            moved_blocks_compacted,
            vec![
                "0", "0", "11", "11", "2", "1", "1", "1", "10", "10", ".", ".", "9", "9", ".", "3",
                "3", "3", ".", "4", "4", ".", "5", "5", "5", "5", ".", "6", "6", "6", "6", ".",
                "7", "7", "7", ".", "8", "8", "8", "8", ".", ".", ".", ".", ".", ".", ".", ".",
                ".", "."
            ]
        );
        let answer = super::calculate_checksum(moved_blocks_compacted);
        assert_eq!(answer, 3823);
    }
}
