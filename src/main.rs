use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod assembler;
mod preproc;

pub struct Args {
    bin: bool,
    verbose: bool,
    path: Option<String>,
}

fn main() {
    let cmd_args: Vec<String> = env::args().collect();

    if cmd_args.len() < 2 {
        eprintln!("Please specify a file.");
        std::process::exit(1);
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
            eprintln!("Failed to open file: {}", why.description());
            std::process::exit(1);
        }
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => {
            eprintln!("Failed to open file: {}", why.description());
            std::process::exit(1);
        }
        Ok(_) => {}
    }

    let result = preproc::parse(&s);
    let mut interm: assembler::Interm;

    match result {
        Ok(i) => interm = i,
        Err(e) => {
            eprintln!("{}\n", e);
            eprintln!("Build failed. Exiting");
            std::process::exit(1);
        }
    }

    let result = assembler::first_pass(&s, &mut interm);

    match result {
        Ok(_) => {
            println!("{:?}", interm);
        }
        Err(e) => {
            eprintln!("{}\n", e);
            eprintln!("Build failed. Exiting");
            std::process::exit(1);
        }
    }
}
