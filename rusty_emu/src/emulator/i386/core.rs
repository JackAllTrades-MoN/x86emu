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
        [GRNames::EAX, GRNames::EBX, GRNames::ECX,
         GRNames::EDX, GRNames::ESI, GRNames::EDI,
         GRNames::ESP, GRNames::EBP]
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
        self.register.to_string()
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
}
