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
            // ...
            // Instruction::LoadStackPointerIntoMemory(v) => { todo!(); },
            // ...
            instructions::LD_C_d8 => {
                let arg = self.next_byte();
                self.load_immediate8(TargetRegister8::C, arg)
            },
            // ...
            instructions::ADD_A_B => self.add(TargetRegister8::B),
            instructions::ADD_A_C => self.add(TargetRegister8::C),
            // ...
            _ => 1,
        };

        self.cycle += cycles_passed;
    }

    pub fn run(&mut self) {
        loop {
            let instruction = self.fetch();
            self.execute(instruction);
        }
    }

    fn run_n(&mut self, v: u16) {
        console_log(format!("Running instructions {} to {} of program", self.registers.pc, self.registers.pc.wrapping_add(v)).as_str());
        for _ in 0..v {
            let instruction = self.fetch();
            self.execute(instruction);
        }
    }
    
    // instruction implementations...

    fn load_immediate8(&mut self, target: TargetRegister8, value: u8) -> u64 {
        self.registers.set8(target, value);
        2
    }

    fn add(&mut self, target: TargetRegister8) -> u64 {
        let a = self.registers.af.0;
        let v = self.registers.get8(target);
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
        assert_eq!(cpu.registers.get8(TargetRegister8::C), 0x12);
        assert_eq!(cpu.registers.af.0, 0x13);
    }
}