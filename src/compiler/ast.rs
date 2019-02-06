pub type ExpressionBox = Box<Expression>;

/**
 * Represents an Expression in the AST
 */
pub enum Expression {
    Addition(ExpressionBox, ExpressionBox),
    Subtraction(ExpressionBox, ExpressionBox),
    Multiplication(ExpressionBox, ExpressionBox),
    Division(ExpressionBox, ExpressionBox),
    BinaryNot(ExpressionBox),
    Equals(ExpressionBox, ExpressionBox),
    NotEquals(ExpressionBox, ExpressionBox),
    LessThan(ExpressionBox, ExpressionBox),
    GreaterThan(ExpressionBox, ExpressionBox),
    LessThanOrEquals(ExpressionBox, ExpressionBox),
    GreaterThanOrEquals(ExpressionBox, ExpressionBox),
    Variable(String),
    Constant(isize),
}

/**
 * Represents argument types
 */
pub enum Argument {
    ByValue(Expression),
    ByReference(String)
}

/**
 * Represents a statement
 */
pub enum Statement {
    Variable(String),
    Assign(String, Expression),
    While(Expression, Vec<Statement>),
    If(Expression, Vec<Statement>),
    ElseIf(Expression, Vec<Statement>, Vec<Statement>),
    Call(String, Vec<Argument>)
}

pub enum Declaration {
    Function(String, Vec<String>, Vec<Statement>)
}

pub struct Program {
    pub decl: Vec<Declaration>
}