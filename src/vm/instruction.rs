use serde_derive::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Opcode {
    HLT = 0,
    LDI = 1,
    LDB = 2,
    LDF = 3,
    ADD = 4,
    SUB = 5,
    MUL = 6,
    DIV = 7,
    ADDF = 8,
    SUBF = 9,
    MULF = 10,
    DIVF = 11,
    ITOF = 12,
    FTOI = 13,
    CALL = 14,
    RET = 15,
    MOVR_R = 16,
    MOVR_P = 17,
    MOVR_S = 18,
    MOVR_H = 19,
    MOVH_H = 20,
    MOVH_R = 21,
    MOVH_S = 22,
    MOVH_P = 23,
    MOVS_S = 24,
    MOVS_P = 25,
    MOVS_R = 26,
    MOVS_H = 27,
    PUSH = 28,
    POP = 29,
    JMP = 30,
    JMP_B = 31,
    JMP_F = 32,
    JT = 33,
    JT_B = 34,
    JT_F = 35,
    JF = 36,
    JF_B = 37,
    JF_F = 38,
    EQ = 39,
    NEQ = 40,
    LT = 41,
    GT = 42,
    GEQ = 43,
    LEQ = 44,
    EQF = 45,
    NEQF = 46,
    LTF = 47,
    GTF = 48,
    GEQF = 49,
    LEQF = 50,
    AND = 51,
    OR = 52,
    NOT = 53
}

impl From<&u8> for Opcode {
    fn from(data: &u8) -> Self {
        bincode::deserialize(&[*data]).unwrap()
    }
}

impl Into<u8> for Opcode {
    fn into(self) -> u8 {
        self as u8
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
