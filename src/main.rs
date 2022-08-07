use std::io::Read;
use brainfuck::runtime::Interpreter;

fn main () -> Result<(), String> {
    let mut code = String::new();
    match std::io::stdin().read_to_string(&mut code) {
        Ok(_)       => {
            // let input = std::io::stdin().lock();
            let output = std::io::stdout().lock();
            let mut interpreter = Interpreter::new(/* in, */ output);
        
            interpreter.run(code.chars())
        },
        Err(err)    => Err(err.to_string())
    }
}
