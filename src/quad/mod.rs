//! Implements a solver for quadratic modular equations.
//!
//! Modular quadratic equations are of the form ax^2 + bx + c = d (mod n) where
//! every term or element is a residue class [*] belonging to the ring of integers
//! Z/nZ. Modulo term `n` must be a positive integer and strictly larger than one.
//!
use crate::{
    arith::{Arith, CoreArith, SignCast},
    lin::LinEq,
    prime, Int, UInt,
};

use num::iter;

/// Type for quadratic equations with only unsigned terms.
///
/// Quadratic modular equations are of the form ax^2 + bx + c = d (mod n) where
/// terms `a`, `b`, `c` and `d` must be nonnegative for this type. Also modulo `n`
/// must be the same unsigned type and strictly larger than one. Solve method
/// of this type will panic if the modulo `n` doesn't satisfy this requirement.
pub struct QuadEq<T: UInt> {
    pub a: T,
    pub b: T,
    pub c: T,
    pub d: T,
    pub modu: T,
}

/// Type for quadratic equations with unsigned modulo and signed other terms.
///
/// Quadratic modular equations are of the form ax^2 + bx + c = d (mod n) where
/// terms `a`, `b`, `c` and `d` are signed for this type. Modulo `n` must be
/// an unsigned type but compatible to the signed type, e.g. u32 if signed type
/// is i32, and strictly larger than one as its value. Solve method of this type
/// will panic if the modulo `n` doesn't satisfy this requirement.
pub struct QuadEqSigned<S: Int, T: UInt> {
    pub a: S,
    pub b: S,
    pub c: S,
    pub d: S,
    pub modu: T,
}

impl<T: UInt> QuadEq<T> {
    pub fn solve(&self) -> Option<Vec<T>> {
        if self.modu <= T::one() {
            return None;
        }

        let zero = T::zero();
        let mut quad = QuadEq { ..*self };

        if quad.c > zero {
            quad.d = T::sub_mod(quad.d, quad.c, quad.modu);
            quad.c = T::zero();
        }

        match prime::is_odd_prime(quad.modu) {
            true if quad.a == T::one() && quad.b == zero => quad.solve_quad_residue_odd_prime_mod(),
            true => {
                // modify to (2ax + b)^2 = b^2 + 4ad' (mod modu), d' = d - c
                let b2 = T::mult_mod(quad.b, quad.b, quad.modu);
                let ad = T::mult_mod(4.into(), T::mult_mod(quad.a, quad.d, quad.modu), quad.modu);

                quad.d = T::add_mod_unsafe(b2, ad, quad.modu);
                quad.solve_quad_simple()
            }
            _ => None,
        }
    }

    /// Solve equation (2ax + b)^2 = d (mod modu).
    /// First, solve z^2 = d (mod modu), and then 2ax + b = z (mod modu).
    fn solve_quad_simple(&self) -> Option<Vec<T>> {
        let z = match self.solve_quad_residue_odd_prime_mod() {
            Some(z) if !z.is_empty() => z,
            _ => return None,
        };

        let mut lin_eq = LinEq {
            a: T::mult_mod(2.into(), self.a, self.modu),
            b: self.b,
            c: z[0],
            modu: self.modu,
        };

        let mut x_sols = match lin_eq.solve() {
            Some(x_sols) => x_sols,
            _ => return None,
        };

        if z[0] == T::zero() || z.len() == 1 {
            // z^2 = d (mod modu) has only one root
            return Some(x_sols);
        }

        lin_eq.c = z[1];

        let mut x_sols_2 = match lin_eq.solve() {
            Some(x_sols) => x_sols,
            _ => {
                // shouldn't go here as the first linear eq had solutions
                return Some(x_sols);
            }
        };

        x_sols.append(&mut x_sols_2);
        x_sols.sort();

        Some(x_sols)
    }

    /// Solve equation x^2 = d (mod modu), where modu is an odd prime.
    /// There will be 0-2 roots for the equation.
    fn solve_quad_residue_odd_prime_mod(&self) -> Option<Vec<T>> {
        if self.d == T::zero() {
            return Some(vec![self.d]);
        }

        if T::exp_mod(self.d, (self.modu - T::one()) / 2.into(), self.modu) != T::one() {
            // doesn't satisfy Euler's criterion
            return None;
        }

        match QuadEq::tonelli_shanks(self.d, self.modu) {
            None => None,
            Some(x) if x == T::zero() => Some(vec![x]),
            Some(x) => {
                let mut x_sols = vec![x, T::sub_mod_unsafe(T::zero(), x, self.modu)];
                x_sols.sort();

                Some(x_sols)
            }
        }
    }

    fn tonelli_shanks(q: T, modu: T) -> Option<T> {
        let modu_half = (modu - T::one()) / 2.into();

        let non_resid = match iter::range(2.into(), modu)
            .find(|&b| T::exp_mod_unsafe(b, modu_half, modu) != T::one())
        {
            Some(non_residue) => non_residue,
            None => return None,
        };

        let modu_ev = modu - T::one();
        let pow = modu_ev.trailing_zeros();
        let modu_odd = modu_ev.unsigned_shr(pow);

        let mut par_c = T::exp_mod_unsafe(non_resid, modu_odd, modu);
        let mut par_t = T::exp_mod(q, modu_odd, modu);
        let mut res = T::exp_mod(q, (modu_odd + T::one()) / 2.into(), modu);

        // pow < 128 => m < 128
        let modu_u128: u128 = modu.into();
        let mut m = (pow as u128) % modu_u128;

        loop {
            if par_t == T::zero() {
                break Some(par_t);
            }
            if par_t == T::one() {
                break Some(res);
            }

            let (mut least_i, mut pow_i) = (0, 1);

            while pow_i < m {
                let ex = (1 << pow_i) % modu_u128;
                if T::exp_mod_unsafe_u128(par_t, ex, modu) == T::one() {
                    least_i = pow_i;
                    break;
                }
                pow_i += 1;
            }

            if least_i == 0 {
                // q isn't quadratic residue
                break None;
            }

            let ex = (1 << (m - least_i - 1)) % modu_u128;
            let par_b = T::exp_mod_unsafe_u128(par_c, ex, modu);

            m = least_i;
            par_c = T::mult_mod_unsafe(par_b, par_b, modu);
            par_t = T::mult_mod_unsafe(par_t, par_c, modu);
            res = T::mult_mod_unsafe(res, par_b, modu);
        }
    }
}

impl<T, S> QuadEqSigned<S, T>
where
    S: Int + SignCast<S, T>,
    T: UInt + TryFrom<S>,
{
    pub fn solve(&self) -> Option<Vec<T>> {
        let a_us = match S::cast_to_unsigned(self.a, self.modu) {
            Some(a) => a,
            None => panic!("arg `a` cannot be casted to unsigned."),
        };

        let b_us = match S::cast_to_unsigned(self.b, self.modu) {
            Some(b) => b,
            None => panic!("arg `b` cannot be casted to unsigned."),
        };

        let c_us = match S::cast_to_unsigned(self.c, self.modu) {
            Some(c) => c,
            None => panic!("arg `c` cannot be casted to unsigned."),
        };

        let d_us = match S::cast_to_unsigned(self.d, self.modu) {
            Some(d) => d,
            None => panic!("arg `d` cannot be casted to unsigned."),
        };

        let quad_eq = QuadEq {
            a: a_us,
            b: b_us,
            c: c_us,
            d: d_us,
            modu: self.modu,
        };

        quad_eq.solve()
    }
}

#[cfg(test)]
mod tests;
