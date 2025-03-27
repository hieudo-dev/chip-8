use std::fmt::Debug;

use super::{RAM, ROM_START_ADDRESS};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub struct CPU {
    pc: u16,
    v: [u8; 16],
    i: u16,
}

impl Debug for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "-------------------------------------------------------")?;
        write!(f, "\nPC: {:04X}\nI: {:04X}\nV: [", self.pc, self.i)?;
        for (index, reg) in self.v.iter().enumerate() {
            if index < self.v.len() - 1 {
                write!(f, "{:02X}, ", reg)?;
            } else {
                write!(f, "{:02X}", reg)?;
            }
        }
        write!(f, "]\n")
    }
}

impl CPU {
    pub fn new() -> CPU {
        return CPU {
            pc: ROM_START_ADDRESS as u16,
            v: [0; 16],
            i: 0,
        };
    }

    pub fn consume_next_opcode(&mut self, ram: &RAM) -> u16 {
        let slice = &ram[self.pc as usize..self.pc as usize + 2];
        self.pc += 2;
        return u16::from_be_bytes([slice[0], slice[1]]);
    }

    pub fn execute_opcode(&mut self, raw_opcode: u16) {
        let optype = FromPrimitive::from_u16(raw_opcode >> 12);
        match optype {
            Some(Optype::SET) => {
                let value = raw_opcode as u8;
                self.v[CPU::get_register(raw_opcode)] = value;
            }
            Some(Optype::ADD) => {
                let value = raw_opcode as u8;
                self.v[CPU::get_register(raw_opcode)] += value;
            }
            None => {
                panic!("Ran out of instructions!");
            }
            x => {
                panic!("Unknown instruction: {:?}", x);
            }
        }
    }

    fn get_register(raw_opcode: u16) -> usize {
        return (raw_opcode & 0x0F00) as usize;
    }
}

#[derive(Debug, FromPrimitive)]
pub enum Optype {
    SET = 0x6,
    ADD = 0x7,
    Unknown,
}
