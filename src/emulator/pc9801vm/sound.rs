use std::sync::{Arc, Mutex};
use log::{debug};

use super::super::pc9801vm::Machine;


pub fn boot_speaker(machine: Arc<Mutex<Machine>>) -> () {
    loop {
        let machine = machine.lock().unwrap();
        let system_port = &machine.system_port;
        debug!("buzzer_flug: {}", system_port.c.get() & 0b00001000);
        if system_port.c.get() & 0b00001000 != 0 {
            debug!("{}", "the buzzer is currently turned on");
        } else {
            debug!("{}", "the buzzer is currently turned off");
        }
    }
}
