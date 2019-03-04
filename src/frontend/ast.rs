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
    NameAccess(ExpressionBox, ExpressionBox),
    MemberAccess(ExpressionBox, ExpressionBox),
    IfExpr(ExpressionBox, Vec<Statement>),
    ElseIfExpr(ExpressionBox, Vec<Statement>),
    ElseExpr(ExpressionBox, Vec<Statement>),
    StaticInteger(i64),
    StaticFloat(f64),
    StaticString(String),
    Variable(String),
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
    Assign(String, Expression),
    VariableDecl(VariableDef),
    VariableDeclAssign(VariableDef, Expression),
    While(Expression, Vec<Statement>),
    If(Expression, Vec<Statement>),
    ElseIf(Expression, Vec<Statement>, Vec<Statement>),
    Call(String, Vec<Argument>)
}

pub enum Declaration {
    Function(String, String, Vec<Argument>, Vec<Statement>),
    Module(String, Vec<Declaration>),
    Struct(String, Vec<MemberDeclaration>)
}

pub enum MemberDeclaration {
    Variable(bool, String, String),
    Method(bool, String, Vec<Argument>, Vec<Statement>)
}

pub struct Program {
    pub declarations: Vec<Declaration>
}

pub struct VariableDef {
    pub constant: bool,
    pub name: String,
    pub vartype: String
}

impl Declaration {
    pub fn print(&self, n: u32) {
        match self {
            Declaration::Function(name, typename, arguments, statements) => {
                for i in 0..n {
                    print!("\t");
                }
                print!("fn: {}(", name);
                let mut i = 0;
                for arg in arguments.iter() {
                    match arg {
                        Argument::Constant(name, typename) => {
                            print!("const:{} {}", typename, name);
                        },
                        Argument::Mutable(name, typename) => {
                            print!("var:{} {}", typename, name);
                        }
                    };
                    if i != arguments.len() - 1 {
                        print!(", ");
                    }
                    i += 1;
                }
                print!(")");
                if !typename.is_empty() {
                    print!(": {}", typename);
                }
                println!(";");
            },
            Declaration::Module(name, declarations) => {
                for i in 0..n {
                    print!("\t");
                }
                println!("mod: {}", name);
                for declaration in declarations.iter() {
                    declaration.print(n + 1);
                }
            },
            _ => {}
        };
    }
}