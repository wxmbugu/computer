#![allow(dead_code)]
const MEM_SIZE: u64 = 65535;
pub const RAM_BASE: u64 = 0x0000CFFF;

#[derive(Debug)]
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
    pub fn store(&mut self, addr: u64, size: u64, value: u64) {
        if size == 16 {
            self.store_word(addr, value);
        };
    }
    // load byte from little endian
    pub fn load(&mut self, addr: u64, size: u64) -> Result<u64, ()> {
        match size {
            16 => Ok(self.load_word(addr)),
            _ => Err(()),
        }
    }

    fn load_word(&self, addr: u64) -> u64 {
        let index = (addr - RAM_BASE) as usize;
        (self.memory[index] as u64) | ((self.memory[index + 1] as u64) << 8)
    }

    fn store_word(&mut self, addr: u64, value: u64) {
        let index = (addr - RAM_BASE) as usize;
        self.memory[index] = (value & 0xff) as u8;
        self.memory[index + 1] = (value >> 8) as u8;
    }
}
