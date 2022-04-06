use crate::lin::{LinEq, LinEqSigned};
use crate::UInt;

fn check_uniq_sol_correctness<T>(sol_cand: Option<Vec<T>>, sol_corr: T)
where
    T: UInt,
{
    match sol_cand {
        Some(sol) => {
            assert_eq!(sol.len(), 1);
            assert_eq!(sol[0], sol_corr, "x_corr: {}, x: {}", sol_corr, sol[0]);
        }
        None => assert!(false, "x_corr: {}, x: None", sol_corr),
    }
}

fn check_multiple_sols_correctness<T>(sols_cand: Option<Vec<T>>, sols_corr: &[T])
where
    T: UInt,
{
    match sols_cand {
        Some(sols) => {
            assert_eq!(sols.len(), sols_corr.len());

            for (elem_l, elem_r) in sols.iter().zip(sols_corr.iter()) {
                assert_eq!(*elem_l, *elem_r, "left: {}, right: {}", *elem_l, *elem_r);
            }
        }
        None => assert!(false, "x_corr: {:?}, x: None", sols_corr),
    }
}

#[test]
fn eq_small_type_uniq_sol() {
    let modu: u8 = 11;

    // [a, b, c, res]: ax + b = c (mod modu)
    let test_cases: [[u8; 4]; 5] = [
        [2, 3, 1, 10],
        [2, 3, 10, 9],
        [19, 100, 99, 4],
        [75, 1, 255, 5],
        [255, 255, 7, 8],
    ];

    for test in test_cases.iter() {
        let lin_eq = LinEq {
            a: test[0],
            b: test[1],
            c: test[2],
            modu,
        };
        let corr_sol = test[3];
        check_uniq_sol_correctness(lin_eq.solve(), corr_sol);
    }
}

#[test]
fn eq_small_signed_type_uniq_sol() {
    let modu: u8 = 7;

    // (a, b, c, res): ax + b = c (mod modu)
    let test_cases: [(i8, i8, i8, u8); 5] = [
        (-1, -1, 3, 3),
        (-127, -125, -99, 2),
        (-127, 125, 99, 5),
        (-3, 127, -1, 3),
        (-1, 0, 127, 6),
    ];

    for test in test_cases.iter() {
        let lin_eq = LinEqSigned {
            a: test.0,
            b: test.1,
            c: test.2,
            modu,
        };
        let corr_sol = test.3;
        check_uniq_sol_correctness(lin_eq.solve(), corr_sol);
    }
}

#[test]
fn eq_small_type_max_mod_uniq_sol() {
    let modu: u8 = u8::MAX;

    // [a, b, c, res]: ax + b = c (mod modu)
    let test_cases: [[u8; 4]; 5] = [
        [2, 0, 101, 178],
        [133, 78, 155, 14],
        [133, 78, 1, 241],
        [2, 179, 1, 166],
        [1, 254, 1, 2],
    ];

    for test in test_cases.iter() {
        let lin_eq = LinEq {
            a: test[0],
            b: test[1],
            c: test[2],
            modu,
        };
        let corr_sol = test[3];
        check_uniq_sol_correctness(lin_eq.solve(), corr_sol);
    }
}

#[test]
fn eq_small_signed_type_max_mod_uniq_sol() {
    let modu: u8 = u8::MAX;

    // (a, b, c, res): ax + b = c (mod modu)
    let test_cases: [(i8, i8, i8, u8); 5] = [
        (-127, -127, 1, 1),
        (-124, -79, 77, 81),
        (-2, -1, 127, 191),
        (-2, 10, 50, 235),
        (-52, -71, -125, 207),
    ];

    for test in test_cases.iter() {
        let lin_eq = LinEqSigned {
            a: test.0,
            b: test.1,
            c: test.2,
            modu,
        };
        let corr_sol = test.3;
        check_uniq_sol_correctness(lin_eq.solve(), corr_sol);
    }
}

#[test]
fn eq_small_type_multiple_sols() {
    let lin_eq = LinEq::<u8> {
        a: 3,
        b: 1,
        c: 250,
        modu: 255,
    };

    let corr_sols: Vec<u8> = vec![83, 168, 253];
    check_multiple_sols_correctness(lin_eq.solve(), &corr_sols);

    let lin_eq = LinEq::<u8> {
        a: 5,
        b: 10,
        c: 50,
        modu: 255,
    };

    let corr_sols: Vec<u8> = vec![8, 59, 110, 161, 212];
    check_multiple_sols_correctness(lin_eq.solve(), &corr_sols);

    let lin_eq = LinEq::<u8> {
        a: 71,
        b: 0,
        c: 142,
        modu: 213,
    };

    let corr_sols: Vec<u8> = (2..).step_by(3).take_while(|x| x < &lin_eq.modu).collect();

    check_multiple_sols_correctness(lin_eq.solve(), &corr_sols);
}

#[test]
fn eq_small_signed_type_multiple_sols() {
    let lin_eq = LinEqSigned::<i8, u8> {
        a: -55,
        b: -55,
        c: 65,
        modu: 105,
    };

    let corr_sols: Vec<u8> = vec![15, 36, 57, 78, 99];
    check_multiple_sols_correctness(lin_eq.solve(), &corr_sols);

    let lin_eq = LinEqSigned::<i8, u8> {
        a: -17,
        b: -1,
        c: -1,
        modu: 255,
    };

    let mut corr_sols: Vec<u8> = (0..).step_by(15).take_while(|x| x < &240).collect();
    corr_sols.push(240); // must be separate push to avoid overflow for type u8

    check_multiple_sols_correctness(lin_eq.solve(), &corr_sols);
}

#[test]
fn eq_small_type_no_sol() {
    // [a, b, c, modu]: ax + b = c (mod modu)
    let test_cases: [[u8; 4]; 3] = [[3, 1, 254, 255], [2, 0, 3, 8], [200, 31, 50, 200]];

    for test in test_cases.iter() {
        let lin_eq = LinEq {
            a: test[0],
            b: test[1],
            c: test[2],
            modu: test[3],
        };
        match lin_eq.solve() {
            None => assert!(true),
            Some(x) => assert!(false, "x_corr: None, x: {:?}", x),
        }
    }
}
