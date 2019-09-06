//! # PC9801VM emulator
//! TODO: write a documentation

mod display;
mod cpu;
mod ram;
mod crt;

use log::{debug, info};
//use crate::binary;
use crate::config::Config;
use std::sync::{Arc, Mutex};

use cpu::Cpu;
use ram::Ram;
use ram::GVRam;
use crt::Crt;

pub struct SysPort {
    pub a: u8, // read only port
    pub b: u8, // read only port
    pub c: u8, // writable port
}
impl SysPort { pub fn init() -> SysPort { SysPort{a:0, b:0, c:0,} } }
pub struct Machine {
    pub cpu: Cpu,
    pub ram: Ram,
    pub graphics_ram: GVRam,
    pub system_port: SysPort,
    pub crt: Crt,
}

impl Machine {
    pub fn init() -> Machine {
        Machine {cpu: Cpu::init(),
                 ram: Ram::init(),
                 graphics_ram: GVRam::init(),
                 system_port: SysPort::init(),
                 crt: Crt::init(),
        }}
    pub fn to_string(&self) -> String { String::from("print machine state (stub)") }
}

pub fn run (cfg: &Config) {
    let _filename = &cfg.filename;
    info!("initialize machine (PC9801VM)");
    let machine = Arc::new(Mutex::new(Machine::init()));
    let machine_ = machine.clone();
    std::thread::spawn(move || {
        'mainlp: loop {
            let machine = machine_.lock().unwrap();
            debug!("{}", &machine.to_string());
            std::thread::sleep(std::time::Duration::from_millis(10));
            break 'mainlp;
        }
        debug!("{}", "finish_main_loop");
    });
    let machine_ = machine.clone();
    display::boot_display(machine_);
}

#[test]
fn test_run() {
    // write an unit test
    assert!(true);
}
