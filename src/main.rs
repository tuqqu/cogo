use std::io::Write;
use std::{env, fs, io, process};

use cogo::compiler::compile;
use cogo::vm::{Vm, VmResult};

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
        "-r" | "--repl" => {
            repl();
            process::exit(0);
        }
        _ => {
            let contents = fs::read_to_string(&args[1]).unwrap_or_else(|_| {
                panic!(
                    "Something went wrong while reading the file \"{}\"",
                    &args[1]
                )
            });

            let res = interpret(contents);
            //FIXME
            if res.is_err() {
                eprintln!("\x1b[0;35m{:#?}\x1b[0m", res);
                print_error("Error.");
                process::exit(1);
            }
        }
    }
}

/// Runs REPL mode from stdin.
fn repl() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line: String = String::new();
        io::stdin().read_line(&mut line).unwrap();

        if line == "\n" {
            break;
        }

        let res = interpret(line);
        //FIXME
        if res.is_err() {
            eprintln!("\x1b[0;31m{:#?}\x1b[0m", res);
            print_error("Error.");
            process::exit(1);
        }
    }
}

fn interpret(src: String) -> VmResult {
    let frame = compile(src);

    let mut vm = Vm::new(None, frame);
    vm.run()?;

    Ok(())
}

fn print_error(msg: &str) {
    eprintln!("\x1b[0;31m{}\x1b[0m", msg);
    eprintln!("Run the command with \"--help\" to see help information.");
}
