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

    pub fn get_pixels(&self) -> &[[bool; 32]; 64] {
        &self.pixels
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}
