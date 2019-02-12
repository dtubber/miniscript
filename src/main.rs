pub mod frontend;

pub mod vm;

use frontend::lexer::Lexer;
use frontend::ast::*;
use frontend::parser::Parser;
use frontend::token::*;

fn main() {
    let code = String::from(
        include_str!("../res/test.shit")
    );

    let mut lexer = Lexer::new(code);

    let tokens = lexer.tokenize();

    /*for token in tokens.iter() {
        println!("{}:{}", map_type(&token.token_type), token.content);
    }*/

    let mut parser = Parser::new(tokens);

    let program = parser.parse();

    println!("Amount of function declarations: {}", program.decl.len());

    for declaration in program.decl.iter() {
        match declaration {
            Declaration::Function(function_name, arguments, statements) => {
                print!("fn {}(", function_name);
                let mut i = 0;
                for argument in arguments.iter() {
                    print!("{}", argument);
                    if i != arguments.len() -1 {
                        print!(",");
                    }
                    i += 1;
                }
                print!(")");
                if statements.len() == 0 {
                    print!(";\n");
                    continue;
                }
                print!("{{\n");
                for statement in statements.iter() {
                    print!("\t");
                    match statement {
                        Statement::Variable(name) => {
                            print!("var {};\n", name);
                        },
                        _ => {}
                    };
                }
                print!("}}\n");
            },
            _ => {

            }
        };
    }

    println!("Finished.");
}

fn map_type(token_type: &TokenType) -> &'static str {
    let ret = match token_type {
        TokenType::Whitespace => {
            "WHITESPACE"
        },
        TokenType::Delimiter => {
            "DELIM"
        },
        TokenType::Operator => {
            "OP"
        },
        TokenType::Word => {
            "WORD"
        },
        TokenType::Keyword => {
            "KEYWORD"
        },
        TokenType::Integer => {
            "INT"
        },
        _ => {
            "N/A"
        }
    };
    ret
}
