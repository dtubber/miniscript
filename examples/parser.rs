extern crate rust_shit;

use rust_shit::prelude::*;

use rust_shit::frontend::parser::Parser;

fn main() {
    let code = String::from(
        include_str!("../res/test.shit")
    );
    println!("Source code:");
    println!("-----");
    println!("{}", code);
    println!("-----");
    println!("Creating lexer...");
    let mut lexer = Lexer::new(code);
    println!("Lexer created!");
    println!("Lexing...");
    let tokens = lexer.tokenize();
    println!("Done lexing!");
    println!("Creating parser...");
    let mut parser = Parser::new(tokens);
    println!("Parser created!");
    println!("Parsing...");
    let program = parser.parse();
    println!("Done parsing!");
}