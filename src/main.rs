use std::fs;
use rand::random;

mod input;
mod output;
mod cpu;
mod system;

use input::*;
use output::*;
use cpu::*;
use system::*;

const ROM_FILE: &str = "roms/test.ch8";
const DEBUG: bool = false;
const CLOCK_HZ: u32 = 500;

fn main() {
    let rom = fs::read(ROM_FILE).unwrap();
    let mut system = System::new(&rom);

    for _ in 0..10000 {
        system.tick();
    }

    print!("{}", system.render());
}
