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

impl Register {
    pub fn init() -> Register {
        Register {general: [0x0000; 8], eflags: 0x0000, eip: 0x0000}
    }
    pub fn to_string(&self) -> String {
        format!("EAX = 0x{}\nECX = 0x{}\n",
                self.general[0],
                self.general[1])
    }
}

impl State {
    pub fn init() -> State {
        State {register: Register::init(), memory: vec![]}
    }
    pub fn to_string(&self) -> String {
        self.register.to_string()
    }
}
