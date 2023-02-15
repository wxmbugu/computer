#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]
use assembler::{instructions::*, RAM_BASE};

use bytes::BufMut;
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
    // filter out labels: i.e loop
    let labels = data
        .iter()
        .filter(|f| f.ends_with(':'))
        .collect::<Vec<&String>>();
    let ok = data.iter().position(|x| x.ends_with(':')).unwrap();
    let r = ok as u16 + 1;
    let sm = RAM_BASE + r as u64;
    println!("postion = {r}");
    println!("memory_location = {sm}");
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
    println!("{}", cleaned_data.len());
    for data in cleaned_data.iter() {
        let s = data
            .split(' ')
            .map(|s| s.to_lowercase())
            .collect::<Vec<String>>();
        let opcodez = transform_instruction_to_opcode(&s, s.len() as u16);
        for byte in opcodez.iter() {
            buf.put_u16(byte.to_owned());
        }
    }
    output_file.write_all(&buf[..]).unwrap();
}

fn transform_instruction_to_opcode(data: &[String], len: u16) -> Vec<u16> {
    let mut opcodes: Vec<u16> = Vec::new();

    match len {
        1 => {
            for value in data.iter() {
                opcodes.push(
                    (TryInto::<Instructions>::try_into(value.to_owned())
                        .unwrap()
                        .match_instruction_opcode()
                        & 0x0f) as u16,
                )
            }
        }
        2 => {
            let d = (TryInto::<Instructions>::try_into(data[1].to_owned())
                .unwrap()
                .match_instruction_opcode()
                << 4)
                & 0xf0
                | (TryInto::<Instructions>::try_into(data[0].to_owned())
                    .unwrap()
                    .match_instruction_opcode())
                    & 0x0f;
            opcodes.push(d as u16);
        }
        3 => {
            let mut z: u32 = 0;
            if data[2].starts_with("0x") {
                let without_prefix = data[2].trim_start_matches("0x");
                z = u32::from_str_radix(without_prefix, 16).unwrap();
            } else {
                z = TryInto::<Instructions>::try_into(data[2].to_owned())
                    .unwrap()
                    .match_instruction_opcode();
            }
            let d = (TryInto::<Instructions>::try_into(data[1].to_owned())
                .unwrap()
                .match_instruction_opcode()
                << 4)
                | (TryInto::<Instructions>::try_into(data[0].to_owned())
                    .unwrap()
                    .match_instruction_opcode())
                    & 0x0f;
            let f = ((z << 8) & 0xff00 | d & 0x00ff) as u16;
            opcodes.push(f);
        }
        4 => {
            let op_register = (TryInto::<Instructions>::try_into(data[1].to_owned())
                .unwrap()
                .match_instruction_opcode()
                << 4)
                | (TryInto::<Instructions>::try_into(data[0].to_owned())
                    .unwrap()
                    .match_instruction_opcode())
                    & 0x0f;
            let register_register = (TryInto::<Instructions>::try_into(data[3].to_owned())
                .unwrap()
                .match_instruction_opcode()
                << 4)
                | (TryInto::<Instructions>::try_into(data[2].to_owned())
                    .unwrap()
                    .match_instruction_opcode())
                    & 0x0f;
            let f = ((register_register << 8) & 0xff00 | op_register & 0x00ff) as u16;
            opcodes.push(f);
        }
        _ => unimplemented!(),
    };
    opcodes
}

fn transform_opcode_bytes(data: &[u16]) -> Vec<u8> {
    let mut buf = vec![];
    for value in data.iter().rev() {
        buf.put_u16(value.to_owned());
    }
    buf
}
