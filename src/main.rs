//! TODO: CLI documentation
//!
//!
use std::{env, process};

extern crate modular_equations;
use modular_equations::{LinEqSigned, QuadEqSigned, UInt};

mod parser;
use parser::EquaKind;

fn main() {
    let args: Vec<String> = env::args().collect();

    let equa = parser::parse_args(&args[1..]).unwrap_or_else(|err| {
        if err == "help" {
            process::exit(0);
        }

        eprintln!("Error with command line args: {}", err);
        process::exit(1);
    });

    match &equa {
        EquaKind::LinearI64(eq_lin) => print_sol(eq_lin.solve(), eq_lin.modu),
        // TODO: complete quadratic cases
        EquaKind::QuadI64(_eq_quad) => (),
        EquaKind::LinearI128(eq_lin_large) => print_sol(eq_lin_large.solve(), eq_lin_large.modu),
        EquaKind::QuadI128(_eq_quad_large) => (),
    }
}

fn print_sol<T: UInt>(solution: Option<Vec<T>>, modu: T) {
    match solution {
        None => println!("There is no solution in Z/{}Z", modu),
        Some(sols) => {
            println!("Solutions x in Z/{}Z", modu);

            for (j, x) in sols.iter().enumerate() {
                println!("x_{}: {}", j + 1, *x);
            }
        }
    }
}
