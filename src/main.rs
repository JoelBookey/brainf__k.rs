use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub mod errors;
pub mod interpreter;

use interpreter::{Program, Token};

fn main() -> Result<(), String> {
    // collect command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("please provide a file".to_string());
    }
    let mut show_memory = false;
    if args.len() > 2 {
        if args[2] == "show-memory" {
            show_memory = true;
        } else {
            return Err(format!("Unexpected argument: {}", args[2]));
        }
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

    let mut program = Program::new(&tokens);
    program.run()?;
    println!();
    if show_memory {
        program.print_memory();
    }
    Ok(())
}

fn read_in_file(file_path: &String) -> Result<String, std::io::Error> {
    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
}
