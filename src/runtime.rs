use std::io::{Read, Write};
use crate::{Node, tokenize, parse};

pub struct Runtime<In, Out> {
    memory: [u8; 100],
    pointer: usize,
    cycles: usize,
    pub input: In,
    pub output: Out,
}

impl<In, Out> Runtime<In, Out> where In: Read, Out: Write {
    pub fn new (input: In, output: Out) -> Self {
        Self {
            memory: [0; 100],
            pointer: 0,
            cycles: 0,
            input,
            output,
        }
    }

    pub fn run (&mut self, input: &mut impl Read) -> Result<(), String> {
        let mut code = String::new();
        input.read_to_string(&mut code).or_else(|err| Err(err.to_string()))?;
        let tokens = tokenize(code.chars())?;
        let ast = parse(&tokens)?;

        for node in ast {
            self.exec(&node);
        }

        Ok(())
    }

    pub fn exec (&mut self, node: &Node) {
        match node {
            Node::Increment    => { self.increment(); },
            Node::Decrement    => { self.decrement(); },
            Node::Backward     => { self.backward(); },
            Node::Forward      => { self.forward(); },
            Node::Loop (children) => {
                while self.memory[self.pointer] != 0 {
                    for child in children {
                        self.exec(child);
                    }
                }
            },
            Node::Print        => { self.print(); },
            Node::Read         => { self.read(); },
        }
    }

    pub fn increment (&mut self) {
        self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1);
        self.cycles += 1;
    }

    pub fn decrement (&mut self) {
        self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1);
        self.cycles += 1;
    }

    pub fn backward (&mut self) {
        self.pointer = self.pointer.wrapping_sub(1);
        self.cycles += 1;
    }

    pub fn forward (&mut self) {
        self.pointer = self.pointer.wrapping_add(1);
        self.cycles += 1;
    }

    pub fn print (&mut self) {
        self.output.write_fmt(
            format_args!("{}", self.memory[self.pointer] as char)
        ).expect("Could not print");
        self.cycles += 1;
    }

    pub fn read (&mut self) {
        self.input.read(
            &mut self.memory[self.pointer .. self.pointer + 1]
        ).expect("Could not read");
        self.cycles += 1;
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
        assert_eq!(interpreter.cycles, 0);
    }
    
    #[test]
    fn increment () {
        let mut interpreter = Runtime::new(std::io::empty(), vec![]);
        interpreter.exec(&Node::Increment);
        assert_eq!(interpreter.memory[0], 1);
    }

    #[test]
    fn increment_wraps () {
        let mut interpreter = Runtime::new(std::io::empty(), vec![]);
        interpreter.memory[0] = u8::MAX;
        interpreter.exec(&Node::Increment);
        assert_eq!(interpreter.memory[0], 0);
    }

    #[test]
    fn decrement () {
        let mut interpreter = Runtime::new(std::io::empty(), vec![]);
        interpreter.memory[0] = 1;
        interpreter.exec(&Node::Decrement);
        assert_eq!(interpreter.memory[0], 0);
    }

    #[test]
    fn decrement_wraps () {
        let mut interpreter = Runtime::new(std::io::empty(), vec![]);
        interpreter.exec(&Node::Decrement);
        assert_eq!(interpreter.memory[0], u8::MAX);
    }

    #[test]
    fn backward () {
        let mut interpreter = Runtime::new(std::io::empty(), vec![]);
        interpreter.pointer = 1;
        interpreter.exec(&Node::Backward);
        assert_eq!(interpreter.pointer, 0);
    }

    #[test]
    fn backward_wraps () {
        let mut interpreter = Runtime::new(std::io::empty(), vec![]);
        interpreter.exec(&Node::Backward);
        assert_eq!(interpreter.pointer, usize::MAX);
    }

    #[test]
    fn forward () {
        let mut interpreter = Runtime::new(std::io::empty(), vec![]);
        interpreter.exec(&Node::Forward);
        assert_eq!(interpreter.pointer, 1);
    }

    #[test]
    fn forward_wraps () {
        let mut interpreter = Runtime::new(std::io::empty(), vec![]);
        interpreter.pointer = usize::MAX;
        interpreter.exec(&Node::Forward);
        assert_eq!(interpreter.pointer, 0);
    }

    #[test]
    fn print () {
        let mut interpreter = Runtime::new(std::io::empty(), vec![]);
        interpreter.exec(&Node::Print);
        assert_eq!(interpreter.output as Vec<u8>, [0]);
    }
    
    #[test]
    fn read () {
        let mut interpreter = Runtime::new(std::io::repeat(b'A').take(1), vec![]);
        interpreter.exec(&Node::Read);
        assert_eq!(interpreter.memory[0], b'A');
        assert_eq!(interpreter.memory[1], 0);
    }

    #[test]
    fn loop_once () {
        let mut interpreter = Runtime::new(std::io::empty(), vec![]);
        interpreter.memory[0] = 1;
        interpreter.exec(&Node::Loop(vec![Node::Decrement]));
        assert_eq!(interpreter.cycles, 1);
    }
}
