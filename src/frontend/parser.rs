use super::token::*;
use super::ast::*;

use std::collections::*;

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
    cursor_stack: VecDeque<usize>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            cursor: 0,
            cursor_stack: VecDeque::new()
        }
    }

    pub fn parse(&mut self) -> Program {
        let mut ret = Program {
            decl: Vec::new()
        };
        
        while !self.eoi() {
            let decl_opt = self.parse_function_declaration();
            if decl_opt.is_some() {
                ret.decl.push(decl_opt.unwrap());
            } else {
                break;
            }
        }

        ret
    }

    fn parse_function_declaration(&mut self) -> Option<Declaration> {
        //////println!("Parsing function declaration...");
        //////println!("Pushing cursor.");
        self.push_cursor();
        ////println!("Swallowing whitespace.");
        self.swallow(TokenType::Whitespace);
        ////println!("Getting keyword");
        let keyword_opt = self.swallow(TokenType::Keyword);
        if keyword_opt.is_none() {
            ////println!("No keyword! ERROR");
            self.pop_cursor();
            return None;
        }
        let keyword = keyword_opt.unwrap();
        if keyword.content.as_str() != "fn" {
            ////println!("Keyword is not \"fn\"! ERROR");
            self.pop_cursor();
            return None;
        }
        ////println!("Swallowing whitespace.");
        self.swallow(TokenType::Whitespace);
        let token_opt = self.peek(0);
        let token = token_opt.unwrap();
        ////println!("Token: {}", token.content);
        let function_name = self.swallow(TokenType::Word);
        if function_name.is_none() {
            ////println!("No function name! ERROR");
            self.pop_cursor();
            return None;
        }
        let name = function_name.unwrap();
        ////println!("Function name: {}", name.content);
        ////println!("Swallowing whitespace...");
        self.swallow(TokenType::Whitespace);
        ////println!("Swallowing delim...");
        let delim_opt = self.swallow(TokenType::Delimiter);
        if delim_opt.is_none() {
            ////println!("Function name not followed by delim! ERROR");
            self.pop_cursor();
            return None;
        }
        let delim = delim_opt.unwrap();
        if delim.content.as_str() != "(" {
            ////println!("Function name not followed by open brace! ERROR");
            self.pop_cursor();
            return None;
        }
        let arguments = self.parse_function_arguments();
        self.swallow(TokenType::Whitespace);
        let final_delim = self.swallow(TokenType::Delimiter);
        if final_delim.is_none() {
            ////println!("Missing semicolon!");
            self.pop_cursor();
            return None;
        }
        let delim = final_delim.unwrap().content;
        let mut statements = Vec::new();
        if delim.as_str() == "{" {
            //println!("Parsing statements...");
            statements = self.parse_statements();
        }
        let decl = Declaration::Function(name.content, arguments, statements);
        Some(
            decl
        )
    }

    fn parse_function_arguments(&mut self) -> Vec<String> {
        let mut ret = Vec::new();
        loop {
            if self.eoi() {
                break;
            }
            self.swallow(TokenType::Whitespace);
            let arg_opt = self.swallow(TokenType::Word);
            self.swallow(TokenType::Whitespace);
            let delim_opt = self.swallow(TokenType::Delimiter);
            if arg_opt.is_none() || delim_opt.is_none() {
                break;
            }
            let arg = arg_opt.unwrap();
            let delim = delim_opt.unwrap();

            ret.push(arg.content);

            if delim.content.as_str() == ")" {
                break;
            }
        }
        ret
    }

    fn parse_statements(&mut self) -> Vec<Statement> {
        let mut ret = Vec::new();

        loop {
            if self.eoi() {
                break;
            }

            { // Try parsing variable declaration
                //println!("Trying to parse variable declaration...");
                self.push_cursor();
                self.swallow(TokenType::Whitespace);
                let keyword_opt = self.swallow(TokenType::Keyword);
                if keyword_opt.is_some() {
                    let keyword = keyword_opt.unwrap();
                    if keyword.content.as_str() == "var" {
                        //println!("var keyword found!");
                        let var_opt = self.parse_var_decl();
                        if var_opt.is_some() {
                            //println!("Parsed a variable!");
                            ret.push(var_opt.unwrap());
                            continue;
                        } else {
                            self.pop_cursor()
                        }
                    }
                } else {
                    self.pop_cursor();
                }
            }
            { // Try parsing end of statement list
                //println!("Trying to parse end of statement list...");
                self.push_cursor();
                self.swallow(TokenType::Whitespace);
                let delim_opt = self.swallow(TokenType::Delimiter);
                if delim_opt.is_some() {
                    if delim_opt.unwrap().content.as_str() == "}" {
                        //println!("End of statement list!");
                        break;
                    } else {
                        self.pop_cursor();
                    }
                } else {
                    self.pop_cursor();
                }
            }
        }

        ret
    }

    fn parse_var_decl(&mut self) -> Option<Statement> {
        //println!("parse_var_decl().");
        self.swallow(TokenType::Whitespace);
        let name_opt = self.swallow(TokenType::Word);
        if name_opt.is_none() {
            //println!("No variable name!");
            return None;
        }
        let name = name_opt.unwrap();
        //println!("Name of variable: {}", name.content);
        self.swallow(TokenType::Whitespace);
        let delim_opt = self.swallow(TokenType::Delimiter);
        if delim_opt.is_none() {
            //println!("Missing semicolon!");
            return None;
        }
        if delim_opt.unwrap().content.as_str() != ";" {
            //println!("Missing semicolon!");
            return None;
        }
        Some(
            Statement::Variable(name.content)
        )
    }

    fn eoi(&self) -> bool {
        self.cursor >= self.tokens.len()
    }

    fn inc_cursor(&mut self, i: usize) {
        self.cursor += i;
    }

    fn push_cursor(&mut self) {
        self.cursor_stack.push_front(self.cursor);
    }

    fn pop_cursor(&mut self) {
        let cursor_opt = self.cursor_stack.pop_front();
        if cursor_opt.is_some() {
            self.cursor = cursor_opt.unwrap();
        }
    }

    fn peek(&self, n: usize) -> Option<Token> {
        let pos = self.cursor + n;
        let token_opt = self.tokens.get(pos);
        if token_opt.is_none() {
            return None;
        }
        Some(
            token_opt.unwrap().clone()
        )
    }

    fn swallow(&mut self, token_type: TokenType) -> Option<Token> {
        if self.eoi() {
            return None;
        }
        let token_opt = self.peek(0);
        if token_opt.is_none() {
            return None;
        }
        let token = token_opt.unwrap();
        if token.token_type != token_type {
            return None;
        }
        self.inc_cursor(1);
        Some(
            token.clone()
        )
    }

    fn swallow_multiple(&mut self, token_type: TokenType) -> Option<Vec<Token>> {
        let mut tokens = Vec::new();

        loop {
            if self.eoi() {
                break;
            }
            let token_opt = self.peek(0);
            if token_opt.is_none() {
                break;
            }
            let token = token_opt.unwrap();
            if token.token_type == token_type {
                tokens.push(token.clone());
                self.inc_cursor(1);
            } else {
                break;
            }
        }

        if tokens.len() == 0 {
            return None;
        }
        Some(
            tokens
        )
    }
}
