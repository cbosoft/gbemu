use std::ops::{Index, IndexMut};

pub struct Memory {
    memory: [u8; 65536]
}

impl Memory {
    pub fn new() -> Self {
        // TODO initial memory
        Memory { memory: [0u8; 65536] }
    }

    pub fn set8(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }

    pub fn get8(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn set16(&mut self, addr: u16, value: u16) {
        self.memory[addr as usize] = (value >> 8) as u8;
        self.memory[addr as usize + 1] = (value & 0x0F) as u8;
    }

    pub fn get16(&self, addr: u16) -> u16 {
        ((self.memory[addr as usize] as u16) << 8) | (self.memory[addr as usize + 1] as u16)
    }

    pub fn len(&self) -> usize {
        self.memory.len()
    }
}

impl Index<u16> for Memory {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        self.memory.index(index as usize)
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        self.memory.index_mut(index as usize)
    }
}