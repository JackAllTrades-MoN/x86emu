extern crate clap;

use clap::{App, Arg};

pub enum Machine {
    I386,
    PC98,
}

pub enum InputFormat {
    Raw,
    PC98FDI,
}

pub struct Config {
    pub filename: String,
    pub machine: Machine,
    pub dbg_mode: bool,
    pub input_format: InputFormat,
}

impl Config {
    fn init(fstr: &str) -> Config {
        Config { filename: String::from(fstr),
                 machine: Machine::I386,
                 dbg_mode: false,
                 input_format: InputFormat::Raw}
    }
    fn update_machine(self, m:&str) -> Config {
        match m {
            "i386" =>
                Config { machine: Machine::I386, .. self},
            "pc98" =>
                Config { machine: Machine::PC98, .. self},
            _ => panic!("Impossible State")
        }
    }
    fn update_input_format(self, i:&str) -> Config {
        match i {
            "raw" => Config { input_format: InputFormat::Raw, .. self },
            "fdi" => Config { input_format: InputFormat::PC98FDI, .. self },
            _ => panic!("Impossible State")
        }
    }
    fn enable_dbg_mode(self) -> Config {
        Config { dbg_mode: true, .. self }
    }
}

pub fn parse() -> Config {
    let app = App::new("x86emulator")
        .version("0.1.0")
        .author("Yuki Satake <kashiwagi513@gmail.com>")
        .about("Rust implementation of x86emulator")
        .arg(Arg::with_name("filename")
             .help("program file (16bit assembly)")
             .required(true)
        )
        .arg(Arg::with_name("machine")
             .help("machine type [i386/pc98] (default:i386)")
             .short("m")
             .long("machine")
             .takes_value(true)
             .possible_values(&["i386", "pc98"])
             .default_value("i386")
        )
        .arg(Arg::with_name("inputformat")
             .help("input format [raw/fdi] (default:raw)")
             .short("i")
             .long("input-format")
             .takes_value(true)
             .possible_values(&["raw", "fdi"])
             .default_value("raw")
        )
        .arg(Arg::with_name("dbgmode")
             .help("enable debug mode")
             .short("dbg")
             .long("dbg-mode")
        );
    let matches = app.get_matches();
    let cfg =
        Config::init(matches.value_of("filename").unwrap())
        .update_machine(matches.value_of("machine").unwrap())
        .update_input_format(matches.value_of("inputformat").unwrap());
    let dbg_mode = matches.is_present("dbgmode");
    let cfg = if dbg_mode { cfg.enable_dbg_mode() } else { cfg };
    cfg
}
