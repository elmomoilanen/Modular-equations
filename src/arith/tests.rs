use crate::{Arith, CoreArith};

impl CoreArith<u8> for u8 {}
impl Arith<u8> for u8 {}

impl CoreArith<u128> for u128 {}
impl Arith<u128> for u128 {}

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
        assert_eq!(u8::add(x, y, modu), test[2], "x: {}, y: {}", x, y);
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
        assert_eq!(u8::add(x, y, modu), test[2], "x: {}, y: {}", x, y);
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
        assert_eq!(u128::add(x, y, modu), test[2], "x: {}, y: {}", x, y);
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
        assert_eq!(u128::add(x, y, modu), test[2], "x: {}, y: {}", x, y);
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
        assert_eq!(u8::sub(x, y, modu), test[2], "x: {}, y: {}", x, y);
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
        assert_eq!(u128::sub(x, y, modu), test[2], "x: {}, y: {}", x, y);
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
        assert_eq!(u8::mult(x, y, modu), test[2], "x: {}, y: {}", x, y);
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
        assert_eq!(u128::mult(x, y, modu), test[2], "x: {}, y: {}", x, y);
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
        assert_eq!(u128::mult(x, y, modu), test[2], "x: {}, y: {}", x, y);
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
        assert_eq!(u8::exp(x, y, modu), test[2], "x: {}, y: {}", x, y);
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
        assert_eq!(u128::exp(x, y, modu), test[2], "x: {}, y: {}", x, y);
    }
}

#[test]
fn gcd_small_type() {
    // [x, y, res]: gcd(x, y) = res
    let test_cases: [[u8; 3]; 5] = [[1, 0, 1], [0, 1, 1], [2, 3, 1], [3, 2, 1], [34, 85, 17]];

    for test in test_cases.iter() {
        let (x, y) = (test[0], test[1]);
        assert_eq!(u8::gcd(x, y), test[2], "x: {}, y: {}", x, y);
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
        assert_eq!(u128::gcd(x, y), test[2], "x: {}, y: {}", x, y);
    }
}
