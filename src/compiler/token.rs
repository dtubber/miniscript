#[derive(PartialEq, Clone)]
pub enum TokenType {
    Whitespace,
    Keyword,
    Word,
    Integer,
    Operator,
    Delimiter
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub content: String
}

impl Token {
    pub fn new(token_type: TokenType, content: String) -> Self {
        Self {
            token_type: token_type,
            content: content
        }
    }
}