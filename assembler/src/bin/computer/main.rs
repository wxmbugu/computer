#![allow(unused_variables)]
#![allow(unused_assignments)]
use std::{
    fs::{self, File},
    io::{BufReader, Read},
};

use assembler::{Cpu, RAM_BASE};

fn main() {
    let mut bin_file = File::open("test.txt").expect("No file found");
    let reader_bin = BufReader::new(&bin_file);
    let metadata = fs::metadata("test.txt").expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    bin_file.read_exact(&mut buffer).expect("buffer overflow");
    let mut address: Vec<u64> = vec![];
    for i in 0..buffer.len() {
        address.push(RAM_BASE + (i as u64));
    }
    let mut cpu = Cpu::new(buffer.to_owned());
    loop {
        let instruction = cpu.fetch().unwrap();
        cpu.pc += 4;
        // let pos = 0;
        cpu.execute(instruction);

        println!(
            "registers = {:?} pc = {} cond = { }",
            cpu.dump_registers(),
            cpu.pc,
            cpu.cond
        );
    }
}
