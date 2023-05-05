use crate::cpu::{Address, Register};
use crate::input::Key;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Clear,                        // 00E0
    Return,                       // 00EE
    Jump(Address),                // 1NNN
    Call(Address),                // 2NNN
    EqNum(Register, u8),          // 3XNN
    NeqNum(Register, u8),         // 4XNN
    Eq(Register, Register),       // 5XY0
    SetNum(Register, u8),         // 6XNN
    AddNum(Register, u8),         // 7XNN
    Move(Register, Register),     // 8XY0
    Or(Register, Register),       // 8XY1
    And(Register, Register),      // 8XY2
    Xor(Register, Register),      // 8XY3
    Add(Register, Register),      // 8XY4
    Sub(Register, Register),      // 8XY5
    Shr(Register, Register),      // 8XY6
    Subb(Register, Register),     // 8XY7
    Shl(Register, Register),      // 8XYE
    Neq(Register, Register),      // 9XY0
    SetIdx(Address),              // ANNN
    JumpV0(Address),              // BNNN
    Rand(Register, u8),           // CXNN
    Draw(Register, Register, u8), // DXYN
    KeyEq(Key),                   // EX9E
    KeyNeq(Key),                  // EXA1
    GetDelay(Register),           // FX07
    WaitKey(Register),            // FX0A
    SetDelay(Register),           // FX15
    SetSound(Register),           // FX18
    AddIdx(Register),             // FX1E
    SetSprite(Register),          // FX29
    StoreBcd(Register),           // FX33
    Store(Register),              // FX55
    Load(Register),               // FX65
}

impl Instruction {
    pub fn new(data: u16) -> Option<Self> {
        let addr = Address::new(data & 0x0FFF);
        let key = Key::new(((data & 0x0F00) >> 8) as u8);
        let reg_x = Register::new(((data & 0x0F00) >> 8) as u8);
        let reg_y = Register::new(((data & 0x00F0) >> 4) as u8);
        let num = (data & 0x00FF) as u8;

        let nib1 = ((data & 0xF000) >> 12) as u8;
        let nib2 = ((data & 0x0F00) >> 8) as u8;
        let nib3 = ((data & 0x00F0) >> 4) as u8;
        let nib4 = (data & 0x000F) as u8;

        match (nib1, nib2, nib3, nib4) {
            (0x0,0x0,0xE,0x0) => Some(Self::Clear),
            (0x0,0x0,0xE,0xE) => Some(Self::Return),
            (0x0, _ , _ , _ ) => None, // TODO?
            (0x1, _ , _ , _ ) => Some(Self::Jump(addr)),
            (0x2, _ , _ , _ ) => Some(Self::Call(addr)),
            (0x3, _ , _ , _ ) => Some(Self::EqNum(reg_x, num)),
            (0x4, _ , _ , _ ) => Some(Self::NeqNum(reg_x, num)),
            (0x5, _ , _ ,0x0) => Some(Self::Eq(reg_x, reg_y)),
            (0x6, _ , _ , _ ) => Some(Self::SetNum(reg_x, num)),
            (0x7, _ , _ , _ ) => Some(Self::AddNum(reg_x, num)),
            (0x8, _ , _ ,0x0) => Some(Self::Move(reg_x, reg_y)),
            (0x8, _ , _ ,0x1) => Some(Self::Or(reg_x, reg_y)),
            (0x8, _ , _ ,0x2) => Some(Self::And(reg_x, reg_y)),
            (0x8, _ , _ ,0x3) => Some(Self::Xor(reg_x, reg_y)),
            (0x8, _ , _ ,0x4) => Some(Self::Add(reg_x, reg_y)),
            (0x8, _ , _ ,0x5) => Some(Self::Sub(reg_x, reg_y)),
            (0x8, _ , _ ,0x6) => Some(Self::Shr(reg_x, reg_y)),
            (0x8, _ , _ ,0x7) => Some(Self::Subb(reg_x, reg_y)),
            (0x8, _ , _ ,0xE) => Some(Self::Shl(reg_x, reg_y)),
            (0x9, _ , _ ,0x0) => Some(Self::Neq(reg_x, reg_y)),
            (0xA, _ , _ , _ ) => Some(Self::SetIdx(addr)),
            (0xB, _ , _ , _ ) => Some(Self::JumpV0(addr)),
            (0xC, _ , _ , _ ) => Some(Self::Rand(reg_x, num)),
            (0xD, _ , _ , _ ) => Some(Self::Draw(reg_x, reg_y, nib4)),
            (0xE, _ ,0x9,0xE) => Some(Self::KeyEq(key)),
            (0xE, _ ,0xA,0x1) => Some(Self::KeyNeq(key)),
            (0xF, _ ,0x0,0x7) => Some(Self::GetDelay(reg_x)),
            (0xF, _ ,0x0,0xA) => Some(Self::WaitKey(reg_x)),
            (0xF, _ ,0x1,0x5) => Some(Self::SetDelay(reg_x)),
            (0xF, _ ,0x1,0x8) => Some(Self::SetSound(reg_x)),
            (0xF, _ ,0x1,0xE) => Some(Self::AddIdx(reg_x)),
            (0xF, _ ,0x2,0x9) => Some(Self::SetSprite(reg_x)),
            (0xF, _ ,0x3,0x3) => Some(Self::StoreBcd(reg_x)),
            (0xF, _ ,0x5,0x5) => Some(Self::Store(reg_x)),
            (0xF, _ ,0x6,0x5) => Some(Self::Load(reg_x)),
            _ => None
        }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Clear         => write!(f, "CLS"),
            Self::Return        => write!(f, "RET"),
            Self::Jump(a)       => write!(f, "JUMP ${:03X}", a.get()),
            Self::Call(a)       => write!(f, "CALL ${:03X}", a.get()),
            Self::EqNum(x, n)   => write!(f, "SEQ V{}, {}", x.get(), n),
            Self::NeqNum(x, n)  => write!(f, "SNE V{}, {}", x.get(), n),
            Self::Eq(x, y)      => write!(f, "SEQ V{}, V{}", x.get(), y.get()),
            Self::SetNum(x, n)  => write!(f, "MOV V{}, {}", x.get(), n),
            Self::AddNum(x, n)  => write!(f, "ADD V{}, {}", x.get(), n),
            Self::Move(x, y)    => write!(f, "MOV V{}, V{}", x.get(), y.get()),
            Self::Or(x, y)      => write!(f, "OR V{}, V{}", x.get(), y.get()),
            Self::And(x, y)     => write!(f, "AND V{}, V{}", x.get(), y.get()),
            Self::Xor(x, y)     => write!(f, "XOR V{}, V{}", x.get(), y.get()),
            Self::Add(x, y)     => write!(f, "ADD V{}, V{}", x.get(), y.get()),
            Self::Sub(x, y)     => write!(f, "SUB V{}, V{}", x.get(), y.get()),
            Self::Shr(x, y)     => write!(f, "SHR V{}, V{}", x.get(), y.get()),
            Self::Subb(x, y)    => write!(f, "SUBB V{}, V{}", x.get(), y.get()),
            Self::Shl(x, y)     => write!(f, "SHL V{}, V{}", x.get(), y.get()),
            Self::Neq(x, y)     => write!(f, "SNE V{}, V{}", x.get(), y.get()),
            Self::SetIdx(a)     => write!(f, "MOV I, ${:03X}", a.get()),
            Self::JumpV0(a)     => write!(f, "JUMP V0 + ${:03X}", a.get()),
            Self::Rand(x, n)    => write!(f, "RAND V{}, {}", x.get(), n),
            Self::Draw(x, y, n) => write!(f, "DRAW V{}, V{}, {}", x.get(), y.get(), n),
            Self::KeyEq(k)      => write!(f, "SEQ K{}", k.get()),
            Self::KeyNeq(k)     => write!(f, "SNE K{}", k.get()),
            Self::GetDelay(x)   => write!(f, "MOV V{}, DT", x.get()),
            Self::WaitKey(k)    => write!(f, "WAIT K{}", k.get()),
            Self::SetDelay(x)   => write!(f, "MOV DT, V{}", x.get()),
            Self::SetSound(x)   => write!(f, "MOV ST, V{}", x.get()),
            Self::AddIdx(x)     => write!(f, "ADD I, V{}", x.get()),
            Self::SetSprite(x)  => write!(f, "CHAR V{}", x.get()),
            Self::StoreBcd(x)   => write!(f, "BCD V{}", x.get()),
            Self::Store(x)      => write!(f, "MOV [I], ..V{}", x.get()),
            Self::Load(x)       => write!(f, "MOV ..V{}, [I]", x.get()),
        }
    }
}
