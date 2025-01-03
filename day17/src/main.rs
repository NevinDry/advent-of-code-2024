use std::fs::File;
use std::io::{self, BufRead};

// Puzzle at : https://adventofcode.com/2024/day/17

enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

enum OperandType {
    Literal,
    Combo,
}

fn get_opcode_operand_from_usize(opcode: usize) -> (Opcode, OperandType) {
    match opcode {
        0 => (Opcode::Adv, OperandType::Combo),
        1 => (Opcode::Bxl, OperandType::Literal),
        2 => (Opcode::Bst, OperandType::Combo),
        3 => (Opcode::Jnz, OperandType::Combo),
        4 => (Opcode::Bxc, OperandType::Literal),
        5 => (Opcode::Out, OperandType::Combo),
        6 => (Opcode::Bdv, OperandType::Combo),
        7 => (Opcode::Cdv, OperandType::Combo),
        _ => panic!("Invalid opcode"),
    }
}

fn main() {
    let path = "./src/data.txt";
    let file = File::open(path).expect("Error opening file");
    let (register_a, register_b, register_c, program) = get_input_from_file(&file);

    // first star
    let answer = compute(register_a, register_b, register_c, &program);
    println!("First Star Answer {:?}", answer);

    // Second star
    let answer = find_compute(register_b, register_c, program);
    println!("Second Star Answer {:?}", answer);
}

fn find_compute(register_b: i64, register_c: i64, program: Vec<usize>) -> i64 {
    let target_output: Vec<String> = program.iter().map(|x| x.to_string()).collect();
    let mut coefficients = vec![0; program.len()];

    loop {
        let register_a = coefficients
            .iter()
            .enumerate()
            .fold(0, |acc: i64, (i, &factor)| {
                acc + 8i64.pow(i as u32) * factor
            });

        let compute_result = compute(register_a, register_b, register_c, &program);
        let output: Vec<&str> = compute_result.split(',').collect();
        if output == target_output {
            return register_a;
        }
        let mut incremented = false;
        for i in (0..coefficients.len()).rev() {
            if i >= output.len() || output[i] != target_output[i] {
                coefficients[i] += 1;
                incremented = true;
                break;
            }
        }
        if !incremented {
            break;
        }
    }

    0
}

fn compute(
    mut register_a: i64,
    mut register_b: i64,
    mut register_c: i64,
    program: &[usize],
) -> String {
    let mut index = 0;
    let mut output = vec![];
    while let Some(digit) = program.get(index) {
        let opcode_operande_type = get_opcode_operand_from_usize(*digit);
        let operand: usize = match opcode_operande_type.1 {
            OperandType::Literal => program[index + 1],
            OperandType::Combo => match program[index + 1] {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => register_a,
                5 => register_b,
                6 => register_c,
                _ => panic!("Invalid operand"),
            }
            .try_into()
            .unwrap(),
        };

        match opcode_operande_type.0 {
            Opcode::Adv => register_a /= 2usize.pow(operand as u32) as i64,
            Opcode::Bxl => register_b ^= operand as i64,
            Opcode::Bst => register_b = operand as i64 % 8,
            Opcode::Jnz => {
                if register_a != 0 {
                    index = operand;
                    continue;
                }
            }
            Opcode::Bxc => register_b ^= register_c,
            Opcode::Out => output.push(operand % 8),
            Opcode::Bdv => register_b = register_a / 2usize.pow(operand as u32) as i64,
            Opcode::Cdv => register_c = register_a / 2usize.pow(operand as u32) as i64,
        }

        index += 2;
    }

    output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn get_input_from_file(file: &File) -> (i64, i64, i64, Vec<usize>) {
    let mut register_a = 0;
    let mut register_b = 0;
    let mut register_c = 0;
    let mut program = vec![];

    let reader: io::BufReader<&File> = io::BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        if index == 0 {
            let line_content = line.unwrap();
            let register = line_content.split(":").collect::<Vec<&str>>()[1].trim();
            register_a = register.parse().unwrap();
        } else if index == 1 {
            let line_content = line.unwrap();
            let register = line_content.split(":").collect::<Vec<&str>>()[1].trim();
            register_b = register.parse().unwrap();
        } else if index == 2 {
            let line_content = line.unwrap();

            let register = line_content.split(":").collect::<Vec<&str>>()[1].trim();
            register_c = register.parse().unwrap();
        } else if index == 4 {
            program = line.unwrap().split(":").collect::<Vec<&str>>()[1]
                .trim()
                .split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<usize>>();
        }
    }
    (register_a, register_b, register_c, program)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program() {
        let register_a = 729;
        let register_b = 0;
        let register_c = 0;
        let program = vec![0, 1, 5, 4, 3, 0];

        let answer = compute(register_a, register_b, register_c, &program);
        assert_eq!(answer, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_program2() {
        let register_a = 117440;
        let register_b = 0;
        let register_c = 0;
        let program = vec![0, 3, 5, 4, 3, 0];

        let answer = compute(register_a, register_b, register_c, &program);
        assert_eq!(answer, "0,3,5,4,3,0");
    }

    #[test]
    fn test_find() {
        let register_a = 2024;
        let register_b = 0;
        let register_c = 0;
        let program = vec![0, 3, 5, 4, 3, 0];

        let answer = find_compute(register_b, register_c, program);
        assert_eq!(answer, 117440);
    }
}
