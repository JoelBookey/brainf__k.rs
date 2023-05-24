use crate::errors::RuntimeError;
use std::io::Read;

pub struct Program<'a> {
    tape: [u8; 30000],
    tokens: &'a Vec<Token>,
    last_pointer: usize,
}

impl<'a> Program<'a> {
    #[must_use]
    pub fn new(input_tokens: &'a Vec<Token>) -> Program<'a> {
        Program {
            tape: [0; 30000],
            tokens: input_tokens,
            last_pointer: 0,
        }
    }
    pub fn run(&mut self) -> Result<(), RuntimeError> {
        let mut pointer: usize = 0;
        let mut token_number: usize = 0;
        let mut loop_indexes: Vec<usize> = Vec::new();
        while token_number < self.tokens.len() {
            if self.tokens.get(token_number).unwrap() == &Token::Left {
                if pointer == 0 {
                    pointer = 29999;
                } else {
                    pointer -= 1;
                }
                token_number += 1;
            } else if self.tokens.get(token_number).unwrap() == &Token::Right {
                if pointer == 30000 - 1 {
                    pointer = 0;
                } else {
                    pointer += 1;
                }
                token_number += 1;
            } else if self.tokens.get(token_number).unwrap() == &Token::Plus {
                self.tape[pointer] = self.tape[pointer].wrapping_add(1);
                token_number += 1;
            } else if self.tokens.get(token_number).unwrap() == &Token::Minus {
                self.tape[pointer] = self.tape[pointer].wrapping_sub(1);
                token_number += 1;
            } else if self.tokens.get(token_number).unwrap() == &Token::Begin {
                if self.tape[pointer] == 0 {
                    let mut loop_number = token_number;
                    let mut offset: i32 = 0;
                    loop {
                        if loop_number >= self.tokens.len() {
                            return Err(RuntimeError::MissingBracket);
                        }
                        if (self.tokens.get(loop_number).unwrap() == &Token::End) && (offset == 0) {
                            break;
                        }
                        if self.tokens.get(loop_number).unwrap() == &Token::Begin {
                            offset -= 1;
                        } else if self.tokens.get(loop_number).unwrap() == &Token::End {
                            offset += 1;
                        }
                        loop_number += 1;
                    }
                    token_number = loop_number + 1;
                } else {
                    loop_indexes.push(token_number);
                    token_number += 1;
                }
            } else if self.tokens.get(token_number).unwrap() == &Token::End {
                if loop_indexes.len() == 0 {
                    return Err(RuntimeError::MissingBracket);
                }
                if self.tape[pointer] == 0 {
                    loop_indexes.pop();
                    token_number += 1;
                } else {
                    token_number = *loop_indexes.last().unwrap() + 1;
                }
            } else if self.tokens.get(token_number).unwrap() == &Token::Print {
                print!("{}", self.tape[pointer] as char);
                token_number += 1;
            } else if self.tokens.get(token_number).unwrap() == &Token::Input {
                self.tape[pointer] = std::io::stdin().bytes().next().unwrap().unwrap();
                token_number += 1;
            } else {
                return Err(RuntimeError::UnexpectedToken);
            }
        }
        self.last_pointer = pointer;
        Ok(())
    }
    pub fn print_memory(&self, begin: usize, end: usize) {
        println!("{:?}", &self.tape[begin..end]);
        println!("Pointer Value: {}", &self.last_pointer);
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Left,
    Right,
    Plus,
    Minus,
    Begin,
    End,
    Print,
    Input,
}
