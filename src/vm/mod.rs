pub mod instruction;

use instruction::*;

pub mod prelude {
    pub use super::instruction::*;
    pub use super::*;
}

use std::collections::VecDeque;

pub struct VM {
    pub registers: [u64; 32],
    pub pc: usize,
    pub program: Vec<u8>,
    pub stack: VecDeque<u8>,
    pub call_stack: VecDeque<u64>
}

impl VM {
    pub fn new() -> Self {
        Self {
            registers: [0; 32],
            pc: 0,
            program: Vec::new(),
            stack: VecDeque::new(),
            call_stack: VecDeque::new()
        }
    }

    pub fn run(&mut self) {
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
            Opcode::IGL => {},
            Opcode::LOAD_U64 => {
                let register = self.decode::<u8>();
                let int = self.decode::<u64>();
                self.load_register(register, int);
            },
            Opcode::ADD_U64 => {
                let source1 = self.decode::<u8>();
                let source2 = self.decode::<u8>();
                let sink = self.decode::<u8>();
                let val1: u64 = self.get_register(source1);
                let val2: u64 = self.get_register(source2);
                let val = val1 + val2;
                self.load_register(sink, val);
            },
            Opcode::SUB_U64 => {
                let source1 = self.decode::<u8>();
                let source2 = self.decode::<u8>();
                let sink = self.decode::<u8>();
                let val1: u64 = self.get_register(source1);
                let val2: u64 = self.get_register(source2);
                let val = val1 - val2;
                self.load_register(sink, val);
            },
            Opcode::DIV_U64 => {
                let source1 = self.decode::<u8>();
                let source2 = self.decode::<u8>();
                let sink = self.decode::<u8>();
                let val1: u64 = self.get_register(source1);
                let val2: u64 = self.get_register(source2);
                let val = val1 / val2;
                self.load_register(sink, val);
            },
            Opcode::MUL_U64 => {
                let source1 = self.decode::<u8>();
                let source2 = self.decode::<u8>();
                let sink = self.decode::<u8>();
                let val1: u64 = self.get_register(source1);
                let val2: u64 = self.get_register(source2);
                let val = val1 * val2;
                self.load_register(sink, val);
            },
            Opcode::LOAD_I64 => {
                let register = self.decode::<u8>();
                let int = self.decode::<i64>();
                self.load(register, int);
            },
            Opcode::ADD_I64 => {
                let source1 = self.decode::<u8>();
                let source2 = self.decode::<u8>();
                let sink = self.decode::<u8>();
                let val1 = self.get::<i64>(source1);
                let val2 = self.get::<i64>(source2);
                let val = val1 + val2;
                self.load(sink, val);
            },
            Opcode::SUB_I64 => {
                let source1 = self.decode::<u8>();
                let source2 = self.decode::<u8>();
                let sink = self.decode::<u8>();
                let val1 = self.get::<i64>(source1);
                let val2 = self.get::<i64>(source2);
                let val = val1 - val2;
                self.load(sink, val);
            },
            Opcode::DIV_I64 => {
                let source1 = self.decode::<u8>();
                let source2 = self.decode::<u8>();
                let sink = self.decode::<u8>();
                let val1 = self.get::<i64>(source1);
                let val2 = self.get::<i64>(source2);
                let val = val1 / val2;
                self.load(sink, val);
            },
            Opcode::MUL_I64 => {
                let source1 = self.decode::<u8>();
                let source2 = self.decode::<u8>();
                let sink = self.decode::<u8>();
                let val1 = self.get::<i64>(source1);
                let val2 = self.get::<i64>(source2);
                let val = val1 * val2;
                self.load(sink, val);
            },
            Opcode::JMP => {
                let destination = self.decode::<u64>();
                self.pc = destination as usize;
            },
            Opcode::JMP_T => {
                let destination = self.decode::<u64>();
                let source = self.decode::<u8>();
                let value = self.get::<bool>(source);
                if value {
                    self.pc = destination as usize;
                }
            },
            Opcode::JMP_F => {
                let destination = self.decode::<u64>();
                let source = self.decode::<u8>();
                let value = self.get::<bool>(source);
                if !value {
                    self.pc = destination as usize;
                }
            },
            Opcode::JMPB => {
                let destination = self.decode::<u64>();
                self.pc -= destination as usize;
            },
            Opcode::JMPB_T => {
                let destination = self.decode::<u64>();
                let source = self.decode::<u8>();
                let value = self.get::<bool>(source);
                if value {
                    self.pc -= destination as usize;
                }
            },
            Opcode::JMPB_F => {
                let destination = self.decode::<u64>();
                let source = self.decode::<u8>();
                let value = self.get::<bool>(source);
                if !value {
                    self.pc -= destination as usize;
                }
            },
            Opcode::JMPF => {
                let destination = self.decode::<u64>();
                self.pc += destination as usize;
            },
            Opcode::JMPF_T => {
                let destination = self.decode::<u64>();
                let source = self.decode::<u8>();
                let value = self.get::<bool>(source);
                if value {
                    self.pc += destination as usize;
                }
            },
            Opcode::JMPF_F => {
                let destination = self.decode::<u64>();
                let source = self.decode::<u8>();
                let value = self.get::<bool>(source);
                if !value {
                    self.pc += destination as usize;
                }
            },
            Opcode::CALL => {
                let destination = self.decode::<u64>();
                let old_pc = self.pc as u64;
                self.call_stack.push_front(old_pc);
                self.pc = destination as usize;
            },
            Opcode::RET => {
                let old_pc_opt = self.call_stack.pop_front();
                if old_pc_opt.is_some() {
                    self.pc = old_pc_opt.unwrap() as usize;
                }
            }
        };
        false
    }

    fn decode_opcode(&mut self) -> Opcode {
        let byte = self.program[self.pc];
        self.pc += 1;
        Opcode::from(&byte)
    }

    fn decode<T : Clone>(&mut self) -> T {
        let size = std::mem::size_of::<T>();
        let program_ptr = self.program.as_slice()[self.pc..(self.pc + size)].as_ptr();
        let slice = unsafe {
            std::slice::from_raw_parts(program_ptr as *const T, 1)
        };
        self.pc += size;
        slice[0].clone()
    }

    fn load<T>(&mut self, register: u8, data: T) {
        let ptr: *mut T = unsafe {
            std::mem::transmute(&self.registers[register as usize])
        };
        unsafe {
            *ptr = data;
        }
    }

    fn get<T: Clone>(&mut self, register: u8) -> T {
        let ptr: *const T = unsafe {
            std::mem::transmute(&self.registers[register as usize])
        };
        unsafe {
            (*ptr).clone()
        }
    }

    fn load_register(&mut self, register: u8, data: u64) {
        self.registers[register as usize] = data;
    }

    fn get_register(&mut self, register: u8) -> u64 {
        self.registers[register as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let vm = VM::new();
        assert_eq!(vm.registers[0], 0);
        assert_eq!(vm.registers[31], 0);
    }
}
