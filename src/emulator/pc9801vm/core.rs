use crate::binary;

pub enum GRNames { EAX, EBX, ECX, EDX, ESI, EDI, ESP, EBP }

pub struct Register {
    pub general: [u32; 8],
    pub eflags: u32,
    pub eip: u32,
}

pub struct State {
    pub register: Register,
    pub memory: Vec<u8>,
}

impl GRNames {
    pub fn to_idx(&self) -> usize {
        match self {
            GRNames::EAX => 0, 
            GRNames::EBX => 1,
            GRNames::ECX => 2,
            GRNames::EDX => 3,
            GRNames::ESI => 4,
            GRNames::EDI => 5,
            GRNames::ESP => 6,
            GRNames::EBP => 7,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            GRNames::EAX => "EAX", 
            GRNames::EBX => "EBX",
            GRNames::ECX => "ECX",
            GRNames::EDX => "EDX",
            GRNames::ESI => "ESI",
            GRNames::EDI => "EDI",
            GRNames::ESP => "ESP",
            GRNames::EBP => "EBP",
        }.to_string()
    }
    pub fn all() -> [GRNames; 8]{
/*        [GRNames::EAX, GRNames::EBX, GRNames::ECX,
         GRNames::EDX, GRNames::ESI, GRNames::EDI,
         GRNames::ESP, GRNames::EBP] */
        [GRNames::EAX, GRNames::ECX, GRNames::EDX,
         GRNames::EBX, GRNames::ESP, GRNames::EBP,
         GRNames::ESI, GRNames::EDI]
    }
}

impl Register {
    pub fn init() -> Register {
        Register {general: [0x0000; 8], eflags: 0x0000, eip: 0x0000}
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

impl State {
    pub fn init() -> State {
        State {register: Register::init(), memory: vec![]}
    }
    pub fn to_string(&self) -> String {
        format!("{}\n{} = 0x{:X}",
                self.register.to_string(),
                "EIP",
                self.register.eip)
    }
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
    }

}
