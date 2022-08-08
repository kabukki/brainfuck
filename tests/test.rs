use std::io::Read;
use brainfuck::Runtime;

#[test]
fn ascii () {
    let mut file = std::fs::File::open("./samples/ascii.bf").expect("Could not open source file");
    let mut interpreter = Runtime::new(std::io::empty(), vec![]);
    interpreter.run(&mut file).unwrap();
    assert_eq!(interpreter.output, b"A");
}

#[test]
fn read () {
    let mut file = std::fs::File::open("./samples/read.bf").expect("Could not open source file");
    let mut interpreter = Runtime::new(std::io::repeat(b'A').take(1), vec![]);
    interpreter.run(&mut file).unwrap();
    assert_eq!(interpreter.output, b"B");
}

// #[test]
// fn hello_world () {
//     let mut file = std::fs::File::open("./samples/hello.bf").expect("Could not open source file");
//     let mut interpreter = Runtime::new(std::io::empty(), vec![]);
//     interpreter.run(&mut file).unwrap();
//     assert_eq!(interpreter.output, b"Hello world");
// }
