use serde_derive::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Opcode {
    HLT = 0,
    ADD_I64,
    SUB_I64,
    MUL_I64,
    DIV_I64,
    ADD_F64,
    SUB_F64,
    MUL_F64,
    DIV_F64,
    JMP,
    JMP_T,
    JMP_F,
    JMPB,
    JMPB_T,
    JMPB_F,
    JMPF,
    JMPF_T,
    JMPF_F,
    CALL,
    RET,
    MOV,
    MOVR,
    PUSH,
    POP
}

impl From<&u8> for Opcode {
    fn from(data: &u8) -> Self {
        match *data {
            0 => Opcode::HLT,
            1 => Opcode::ADD_I64,
            2 => Opcode::SUB_I64,
            3 => Opcode::MUL_I64,
            4 => Opcode::DIV_I64,
            5 => Opcode::ADD_F64,
            6 => Opcode::SUB_F64,
            7 => Opcode::MUL_F64,
            8 => Opcode::DIV_F64,
            9 => Opcode::JMP,
            10 => Opcode::JMP_T,
            11 => Opcode::JMP_F,
            12 => Opcode::JMPB,
            13 => Opcode::JMPB_T,
            14 => Opcode::JMPB_F,
            15 => Opcode::JMPF,
            16 => Opcode::JMPF_T,
            17 => Opcode::JMPF_F,
            18 => Opcode::CALL,
            19 => Opcode::RET,
            20 => Opcode::MOV,
            21 => Opcode::PUSH,
            22 => Opcode::POP,
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
