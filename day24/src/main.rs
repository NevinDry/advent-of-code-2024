use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Gate {
    input1: String,
    input2: String,
    operation: Operation,
    output: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Operation {
    And,
    Or,
    Xor,
}

fn main() {
    let path = "./src/data.txt";
    let file = File::open(path).expect("Error opening file");

    let (values, gates) = get_input_from_file(&file);

    // first star
    let answer = open_the_gates(values, gates.clone());
    println!("First star answer : {:?}", answer);

    // second star
    let answer: String = find_swapped_gates(&gates, "z45".to_string());
    println!("Second star answer : {:?}", answer);
}

fn find_swapped_gates(gates: &[Gate], max_gate: String) -> String {
    let mut swapped = HashSet::new();
    let mut existing_operations = HashSet::new();

    for gate in gates {
        existing_operations.insert((gate.input1.clone(), gate.operation.clone()));
        existing_operations.insert((gate.input2.clone(), gate.operation.clone()));
    }

    for gate in gates {
        match gate.operation {
            Operation::And => {
                if gate.input1 != "x00"
                    && gate.input2 != "x00"
                    && !existing_operations.contains(&(gate.output.clone(), Operation::Or))
                {
                    swapped.insert(gate.output.clone());
                }
            }
            Operation::Or => {
                if gate.output.starts_with('z') && gate.output != max_gate {
                    swapped.insert(gate.output.clone());
                }
                if existing_operations.contains(&(gate.output.clone(), Operation::Or)) {
                    swapped.insert(gate.output.clone());
                }
            }
            Operation::Xor => {
                if gate.input1.starts_with('x') || gate.input2.starts_with('x') {
                    if gate.input1 != "x00"
                        && gate.input2 != "x00"
                        && !existing_operations.contains(&(gate.output.clone(), Operation::Xor))
                    {
                        swapped.insert(gate.output.clone());
                    }
                } else {
                    if !gate.output.starts_with('z') {
                        swapped.insert(gate.output.clone());
                    }
                }
            }
        }
    }
    let mut result: Vec<String> = swapped.into_iter().collect();
    result.sort();
    result.join(",")
}

fn open_the_gates(values: HashMap<String, usize>, gates: Vec<Gate>) -> usize {
    let mut values = values;
    let mut gates = gates;
    let mut index = 0;
    loop {
        if gates.is_empty() {
            break;
        }
        let gate = &gates[index];
        if values.contains_key(&gate.input1) && values.contains_key(&gate.input2) {
            let input1 = values.get(&gate.input1).unwrap();
            let input2 = values.get(&gate.input2).unwrap();
            let result = match gate.operation {
                Operation::And => and_operation(*input1, *input2),
                Operation::Or => or_operation(*input1, *input2),
                Operation::Xor => xor_operation(*input1, *input2),
            };
            values.insert(gate.output.clone(), result);
            gates.remove(index);
            index = 0;
        } else {
            index += 1;
        }
    }

    let mut z: Vec<_> = values
        .iter()
        .filter(|(key, _)| key.starts_with("z"))
        .map(|(key, value)| (key, value.to_string()))
        .collect();

    z.sort_by_key(|(key, _)| std::cmp::Reverse((*key).clone()));

    let concat_num: String = z.iter().map(|(_, value)| value.to_string()).collect();
    println!("Concatenated values: {}", concat_num);
    usize::from_str_radix(&concat_num, 2).unwrap()
}

fn and_operation(input1: usize, input2: usize) -> usize {
    if input1 == 1 && input2 == 1 {
        1
    } else {
        0
    }
}

fn or_operation(input1: usize, input2: usize) -> usize {
    if input1 == 1 || input2 == 1 {
        1
    } else {
        0
    }
}

fn xor_operation(input1: usize, input2: usize) -> usize {
    if input1 != input2 {
        1
    } else {
        0
    }
}

fn get_input_from_file(file: &File) -> (HashMap<String, usize>, Vec<Gate>) {
    let mut values: HashMap<String, usize> = HashMap::new();
    let mut gates: Vec<Gate> = Vec::new();

    let reader: io::BufReader<&File> = io::BufReader::new(file);
    let mut first_part = true;
    for line in reader.lines() {
        let line = line.expect("Cannot get line");
        if line.is_empty() {
            first_part = false;
            continue;
        }
        if first_part {
            let line = line.trim().split(": ").collect::<Vec<&str>>();
            values.insert(line[0].to_string(), line[1].parse::<usize>().unwrap());
        } else {
            let line = line.replace("-> ", "");
            let line = line.trim().split(" ").collect::<Vec<&str>>();
            let gate = Gate {
                input1: line[0].to_string(),
                input2: line[2].to_string(),
                operation: match line[1] {
                    "AND" => Operation::And,
                    "OR" => Operation::Or,
                    "XOR" => Operation::Xor,
                    _ => panic!("Unknown operation"),
                },
                output: line[3].to_string(),
            };
            gates.push(gate);
        }
    }
    (values, gates)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_operations() {
        assert_eq!(and_operation(1, 1), 1);
        assert_eq!(and_operation(1, 0), 0);
        assert_eq!(and_operation(0, 1), 0);
        assert_eq!(and_operation(0, 0), 0);

        assert_eq!(or_operation(1, 1), 1);
        assert_eq!(or_operation(1, 0), 1);
        assert_eq!(or_operation(0, 1), 1);
        assert_eq!(or_operation(0, 0), 0);

        assert_eq!(xor_operation(1, 1), 0);
        assert_eq!(xor_operation(1, 0), 1);
        assert_eq!(xor_operation(0, 1), 1);
        assert_eq!(xor_operation(0, 0), 0);
    }

    #[test]
    fn test_simple_sample() {
        let values = HashMap::from([
            ("x00".to_string(), 1),
            ("x01".to_string(), 1),
            ("x02".to_string(), 1),
            ("y00".to_string(), 0),
            ("y01".to_string(), 1),
            ("y02".to_string(), 0),
        ]);

        let gates = vec![
            Gate {
                input1: "x00".to_string(),
                input2: "y00".to_string(),
                operation: Operation::And,
                output: "z00".to_string(),
            },
            Gate {
                input1: "x01".to_string(),
                input2: "y01".to_string(),
                operation: Operation::Xor,
                output: "z01".to_string(),
            },
            Gate {
                input1: "x02".to_string(),
                input2: "y02".to_string(),
                operation: Operation::Or,
                output: "z02".to_string(),
            },
        ];

        let result = open_the_gates(values, gates);

        println!("{:?}", result);

        assert_eq!(result, 4);
    }

    #[test]
    fn test_complex_sample() {
        let path = "./src/data_sample.txt";
        let file = File::open(path).expect("Error opening file");

        let (values, gates) = get_input_from_file(&file);

        let result = open_the_gates(values, gates);

        println!("{:?}", result);

        assert_eq!(result, 2024);
    }
}
