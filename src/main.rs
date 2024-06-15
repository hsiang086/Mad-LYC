mod tokenization;

use std::env;
use crate::tokenization::{ Tokenizer, Token, TokenType };

fn tokens2asm(tokens: Vec<Token>) -> String {
    let mut asm_str: String = String::new();

    asm_str.push_str("section .text\n\tglobal _start\n\n_start:\n");
    for token in tokens {
        match token {
            Token { token_type: TokenType::Exit, .. } => {
                asm_str.push_str("\tmov rax, 60\n");
            },
            Token { token_type: TokenType::IntLit, value } => {
                asm_str.push_str(&format!("\tmov rdi, {}\n", value));
            },
            Token { token_type: TokenType::Semi, .. } => {
                asm_str.push_str("\tsyscall\n");
            },
        }
    }

    asm_str
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len: usize = args.len();

    if args_len < 2 {
        println!("Usage: Mad <source file>");
        return;
    }

    let contents: String = match std::fs::read_to_string(&args[1]) {
        Ok(contents) => contents,
        Err(e) => {
            println!("Error reading file: {}", e);
            return;
        }
    };

    let mut tokenizer = Tokenizer::new(contents);
    let tokens: Vec<Token> = tokenizer.tokenize();

    let asm_str: String = tokens2asm(tokens);
    let output_file: String = format!("{}.asm", &args[1]);

    match std::fs::write(&output_file, asm_str) {
        Ok(_) => println!("Output written to {}", output_file),
        Err(e) => println!("Error writing output: {}", e),
    }
}
