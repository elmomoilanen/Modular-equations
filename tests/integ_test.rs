//! Integration tests.
//!
//! Test linear and quadratic equations.
//!
use modular_equations::{LinEq, LinEqSigned};

#[test]
fn linear_equation() {
    let lin_eq = LinEq::<u8> {
        a: 81,
        b: 9,
        c: 77,
        modu: 79,
    };

    let sol = lin_eq.solve();

    assert!(sol.is_some());
    // correct solution is 34
    assert_eq!(sol.unwrap()[0], 34);
}

#[test]
fn signed_linear_equation() {
    let lin_eq = LinEqSigned::<i16, u16> {
        a: -1,
        b: -1000,
        c: 17,
        modu: 7,
    };

    match lin_eq.solve() {
        None => assert!(false),
        Some(sol) => {
            assert_eq!(sol.len(), 1);
            // correct solution is 5
            assert_eq!(sol[0], 5);
        }
    }
}

#[test]
fn linear_equation_multiple_solutions() {
    let lin_eq = LinEq::<u32> {
        a: 15,
        b: 3,
        c: 33,
        modu: 55,
    };

    match lin_eq.solve() {
        None => assert!(false),
        Some(sols) => {
            // solution should be a vector of length five
            assert_eq!(sols.len(), 5);
            assert_eq!(sols, vec![2, 13, 24, 35, 46]);
        }
    }
}

#[test]
fn linear_equation_one_hundred_solutions() {
    let lin_eq = LinEqSigned::<i32, u32> {
        a: 100,
        b: -1,
        c: 199,
        modu: 500,
    };

    match lin_eq.solve() {
        None => assert!(false),
        Some(sols) => {
            // there should be 100 solutions
            assert_eq!(sols.len(), 100);
            // test first and last, solutions should be ordered
            assert_eq!(sols[0], 2);
            assert_eq!(sols[99], 497);
        }
    }
}

#[test]
fn linear_equation_no_solution() {
    let mut lin_eq = LinEq::<u8> {
        a: 5,
        b: 0,
        c: 1,
        modu: 5,
    };

    // a % modu == 0
    assert_eq!(lin_eq.solve(), None);

    lin_eq.modu = 10;
    // gcd(a, modu) does not divide c
    assert_eq!(lin_eq.solve(), None);
}

#[test]
fn linear_equation_large_modulo() {
    let lin_eq = LinEq::<u128> {
        a: 7,
        b: u128::MAX - 4,
        c: 0,
        modu: u128::MAX,
    };

    match lin_eq.solve() {
        None => assert!(false),
        Some(sols) => {
            assert_eq!(sols.len(), 1);
            assert_eq!(sols[0], 48_611_766_702_991_209_066_196_372_490_252_601_637);
        }
    }
}

#[test]
#[should_panic(expected = "arg `a` cannot be casted to unsigned.")]
fn linear_equation_failure() {
    let lin_eq = LinEqSigned::<i64, u64> {
        a: i64::MIN, // should cause panic, no abs value in 2's complement
        b: 1,
        c: -1,
        modu: 5,
    };

    lin_eq.solve();
}
