//! Implements a solver for quadratic modular equations.
//!
//! Modular quadratic equations are of the form ax^2 + bx + c = d (mod n) where
//! every term or element is a residue class [*] belonging to the ring of integers
//! Z/nZ. Modulo term `n` must be a positive integer and strictly larger than one.
//!
use crate::{
    arith::{Arith, SignCast},
    prime, Int, UInt,
};

/// A type for quadratic equations with all terms being unsigned.
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

/// A type for quadratic equations with all terms except modulo being signed.
///
/// Quadratic modular equations are of the form ax^2 + bx + c = d (mod n) where
/// terms `a`, `b`, `c` and `d` are signed for this type. Modulo `n` must be
/// an unsigned type, compatible to the signed type, e.g. u32 if signed type
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
            // smallest accepted modulo equals two
            return None;
        }

        let mut quad = QuadEq { ..*self };

        if quad.c > T::zero() {
            quad.d = T::sub_mod(quad.d, quad.c, quad.modu);
        }

        if prime::is_odd_prime(quad.modu) {
            // ax^2 + bx + c = d (mod n) => (2ax + b)^2 = b^2 + 4ad' (mod n), d' = d - c
            let b_sqr = T::mult_mod(quad.b, quad.b, quad.modu);
            let four_ad = T::mult_mod(4.into(), T::mult_mod(quad.a, quad.d, quad.modu), quad.modu);

            let rhs = T::add_mod(b_sqr, four_ad, quad.modu);

            return quad.quad_residue();
        }

        None
    }

    fn quad_residue(&self) -> Option<Vec<T>> {
        None
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
