//!
//!
//!
use std::{env, process};

extern crate modular_arithmetic;
use modular_arithmetic::{LinEqSigned, QuadEqSigned};

mod parser;
use parser::EquaKind;

fn main() {
    let args: Vec<String> = env::args().collect();

    let equation = parser::parse_args(&args[1..]).unwrap_or_else(|err| {
        if err == "help" {
            process::exit(0);
        }

        eprintln!("Error with command line args: {}", err);
        process::exit(1);
    });
}
