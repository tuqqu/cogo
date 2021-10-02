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
        "-h" | "--help" => {
            print_help();
            process::exit(0);
        }
        _ => {}
    }

    let debug = args.contains(&"--debug".to_string()) || args.contains(&"-d".to_string());
    let args: Vec<String> = args
        .into_iter()
        .filter(|arg| !arg.starts_with('-'))
        .collect();

    let contents = fs::read_to_string(&args[1]).unwrap_or_else(|_| {
        panic!(
            "Something went wrong while reading the file \"{}\"",
            &args[1]
        )
    });

    let frame = compile(&contents, &mut ToStderrErrorHandler);

    //fixme
    if debug {
        eprintln!("\x1b[0;34m{:#?}\x1b[0m", frame);
    }

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

fn print_error(msg: &str) {
    eprintln!("\x1b[0;31m{}\x1b[0m", msg);
    eprintln!("Run the command with \"--help\" to see help information.");
}

fn print_help() {
    println!(
        r#"
    FLAGS:
        -h, --help     Print help
        -d, --debug    Dump opcodes to stdout
    "#
    )
}
