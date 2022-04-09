//! Implements a solver for linear modular equations.
//!
//! Modular linear equations are of the form ax + b = c (mod n) where
//! every term or element is a residue class [.] belonging to the ring
//! Z/nZ. Modulo term `n` must be a positive integer and strictly larger
//! than one.
//!
use crate::{
    arith::{Arith, SignCast},
    Int, UInt,
};
use num::iter;

/// A type for linear equations with all coefficients being unsigned.
///
/// Linear modular equations are of the form ax + b = c (mod n) where
/// coefs `a`, `b` and `c` must be nonnegative for this type. Modulo `n`
/// must be the same unsigned type and strictly larger than one. Solve
/// method of this type will panic if the modulo `n` doesn't satisfy
/// this requirement.
pub struct LinEq<T: UInt> {
    pub a: T,
    pub b: T,
    pub c: T,
    pub modu: T,
}

/// A type for linear equations with all coefficients being signed.
///
/// Linear modular equations are of the form ax + b = c (mod n) where
/// coefs `a`, `b` and `c` are signed for this type. Modulo `n` must be
/// an unsigned type, compatible to the signed type (e.g. u32 is for i32)
/// and strictly larger than one. Solve method of this type will panic
/// if the modulo `n` doesn't satisfy this requirement.
pub struct LinEqSigned<S: Int, T: UInt> {
    pub a: S,
    pub b: S,
    pub c: S,
    pub modu: T,
}

impl<T: UInt> LinEq<T> {
    pub fn solve(&self) -> Option<Vec<T>> {
        if self.modu <= T::one() {
            // smallest accepted modulus is two
            return None;
        }

        let c = if self.b > T::zero() {
            T::sub_mod(self.c, self.b, self.modu)
        } else {
            self.c
        };

        let gcd_am = T::gcd_mod(self.a, self.modu);

        if c % gcd_am > T::zero() {
            // gcd(a, modu) doesn't divide c, no solution
            return None;
        }

        if gcd_am == T::one() {
            Some(vec![LinEq::solve_unique(self.a, c, self.modu)])
        } else {
            let new_modu = self.modu / gcd_am;
            let base_sol = LinEq::solve_unique(self.a / gcd_am, c / gcd_am, new_modu);

            Some(iter::range_step(base_sol, self.modu, new_modu).collect())
        }
    }

    fn solve_unique(a: T, c: T, modu: T) -> T {
        T::mult_mod(T::multip_inv(a, modu), c, modu)
    }
}

impl<T, S> LinEqSigned<S, T>
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

        let lin_eq = LinEq {
            a: a_us,
            b: b_us,
            c: c_us,
            modu: self.modu,
        };

        lin_eq.solve()
    }
}

#[cfg(test)]
mod tests;
