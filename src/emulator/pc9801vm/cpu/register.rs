pub enum GRNames { EAX, EBX, ECX, EDX, ESI, EDI, ESP, EBP }

pub struct Register {
    pub general: [u32; 8],
    pub eflags: u32,
    pub eip: u32,
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
