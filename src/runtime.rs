use std::io::{Read, Write};
use crate::{Token, tokenize, parse};

pub struct Runtime<In, Out> {
    memory: [u8; 100],
    pointer: usize,
    pub input: In,
    pub output: Out,
}

impl<In, Out> Runtime<In, Out> where In: Read, Out: Write {
    pub fn new (input: In, output: Out) -> Self {
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
                Token::INCREMENT    => { self.increment(); },
                Token::DECREMENT    => { self.decrement(); },
                Token::LEFT         => { self.backward(); },
                Token::RIGHT        => { self.forward(); },
                Token::PRINT        => { self.print(); },
                _ => {},
            }
        }

        Ok(())
    }

    pub fn increment (&mut self) {
        self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1);
    }

    pub fn decrement (&mut self) {
        self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1);
    }

    pub fn backward (&mut self) {
        self.pointer -= 1;
    }

    pub fn forward (&mut self) {
        self.pointer += 1;
    }

    pub fn print (&mut self) {
        self.output.write_fmt(
            format_args!("{}", self.memory[self.pointer] as char)
        ).expect("Could not write out");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn initial_state () {
        let interpreter = Runtime::new(std::io::empty(), vec![]);
    
        assert_eq!(interpreter.memory, [0; 100]);
        assert_eq!(interpreter.pointer, 0);
    }
    
    #[test]
    fn print () {
        let mut interpreter = Runtime::new(std::io::empty(), vec![]);
        interpreter.run(&mut vec![b'.'].as_slice()).unwrap();
    
        assert_eq!((interpreter.output as Vec<u8>).len(), 1);
    }
    
    #[test]
    fn increment () {
        let mut interpreter = Runtime::new(std::io::empty(), vec![]);
        interpreter.run(&mut vec![b'+'].as_slice()).unwrap();
    
        assert_eq!(interpreter.memory[0], 1);
    }

    #[test]
    fn increment_wraps () {
        let mut interpreter = Runtime::new(std::io::empty(), vec![]);

        assert_eq!(interpreter.memory[0], 0);
        interpreter.run(&mut std::io::repeat(b'+').take(u8::MAX as u64)).unwrap();
        assert_eq!(interpreter.memory[0], u8::MAX);
        interpreter.run(&mut std::io::repeat(b'+').take(1)).unwrap();
        assert_eq!(interpreter.memory[0], 0);
    }

    #[test]
    fn decrement () {
        let mut interpreter = Runtime::new(std::io::empty(), vec![]);
        interpreter.run(&mut vec![b'-'].as_slice()).unwrap();
    
        assert_eq!(interpreter.memory[0], u8::MAX);
    }

    #[test]
    fn decrement_wraps () {
        let mut interpreter = Runtime::new(std::io::empty(), vec![]);

        assert_eq!(interpreter.memory[0], 0);
        interpreter.run(&mut std::io::repeat(b'-').take(1)).unwrap();
        assert_eq!(interpreter.memory[0], u8::MAX);
        interpreter.run(&mut std::io::repeat(b'-').take(u8::MAX as u64)).unwrap();
        assert_eq!(interpreter.memory[0], 0);
    }
}
