use super::token::*;
use super::ast::*;
use super::error::*;

use std::collections::*;

pub type ParseResult<T> = Result<T, Error>;

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

    pub fn parse(&mut self) -> ParseResult<Program> {
        let decls = self.parse_decl_list()?;
        Ok(
            Program {
                declarations: decls
            }
        )
    }

    fn parse_decl_list(&mut self) -> ParseResult<Vec<Declaration>> {
        let mut ret = Vec::new();
        while !self.eoi() {
            let _ = self.swallow(TokenType::Whitespace);
            self.push_cursor();
            let mut keyword_opt = self.swallow(TokenType::Keyword);
            if keyword_opt.is_some() {
                let keyword = keyword_opt.unwrap().content;
                match keyword.as_str() {
                    "mod" => {
                        let _ = self.swallow(TokenType::Whitespace);
                        let delim_opt = self.swallow(TokenType::Delimiter);
                        if  delim_opt.is_some() &&
                            delim_opt.unwrap().content == ":" {
                                let mod_expr = self.parse_mod_decl()?;
                                ret.push(mod_expr);
                        } else {
                            return Err(
                                Error {
                                    kind: ErrorKind::Syntax,
                                    message: format!("Missing delimiter \":\"!")
                                }
                            );
                        }
                    },
                    "fn" => {
                        let _ = self.swallow(TokenType::Whitespace);
                        let delim_opt = self.swallow(TokenType::Delimiter);
                        if  delim_opt.is_some() &&
                            delim_opt.unwrap().content == ":" {
                                let fn_expr = self.parse_fn_decl()?;
                                ret.push(fn_expr);
                        } else {
                            return Err(
                                Error {
                                    kind: ErrorKind::Syntax,
                                    message: format!("Missing delimiter \":\"!")
                                }
                            );
                        }
                    },
                    "struct" => {
                        let _ = self.swallow(TokenType::Whitespace);
                        let delim_opt = self.swallow(TokenType::Delimiter);
                        if  delim_opt.is_some() &&
                            delim_opt.unwrap().content == ":" {
                                let mod_expr = self.parse_mod_decl()?;
                                ret.push(mod_expr);
                        } else {
                            return Err(
                                Error {
                                    kind: ErrorKind::Syntax,
                                    message: format!("Missing delimiter \":\"!")
                                }
                            );
                        }
                    },
                    _ => {
                        panic!("Invalid declaration keyword!");
                    }
                };
            } else {
                break;
            }
        }

        Ok(
            ret
        )
    }

    fn parse_mod_decl(&mut self) -> ParseResult<Declaration> {
        let mut name = String::new();
        let mut ret = Vec::new();
        while !self.eoi() {
            let _ = self.swallow(TokenType::Whitespace);
            let name_opt = self.swallow(TokenType::Word);
            if name_opt.is_some() {
                name = name_opt.unwrap().content;
                let _ = self.swallow(TokenType::Whitespace);
                let mut delim_opt = self.swallow(TokenType::Delimiter);
                if delim_opt.is_some() {
                    let delim = delim_opt.unwrap().content;
                    if delim == "{" {
                        let decl_list = self.parse_decl_list()?;
                        ret = decl_list;
                        let _ = self.swallow(TokenType::Whitespace);
                        delim_opt = self.swallow(TokenType::Delimiter);
                        if delim_opt.is_some() {
                            let delim = delim_opt.unwrap().content;
                            if delim == "}" {
                                break;
                            } else {
                                return Err(
                                    Error {
                                        kind: ErrorKind::Syntax,
                                        message: format!("Incorrect delimiter \"{}\"!", delim)
                                    }
                                );
                            }
                        } else {
                            return Err(
                                Error {
                                    kind: ErrorKind::Syntax,
                                    message: String::from(
                                        "Missing delimiter \"}\"!"
                                    )
                                }
                            );
                        }
                    }
                } else {
                    return Err(
                        Error {
                            kind: ErrorKind::Syntax,
                            message: String::from(
                                "Missing delimiter \";\" or \"{\"!"
                            )
                        }
                    );
                }
            }
        }
        if name.is_empty() {
            return Err(
                Error {
                    kind: ErrorKind::Syntax,
                    message: String::from(
                        "Missing module name!"
                    )
                }
            );
        }
        Ok(
            Declaration::Module(name, ret)
        )
    }

    fn parse_fn_decl(&mut self) -> ParseResult<Declaration> {
        let mut name = String::new();
        let mut ret_type = String::new();
        let mut args: Vec<Argument> = Vec::new();
        let mut statements: Vec<Statement> = Vec::new();
        while !self.eoi() {
            let _ = self.swallow(TokenType::Whitespace);
            let name_opt = self.swallow(TokenType::Word);
            if name_opt.is_some() {
                name = name_opt.unwrap().content;
                let _ = self.swallow(TokenType::Whitespace);
                let mut delim_opt = self.swallow(TokenType::Delimiter);
                if delim_opt.is_some() {
                    let mut delim = delim_opt.unwrap().content;
                    if delim.as_str() == "(" {
                        args = self.parse_fn_args()?;
                        let _ = self.swallow(TokenType::Delimiter);
                        let _ = self.swallow(TokenType::Whitespace);
                        delim_opt = self.swallow(TokenType::Delimiter);
                        if delim_opt.is_some() {
                            delim = delim_opt.unwrap().content;
                            if delim == ":" {
                                let _ = self.swallow(TokenType::Whitespace);
                                let type_opt = self.swallow(TokenType::Word);
                                if type_opt.is_some() {
                                    ret_type = type_opt.unwrap().content;
                                    let _ = self.swallow(TokenType::Whitespace);
                                    delim_opt = self.swallow(TokenType::Delimiter);
                                    if delim_opt.is_some() {
                                        delim = delim_opt.unwrap().content;
                                        if delim == "{" {
                                            statements = self.parse_statements()?;
                                            let _ = self.swallow(TokenType::Whitespace);
                                            delim_opt = self.swallow(TokenType::Delimiter);
                                            if delim_opt.is_some() {
                                                delim = delim_opt.unwrap().content;
                                                if delim == "}" {
                                                    break;
                                                } else {
                                                    // TODO: Throw appropriate error!
                                                }
                                            } else {
                                                // TODO: Throw appropriate error!
                                            }
                                        } else {
                                            // TODO: Throw appropriate error!
                                        }
                                    } else {
                                        // TODO: Throw appropriate error!
                                    }
                                } else {
                                    // TODO: Throw appropriate error!
                                }
                            } else if delim == "{" {
                                statements = self.parse_statements()?;
                                let _ = self.swallow(TokenType::Whitespace);
                                delim_opt = self.swallow(TokenType::Delimiter);
                                if delim_opt.is_some() {
                                    delim = delim_opt.unwrap().content;
                                    if delim == "}" {
                                        break;
                                    } else {
                                        // TODO: Throw appropriate error!
                                    }
                                } else {
                                    // TODO: Throw appropriate error!
                                }
                            }
                        } else {
                            // TODO: Throw appropriate error!
                        }
                    }
                }
            } else {
                return Err(
                    Error {
                        kind: ErrorKind::Syntax,
                        message: format!("Missing function name!")
                    }  
                );
            }
        }
        Ok(
            Declaration::Function(name, ret_type, args, statements)
        )
    }

    fn parse_struct_decl(&mut self) -> ParseResult<Declaration> {
        let mut typename = String::new();
        let mut declarations = Vec::new();
        while !self.eoi() {
            self.swallow(TokenType::Whitespace);
            let name_opt = self.swallow(TokenType::Word);
            if name_opt.is_some() {
                typename = name_opt.unwrap().content;
                self.swallow(TokenType::Whitespace);
                let mut delim_opt = self.swallow(TokenType::Delimiter);
                if delim_opt.is_some() {
                    let mut delim = delim_opt.unwrap().content;
                    if delim == "{" {
                        let member_decls = self.parse_member_decls()?;
                    } else {
                        return Err(
                            Error {
                                kind: ErrorKind::Syntax,
                                message: format!("Missing function name!")
                            }  
                        );
                    }
                } else {
                    return Err(
                        Error {
                            kind: ErrorKind::Syntax,
                            message: format!("Missing function name!")
                        }  
                    );
                }
            } else {
                return Err(
                    Error {
                        kind: ErrorKind::Syntax,
                        message: format!("Missing function name!")
                    }  
                );   
            }
        }
        Ok(
            Declaration::Struct(typename, declarations)
        )
    }

    fn parse_member_decls(&mut self) -> ParseResult<Vec<MemberDeclaration>> {
        let mut some = true;
        while !self.eoi() && some {
            some = false;
            self.swallow(TokenType::Whitespace);
            self.push_cursor();
            let mut public = false;
            let mut keyword_opt = self.swallow(TokenType::Keyword);
            if keyword_opt.is_some() {
                some = true;
                let keyword = keyword_opt.unwrap().content;
                if keyword == "pub" {
                    self.drop_cursor();
                    self.swallow(TokenType::Whitespace);
                    public = true;
                } else {
                    self.pop_cursor();
                }
            } else {
                self.drop_cursor();
            }
            self.push_cursor();
            keyword_opt = self.swallow(TokenType::Keyword);
            if keyword_opt.is_some() {
                some = true;
                let keyword = keyword_opt.unwrap().content;
                if keyword == "var" || keyword == "const" {
                    self.drop_cursor();
                    // TODO: Implement member variable parsing
                } else {
                    self.pop_cursor();
                }
            } else {
                self.drop_cursor();
            }
            keyword_opt = self.swallow(TokenType::Keyword);
            if keyword_opt.is_some() {
                some = true;
                let keyword = keyword_opt.unwrap().content;
                if keyword == "fn" {
                    self.drop_cursor();
                    // TODO: Implement member function parsing
                } else {
                    self.pop_cursor();
                }
            } else {
                self.drop_cursor();
            }
        }
        return Err(Error {
            kind: ErrorKind::Syntax,
            message: format!("Unknown error!"),
        });
    }

    fn parse_fn_args(&mut self) -> ParseResult<Vec<Argument>> {
        let mut ret = Vec::new();
        while !self.eoi() {
            let _ = self.swallow(TokenType::Whitespace);
            let keyword_opt = self.swallow(TokenType::Keyword);
            if keyword_opt.is_some() {
                let keyword = keyword_opt.unwrap().content;
                let _ = self.swallow(TokenType::Whitespace);
                let mut delim_opt = self.swallow(TokenType::Delimiter);
                if delim_opt.is_some() {
                    let mut delim = delim_opt.unwrap().content;
                    if delim == ":" {
                        let _ = self.swallow(TokenType::Whitespace);
                        let type_opt = self.swallow(TokenType::Word);
                        if type_opt.is_some() {
                            let vartype = type_opt.unwrap();
                            let _ = self.swallow(TokenType::Whitespace);
                            let name_opt = self.swallow(TokenType::Word);
                            if name_opt.is_some() {
                                let name = name_opt.unwrap();
                                match keyword.as_str() {
                                    "var" => {
                                        ret.push(Argument::Mutable(vartype.content, name.content));
                                    },
                                    "const" => {
                                        ret.push(Argument::Constant(vartype.content, name.content));
                                    },
                                    _ => {}
                                };
                                let _ = self.swallow(TokenType::Whitespace);
                                self.push_cursor();
                                delim_opt = self.swallow(TokenType::Delimiter);
                                if delim_opt.is_some() {
                                    delim = delim_opt.unwrap().content;
                                    if delim == ")" {
                                        self.pop_cursor();
                                        break;
                                    } else if delim == "," {
                                        self.drop_cursor();
                                        continue;
                                    }
                                } else {
                                    // TODO: Throw error
                                }
                            } else {
                                // TODO: Throw error
                            }
                        } else {
                            // TODO: Throw error
                        }
                    } else {
                        // TODO: Throw error
                    }
                } else {
                    // TODO: Throw appropriate error!
                }
            } else {
                break;
            }
        }
        Ok(ret)
    }

    fn parse_statements(&mut self) -> ParseResult<Vec<Statement>> {
        let mut ret = Vec::new();
        let mut some = true;
        while !self.eoi() && some {
            some = false;
            self.swallow(TokenType::Whitespace);
            let keyword_opt = self.swallow(TokenType::Keyword);
            if keyword_opt.is_some() {
                some = true;
                let keyword = keyword_opt.unwrap().content;
                if keyword == "var" || keyword == "const" {
                    let var_decl = self.parse_var_decl(keyword)?;
                    self.swallow(TokenType::Whitespace);
                    let delim_opt = self.swallow(TokenType::Delimiter);
                    if delim_opt.is_some() {
                        let delim = delim_opt.unwrap().content;
                        if delim == ";" {
                            continue;
                        } else {
                            // TODO: Throw error
                        }
                    } else {
                        // TODO: Throw error
                    }
                }
            }
            let name_opt = self.swallow(TokenType::Word);
            if name_opt.is_some() {
                let name = name_opt.unwrap().content;
                self.swallow(TokenType::Whitespace);
                let mut delim_opt = self.swallow(TokenType::Delimiter);
                if delim_opt.is_some() {
                    let mut delim = delim_opt.unwrap().content;
                    if delim == "=" {
                        some = true;
                        let expr = self.parse_expr()?;
                        let stmt = Statement::Assign(name, expr);
                        ret.push(stmt);
                        self.swallow(TokenType::Whitespace);
                        delim_opt = self.swallow(TokenType::Delimiter);
                        if delim_opt.is_some() {
                            delim = delim_opt.unwrap().content;
                            if delim == ";" {
                                continue;
                            } else {
                                // TODO: Throw error
                            }
                        } else {
                            // TODO: Throw error
                        }
                    }
                }
            }
        }
        Ok(ret)
    }

    fn parse_var_decl(&mut self, keyword: String) -> ParseResult<Statement> {
        while !self.eoi() {
            self.swallow(TokenType::Whitespace);
            let mut delim_opt = self.swallow(TokenType::Delimiter);
            if delim_opt.is_some() {
                let mut delim = delim_opt.unwrap().content;
                if delim == ":" {
                    self.swallow(TokenType::Whitespace);
                    let typename_opt = self.swallow(TokenType::Word);
                    if typename_opt.is_some() {
                        let typename = typename_opt.unwrap().content;
                        self.swallow(TokenType::Whitespace);
                        let name_opt = self.swallow(TokenType::Word);
                        if name_opt.is_some() {
                            let name = name_opt.unwrap().content;
                            self.swallow(TokenType::Whitespace);
                            delim_opt = self.swallow(TokenType::Delimiter);
                            if delim_opt.is_some() {
                                delim = delim_opt.unwrap().content;
                                if delim == ";" {
                                    return Ok(
                                        Statement::VariableDecl(
                                            VariableDef {
                                                constant: match keyword.as_str() {
                                                    "const" => {true},
                                                    "var" => {false},
                                                    _ => {false}
                                                },
                                                vartype: typename,
                                                name: name
                                            }
                                        )
                                    );
                                } else if delim == "=" {
                                    let expr = self.parse_expr()?;
                                    self.swallow(TokenType::Whitespace);
                                    delim_opt = self.swallow(TokenType::Delimiter);
                                    if delim_opt.is_some() {
                                        delim = delim_opt.unwrap().content;
                                        if delim == ";" {
                                            return Ok(
                                                Statement::VariableDeclAssign(
                                                    VariableDef {
                                                        constant: match keyword.as_str() {
                                                            "const" => {true},
                                                            "var" => {false},
                                                            _ => {false}
                                                        },
                                                        vartype: typename,
                                                        name: name
                                                    },
                                                    expr
                                                )
                                            );
                                        } else {
                                            return Err(
                                                Error {
                                                    kind: ErrorKind::Syntax,
                                                    message: format!("Unknown error!")
                                                }
                                            );
                                        }
                                    } else {
                                        return Err(
                                            Error {
                                                kind: ErrorKind::Syntax,
                                                message: format!("Unknown error!")
                                            }
                                        );
                                    }
                                }
                            } else {
                                return Err(
                                    Error {
                                        kind: ErrorKind::Syntax,
                                        message: format!("Unknown error!")
                                    }
                                );
                            }
                        } else {
                            return Err(
                                Error {
                                    kind: ErrorKind::Syntax,
                                    message: format!("Unknown error!")
                                }
                            );
                        }
                    } else {
                        return Err(
                            Error {
                                kind: ErrorKind::Syntax,
                                message: format!("Unknown error!")
                            }
                        );
                    }
                } else {
                    return Err(
                        Error {
                            kind: ErrorKind::Syntax,
                            message: format!("Unknown error!")
                        }
                    );
                }
            } else {
                return Err(
                    Error {
                        kind: ErrorKind::Syntax,
                        message: format!("Unknown error!")
                    }
                );
            }
        }
        Err(
            Error {
                kind: ErrorKind::Syntax,
                message: format!("Unknown error!"),
            }
        )
    }

    fn parse_expr(&mut self) -> ParseResult<Expression> {
        while !self.eoi() {
            self.swallow(TokenType::Whitespace);
            let int_opt = self.swallow(TokenType::Integer);
            if int_opt.is_some() {
                let int = int_opt.unwrap().content;
                self.swallow(TokenType::Whitespace);
                let delim_opt = self.swallow(TokenType::Integer);
                if delim_opt.is_some() {
                    let delim = delim_opt.unwrap().content;
                    if delim == ";" {
                        return Ok(
                            Expression::StaticInteger(
                                int.as_str().parse::<i64>().unwrap()
                            )
                        );
                    }
                }
            }
        }
        Err(
            Error {
                kind: ErrorKind::Syntax,
                message: format!("Unknown error!"),
            }
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
    fn drop_cursor(&mut self) {
        let _ = self.cursor_stack.pop_front();
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
