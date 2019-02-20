pub mod instruction;

pub mod prelude {
    pub use super::instruction::*;
    pub use super::*;
}
use std::collections::VecDeque;

use serde::de::DeserializeOwned;

use instruction::*;

pub struct VM {
    pub registers: [u64; 32],
    pub pc: usize,
    pub sp: usize,
    pub program: Vec<u8>,
    pub stack: Vec<u8>,
    pub call_stack: VecDeque<usize>
}

impl VM {
    pub fn new(stack_size: usize) -> Self {
        let mut stack = Vec::new();
        stack.resize(stack_size, 0);
        Self {
            registers: [0; 32],
            pc: 0,
            sp: 0,
            program: Vec::new(),
            stack: stack,
            call_stack: VecDeque::new()
        }
    }

    pub fn run(&mut self) {
        let mut done = self.pc >= self.program.len();
        while !done {
            done = self.execute_instruction();
        }
    }

    pub fn run_from(&mut self, offset: usize) {
        self.pc = offset;
        let mut done = self.pc >= self.program.len();
        while !done {
            done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }
        let opcode = self.decode_opcode();
        match opcode {
            Opcode::HLT => {
                return true;
            },
            Opcode::MOV => {},
            Opcode::PUSH => {},
            Opcode::POP => {},
            _ => {
                unimplemented!("Opcode \"{:X}\" not implemented!", opcode as u8);
            }
        };
        false
    }

    fn decode_opcode(&mut self) -> Opcode {
        let byte = self.program[self.pc];
        self.pc += 1;
        Opcode::from(&byte)
    }

    fn decode_address(&mut self) -> (AddressLocation, u64) {
        let offset = 64 - 3;
        let address_raw = self.decode::<u64>();
        let location_raw = (address_raw >> offset) as u8;
        let address = (address_raw << 3) >> 3;
        let location = AddressLocation::from(&location_raw);
        (location, address)
    }

    fn decode_at<T: DeserializeOwned>(&mut self, pos: usize) -> T {
        let size = std::mem::size_of::<T>();
        let mut bytes = Vec::new();
        for i in 0..size {
            bytes.push(self.program[pos + i]);
        }
        bincode::deserialize(&bytes).unwrap()
    }

    fn decode<T: DeserializeOwned>(&mut self) -> T {
        let size = std::mem::size_of::<T>();
        let mut bytes = Vec::new();
        for i in 0..size {
            bytes.push(self.program[self.pc + i]);
        }
        self.pc += size;
        bincode::deserialize(&bytes).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let vm = VM::new(2^8);
        assert_eq!(vm.registers[0], 0);
        assert_eq!(vm.registers[31], 0);
    }
}
