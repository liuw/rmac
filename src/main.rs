extern crate getopts;

use getopts::Options;
use std::env;
use std::fs::File;

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
    let urandom;

    opts.optflag("h", "help", "print this help menu");
    opts.optflag("", "tla", "associate with a well-known three letter agency");

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
        Ok(val) => val,
        Err(err) => panic!("{}", err),
    };
}
