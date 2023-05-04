extern crate piston_window;

use piston_window::{PistonWindow, WindowSettings, Event, Loop, Window, Input, Button};
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

const ROM_FILE: &str = "roms/test_rng.ch8";
const DEBUG: bool = true;

fn main() {
    let rom = fs::read(ROM_FILE).unwrap();
    let mut system = System::new(&rom);

    let mut window: PistonWindow = WindowSettings::new("Chip8", [640, 320])
        .exit_on_esc(true).build().unwrap();

    while let Some(event) = window.next() {
        match event {
            Event::Loop(Loop::Render(_)) => {}
            Event::Loop(Loop::Update(args)) => system.update(args.dt),
            Event::Input(Input::Button(args), _) => {
                let key = match args.button {
                    Button::Keyboard(key) => {
                        match key {
                            piston_window::Key::D1 => Some(Key::new(0x1)),
                            piston_window::Key::D2 => Some(Key::new(0x2)),
                            piston_window::Key::D3 => Some(Key::new(0x3)),
                            piston_window::Key::D4 => Some(Key::new(0xC)),
                            piston_window::Key::Q  => Some(Key::new(0x4)),
                            piston_window::Key::W  => Some(Key::new(0x5)),
                            piston_window::Key::E  => Some(Key::new(0x6)),
                            piston_window::Key::R  => Some(Key::new(0xD)),
                            piston_window::Key::A  => Some(Key::new(0x7)),
                            piston_window::Key::S  => Some(Key::new(0x8)),
                            piston_window::Key::D  => Some(Key::new(0x9)),
                            piston_window::Key::F  => Some(Key::new(0xE)),
                            piston_window::Key::Z  => Some(Key::new(0xA)),
                            piston_window::Key::X  => Some(Key::new(0x0)),
                            piston_window::Key::C  => Some(Key::new(0xB)),
                            piston_window::Key::V  => Some(Key::new(0xF)),
                            _ => None,
                        }
                    }
                    _ => None,
                };

                let state = match args.state {
                    piston_window::ButtonState::Press => true,
                    piston_window::ButtonState::Release => false,
                };

                if let Some(key) = key {
                    system.update_keypad(key, state);
                }
            }
            _ => {}
        }

        let window_size = window.size();
        window.draw_2d(&event, |context, graphics, _device| {
            const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
            const GRAY:  [f32; 4] = [0.5, 0.5, 0.5, 1.0];
            const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

            piston_window::clear(GRAY, graphics);

            let scale = (window_size.width / 64.0).min(window_size.height / 32.0);

            let offset_x = (window_size.width - 64.0*scale) / 2.0;
            let offset_y = (window_size.height - 32.0*scale) / 2.0;

            piston_window::rectangle(BLACK,[offset_x, offset_y, 64.0*scale, 32.0*scale], context.transform, graphics);

            let pixels = system.get_pixels();
            for (x, row) in pixels.iter().enumerate() {
                for (y, pix) in row.iter().enumerate() {
                    if *pix {
                        let x = offset_x + (x as f64) * scale;
                        let y = offset_y + (y as f64) * scale;
                        piston_window::rectangle(WHITE, [x, y, scale, scale], context.transform, graphics);
                    }
                }
            }
        });
    }
}
