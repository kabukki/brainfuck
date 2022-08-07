use std::io::Write;
use crate::parser::{Token, tokenize, parse};

pub struct Interpreter<Out> {
    memory: [u8; 100],
    pointer: usize,
    pub out: Out,
}

impl<Out: Write> Interpreter<Out> {
    pub fn new (out: Out) -> Self {
        Self {
            memory: [0; 100],
            pointer: 0,
            out,
        }
    }

    pub fn run (&mut self, input: std::str::Chars) -> Result<(), String> {
        let tokens = tokenize(input)?;
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
        self.out.write_fmt(
            format_args!("{}", self.memory[self.pointer] as char)
        ).expect("Could not write out");
    }
}
