use serde_derive::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Opcode {
    HLT = 0,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    LOADF,
    ADDF,
    SUBF,
    MULF,
    DIVF,
    ITOF,
    FTOI,
    CALL,
    RET,
    MOV,
    PUSH,
    POP,
    JMP,
    JMP_B,
    JMP_F,
    JT,
    JT_B,
    JT_F,
    JF,
    JF_B,
    JF_F,
    EQ,
    NEQ,
    LT,
    GT,
    GEQ,
    LEQ,
    EQF,
    NEQF,
    LTF,
    GTF,
    GEQF,
    LEQF,
    AND,
    OR,
    NOT
}

impl From<&u8> for Opcode {
    fn from(data: &u8) -> Self {
        match *data {
            _ => Opcode::HLT
        }
    }
}

#[derive(PartialEq)]
pub struct Instruction {
    pub opcode: Opcode
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Self {
        Self {
            opcode: opcode
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hlt_opcode() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }
}

#[derive(PartialEq, Serialize, Deserialize)]
pub enum AddressLocation {
    Program = 0,
    Register,
    Stack,
    Heap,
    External
}

impl From<&u8> for AddressLocation {
    fn from(data: &u8) -> Self {
        match *data {
            0 => {
                AddressLocation::Program
            },
            1 => {
                AddressLocation::Register
            },
            2 => {
                AddressLocation::Stack
            },
            3 => {
                AddressLocation::Heap
            },
            4 => {
                AddressLocation::External
            },
            _ => {
                AddressLocation::External
            }
        }
    }
}
