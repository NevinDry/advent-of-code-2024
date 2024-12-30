use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::vec;

fn main() {
    let path = "./src/data.txt";
    let file = File::open(path).expect("Error opening file");

    let buyers = get_input_from_file(&file);

    // first star
    let answer = perform_secret_numbers(buyers, 2000);
    println!("First star answer : {:?}", answer.0);

    // first star
    println!("Second star answer : {:?}", answer.1);
}

fn perform_secret_numbers(input: Vec<i64>, depth: i64) -> (i64, isize) {
    let mut result = 0;
    let mut buyers_deals: Vec<Vec<usize>> = vec![];
    for secret_number in input {
        let buyers_deal = perform_secret_number(secret_number, depth);
        buyers_deals.push(buyers_deal.1);
        result += buyers_deal.0;
    }

    let mut sequences_with_total: HashMap<(isize, isize, isize, isize), isize> = HashMap::new();
    for buyers_deal in buyers_deals {
        let mut price_changes: Vec<isize> = Vec::new();
        let mut sequences: HashSet<(isize, isize, isize, isize)> = HashSet::new();
        for (index, deal) in buyers_deal.iter().enumerate() {
            if index == 0 {
                continue;
            }
            price_changes.push(*deal as isize - buyers_deal[index - 1] as isize);
            if price_changes.len() == 4 {
                let sequence: (isize, isize, isize, isize) = (
                    price_changes[0],
                    price_changes[1],
                    price_changes[2],
                    price_changes[3],
                );
                if !sequences.contains(&sequence) {
                    sequences.insert(sequence);
                    let sequence_value = sequences_with_total.get_mut(&sequence);
                    if let Some(value) = sequence_value {
                        *value += *deal as isize;
                    } else {
                        sequences_with_total.insert(sequence, *deal as isize);
                    }
                }
                price_changes.remove(0);
            }
        }
    }

    (
        result,
        *sequences_with_total
            .iter()
            .max_by(|a, b| a.1.cmp(b.1))
            .unwrap()
            .1,
    )
}

fn perform_secret_number(secret_number: i64, depth: i64) -> (i64, Vec<usize>) {
    let mut result = 0;
    let mut secret_number = secret_number;

    let mut bananas: Vec<usize> = vec![];
    bananas.push((secret_number % 10) as usize);

    for _ in 0..depth {
        result = perform_operations(secret_number);
        bananas.push((result % 10) as usize);
        secret_number = result;
    }
    (result, bananas)
}

fn perform_operations(secret_number: i64) -> i64 {
    let first = first_operation(secret_number);
    let second = second_operation(first);
    let third = third_operation(second);
    third
}

fn first_operation(secret_number: i64) -> i64 {
    prune(mix(secret_number, secret_number * 64))
}

fn second_operation(secret_number: i64) -> i64 {
    prune(mix(secret_number, secret_number / 32))
}

fn third_operation(secret_number: i64) -> i64 {
    prune(mix(secret_number, secret_number * 2048))
}

fn mix(secret_number: i64, value: i64) -> i64 {
    let res = secret_number ^ value;
    res
}

fn prune(secret_number: i64) -> i64 {
    let res = secret_number % 16777216;
    res
}

fn get_input_from_file(file: &File) -> Vec<i64> {
    let mut buyers = vec![];
    let reader: io::BufReader<&File> = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        buyers.push(line.parse::<i64>().unwrap());
    }
    buyers
}

#[cfg(test)]
mod tests {
    use std::vec;

    #[test]
    fn test_mix() {
        let result = super::mix(42, 15);
        assert_eq!(result, 37);
    }

    #[test]
    fn test_prune() {
        let result = super::prune(100000000);
        assert_eq!(result, 16113920);
    }

    #[test]
    fn test_operations() {
        let result = super::perform_operations(123);
        assert_eq!(result, 15887950);
    }

    #[test]
    fn test_a_secret_10() {
        let result = super::perform_secret_number(123, 10);
        assert_eq!(result.0, 5908254);
    }

    #[test]
    fn test_sample() {
        let input = vec![1, 10, 100, 2024];
        let result = super::perform_secret_numbers(input, 2000);
        assert_eq!(result.0, 37327623);
    }

    #[test]
    fn test_deals() {
        let input = vec![1, 2, 3, 2024];
        let result = super::perform_secret_numbers(input, 2000);
        assert_eq!(result.1, 23);
    }
}
