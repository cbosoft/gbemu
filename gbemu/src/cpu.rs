use crate::registers::*;
use crate::instructions;
use crate::memory::Memory;

use crate::log as console_log;




pub struct LR35902 {
    pub cycle: u64,
    pub registers: Registers,
    pub memory: Memory,
}

impl LR35902 {
    pub fn open(rom: Vec<u8>) -> Self {
        console_log("Initialising CPU");
        let mut memory = Memory::new();
        for (i, value) in rom.into_iter().enumerate() {
            let addr = (i + 256) as u16;
            memory.set8(addr, value);
        }
        Self {
            cycle: 0,
            registers: Registers::new(),
            memory: memory,
        }
    }

    fn next_byte(&mut self) -> u8 {
        let byte = self.memory[self.registers.pc];
        self.registers.pc = self.registers.pc.wrapping_add(1);
        byte
    }

    fn next_word(&mut self) -> u16 {
        let lsb = self.next_byte() as u16;
        let msb = self.next_byte() as u16;
        (msb << 8) | lsb
    }

    fn fetch(&mut self) -> u8 {
        self.next_byte()
    }

    fn execute(&mut self, instruction: u8) {
        console_log(format!("Executing {instruction:?}").as_str());
        let cycles_passed = match instruction {

            instructions::NO_OP => 1,
            instructions::STOP => todo!(),

            instructions::JR_s8 => self.jump_relative(),
            instructions::JR_Z_s8 => self.conditional_jump_relative(Flag::Zero),
            instructions::JR_C_s8 => self.conditional_jump_relative(Flag::Carry),

            instructions::INC_B => self.increment8(Register8::B),
            instructions::INC_C => self.increment8(Register8::C),
            instructions::INC_D => self.increment8(Register8::D),
            instructions::INC_E => self.increment8(Register8::E),

            instructions::DEC_B => self.decrement8(Register8::B),
            instructions::DEC_C => self.decrement8(Register8::C),
            instructions::DEC_D => self.decrement8(Register8::D),
            instructions::DEC_E => self.decrement8(Register8::E),

            instructions::INC_BC => self.increment16(Register16::BC),
            instructions::INC_DE => self.increment16(Register16::DE),

            instructions::DEC_BC => self.decrement16(Register16::BC),
            instructions::DEC_DE => self.decrement16(Register16::DE),

            instructions::LD_B_d8 => self.load_byte_to_reg(Register8::B),
            instructions::LD_C_d8 => self.load_byte_to_reg(Register8::C),
            instructions::LD_D_d8 => self.load_byte_to_reg(Register8::D),
            instructions::LD_E_d8 => self.load_byte_to_reg(Register8::E),

            instructions::LD_BC_d16 => self.load_word_to_reg(Register16::BC),
            instructions::LD_DE_d16 => self.load_word_to_reg(Register16::DE),

            instructions::LD_A_aBC => self.load_mem_at_reg_to_reg(Register8::A, Register16::BC),
            instructions::LD_A_aDE => self.load_mem_at_reg_to_reg(Register8::A, Register16::DE),

            instructions::LD_aBC_A => self.load_reg_to_mem_at_reg(Register16::BC, Register8::A),
            instructions::LD_aDE_A => self.load_reg_to_mem_at_reg(Register16::DE, Register8::A),

            instructions::LD_a16_SP => todo!(),

            instructions::RLCA => self.rotate_left_circular(Register8::A),
            instructions::RRCA => self.rotate_right_circular(Register8::A),

            instructions::RLA => self.rotate_left(Register8::A),
            instructions::RRA => self.rotate_right(Register8::A),

            instructions::ADD_A_B => self.add(Register8::B),
            instructions::ADD_A_C => self.add(Register8::C),
            instructions::ADD_HL_BC => self.add16(Register16::HL, Register16::BC),
            instructions::ADD_HL_DE => self.add16(Register16::HL, Register16::DE),

            0x20..=0xFF => todo!(),
        };

        self.cycle += cycles_passed;
    }

    pub fn run(&mut self) {
        loop {
            let instruction = self.fetch();
            self.execute(instruction);
        }
    }

    #[allow(unused)]
    fn run_n(&mut self, v: u16) {
        console_log(format!("Running instructions {} to {} of program", self.registers.pc, self.registers.pc.wrapping_add(v)).as_str());
        for _ in 0..v {
            let instruction = self.fetch();
            self.execute(instruction);
        }
    }
    
    // instruction implementations...

    fn load_word_to_reg(&mut self, reg: Register16) -> u64 {
        let value = self.next_word();
        self.registers.set16(reg, value);
        3
    }

    fn load_byte_to_reg(&mut self, reg: Register8) -> u64 {
        let value = self.next_byte();
        self.registers.set8(reg, value);
        2
    }

    fn load_reg_to_mem_at_reg(&mut self, dest: Register16, src: Register8) -> u64 {
        let addr = self.registers.get16(dest);
        let value = self.registers.get8(src);
        self.memory[addr] = value;
        2
    }

    fn load_mem_at_reg_to_reg(&mut self, dest: Register8, src: Register16) -> u64 {
        let addr = self.registers.get16(src);
        let value = self.memory[addr];
        self.registers.set8(dest, value);
        2
    }

    fn increment8(&mut self, reg: Register8) -> u64 {
        let v = self.registers.get8(reg);

        let half_carry = (v & 0x0F) + 1 > 0x0F;
        let (v, carry) = v.overflowing_add(1);
        self.registers.set8(reg, v);

        self.registers.set_flags(
            v == 0,
            false,
            half_carry,
            carry,
        );

        1
    }

    fn increment16(&mut self, reg: Register16) -> u64 {
        let v = self.registers.get16(reg);
        const BIT_11_MASK: u16 = 0b0000_1111_1111_1111;
        let half_carry: bool = (v & BIT_11_MASK) + 1 > BIT_11_MASK;
        let (v, carry) = v.overflowing_add(1);
        self.registers.set16(reg, v);

        self.registers.set_flags(
            v == 0,
            false,
            half_carry,
            carry,
        );

        1
    }

    fn add(&mut self, reg: Register8) -> u64 {
        let a = self.registers.af.0;
        let v = self.registers.get8(reg);
        let half_carry = (a & 0x0F) + (v & 0x0F) > 0x0F;
        let (a, carry) = a.overflowing_add(v);
        self.registers.af.0 = a;

        self.registers.set_flags(
            a == 0,
            false,
            half_carry,
            carry,
        );

        1
    }

    fn add16(&mut self, left: Register16, right: Register16) -> u64 {
        // 16bit add- halfcarry is from bit11 to 12
        let a = self.registers.get16(left);
        let b = self.registers.get16(right);

        const BIT_11_MASK: u16 = 0b0000_1111_1111_1111;
        let half_carry: bool = (a & BIT_11_MASK) + (b & BIT_11_MASK) > BIT_11_MASK;

        let (a, carry) = a.overflowing_add(b);
        self.registers.set16(left, a);

        self.registers.set_flags(
            a == 0,
            false,
            half_carry,
            carry,
        );

        2
    }



    fn decrement8(&mut self, reg: Register8) -> u64 {
        let v = self.registers.get8(reg);

        let half_carry = (v & 0x0F) - 1 > 0x0F;
        let (v, carry) = v.overflowing_sub(1);
        self.registers.set8(reg, v);

        self.registers.set_flags(
            v == 0,
            true,
            half_carry,
            carry,
        );

        1
    }

    fn decrement16(&mut self, reg: Register16) -> u64 {
        let v = self.registers.get16(reg);
        const BIT_11_MASK: u16 = 0b0000_1111_1111_1111;
        let half_carry: bool = (v & BIT_11_MASK) - 1 > BIT_11_MASK;
        let (v, carry) = v.overflowing_sub(1);
        self.registers.set16(reg, v);

        self.registers.set_flags(
            v == 0,
            true,
            half_carry,
            carry,
        );

        1
    }

    fn sub8(&mut self, reg: Register8) -> u64 {
        let a = self.registers.af.0;
        let v = self.registers.get8(reg);

        let half_carry = (a & 0x0F) - (v & 0x0F) > 0x0F;
        let (a, carry) = a.overflowing_sub(v);
        self.registers.af.0 = a;

        self.registers.set_flags(
            v == 0,
            true,
            half_carry,
            carry,
        );

        1
    }

    fn sub_mem_at_reg(&mut self, reg: Register16) -> u64 {
        let addr = self.registers.get16(reg);
        let a = self.registers.af.0;
        let v = self.memory[addr];

        let half_carry = (a & 0x0F) - (v & 0x0F) > 0x0F;
        let (a, carry) = a.overflowing_sub(v);
        self.registers.af.0 = a;

        self.registers.set_flags(
            v == 0,
            true,
            half_carry,
            carry,
        );

        2
    }

    fn sub_byte(&mut self) -> u64 {
        let a = self.registers.af.0;
        let v = self.next_byte();

        let half_carry = (a & 0x0F) - (v & 0x0F) > 0x0F;
        let (a, carry) = a.overflowing_sub(v);
        self.registers.af.0 = a;

        self.registers.set_flags(
            v == 0,
            true,
            half_carry,
            carry,
        );

        2
    }

    fn jump_relative(&mut self) -> u64 {
        let s8 = self.next_byte() as i32;
        let new_pc = (self.registers.pc as i32) + s8;
        self.registers.pc = new_pc as u16;
        3
    }

    fn conditional_jump_relative(&mut self, flag: Flag) -> u64 {
        let s8 = self.next_byte() as i32;

        if self.registers.get_flag(flag) {
            let new_pc = (self.registers.pc as i32) + s8;
            self.registers.pc = new_pc as u16;
            3
        }
        else {
            2
        }

    }

    fn rotate_left_circular(&mut self, reg: Register8) -> u64 {
        let v = self.registers.get8(reg);
        let c = (v & 0b1000_0000) >> 8;
        let v = (v << 1) | c;
        self.registers.set_flags(false, false, false, c == 1);
        self.registers.set8(reg, v);
        1
    }

    fn rotate_left(&mut self, reg: Register8) -> u64 {
        let v = self.registers.get8(reg);
        let new_carry = (v & 0b1000_0000) >> 8;
        let new_lsb = self.registers.get_flag(Flag::Carry) as u8;
        let v = (v << 1) | new_lsb;
        self.registers.set_flags(false, false, false, new_carry == 1);
        self.registers.set8(reg, v);
        1
    }

    fn rotate_right_circular(&mut self, reg: Register8) -> u64 {
        let v = self.registers.get8(reg);
        let c = v & 0b0000_0001;
        let v = (v >> 1) | (c << 8);
        self.registers.set_flags(false, false, false, c == 1);
        self.registers.set8(reg, v);
        1
    }

    fn rotate_right(&mut self, reg: Register8) -> u64 {
        let v = self.registers.get8(reg);
        let new_carry = v & 0b0000_0001;
        let new_msb = self.registers.get_flag(Flag::Carry) as u8;
        let v = (v << 1) | (new_msb << 8);
        self.registers.set_flags(false, false, false, new_carry == 1);
        self.registers.set8(reg, v);
        1
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let program: Vec<u8> = vec![
            instructions::NO_OP,
            instructions::ADD_A_B,
            instructions::LD_C_d8, 0x12,
            instructions::ADD_A_C,
        ];
        let mut cpu = LR35902::open(program);
        cpu.run_n(2);
        assert_eq!(cpu.registers.af.0, 0x01);
        cpu.run_n(2);
        assert_eq!(cpu.registers.get8(Register8::C), 0x12);
        assert_eq!(cpu.registers.af.0, 0x13);
    }
}