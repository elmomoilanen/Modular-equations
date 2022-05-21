use crate::arith::{Arith, SignCast};

#[test]
fn add_small_type() {
    let modu = 5;

    // [x, y, res]: x + y = res (mod modu)
    let test_cases: [[u8; 3]; 8] = [
        [0, 0, 0],
        [2, 1, 3],
        [1, 2, 3],
        [2, 3, 0],
        [3, 2, 0],
        [4, 4, 3],
        [u8::MAX, 2, 2],
        [2, u8::MAX - 1, 1],
    ];

    for test in test_cases.iter() {
        let (x, y) = (test[0], test[1]);

        assert_eq!(u8::add_mod(x, y, modu), test[2], "x: {}, y: {}", x, y);
    }
}

#[test]
fn add_small_type_max_mod() {
    let modu = u8::MAX;

    // [x, y, res]: x + y = res (mod modu)
    let test_cases: [[u8; 3]; 8] = [
        [0, 0, 0],
        [modu, modu, 0],
        [modu - 1, modu, modu - 1],
        [modu, modu - 1, modu - 1],
        [modu - 1, modu - 1, modu - 2],
        [2, modu - 1, 1],
        [modu - 1, 2, 1],
        [0, modu - 2, modu - 2],
    ];

    for test in test_cases.iter() {
        let (x, y) = (test[0], test[1]);

        assert_eq!(u8::add_mod(x, y, modu), test[2], "x: {}, y: {}", x, y);
    }
}

#[test]
fn add_large_type() {
    let modu = u16::MAX as u128;
    let u32max = u32::MAX as u128;

    // [x, y, res]: x + y = res (mod modu)
    let test_cases: [[u128; 3]; 8] = [
        [0, 0, 0],
        [5, 5, 10],
        [modu, modu + 1, 1],
        [modu + 1, modu, 1],
        [modu - 1, 2, 1],
        [2, modu - 1, 1],
        [u32max - 1, u32max - 1, modu - 2],
        [u32max + 1, u32max - 1, 0],
    ];

    for test in test_cases.iter() {
        let (x, y) = (test[0], test[1]);

        assert_eq!(u128::add_mod(x, y, modu), test[2], "x: {}, y: {}", x, y);
    }
}

#[test]
fn add_large_type_max_mod() {
    let modu = u128::MAX;

    // [x, y, res]: x + y = res (mod modu)
    let test_cases: [[u128; 3]; 8] = [
        [0, 0, 0],
        [modu, modu, 0],
        [modu - 1, modu, modu - 1],
        [modu, modu - 1, modu - 1],
        [modu - 1, modu - 1, modu - 2],
        [2, modu - 1, 1],
        [modu - 1, 2, 1],
        [0, modu - 2, modu - 2],
    ];

    for test in test_cases.iter() {
        let (x, y) = (test[0], test[1]);

        assert_eq!(u128::add_mod(x, y, modu), test[2], "x: {}, y: {}", x, y);
    }
}

#[test]
fn sub_small_type() {
    let modu = 3;

    // [x, y, res]: x - y = res (mod modu)
    let test_cases: [[u8; 3]; 10] = [
        [0, 0, 0],
        [2, 2, 0],
        [2, 1, 1],
        [1, 2, 2],
        [1, 0, 1],
        [0, 1, 2],
        [1, 7, 0],
        [2, u8::MAX, 2],
        [u8::MAX, 2, 1],
        [u8::MAX, u8::MAX - 1, 1],
    ];

    for test in test_cases.iter() {
        let (x, y) = (test[0], test[1]);

        assert_eq!(u8::sub_mod(x, y, modu), test[2], "x: {}, y: {}", x, y);
    }
}

#[test]
fn sub_large_type() {
    let modu = u128::MAX;
    let u16max = u16::MAX as u128;

    // [x, y, res]: x - y = res (mod modu)
    let test_cases: [[u128; 3]; 8] = [
        [0, 0, 0],
        [modu, modu, 0],
        [modu, modu - 1, 1],
        [modu - 1, modu, modu - 1],
        [modu - 2, modu - 1, modu - 1],
        [1, modu - 1, 2],
        [modu - 1, 2, modu - 3],
        [u16max, u16max + 1, modu - 1],
    ];

    for test in test_cases.iter() {
        let (x, y) = (test[0], test[1]);

        assert_eq!(u128::sub_mod(x, y, modu), test[2], "x: {}, y: {}", x, y);
    }
}

#[test]
fn mult_small_type() {
    let modu = 7;

    // [x, y, res]: x * y = res (mod modu)
    let test_cases: [[u8; 3]; 10] = [
        [0, 0, 0],
        [0, 5, 0],
        [1, 1, 1],
        [2, 3, 6],
        [3, 2, 6],
        [modu - 1, modu - 1, 1],
        [modu - 1, 1, modu - 1],
        [1, modu - 1, modu - 1],
        [u8::MAX, 1, 3],
        [u8::MAX, u8::MAX, 2],
    ];

    for test in test_cases.iter() {
        let (x, y) = (test[0], test[1]);

        assert_eq!(u8::mult_mod(x, y, modu), test[2], "x: {}, y: {}", x, y);
    }
}

#[test]
fn mult_large_type() {
    let modu = u32::MAX as u128;
    let u16max = u16::MAX as u128;

    // [x, y, res]: x * y = res (mod modu)
    let test_cases: [[u128; 3]; 10] = [
        [modu - 1, 0, 0],
        [1, modu - 1, modu - 1],
        [modu - 1, 1, modu - 1],
        [2, modu - 1, modu - 2],
        [modu - 1, 2, modu - 2],
        [modu - 1, modu - 1, 1],
        [u16max + 1, u16max + 1, 1],
        [u16max + 2, u16max + 1, u16max + 2],
        [1, u128::MAX, 0],
        [u128::MAX - 1, u128::MAX - 1, 1],
    ];

    for test in test_cases.iter() {
        let (x, y) = (test[0], test[1]);

        assert_eq!(u128::mult_mod(x, y, modu), test[2], "x: {}, y: {}", x, y);
    }
}

#[test]
fn mult_large_max_mod() {
    let modu = u128::MAX;

    // [x, y, res]: x * y = res (mod modu)
    let test_cases: [[u128; 3]; 5] = [
        [modu, 1, 0],
        [modu - 1, modu - 1, 1],
        [modu - 1, 1, modu - 1],
        [modu - 2, modu - 1, 2],
        [modu - 3, modu - 3, 9],
    ];

    for test in test_cases.iter() {
        let (x, y) = (test[0], test[1]);

        assert_eq!(u128::mult_mod(x, y, modu), test[2], "x: {}, y: {}", x, y);
    }
}

#[test]
fn exp_small_type() {
    let modu = 5;

    // [x, y, res]: x^y = res (mod modu)
    let test_cases: [[u8; 3]; 10] = [
        [0, 0, 0],
        [0, 1, 0],
        [1, 0, 1],
        [5, 1, 0],
        [2, 4, 1],
        [4, 2, 1],
        [3, 4, 1],
        [4, 3, 4],
        [4, 40, 1],
        [8, 50, 4],
    ];

    for test in test_cases.iter() {
        let (x, y) = (test[0], test[1]);

        assert_eq!(u8::exp_mod(x, y, modu), test[2], "x: {}, y: {}", x, y);
    }
}

#[test]
fn exp_large_type() {
    let modu = u64::MAX as u128;

    // [x, y, res]: x^y = res (mod modu)
    let test_cases: [[u128; 3]; 8] = [
        [2, 1_000_000_000, 1],
        [modu - 1, 1_000_000_000, 1],
        [modu - 1, 1_000_000_001, modu - 1],
        [2, 9_999_999_999_999, i64::MAX as u128 + 1],
        [modu - 1, modu - 1, 1],
        [modu - 1, modu - 2, modu - 1],
        [modu - 1, modu + 1, 1],
        [modu - 1, modu + 2, modu - 1],
    ];

    for test in test_cases.iter() {
        let (x, y) = (test[0], test[1]);

        assert_eq!(u128::exp_mod(x, y, modu), test[2], "x: {}, y: {}", x, y);
    }
}

#[test]
fn gcd_small_type() {
    // [x, y, res]: gcd(x, y) = res
    let test_cases: [[u8; 3]; 5] = [[1, 0, 1], [0, 1, 1], [2, 3, 1], [3, 2, 1], [34, 85, 17]];

    for test in test_cases.iter() {
        let (x, y) = (test[0], test[1]);
        assert_eq!(u8::gcd_mod(x, y), test[2], "x: {}, y: {}", x, y);
    }
}

#[test]
fn gcd_large_type() {
    let i64max = i64::MAX as u128;
    let u64max = u64::MAX as u128;

    // [x, y, res]: gcd(x, y) = res
    let test_cases: [[u128; 3]; 10] = [
        [224, 412, 4],
        [526, 17_210, 2],
        [10_500, 975, 75],
        [100_000, 15_888, 16],
        [900, 999_888_000, 300],
        [1_001_116_321, 10_011_18_301, 1],
        [i64max, 3, 1],
        [i64max, 9_933_434_335_423, 73],
        [u64max as u128, 1_640_877_430_502_539, 17],
        [u64max as u128, 572_590_724_124, 3],
    ];

    for test in test_cases.iter() {
        let (x, y) = (test[0], test[1]);

        assert_eq!(u128::gcd_mod(x, y), test[2], "x: {}, y: {}", x, y);
    }
}

#[test]
fn multip_inv_small_type() {
    let u8max = u8::MAX;

    // [x, modu, x^(-1)]: x * x^(-1) = 1 (mod modu)
    // if x^(-1) is zero, proper inverse doesn't exist
    let test_cases: [[u8; 3]; 10] = [
        [0, 11, 0],
        [1, 11, 1],
        [5, 11, 9],
        [8, 11, 7],
        [10, 11, 10],
        [1, u8max, 1],
        [2, u8max, 128],
        [u8max - 1, u8max, u8max - 1],
        [100, u8max, 0],
        [104, u8max, 179],
    ];

    for test in test_cases.iter() {
        let (x, modu) = (test[0], test[1]);

        assert_eq!(u8::multip_inv(x, modu), test[2], "x: {}, mod: {}", x, modu);
    }
}

#[test]
fn multip_inv_large_type() {
    let u128max = u128::MAX;
    let i64max = i64::MAX as u128;

    // [x, modu, x^(-1)]: x * x^(-1) = 1 (mod modu)
    // if x^(-1) is zero, proper inverse doesn't exist
    let test_cases: [[u128; 3]; 10] = [
        [3, 5000, 1667],
        [1667, 5000, 3],
        [999, 5000, 3999],
        [55, 5000, 0],
        [999, i64max, 3_619_181_019_466_538_655],
        [i64max - 3, i64max, 3_074_457_345_618_258_602],
        [0, u128max, 0],
        [u128max, u128max, 0],
        [u128max - 1, u128max, u128max - 1],
        [
            2,
            u128max,
            170_141_183_460_469_231_731_687_303_715_884_105_728,
        ],
    ];

    for test in test_cases.iter() {
        let (x, modu) = (test[0], test[1]);

        assert_eq!(
            u128::multip_inv(x, modu),
            test[2],
            "x: {}, mod: {}",
            x,
            modu
        );
    }
}

#[test]
fn jacobi_symbol_small_operands() {
    let test_cases: [(u32, u32, i8); 15] = [
        (1, 1, 1),
        (15, 1, 1),
        (2, 3, -1),
        (29, 9, 1),
        (4, 11, 1),
        (17, 11, -1),
        (19, 29, -1),
        (10, 33, -1),
        (11, 33, 0),
        (12, 33, 0),
        (14, 33, -1),
        (15, 33, 0),
        (15, 37, -1),
        (29, 59, 1),
        (30, 59, -1),
    ];

    for case in test_cases.iter() {
        let (x, n, res) = case;

        assert_eq!(u32::jacobi_symbol(*x, *n), *res, "x: {}, n: {}", *x, *n);
    }
}

#[test]
fn jacobi_symbol_large_operands() {
    let max_i128 = i128::MAX as u128;

    let test_cases: [(u128, u128, i8); 4] = [
        (1_241_942_351, 2_147_483_647, 1),
        (99, max_i128, 1),
        (max_i128 - 1, max_i128, -1),
        (max_i128, max_i128, 0),
    ];

    for case in test_cases.iter() {
        let (x, n, res) = case;

        assert_eq!(u128::jacobi_symbol(*x, *n), *res, "x: {}, n: {}", *x, *n);
    }
}

#[test]
fn trunc_square_mid_type() {
    let test_cases: [[u64; 2]; 4] = [
        [0, 0],
        [2, 4],
        [u32::MAX as u64, 18_446_744_065_119_617_025],
        [u32::MAX as u64 + 1, 0],
    ];

    for case in test_cases.iter() {
        assert_eq!(u64::trunc_square(case[0]), case[1]);
    }
}

#[test]
fn trunc_square_large_type() {
    let test_cases: [[u128; 2]; 4] = [
        [0, 0],
        [3, 9],
        [
            u64::MAX as u128,
            340_282_366_920_938_463_426_481_119_284_349_108_225,
        ],
        [u64::MAX as u128 + 1, 0],
    ];

    for case in test_cases.iter() {
        assert_eq!(u128::trunc_square(case[0]), case[1]);
    }
}

#[test]
fn sign_cast_success_small_type() {
    let modu = 5;
    let i8_min_valid = i8::MIN + 1;

    let test_cases: [(i8, u8); 10] = [
        (0, 0),
        (-1, 4),
        (-2, 3),
        (-5, 0),
        (-11, 4),
        (i8_min_valid, 3),
        (1, 1),
        (5, 5),
        (6, 6),
        (i8::MAX, i8::MAX as u8),
    ];

    for test in test_cases.iter() {
        let (x, x_corr) = *test;

        match i8::cast_to_unsigned(x, modu) {
            Some(res) => assert_eq!(res, x_corr),
            _ => assert_eq!(u8::MAX, x_corr, "x: {}", x),
        }
    }
}

#[test]
fn sign_cast_success_small_type_max_modu() {
    let modu = u8::MAX;
    let i8_min_valid = i8::MIN + 1;

    let test_cases: [(i8, u8); 5] = [
        (0, 0),
        (-1, modu - 1),
        (-100, modu - 100),
        (i8_min_valid, i8::MAX as u8 + 1),
        (i8::MAX, i8::MAX as u8),
    ];

    for test in test_cases.iter() {
        let (x, x_corr) = *test;

        match i8::cast_to_unsigned(x, modu) {
            Some(res) => assert_eq!(res, x_corr),
            _ => assert_eq!(u8::MAX, x_corr, "x: {}", x),
        }
    }
}

#[test]
fn sign_cast_failure_small_type() {
    match i8::cast_to_unsigned(i8::MIN, 1u8) {
        Some(_) => assert!(false),
        None => assert!(true),
    }
}

#[test]
fn sign_cast_success_large_type() {
    let modu = 7;
    let i128_min_valid = i128::MIN + 1;

    let test_cases: [(i128, u128); 8] = [
        (0, 0),
        (-3, 4),
        (-11, 3),
        (i128_min_valid, 6),
        (1, 1),
        (7, 7), // == modu
        (8, 8),
        (i128::MAX, i128::MAX as u128),
    ];

    for test in test_cases.iter() {
        let (x, x_corr) = *test;

        match i128::cast_to_unsigned(x, modu) {
            Some(res) => assert_eq!(res, x_corr),
            _ => assert_eq!(u128::MAX, x_corr, "x: {}", x),
        }
    }
}

#[test]
fn sign_cast_success_large_type_max_modu() {
    let modu = u128::MAX;
    let i128_min_valid = i128::MIN + 1;

    let test_cases: [(i128, u128); 4] = [
        (-1, modu - 1),
        (-100, modu - 100),
        (i128_min_valid, i128::MAX as u128 + 1),
        (i128::MAX, i128::MAX as u128),
    ];

    for test in test_cases.iter() {
        let (x, x_corr) = *test;

        match i128::cast_to_unsigned(x, modu) {
            Some(res) => assert_eq!(res, x_corr),
            _ => assert_eq!(u128::MAX, x_corr, "x: {}", x),
        }
    }
}

#[test]
fn sign_cast_failure_large_type() {
    match i128::cast_to_unsigned(i128::MIN, 1u128) {
        Some(_) => assert!(false),
        None => assert!(true),
    }
}
