use brainfuck::Runtime;

#[test]
fn ascii () {
    let mut file = std::fs::File::open("./tests/code/ascii.bf").expect("Could not open source file");
    let mut interpreter = Runtime::new(std::io::empty(), vec![]);
    interpreter.run(&mut file).unwrap();
    assert_eq!(interpreter.output, b"A");
}

// #[test]
// fn hello_world () {
//     let mut file = std::fs::File::open("./tests/code/hello.bf").expect("Could not open source file");
//     let mut interpreter = Runtime::new(std::io::empty(), vec![]);
//     interpreter.run(&mut file).unwrap();
//     assert_eq!(interpreter.output, b"Hello world");
// }