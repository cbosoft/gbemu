// https://meganesu.github.io/generate-gb-opcodes/
#[allow(non_upper_case_globals)]
pub mod Instructions {
    /*
        0x00 NOP
        B1 C1
        Flags - - - -
        No operation.
    */
    pub const NO_OP: u8 = 0x00;

    /*
        0x01 LD BC, d16
        B3 C3
        Flags - - - -
        Load immediate two bytes into reg BC
    */
    pub const LD_BC_d16: u8 = 0x01;

    /*
        0x02 LD (BC), A
        B1 C2
        Flags - - - -
        Load contents of A into mem specified by BC
    */
    pub const LD_aBC_A: u8 = 0x02;

    /*
        0x03 INC BC
        B1 C2
        Flags - - - -
        Increment BC by 1
    */
    pub const INC_BC: u8 = 0x03;

    /*
        0x04 INC B
        B1 C1
        Flags Z 0 H -
        Increment B by 1
    */
    pub const INC_B: u8 = 0x04;

    /*
        0x05 DEC B
        B1 C1
        Flags Z 1 H -
        Decrement B by 1
    */
    pub const DEC_B: u8 = 0x05;

    /*
        0x06 LD B, d8
        B2 C2
        Flags - - - -
        Load immediate next byte into B
    */
    pub const LD_B_d8: u8 = 0x06;

    /*
        0x07 RLCA
        B1 C1
        Flags 0 0 0 A7
        rotate A left 1 (left-shift 1 and store pos 8 back in pos 0 as well as carry flag)
    */
    pub const RLCA: u8 = 0x07;

    /*
        0x08 LD (a16), SP
        B3 C5
        Flags - - - -
        Store stack pointer value at address specified by next immediate word
    */
    pub const LD_a16_SP: u8 = 0x08;

    /*
        0x09 ADD HL, BC
        B1 C2
        Flags - 0 H CY
        Add BC to HL, store in HL
    */
    pub const ADD_HL_CB: u8 = 0x09;

    /*
        0x0A LD A, (BC)
        B1 C2
        Flags - - - -
        Load 8bits of memory at position specified by BC into A
    */
    pub const LD_A_aBC: u8 = 0x0A;

    /*
        0x0B DEC BC
        B1 C2
        Flags - - - -
        decrement BC
    */
    pub const DEC_BC: u8 = 0x0B;

    /*
        0x0C INC C
        B1 C1
        Flags Z 0 H -
    */
    pub const INC_C: u8 = 0x0C;
    
    /*
        0x0D DEC C
        B1 C1
        Flags Z 1 H 0
    */
    pub const DEC_C: u8 = 0x0D;

    /*
        0x0E LD C, d8
        B2 C2
        Flags - - - -
        Load immediate next byte into C
    */
    pub const LD_C_d8: u8 = 0x0E;

    /*
        0x0F RRCA
        B1 C1
        Flags 0 0 0 A0
        Rotate A right, store A0 in A7 and carry
    */
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

}
