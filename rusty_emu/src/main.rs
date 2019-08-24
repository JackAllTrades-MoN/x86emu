//#[macro_use]
//pub extern crate log;
//pub extern crate env_logger;

//mod config;
//mod emulator;
//mod binary;

//use emulator::i386;
//use config::Machine;
//use env_logger;
//use env_logger::log::{error, warn, info, debug};
//use std::env;

mod config;
mod emulator;
mod binary;

use config::Machine;
use emulator::i386;
use env_logger;
use std::env;

fn main() {
    let (filename, cf) = config::parse();
    if cf.dbg_mode {
        env::set_var("RUST_LOG", "debug");
    } else {
        env::set_var("RUST_LOG", "info");
    };
    env_logger::init();
    match cf.machine {
        Machine::I386 => i386::run(&cf, &filename),
        Machine::PC98 => unimplemented!()
    }
}
