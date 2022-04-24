//! Utility for command line argument parsing.
//!
//! Library crate doesn't need this but binary crate does.
//!
use std::str::FromStr;

use num::PrimInt;

use crate::{LinEqSigned, QuadEqSigned};

const I64_VALID_MIN: i128 = (i64::MIN + 1) as i128;
const I64_VALID_MAX: i128 = i64::MAX as i128;
const U64_VALID_MAX: u128 = u64::MAX as u128;

pub enum EquaKind {
    LinearI64(LinEqSigned<i64, u64>),
    LinearI128(LinEqSigned<i128, u128>),
    QuadI64(QuadEqSigned<i64, u64>),
    QuadI128(QuadEqSigned<i128, u128>),
}

#[derive(PartialEq)]
enum EqType {
    Linear,
    Quad,
}

pub fn parse_args(args: &[String]) -> Result<EquaKind, String> {
    let args_len = args.len();

    match args_len {
        0 => Err("no arguments provided.".to_string()),
        1 if &args[0] == "--help" || &args[0] == "-h" => {
            show_help();
            Err("help".to_string())
        }
        4 => {
            if let Some(lin_equa) = parse_to_equation(args) {
                Ok(lin_equa)
            } else {
                Err("invalid arg values for linear equation.".to_string())
            }
        }
        5 => {
            if let Some(quad_eq) = parse_to_equation(args) {
                Ok(quad_eq)
            } else {
                Err("invalid arg values for quadratic equation.".to_string())
            }
        }
        _ => Err(
            "arg count mismatch: pass either four for linear or five for quadratic equation."
                .to_string(),
        ),
    }
}

fn parse_to_equation(args: &[String]) -> Option<EquaKind> {
    let args_len = args.len();

    let eq_type = match args_len {
        4 => EqType::Linear,
        5 => EqType::Quad,
        _ => return None,
    };
    let mut coefs: [Option<i128>; 4] = [None; 4];
    let mut modulo: Option<u128> = None;

    for (idx, arg) in args.iter().enumerate() {
        if idx == args_len - 1 {
            modulo = parse_to_number::<u128>(arg);
        } else {
            coefs[idx] = parse_to_number::<i128>(arg);
        }
    }

    let coefs_len = coefs.len();

    match eq_type {
        EqType::Linear => parse_proper_type(&coefs[..coefs_len - 1], modulo, eq_type),
        EqType::Quad => parse_proper_type(&coefs, modulo, eq_type),
    }
}

fn parse_to_number<T: PrimInt + FromStr>(arg: &str) -> Option<T> {
    match (*arg).parse::<T>() {
        Ok(num) => Some(num),
        Err(_) => {
            let mut arg = String::from(arg);
            arg.retain(|c| c != '_');

            if let Ok(num) = arg.parse::<T>() {
                Some(num)
            } else {
                None
            }
        }
    }
}

fn parse_proper_type(
    coefs: &[Option<i128>],
    modulo: Option<u128>,
    eq_type: EqType,
) -> Option<EquaKind> {
    if coefs.iter().any(|&coef| coef.is_none()) || modulo.is_none() {
        return None;
    }

    let modu = modulo.unwrap();

    if modu <= 1 {
        // modulo must be at least two
        return None;
    }

    let coefs: Vec<i128> = coefs.iter().map(|&coef| coef.unwrap()).collect();

    if coefs[0] == 0 {
        // coef for x^2 or x term must be non-zero
        return None;
    }

    Some(get_proper_eq_type(&coefs, modu, eq_type))
}

fn get_proper_eq_type(coefs: &[i128], modu: u128, eq_type: EqType) -> EquaKind {
    let smaller_modu = modu <= U64_VALID_MAX;
    let smaller_coefs = coefs
        .iter()
        .all(|&coef| coef >= I64_VALID_MIN && coef <= I64_VALID_MAX);

    match (eq_type, smaller_coefs && smaller_modu) {
        (EqType::Linear, true) => EquaKind::LinearI64(LinEqSigned::<i64, u64> {
            a: coefs[0].try_into().unwrap(),
            b: coefs[1].try_into().unwrap(),
            c: coefs[2].try_into().unwrap(),
            modu: modu.try_into().unwrap(),
        }),
        (EqType::Linear, false) => EquaKind::LinearI128(LinEqSigned::<i128, u128> {
            a: coefs[0],
            b: coefs[1],
            c: coefs[2],
            modu,
        }),
        (EqType::Quad, true) => EquaKind::QuadI64(QuadEqSigned::<i64, u64> {
            a: coefs[0].try_into().unwrap(),
            b: coefs[1].try_into().unwrap(),
            c: coefs[2].try_into().unwrap(),
            d: coefs[3].try_into().unwrap(),
            modu: modu.try_into().unwrap(),
        }),
        (EqType::Quad, false) => EquaKind::QuadI128(QuadEqSigned::<i128, u128> {
            a: coefs[0],
            b: coefs[1],
            c: coefs[2],
            d: coefs[3],
            modu,
        }),
    }
}

fn show_help() {
    println!("Solve modular equations\n\nUSAGE:\n  ./modular_arithmetic ...\n");
}
