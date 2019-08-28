use super::core::State;

use log::error;

pub enum Opcode {
    Dummy_code,
    Short_jump,
    Near_jump,
    Mov_r32_imm32(u8),
}

pub enum Disp { Disp8(i8), Disp32(u32) }

pub struct ModRM {
    md: u8,
    ope_ri: u8,
    rm: u8,
    sib: u8,
    disp: Disp,
}

pub struct Code {
    prefix: Option<u32>,
    opcode: Opcode,
    modrm: Option<ModRM>,
    imid: Option<u32>,
}

impl Code {
    pub fn init(opcode: Opcode) -> Code {
        Code {prefix: None, opcode: opcode, modrm: None, imid: None}
    }
    pub fn update_imid(self, imid: u32) -> Code {
        Code {imid: Some(imid), .. self}
    }
}

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
