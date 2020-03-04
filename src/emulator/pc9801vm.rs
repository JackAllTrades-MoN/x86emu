/// PC9801VM emulator
/// TODO: write a documentation

mod display;
pub mod cpu;
mod ram;
mod crt;
mod sound;
mod sysport;

use log::{debug, info};
//use crate::binary;
use crate::config::Config;
use std::sync::{Arc, Mutex};

use cpu::Cpu;
use cpu::instruction::Code;
use ram::Ram;
use ram::GVRam;
use crt::Crt;
use sysport::SysPort;

pub struct PC9801VM {
    pub cpu: Cpu,
    pub ram: Ram,
    pub graphics_ram: GVRam,
    pub system_port: SysPort,
    pub crt: Crt,
}

impl PC9801VM {
    pub fn init() -> PC9801VM {
        PC9801VM {cpu: Cpu::init(),
                  ram: Ram::init(),
                  graphics_ram: GVRam::init(),
                  system_port: SysPort::init(),
                  crt: Crt::init(),
        }}
    pub fn fetch(&mut self) -> u8 {
        let addr = self.cpu.get_ip();
        let byte = self.ram.fetch_u8(addr as usize);
        self.cpu.inc_ip(1);
        byte
    }
    pub fn decode(&self, bytes: &Vec<u8>) -> Option<Code> { None }
    pub fn execute(&mut self, code: &Code) { () }
    pub fn to_string(&self) -> String { String::from("print machine state (stub)") }
}

pub fn run (cfg: &Config) {
    let _filename = &cfg.filename;
    info!("initialize machine (PC9801VM)");
    let machine = Arc::new(Mutex::new(PC9801VM::init()));
    let machine_ = machine.clone();
    std::thread::spawn(move || {
        let mut bytes = Vec::new();
        'mainlp: loop {
            let mut machine = machine_.lock().unwrap();
            debug!("{}", &machine.to_string());
            bytes.push(machine.fetch());
            match machine.decode(&bytes) {
                None => (),
                Some(code) => {
                    bytes.clear();
                    machine.execute(&code);
                },
            }
            break 'mainlp;
        }
        debug!("{}", "finish_main_loop");
    });
    let machine_ = machine.clone();
    std::thread::spawn(move || { sound::boot_speaker(machine_) });
    let machine_ = machine.clone();
    display::boot_display(machine_);
}

#[test]
fn test_run() {
    // write an unit test
    assert!(true);
}
