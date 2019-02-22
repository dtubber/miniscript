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
            Opcode::LOAD => {
                let val = self.decode::<i64>();
                let destination = self.program[self.pc];
                self.pc += 1;
                self.registers[destination as usize].int = val;
            },
            Opcode::LOADF => {
                let val = self.decode::<f64>();
                let destination = self.program[self.pc];
                self.pc += 1;
                self.registers[destination as usize].float = val;
            },
            Opcode::MOV => {
                let size = self.program[self.pc];
                self.pc += 1;

                let source_addr = self.decode_address();
                let dest_addr = self.decode_address();

                let mut data = Vec::new();

                match source_addr.0 {
                    AddressLocation::Program => {
                        let mut addr = source_addr.1;
                        for i in 0..size {
                            data.push(self.program[(addr + (i as u64)) as usize]);
                            addr += i as u64;
                        }
                    },
                    AddressLocation::Register => {
                        let reg_bytes = unsafe { bincode::serialize(&self.registers[source_addr.0 as usize].int).unwrap() };
                        let len = if size > 4 { 4 } else { size };

                        for i in 0..len {
                            data.push(reg_bytes[i as usize]);
                        }
                    },
                    AddressLocation::Stack => {
                        let mut addr = source_addr.1;
                        for i in 0..size {
                            data.push(self.stack[(addr + (i as u64)) as usize]);
                            addr += i as u64;
                        }
                    },
                    _ => {
                        unimplemented!("Invalid memory access!");
                    }
                };

                match dest_addr.0 {
                    AddressLocation::Register => {
                        if data.len() != 4 {
                            data.resize(4, 0);
                        }
                        let reg_int = bincode::deserialize::<i64>(&data).unwrap();
                        self.registers[dest_addr.1 as usize].int = reg_int;
                    },
                    AddressLocation::Stack => {
                        let mut addr = dest_addr.1;
                        for i in 0..size {
                            self.stack[(addr + i as u64) as usize] = data[i as usize];
                            addr += i as u64;
                        }
                    },
                    _ => {
                        unimplemented!("Invalid memory access!");
                    }
                };
            },
            Opcode::PUSH => {
                let push_size = self.decode::<u64>();
                let stack_diff = (self.sp + push_size as usize) - self.stack.len();
                if stack_diff > 0 {
                    self.stack.resize(self.stack.len() + (stack_diff * 2), 0);
                }
                self.sp += push_size as usize;
            },
            Opcode::POP => {
                let pop_size = self.decode::<u64>();
                self.sp -= pop_size as usize;
            },
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
        let address_raw = self.decode::<u64>();
        let location_raw = (address_raw >> 61) as u8;
        let location = AddressLocation::from(&location_raw);
        let address = (address_raw << 3) >> 3;
        (location, address)
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
