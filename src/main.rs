use std::{env, fs, process};

use cogo::compiler::{compile, ToStderrErrorHandler};
use cogo::vm::Vm;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        print_error("Arguments not found.");
        process::exit(1);
    }

    match args[1].as_str() {
        // "-v" | "--version" => {
        //     print_version();
        //     process::exit(0);
        // }
        // "-h" | "--help" => {
        //     print_help();
        //     process::exit(0);
        // }
        _ => {
            let contents = fs::read_to_string(&args[1]).unwrap_or_else(|_| {
                panic!(
                    "Something went wrong while reading the file \"{}\"",
                    &args[1]
                )
            });

            let frame = compile(contents, &mut ToStderrErrorHandler);

            let mut vm = Vm::new(None, frame);
            let res = vm.run();
            match res {
                Ok(()) => {
                    process::exit(0);
                }
                Err(e) => {
                    print_error(&e.to_string());
                    process::exit(1);
                }
            }
        }
    }
}

fn print_error(msg: &str) {
    eprintln!("\x1b[0;31m{}\x1b[0m", msg);
    eprintln!("Run the command with \"--help\" to see help information.");
}
