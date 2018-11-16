extern crate hashbrown;

#[macro_use]
extern crate derivative;

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod assembler;
mod util;
mod preproc;

pub struct Args {
    bin: bool,
    verbose: bool,
    path: Option<String>,
}

macro_rules! fail {
    ($reason:expr) => {
        eprintln!("{}\n", $reason);
        eprintln!("Build failed. Exiting");
        std::process::exit(1);
    };
}

fn main() {
    let cmd_args: Vec<String> = env::args().collect();

    if cmd_args.len() < 2 {
        fail!("No file specified");
    }

    let mut args = Args {
        bin: false,
        verbose: false,
        path: None,
    };

    for arg in cmd_args {
        match arg.as_str() {
            "--bin" => args.bin = true,
            "--verbose" => args.verbose = true,
            _ => args.path = Some(arg),
        }
    }

    let path = match args.path {
        Some(s) => s,
        None => String::new(),
    };
    let path = Path::new(&path);

    let mut file = match File::open(&path) {
        Err(why) => {
            fail!(format!("Failed to open file: {}", why.description()));
        }
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => {
            fail!(format!("Failed to open file: {}", why.description()));
        }
        Ok(_) => {}
    }

    let mut interm = assembler::Interm {
        lines: Vec::new(),
        optab: Vec::new(),
        instructions: HashMap::new(),
        locctr: 0,
        linectr: 0,
        symtab: HashMap::new(),
    };

    let result = preproc::parse(&s, &mut interm);

    match result {
        Ok(_) => {}
        Err(e) => {
            fail!(e);
        }
    }

    let result = assembler::first_pass(&s, &mut interm);

    match result {
        Ok(_) => {
            println!("{:?}", interm);
        }
        Err(e) => {
            fail!(e);
        }
    }

    println!("---             ---");
    println!("--- Second pass ---");
    println!("---             ---");

    let result = assembler::second_pass(&s, &mut interm);

    match result {
        Ok(_) => println!("{:?}", interm),
        Err(e) => {
            fail!(e);
        }
    }
}
