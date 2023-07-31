use crate::registers::{TargetRegister8, TargetRegister16};

// https://meganesu.github.io/generate-gb-opcodes/
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum Instruction {
    /*
        0x00 NOP
        B1 C1
        Flags - - - -
        No operation.
    */
    NoOperation,

    /*
        0x01 LD BC
        B3 C3
        Flags - - - -
        Load immediate two bytes into reg BC
    */
    LoadIntoBCFromMemory(u16),

    /*
        0x02 LD (BC), A
        B1 C2
        Flags - - - -
        Load contents of A into mem specified by BC
    */
    LoadContentsOfAIntoMemoryAtBC,

    /*
        0x03 INC BC
        B1 C2
        Flags - - - -
        Increment BC by 1
    */
    IncrementBC,

    /*
        0x04 INC B
        B1 C1
        Flags Z 0 H -
        Increment B by 1
    */
    IncrementB,

    /*
        0x05 DEC B
        B1 C1
        Flags Z 1 H -
        Decrement B by 1
    */
    DecrementB,

    /*
        0x06 LD B, d8
        B2 C2
        Flags - - - -
        Load immediate next byte into B
    */
    LoadIntoB(u8),

    /*
        0x07 RLCA
        B1 C1
        Flags 0 0 0 A7
        rotate A left 1 (left-shift 1 and store pos 8 back in pos 0 as well as carry flag)
    */
    RotateALeftCopyIntoCarry,

    /*
        0x08 LD (a16, SP)
        B3 C5
        Flags - - - -
        Store stack pointer value at address specified by next immediate word
    */
    LoadStackPointerIntoMemory(u16),

    /*
        0x09 ADD HL, BC
        B1 C2
        Flags - 0 H CY
        Add BC to HL, store in HL
    */
    AddBCToHL,

    /*
        0x0A LD A, (BC)
        B1 C2
        Flags - - - -
        Load 8bits of memory at position specified by BC into A
    */
    LoadIntoAFromMemoryAtBC,

    /*
        0x0B DEC BC
        B1 C2
        Flags - - - -
        decrement BC
    */
    DecrementBC,

    /*
        0x0C INC C
        B1 C1
        Flags Z 0 H -
    */
    
    IncrementC,
    /*
        0x0D DEC C
        B1 C1
        Flags Z 1 H 0
    */
    DecrementC,

    /*
        0x0E LD C, d8
        B2 C2
        Flags - - - -
        Load immediate next byte into C
    */
    LoadIntoC(u8),

    /*
        0x0F RRCA
        B1 C1
        Flags 0 0 0 A0
        Rotate A right, store A0 in A7 and carry
    */
    RotateARightCopyIntoCarry,

    // TODO ...

    /*
        0x80 ADD A, B
        B1 C1
        Flags Z 0 H CY
        Add A and B, store result in A
    */
    AddBToA,

    /*
        0x81 ADD A, C
        B1 C1
        Flags Z 0 H CY
        Add A and B, store result in A
    */
    AddCToA,

    // TODO ...

    /*
        0F8 LD HL, SP+s8
        B2 C3
        Flags 0 0 H C
        Add signed integer 8 to stack pointer, store result in hl
    */
    AddToSPStoreInHL(i8),
}

impl Instruction {
    pub fn decode_bytes(bytes: Vec<u8>) -> Vec<Instruction> {
        let mut i = 0usize;
        let mut instructions = Vec::new();
        while i < bytes.len() {
            let byte = bytes[i];
            let (inst, adv) = match byte {
                0x00 => (Instruction::NoOperation, 1),
                // ...                                             /* I think I may have the endian-ness wrong here */
                0x08 => (Instruction::LoadStackPointerIntoMemory(((bytes[i+1] as u16) << 8) + (bytes[i+2] as u16)), 3),
                // ...
                0x0E => (Instruction::LoadIntoC(bytes[i+1]), 2),
                // ...
                0x80 => (Instruction::AddBToA, 1),
                0x81 => (Instruction::AddCToA, 1),
                // ...
                0xF8 => (Instruction::AddToSPStoreInHL(bytes[i + 1] as i8), 2),
                _ => { todo!(); }
            };
            instructions.push(inst);
            i += adv;
        }
        instructions
    }
}