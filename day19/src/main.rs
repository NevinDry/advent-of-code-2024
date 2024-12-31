use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let path = "./src/data.txt";
    let file = File::open(path).expect("Error opening file");
    let (patterns, available_colors) = get_input_from_file(&file);
    // first star
    let mut answer = 0;
    let cache: &mut HashMap<String, bool> = &mut HashMap::new();
    for pattern in patterns.clone() {
        let mut available_colors_cloned = available_colors.clone();
        let possible = can_form_pattern(pattern, &mut available_colors_cloned, cache);
        if possible {
            answer += 1;
        }
    }
    println!("Answer 1: {}", answer);

    // second star
    let mut answer = 0;
    let cache: &mut HashMap<String, usize> = &mut HashMap::new();
    for pattern in patterns {
        let mut available_colors_cloned = available_colors.clone();
        let possibilities = pattern_possibilities(pattern, &mut available_colors_cloned, cache);
        answer += possibilities;
    }

    println!("Answer 2: {}", answer);
}

fn can_form_pattern(
    pattern: String,
    available_colors: &mut Vec<String>,
    cache: &mut HashMap<String, bool>,
) -> bool {
    if pattern.is_empty() {
        return true;
    }
    if let Some(&cached_result) = cache.get(&pattern) {
        return cached_result;
    }
    for index in 0..available_colors.len() {
        let color = &available_colors[index];
        if pattern.starts_with(color) && can_form_pattern(
                pattern.strip_prefix(color).unwrap().to_string(),
                available_colors,
                cache,
            ) {
            cache.insert(pattern.to_string(), true);
            return true;
        }
    }
    cache.insert(pattern.to_string(), false);
    false
}

fn pattern_possibilities(
    pattern: String,
    available_colors: &mut Vec<String>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if pattern.is_empty() {
        return 1;
    }
    if let Some(&cached_result) = cache.get(&pattern) {
        return cached_result;
    }
    let mut count = 0;
    for index in 0..available_colors.len() {
        let color = &available_colors[index];
        if pattern.starts_with(color) {
            count += pattern_possibilities(
                pattern.strip_prefix(color).unwrap().to_string(),
                available_colors,
                cache,
            );
        }
    }
    cache.insert(pattern.to_string(), count);
    count
}

fn get_input_from_file(file: &File) -> (Vec<String>, Vec<String>) {
    let mut patterns = vec![];
    let mut colors = vec![];
    let reader: io::BufReader<&File> = io::BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if index == 0 {
            let line_splitted: Vec<&str> = line.split(",").collect();
            line_splitted
                .iter()
                .for_each(|x| colors.push(x.trim().to_string()));
        } else if index > 1 {
            let pattern = line.trim();
            patterns.push(pattern.to_string());
        }
    }
    (patterns, colors)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patterns() {
        let patterns = vec![
            "brwrr".to_string(),
            "bggr".to_string(),
            "gbbr".to_string(),
            "rrbgbr".to_string(),
            "ubwu".to_string(),
            "bwurrg".to_string(),
            "brgr".to_string(),
            "bbrgwb".to_string(),
        ];

        let available_colors = vec![
            "r".to_string(),
            "wr".to_string(),
            "b".to_string(),
            "g".to_string(),
            "bwu".to_string(),
            "rb".to_string(),
            "gb".to_string(),
            "br".to_string(),
        ];

        let mut result = 0;
        let cache: &mut HashMap<String, bool> = &mut HashMap::new();
        for pattern in patterns {
            let mut available_colors_cloned = available_colors.clone();
            let possible = can_form_pattern(pattern, &mut available_colors_cloned, cache);
            if possible {
                result += 1;
            }
        }

        assert_eq!(result, 6);
    }
}
