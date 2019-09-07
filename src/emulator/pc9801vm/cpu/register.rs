//! # Register for intel80286
//! the official manual is available from: 
//! http://bitsavers.trailing-edge.com/components/intel/80286/210498-005_80286_and_80287_Programmers_Reference_Manual_1987.pdf

#[derive(Copy, Clone)]
pub enum GRNames { AX = 0, BX = 1, CX = 2, DX = 3, SP = 4, BP = 5, SI = 6, DI = 7}

impl GRNames {
    pub fn to_idx(self) -> usize { self as usize }
    pub fn to_string(self) -> String {
        let strs = ["AX", "BX", "CX", "DX", "SP", "BP", "SI", "DI"];
        strs[self.to_idx()].to_string()
    }
    pub fn all() -> [GRNames; 8]{
        [GRNames::AX, GRNames::BX, GRNames::CX, GRNames::DX, 
         GRNames::SP, GRNames::BP, GRNames::SI, GRNames::DI]
    }
}

pub struct Register {
    // GENERAL REGISTERS
    pub general: [u16; 8],
    // SEGMENT REGISTERS
    pub cs: u16, // CODE SEGMENT SELECTOR
    pub ds: u16, // DATA SEGMENT SELECTOR
    pub ss: u16, // STACK SEGMENT SELECTOR
    pub es: u16, // EXTRA SEGMENT SELECTOR
    // STATUS AND CONTROL REGISTERS
    pub flags: u16,   // FLAGS
    pub ip:  u16,      // INSTRUCTION POINTER
    pub msw: u16,     // MACHINE STATUS WORD
}

impl Register {
    pub fn init() -> Register {
        Register {general: [0x00; 8],
                  cs: 0x00,
                  ds: 0x00,
                  ss: 0x00,
                  es: 0x00,
                  flags: 0x00,
                  ip: 0x00,
                  msw: 0x00,}
    }
    pub fn to_string(&self) -> String {
        GRNames::all().iter().fold(
            "".to_string(),
            |acc, x| format!("{}\n{} = 0x{:X}",
                             acc,
                             x.to_string(),
                             self.general[x.to_idx()]))
    }
}
