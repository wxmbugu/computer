#![allow(dead_code)]

use crate::{
    instructions::Instructions,
    memory::{Ram, RAM_BASE},
};
#[derive(Debug)]
struct Cpu {
    register: [u8; 5],
    pc: u64,
    cond: bool,
    code: Ram,
    opcodes: Instructions,
}

impl Cpu {
    fn new(code: Vec<u8>) -> Cpu {
        let register: [u8; 5] = [1, 2, 3, 4, 5];
        let pc = RAM_BASE;
        let cond = false;
        let code = Ram::new(code);
        let opcodes = Instructions::new().unwrap();
        Self {
            register,
            pc,
            cond,
            code,
            opcodes,
        }
    }
    fn fetch(mut self, addr: u64, size: u64) -> u64 {
        self.code.load(addr, size).unwrap()
    }
    // fn execute(self) {
    //     match self.opcodes.match_instruction_opcode() {
    //         _ => todo!(),
    //     };
    // }

    fn dump_registers() {
        unimplemented!()
    }
}
