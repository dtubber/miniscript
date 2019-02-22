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
    Reference(ExpressionBox),
    Equals(ExpressionBox, ExpressionBox),
    NotEquals(ExpressionBox, ExpressionBox),
    LessThan(ExpressionBox, ExpressionBox),
    GreaterThan(ExpressionBox, ExpressionBox),
    LessThanOrEquals(ExpressionBox, ExpressionBox),
    GreaterThanOrEquals(ExpressionBox, ExpressionBox),
    Variable(bool, String, String),
    NameAccess(ExpressionBox, ExpressionBox),
    MemberAccess(ExpressionBox, ExpressionBox),
    StaticInteger(i64),
    StaticFloat(f64),
    StaticString(String)
}

/**
 * Represents argument types
 */
pub enum Argument {
    Mutable(String, String),
    Constant(String, String)
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