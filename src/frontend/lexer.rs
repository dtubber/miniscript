use super::token::*;

use std::collections::VecDeque;

pub struct Lexer {
    chars: Vec<char>,
    cursor: usize,
    cursor_stack: VecDeque<usize>
}

impl Lexer {
    pub fn new(code: String) -> Self {
        Self {
            chars: code.chars().collect(),
            cursor: 0,
            cursor_stack: VecDeque::new()
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while !self.eoi() {
            let mut some = false;
            let mut token_opt;

            token_opt = self.swallow_whitespaces();
            if token_opt.is_some() {
                some |= true;
                tokens.push(token_opt.unwrap());
                continue;
            }

            token_opt = self.swallow_keyword();
            if token_opt.is_some() {
                some |= true;
                tokens.push(token_opt.unwrap());
                continue;
            }

            token_opt = self.swallow_word();
            if token_opt.is_some() {
                some |= true;
                tokens.push(token_opt.unwrap());
                continue;
            }

            token_opt = self.swallow_integer();
            if token_opt.is_some() {
                some |= true;
                tokens.push(token_opt.unwrap());
                continue;
            }

            token_opt = self.swallow_delimiter();
            if token_opt.is_some() {
                some |= true;
                tokens.push(token_opt.unwrap());
                continue;
            }

            token_opt = self.swallow_operator();
            if token_opt.is_some() {
                some |= true;
                tokens.push(token_opt.unwrap());
                continue;
            }

            if !some {
                break;
            }
        }

        tokens
    }

    fn eoi(&self) -> bool {
        self.cursor >= self.chars.len()
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

    fn swallow_comment(&mut self) {
        
    }

    fn swallow_whitespaces(&mut self) -> Option<Token> {
        let mut chars = String::new();
        loop {
            if self.eoi() {
                break;
            }
            let c = self.chars[self.cursor];
            if c.is_whitespace() {
                chars.push(c);   
                self.cursor += 1;
            } else {
                break;
            }
        }
        if chars.len() == 0 {
            return None;
        }

        let token = Token::new(
            TokenType::Whitespace,
            chars
        );
        Some(
            token
        )
    }

    fn swallow_word(&mut self) -> Option<Token> {
        let mut chars = String::new();
        loop {
            if self.eoi() {
                break;
            }
            let c = self.chars[self.cursor];
            if  c.is_alphanumeric() ||
                c == '_'            {
                chars.push(c);
                self.cursor += 1;
            } else {
                break;
            }
        }
        if chars.len() == 0 {
            return None;
        }

        let token = Token::new(
            TokenType::Word,
            chars
        );
        Some(
            token
        )
    }

    fn swallow_integer(&mut self) -> Option<Token> {
        let mut chars = String::new();
        loop {
            if self.eoi() {
                break;
            }
            let c = self.chars[self.cursor];
            if c.is_numeric() {
                chars.push(c);
                self.cursor += 1;
            } else {
                break;
            }
        }
        if chars.len() == 0 {
            return None;
        }
        let token = Token::new(
            TokenType::Integer,
            chars
        );
        Some(
            token
        )
    }

    fn swallow_delimiter(&mut self) -> Option<Token> {
        let allowed = vec![
            '(',
            ')',
            '{',
            '}',
            '.',
            ',',
            ':',
            ';'
        ];
        let mut chars = String::new();
        loop {
            if self.eoi() {
                break;
            }
            let c = self.chars[self.cursor];
            if allowed.contains(&c) {
                chars.push(c);
                self.cursor += 1;
                break;
            } else {
                break;
            }
        }

        if chars.len() == 0 {
            return None;
        }
        let token = Token::new(
            TokenType::Delimiter,
            chars
        );
        Some(
            token
        )
    }

    fn swallow_operator(&mut self) -> Option<Token> {
        let allowed = vec![
            '+',
            '-',
            '*',
            '/',
            '=',
            '>',
            '<',
            '!'
        ];
        let mut chars = String::new();

        loop {
            if self.eoi() {
                break;
            }
            let c = self.chars[self.cursor];
            if allowed.contains(&c) {
                chars.push(c);
                self.cursor += 1;
            } else {
                break;
            }
        }

        if chars.len() == 0 {
            return None;
        }
        let token = Token::new(
            TokenType::Operator,
            chars
        );
        Some(
            token
        )
    }

    fn swallow_keyword(&mut self) -> Option<Token> {
        let allowed = vec![
            "fn",
            "if",
            "while",
            "return",
            "break",
            "for",
            "loop",
            "else",
            "true",
            "false",
            "var",
            "const",
            "mod",
        ];

        self.push_cursor();

        let word_opt = self.swallow_word();
        if word_opt.is_none() {
            self.pop_cursor();
            return None;
        }

        let mut word = word_opt.unwrap();
        
        if !allowed.contains(&word.content.as_str()) {
            self.pop_cursor();
            return None;
        }

        word.token_type = TokenType::Keyword;
        
        Some(
            word
        )
    }
}
