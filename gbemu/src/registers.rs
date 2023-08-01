use crate::word::Word;

// CPU registers, 4 two byte words consisting of two 8bit registers, little endian
pub struct Registers {
    pub af: Word,
    pub bc: Word,
    pub de: Word,
    pub hl: Word,

    pub pc: u16,
    pub sp: u16,
}

#[derive(Copy, Clone, Debug)]
pub enum Register8 {
    A, B, C, D, E, F, H, L
}

#[derive(Copy, Clone, Debug)]
pub enum Register16 {
    AF, BC, DE, HL, SP, PC
}

pub enum Flag {
    Zero, Subtract, HalfCarry, Carry
}

impl Registers {
    const ZERO_FLAG_MASK: u8 = 0b1000_0000;
    const SUBTRACT_FLAG_MASK: u8 = 0b0100_0000;
    const HALF_CARRY_FLAG_MASK: u8 = 0b0010_0000;
    const CARRY_FLAG_MASK: u8 = 0b0001_0000;

    pub fn new() -> Self {
        Self {
            af: Word(0x01, 0xb0),
            bc: Word(0x00, 0x13),
            de: Word(0x00, 0xd8),
            hl: Word(0x01, 0x4d),

            pc: 0x0100,
            sp: 0xfffe,
        }
    }

    pub fn set8(&mut self, reg: Register8, value: u8)  {
        match reg {
            Register8::A => { self.af.0 = value; }
            Register8::B => { self.bc.0 = value; }
            Register8::C => { self.bc.1 = value; }
            Register8::D => { self.de.0 = value; }
            Register8::E => { self.de.1 = value; }
            Register8::F => { self.af.1 = value; }
            Register8::H => { self.hl.0 = value; }
            Register8::L => { self.hl.1 = value; }
        }
    }

    pub fn get8(&self, reg: Register8) -> u8 {
        match reg {
            Register8::A => { self.af.0 }
            Register8::B => { self.bc.0 }
            Register8::C => { self.bc.1 }
            Register8::D => { self.de.0 }
            Register8::E => { self.de.1 }
            Register8::F => { self.af.1 }
            Register8::H => { self.hl.0 }
            Register8::L => { self.hl.1 }
        }
    }

    pub fn set16(&mut self, reg: Register16, value: u16)  {
        match reg {
            Register16::AF => { self.af.set16(value); }
            Register16::BC => { self.bc.set16(value); }
            Register16::DE => { self.de.set16(value); }
            Register16::HL => { self.hl.set16(value); }
            Register16::PC => { self.pc = value; },
            Register16::SP => { self.sp = value; },
        }
    }

    pub fn get16(&mut self, reg: Register16) -> u16  {
        match reg {
            Register16::AF => { self.af.get16() }
            Register16::BC => { self.bc.get16() }
            Register16::DE => { self.de.get16() }
            Register16::HL => { self.hl.get16() }
            Register16::PC => { self.pc },
            Register16::SP => { self.sp },
        }
    }

    pub fn set_flag(&mut self, flag: Flag, value: bool) {
        let v = match flag {
            Flag::Zero => Self::ZERO_FLAG_MASK,
            Flag::Subtract => Self::SUBTRACT_FLAG_MASK,
            Flag::HalfCarry => Self::HALF_CARRY_FLAG_MASK,
            Flag::Carry => Self::CARRY_FLAG_MASK,
        };

        if value {
            self.af.1 |= v;
        }
        else {
            self.af.1 &= !v;
        }
    }

    pub fn set_flags(&mut self, zero: bool, subtract: bool, half_carry: bool, carry: bool) {
        self.af.1 = 0;
        if zero { self.af.1 |= Self::ZERO_FLAG_MASK; }
        if subtract { self.af.1 |= Self::SUBTRACT_FLAG_MASK; }
        if half_carry { self.af.1 |= Self::HALF_CARRY_FLAG_MASK; }
        if carry { self.af.1 |= Self::CARRY_FLAG_MASK; }
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        let v = match flag {
            Flag::Zero => Self::ZERO_FLAG_MASK,
            Flag::Subtract => Self::SUBTRACT_FLAG_MASK,
            Flag::HalfCarry => Self::HALF_CARRY_FLAG_MASK,
            Flag::Carry => Self::CARRY_FLAG_MASK,
        };
        (self.af.1 & v) != 0
    }
}