use modular_arithmetic::{LinEq, LinEqSigned};

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

    let sols = lin_eq.solve();

    assert!(sols.is_some());
    // solution should be a vector of length five
    assert_eq!(sols.unwrap().len(), 5);
}

#[test]
fn linear_equation_no_solution() {
    let lin_eq = LinEq::<u8> {
        a: 6,
        b: 0,
        c: 1,
        modu: 2,
    };

    assert_eq!(lin_eq.solve(), None);
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
