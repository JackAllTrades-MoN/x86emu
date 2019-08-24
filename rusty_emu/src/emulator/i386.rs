//! # i386 emulator
//! TODO: write a documentation

mod core;
mod instruction;

use self::core::State;
use log::{debug, info};
use crate::binary;
use crate::config::Config;

pub fn run (_cfg: &Config, filename: &str) {
    fn main_loop (st: State) {
        debug!("{}", &st.to_string());
        let (st, code) = instruction::fetch_and_decode(st);
        let st = instruction::exec(st, code);
        if st.register.eip == 0x00 {
            info!("finish @ i386 emulator");
            info!("{}", &st.to_string());
        } else {
            main_loop(st)
        }
    };
    info!("run @ i386 emulator");
    let bin = binary::load(filename);
    let ist = State::init()
        .set_mem_size(1024 * 1024)
        .set_eip(0x7c00)
        .set_esp(0x7c00)
        .allocate(&bin, 0x7c00);
    main_loop(ist);
}

#[test]
fn test_run() {
    // write an unit test
    assert!(true);
}
