#![allow(unused_assignments)]
use assembler::{instructions::*, RAM_BASE};

use bytes::BufMut;
use clap::{Arg, Command};
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};
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

    //binary file location format in text file
    let mut table = LabelTable::default();
    let cleaned_data = data
        .iter()
        .map(|x| {
            let mut new_label = " ";
            let mut addr = 0;
            for label in &labels {
                new_label = label.strip_suffix(':').unwrap();
                let position = data.iter().position(|x| x.starts_with(new_label)).unwrap();
                addr = RAM_BASE + (position as u64 * 4) + 1;
                table.insert(position as u32, addr as u32);
            }
            let mut new_addr = format!("{addr:x}");
            new_addr.insert_str(0, "0x");
            if !new_label.is_empty() {
                x.to_string()
                    .replace("loop:", " ")
                    .replace(new_label, &new_addr)
                    .replace(':', "\t")
                    .trim()
                    .to_string()
            } else {
                // This might not work
                x.to_string().replace(':', "\t").trim().to_string()
            }
        })
        .filter(|x| !x.is_empty())
        .collect::<Vec<String>>();

    let mut buf = vec![];
    for data in cleaned_data.iter() {
        let s = data
            .split(' ')
            .map(|s| s.to_lowercase())
            .collect::<Vec<String>>();
        let opcodez = transform_instruction_to_opcode(&s, s.len() as u16);
        for byte in opcodez.iter() {
            buf.put_u32(byte.to_owned());
            // println!("{byte:034b}")
        }
    }
    output_file.write_all(&buf[..]).unwrap();
    let table_file = File::create("LabelTable.json").unwrap();
    serde_json::to_writer(table_file, &table.table).unwrap();
}

fn transform_instruction_to_opcode(data: &[String], len: u16) -> Vec<u32> {
    let mut opcodes: Vec<u32> = Vec::new();

    match len {
        1 => {
            for value in data.iter() {
                opcodes.push(
                    TryInto::<Instructions>::try_into(value.to_owned())
                        .unwrap()
                        .match_instruction_opcode()
                        & 0x0f,
                )
            }
        }
        2 => {
            let mut z: u32 = 0;
            let mut d: u32 = 0;
            if data[1].starts_with("0x") {
                let without_prefix = data[1].trim_start_matches("0x");
                z = u32::from_str_radix(without_prefix, 16).unwrap();
                d = (z << 8)
                    | (TryInto::<Instructions>::try_into(data[0].to_owned())
                        .unwrap()
                        .match_instruction_opcode())
                        & 0x0f;
            } else {
                d = (TryInto::<Instructions>::try_into(data[1].to_owned())
                    .unwrap()
                    .match_instruction_opcode()
                    << 4)
                    & 0xf0
                    | (TryInto::<Instructions>::try_into(data[0].to_owned())
                        .unwrap()
                        .match_instruction_opcode())
                        & 0x0f;
            }
            println!("{d:032b},{:032b}", 53620);
            opcodes.push(d);
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
            let f = (z << 8) & 0xffff00 | d & 0x0000ff;
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
            let f = (register_register << 8) & 0xffff00 | op_register & 0x0000ff;
            opcodes.push(f);
        }
        _ => unimplemented!(),
    };
    opcodes
}
