pub mod instruction;

pub mod prelude {
    pub use super::instruction::*;
    pub use super::*;
}
use std::collections::VecDeque;

use serde::de::DeserializeOwned;

use instruction::*;

#[derive(Copy, Clone)]
pub union Reg {
    pub float: f64,
    pub int: i64,
    pub byte: u8,
    pub boolean: bool,
    pub bytes: [u8; 8]
}

impl Reg {
    pub fn new() -> Self {
        Self {
            int: 0
        }
    }

    pub fn clear(&mut self) {
        self.int = 0;
    }
}

pub struct VM {
    pub registers: [Reg; 16],
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
            registers: [Reg::new(); 16],
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
            Opcode::LDB => {
                let dest = self.decode_byte();
                let byte = self.decode_byte();
                self.registers[dest as usize].byte = byte;
            },
            Opcode::LDI => {
                let dest = self.decode_byte();
                let int = self.decode::<i64>();
                self.registers[dest as usize].int = int;
            },
            Opcode::LDF => {
                let dest = self.decode_byte();
                let float = self.decode::<f64>();
                self.registers[dest as usize].float = float;
            },
            Opcode::MOVR_R => {
                let dest = self.decode_byte();
                let source = self.decode_byte();
                self.registers[dest as usize] = self.registers[source as usize];
            },
            Opcode::MOVR_P => {
                let dest = self.decode_byte();
                let source = self.decode::<u64>();
                let mut size = self.decode_byte() as u64;
                size = if size > 8 { 8 } else { size };
                let mut buf = [0; 8];
                for i in 0..size - 1 {
                    let addr = (source + i) as usize;
                    buf[i as usize] = self.program[addr];
                }
                self.registers[dest as usize].bytes = buf;
            },
            Opcode::MOVR_S => {
                let dest = self.decode_byte();
                let source = self.decode::<u64>();
                let mut size = self.decode_byte() as u64;
                size = if size > 8 { 8 } else { size };
                let mut buf = [0; 8];
                for i in 0..size - 1 {
                    let addr = (source + i) as usize;
                    buf[i as usize] = self.stack[addr];
                }
                self.registers[dest as usize].bytes = buf;
            },
            Opcode::MOVR_H => {
                unimplemented!("Heap not yet implemented!");
            },
            Opcode::MOVS_R => {
                let mut dest = self.decode::<u64>();
                let source = self.decode_byte();
                let mut size = self.decode_byte() as u64;
                size = if size > 8 { 8 } else { size };
                for i in 0..size - 1 {
                    self.stack[dest as usize] = unsafe { self.registers[source as usize].bytes[i as usize] };
                    dest += 1;
                }
            },
            Opcode::MOVS_P => {
                let mut dest = self.decode::<u64>();
                let mut source = self.decode::<u64>();
                let size = self.decode_byte() as u64;
                for i in 0..size - 1 {
                    self.stack[dest as usize] = self.program[source as usize];
                    dest += 1;
                    source += 1;
                }
            },
            Opcode::MOVS_S => {
                unimplemented!("Stack copy not yet implemented!");
            },
            Opcode::MOVS_H => {
                unimplemented!("Heap not yet implemented!");
            },
            Opcode::MOVH_R => {
                unimplemented!("Heap not yet implemented!");
            },
            Opcode::MOVH_P => {
                unimplemented!("Heap not yet implemented!");
            },
            Opcode::MOVH_S => {
                unimplemented!("Heap not yet implemented!");
            },
            Opcode::MOVH_H => {
                unimplemented!("Heap not yet implemented!");
            },
            Opcode::ADD => {
                let dest = self.decode_byte();
                let lhs = self.decode_byte();
                let rhs = self.decode_byte();
                self.registers[dest as usize].int = unsafe { self.registers[lhs as usize].int + self.registers[rhs as usize].int };
            },
            Opcode::SUB => {
                let dest = self.decode_byte();
                let lhs = self.decode_byte();
                let rhs = self.decode_byte();
                self.registers[dest as usize].int = unsafe { self.registers[lhs as usize].int - self.registers[rhs as usize].int };
            },
            Opcode::MUL => {
                let dest = self.decode_byte();
                let lhs = self.decode_byte();
                let rhs = self.decode_byte();
                self.registers[dest as usize].int = unsafe { self.registers[lhs as usize].int * self.registers[rhs as usize].int };
            },
            Opcode::DIV => {
                let dest = self.decode_byte();
                let lhs = self.decode_byte();
                let rhs = self.decode_byte();
                self.registers[dest as usize].int = unsafe { self.registers[lhs as usize].int / self.registers[rhs as usize].int };
            },
            Opcode::ADDF => {
                let dest = self.decode_byte();
                let lhs = self.decode_byte();
                let rhs = self.decode_byte();
                self.registers[dest as usize].float = unsafe { self.registers[lhs as usize].float + self.registers[rhs as usize].float };
            },
            Opcode::SUBF => {
                let dest = self.decode_byte();
                let lhs = self.decode_byte();
                let rhs = self.decode_byte();
                self.registers[dest as usize].float = unsafe { self.registers[lhs as usize].float - self.registers[rhs as usize].float };
            },
            Opcode::MULF => {
                let dest = self.decode_byte();
                let lhs = self.decode_byte();
                let rhs = self.decode_byte();
                self.registers[dest as usize].float = unsafe { self.registers[lhs as usize].float * self.registers[rhs as usize].float };
            },
            Opcode::DIVF => {
                let dest = self.decode_byte();
                let lhs = self.decode_byte();
                let rhs = self.decode_byte();
                self.registers[dest as usize].float = unsafe { self.registers[lhs as usize].float / self.registers[rhs as usize].float };
            },
            _ => {
                unimplemented!("Opcode \"{:X}\" not implemented!", opcode as u8);
            }
        };
        false
    }

    #[inline]
    fn decode_opcode(&mut self) -> Opcode {
        let byte = self.program[self.pc];
        self.pc += 1;
        Opcode::from(&byte)
    }
    
    #[inline]
    fn decode_byte(&mut self) -> u8 {
        let byte = self.program[self.pc];
        self.pc += 1;
        byte
    }
 
    #[inline]
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
