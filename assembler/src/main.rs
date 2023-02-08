pub mod instructions;
use bytes::BufMut;
use instructions::*;
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

use clap::{Arg, Command};
fn main() {
    let cmd = Command::new("assembler")
        .author("Wxmbugu (https://github.com/wxmbugu)")
        .about("Generate a bin file to pesapal isa")
        .version("0.0.1")
        .arg(
            Arg::new("input")
                .help("input assembly filename")
                .short('i')
                .long("input"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .help("output binary filename")
                .long("output")
                .default_value("out.bin"),
        )
        .get_matches();
    let input = cmd.get_one::<String>("input").unwrap();
    let output = cmd.get_one::<String>("output").unwrap();
    let file = File::open(input).unwrap();
    let mut output_file = File::create(output).unwrap();
    let reader = BufReader::new(file);
    // remove comments
    let data = reader
        .lines()
        .filter(|x| {
            x.as_ref()
                .expect("nothing")
                .trim()
                .strip_prefix(';')
                .is_none()
        })
        .flatten()
        .collect::<Vec<String>>();
    // vex without loop value
    let cleaned_data = data
        .iter()
        .map(|x| {
            x.to_string()
                .replace("loop", " ")
                .replace("loop:", " ")
                .replace(':', "\t")
                .trim()
                .to_string()
        })
        .filter(|x| !x.is_empty())
        .collect::<Vec<String>>();
    let mut buf = vec![];
    for data in cleaned_data.iter() {
        let s = data.split(' ').collect::<Vec<&str>>();
        let opcodes = transform_instruction_opcode(&s);
        let bytes = transform_opcode_bytes(&opcodes);
        for byte in bytes.iter() {
            buf.push(byte.to_owned());
        }
    }
    output_file.write_all(&buf[..]).unwrap();
}

fn transform_instruction_opcode(data: &[&str]) -> Vec<u16> {
    let mut opcodes: Vec<u16> = Vec::new();
    for value in data.iter() {
        if value.starts_with("0x") {
            let without_prefix = value.trim_start_matches("0x");
            let z = i32::from_str_radix(without_prefix, 16);
            opcodes.push(z.unwrap() as u16);
        } else {
            opcodes.push(
                TryInto::<Instructions>::try_into(value.to_lowercase().as_ref())
                    .unwrap()
                    .match_instruction_opcode() as u16,
            );
        }
    }
    opcodes
}

fn transform_opcode_bytes(data: &[u16]) -> Vec<u8> {
    let mut buf = vec![];
    for value in data.iter() {
        buf.put_u16_le(value.to_owned());
    }
    buf
}
