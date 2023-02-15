#![allow(unused_variables)]
#![allow(unused_assignments)]
use std::{
    fs::{self, File},
    io::{BufReader, Read},
};

use assembler::Ram;

fn main() {
    let mut ok: Vec<u16> = vec![];
    let mut bin_file = File::open("test.txt").expect("No file found");
    let reader_bin = BufReader::new(&bin_file);
    let metadata = fs::metadata("test.txt").expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    bin_file.read_exact(&mut buffer).expect("buffer overflow");

    unsafe {
        ok = (buffer[..].align_to::<u16>().1).to_vec();
    }

    for e in ok.iter() {
        let value = e >> 8 | (e & 0xff) << 8;
    }

    ok = ok.iter().map(|x| x >> 8 | (x & 0xff) << 8).collect();
    println!("{ok:#?}");
    let code = vec![];
    let ram = Ram::new(code);
    // ram.load(addr, size)
}
