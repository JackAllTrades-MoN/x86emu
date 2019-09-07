//! Intel80286 Instructions
//!
//! # Notations
//!
//! - /n: (n is a digit from 0 through 7) A ModRM byte, plus a possible immediate and
//! displacement field follow the opcode.  The digit n is the value of the REG field 
//! of the ModRM byte.
//!
//! - /r: A ModRM byte that contains both a register operand and an effective address
//! operand, followed by a possible immediate and displacement field.
//!
//! - cb: A one-byte signed displacement in the range of -128 to +127 follows the opcode.
//!
//! - cw: A two-byte displacement
//!
//! - cd: A two-word pointer
//!
//! - db: An immediate byte operand to the instruction
//!
//! - dw: An immediate byte word opperand
//!
//! - +rb: A register code from 0 through 7 (AL=0, CL, DL, BL, AH, CH, DH, BH)
//!
//! - +rw: A register code from 0 through 7 (AX=0, CX, DX, BX, SP, BP, SI, DI)
//!
//! - pm: the instruction take more time to execute when the 80286 in Protected Mode
//!   (which will not be implemented here)
//!
//! # Instruction
//!
//! ## "mod" Field Bit Assignments
//! | mod | Displacement |
//! | -:- | -:- |
//! | 00 | DISP = 0, disp-low and disp-high are absent |
//! | 01 | DISP = disp-low sign-extended to 16-bits, disp-high is absent |
//! | 10 | DISP = disp-high; disp-low |
//! | 11 | r/m is treate as a "reg" field |
//!
//! ## "r/m" Field Bit Assignments
//! | r/m | Operand Address |
//! | -:- | -:- |
//! | 000 | (BX) + (SI) + DISP |
//! | 001 | (BX) + (DI) + DISP |
//! | 010 | (BP) + (SI) + DISP |
//! | 011 | (BP) + (DI) + DISP |
//! | 100 | (SI) + DISP |
//! | 101 | (DI) + DISP |
//! | 110 | (BP) + DISP |
//! | 111 | (BX) + DISP |

//use log::error;

/// Segment Override Prefix
pub enum SegOPrefix {
    /// code segment
    CS = 0x2E,
    /// stack segment
    SS = 0x36,
    /// data segment
    DS = 0x3E,
    /// data segment
    ES = 0x26,
    // FS and GS are adoped from i386
}

/// Repeat Prefix
pub enum RepPrefix {
    RepneRepze = 0xF2,
    RepRepeRepz = 0xF3,
}

pub enum Prefix {
    SOP(SegOPrefix),
    RP(RepPrefix),
    // Note: an operand size prefix and an address size prefix are adopted since i386.
}

pub enum Opcode {
    DummyCode,
    ShortJump, JumpTG, NearJump, FarJump,
}

pub enum Disp {
    Absent, Exlow(i16), HL{low:u8, high:u8}
}

#[derive(Copy, Clone)]
pub enum Mod { M00 = 0, M01 = 1, M10 = 2, M11 = 3, }

pub struct ModRM {
    md: Mod,
    n_r: u8, // 3bit
    rm: u8,  // 3bit
}

pub struct Code {
    prefix: Option<u32>,
    opcode: Opcode,
    modrm: Option<ModRM>,
    disp: Disp,
    imid: Option<u16>,
    // sib is adopted since i386
}
/*
impl Code {
    pub fn init(opcode: Opcode) -> Code {
        Code {prefix: None, opcode: opcode, modrm: None, imid: None}
    }
    pub fn update_imid(self, imid: u32) -> Code {
        Code {imid: Some(imid), .. self}
    }
}*/
/*
pub fn exec(s: State, code: Code) -> State {
    match code.opcode {
        Opcode::Short_jump => s.inc_pc(code.imid.unwrap() as i32),
        Opcode::Near_jump => s.inc_pc(code.imid.unwrap() as i32),
        Opcode::Mov_r32_imm32(ireg) =>
            s.update_register(ireg as usize, code.imid.unwrap()),
        _ => unimplemented!(),
    }
}

pub fn fetch_and_decode(s: State) -> (State, Code) {
    match s.fetch_u8(0) {
        x if 0xB8 <= x && x < 0xB8 + 8 => {
            let target_reg = s.fetch_u8(0) - 0xB8;
            let imid = s.fetch_u32(1);
            (s.inc_pc(5),
             Code::init(Opcode::Mov_r32_imm32(target_reg))
             .update_imid(imid))
        },
        0xEB => {
            let imid = s.fetch_i8(1) as u32;
            (s.inc_pc(2), Code::init(Opcode::Short_jump).update_imid(imid))
        },
        0xE9 => {
            let imid = s.fetch_i32(1) as u32;
            (s.inc_pc(5), Code::init(Opcode::Near_jump).update_imid(imid))
        },
        x => {
            error!("unimplemented code: 0x{:X}", x);
            unimplemented!()
        }
    }
}
*/
