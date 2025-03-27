use std::{thread::sleep, time::Duration};

use cpu::CPU;

mod cpu;

const ROM_START_ADDRESS: usize = 0x200;
const RAM_SIZE: usize = 4096;
type RAM = [u8; RAM_SIZE];

pub struct Chip8 {
    cpu: CPU,
    ram: RAM,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        return Chip8 {
            cpu: CPU::new(),
            ram: [0u8; RAM_SIZE],
        };
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        if rom.len() + ROM_START_ADDRESS > RAM_SIZE {
            panic!("ROM size too big!");
        } else {
            self.ram[ROM_START_ADDRESS..(ROM_START_ADDRESS + rom.len())].copy_from_slice(&rom);
        }
    }

    pub fn run_loop(&mut self) {
        loop {
            let next_opcode = self.cpu.consume_next_opcode(&self.ram);
            self.cpu.execute_opcode(next_opcode);
            println!("{:?}", self.cpu);
            sleep(Duration::new(0, 300000000));
        }
    }
}
