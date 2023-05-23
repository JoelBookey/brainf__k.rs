use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), String> {
    // collect command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("please provide a file".to_string());
    }

    // read file from arguments
    let file_path = &args[1];
    let contents = match read_in_file(file_path) {
        Ok(contents) => contents,
        Err(error) => {
            return Err(error.to_string());
        }
    };

    // do some lexering into a vector (oh yeahhhhh)
    let tokens = contents
        .trim()
        .chars()
        .filter_map(|x| match x {
            '<' => Some(Token::Left),
            '>' => Some(Token::Right),
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '[' => Some(Token::Begin),
            ']' => Some(Token::End),
            '.' => Some(Token::Print),
            ',' => Some(Token::Input),
            _ => None,
        })
        .collect::<Vec<Token>>();

    do_your_thing(tokens)?;

    Ok(())
}

fn do_your_thing(tokens: Vec<Token>) -> Result<(), String> {
    let mut tape: [u8; 30000] = [0; 30000];
    let mut pointer: usize = 0;
    let mut token_number = 0;
    let mut loop_indexes: Vec<usize> = Vec::new();

    while token_number < tokens.len() {
        if tokens.get(token_number).unwrap() == &Token::Left {
            if pointer == 0 {
                pointer = 29999;
            } else {
                pointer -= 1;
            }
            token_number += 1;
        } else if tokens.get(token_number).unwrap() == &Token::Right {
            if pointer == 30000 - 1 {
                pointer = 0;
            } else {
                pointer += 1;
            }
            token_number += 1;
        } else if tokens.get(token_number).unwrap() == &Token::Plus {
            tape[pointer] = tape[pointer].wrapping_add(1);
            token_number += 1;
        } else if tokens.get(token_number).unwrap() == &Token::Minus {
            tape[pointer] = tape[pointer].wrapping_sub(1);
            token_number += 1;
        } else if tokens.get(token_number).unwrap() == &Token::Begin {
            if tape[pointer] != 0 {
                loop_indexes.push(token_number);
                token_number += 1;
            } else {
                let mut loop_number = token_number + 1;
                let mut offset = 0;
                while loop_number < tokens.len() {
                    if tokens.get(loop_number).unwrap() == &Token::End && offset == 0 {
                        token_number = loop_number + 1;
                        break;
                    } else if tokens.get(loop_number).unwrap() == &Token::Begin {
                        offset -= 1;
                    } else if tokens.get(loop_number).unwrap() == &Token::End {
                        offset += 1;
                    }
                    loop_number += 1;
                }
            }
        } else if tokens.get(token_number).unwrap() == &Token::End {
            if tape[pointer] != 0 {
                token_number = *loop_indexes.last().unwrap() + 1;
            } else {
                loop_indexes.pop();
                token_number += 1;
            }
        } else if tokens.get(token_number).unwrap() == &Token::Print {
            token_number += 1;
            print!("{}", tape[pointer] as char);
        } else if tokens.get(token_number).unwrap() == &Token::Input {
            let input = std::io::stdin().bytes().next().unwrap().unwrap();
            tape[pointer] = input;
            token_number += 1;
        } else {
            return Err("token not handled".into());
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
enum Token {
    Left,
    Right,
    Plus,
    Minus,
    Begin,
    End,
    Print,
    Input,
}
