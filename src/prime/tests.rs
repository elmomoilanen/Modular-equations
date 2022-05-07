use crate::prime::is_odd_prime;

#[test]
fn is_prime_first_odd_primes() {
    let test_primes: [u8; 17] = [
        3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61,
    ];

    for prime in test_primes.iter() {
        assert_eq!(is_odd_prime(*prime), true, "{}", *prime);
    }
}

#[test]
fn is_prime_small_composites() {
    // 2 is not composite but it's also not an odd prime
    let test_numbers: [u8; 8] = [0, 1, 2, 4, 8, 15, 25, u8::MAX];

    for number in test_numbers.iter() {
        assert_eq!(is_odd_prime(*number), false, "{}", *number);
    }
}

#[test]
fn is_prime_small_odd_primes() {
    let test_primes: [u32; 10] = [67, 71, 73, 79, 83, 89, 97, 101, 103, 107];

    for prime in test_primes.iter() {
        assert_eq!(is_odd_prime(*prime), true, "{}", *prime);
    }
}

#[test]
fn is_prime_small_range() {
    let start_num: u32 = 67;
    let stop_num = 108; // range [67, 108)

    // there should be ten primes within this range, see the previous test

    let prime_count = (start_num..stop_num).filter(|x| is_odd_prime(*x)).count();

    assert_eq!(prime_count, 10);
}

#[test]
fn is_prime_smaller_primes() {
    let test_primes: [u64; 25] = [
        7927,
        7933,
        7937,
        7949,
        8009,
        8191,
        16_369,
        131_071,
        319_993,
        999_331,
        15_485_863,
        256_203_221,
        633_910_099,
        982_451_653,
        2_147_483_647,
        4_294_967_291,
        50_000_038_603,
        549_755_813_881,
        36_028_797_018_963_913,
        72_057_594_037_927_931,
        2_305_843_009_213_693_951,
        9_223_372_036_854_775_337,
        9_223_372_036_854_775_783,
        18_446_744_073_709_551_533,
        18_446_744_073_709_551_557,
    ];

    for prime in test_primes.iter() {
        assert_eq!(is_odd_prime(*prime), true, "{}", *prime);
    }
}

#[test]
fn is_prime_smaller_composites() {
    let test_composites: [u64; 15] = [
        1_795_265_021,
        1_795_265_022,
        1_795_265_023,
        2_147_483_643,
        4_294_967_293,
        10_449_049_901,
        150_267_335_403,
        430_558_874_533,
        35_184_372_088_697,
        50_131_820_635_651,
        936_916_995_253_453,
        25_012_804_853_117_569,
        9_223_372_036_854_775_781,
        9_223_372_036_854_775_806,
        9_223_372_036_854_775_807,
    ];

    for comp in test_composites.iter() {
        assert_eq!(is_odd_prime(*comp), false, "{}", *comp);
    }
}

#[test]
fn is_prime_larger_primes() {
    let test_primes: [u128; 20] = [
        36_893_488_147_419_103_183,
        36_893_488_147_419_102_739,
        73_786_976_294_838_206_459,
        37_778_931_862_957_161_709_471,
        37_778_931_862_957_161_709_361,
        37_778_931_862_957_161_709_289,
        37_778_931_862_957_161_709_279,
        618_970_019_642_690_137_449_562_111,
        618_970_019_642_690_137_449_562_091,
        618_970_019_642_690_137_449_562_081,
        19_807_040_628_566_084_398_385_987_581,
        19_807_040_628_566_084_398_385_987_573,
        2_535_301_200_456_458_802_993_406_410_683,
        2_535_301_200_456_458_802_993_406_410_653,
        2_535_301_200_456_458_802_993_406_410_539,
        2_535_301_200_456_458_802_993_406_410_049,
        162_259_276_829_213_363_391_578_010_288_127,
        162_259_276_829_213_363_391_578_010_287_957,
        162_259_276_829_213_363_391_578_010_287_051,
        1_298_074_214_633_706_907_132_624_082_304_889,
    ];

    for prime in test_primes.iter() {
        assert_eq!(is_odd_prime(*prime), true, "{}", *prime);
    }
}

#[test]
fn is_prime_larger_primes_other() {
    let test_primes: [u128; 21] = [
        41_538_374_868_278_621_028_243_970_633_760_399,
        41_538_374_868_278_621_028_243_970_633_760_057,
        166_153_499_473_114_484_112_975_882_535_042_517,
        166_153_499_473_114_484_112_975_882_535_042_279,
        332_306_998_946_228_968_225_951_765_070_086_139,
        5_316_911_983_139_663_491_615_228_241_121_378_301,
        5_316_911_983_139_663_491_615_228_241_121_378_191,
        42_535_295_865_117_307_932_921_825_928_971_026_423,
        42_535_295_865_117_307_932_921_825_928_971_026_047,
        42_535_295_865_117_307_932_921_825_928_971_026_027,
        170_141_183_460_469_231_731_687_303_715_884_105_727,
        170_141_183_460_469_231_731_687_303_715_884_105_703,
        170_141_183_460_469_231_731_687_303_715_884_105_689,
        170_141_183_460_469_231_731_687_303_715_884_105_433,
        170_141_183_460_469_231_731_687_303_715_884_105_419,
        170_141_183_460_469_231_731_687_303_715_884_104_993,
        340_282_366_920_938_463_463_374_607_431_768_210_659,
        340_282_366_920_938_463_463_374_607_431_768_211_219,
        340_282_366_920_938_463_463_374_607_431_768_211_223,
        340_282_366_920_938_463_463_374_607_431_768_211_283,
        340_282_366_920_938_463_463_374_607_431_768_211_297,
    ];

    for prime in test_primes.iter() {
        assert_eq!(is_odd_prime(*prime), true, "{}", *prime);
    }
}

#[test]
fn is_prime_large_composites() {
    let test_composites: [u128; 5] = [
        83_076_749_736_557_242_056_487_941_267_521_531,
        332_306_998_946_228_968_225_951_765_070_086_141,
        5_316_911_983_139_663_491_615_228_241_121_378_303,
        170_141_183_460_469_231_731_687_303_715_884_105_723,
        // following is u128::MAX and should not cause a panic with this implementation
        340_282_366_920_938_463_463_374_607_431_768_211_455,
    ];

    for comp in test_composites.iter() {
        assert_eq!(is_odd_prime(*comp), false, "{}", *comp);
    }
}

#[test]
fn is_prime_range_containing_two_primes() {
    let start_num = (i128::MAX - 511) as u128;
    let stop_num = (i128::MAX - 505) as u128;

    let prime_count = (start_num..stop_num).filter(|x| is_odd_prime(*x)).count();

    assert_eq!(prime_count, 2);
}

#[test]
fn is_prime_range_containing_three_primes() {
    let start_num = u128::pow(2, 119) - 801;
    let stop_num = u128::pow(2, 119) - 744;

    let prime_count = (start_num..stop_num).filter(|x| is_odd_prime(*x)).count();

    assert_eq!(prime_count, 3);
}

#[test]
fn is_prime_range_containing_no_primes() {
    let start_num = u128::pow(2, 107) - 170;
    let stop_num = u128::pow(2, 107) - 1;

    // range is exclusive, the stop number is prime but not included here
    let prime_count = (start_num..stop_num).filter(|x| is_odd_prime(*x)).count();

    assert_eq!(prime_count, 0);
}
