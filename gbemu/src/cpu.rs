use crate::registers::*;
use crate::instructions;
use crate::memory::Memory;

use crate::log as console_log;

enum RegisterAction {
    Decrement,
    Nothing,
    Increment,
}


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
            instructions::PREFIX => self.execute_prefixed(),
            instructions::DAA => todo!(),
            instructions::SCF => todo!(),
            instructions::CPL => self.invert_reg8(Register8::A),
            instructions::CCF => self.invert_carry(),
            instructions::HALT => todo!(),

            instructions::JR_s8 => self.jump_relative(),
            instructions::JR_Z_s8 => self.conditional_jump_relative(Flag::Zero, true),
            instructions::JR_C_s8 => self.conditional_jump_relative(Flag::Carry, true),
            instructions::JR_NZ_s8 => self.conditional_jump_relative(Flag::Zero, false),
            instructions::JR_NC_s8 => self.conditional_jump_relative(Flag::Carry, false),

            instructions::INC_A => self.increment8(Register8::A),
            instructions::INC_B => self.increment8(Register8::B),
            instructions::INC_C => self.increment8(Register8::C),
            instructions::INC_D => self.increment8(Register8::D),
            instructions::INC_E => self.increment8(Register8::E),
            instructions::INC_H => self.increment8(Register8::H),
            instructions::INC_L => self.increment8(Register8::L),

            instructions::DEC_A => self.decrement8(Register8::A),
            instructions::DEC_B => self.decrement8(Register8::B),
            instructions::DEC_C => self.decrement8(Register8::C),
            instructions::DEC_D => self.decrement8(Register8::D),
            instructions::DEC_E => self.decrement8(Register8::E),
            instructions::DEC_H => self.decrement8(Register8::E),
            instructions::DEC_L => self.decrement8(Register8::L),

            instructions::INC_BC => self.increment16(Register16::BC),
            instructions::INC_DE => self.increment16(Register16::DE),
            instructions::INC_HL => self.increment16(Register16::HL),
            instructions::INC_SP => self.increment16(Register16::SP),

            instructions::DEC_BC => self.decrement16(Register16::BC),
            instructions::DEC_DE => self.decrement16(Register16::DE),
            instructions::DEC_HL => self.decrement16(Register16::HL),
            instructions::DEC_SP => self.decrement16(Register16::SP),

            instructions::INC_aHL => self.increment_mem_at_reg16(Register16::HL),
            instructions::DEC_aHL => self.decrement_mem_at_reg16(Register16::HL),

            instructions::LD_A_A => self.load_reg_to_reg(Register8::A, Register8::A),
            instructions::LD_A_B => self.load_reg_to_reg(Register8::A, Register8::B),
            instructions::LD_A_C => self.load_reg_to_reg(Register8::A, Register8::C),
            instructions::LD_A_D => self.load_reg_to_reg(Register8::A, Register8::D),
            instructions::LD_A_E => self.load_reg_to_reg(Register8::A, Register8::E),
            instructions::LD_A_H => self.load_reg_to_reg(Register8::A, Register8::H),
            instructions::LD_A_L => self.load_reg_to_reg(Register8::A, Register8::L),

            instructions::LD_B_A => self.load_reg_to_reg(Register8::B, Register8::A),
            instructions::LD_B_B => self.load_reg_to_reg(Register8::B, Register8::B),
            instructions::LD_B_C => self.load_reg_to_reg(Register8::B, Register8::C),
            instructions::LD_B_D => self.load_reg_to_reg(Register8::B, Register8::D),
            instructions::LD_B_E => self.load_reg_to_reg(Register8::B, Register8::E),
            instructions::LD_B_H => self.load_reg_to_reg(Register8::B, Register8::H),
            instructions::LD_B_L => self.load_reg_to_reg(Register8::B, Register8::L),

            instructions::LD_C_A => self.load_reg_to_reg(Register8::C, Register8::A),
            instructions::LD_C_B => self.load_reg_to_reg(Register8::C, Register8::B),
            instructions::LD_C_C => self.load_reg_to_reg(Register8::C, Register8::C),
            instructions::LD_C_D => self.load_reg_to_reg(Register8::C, Register8::D),
            instructions::LD_C_E => self.load_reg_to_reg(Register8::C, Register8::E),
            instructions::LD_C_H => self.load_reg_to_reg(Register8::C, Register8::H),
            instructions::LD_C_L => self.load_reg_to_reg(Register8::C, Register8::L),

            instructions::LD_D_A => self.load_reg_to_reg(Register8::D, Register8::A),
            instructions::LD_D_B => self.load_reg_to_reg(Register8::D, Register8::B),
            instructions::LD_D_C => self.load_reg_to_reg(Register8::D, Register8::C),
            instructions::LD_D_D => self.load_reg_to_reg(Register8::D, Register8::D),
            instructions::LD_D_E => self.load_reg_to_reg(Register8::D, Register8::E),
            instructions::LD_D_H => self.load_reg_to_reg(Register8::D, Register8::H),
            instructions::LD_D_L => self.load_reg_to_reg(Register8::D, Register8::L),

            instructions::LD_E_A => self.load_reg_to_reg(Register8::E, Register8::A),
            instructions::LD_E_B => self.load_reg_to_reg(Register8::E, Register8::B),
            instructions::LD_E_C => self.load_reg_to_reg(Register8::E, Register8::C),
            instructions::LD_E_D => self.load_reg_to_reg(Register8::E, Register8::D),
            instructions::LD_E_E => self.load_reg_to_reg(Register8::E, Register8::E),
            instructions::LD_E_H => self.load_reg_to_reg(Register8::E, Register8::H),
            instructions::LD_E_L => self.load_reg_to_reg(Register8::E, Register8::L),

            instructions::LD_H_A => self.load_reg_to_reg(Register8::H, Register8::A),
            instructions::LD_H_B => self.load_reg_to_reg(Register8::H, Register8::B),
            instructions::LD_H_C => self.load_reg_to_reg(Register8::H, Register8::C),
            instructions::LD_H_D => self.load_reg_to_reg(Register8::H, Register8::D),
            instructions::LD_H_E => self.load_reg_to_reg(Register8::H, Register8::E),
            instructions::LD_H_H => self.load_reg_to_reg(Register8::H, Register8::H),
            instructions::LD_H_L => self.load_reg_to_reg(Register8::H, Register8::L),

            instructions::LD_L_A => self.load_reg_to_reg(Register8::L, Register8::A),
            instructions::LD_L_B => self.load_reg_to_reg(Register8::L, Register8::B),
            instructions::LD_L_C => self.load_reg_to_reg(Register8::L, Register8::C),
            instructions::LD_L_D => self.load_reg_to_reg(Register8::L, Register8::D),
            instructions::LD_L_E => self.load_reg_to_reg(Register8::L, Register8::E),
            instructions::LD_L_H => self.load_reg_to_reg(Register8::L, Register8::H),
            instructions::LD_L_L => self.load_reg_to_reg(Register8::L, Register8::L),

            instructions::LD_A_d8 => self.load_byte_to_reg(Register8::A),
            instructions::LD_B_d8 => self.load_byte_to_reg(Register8::B),
            instructions::LD_C_d8 => self.load_byte_to_reg(Register8::C),
            instructions::LD_D_d8 => self.load_byte_to_reg(Register8::D),
            instructions::LD_E_d8 => self.load_byte_to_reg(Register8::E),
            instructions::LD_H_d8 => self.load_byte_to_reg(Register8::H),
            instructions::LD_L_d8 => self.load_byte_to_reg(Register8::L),

            instructions::LD_BC_d16 => self.load_word_to_reg(Register16::BC),
            instructions::LD_DE_d16 => self.load_word_to_reg(Register16::DE),
            instructions::LD_HL_d16 => self.load_word_to_reg(Register16::HL),
            instructions::LD_SP_d16 => self.load_word_to_reg(Register16::SP),

            instructions::LD_A_aBC => self.load_mem_at_reg_to_reg(Register8::A, Register16::BC, RegisterAction::Nothing),
            instructions::LD_A_aDE => self.load_mem_at_reg_to_reg(Register8::A, Register16::DE, RegisterAction::Nothing),
            instructions::LD_A_aHL => self.load_mem_at_reg_to_reg(Register8::A, Register16::HL, RegisterAction::Nothing),
            instructions::LD_B_aHL => self.load_mem_at_reg_to_reg(Register8::B, Register16::HL, RegisterAction::Nothing),
            instructions::LD_C_aHL => self.load_mem_at_reg_to_reg(Register8::C, Register16::HL, RegisterAction::Nothing),
            instructions::LD_D_aHL => self.load_mem_at_reg_to_reg(Register8::D, Register16::HL, RegisterAction::Nothing),
            instructions::LD_E_aHL => self.load_mem_at_reg_to_reg(Register8::E, Register16::HL, RegisterAction::Nothing),
            instructions::LD_H_aHL => self.load_mem_at_reg_to_reg(Register8::H, Register16::HL, RegisterAction::Nothing),
            instructions::LD_L_aHL => self.load_mem_at_reg_to_reg(Register8::L, Register16::HL, RegisterAction::Nothing),

            instructions::LD_A_aHLp => self.load_mem_at_reg_to_reg(Register8::A, Register16::HL, RegisterAction::Increment),
            instructions::LD_A_aHLm => self.load_mem_at_reg_to_reg(Register8::A, Register16::HL, RegisterAction::Decrement),

            instructions::LD_aBC_A => self.load_reg_to_mem_at_reg(Register16::BC, Register8::A, RegisterAction::Nothing),
            instructions::LD_aDE_A => self.load_reg_to_mem_at_reg(Register16::DE, Register8::A, RegisterAction::Nothing),
            instructions::LD_aHL_A => self.load_reg_to_mem_at_reg(Register16::HL, Register8::A, RegisterAction::Nothing),
            instructions::LD_aHL_B => self.load_reg_to_mem_at_reg(Register16::HL, Register8::B, RegisterAction::Nothing),
            instructions::LD_aHL_C => self.load_reg_to_mem_at_reg(Register16::HL, Register8::C, RegisterAction::Nothing),
            instructions::LD_aHL_D => self.load_reg_to_mem_at_reg(Register16::HL, Register8::D, RegisterAction::Nothing),
            instructions::LD_aHL_E => self.load_reg_to_mem_at_reg(Register16::HL, Register8::E, RegisterAction::Nothing),
            instructions::LD_aHL_H => self.load_reg_to_mem_at_reg(Register16::HL, Register8::H, RegisterAction::Nothing),
            instructions::LD_aHL_L => self.load_reg_to_mem_at_reg(Register16::HL, Register8::L, RegisterAction::Nothing),

            instructions::LD_aHLp_A => self.load_reg_to_mem_at_reg(Register16::HL, Register8::A, RegisterAction::Increment),
            instructions::LD_aHLm_A => self.load_reg_to_mem_at_reg(Register16::HL, Register8::A, RegisterAction::Decrement),

            instructions::LD_aHL_d8 => self.load_byte_to_mem_at_reg(Register16::HL),

            instructions::LD_a16_SP => self.load_reg16_to_mem(Register16::SP),

            instructions::RLCA => self.rotate_left_circular(Register8::A),
            instructions::RRCA => self.rotate_right_circular(Register8::A),

            instructions::RLA => self.rotate_left(Register8::A),
            instructions::RRA => self.rotate_right(Register8::A),

            instructions::ADD_A_B => self.add(Register8::B),
            instructions::ADD_A_C => self.add(Register8::C),
            instructions::ADD_HL_BC => self.add16(Register16::HL, Register16::BC),
            instructions::ADD_HL_DE => self.add16(Register16::HL, Register16::DE),
            instructions::ADD_HL_HL => self.add16(Register16::HL, Register16::HL),
            instructions::ADD_HL_SP => self.add16(Register16::HL, Register16::SP),

            0x80..=0xFF => todo!(),
        };

        self.cycle += cycles_passed;
    }

    fn execute_prefixed(&mut self) -> u64 {
        let instruction = self.next_byte();
        match instruction {
            0x00..=0xFF => todo!()
        }
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

    fn load_reg_to_reg(&mut self, dest: Register8, src: Register8) -> u64 {
        let v = self.registers.get8(src);
        self.registers.set8(dest, v);
        1
    }

    fn load_reg_to_mem_at_reg(&mut self, dest: Register16, src: Register8, reg_action: RegisterAction) -> u64 {
        let addr = self.registers.get16(dest);
        let value = self.registers.get8(src);
        self.memory[addr] = value;

        match reg_action {
            RegisterAction::Nothing => (),
            RegisterAction::Decrement => self.registers.set16(dest, addr - 1),
            RegisterAction::Increment => self.registers.set16(dest, addr + 1),
        }

        2
    }

    fn load_reg16_to_mem(&mut self, src: Register16) -> u64 {
        let addr = self.next_word();
        let (lsb, msb) = self.registers.get8_8(src);
        self.memory[addr] = lsb;
        self.memory[addr + 1] = msb;
        5
    }

    fn load_mem_at_reg_to_reg(&mut self, dest: Register8, src: Register16, reg_action: RegisterAction) -> u64 {
        let addr = self.registers.get16(src);
        let value = self.memory[addr];
        self.registers.set8(dest, value);

        match reg_action {
            RegisterAction::Nothing => (),
            RegisterAction::Decrement => self.registers.set16(src, addr - 1),
            RegisterAction::Increment => self.registers.set16(src, addr + 1),
        }

        2
    }

    fn load_byte_to_mem_at_reg(&mut self, reg: Register16) -> u64 {
        let value = self.next_byte();
        let addr = self.registers.get16(reg);
        self.memory[addr] = value;
        3
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

    fn increment_mem_at_reg16(&mut self, reg: Register16) -> u64 {
        let addr = self.registers.get16(reg);
        let v = self.memory[addr];

        let half_carry = (v & 0x0F) + 1 > 0x0F;
        let (v, carry) = v.overflowing_add(1);
        self.memory[addr] = v;

        self.registers.set_flags(
            v == 0,
            false,
            half_carry,
            carry,
        );

        3
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

    fn decrement_mem_at_reg16(&mut self, reg: Register16) -> u64 {
        let addr = self.registers.get16(reg);
        let v = self.memory[addr];

        let half_carry = (v & 0x0F) - 1 > 0x0F;
        let (v, carry) = v.overflowing_sub(1);
        self.memory[addr] = v;

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

    fn conditional_jump_relative(&mut self, flag: Flag, expected_value: bool) -> u64 {
        let s8 = self.next_byte() as i32;

        if self.registers.get_flag(flag) == expected_value {
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
        let c = (v & 0b1000_0000) >> 7;
        let v = (v << 1) | c;
        self.registers.set_flags(false, false, false, c == 1);
        self.registers.set8(reg, v);
        1
    }

    fn rotate_left(&mut self, reg: Register8) -> u64 {
        let v = self.registers.get8(reg);
        let new_carry = (v & 0b1000_0000) >> 7;
        let new_lsb = self.registers.get_flag(Flag::Carry) as u8;
        let v = (v << 1) | new_lsb;
        self.registers.set_flags(false, false, false, new_carry == 1);
        self.registers.set8(reg, v);
        1
    }

    fn rotate_right_circular(&mut self, reg: Register8) -> u64 {
        let v = self.registers.get8(reg);
        let c = v & 0b0000_0001;
        let v = (v >> 1) | (c << 7);
        self.registers.set_flags(false, false, false, c == 1);
        self.registers.set8(reg, v);
        1
    }

    fn rotate_right(&mut self, reg: Register8) -> u64 {
        let v = self.registers.get8(reg);
        let new_carry = v & 0b0000_0001;
        let new_msb = self.registers.get_flag(Flag::Carry) as u8;
        let v = (v << 1) | (new_msb << 7);
        self.registers.set_flags(false, false, false, new_carry == 1);
        self.registers.set8(reg, v);
        1
    }

    // Stack ops
    fn pop_reg16(&mut self, reg: Register16) -> u64 {
        self.registers.sp += 1;
        let lsb = self.memory[self.registers.sp];
        self.registers.sp += 1;
        let msb = self.memory[self.registers.sp];
        self.registers.set8_8(reg, lsb, msb);
        3
    }

    fn push_reg16(&mut self, reg: Register16) -> u64 {
        let (lsb, msb) = self.registers.get8_8(reg);
        let addr = self.registers.sp;
        self.memory[addr] = msb;
        self.registers.sp -= 1;
        let addr = self.registers.sp;
        self.memory[addr] = lsb;
        self.registers.sp -= 1;
        4
    }

    //
    fn invert_carry(&mut self) -> u64 {
        let new_carry_value = !self.registers.get_flag(Flag::Carry);
        self.registers.set_flag(Flag::Subtract, false);
        self.registers.set_flag(Flag::HalfCarry, false);
        self.registers.set_flag(Flag::Carry, new_carry_value);
        1
    }

    fn invert_reg8(&mut self, reg: Register8) -> u64 {
        let v = self.registers.get8(reg);
        self.registers.set8(reg, !v);
        self.registers.set_flags(false, true, true, false);
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