use std::collections::HashMap;

use crate::quad::{QuadEq, QuadEqSigned};
use crate::UInt;

fn check_multiple_sols_correctness<T: UInt>(sols_cand: Option<Vec<T>>, sols_corr: &[T], modu: T) {
    match sols_cand {
        Some(sols) => {
            assert_eq!(sols.len(), sols_corr.len(), "mod: {}", modu);

            for (elem_l, elem_r) in sols.iter().zip(sols_corr.iter()) {
                assert_eq!(
                    *elem_l, *elem_r,
                    "x: {}, x_corr: {}, mod: {}",
                    *elem_l, *elem_r, modu
                );
            }
        }
        None => assert!(false, "x_corr: {:?}, x: None", sols_corr),
    }
}

#[test]
fn eq_small_type_quad_residue_odd_prime_mod() {
    // [d, modu, res_1, res_2]: x^2 = d (mod modu), modu odd prime
    // quadratic residue d has two roots res_1 and res_2
    let test_cases: [[u8; 4]; 8] = [
        [0, 3, 0, 0],
        [1, 3, 1, 2],
        [1, 5, 1, 4],
        [4, 5, 2, 3],
        [1, 11, 1, 10],
        [9, 11, 3, 8],
        [5, 41, 13, 28],
        [99, 139, 51, 88],
    ];

    for test in test_cases.iter() {
        let quad_eq = QuadEq {
            a: 1,
            b: 0,
            c: 0,
            d: test[0],
            modu: test[1],
        };

        let corr_sol = if test[2] > 0 {
            vec![test[2], test[3]]
        } else {
            vec![test[2]]
        };

        check_multiple_sols_correctness(quad_eq.solve(), &corr_sol, test[1]);
    }
}

#[test]
fn eq_small_signed_type_quad_residue_odd_prime_mod() {
    // (d, modu, res_1, res_2): x^2 = d (mod modu), modu odd prime
    // quadratic residue d has two roots res_1 and res_2

    let test_cases: [(i8, u8, u8, u8); 8] = [
        (0, 3, 0, 0),
        (-2, 3, 1, 2),
        (-4, 5, 1, 4),
        (-1, 5, 2, 3),
        (-10, 11, 1, 10),
        (-2, 11, 3, 8),
        (-36, 41, 13, 28),
        (-40, 139, 51, 88),
    ];

    for test in test_cases.iter() {
        let quad_eq = QuadEqSigned {
            a: 1,
            b: 0,
            c: 0,
            d: test.0,
            modu: test.1,
        };

        let corr_sol = if test.2 > 0 {
            vec![test.2, test.3]
        } else {
            vec![test.2]
        };

        check_multiple_sols_correctness(quad_eq.solve(), &corr_sol, test.1);
    }
}

#[test]
fn eq_small_type_quad_residue_and_nonresidue_odd_prime_mod() {
    let modu: u8 = 23; // small odd prime

    let quad_residues: [u8; 12] = [0, 1, 2, 3, 4, 6, 8, 9, 12, 13, 16, 18];
    let quad_roots: [[u8; 2]; 12] = [
        [0, 0],
        [1, 22],
        [5, 18],
        [7, 16],
        [2, 21],
        [11, 12],
        [10, 13],
        [3, 20],
        [9, 14],
        [6, 17],
        [4, 19],
        [8, 15],
    ];

    let residue_roots: HashMap<_, _> = quad_residues.iter().zip(quad_roots.iter()).collect();

    for d in 0..modu {
        let quad_eq = QuadEq {
            a: 1,
            b: 0,
            c: 0,
            d,
            modu,
        };

        match (quad_eq.solve(), residue_roots.get(&d)) {
            (Some(x_sols), Some(roots)) => {
                let x_corr = *roots;

                if x_sols.len() == 1 {
                    assert_eq!(x_sols[0], x_corr[0], "d: {}", d);
                } else {
                    assert_eq!(x_sols[0], x_corr[0], "d: {}", d);
                    assert_eq!(x_sols[1], x_corr[1], "d: {}", d);
                }
            }
            (None, None) => {}
            _ => {
                assert!(false, "d: {}", d);
            }
        }
    }
}

#[test]
fn eq_mid_type_quad_residue_odd_prime_mod() {
    // [d, modu, res_1, res_2]: x^2 = d (mod modu), modu odd prime
    // quadratic residue d has two roots res_1 and res_2
    let test_cases: [[u64; 4]; 8] = [
        [999, 14_867, 3168, 11_699],
        [899, 50_261, 14_696, 35_565],
        [65_535, 65_521, 29_977, 35_544],
        [987_321, 2_147_483_647, 1_009_548_103, 1_137_935_544],
        [100_000, 50_000_038_603, 11_742_120_277, 38_257_918_326],
        [
            900_999,
            72_057_594_037_927_931,
            8_535_927_834_217_309,
            63_521_666_203_710_622,
        ],
        [
            999_999_999_999,
            9_223_372_036_854_775_337,
            3_809_988_184_654_627_668,
            5_413_383_852_200_147_669,
        ],
        [
            9_999_999_999_999_999,
            9_223_372_036_854_775_783,
            287_990_794_123_520_843,
            8_935_381_242_731_254_940,
        ],
    ];

    for test in test_cases.iter() {
        let quad_eq = QuadEq {
            a: 1,
            b: 0,
            c: 0,
            d: test[0],
            modu: test[1],
        };

        let corr_sol = vec![test[2], test[3]];

        check_multiple_sols_correctness(quad_eq.solve(), &corr_sol, test[1]);
    }
}

#[test]
fn eq_large_type_quad_residue_odd_prime_mod() {
    let smaller_modu = 41_538_374_868_278_621_028_243_970_633_760_399;
    let larger_modu = 340_282_366_920_938_463_463_374_607_431_768_211_297;

    // [d, modu, res_1, res_2]: x^2 = d (mod modu), modu odd prime
    // quadratic residue d has two roots res_1 and res_2
    let test_cases: [[u128; 4]; 8] = [
        [
            1,
            smaller_modu,
            1,
            41_538_374_868_278_621_028_243_970_633_760_398,
        ],
        [
            1_902_359_235_235_235,
            smaller_modu,
            4_308_797_534_900_248_116_584_966_211_687_609,
            37_229_577_333_378_372_911_659_004_422_072_790,
        ],
        [
            9_824_202_184_002_518_284_814_224,
            smaller_modu,
            11_240_629_191_872_231_686_281_671_522_360_515,
            30_297_745_676_406_389_341_962_299_111_399_884,
        ],
        [
            41_538_374_868_278_621_028_243_970_633_760_388,
            smaller_modu,
            4_736_182_786_991_917_864_540_101_503_501_134,
            36_802_192_081_286_703_163_703_869_130_259_265,
        ],
        [
            1,
            larger_modu,
            1,
            340_282_366_920_938_463_463_374_607_431_768_211_296,
        ],
        [
            1111,
            larger_modu,
            42_975_499_967_547_402_654_183_974_193_836_944_053,
            297_306_866_953_391_060_809_190_633_237_931_267_244,
        ],
        [
            340_282_366_920_938_463_463_374_607_431_768_211_295,
            larger_modu,
            33_190_663_755_207_043_105_942_532_539_854_070_407,
            307_091_703_165_731_420_357_432_074_891_914_140_890,
        ],
        [
            340_282_366_920_938_463_463_374_607_431_768_211_291,
            larger_modu,
            8_159_441_886_976_089_234_691_297_995_035_384_680,
            332_122_925_033_962_374_228_683_309_436_732_826_617,
        ],
    ];

    for test in test_cases.iter() {
        let modu = test[1];

        let quad_eq = QuadEq {
            a: 1,
            b: 0,
            c: 0,
            d: test[0],
            modu,
        };

        let corr_sol = vec![test[2], test[3]];

        check_multiple_sols_correctness(quad_eq.solve(), &corr_sol, modu);
    }
}

#[test]
fn eq_small_type_odd_prime_mod() {
    // [a, b, c, d, modu, res_1, res_2]: ax^2 + bx + c = d (mod modu)
    // modu is odd prime and the equation has two solutions

    let test_cases: [[u8; 7]; 10] = [
        [3, 6, 1, 0, 19, 7, 10],
        [3, 6, 0, 18, 19, 7, 10],
        [1, 0, 1, 0, 5, 2, 3],
        [1, 3, 0, 1, 3, 1, 2],
        [1, 1, 0, 0, 7, 0, 6],
        [1, 1, 5, 0, 11, 2, 8],
        [2, 8, 2, 0, 23, 5, 14],
        [11, 7, 99, 145, 251, 108, 188],
        [255, 254, 99, 145, 251, 71, 242],
        [255, 254, 255, 251, 251, 92, 221],
    ];

    for test in test_cases.iter() {
        let modu = test[4];

        let quad_eq = QuadEq {
            a: test[0],
            b: test[1],
            c: test[2],
            d: test[3],
            modu,
        };

        let corr_sol = vec![test[5], test[6]];
        check_multiple_sols_correctness(quad_eq.solve(), &corr_sol, modu);
    }
}

#[test]
fn hensel_method_with_power_of_three() {
    let quad_eq = QuadEq::<u8> {
        a: 1,
        b: 3,
        c: 0,
        d: 1,
        modu: 3,
    };

    // quad_eq sols: 1, 2
    let quad_sols: Vec<u8> = vec![1, 2];

    // [x_1, x_2, prm_k]: lifted solutions x_1 and x_2 for modu^prm_k
    let test_cases: [[u8; 3]; 3] = [
        [1, 2, 1],  // normally for this Hensel's method wouldn't be used
        [4, 2, 2],  // mod 9
        [4, 20, 3], // mod 27
    ];

    for test in test_cases.iter() {
        let correct_sols = vec![test[0], test[1]];
        let modulo = quad_eq.modu.pow(test[2].into());

        let lifted_sols = quad_eq.lift_with_hensel_method(quad_sols.clone(), test[2]);

        check_multiple_sols_correctness(lifted_sols, &correct_sols, modulo);
    }
}

#[test]
fn hensel_method_with_power_of_five() {
    let quad_eq = QuadEq::<u32> {
        a: 1,
        b: 0,
        c: 0,
        d: 4,
        modu: 5,
    };

    // quad_eq sols: 2, 3
    let quad_sols: Vec<u32> = vec![2, 3];

    // [x_1, x_2, prm_k]: lifted solutions x_1 and x_2 for modu^prm_k
    let test_cases: [(u32, u32, u8); 3] = [
        (2, 3, 1),   // normally for this Hensel's method wouldn't be used
        (2, 23, 2),  // mod 25
        (2, 123, 3), // mod 125
    ];

    for test in test_cases.iter() {
        let correct_sols = vec![test.0, test.1];
        let modulo = quad_eq.modu.pow(test.2.into());

        let lifted_sols = quad_eq.lift_with_hensel_method(quad_sols.clone(), test.2);

        check_multiple_sols_correctness(lifted_sols, &correct_sols, modulo);
    }
}
