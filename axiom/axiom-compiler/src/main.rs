use axiom_compiler::lexer::Lexer;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: axiom compile <file.axiom>");
        return;
    }

    let file_path = &args[1];
    let source_code = fs::read_to_string(file_path).expect("Failed to read file");

    let mut lexer = Lexer::new(&source_code);
    let tokens = lexer.tokenize();

    for token in tokens {
        println!("{:?}", token);
    }
}
