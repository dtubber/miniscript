extern crate miniscript as mns;

use mns::prelude::*;

use mns::frontend::parser::Parser;

fn main() {
    let code = String::from(
        include_str!("../res/mod.mns")
    );
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
    let _program = parser.parse().unwrap();
    println!("Done parsing!");
    for decl in _program.declarations {
        decl.print(0);
    }
}