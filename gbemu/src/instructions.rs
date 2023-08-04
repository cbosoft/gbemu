#![allow(non_upper_case_globals)]

// https://meganesu.github.io/generate-gb-opcodes/

    
// 0x0*
pub const NO_OP: u8 = 0x00;
pub const LD_BC_d16: u8 = 0x01;
pub const LD_aBC_A: u8 = 0x02;
pub const INC_BC: u8 = 0x03;
pub const INC_B: u8 = 0x04;
pub const DEC_B: u8 = 0x05;
pub const LD_B_d8: u8 = 0x06;
pub const RLCA: u8 = 0x07;
pub const LD_a16_SP: u8 = 0x08;
pub const ADD_HL_BC: u8 = 0x09;
pub const LD_A_aBC: u8 = 0x0A;
pub const DEC_BC: u8 = 0x0B;
pub const INC_C: u8 = 0x0C;
pub const DEC_C: u8 = 0x0D;
pub const LD_C_d8: u8 = 0x0E;
pub const RRCA: u8 = 0x0F;

// 0x1*
pub const STOP: u8 = 0x10;
pub const LD_DE_d16: u8 = 0x11;
pub const LD_aDE_A: u8 = 0x12;
pub const INC_DE: u8 = 0x13;
pub const INC_D: u8 = 0x14;
pub const DEC_D: u8 = 0x15;
pub const LD_D_d8: u8 = 0x16;
pub const RLA: u8 = 0x17;
pub const JR_s8: u8 = 0x18;
pub const ADD_HL_DE: u8 = 0x19;
pub const LD_A_aDE: u8 = 0x1A;
pub const DEC_DE: u8 = 0x1B;
pub const INC_E: u8 = 0x1C;
pub const DEC_E: u8 = 0x1D;
pub const LD_E_d8: u8 = 0x1E;
pub const RRA: u8 = 0x1F;

// 0x2*
pub const JR_NZ_s8: u8 = 0x20;
pub const LD_HL_d16: u8 = 0x21;
pub const LD_aHLp_A: u8 = 0x22;
pub const INC_HL: u8 = 0x23;
pub const INC_H: u8 = 0x24;
pub const DEC_H: u8 = 0x25;
pub const LD_H_d8: u8 = 0x26;
pub const DAA: u8 = 0x27;
pub const JR_Z_s8: u8 = 0x28;
pub const ADD_HL_HL: u8 = 0x29;
pub const LD_A_aHLp: u8 = 0x2A;
pub const DEC_HL: u8 = 0x2B;
pub const INC_L: u8 = 0x2C;
pub const DEC_L: u8 = 0x2D;
pub const LD_L_d8: u8 = 0x2E;
pub const CPL: u8 = 0x2F;

// 0x3*
pub const JR_NC_s8: u8 = 0x30;
pub const LD_SP_d16: u8 = 0x31;
pub const LD_aHLm_A: u8 = 0x32;
pub const INC_SP: u8 = 0x33;
pub const INC_aHL: u8 = 0x34;
pub const DEC_aHL: u8 = 0x35;
pub const LD_aHL_d8: u8 = 0x36;
pub const SCF: u8 = 0x37;
pub const JR_C_s8: u8 = 0x38;
pub const ADD_HL_SP: u8 = 0x39;
pub const LD_A_aHLm: u8 = 0x3A;
pub const DEC_SP: u8 = 0x3B;
pub const INC_A: u8 = 0x3C;
pub const DEC_A: u8 = 0x3D;
pub const LD_A_d8: u8 = 0x3E;
pub const CCF: u8 = 0x3F;

// 0x4*
pub const LD_B_B: u8 = 0x40;
pub const LD_B_C: u8 = 0x41;
pub const LD_B_D: u8 = 0x42;
pub const LD_B_E: u8 = 0x43;
pub const LD_B_H: u8 = 0x44;
pub const LD_B_L: u8 = 0x45;
pub const LD_B_aHL: u8 = 0x46;
pub const LD_B_A: u8 = 0x47;
pub const LD_C_B: u8 = 0x48;
pub const LD_C_C: u8 = 0x49;
pub const LD_C_D: u8 = 0x4A;
pub const LD_C_E: u8 = 0x4B;
pub const LD_C_H: u8 = 0x4C;
pub const LD_C_L: u8 = 0x4D;
pub const LD_C_aHL: u8 = 0x4E;
pub const LD_C_A: u8 = 0x4F;

// 0x5*
pub const LD_D_B: u8 = 0x50;
pub const LD_D_C: u8 = 0x51;
pub const LD_D_D: u8 = 0x52;
pub const LD_D_E: u8 = 0x53;
pub const LD_D_H: u8 = 0x54;
pub const LD_D_L: u8 = 0x55;
pub const LD_D_aHL: u8 = 0x56;
pub const LD_D_A: u8 = 0x57;
pub const LD_E_B: u8 = 0x58;
pub const LD_E_C: u8 = 0x59;
pub const LD_E_D: u8 = 0x5A;
pub const LD_E_E: u8 = 0x5B;
pub const LD_E_H: u8 = 0x5C;
pub const LD_E_L: u8 = 0x5D;
pub const LD_E_aHL: u8 = 0x5E;
pub const LD_E_A: u8 = 0x5F;

// 0x6*
pub const LD_H_B: u8 = 0x60;
pub const LD_H_C: u8 = 0x61;
pub const LD_H_D: u8 = 0x62;
pub const LD_H_E: u8 = 0x63;
pub const LD_H_H: u8 = 0x64;
pub const LD_H_L: u8 = 0x65;
pub const LD_H_aHL: u8 = 0x66;
pub const LD_H_A: u8 = 0x67;
pub const LD_L_B: u8 = 0x68;
pub const LD_L_C: u8 = 0x69;
pub const LD_L_D: u8 = 0x6A;
pub const LD_L_E: u8 = 0x6B;
pub const LD_L_H: u8 = 0x6C;
pub const LD_L_L: u8 = 0x6D;
pub const LD_L_aHL: u8 = 0x6E;
pub const LD_L_A: u8 = 0x6F;

// 0x7*
pub const LD_aHL_B: u8 = 0x70;
pub const LD_aHL_C: u8 = 0x71;
pub const LD_aHL_D: u8 = 0x72;
pub const LD_aHL_E: u8 = 0x73;
pub const LD_aHL_H: u8 = 0x74;
pub const LD_aHL_L: u8 = 0x75;
pub const HALT: u8 = 0x76;
pub const LD_aHL_A: u8 = 0x77;
pub const LD_A_B: u8 = 0x78;
pub const LD_A_C: u8 = 0x79;
pub const LD_A_D: u8 = 0x7A;
pub const LD_A_E: u8 = 0x7B;
pub const LD_A_H: u8 = 0x7C;
pub const LD_A_L: u8 = 0x7D;
pub const LD_A_aHL: u8 = 0x7E;
pub const LD_A_A: u8 = 0x7F;



// // 0xY*
// pub const XX: u8 = 0xY0;
// pub const XX: u8 = 0xY1;
// pub const XX: u8 = 0xY2;
// pub const XX: u8 = 0xY3;
// pub const XX: u8 = 0xY4;
// pub const XX: u8 = 0xY5;
// pub const XX: u8 = 0xY6;
// pub const XX: u8 = 0xY7;
// pub const XX: u8 = 0xY8;
// pub const XX: u8 = 0xY9;
// pub const XX: u8 = 0xYA;
// pub const XX: u8 = 0xYB;
// pub const XX: u8 = 0xYC;
// pub const XX: u8 = 0xYD;
// pub const XX: u8 = 0xYE;
// pub const XX: u8 = 0xYF;

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

pub const PREFIX: u8 = 0xCB;

// TODO ...

/*
    0xF8 LD HL, SP+s8
    B2 C3
    Flags 0 0 H C
    Add signed integer 8 to stack pointer, store result in hl
*/
pub const LD_HL_SP_ADD_s8: u8 = 0xF8;
