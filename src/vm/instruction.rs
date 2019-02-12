#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Opcode {
    HLT,
    IGL,
    LOAD_U64,
    ADD_U64,
    SUB_U64,
    MUL_U64,
    DIV_U64,
    LOAD_I64,
    ADD_I64,
    SUB_I64,
    MUL_I64,
    DIV_I64,
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
    RET
}

impl From<&u8> for Opcode {
    fn from(data: &u8) -> Self {
        match *data {
            0 => Opcode::HLT,
            1 => Opcode::IGL,
            2 => Opcode::ADD_U64,
            3 => Opcode::SUB_U64,
            4 => Opcode::MUL_U64,
            5 => Opcode::DIV_U64,
            6 => Opcode::ADD_I64,
            7 => Opcode::SUB_I64,
            8 => Opcode::MUL_I64,
            9 => Opcode::DIV_I64,
            10 => Opcode::JMP,
            11 => Opcode::JMP_T,
            12 => Opcode::JMP_F,
            13 => Opcode::JMPB,
            14 => Opcode::JMPB_T,
            15 => Opcode::JMPB_F,
            16 => Opcode::JMPF,
            17 => Opcode::JMPF_T,
            18 => Opcode::JMPF_F,
            19 => Opcode::CALL,
            20 => Opcode::RET,
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
