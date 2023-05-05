use rand::random;

use crate::input::*;
use crate::output::*;
use crate::cpu::*;
use crate::DEBUG;

// https://tobiasvl.github.io/blog/write-a-chip-8-emulator
// http://www.emulator101.com/chip-8-instruction-set.html
// https://en.wikipedia.org/wiki/CHIP-8
// https://chip-8.github.io/links

const CLOCK_HZ: f64 = 10000.0; // 10 kHz

#[derive(Debug)]
pub struct System {
    display: Screen,
    keypad: Keypad,

    memory: Memory,
    stack: Vec<Address>,
    pc: Address,
    i: Address,
    v: [u8; 16],

    delay_timer: u8,
    sound_timer: u8,

    clock_dt: f64,
    timer_dt: f64,
}

impl System {
    pub fn new(rom: &[u8]) -> Self {
        let mut ram = Memory::new();

        ram.write(0x050.into(), &[
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ]);

        ram.write(0x200.into(), rom);

        System {
            display: Screen::new(),
            keypad: Keypad::new(),

            memory: ram,
            stack: Vec::new(),
            pc: Address::new(0x200),
            i: Address::new(0),
            v: [0; 16],

            delay_timer: 0,
            sound_timer: 0,

            clock_dt: 0.0,
            timer_dt: 0.0,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.clock_dt += dt;
        self.timer_dt += dt;

        while self.clock_dt >= 1.0 / CLOCK_HZ {
            self.clock_dt -= 1.0 / CLOCK_HZ;

            let pc_old = self.pc;

            let opcode = self.memory.read16(self.pc);
            let instruction = Instruction::new(opcode).unwrap();
            self.execute(instruction);

            if DEBUG && pc_old != self.pc {
                println!("[{:03X}]: {:04X} => {}", pc_old.get(), opcode, instruction);
            }
        }

        while self.timer_dt >= 1.0 / 60.0 {
            self.timer_dt -= 1.0 / 60.0;

            if self.delay_timer > 0 { self.delay_timer -= 1; }
            if self.sound_timer > 0 { self.sound_timer -= 1; }
        }
    }

    pub fn update_keypad(&mut self, key: Key, pressed: bool) {
        self.keypad.set_pressed(key, pressed);
    }

    pub fn get_pixels(&self) -> &[[bool; 32]; 64] { self.display.get_pixels() }
    pub fn get_sound(&self) -> bool { self.sound_timer > 0 }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Clear => self.display.clear(),
            Instruction::Return => self.pc = self.stack.pop().unwrap(),
            Instruction::Jump(addr) => return self.pc = addr,
            Instruction::Call(addr) => {
                self.stack.push(self.pc);
                return self.pc = addr;
            }
            Instruction::EqNum(reg, num) => {
                if self.v[reg.idx()] == num { self.pc = self.pc.add(2); }
            }
            Instruction::NeqNum(reg, num) => {
                if self.v[reg.idx()] != num { self.pc = self.pc.add(2); }
            }
            Instruction::Eq(reg_x, reg_y) => {
                if self.v[reg_x.idx()] == self.v[reg_y.idx()] { self.pc = self.pc.add(2); }
            }
            Instruction::Neq(reg_x, reg_y) => {
                if self.v[reg_x.idx()] != self.v[reg_y.idx()] { self.pc = self.pc.add(2); }
            }
            Instruction::SetNum(reg, num) => self.v[reg.idx()] = num,
            Instruction::AddNum(reg, num) => {
                self.v[reg.idx()] = self.v[reg.idx()].overflowing_add(num).0
            }
            Instruction::Move(reg_x, reg_y) => self.v[reg_x.idx()] = self.v[reg_y.idx()],
            Instruction::Or(reg_x, reg_y) => self.v[reg_x.idx()] |= self.v[reg_y.idx()],
            Instruction::And(reg_x, reg_y) => self.v[reg_x.idx()] &= self.v[reg_y.idx()],
            Instruction::Xor(reg_x, reg_y) => self.v[reg_x.idx()] ^= self.v[reg_y.idx()],
            Instruction::Add(reg_x, reg_y) => {
                let value1 = self.v[reg_x.idx()];
                let value2 = self.v[reg_y.idx()];

                let (result, overflow) = value1.overflowing_add(value2);

                self.v[0xF] = overflow as u8;
                self.v[reg_x.idx()] = result;
            }
            Instruction::Sub(reg_x, reg_y) => {
                let value1 = self.v[reg_x.idx()];
                let value2 = self.v[reg_y.idx()];

                let (result, overflow) = value1.overflowing_sub(value2);

                self.v[0xF] = !overflow as u8;
                self.v[reg_x.idx()] = result;
            }
            Instruction::Shr(reg_x, reg_y) => {
                self.v[0xF] = self.v[reg_y.idx()] & 1;
                self.v[reg_x.idx()] = self.v[reg_y.idx()] >> 1;
            }
            Instruction::Subb(reg_x, reg_y) => {
                let value1 = self.v[reg_y.idx()];
                let value2 = self.v[reg_x.idx()];

                let (result, overflow) = value1.overflowing_sub(value2);

                self.v[0xF] = !overflow as u8;
                self.v[reg_x.idx()] = result;
            }
            Instruction::Shl(reg_x, reg_y) => {
                self.v[0xF] = (self.v[reg_y.idx()] & 0x80) >> 7;
                self.v[reg_x.idx()] = self.v[reg_y.idx()] << 1;
            }
            Instruction::SetIdx(addr) => self.i = addr,
            Instruction::JumpV0(addr) => return self.pc = addr.add(self.v[0] as u16),
            Instruction::Rand(reg, num) => {
                self.v[reg.idx()] = random::<u8>() & num;
            }
            Instruction::Draw(reg_x, reg_y, size) => {
                let x = self.v[reg_x.idx()];
                let y = self.v[reg_y.idx()];
                let sprite = self.memory.read(self.i, size.into());
                let collision = self.display.draw(x, y, sprite);
                self.v[0xF] = collision as u8;
            }
            Instruction::KeyUp(reg) => {
                let key = Key::new(self.v[reg.idx()]);
                if self.keypad.is_pressed(key) {
                    self.pc = self.pc.add(2);
                }
            }
            Instruction::KeyDown(reg) => {
                let key = Key::new(self.v[reg.idx()]);
                if !self.keypad.is_pressed(key) {
                    self.pc = self.pc.add(2);
                }
            }
            Instruction::GetDelay(reg) => self.v[reg.idx()] = self.delay_timer,
            Instruction::WaitKey(reg) => {
                if let Some(key) = self.keypad.get_key() {
                    self.v[reg.idx()] = key.get();
                } else {
                    self.pc = self.pc.sub(2);
                }
            }
            Instruction::SetDelay(reg) => self.delay_timer = self.v[reg.idx()],
            Instruction::SetSound(reg) => self.sound_timer = self.v[reg.idx()],
            Instruction::AddIdx(reg) => self.i = self.i.add(self.v[reg.idx()].into()),
            Instruction::SetSprite(reg) => self.i = (0x050 + self.v[reg.idx()] as u16 * 5).into(),
            Instruction::StoreBcd(reg) => {
                let value = self.v[reg.idx()];
                self.memory.write(self.i, &[value / 100, (value / 10) % 10, value % 10]);
            }
            Instruction::Store(reg) => {
                self.memory.write(self.i, &self.v[..=reg.idx()]);
                self.i = self.i.add(1 + reg.get() as u16);
            }
            Instruction::Load(reg) => {
                let data = self.memory.read(self.i, 1 + reg.get() as u16);
                self.v[..=reg.idx()].copy_from_slice(data);
                self.i = self.i.add(1 + reg.get() as u16);
            }
        }

        self.pc = self.pc.add(2);
    }
}
