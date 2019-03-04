use std::vec::Vec;
use std::collections::HashMap;

pub struct Compiler {
    pub data: Vec<u8>,
    pub program: Vec<u8>,
    pub data_labels: HashMap<String, u64>,
    pub program_labels: HashMap<String, u64>
}

impl Compiler {
    pub fn append_data(&mut self, data: &mut Vec<u8>){
        self.data.append(data);
    }

    pub fn append_program(&mut self, program: &mut Vec<u8>) {
        self.program.append(program);
    }

    pub fn label_data(&mut self, key: String) -> u64 {
        self.data_labels.insert(key, self.data.len() as u64);
        self.data.len() as u64
    }

    pub fn label_program(&mut self, key: String) -> u64 {
        self.program_labels.insert(key, self.program.len() as u64);
        self.program.len() as u64
    }
}
