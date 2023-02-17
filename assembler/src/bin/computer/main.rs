use std::{fs::File, io::Read};

use assembler::{Cpu, RAM_BASE};
use clap::{Arg, Command};

fn main() {
    let cmd = Command::new("simulator")
        .author("Wxmbugu (https://github.com/wxmbugu)")
        .about("Execute a program")
        .version("0.0.1")
        .arg(
            Arg::new("input")
                .help("input binary")
                .short('i')
                .long("input"),
        )
        .get_matches();
    let input = cmd.get_one::<String>("input").unwrap();
    let mut bin_file = File::open(input).expect("No file found");
    let metadata = bin_file.metadata().expect("can't read file");
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
