
mod config;
mod emulator;

use emulator::i386;
use config::Machine;

fn main() {
    let (filename, cf) = config::parse();
    match cf.machine {
        Machine::I386 => i386::run(&cf, &filename),
        Machine::PC98 => unimplemented!()
    }
}
