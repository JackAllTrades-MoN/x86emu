pub type RGB = (u8, u8, u8);
pub struct Crt {
    pub scrx: usize,
    pub scry: usize,
    pub title: String,
    pub palette: [RGB; 16], // 16 colors palette
}

impl Crt {
    pub fn init() -> Crt {
        Crt {
            scrx: 640,
            scry: 400,
            title: String::from("PC9801VM emulator"),
            palette:  [(0x00, 0x00, 0x00), // black
                            (0xff, 0xff, 0xff), // white
                            (0xc0, 0xc0, 0xc0), // silver
                            (0x80, 0x00, 0x00), // maroon
                            (0x80, 0x00, 0x80), // purple
                            (0x00, 0x80, 0x00), // green
                            (0x80, 0x80, 0x00), // olive
                            (0x00, 0x00, 0x80), // navy
                            (0x00, 0x80, 0x80), // teal
                            (0x80, 0x80, 0x80), // gray
                            (0xff, 0x00, 0x00), // red
                            (0xff, 0x00, 0xff), // fuchsia
                            (0x00, 0xff, 0x00), // lime
                            (0xff, 0xff, 0x00), // yellow
                            (0x00, 0x00, 0xff), // blue
                            (0x00, 0xff, 0xff), // aqua
            ], }
    }
    pub fn cpy(&self) -> Crt {
        Crt { scrx: self.scrx, scry: self.scry, title: self.title.to_string(), palette: self.palette}
    }
}
