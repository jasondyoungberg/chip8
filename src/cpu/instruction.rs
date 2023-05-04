use crate::cpu::{Address, Register};
use crate::input::Key;

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
    Shr(Register),                // 8XY6
    Subb(Register, Register),     // 8XY7
    Shl(Register),                // 8XYE
    Neq(Register, Register),      // 9XY0
    SetIdx(Address),              // ANNN
    JumpV0(Address),              // BNNN
    Rand(Register, u8),           // CXNN
    Draw(Register, Register, u8), // DXYN
    KeyEq(Key),                   // EX9E
    KeyNeq(Key),                  // EXA1
    GetDelay(Register),           // FX07
    WaitKey(Register),                 // FX0A
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
            (0x8, _ , _ ,0x6) => Some(Self::Shr(reg_x)),
            (0x8, _ , _ ,0x7) => Some(Self::Subb(reg_x, reg_y)),
            (0x8, _ , _ ,0xE) => Some(Self::Shl(reg_x)),
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
            Self::Clear                   => write!(f, "CLS"),
            Self::Return                  => write!(f, "RET"),
            Self::Jump(addr)              => write!(f, "JUMP ${:03X}", addr.get()),
            Self::Call(addr)              => write!(f, "CALL ${:03X}", addr.get()),
            Self::EqNum(reg, num)         => write!(f, "SKIP.EQ {}, {}", reg, num),
            Self::NeqNum(reg, num)        => write!(f, "SKIP.NE {}, {}", reg, num),
            Self::Eq(reg_x, reg_y)        => write!(f, "SKIP.EQ {}, {}", reg_x, reg_y),
            Self::SetNum(reg, num)        => write!(f, "MOV {}, {}", reg, num),
            Self::AddNum(reg, num)        => write!(f, "ADD {}, {}", reg, num),
            Self::Move(reg_x, reg_y)      => write!(f, "MOV {}, {}", reg_x, reg_y),
            Self::Or(reg_x, reg_y)        => write!(f, "OR {}, {}", reg_x, reg_y),
            Self::And(reg_x, reg_y)       => write!(f, "AND {}, {}", reg_x, reg_y),
            Self::Xor(reg_x, reg_y)       => write!(f, "XOR {}, {}", reg_x, reg_y),
            Self::Add(reg_x, reg_y)       => write!(f, "ADD {}, {}", reg_x, reg_y),
            Self::Sub(reg_x, reg_y)       => write!(f, "SUB {}, {}", reg_x, reg_y),
            Self::Shr(reg_x)              => write!(f, "SHR {}", reg_x),
            Self::Subb(reg_x, reg_y)      => write!(f, "SUBB {}, {}", reg_x, reg_y),
            Self::Shl(reg_x)              => write!(f, "SHL {}", reg_x),
            Self::Neq(reg_x, reg_y)       => write!(f, "SKIP.NE {}, {}", reg_x, reg_y),
            Self::SetIdx(addr)            => write!(f, "MOV I, ${:03X}", addr.get()),
            Self::JumpV0(addr)            => write!(f, "JUMP V0 + ${:03X}", addr.get()),
            Self::Rand(reg, num)          => write!(f, "RAND {}, {}", reg, num),
            Self::Draw(reg_x, reg_y, num) => write!(f, "DRAW {}, {}, {}", reg_x, reg_y, num),
            Self::KeyEq(reg_x)            => write!(f, "SKIP.EQ K{}", reg_x),
            Self::KeyNeq(reg_x)           => write!(f, "SKIP.NE K{}", reg_x),
            Self::GetDelay(reg_x)         => write!(f, "MOV {}, DT", reg_x),
            Self::WaitKey(reg_x)          => write!(f, "WAIT {}", reg_x),
            Self::SetDelay(reg_x)         => write!(f, "MOV DT, {}", reg_x),
            Self::SetSound(reg_x)         => write!(f, "MOV ST, {}", reg_x),
            Self::AddIdx(reg_x)           => write!(f, "ADD I, {}", reg_x),
            Self::SetSprite(reg_x)        => write!(f, "MOV I, {} * 5", reg_x),
            Self::StoreBcd(reg_x)         => write!(f, "MOV B, {}", reg_x),
            Self::Store(reg_x)            => write!(f, "MOV [I], ..{}", reg_x),
            Self::Load(reg_x)             => write!(f, "MOV ..{}, [I]", reg_x),
        }
    }
}
