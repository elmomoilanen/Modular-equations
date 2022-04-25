//!
//!
//!
use std::cmp::Ordering;
use std::convert::{Into, TryInto};

use num::{integer, PrimInt};

use crate::{arith::Arith, UInt};

struct LucasParams<T: UInt>(T, T, T);

pub fn is_odd_prime<T: UInt>(num: T) -> bool {
    let (zero, one) = (T::zero(), T::one());

    if num <= one || num & one == zero {
        return false;
    }

    let mr_limit = 67.into();
    let small_prime = is_sure_odd_small_prime(num);

    if small_prime || num < mr_limit {
        return small_prime;
    }

    let num_u128: u128 = num.into();

    if num_u128 > u64::MAX as u128 {
        is_prime_strong_bpsw(num_u128)
    } else if num_u128 > u32::MAX as u128 {
        let mr_base_large: [u64; 7] = [2, 325, 9375, 28_178, 450_775, 9_780_504, 1_795_265_022];
        is_prime_mr(num_u128.try_into().unwrap(), &mr_base_large[..])
    } else {
        let mr_base_small: [u32; 3] = [2, 7, 61];
        is_prime_mr(num_u128.try_into().unwrap(), &mr_base_small[..])
    }
}

fn is_sure_odd_small_prime<T: UInt>(num: T) -> bool {
    static PRIMES: [u8; 17] = [
        3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61,
    ];

    for prime in PRIMES.iter() {
        let prm = (*prime).into();

        if prm > num / prm {
            return true;
        }
        if num % prm == T::zero() {
            return false;
        }
    }

    false
}

fn is_prime_mr<T: UInt>(num: T, bases: &[T]) -> bool {
    let one = T::one();
    let num_even = num - one;

    let pow = num_even.trailing_zeros();
    let num_odd = num_even.unsigned_shr(pow);
    // num_even = 2^pow * num_odd

    for base in bases.iter() {
        let mut q = T::exp_mod(*base, num_odd, num);

        if q == one || q == num_even {
            continue;
        }

        let mut jump = false;

        for _ in 1..pow {
            q = T::mult_mod(q, q, num);

            if q == num_even {
                jump = true;
                break;
            }
        }

        if jump {
            continue;
        }

        return false;
    }

    true
}

fn is_prime_strong_bpsw(num: u128) -> bool {
    let mr_test_base: [u128; 1] = [2];

    if !is_prime_mr(num, &mr_test_base[..]) {
        return false;
    }

    if num == i128::MAX as u128 {
        return true;
    }

    match select_lucas_params(num) {
        Some(params) => pass_strong_lucas_test(num, params),
        None => false,
    }
}

fn select_lucas_params(num: u128) -> Option<LucasParams<u128>> {
    let d_seq = (5..).step_by(2).enumerate();

    for (i, mut d) in d_seq {
        let d_orig = d;

        if i & 1 == 1 {
            d = num - d % num;
        }

        let jac_sym = u128::jacobi_symbol(d, num);

        if jac_sym == -1 {
            let (p, q) = if i & 1 == 1 {
                (1, (1 + d_orig) >> 2)
            } else if d == 5 {
                (5, 5)
            } else {
                let q_temp = (d_orig - 1) >> 2;
                (1, num - q_temp % num)
            };
            return Some(LucasParams(d, p, q));
        }

        if jac_sym == 0 && (d_orig < num || d_orig % num != 0) {
            return None;
        }

        if i == 10 {
            let num_sqrt = integer::sqrt(num);
            if num_sqrt * num_sqrt == num {
                return None;
            }
        }
    }

    None
}

fn pass_strong_lucas_test(num: u128, params: LucasParams<u128>) -> bool {
    let num_even = num + 1; // cannot be done with u128::MAX
    let num_odd = num_even.unsigned_shr(num_even.trailing_zeros());
    // num_even = 2^pow * num_odd, for pow == num_even.trailing_zeros()

    let num_even_lead_zeros = num_even.leading_zeros();

    let bits_to_check = u128::BITS - num_even_lead_zeros;
    let num_even_rev = num_even.reverse_bits() >> num_even_lead_zeros;

    let LucasParams(_, _, luc_q) = params;
    let (mut luc_u, mut luc_v, mut luc_w) = (0, 2, 1);

    let (mut round, euler_check_round) = (0, num_even >> 1);
    let (mut is_slprp, mut pass_euler_crit) = (false, false);

    for bit in 0..bits_to_check {
        if bit > 0 {
            update_lucas_normal_uvq(num, &mut luc_u, &mut luc_v, &mut luc_w);
            round *= 2;
        }

        if !is_slprp && luc_v == 0 && round > num_odd && bit < bits_to_check - 1 {
            is_slprp = true;
        }

        if (num_even_rev >> bit) & 1 == 1 {
            update_lucas_odd_bit_uvq(num, &params, &mut luc_u, &mut luc_v, &mut luc_w);
            round += 1;
        }

        if round == num_odd && (luc_u == 0 || luc_v == 0) {
            is_slprp = true;
        }

        if round == euler_check_round {
            let luc_q_jac: u128 = match u128::jacobi_symbol(luc_q, num).cmp(&0) {
                Ordering::Equal => 0,
                Ordering::Greater => num - luc_q % num,
                Ordering::Less => luc_q,
            };

            if u128::add_mod(luc_w, luc_q_jac, num) == 0 {
                pass_euler_crit = true;
            }
        }
    }

    if luc_u != 0 || !is_slprp || !pass_euler_crit {
        return false;
    }
    if u128::mult_mod(2, luc_q, num) != luc_v % num {
        return false;
    }

    true
}

fn update_lucas_normal_uvq(num: u128, u: &mut u128, v: &mut u128, w: &mut u128) {
    *u = u128::mult_mod(*u, *v, num);

    *v = u128::add_mod(
        u128::mult_mod(*v, *v, num),
        u128::mult_mod(num - 2, *w, num),
        num,
    );

    *w = u128::mult_mod(*w, *w, num);
}

fn modify_lucas_coef(x_left: u128, x_right: u128, num: u128) -> u128 {
    let numer = u128::add_mod(x_left, x_right, num);

    if numer & 1 == 1 {
        // decompose both odds to 2k + 1, and compute k1 + k2 + 1 (mod num)
        u128::add_mod((numer - 1) >> 1, ((num - 1) >> 1) + 1, num)
    } else {
        numer >> 1
    }
}

fn update_lucas_odd_bit_uvq(
    num: u128,
    params: &LucasParams<u128>,
    u: &mut u128,
    v: &mut u128,
    w: &mut u128,
) {
    let LucasParams(d, p, q) = *params;

    let new_u = modify_lucas_coef(u128::mult_mod(p, *u, num), *v, num);

    let new_v = modify_lucas_coef(u128::mult_mod(d, *u, num), u128::mult_mod(p, *v, num), num);

    *u = new_u;
    *v = new_v;
    *w = u128::mult_mod(q, *w, num);
}

#[cfg(test)]
mod tests;
