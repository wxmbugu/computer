#![allow(dead_code)]

use std::{collections::HashMap, fs::File, process::exit};

use crate::{
    instructions::Instructions,
    memory::{Ram, RAM_BASE},
    CompError,
};
#[derive(Debug, Clone)]
pub struct Cpu {
    pub register: [u32; 3],
    pub pc: u64,
    pub cond: u16,
    pub code: Ram,
    pub opcodes: Option<Instructions>,
}

impl Cpu {
    pub fn new(code: Vec<u8>) -> Cpu {
        let register: [u32; 3] = [1, 2, 3];
        let pc = RAM_BASE;
        let cond = 0;
        let code = Ram::new(code);
        let opcodes = Instructions::new();
        Self {
            register,
            pc,
            cond,
            code,
            opcodes,
        }
    }
    pub fn fetch(&self) -> Result<u64, CompError> {
        self.code.load(self.pc, 32)
    }

    pub fn execute(&mut self, instruction: u64) {
        let new_instruction = (instruction as u32).to_le_bytes();
        let instruction_to_be = as_u32_be(&new_instruction);
        let first_quater_bytes = instruction_to_be & 0xff;
        let memory_address = (instruction_to_be & 0xffff00) >> 8;
        let opcode = (first_quater_bytes & 0x0f) as u16;
        let register = ((first_quater_bytes & 0xf0) >> 4) as u16;
        match decode_opcode_instruction(opcode) {
            Instructions::HALT => {
                println!("exited");
                exit(0)
            }
            Instructions::NOP => {}
            Instructions::LI => {
                let reg = register - 1;
                let table_file = File::open("LabelTable.json").unwrap();
                let lookup: HashMap<String, u32> = serde_json::from_reader(table_file).unwrap();
                for value in lookup.values() {
                    if *value == memory_address {
                        let addr = memory_address;
                        self.register[reg as usize] = memory_address;
                        let instruction = self.code.load((addr - 2) as u64, 32).unwrap();
                        let _ = self.pc + 4;
                        self.execute(instruction);
                    }
                }
                self.register[reg as usize] = memory_address;
            }
            Instructions::LW => {
                let rg = (register - 1) as usize;
                let rg1 = (memory_address - 1) as usize;
                self.register[rg] = self.register[rg1];
            }
            Instructions::SW => {
                let content_register = (memory_address - 1) as usize;
                let contents = self.register[content_register];
                self.code.store(RAM_BASE, 32, contents as u64).unwrap();
            }
            Instructions::ADD => {
                let r_pos_2 = ((memory_address & 0x0f) - 1) as u16;
                let r_pos_3 = (((memory_address & 0xf0) >> 4) - 1) as u16;
                let r_pos_1 = register - 1;
                self.register[r_pos_3 as usize] =
                    self.register[r_pos_1 as usize] + self.register[r_pos_2 as usize];
            }
            Instructions::SUB => {
                let r_pos_2 = ((memory_address & 0x0f) - 1) as u16;
                let r_pos_3 = (((memory_address & 0xf0) >> 4) - 1) as u16;
                let r_pos_1 = register - 1;
                self.register[r_pos_3 as usize] =
                    self.register[r_pos_1 as usize] - self.register[r_pos_2 as usize];
            }
            Instructions::MULT => {
                let r_pos_2 = ((memory_address & 0x0f) - 1) as u16;
                let r_pos_3 = (((memory_address & 0xf0) >> 4) - 1) as u16;
                let r_pos_1 = register - 1;
                self.register[r_pos_3 as usize] =
                    self.register[r_pos_2 as usize] * self.register[r_pos_1 as usize];
            }
            Instructions::DIV => {
                let r_pos_2 = ((memory_address & 0x0f) - 1) as u16;
                let r_pos_3 = (((memory_address & 0xf0) >> 4) - 1) as u16;
                let r_pos_1 = register - 1;
                self.register[r_pos_3 as usize] =
                    self.register[r_pos_1 as usize] / self.register[r_pos_2 as usize];
            }
            Instructions::J => {
                self.pc = (memory_address - 5) as u64;
                let addr = self.pc + 4;
                let instruction = self.code.load(addr, 32).unwrap();
                self.execute(instruction)
            }
            Instructions::JR => {
                let r_pos_1 = register - 1;
                let addr = self.register[r_pos_1 as usize];
                self.pc = (addr - 5) as u64;
                let _ = self.pc + 4;
                let instruction = self.code.load(self.pc, 32).unwrap();
                self.execute(instruction)
            }
            Instructions::BEQ => {
                let r_pos_3 = ((memory_address & 0x0f) - 1) as u16;
                let r_pos_2 = (((memory_address & 0xf0) >> 4) - 1) as u16;
                let r_pos_1 = register - 1;
                let r1 = self.register[r_pos_1 as usize];
                let r2 = self.register[r_pos_2 as usize];
                let r3 = self.register[r_pos_3 as usize];
                if r1 == r2 {
                    let instruction = self.code.load((r3 - 2) as u64, 32).unwrap();
                    self.pc = (r3 - 5) as u64;
                    let _ = self.pc + 4;
                    self.execute(instruction);
                    self.cond = 1;
                }
            }
            Instructions::BNE => {
                let r_pos_2 = ((memory_address & 0x0f) - 1) as u16;
                let r_pos_3 = (((memory_address & 0xf0) >> 4) - 1) as u16;
                let r_pos_1 = register - 1;
                let r1 = self.register[r_pos_1 as usize];
                let r2 = self.register[r_pos_2 as usize];
                let r3 = self.register[r_pos_3 as usize];
                if r1 != r2 {
                    let instruction = self.code.load((r3 - 2) as u64, 32).unwrap();
                    self.pc = (r3 - 5) as u64;
                    let _ = self.pc + 4;
                    self.execute(instruction);
                    self.cond = 1;
                }
            }
            Instructions::INC => {
                let r_pos_1 = register - 1;
                self.register[r_pos_1 as usize] += 1;
            }
            Instructions::DEC => {
                let r_pos_1 = register - 1;
                self.register[r_pos_1 as usize] -= 1;
            }
            _ => unimplemented!(),
        };
    }

    fn decode_register_instruction(&self, register: u16) -> u32 {
        match register {
            1 => self.register[0],
            2 => self.register[1],
            3 => self.register[2],
            _ => unimplemented!(),
        }
    }
    pub fn dump_registers(&self) -> [u32; 3] {
        self.register
    }
}

fn decode_opcode_instruction(data: u16) -> Instructions {
    match data {
        0 => Instructions::HALT,
        1 => Instructions::NOP,
        2 => Instructions::LI,
        3 => Instructions::LW,
        4 => Instructions::SW,
        5 => Instructions::ADD,
        6 => Instructions::SUB,
        7 => Instructions::MULT,
        8 => Instructions::DIV,
        9 => Instructions::J,
        10 => Instructions::JR,
        11 => Instructions::BEQ,
        12 => Instructions::BNE,
        13 => Instructions::INC,
        14 => Instructions::DEC,
        _ => unimplemented!(),
    }
}
fn decode_register_instruction(data: u16) -> Instructions {
    match data {
        1 => Instructions::R1,
        2 => Instructions::R2,
        3 => Instructions::R3,
        _ => unimplemented!(),
    }
}

fn as_u32_be(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) << 24)
        + ((array[1] as u32) << 16)
        + ((array[2] as u32) << 8)
        + (array[3] as u32)
}
