use crate::registers::*;
use crate::instructions::Instruction;
use crate::memory::Memory;

use crate::log as console_log;




pub struct LR35902 {
    pub cycle: u64,
    pub registers: Registers,
    pub memory: Memory,
    pub program: Vec<Instruction>,
}

impl LR35902 {
    pub fn open(rom: Vec<u8>) -> Self {
        console_log("Initialising CPU");
        Self {
            cycle: 0,
            registers: Registers::new(),
            memory: Memory::new(),
            program: Instruction::decode_bytes(rom),
        }
    }

    fn fetch(&mut self) -> Option<Instruction> {
        if (self.registers.pc as usize) >= self.program.len() {
            None
        }
        else {
            let inst = self.program[self.registers.pc as usize].clone();
            self.registers.pc = self.registers.pc.wrapping_add(1);
            Some(inst)
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        console_log(format!("Executing {instruction:?}").as_str());
        let cycles_passed = match instruction {
            Instruction::NoOperation => 1,
            // ...
            Instruction::LoadStackPointerIntoMemory(v) => { todo!(); },
            // ...
            Instruction::LoadIntoC(v) => self.load_immediate8(TargetRegister8::C, v),
            // ...
            Instruction::AddBToA => self.add(TargetRegister8::B),
            Instruction::AddCToA => self.add(TargetRegister8::C),
            // ...
            _ => 1,
        };

        self.cycle += cycles_passed;
    }

    pub fn run(&mut self) {
        console_log("Running program");
        while let Some(instruction) = self.fetch() {
            self.execute(instruction);
        }
        console_log("Done!");
    }

    fn run_n(&mut self, v: usize) {
        console_log(format!("Running instructions {} to {} of {} of program", self.registers.pc, self.registers.pc as usize + v, self.program.len()).as_str());
        for _ in 0..v {
            if let Some(instruction) = self.fetch() {
                self.execute(instruction);
            }
            else  {
                panic!();
            }
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
        let mut program: Vec<u8> = (0..256).map(|_| { 0x00}).collect();
        program.extend(vec![

            0x00, // 0. NOOP

            0x80, // 1. ADD A B

            0x0E, // 2. LD C, 8
            0x12,

            0x81, // 3. ADD A C

        ].into_iter());
        let mut cpu = LR35902::open(program);
        println!("{:?} // {} -> {:?}", cpu.program, cpu.registers.pc, cpu.program[cpu.registers.pc as usize]);

        cpu.run_n(1);
        println!("// {} -> {:?}", cpu.registers.pc, cpu.program[cpu.registers.pc as usize]);
        cpu.run_n(1);
        println!("// {} -> {:?}", cpu.registers.pc, cpu.program[cpu.registers.pc as usize]);
        assert_eq!(cpu.registers.af.0, 0x01);
        cpu.run_n(1);
        println!("// {} -> {:?}", cpu.registers.pc, cpu.program[cpu.registers.pc as usize]);
        cpu.run_n(1);
        println!("// {}", cpu.registers.pc);
        assert_eq!(cpu.registers.get8(TargetRegister8::C), 0x12);
        assert_eq!(cpu.registers.af.0, 0x13);
    }
}