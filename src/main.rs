mod scanner;
mod parser;
mod token;
mod token_type;
mod expr;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::process;

static mut HAD_ERROR: bool = false;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() > 1 {
        println!("Usage: rlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1])?;
    } else if args.len() == 1 {
        run_prompt()?;
    }
    Ok(())
}

fn run_prompt() -> io::Result<()> {
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut line = String::new();
        stdin.lock().read_line(&mut line)?;
        if line.is_empty() {
            break;
        } else {
            run(&line);
            unsafe { HAD_ERROR = false };
        }
    }

    Ok(())
}

fn run_file(args: &str) -> io::Result<()> {
    let mut file = File::open(args)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    run(&contents);
    if unsafe { HAD_ERROR } == true {
        process::exit(65)
    };
    Ok(())
}

fn run(source: &str) {
    let tokens = scanner::scan(source);

    for token in tokens.iter() {
        dbg!(token);
    }
}

fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, place: &str, message: &str) {
    eprint!("[line {}] Error{}: {}", line, place, message);
    unsafe { HAD_ERROR = true };
}
