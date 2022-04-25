//! Defines basic modular arithmetic operations.
//!
//! Use functions under `Arith` trait unless it's guaranteed that the operands
//! are less than the modulus `modu`, or in other words the operands are the
//! smallest nonnegative representatives of their residue class.
//!
use std::convert::{From, TryFrom};
use std::{cmp, mem};

use num::{PrimInt, Signed, Unsigned};

pub trait CoreArith<T: PrimInt + Unsigned> {
    fn add_mod_unsafe(x: T, y: T, modu: T) -> T {
        if x < modu - y {
            x + y
        } else {
            cmp::min(x, y) - (modu - cmp::max(x, y))
        }
    }

    fn sub_mod_unsafe(x: T, y: T, modu: T) -> T {
        if x >= y {
            x - y
        } else {
            modu - (y - x)
        }
    }

    fn mult_mod_unsafe(mut x: T, mut y: T, modu: T) -> T {
        let zero = T::zero();
        let one = T::one();

        if x == zero || y == zero {
            return zero;
        }

        let mut res = zero;

        while y > zero {
            if y & one == one {
                res = Self::add_mod_unsafe(res, x, modu);
            }
            y = y.unsigned_shr(1);
            x = Self::add_mod_unsafe(x, x, modu);
        }

        res
    }

    fn exp_mod_unsafe(mut base: T, mut ex: T, modu: T) -> T {
        let zero = T::zero();
        let one = T::one();

        if base == zero {
            return zero;
        }

        let mut res = one;

        while ex > zero {
            if ex & one == one {
                res = Self::mult_mod_unsafe(res, base, modu);
            }
            ex = ex.unsigned_shr(1);
            base = Self::mult_mod_unsafe(base, base, modu);
        }

        res
    }
}

pub trait Arith<T: PrimInt + Unsigned + From<u8>>: CoreArith<T> {
    fn add_mod(x: T, y: T, modu: T) -> T {
        if x < modu && y < modu {
            Self::add_mod_unsafe(x, y, modu)
        } else {
            Self::add_mod_unsafe(x % modu, y % modu, modu)
        }
    }

    fn sub_mod(x: T, y: T, modu: T) -> T {
        if x < modu && y < modu {
            Self::sub_mod_unsafe(x, y, modu)
        } else {
            Self::sub_mod_unsafe(x % modu, y % modu, modu)
        }
    }

    fn mult_mod(x: T, y: T, modu: T) -> T {
        if x < modu && y < modu {
            Self::mult_mod_unsafe(x, y, modu)
        } else {
            Self::mult_mod_unsafe(x % modu, y % modu, modu)
        }
    }

    fn exp_mod(base: T, ex: T, modu: T) -> T {
        if base < modu {
            Self::exp_mod_unsafe(base, ex, modu)
        } else {
            Self::exp_mod_unsafe(base % modu, ex, modu)
        }
    }

    fn gcd_mod(mut x: T, mut y: T) -> T {
        let zero = T::zero();

        if x == zero || y == zero {
            return x | y;
        }

        let shift = (x | y).trailing_zeros();
        x = x.unsigned_shr(x.trailing_zeros());

        loop {
            y = y.unsigned_shr(y.trailing_zeros());
            if x > y {
                mem::swap(&mut x, &mut y);
            }
            y = y - x;
            if y == zero {
                break x.unsigned_shl(shift);
            }
        }
    }

    fn multip_inv(mut x: T, modu: T) -> T {
        let zero = T::zero();
        let one = T::one();

        if x >= modu {
            x = x % modu;
        }

        let (mut rem, mut rem_new) = (modu, x);
        let (mut inv, mut inv_new) = (zero, one);

        while rem_new > zero {
            let quo = rem / rem_new;

            let rem_temp = rem_new;
            rem_new = rem - quo * rem_new;
            rem = rem_temp;

            let inv_temp = inv_new;
            inv_new = Self::sub_mod_unsafe(inv, Self::mult_mod_unsafe(quo, inv_new, modu), modu);
            inv = inv_temp;
        }

        if rem > one {
            // inverse doesn't exist, gcd(x, modu) > 1
            return zero;
        }

        inv
    }

    fn jacobi_symbol(mut x: T, mut n: T) -> i8 {
        if x >= n {
            x = x % n;
        }

        let (zero, one) = (T::zero(), T::one());
        let (three, five, seven) = (3.into(), 5.into(), 7.into());

        let mut param_t = 1;

        while x > zero {
            while x & one == zero {
                x = x.signed_shr(1);

                let param_r = n & seven;
                if param_r == three || param_r == five {
                    param_t = -param_t;
                }
            }

            mem::swap(&mut x, &mut n);

            if (x & three) == three && (n & three) == three {
                param_t = -param_t;
            }
            x = x % n;
        }

        if n == one {
            param_t
        } else {
            0
        }
    }
}

pub trait SignCast<S, T>
where
    S: PrimInt + Signed,
    T: PrimInt + Unsigned + TryFrom<S>,
{
    fn cast_to_unsigned(x: S, modu: T) -> Option<T> {
        if x > S::zero() {
            return match T::try_from(x) {
                Ok(x) => Some(x % modu),
                Err(_) => None,
            };
        }
        if x == S::min_value() {
            // no abs value
            return None;
        }

        let x_abs = match T::try_from(x.abs()) {
            Ok(x) => x,
            Err(_) => return None,
        };

        if x_abs <= modu {
            return Some(modu - x_abs);
        }

        let mut k = x_abs / modu;

        if x_abs % modu > T::zero() {
            k = k + T::one();
        }

        Some(k * modu - x_abs)
    }
}

#[cfg(test)]
mod tests;
