//!
//!
//!
use crate::{
    arith::{Arith, SignCast},
    Int, UInt,
};
use num::iter;

pub struct LinEq<T: UInt> {
    pub a: T,
    pub b: T,
    pub c: T,
    pub modu: T,
}

pub struct LinEqSigned<S: Int, T: UInt> {
    pub a: S,
    pub b: S,
    pub c: S,
    pub modu: T,
}

impl<T: UInt> LinEq<T> {
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
    fn solve(&self) -> Option<Vec<T>> {
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
