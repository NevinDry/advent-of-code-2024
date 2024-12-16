use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

fn main() {
    let path = "./src/data.txt";
    let file = File::open(path).expect("Error opening file");
    let mut input = get_input_from_file(&file);

    // first star
    let region = get_regions(&mut input, false);
    println!("First star: {}", region);

    let region = get_regions(&mut input, true);
    println!("Second star: {}", region);
}

fn get_regions(input: &mut Vec<Vec<char>>, is_discounted: bool) -> i32 {
    let mut regions = vec![];
    let mut plants: HashMap<(usize, usize), char> = HashMap::new();
    for y in 0..input.len() {
        for i in 0..input[y].len() {
            if plants.contains_key(&(y, i)) {
                continue;
            } else {
                let region = get_region(input, y, i, (0, 0, vec![]), &mut plants);
                regions.push(region.clone());
            }
        }
    }
    if is_discounted {
        for region in &mut regions {
            let mut fences: Vec<((i32, i32), Direction)> = vec![];
            for plant in region.2.iter() {
                if !region.2.contains(&(plant.0.saturating_sub(1), plant.1)) {
                    let mut has_fence = false;

                    if plant.1 > 0 {
                        let mut x = (0..=plant.1 as i32 - 1).rev();
                        while let Some(x) = x.next() {
                            if !region.2.contains(&(plant.0, x))
                                || (region.2.contains(&(plant.0, x))
                                    && region.2.contains(&(plant.0.saturating_sub(1), x)))
                            {
                                break;
                            } else if fences.contains(&((plant.0, x), Direction::Up)) {
                                has_fence = true;
                                break;
                            }
                        }
                    }

                    let mut x = (plant.1 + 1)..(input.len() as i32);
                    while let Some(x) = x.next() {
                        if !region.2.contains(&(plant.0, x))
                            || (region.2.contains(&(plant.0, x))
                                && region.2.contains(&(plant.0.saturating_sub(1), x)))
                        {
                            break;
                        } else if fences.contains(&((plant.0, x), Direction::Up)) {
                            has_fence = true;
                            break;
                        }
                    }

                    if !has_fence {
                        fences.push((*plant, Direction::Up));
                    }
                }

                if !region.2.contains(&(plant.0 + 1, plant.1)) {
                    let mut has_fence = false;

                    if plant.1 > 0 {
                        let mut x = (0..=plant.1 as i32 - 1).rev();
                        while let Some(x) = x.next() {
                            if !region.2.contains(&(plant.0, x))
                                || (region.2.contains(&(plant.0, x))
                                    && region.2.contains(&(plant.0.saturating_add(1), x)))
                            {
                                break;
                            } else if fences.contains(&((plant.0, x), Direction::Down)) {
                                has_fence = true;
                                break;
                            }
                        }
                    }

                    let mut x = (plant.1 + 1)..(input.len() as i32);
                    while let Some(x) = x.next() {
                        if !region.2.contains(&(plant.0, x))
                            || (region.2.contains(&(plant.0, x))
                                && region.2.contains(&(plant.0.saturating_add(1), x)))
                        {
                            break;
                        } else if fences.contains(&((plant.0, x), Direction::Down)) {
                            has_fence = true;
                            break;
                        }
                    }

                    if !has_fence {
                        fences.push((*plant, Direction::Down));
                    }
                }

                if !region.2.contains(&(plant.0, plant.1.saturating_sub(1))) {
                    let mut has_fence = false;

                    if plant.0 > 0 {
                        let mut x = (0..=plant.0 - 1).rev();
                        while let Some(x) = x.next() {
                            if !region.2.contains(&(x, plant.1))
                                || (region.2.contains(&(x, plant.1))
                                    && region.2.contains(&(x, plant.1.saturating_sub(1))))
                            {
                                break;
                            } else if fences.contains(&((x, plant.1), Direction::Left)) {
                                has_fence = true;
                                break;
                            }
                        }
                    }

                    let mut x = (plant.0 + 1)..(input.len() as i32);
                    while let Some(x) = x.next() {
                        if !region.2.contains(&(x, plant.1))
                            || (region.2.contains(&(x, plant.1))
                                && region.2.contains(&(x, plant.1.saturating_sub(1))))
                        {
                            break;
                        } else if fences.contains(&((x, plant.1), Direction::Left)) {
                            has_fence = true;
                            break;
                        }
                    }

                    if !has_fence {
                        fences.push((*plant, Direction::Left));
                    }
                }

                if !region.2.contains(&(plant.0, plant.1 + 1)) {
                    let mut has_fence = false;

                    if plant.0 > 0 {
                        let mut x = (0..=plant.0 - 1).rev();
                        while let Some(x) = x.next() {
                            if !region.2.contains(&(x, plant.1))
                                || (region.2.contains(&(x, plant.1))
                                    && region.2.contains(&(x, plant.1.saturating_add(1))))
                            {
                                break;
                            } else if fences.contains(&((x, plant.1), Direction::Right)) {
                                has_fence = true;
                                break;
                            }
                        }
                    }

                    let mut x = (plant.0 + 1)..(input.len() as i32);
                    while let Some(x) = x.next() {
                        if !region.2.contains(&(x, plant.1))
                            || (region.2.contains(&(x, plant.1))
                                && region.2.contains(&(x, plant.1.saturating_add(1))))
                        {
                            break;
                        } else if fences.contains(&((x, plant.1), Direction::Right)) {
                            has_fence = true;
                            break;
                        }
                    }

                    if !has_fence {
                        fences.push((*plant, Direction::Right));
                    }
                }
            }

            region.1 = fences.len() as i32;
        }
    }

    regions.iter().map(|region| region.0 * region.1).sum()
}

fn get_region(
    input: &mut Vec<Vec<char>>,
    y: usize,
    x: usize,
    mut region: (i32, i32, Vec<(i32, i32)>),
    plants: &mut HashMap<(usize, usize), char>,
) -> (i32, i32, Vec<(i32, i32)>) {
    let current: char = input[y][x];
    if plants.contains_key(&(y, x)) {
        return region;
    } else {
        plants.insert((y, x), current);
        region.2.push((y as i32, x as i32));
        region.0 += 1;
    }
    let mut fence_count = 4;
    if x < input[y].len() - 1 {
        if let Some(next_right) = input[y].get(x + 1) {
            if *next_right == current {
                fence_count -= 1;
                region = get_region(input, y, x + 1, region, plants);
            }
        }
    }
    if y < input.len() - 1 {
        if let Some(next_down) = input[y + 1].get(x) {
            if *next_down == current {
                fence_count -= 1;
                region = get_region(input, y + 1, x, region, plants);
            }
        }
    }

    if x > 0 {
        if let Some(next_left) = input[y].get(x - 1) {
            if *next_left == current {
                fence_count -= 1;
                region = get_region(input, y, x - 1, region, plants);
            }
        }
    }
    if y > 0 {
        if let Some(row_above) = input.get(y - 1) {
            if let Some(next_up) = row_above.get(x) {
                if *next_up == current {
                    fence_count -= 1;
                    region = get_region(input, y - 1, x, region, plants);
                }
            }
        }
    }
    region.1 = region.1 + fence_count;
    return region;
}

fn get_input_from_file(file: &File) -> Vec<Vec<char>> {
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect::<Vec<Vec<char>>>()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_basic() {
        let mut input = vec![vec!['A']];

        let regions = super::get_regions(&mut input, false);
        assert_eq!(regions, 4);
    }

    #[test]
    fn test_line_right() {
        let mut input = vec![vec!['A', 'A', 'A', 'A']];

        let regions = super::get_regions(&mut input, false);
        assert_eq!(regions, 40);
    }

    #[test]
    fn test_line_down() {
        let mut input = vec![vec!['A'], vec!['A'], vec!['A'], vec!['A']];

        let regions = super::get_regions(&mut input, false);
        assert_eq!(regions, 40);
    }

    #[test]
    fn test_rect() {
        let mut input = vec![vec!['A', 'A'], vec!['A', 'A']];

        let regions = super::get_regions(&mut input, false);
        assert_eq!(regions, 32);
    }

    #[test]
    fn test_complex() {
        let mut input = vec![
            vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'F', 'F'],
            vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'C', 'F'],
            vec!['V', 'V', 'R', 'R', 'R', 'C', 'C', 'F', 'F', 'F'],
            vec!['V', 'V', 'R', 'C', 'C', 'C', 'J', 'F', 'F', 'F'],
            vec!['V', 'V', 'V', 'V', 'C', 'J', 'J', 'C', 'F', 'E'],
            vec!['V', 'V', 'I', 'V', 'C', 'C', 'J', 'J', 'E', 'E'],
            vec!['V', 'V', 'I', 'I', 'I', 'C', 'J', 'J', 'E', 'E'],
            vec!['M', 'I', 'I', 'I', 'I', 'I', 'J', 'J', 'E', 'E'],
            vec!['M', 'I', 'I', 'I', 'S', 'I', 'J', 'E', 'E', 'E'],
            vec!['M', 'M', 'M', 'I', 'S', 'S', 'J', 'E', 'E', 'E'],
        ];

        let regions = super::get_regions(&mut input, false);
        assert_eq!(regions, 1930);
    }

    #[test]
    fn test_basic_with_discount() {
        let mut input = vec![vec!['A']];

        let regions = super::get_regions(&mut input, true);
        assert_eq!(regions, 4);
    }

    #[test]
    fn test_line_right_discount() {
        let mut input = vec![vec!['A', 'A', 'A', 'A']];

        let regions = super::get_regions(&mut input, true);
        assert_eq!(regions, 16);
    }

    #[test]
    fn test_line_down_discount() {
        let mut input = vec![vec!['A'], vec!['A'], vec!['A'], vec!['A']];

        let regions = super::get_regions(&mut input, true);
        assert_eq!(regions, 16);
    }

    #[test]
    fn test_rect_discount() {
        let mut input = vec![vec!['A', 'A'], vec!['A', 'A']];

        let regions = super::get_regions(&mut input, true);
        assert_eq!(regions, 16);
    }

    #[test]
    fn test_snake_discount() {
        let mut input = vec![vec!['A', 'B'], vec!['A', 'A'], vec!['B', 'A']];

        let regions = super::get_regions(&mut input, true);
        assert_eq!(regions, 40);
    }

    #[test]
    fn test_complex_edge_easier_with_discount() {
        let mut input = vec![
            vec!['R', 'R', 'R', 'I'],
            vec!['R', 'R', 'R', 'A'],
            vec!['B', 'R', 'R', 'R'],
            vec!['A', 'R', 'C', 'A'],
        ];

        let regions = super::get_regions(&mut input, true);
        assert_eq!(regions, 124);
    }

    #[test]
    fn test_complex_edge_with_discount() {
        let mut input = vec![
            vec!['R', 'R', 'R', 'R', 'I'],
            vec!['R', 'R', 'R', 'R', 'A'],
            vec!['E', 'B', 'R', 'R', 'R'],
            vec!['U', 'A', 'R', 'C', 'A'],
        ];

        let regions = super::get_regions(&mut input, true);
        assert_eq!(regions, 152);
    }

    #[test]
    fn test_complex_c_with_discount() {
        let mut input = vec![
            vec!['a', 'x', 'k', 'C', 'C', 'F'],
            vec!['e', 'l', 'p', 'C', 'C', 'C'],
            vec!['d', 'h', 'C', 'C', 'b', 'g'],
            vec!['C', 'C', 'C', 'v', 'z', 'l'],
            vec!['w', 'C', 'J', 'b', 'x', 'm'],
            vec!['V', 'C', 'C', 'n', 'o', 'p'],
            vec!['b', 'I', 'C', 'y', 'u', 'E'],
        ];

        let regions = super::get_regions(&mut input, true);
        assert_eq!(regions, 420);
    }

    #[test]
    fn test_complex_v_with_discount() {
        let mut input = vec![
            vec!['V', 'V', 'a', 'z'],
            vec!['V', 'V', 'R', 'C'],
            vec!['V', 'V', 'V', 'V'],
            vec!['V', 'V', 'u', 'V'],
            vec!['V', 'V', 'o', 'I'],
        ];
        let regions = super::get_regions(&mut input, true);
        assert_eq!(regions, 158);
    }

    #[test]
    fn test_complex_j_with_discount() {
        let mut input = vec![
            vec!['C', 'J', 'F'],
            vec!['J', 'J', 'C'],
            vec!['r', 'J', 'J'],
            vec!['b', 'J', 'J'],
            vec!['I', 'J', 'J'],
            vec!['q', 'J', 'E'],
            vec!['S', 'J', 'o'],
        ];
        let regions = super::get_regions(&mut input, true);
        assert_eq!(regions, 172);
    }

    #[test]
    fn test_complex_with_discount() {
        let mut input = vec![
            vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'F', 'F'],
            vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'C', 'F'],
            vec!['V', 'V', 'R', 'R', 'R', 'C', 'C', 'F', 'F', 'F'],
            vec!['V', 'V', 'R', 'C', 'C', 'C', 'J', 'F', 'F', 'F'],
            vec!['V', 'V', 'V', 'V', 'C', 'J', 'J', 'C', 'F', 'E'],
            vec!['V', 'V', 'I', 'V', 'C', 'C', 'J', 'J', 'E', 'E'],
            vec!['V', 'V', 'I', 'I', 'I', 'C', 'J', 'J', 'E', 'E'],
            vec!['M', 'I', 'I', 'I', 'I', 'I', 'J', 'J', 'E', 'E'],
            vec!['M', 'I', 'I', 'I', 'S', 'I', 'J', 'E', 'E', 'E'],
            vec!['M', 'M', 'M', 'I', 'S', 'S', 'J', 'E', 'E', 'E'],
        ];

        let regions = super::get_regions(&mut input, true);
        assert_eq!(regions, 1206);
    }
}
