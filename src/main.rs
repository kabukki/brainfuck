use brainfuck::Runtime;

fn main () -> Result<(), String> {
    let args = std::env::args().collect::<Vec<String>>();

    match args.get(1) {
        Some(path) => {
            match std::fs::File::open(path) {
                Ok(mut file) => {
                    let mut interpreter = Runtime::new(
                        std::io::stdin(),
                        std::io::stdout(),
                    );
                
                    interpreter.run(&mut file)
                },
                Err(err) => { Err(err.to_string()) }
            }
        },
        None => Err(format!("Usage: {} <file.bf>", args.get(0).unwrap()))
    }
}
