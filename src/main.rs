use brainfuck::runtime::Runtime;

fn main () -> Result<(), String> {
    let input = std::io::stdin().lock();
    let output = std::io::stdout().lock();
    let mut interpreter = Runtime::new(input, output);

    interpreter.run(&mut std::io::stdin())
}
