#[derive(Clone)]
pub struct Word(pub u8, pub u8);

impl Word {

    pub fn get16(&self) -> u16  {
        ((self.0 as u16) << 8) | (self.1 as u16)
    }

    pub fn set16(&mut self, v: u16) {
        self.0 = (v >> 8) as u8;
        self.1 = (v & 255) as u8;
    }

}