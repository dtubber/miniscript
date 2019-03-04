pub mod compiler;

use compiler::*;
use crate::vm::instruction::*;
use crate::frontend::ast::*;


pub trait BytecodeCodegen {
    fn codegen(&mut self, compiler: &mut Compiler) {

    }
}

impl BytecodeCodegen for Expression {
    fn codegen(&mut self, compiler: &mut Compiler) {
        match self {
            Expression::Addition(lhs, rhs) => {
                lhs.codegen(compiler);
                rhs.codegen(compiler);
            },
            Expression::StaticFloat(float) => {
                let mut prg = vec![];
                prg.push(Opcode::LDF as u8);
                prg.append(&mut bincode::serialize(float).unwrap());
            },
            Expression::StaticInteger(int) => {
                let mut prg = vec![];
                prg.push(Opcode::LDI as u8);
                prg.append(&mut bincode::serialize(int).unwrap());
            },
            Expression::Equals(lhs, rhs) => {

            },
            _ => {

            }
        };
    }
}