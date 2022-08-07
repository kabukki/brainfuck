use std::io::{Read, Write};
use crate::parser::{Token, tokenize, parse};

pub struct Runtime<Input, Output> {
    memory: [u8; 100],
    pointer: usize,
    pub input: Input,
    pub output: Output,
}

impl<Input: Read, Output: Write> Runtime<Input, Output> {
    pub fn new (input: Input, output: Output) -> Self {
        Self {
            memory: [0; 100],
            pointer: 0,
            input,
            output,
        }
    }

    pub fn run (&mut self, input: &mut impl Read) -> Result<(), String> {
        let mut code = String::new();
        input.read_to_string(&mut code).or_else(|err| Err(err.to_string()))?;
        let tokens = tokenize(code.chars())?;
        let ast = parse(tokens)?;

        for token in ast {
            match token {
                Token::INCREMENT => { self.increment(); },
                Token::DECREMENT => { self.decrement(); },
                Token::PRINT => { self.print(); },
                _ => {},
            }
        }

        Ok(())
    }

    pub fn increment (&mut self) {
        self.memory[self.pointer] += 1;
    }

    pub fn decrement (&mut self) {
        self.memory[self.pointer] -= 1;
    }

    pub fn print (&mut self) {
        self.output.write_fmt(
            format_args!("{}", self.memory[self.pointer] as char)
        ).expect("Could not write out");
    }
}
