#![allow(non_upper_case_globals)]

// https://meganesu.github.io/generate-gb-opcodes/

    
//0x0*
pub const NO_OP: u8 = 0x00;
pub const LD_BC_d16: u8 = 0x01;
pub const LD_aBC_A: u8 = 0x02;
pub const INC_BC: u8 = 0x03;
pub const INC_B: u8 = 0x04;
pub const DEC_B: u8 = 0x05;
pub const LD_B_d8: u8 = 0x06;
pub const RLCA: u8 = 0x07;
pub const LD_a16_SP: u8 = 0x08;
pub const ADD_HL_CB: u8 = 0x09;
pub const LD_A_aBC: u8 = 0x0A;
pub const DEC_BC: u8 = 0x0B;
pub const INC_C: u8 = 0x0C;
pub const DEC_C: u8 = 0x0D;
pub const LD_C_d8: u8 = 0x0E;
pub const RRCA: u8 = 0x0F;

// TODO ...

/*
    0x80 ADD A, B
    B1 C1
    Flags Z 0 H CY
    Add A and B, store result in A
*/
pub const ADD_A_B: u8 = 0x80;

/*
    0x81 ADD A, C
    B1 C1
    Flags Z 0 H CY
    Add A and B, store result in A
*/
pub const ADD_A_C: u8 = 0x81;

// TODO ...

/*
    0xF8 LD HL, SP+s8
    B2 C3
    Flags 0 0 H C
    Add signed integer 8 to stack pointer, store result in hl
*/
pub const LD_HL_SP_ADD_s8: u8 = 0xF8;
