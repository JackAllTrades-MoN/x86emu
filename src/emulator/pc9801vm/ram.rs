pub struct Ram([u8; 384000]);
pub struct GVRam(pub [u8; 256000]); // 256Kbyte if graphics board with 16 colors is used
// 192Kbyte is used if not
//pub struct GVRam(pub [u8; 192000]); // 256Kbyte if graphics board with 16 colors is used

impl Ram {
    pub fn init() -> Ram { Ram([0; 384000]) }
}

impl GVRam {
    pub fn init() -> GVRam { GVRam([0; 256000]) }
}
