extern crate piston_window;

use piston_window::{PistonWindow, WindowSettings};
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
const SCALE: u32 = 10;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

fn main() {
    let rom = fs::read(ROM_FILE).unwrap();
    let mut system = System::new(&rom);

    let mut window: PistonWindow = WindowSettings::new("Chip8", [64*SCALE, 32*SCALE])
        .exit_on_esc(true).resizable(false).build().unwrap();

    while let Some(event) = window.next() {
        system.tick();

        window.draw_2d(&event, |context, graphics, _device| {
            piston_window::clear(BLACK, graphics);

            let data = system.get_pixels();

            for (x, row) in data.iter().enumerate() {
                for (y, pix) in row.iter().enumerate() {
                    if *pix {
                        let x = (x as f64) * (SCALE as f64);
                        let y = (y as f64) * (SCALE as f64);
                        let w = SCALE as f64;
                        let h = SCALE as f64;
                        piston_window::rectangle(WHITE,[x,y,w,h], context.transform, graphics);
                    }
                }
            }
        });
    }
}
