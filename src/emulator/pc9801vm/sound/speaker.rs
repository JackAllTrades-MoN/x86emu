use std::sync::{Arc, Mutex};
use super::super::pc9801vm::Machine;;

fn boot_speaker(machine: Arc<Mutex<Machine>>) -> {
    loop {
        let machine = machine.lock().unwarp();
        let system_port = &machine.system_port;
        if system_port.c & 0b00001000 == 1 {
            debug!("{}", "the buzzer is currently turned on");
        } else {
            debug!("{}", "the buzzer is currently turned off");
        }
    }
}
