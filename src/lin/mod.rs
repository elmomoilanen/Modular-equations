//! Implements a solver for linear modular equations.
//!
//! Modular linear equations are of the form ax + b = c (mod n) where
//! every coefficient or term is a residue class \[*\] belonging to
//! the ring of integers Z/nZ. Modulo n must be a positive integer and
//! strictly larger than one.
//!
//! Solutions x, if any, are given as residue classes \[x\] such that
//! each class is represented by smallest nonnegative integer (modulo n).
//!
use crate::{
    arith::{Arith, SignCast},
    Int, UInt,
};
use num::iter;

/// Type for linear equations with unsigned terms only.
///
/// Linear modular equations are of the form ax + b = c (mod modu) where
/// coefficients `a`, `b` and `c` must be nonnegative for this type. Also
/// `modu` must be the same unsigned type and strictly larger than one.

#[derive(Debug)]
pub struct LinEq<T: UInt> {
    pub a: T,
    pub b: T,
    pub c: T,
    pub modu: T,
}

/// Type for linear equations with unsigned modulo and signed other coefficients.
///
/// Linear modular equations are of the form ax + b = c (mod modu) where
/// coefficients `a`, `b` and `c` are signed for this type. Modulo `modu`
/// must be an unsigned type but compatible to the signed type (same byte count),
/// e.g. u32 if the signed type is i32, and strictly larger than one as its value.

#[derive(Debug)]
pub struct LinEqSigned<S: Int, T: UInt> {
    pub a: S,
    pub b: S,
    pub c: S,
    pub modu: T,
}

impl<T: UInt> LinEq<T> {
    /// Solve linear modular equation ax + b = c (mod modu).
    ///
    /// There will be 0 to N solutions x, 0 case occurring when gcd(a, modu) doesn't
    /// divide the c coefficient and on the contrary, magnitude of N depending on the
    /// equation. If gcd(a, modu) == 1, there will be a unique solution.
    ///
    /// If a % modu == 0 (0 is the smallest nonnegative representative of \[a\]),
    /// there are no solutions since the variable x vanishes from the equation.
    pub fn solve(&self) -> Option<Vec<T>> {
        if self.modu <= T::one() || self.a % self.modu == T::zero() {
            return None;
        }

        let c = if self.b > T::zero() {
            T::sub_mod(self.c, self.b, self.modu)
        } else {
            self.c
        };

        let gcd_am = T::gcd_mod(self.a, self.modu);

        if c % gcd_am > T::zero() {
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
    /// Solve linear modular equation for signed type terms.
    ///
    /// This method will try to cast the signed coefficients to unsigned type
    /// such that after the cast they will represent the smallest nonnegative
    /// integers of their corresponding residue classes (modulo modu). If some
    /// of the casts fails, this method will return None. This should only occur
    /// for S::min_value() value of the signed type S.
    ///
    /// After the cast to unsigned, the `solve` method of struct `LinEq` will
    /// be called to solve the equation.
    pub fn solve(&self) -> Option<Vec<T>> {
        let a_us = match S::cast_to_unsigned(self.a, self.modu) {
            Some(a) => a,
            None => {
                // Arg `a` cannot be casted to unsigned. Is it S::min_value()?
                return None;
            }
        };

        let b_us = match S::cast_to_unsigned(self.b, self.modu) {
            Some(b) => b,
            None => {
                // Arg `b` cannot be casted to unsigned. Is it S::min_value()?
                return None;
            }
        };

        let c_us = match S::cast_to_unsigned(self.c, self.modu) {
            Some(c) => c,
            None => {
                // Arg `c` cannot be casted to unsigned. Is it S::min_value()?
                return None;
            }
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
