mod register;

//use crate::binary;

use register::Register;

pub struct Cpu {
    pub register: Register,
}

impl Cpu {
    pub fn init() -> Cpu {
        Cpu { register: Register::init() }
    }
    pub fn to_string(&self) -> String {
        format!("{}\n{} = 0x{:X}", self.register.to_string(), "EIP", self.register.eip)
    }
/*
    pub fn set_mem_size(self, size:usize) -> State {
        State {memory: vec![0x0000; size], .. self }
    }
    pub fn set_eip(self, eip: u32) -> State {
        let register_ = Register { eip:eip, .. self.register};
        State {register: register_, .. self }
    }
    pub fn set_esp(self, esp: u32) -> State {
        let mut gr = self.register.general;
        gr[GRNames::ESP.to_idx()] = esp;
        let register_ = Register {general: gr, .. self.register };
        State {register: register_, .. self}
    }
    pub fn allocate(self, data:&binary::BinFile, offset: usize) -> State {
        let mut mem_ = self.memory;
        if data.len() + offset >= mem_.len() {
            panic!("Out of Memory @ allocate ")
        } else {
            for (i, b) in data.iter().enumerate() {
                mem_[offset + i] = b.clone();
            }
            State {memory: mem_, .. self}
        }
    }
    pub fn inc_pc(self, diff: i32) -> State {
        let eip_ = ((self.register.eip as i32) + diff) as u32;
        let reg = Register {eip: eip_, .. self.register};
        State {register: reg, .. self}
    }

    pub fn update_register(self, ireg:usize, val:u32) -> State {
        let mut gr = self.register.general;
        gr[ireg] = val;
        let reg = Register {general: gr, .. self.register};
        State {register: reg, .. self}
    }

    pub fn fetch_u8(&self, offset: i32) -> u8 {
        let idx = (self.register.eip as i32) + offset;
        self.memory[idx as usize]
    }

    pub fn fetch_i8(&self, offset: i32) -> i8 {
        let idx = (self.register.eip as i32) + offset;
        self.memory[idx as usize] as i8
    }

    pub fn fetch_u32(&self, offset: i32) -> u32 {
        let mut res: u32 = 0;
        for i in 0..4 {
            res |= (self.fetch_u8(offset + i) as u32) << (i * 8);
        }
        res
    }
    pub fn fetch_i32(&self, offset:i32) -> i32 {
        self.fetch_u32(offset) as i32
    } */

}
