use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    let file_path = &args[1];
    let contents = match read_in_file(&file_path) {
        Ok(contents) => contents,
        Err(error) => {
            return Err(error.to_string());
        }
    };

    let mut keys: HashMap<char, Instruction> = HashMap::new();
    keys.insert('<', Instruction::Left);
    keys.insert('>', Instruction::Right);
    keys.insert('+', Instruction::Plus);
    keys.insert('-', Instruction::Minus);
    keys.insert('[', Instruction::Begin);
    keys.insert(']', Instruction::End);
    keys.insert('.', Instruction::Print);
    keys.insert(',', Instruction::Input);
    let instructions = contents
        .chars()
        .filter_map(|x| keys.get(&x))
        .collect::<Vec<&Instruction>>();

    // println!("{:?}", instructions);

    do_your_thing(instructions)?;

    Ok(())
}

fn do_your_thing(instructions: Vec<&Instruction>) -> Result<(), &'static str> {
    let mut big_array: [u8; 30000] = [0; 30000];
    let mut pointer: usize = 0;
    let mut instruction_number = 0;
    let mut loop_indexes: Vec<usize> = Vec::new();
    while instruction_number < instructions.len() {
        //println!("BEGIN\n{:?}{:?}", &big_array.last(), &big_array[0..70]);
        //dbg!(instructions.get(instruction_number).unwrap());
        //println!("{}, END\n", pointer);
        if instructions.get(instruction_number).unwrap() == &&Instruction::Left {
            if pointer == 0 {
                pointer = 30000 - 1;
            } else {
                pointer -= 1;
            }
            instruction_number += 1;
        } else if instructions.get(instruction_number).unwrap() == &&Instruction::Right {
            if pointer == 30000 - 1 {
                pointer = 0;
            } else {
                pointer += 1;
            }
            instruction_number += 1;
        } else if instructions.get(instruction_number).unwrap() == &&Instruction::Plus {
            if big_array[pointer] == 255 {
                big_array[pointer] = 0;
            } else {
                big_array[pointer] += 1;
            }
            instruction_number += 1;
        } else if instructions.get(instruction_number).unwrap() == &&Instruction::Minus {
            if big_array[pointer] == 0 {
                big_array[pointer] = 255;
            } else {
                big_array[pointer] -= 1;
            }
            instruction_number += 1;
        } else if instructions.get(instruction_number).unwrap() == &&Instruction::Begin {
            if big_array[pointer] != 0 {
                loop_indexes.push(instruction_number as usize);
                instruction_number += 1;
            } else {
                let mut loop_number = instruction_number.clone() + 1;
                let mut offset = 0;
                while loop_number < instructions.len() {
                    if instructions.get(loop_number).unwrap() == &&Instruction::End && offset == 0 {
                        instruction_number = loop_number + 1;
                        break;
                    } else if instructions.get(loop_number).unwrap() == &&Instruction::Begin {
                        offset -= 1;
                    } else if instructions.get(loop_number).unwrap() == &&Instruction::End {
                        offset += 1;
                    }
                    loop_number += 1;
                }
            }
        } else if instructions.get(instruction_number).unwrap() == &&Instruction::End {
            if big_array[pointer] != 0 {
                instruction_number = *loop_indexes.last().unwrap() + 1;
            } else {
                loop_indexes.pop();
                instruction_number += 1;
            }
        } else if instructions.get(instruction_number).unwrap() == &&Instruction::Print {
            print!("{}", big_array[pointer] as char);
            instruction_number += 1;
        } else if instructions.get(instruction_number).unwrap() == &&Instruction::Input {
            let input: i32 = std::io::stdin()
                .bytes()
                .next()
                .and_then(|result| result.ok())
                .map(|byte| byte as i32)
                .unwrap();
            big_array[pointer] = input as u8;
            instruction_number += 1;
        } else {
            return Err("instruction not handled");
        }
    }
    println!(" ");
    Ok(())
}

fn read_in_file(file_path: &String) -> Result<String, std::io::Error> {
    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Left,
    Right,
    Plus,
    Minus,
    Begin,
    End,
    Print,
    Input,
}
