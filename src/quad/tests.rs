use std::collections::{HashMap, HashSet};

use crate::quad::{QuadEq, QuadEqSigned};
use crate::UInt;

fn check_multiple_sols_correctness<T: UInt>(sols_cand: Option<Vec<T>>, sols_corr: &[T], modu: T) {
    // right_arr can be larger as it might contain zero padding
    match sols_cand {
        Some(sols) => {
            assert!(sols.len() <= sols_corr.len(), "mod: {}", modu);

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

    let test_cases: [[u8; 7]; 12] = [
        [9, 5, 1, 0, 3, 1, 1],
        [165, 7, 2, 0, 11, 6, 6],
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

        let corr_sol = if test[5] == test[6] {
            // only one root
            vec![test[5]]
        } else {
            vec![test[5], test[6]]
        };

        check_multiple_sols_correctness(quad_eq.solve(), &corr_sol, modu);
    }
}

#[test]
fn eq_large_type_odd_prime_mod() {
    // [a, b, c, d, modu, res_1, res_2]: ax^2 + bx + c = d (mod modu)
    // modu is odd prime and the equation has two solutions

    let test_cases: [[u128; 7]; 5] = [
        [90_853, 51_252, 18_000, 6000, 99_991, 267, 63_226],
        [
            1,
            0,
            0,
            9_999_999_999_999_999,
            9_223_372_036_854_775_783,
            287_990_794_123_520_843,
            8_935_381_242_731_254_940,
        ],
        [
            1_212_421_490_235,
            91_595_920_724_124,
            0,
            74_825_828_142,
            9_223_372_036_854_775_783,
            3_586_932_499_142_287_740,
            6_079_892_515_866_449_701,
        ],
        [
            99_999_988_888_777_777_755_555_333_333_311_111_111,
            9_999_999_999_999_999_999_999_999_999_999_999_999,
            1_112_223_334_445_550_009_991_241_241_241_252_141,
            0,
            340_282_366_920_938_463_463_374_607_431_768_211_297,
            93_206_577_167_428_250_771_283_116_410_888_834_940,
            227_169_082_945_979_312_544_589_254_078_268_951_586,
        ],
        [
            340_282_366_920_938_463_463_374_607_431_768_211_295,
            340_282_366_920_938_463_463_374_607_431_768_211_291,
            340_282_366_920_938_463_463_374_607_431_768_211_287,
            0,
            340_282_366_920_938_463_463_374_607_431_768_211_297,
            29_529_484_007_650_875_106_259_903_148_918_400_628,
            310_752_882_913_287_588_357_114_704_282_849_810_666,
        ],
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
fn eq_small_signed_type_odd_prime_mod() {
    // [a, b, c, d, modu, res_1, res_2]: ax^2 + bx + c = d (mod modu)
    // modu is odd prime and the equation has two solutions

    let test_cases: [(i8, i8, i8, i8, u8, u8, u8); 5] = [
        (-126, 125, -125, -125, 11, 0, 3),
        (-126, -126, -126, -126, 103, 0, 102),
        (-1, 126, -126, -121, 5, 0, 1),
        (-3, -1, 1, 0, 103, 10, 24),
        (-126, -125, 125, 1, 251, 48, 204),
    ];

    for test in test_cases.iter() {
        let modu = test.4;

        let quad_eq = QuadEqSigned {
            a: test.0,
            b: test.1,
            c: test.2,
            d: test.3,
            modu,
        };

        let corr_sol = vec![test.5, test.6];
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

#[test]
fn combine_solution_for_composite_modu_small_type() {
    let modu = 77;
    let all_sols: Vec<(u8, u8)> = vec![(3, 7), (4, 7), (1, 11), (10, 11)];

    let modu_start_indices: Vec<usize> = vec![0, 2];
    let modu_sol_counts: Vec<usize> = vec![2, 2];

    let combined_sols = QuadEq::combine_solution_for_compo_modu(
        all_sols,
        modu,
        modu_start_indices,
        modu_sol_counts,
    );

    let correct_sols: Vec<u8> = vec![10, 32, 45, 67];

    assert_eq!(combined_sols.len(), correct_sols.len());
    assert_eq!(combined_sols, correct_sols);
}

#[test]
fn combine_solution_for_composite_modu_small_type_zero_sol() {
    let modu = 60;
    let all_sols: Vec<(u8, u8)> = vec![(0, 3), (0, 4), (2, 5), (3, 5)];

    let modu_start_indices: Vec<usize> = vec![0, 1, 2];
    let modu_sol_counts: Vec<usize> = vec![1, 1, 2];

    let combined_sols = QuadEq::combine_solution_for_compo_modu(
        all_sols,
        modu,
        modu_start_indices,
        modu_sol_counts,
    );

    let correct_sols: Vec<u8> = vec![12, 48];

    assert_eq!(combined_sols.len(), correct_sols.len());
    assert_eq!(combined_sols, correct_sols);
}

#[test]
fn combine_solution_for_composite_modu_mid_type() {
    let modu = 315;
    let all_sols: Vec<(u32, u32)> = vec![(3, 5), (4, 5), (1, 7), (3, 7), (2, 9), (4, 9)];

    let modu_start_indices: Vec<usize> = vec![0, 2, 4];
    let modu_sol_counts: Vec<usize> = vec![2, 2, 2];

    let combined_sols = QuadEq::combine_solution_for_compo_modu(
        all_sols,
        modu,
        modu_start_indices,
        modu_sol_counts,
    );

    let correct_sols: Vec<u32> = vec![29, 38, 94, 148, 164, 218, 274, 283];

    assert_eq!(combined_sols.len(), correct_sols.len());
    assert_eq!(combined_sols, correct_sols);
}

#[test]
fn eq_mid_signed_type_odd_power_of_prime_mod() {
    // [a, b, c, d, modu, res_1, res_2]: ax^2 + bx + c = d (mod modu)
    // modu is a power of odd prime and the equation has two solutions

    let test_cases: [(i64, i64, i64, i64, u64, u64, u64); 6] = [
        (1, 1, 47, 0, 343, 99, 243),
        (
            999_999,
            1,
            -111_111,
            0,
            4_501_401_006_735_361,
            1_557_059_636_720_593,
            2_962_779_126_976_113,
        ),
        (
            999_999_999_999_999_999,
            -999_999_999_912_421,
            214_081_248_358_023_524,
            0,
            2_862_423_051_509_815_793,
            2_303_508_973_012_165_250,
            2_721_119_028_450_610_552,
        ),
        (
            -125_125_121_242_124,
            -54_224_212_353_523,
            113_535_124_124_255,
            0,
            4_611_686_014_132_420_609,
            1_523_832_291_260_501_430,
            2_424_331_690_299_886_142,
        ),
        (
            -1,
            1,
            -1,
            0,
            4_611_686_014_132_420_609,
            581_405_252_161_832_858,
            4_030_280_761_970_587_752,
        ),
        (
            1,
            0,
            -1,
            0,
            79_792_266_297_612_001,
            1,
            79_792_266_297_612_000,
        ),
    ];

    for test in test_cases.iter() {
        let modu = test.4;

        let quad_eq = QuadEqSigned {
            a: test.0,
            b: test.1,
            c: test.2,
            d: test.3,
            modu,
        };

        let corr_sol = vec![test.5, test.6];
        check_multiple_sols_correctness(quad_eq.solve(), &corr_sol, modu);
    }
}

#[test]
fn eq_small_type_composite_mod() {
    // [a, b, c, d, modu]: ax^2 + bx + c = d (mod modu)

    let test_cases: [[u16; 5]; 5] = [
        [1, 3, 0, 298, 315],
        [250, 253, 251, 251, 171],
        [250, 253, 253, 249, 221],
        [250, 253, 4, 0, 125],
        [911, 1211, 7512, 0, 22599],
    ];

    let correct_sols: [[u16; 8]; 5] = [
        [29, 38, 94, 148, 164, 218, 274, 283],
        [0, 36, 95, 131, 0, 0, 0, 0],
        [75, 92, 166, 183, 0, 0, 0, 0],
        [82, 0, 0, 0, 0, 0, 0, 0],
        [33, 437, 1491, 21578, 0, 0, 0, 0],
    ];

    let it = test_cases.iter().zip(correct_sols.iter());

    for (test, corr) in it {
        let modu = test[4];

        let quad_eq = QuadEq {
            a: test[0],
            b: test[1],
            c: test[2],
            d: test[3],
            modu,
        };

        check_multiple_sols_correctness(quad_eq.solve(), corr, modu);
    }
}

#[test]
fn eq_small_signed_type_composite_mod() {
    // [a, b, c, d, modu]: ax^2 + bx + c = d (mod modu)

    let test_cases: [(i8, i8, i8, i8, u8); 5] = [
        (-1, 5, -11, 0, 115),
        (-125, 51, -119, 0, 203),
        (-127, 125, -127, 0, 125),
        (-127, 125, -127, 0, 215),
        (-110, 101, -11, 0, 253),
    ];

    let correct_sols: [[u8; 4]; 5] = [
        [13, 38, 82, 107],
        [7, 47, 105, 152],
        [57, 68, 0, 0],
        [87, 173, 0, 0],
        [110, 132, 0, 0],
    ];

    let it = test_cases.iter().zip(correct_sols.iter());

    for (test, corr) in it {
        let modu = test.4;

        let quad_eq = QuadEqSigned {
            a: test.0,
            b: test.1,
            c: test.2,
            d: test.3,
            modu,
        };

        check_multiple_sols_correctness(quad_eq.solve(), corr, modu);
    }
}

#[test]
fn eq_large_type_composite_mod() {
    // [a, b, c, d, modu]: ax^2 + bx + c = d (mod modu)

    let test_cases: [[u128; 5]; 3] = [
        [
            1,
            3_124_213,
            1_231_121_313_123,
            0,
            9_223_372_036_854_775_803,
        ],
        //[2, 3, 5, 0, 5_000_000_000_000_000_000],
        //[1, 2, 1, 0, 614_889_782_588_491_410],
        [
            99,
            1,
            12,
            0,
            170_141_183_460_469_231_731_687_303_715_884_105_725,
        ],
        [
            999_999_999_999_999_999_999_999_995,
            1,
            129_898_232_356_236_523_552_122_222,
            0,
            170_141_183_460_469_231_731_687_303_715_884_105_725,
        ],
    ];

    let correct_sols: [[u128; 16]; 3] = [
        [
            566_238_308_012_032_964,
            1_255_535_490_499_711_868,
            2_165_339_584_199_169_968,
            2_854_636_766_686_848_872,
            3_294_277_924_546_544_117,
            3_640_695_653_630_291_565,
            3_983_575_107_034_223_021,
            4_329_992_836_117_970_469,
            4_893_379_200_733_681_121,
            5_239_796_929_817_428_569,
            5_582_676_383_221_360_025,
            5_929_094_112_305_107_473,
            6_368_735_270_164_802_718,
            7_058_032_452_652_481_622,
            7_967_836_546_351_939_722,
            8_657_133_728_839_618_626,
        ],
        // [
        //     966_704_601_316_436_515,
        //     2_719_160_800_905_243_171,
        //     0,
        //     0,
        //     0,
        //     0,
        //     0,
        //     0,
        //     0,
        //     0,
        //     0,
        //     0,
        //     0,
        //     0,
        //     0,
        //     0,
        // ],
        // [
        //     614_889_782_588_491_409,
        //     0,
        //     0,
        //     0,
        //     0,
        //     0,
        //     0,
        //     0,
        //     0,
        //     0,
        //     0,
        //     0,
        //     0,
        //     0,
        //     0,
        //     0,
        // ],
        [
            8_260_234_846_403_945_420_740_348_997_530_770_679,
            13_531_585_419_037_972_340_449_499_599_614_789_529,
            26_501_404_144_720_614_684_539_716_431_294_058_029,
            110_986_420_873_840_380_654_197_498_692_652_694_072,
            123_956_239_599_523_022_998_287_715_524_331_962_572,
            129_227_590_172_157_049_917_996_866_126_415_981_422,
            142_197_408_897_839_692_262_087_082_958_095_249_922,
            165_431_599_581_190_534_808_337_435_881_735_607_904,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            60_434_384_790_066_986_729_016_865_994_249_286_173,
            61_697_653_765_764_595_985_970_550_442_907_945_523,
            94_022_010_059_619_704_436_855_214_975_699_552_698,
            95_285_279_035_317_313_693_808_899_424_358_212_048,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    ];

    let it = test_cases.iter().zip(correct_sols.iter());

    for (test, corr) in it {
        let modu = test[4];

        let quad_eq = QuadEq {
            a: test[0],
            b: test[1],
            c: test[2],
            d: test[3],
            modu,
        };

        check_multiple_sols_correctness(quad_eq.solve(), corr, modu);
    }
}

#[test]
fn eq_large_signed_type_composite_mod() {
    // [a, b, c, d, modu, res_1, res_2]: ax^2 + bx + c = d (mod modu)
    // modu is odd prime and the equation has two solutions

    let test_cases: [(i128, i128, i128, i128, u128); 4] = [
        // (
        //     -999_999_999_999_991_320,
        //     -911_191_191_232_131_242,
        //     -982_481_241_441_241_120,
        //     0,
        //     9_223_372_036_854_775_798,
        // ),
        (
            10_000_000_000_000_000_000_000_000_000_323,
            -1_111_325_235_235_325_235_125_325_235_363,
            34_632_521_502_352_554_856_485_672_367_457,
            0,
            340_282_366_920_938_463_463_374_607_431_768_211_451,
        ),
        (
            1_000_000_000_000_000_000_000_000_000_032_303_951,
            -18_973_295_233_255_235_235_325_235_125_325_235_363,
            -9_934_963_986_720_693_463_409_640_960_136_233_235,
            0,
            340_282_366_920_938_463_463_374_607_431_768_211_451,
        ),
        (
            -1_121_412,
            -9_242_141_254,
            11_124_124,
            0,
            15_413_179_794_770_734_626_518_662_321_719_325_259,
        ),
        (
            -1,
            -9_215_802_424,
            0,
            0,
            7_060_005_655_815_754_299_976_961_394_452_809,
        ),
    ];

    let correct_sols: [[u128; 8]; 4] = [
        // [
        //     1_732_184_316_592_242_493,
        //     2_942_415_274_567_158_820,
        //     3_204_775_211_312_642_198,
        //     4_415_006_169_287_558_525,
        //     6_343_870_335_019_630_392,
        //     7_554_101_292_994_546_719,
        //     7_816_461_229_740_030_097,
        //     9_026_692_187_714_946_424,
        // ],
        [
            6_589_677_581_610_708_044_841_679_428_405_462_385,
            21_051_736_208_735_753_826_549_962_876_354_104_906,
            34_393_620_280_133_968_662_360_643_550_230_898_822,
            333_530_160_431_150_956_672_405_606_186_296_879_920,
            0,
            0,
            0,
            0,
        ],
        [
            76_258_869_869_339_036_240_435_830_412_688_421_207,
            181_300_214_722_385_675_736_816_450_539_867_151_718,
            189_717_163_397_186_336_448_057_011_930_730_161_950,
            294_758_508_250_232_975_944_437_632_057_908_892_461,
            0,
            0,
            0,
            0,
        ],
        [
            9_167_481_338_520_452_481_980_471_390_430_002_497,
            14_046_245_344_533_778_550_322_732_786_908_149_789,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0,
            1_193_690_482_538_628_328_622_478_335_188_826,
            5_866_315_173_277_125_971_354_473_843_461_559,
            7_060_005_655_815_754_299_976_952_178_650_385,
            0,
            0,
            0,
            0,
        ],
    ];

    let it = test_cases.iter().zip(correct_sols.iter());

    for (test, corr) in it {
        let modu = test.4;

        let quad_eq = QuadEqSigned {
            a: test.0,
            b: test.1,
            c: test.2,
            d: test.3,
            modu,
        };

        check_multiple_sols_correctness(quad_eq.solve(), corr, modu);
    }
}

#[test]
fn eq_large_signed_type_composite_mod_count_of_solutions() {
    let quad_eq = QuadEqSigned::<i128, u128> {
        a: 1,
        b: 0,
        c: -1,
        d: 0,
        modu: 340_282_366_920_938_463_463_374_607_431_768_211_455,
    };

    let corr_sol_count = 512;

    let result = quad_eq.solve();

    match result {
        None => assert!(false),
        Some(res) => {
            assert_eq!(res.len(), corr_sol_count);

            let sol_set: HashSet<u128> = HashSet::from_iter(res);

            // test few correct solution that should be there
            assert!(sol_set.contains(&1));
            assert!(sol_set.contains(&18_446_744_073_709_551_616));
            assert!(sol_set.contains(&340_282_366_920_938_463_463_374_607_431_768_211_454));
        }
    }
}
