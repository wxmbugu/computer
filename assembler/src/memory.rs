#![allow(dead_code)]

use crate::CompError;
const MEM_SIZE: u64 = 65535;
pub const RAM_BASE: u64 = 0x0000CFFF;

#[derive(Debug, Clone)]
pub struct Ram {
    memory: Vec<u8>,
}

impl Ram {
    pub fn new(code: Vec<u8>) -> Ram {
        let mut ram = vec![0; MEM_SIZE as usize];
        ram.splice(..code.len(), code.iter().cloned());
        Self { memory: ram }
    }

    /// store bytes in little endian .
    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), CompError> {
        match size {
            16 => {
                self.store_word(addr, value);
                Ok(())
            }
            32 => {
                self.store_double_word(addr, value);
                Ok(())
            }
            _ => Err(CompError::InvalidSize),
        }
    }
    // load byte from little endian
    pub fn load(&self, addr: u64, size: u64) -> Result<u64, CompError> {
        match size {
            16 => Ok(self.load_word(addr)),
            32 => Ok(self.load_double_word(addr)),
            _ => Err(CompError::InvalidSize),
        }
    }
    fn load_bytes(&self, addr: u64) -> u64 {
        let index = (addr - RAM_BASE) as usize;
        self.memory[index] as u64
    }

    fn store_bytes(&mut self, addr: u64, value: u64) {
        let index = (addr - RAM_BASE) as usize;
        self.memory[index] = value as u8;
    }
    fn load_word(&self, addr: u64) -> u64 {
        let index = (addr - RAM_BASE) as usize;
        (self.memory[index] as u64)
            | ((self.memory[index + 1] as u64) << 8)
            | ((self.memory[index + 2] as u64) << 16)
            | ((self.memory[index + 3] as u64) << 24)
    }

    fn store_word(&mut self, addr: u64, value: u64) {
        let index = (addr - RAM_BASE) as usize;
        self.memory[index] = (value & 0xff) as u8;
        self.memory[index + 1] = (value >> 8) as u8;
    }
    fn load_double_word(&self, addr: u64) -> u64 {
        let index = (addr - RAM_BASE) as usize;
        (self.memory[index] as u64)
            | ((self.memory[index + 1] as u64) << 8)
            | ((self.memory[index + 2] as u64) << 16)
            | ((self.memory[index + 3] as u64) << 24)
    }
    fn store_double_word(&mut self, addr: u64, value: u64) {
        let index = (addr - RAM_BASE) as usize;
        self.memory[index] = (value & 0xff) as u8;
        self.memory[index + 1] = ((value >> 8) & 0xff) as u8;
        self.memory[index + 2] = ((value >> 16) & 0xff) as u8;
        self.memory[index + 3] = ((value >> 24) & 0xff) as u8;
    }
}
