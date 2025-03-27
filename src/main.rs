use std::{
    fs::File,
    io::{Read, Result},
};

use chip8::Chip8;

mod chip8;

fn main() -> Result<()> {
    let mut chip8 = Chip8::new();

    // Load ROM
    let mut file = File::open("rom.ch8")?;
    let mut rom = Vec::new();
    file.read_to_end(&mut rom)?;
    chip8.load_rom(&rom);

    // FDE Loop
    chip8.run_loop();

    Ok(())
}
