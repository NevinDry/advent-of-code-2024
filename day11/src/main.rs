use std::collections::HashMap;

fn main() {
    let input = "4610211 4 0 59 3907 201586 929 33750"
        .split_whitespace()
        .map(|digit| digit.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();
    let answer = blinks(&input, 25);
    println!("First star answer {:?}", answer);

    let answer: u128 = blinks(&input, 75);
    println!("Second star answer {:?}", answer);
}

fn blinks(input: &Vec<u128>, times: u32) -> u128 {
    let mut cache: HashMap<(u128, u32), u128> = HashMap::new();
    input
        .iter()
        .map(|stone| blink(*stone, times, &mut cache))
        .sum()
}

fn blink(stone: u128, index: u32, cache: &mut HashMap<(u128, u32), u128>) -> u128 {
    if cache.contains_key(&(stone, index)) {
        return cache[&(stone, index)];
    }
    let mut value = 1;
    if index == 0 {
        return value;
    } else if stone == 0 {
        value = blink(1, index - 1, cache);
    } else {
        let len = (stone as f64).log10().floor() as u32 + 1;
        if len % 2 == 0 {
            let puis = 10_u128.pow(len / 2);
            value = blink(stone / puis, index - 1, cache);
            value += blink(stone % puis, index - 1, cache);
        } else {
            value = blink(stone * 2024, index - 1, cache);
        }
    }
    cache.insert((stone, index), value);

    value
}

#[cfg(test)]
mod tests {
    

    #[test]
    fn test_basic_blinks() {
        let input = "125 17"
            .split_whitespace()
            .map(|digit| digit.parse::<u128>().unwrap())
            .collect::<Vec<u128>>();
        let answer = super::blinks(&input, 25);
        assert_eq!(answer, 55312);
    }

    #[test]
    fn test_complex_blinks() {
        let input = "125 17"
            .split_whitespace()
            .map(|digit| digit.parse::<u128>().unwrap())
            .collect::<Vec<u128>>();
        let answer = super::blinks(&input, 25);
        assert_eq!(answer, 55312);
    }
}
