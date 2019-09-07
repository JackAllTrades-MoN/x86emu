pub struct Ram([u8; 384000]);
pub struct GVRam(pub [u8; 256000]); // 256Kbyte if graphics board with 16 colors is used
// 192Kbyte is used if not
//pub struct GVRam(pub [u8; 192000]); // 256Kbyte if graphics board with 16 colors is used

impl Ram {
    pub fn init() -> Ram { Ram([0; 384000]) }
    pub fn fetch_u8(&self, addr: usize) -> u8 {
        let Ram(mem) = self;
        mem[addr] as u8
    }
}

impl GVRam {
    pub fn init() -> GVRam { GVRam([0; 256000]) }
}
