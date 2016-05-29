extern crate getopts;

use getopts::Options;
use std::env;
use std::fs::File;
use std::io::*;

static URANDOM: &'static str = "/dev/urandom";

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {}", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    let program = args[0].clone();
    let mut tla = false;
    let mut urandom;
    let mut mac: [u8; 6] = [0; 6];

    opts.optflag("h", "help", "print this help menu");
    opts.optflag("", "tla", "associate with a well-known three letter agency");
    opts.optflag("u", "upper", "use upper case characters");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("tla") {
        tla = true;
    }

    urandom = match File::open(URANDOM) {
        Ok(f) => f,
        Err(err) => panic!("{}", err),
    };

    match urandom.read_exact(&mut mac) {
        Ok(()) => (),
        Err(err) => panic!("{}", err),
    };

    if tla {
        mac[0] = 0x00;
        mac[1] = 0x20;
        mac[2] = 0x91;
    }

    if matches.opt_present("u") {
        print!("{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]);
    } else {
        print!("{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
               mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]);
    }
}
