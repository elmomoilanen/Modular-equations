//! Implements a solver for linear modular equations.
//!
//! Modular linear equations are of the form ax + b = c (mod n) where
//! every term or element is a residue class \[*\] belonging to the ring
//! of integers Z/nZ. Modulo term `n` must be a positive integer
//! and strictly larger than one.
//!
use crate::{
    arith::{Arith, SignCast},
    Int, UInt,
};
use num::iter;

/// Type for linear equations with only unsigned terms.
///
/// Linear modular equations are of the form ax + b = c (mod n) where
/// terms `a`, `b` and `c` must be nonnegative for this type. Also modulo `n`
/// must be the same unsigned type and strictly larger than one. Solve method
/// of this type will panic if the modulo `n` doesn't satisfy this requirement.
pub struct LinEq<T: UInt> {
    pub a: T,
    pub b: T,
    pub c: T,
    pub modu: T,
}

/// Type for linear equations with unsigned modulo and signed other terms.
///
/// Linear modular equations are of the form ax + b = c (mod n) where
/// terms `a`, `b` and `c` are signed for this type. Modulo `n` must be
/// an unsigned type but compatible to the signed type, e.g. u32 if signed type
/// is i32, and strictly larger than one as its value. Solve method of this type
/// will panic if the modulo `n` doesn't satisfy this requirement.
pub struct LinEqSigned<S: Int, T: UInt> {
    pub a: S,
    pub b: S,
    pub c: S,
    pub modu: T,
}

impl<T: UInt> LinEq<T> {
    /// Solve linear modular equation ax + b = c (mod modu).
    ///
    /// There will be 0-N solutions x, 0 case occurring when gcd(a, modu) doesn't divide
    /// the c parameter and on the contrary, magnitude of N depending on the equation.
    /// If gcd(a, modu) == 1, there will be a unique solution.
    pub fn solve(&self) -> Option<Vec<T>> {
        if self.modu <= T::one() {
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
    /// This method will try to cast the signed terms to unsigned such that
    /// after the cast they will represent the smallest nonnegative
    /// integers of their corresponding residue classes (modulo modu). If some
    /// of the casts fails, this method will panic but this should only occur
    /// for S::min_value() value of the signed type S.
    ///
    /// After the cast to unsigned, the `solve` method of struct `LinEq` will
    /// be called to solve the equation.
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
