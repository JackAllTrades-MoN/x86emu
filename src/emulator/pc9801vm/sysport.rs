use std::cell::Cell;

pub struct SysPort {
    pub a: Cell<u8>, // read only port
    pub b: Cell<u8>, // read only port
    pub c: Cell<u8>, // writable port
}
impl SysPort {
    pub fn init() -> SysPort {
        SysPort{a:Cell::new(0), b:Cell::new(0), c:Cell::new(0),}
    }
    pub fn turn_on_buzzer(&mut self) -> () {
        self.c.set(self.c.get() | 0b00001000);
    }
}
