/**
 * Defines an expression.
 */
pub enum Expr {
    StaticInt(i64),
    StaticFloat(i64),
    StaticBool(bool),
    StaticString(String),
    Variable(String),
    Expr(Box<Expr>),
    BinaryOp(Box<Expr>, BinOp, Box<Expr>),
    UnaryOp(UnOp, Box<Expr>)
}

/**
 * Defines a binary operator.
 */
pub enum BinOp {
    Plus,
    Minus,
    Multiply,
    Divide,
    MemberAccess,
}

/**
 * Defines a unary operator.
 */
pub enum UnOp {
    Reference,
    Negate,
    Minus
}

/**
 * Defines a statement.
 */
pub enum Statement {
    Call,
    Assignment,
    Loop(Vec<Statement>),
    While(Box<Expr>, Vec<Statement>)
}

/**
 * Defines a top level declaration.
 */
pub enum Declaration {
    Function(bool, String),
    Module(bool, String),
    Struct(bool, String, Vec<StructDeclaration>)
}

/**
 * Defines declarations only valid inside a struct declaration
 */
pub enum StructDeclaration {
    MemberVariable(bool, String, String),
    MemberFunction(bool, String)
}