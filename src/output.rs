#[allow(dead_code)]
pub enum RenderMode {
    None,
    Braille,
    Ascii,
    Block,
}

pub struct Screen {
    pixels: [[bool; 32]; 64],
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            pixels: [[false; 32]; 64],
        }
    }

    pub fn draw(&mut self, x: u8, y: u8, sprite: &[u8]) -> bool {
        let mut collision = false;

        for (i, byte) in sprite.iter().enumerate() {
            for j in 0..8 {
                let bit = (byte >> (7 - j)) & 1;
                let x = (x as usize + j) % 64;
                let y = (y as usize + i) % 32;

                if bit == 1 && self.pixels[x][y] {
                    collision = true;
                }

                self.pixels[x][y] ^= bit == 1;
            }
        }

        collision
    }

    pub fn clear(&mut self) {
        self.pixels = [[false; 32]; 64];
    }

    pub fn render(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!(" {:\u{2581}>64} \n", ""));

        for y in (0..32).step_by(2) {
            output.push('\u{2595}');
            for x in 0..64 {
                output.push(match (self.pixels[x][y], self.pixels[x][y+1]) {
                    (false, false) => ' ',
                    (true,  false) => '\u{2580}',
                    (false, true)  => '\u{2584}',
                    (true,  true)  => '\u{2588}',
                });
            }
            output.push_str("\u{258F}\n");
        }

        output.push_str(&format!(" {:\u{2594}>64} \n", ""));

        output
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}
