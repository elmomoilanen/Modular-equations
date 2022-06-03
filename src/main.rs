//! CLI for the modular equations program.
//!
//! In its most general form, modular equation means here a equation of
//! ax^2 + bx + c = d (mod n).
//!
//! Coefficients from a to d are restricted to be a signed type and the
//! modulo M corresponding unsigned type. This restriction applies only
//! for the CLI, not for the library.
//!
//! Following example shows how to solve a linear equation, assuming
//! that the binary has been built first or just installed
//!
//! ```bash
//! modular_equations b c d n
//! ```
//!
//! Solving quadratic equation is almost the same, one just need to add
//! the a coefficient as the following example shows
//!
//! ```bash
//! modular_equations a b c d n
//! ```
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
        EquaKind::QuadI64(eq_quad) => print_sol(eq_quad.solve(), eq_quad.modu),
        EquaKind::LinearI128(eq_lin_large) => print_sol(eq_lin_large.solve(), eq_lin_large.modu),
        EquaKind::QuadI128(eq_quad_large) => print_sol(eq_quad_large.solve(), eq_quad_large.modu),
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
