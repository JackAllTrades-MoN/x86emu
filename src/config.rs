extern crate clap;

use clap::{App, Arg};

pub enum Machine {
    I386,
    PC98,
}

pub struct Config {
    pub machine: Machine,
    pub dbg_mode: bool,
}

impl Config {
    fn init() -> Config {
        Config { machine: Machine::I386, dbg_mode: false}
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
    fn enable_dbg_mode(self) -> Config {
        Config { dbg_mode: true, .. self }
    }
}

pub fn parse() -> (String, Config) {
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
        .arg(Arg::with_name("dbgmode")
             .help("enable debug mode")
             .short("dbg")
             .long("dbg-mode")
        );
    let matches = app.get_matches();
    let filename = matches.value_of("filename").unwrap();
    let m = match matches.value_of("machine") {
        None => panic!("impossible state"),
        Some(m) => m
    };
    let dbg_mode = matches.is_present("dbgmode");
    let cfg = Config::init();
    let cfg =
        if dbg_mode {
            cfg
                .enable_dbg_mode()
                .update_machine(m)
        } else {
            cfg.update_machine(m)
        };
    (filename.to_string(), cfg)
}
