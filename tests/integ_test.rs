//! Integration tests.
//!
//! Tests for linear and quadratic equations.
//!
use modular_equations::{LinEq, LinEqSigned, QuadEq, QuadEqSigned};

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
fn quadratic_equation() {
    let quad_eq = QuadEq::<u8> {
        a: 111,
        b: 1,
        c: 250,
        d: 1,
        modu: 255,
    };

    match quad_eq.solve() {
        Some(sols) if sols.len() == 2 => {
            assert_eq!(sols, vec![42, 177]);
        }
        _ => assert!(false),
    }
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
fn signed_quadratic_equation() {
    let quad_eq = QuadEqSigned::<i128, u128> {
        a: -11,
        b: 99,
        c: 0,
        d: -110,
        modu: 20_871_587_710_370_244_961,
    };

    match quad_eq.solve() {
        Some(sols) if sols.len() == 4 => {
            assert_eq!(
                sols,
                vec![
                    10,
                    7_399_711_637_570_012_490,
                    13_471_876_072_800_232_480,
                    20_871_587_710_370_244_960
                ]
            );
        }
        _ => assert!(false),
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
fn quadratic_equation_large_modulo() {
    let quad_eq = QuadEq::<u128> {
        a: 1,
        b: 1,
        c: 0,
        d: 1,
        modu: 416_997_623_116_370_028_124_580_469_121,
    };

    match quad_eq.solve() {
        Some(sols) if sols.len() == 2 => {
            assert_eq!(
                sols,
                vec![
                    137_307_780_239_429_241_193_741_330_788,
                    279_689_842_876_940_786_930_839_138_332
                ]
            );
        }
        _ => assert!(false),
    }
}

#[test]
fn quadratic_equation_semiprime_modulo() {
    let quad_eq = QuadEq::<u128> {
        a: 1,
        b: 0,
        c: 0,
        d: 110,
        modu: 90_124_258_835_295_998_242_413_094_252_351,
    };

    match quad_eq.solve() {
        Some(sols) if sols.len() == 4 => {
            assert_eq!(
                sols,
                vec![
                    29_129_589_224_271_400_202_982_829_638_184,
                    43_668_906_256_281_904_644_985_325_179_904,
                    46_455_352_579_014_093_597_427_769_072_447,
                    60_994_669_611_024_598_039_430_264_614_167,
                ]
            );
        }
        _ => assert!(false),
    }
}

#[test]
fn linear_equation_failure() {
    let lin_eq = LinEqSigned::<i64, u64> {
        a: i64::MIN, // No absolute value in 2's complement
        b: 1,
        c: -1,
        modu: 5,
    };

    assert_eq!(lin_eq.solve(), None);
}

#[test]
fn quadratic_equation_failure() {
    let quad_eq = QuadEqSigned::<i32, u32> {
        a: i32::MIN, // No absolute value in 2's complement
        b: 1,
        c: -1,
        d: -1,
        modu: 15,
    };

    assert_eq!(quad_eq.solve(), None);
}

#[test]
fn linear_equation_readme() {
    let lin_eq = LinEq::<u8> {
        a: 17,
        b: 0,
        c: 1,
        modu: u8::MAX,
    };

    assert_eq!(lin_eq.solve(), None);
}

#[test]
fn quadratic_equation_readme_old() {
    let quad_eq = QuadEq::<u32> {
        a: 1,
        b: 3,
        c: 2,
        d: 0,
        modu: 2u32.pow(30),
    };

    if let Some(x) = quad_eq.solve() {
        assert_eq!(x, vec![1_073_741_822, 1_073_741_823]);
    } else {
        assert!(false);
    }
}

#[test]
fn quadratic_equation_readme() {
    let quad_eq = QuadEq::<u64> {
        a: 1,
        b: 3,
        c: 4,
        d: 0,
        modu: 2u64.pow(60),
    };

    if let Some(x) = quad_eq.solve() {
        assert_eq!(x, vec![226_765_812_977_082_276, 926_155_691_629_764_697]);
    } else {
        assert!(false);
    }
}

#[test]
fn quadratic_signed_equation_readme() {
    let quad_eq = QuadEqSigned::<i128, u128> {
        a: -1,
        b: 2,
        c: -1,
        d: 0,
        modu: 2_082_064_493_491_567_088_228_629_031_592_644_077,
    };

    if let Some(x) = quad_eq.solve() {
        // Residue class [1] is the only solution
        assert_eq!(x, vec![1]);
    } else {
        assert!(false);
    }
}
