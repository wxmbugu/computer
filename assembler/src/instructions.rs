#![allow(dead_code)]

use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
pub enum Instructions {
    /// Do nothing
    NOP,
    /// Terminate program
    HALT,
    /// Load Immediate
    LI,
    /// Load Word
    LW,
    /// Store Word
    SW,
    /// Add
    ADD,
    /// Subtract
    SUB,
    /// Multiply
    MULT,
    /// Divide
    DIV,
    /// unconditional jump
    J,
    /// unconditional jump register
    JR,
    /// Branch if equal
    BEQ,
    /// Branch if NOT Equal
    BNE,
    /// Increment register
    INC,
    /// decrease register
    DEC,
    /// Genereal Registers R1,R2,R3
    R1,
    R2,
    R3,
    /// Program counter
    PC,
    /// store conditional flag
    COND,
}

impl Instructions {
    /// matches an instruction to  an opcode
    pub fn match_instruction_opcode(self) -> u32 {
        match self {
            Self::HALT => 0x00,
            Self::NOP => 0x01,
            Self::LI => 0x02,
            Self::LW => 0x03,
            Self::SW => 0x04,
            Self::ADD => 0x05,
            Self::SUB => 0x06,
            Self::MULT => 0x07,
            Self::DIV => 0x08,
            Self::J => 0x09,
            Self::JR => 0x0A,
            Self::BEQ => 0x0B,
            Self::BNE => 0x0C,
            Self::INC => 0x0D,
            Self::DEC => 0x0E,
            Self::R1 => 0x01,
            Self::R2 => 0x02,
            Self::R3 => 0x03,
            Self::PC => 0x0f,
            Self::COND => 0x10,
        }
    }
}

impl Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl FromStr for Instructions {
    type Err = String;
    fn from_str(instruction: &str) -> Result<Self, Self::Err> {
        match instruction {
            "halt" => Ok(Instructions::HALT),
            "nop" => Ok(Instructions::NOP),
            "li" => Ok(Instructions::LI),
            "lw" => Ok(Instructions::LW),
            "sw" => Ok(Instructions::SW),
            "add" => Ok(Instructions::ADD),
            "sub" => Ok(Instructions::SUB),
            "mult" => Ok(Instructions::MULT),
            "div" => Ok(Instructions::DIV),
            "j" => Ok(Instructions::J),
            "jr" => Ok(Instructions::JR),
            "beq" => Ok(Instructions::BEQ),
            "bne" => Ok(Instructions::BNE),
            "inc" => Ok(Instructions::INC),
            "dec" => Ok(Instructions::DEC),
            "r1" => Ok(Instructions::R1),
            "r2" => Ok(Instructions::R2),
            "r3" => Ok(Instructions::R3),
            "pc" => Ok(Instructions::PC),
            "cond" => Ok(Instructions::COND),
            _ => Err("No such Instruction".to_string()),
        }
    }
}

impl From<&str> for Instructions {
    fn from(value: &str) -> Self {
        match value {
            "halt" => Instructions::HALT,
            "nop" => Instructions::NOP,
            "li" => Instructions::LI,
            "lw" => Instructions::LW,
            "sw" => Instructions::SW,
            "add" => Instructions::ADD,
            "sub" => Instructions::SUB,
            "mult" => Instructions::MULT,
            "div" => Instructions::DIV,
            "j" => Instructions::J,
            "jr" => Instructions::JR,
            "beq" => Instructions::BEQ,
            "bne" => Instructions::BNE,
            "inc" => Instructions::INC,
            "dec" => Instructions::DEC,
            "r1" => Instructions::R1,
            "r2" => Instructions::R2,
            "r3" => Instructions::R3,
            "pc" => Instructions::PC,
            "cond" => Instructions::COND,
            _ => todo!(),
        }
    }
}
