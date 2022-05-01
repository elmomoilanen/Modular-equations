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
                assert_eq!(*elem_l, *elem_r, "x: {}, x_corr: {}", *elem_l, *elem_r);
            }
        }
        None => assert!(false, "x_corr: {:?}, x: None", sols_corr),
    }
}

#[test]
fn eq_small_type_uniq_sol() {
    let modu: u8 = 11;

    // [a, b, c, res]: ax + b = c (mod modu), res solution
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

    // (a, b, c, res): ax + b = c (mod modu), res solution
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

    // [a, b, c, res]: ax + b = c (mod modu), res solution
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

    // (a, b, c, res): ax + b = c (mod modu), res solution
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
    // [a, b, c, modu]: ax + b = c (mod modu), res solution
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

#[test]
fn eq_misc_uniq_sol() {
    // (a, b, c, modu, res): ax + b = c (mod modu), res solution
    let test_cases: [(i64, i64, i64, u64, u64); 5] = [
        (-9_832_503_233, 235_232_447, 653_245_724, 7919, 1395),
        (
            -9_832_503_233,
            235_232_447,
            653_245_724,
            50_131_820_635_651,
            44_363_860_600_404,
        ),
        (
            2,
            23_523_244_703_424_242,
            653_245_724_232,
            9_223_372_036_854_775_783,
            9_211_610_741_125_925_778,
        ),
        (
            -999_999_999_997,
            91_922_559,
            902_412_412,
            9_223_372_036_854_775_782,
            5_344_334_800_772_456_633,
        ),
        (199, 11_598, 7815, 1723, 1349),
    ];

    for test in test_cases.iter() {
        let lin_eq = LinEqSigned {
            a: test.0,
            b: test.1,
            c: test.2,
            modu: test.3,
        };
        let corr_sol = test.4;

        check_uniq_sol_correctness(lin_eq.solve(), corr_sol);
    }
}

#[test]
fn eq_large_type_max_modu_uniq_sol() {
    let modu: u128 = u128::MAX;

    // [a, b, c, res]: ax + b = c (mod modu), res solution
    let test_cases: [[u128; 4]; 3] = [
        [7, 5, 0, 194_447_066_811_964_836_264_785_489_961_010_406_545],
        [
            109_512_095_090_913_509_352_358_977_724_125_952,
            99_999_973_757_351_492_149_999_249_199_999_999_242,
            49_750_915_091_240_912_049_812_058_240_912_972_127,
            80_319_058_862_865_296_734_757_248_079_148_894_375,
        ],
        [
            9_879_248_124_912_945_125_128_451_298_412_424,
            92_194_944_128_529_852_491_240_124_120_950_185_249,
            3,
            338_366_548_178_000_552_919_815_972_240_182_319_536,
        ],
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
fn eq_large_signed_type_max_modu_uniq_sol() {
    let modu: u128 = u128::MAX;

    // (a, b, c, res): ax + b = c (mod modu), res solution
    let test_cases: [(i128, i128, i128, u128); 3] = [
        (
            -44_543_623_469_468_494_086_238_567_586_756_668_237,
            -23_535_923_957_235_048_235_923_659_813_587_358_522,
            -88_729_474_847_824_728_478_888_247_248_247_248,
            57_203_058_131_439_221_093_770_347_425_097_159_623,
        ),
        (
            -170_141_183_460_469_231_731_687_303_715_884_105_727,
            -170_141_183_460_469_231_731_687_303_715_884_105_725,
            170_141_183_460_469_231_731_687_303_715_884_105_727,
            340_282_366_920_938_463_463_374_607_431_768_211_449,
        ),
        (
            -99_999_999_999_999_999_999_999_999_999_999_999_998,
            -70_975_017_509_590_957_194_810_980_581_850_150_915,
            -139_581_984_501_959_359_150_124_120_412_041_240_360,
            241_748_985_104_027_545_122_753_820_863_754_140_795,
        ),
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
