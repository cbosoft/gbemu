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

            instructions::DEC_B => todo!(),
            instructions::DEC_C => todo!(),
            instructions::DEC_D => todo!(),
            instructions::DEC_E => todo!(),

            instructions::INC_BC => self.increment16(Register16::BC),
            instructions::INC_DE => self.increment16(Register16::DE),

            instructions::DEC_BC => todo!(),
            instructions::DEC_DE => todo!(),

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

            instructions::RLCA => todo!(),
            instructions::RRCA => todo!(),

            instructions::RLA => todo!(),
            instructions::RRA => todo!(),

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
        let (v, carry) = v.overflowing_add(1);
        self.registers.set8(reg, v);

        let half_carry: bool = (v & 0xF) + (1 & 0xF) > 0xF;

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
        let (v, carry) = v.overflowing_add(1);
        self.registers.set16(reg, v);

        let half_carry: bool = (v & 0xF) + (1 & 0xF) > 0xF;

        // self.registers.set_flags(
        //     v == 0,
        //     false,
        //     half_carry,
        //     carry,
        // );

        1
    }

    fn add(&mut self, reg: Register8) -> u64 {
        let a = self.registers.af.0;
        let v = self.registers.get8(reg);
        let (a, carry) = a.overflowing_add(v);
        self.registers.af.0 = a;

        let half_carry: bool = (a & 0xF) + (v & 0xF) > 0xF;

        self.registers.set_flags(
            a == 0,
            false,
            half_carry,
            carry,
        );

        1
    }

    fn add16(&mut self, left: Register16, right: Register16) -> u64 {
        let a = self.registers.get16(left);
        let b = self.registers.get16(right);
        let (a, carry) = a.overflowing_add(b);
        self.registers.set16(left, a);

        let half_carry: bool = (a & 0xF) + (b & 0xF) > 0xF;

        self.registers.set_flags(
            a == 0,
            false,
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