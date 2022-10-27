//! Implements basic modular arithmetic operations.
//!
//! It's recommended to use functions of `Arith` trait unless it is
//! guaranteed that the operands for the functions are less than the
//! modulus `modu`, or in other words the operands are the smallest
//! nonnegative representatives of their residue class. Violating this
//! constraint causes two's complement wrapping.
//!
use std::cmp::{self, Ordering};
use std::convert::{From, TryFrom};
use std::mem;

use num::{PrimInt, Signed, Unsigned};

pub trait CoreArith<T: PrimInt + Unsigned> {
    /// Unsafe modular addition, `x` + `y`.
    ///
    /// Two's complement wrapping occurs if the arguments
    /// `x` and `y` are not smaller than `modu`.
    fn add_mod_unsafe(x: T, y: T, modu: T) -> T {
        if x < modu - y {
            x + y
        } else {
            cmp::min(x, y) - (modu - cmp::max(x, y))
        }
    }

    /// Unsafe modular subtraction, `x` - `y`.
    ///
    /// Two's complement wrapping occurs if the arguments
    /// `x` and `y` are not smaller than `modu`.
    fn sub_mod_unsafe(x: T, y: T, modu: T) -> T {
        if x >= y {
            x - y
        } else {
            modu - (y - x)
        }
    }

    /// Unsafe modular multiplication, `x` * `y`.
    ///
    /// Two's complement wrapping occurs if the argument
    /// `x` is not smaller than `modu`.
    fn mult_mod_unsafe(mut x: T, mut y: T, modu: T) -> T {
        let mut res = T::zero();

        while y > T::zero() {
            if y & T::one() == T::one() {
                res = Self::add_mod_unsafe(res, x, modu);
            }

            y = y.unsigned_shr(1);
            x = Self::add_mod_unsafe(x, x, modu);
        }

        res
    }

    /// Unsafe modular exponentation, `base` ^ `ex`.
    ///
    /// Uses directly unsafe modular multiplication.
    fn exp_mod_unsafe(mut base: T, mut ex: T, modu: T) -> T {
        let mut res = T::one();

        while ex > T::zero() {
            if ex & T::one() == T::one() {
                res = Self::mult_mod_unsafe(res, base, modu);
            }

            ex = ex.unsigned_shr(1);
            base = Self::mult_mod_unsafe(base, base, modu);
        }

        res
    }

    /// Unsafe modular exponentation with fixed exponent type.
    ///
    /// Uses directly unsafe modular multiplication.
    fn exp_mod_unsafe_u128(mut base: T, mut ex: u128, modu: T) -> T {
        let mut res = T::one();

        while ex > 0 {
            if ex & 1 == 1 {
                res = Self::mult_mod_unsafe(res, base, modu);
            }

            ex >>= 1;
            base = Self::mult_mod_unsafe(base, base, modu);
        }

        res
    }
}

pub trait Arith<T>: CoreArith<T>
where
    T: PrimInt + Unsigned + From<u8>,
{
    /// Modular addition, `x` + `y`.
    fn add_mod(x: T, y: T, modu: T) -> T {
        if x < modu && y < modu {
            Self::add_mod_unsafe(x, y, modu)
        } else {
            Self::add_mod_unsafe(x % modu, y % modu, modu)
        }
    }

    /// Modular subtraction, `x` - `y`.
    fn sub_mod(x: T, y: T, modu: T) -> T {
        if x < modu && y < modu {
            Self::sub_mod_unsafe(x, y, modu)
        } else {
            Self::sub_mod_unsafe(x % modu, y % modu, modu)
        }
    }

    /// Modular multiplication, `x` * `y`.
    fn mult_mod(x: T, y: T, modu: T) -> T {
        if x < modu && y < modu {
            Self::mult_mod_unsafe(x, y, modu)
        } else {
            Self::mult_mod_unsafe(x % modu, y % modu, modu)
        }
    }

    /// Modular exponentiation, `base` ^ `ex`.
    fn exp_mod(base: T, ex: T, modu: T) -> T {
        if base < modu {
            Self::exp_mod_unsafe(base, ex, modu)
        } else {
            Self::exp_mod_unsafe(base % modu, ex, modu)
        }
    }

    /// Greatest common divisor for `x` and `y`.
    fn gcd_mod(mut x: T, mut y: T) -> T {
        if x == T::zero() || y == T::zero() {
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
            if y == T::zero() {
                break x.unsigned_shl(shift);
            }
        }
    }

    /// Multiplicative inverse of `x`.
    ///
    /// If the inverse `x^(-1)` exists, meaning that
    /// x * x^(-1) = 1 (mod modu) holds, it will be returned.
    /// Otherwise the return value will be zero.
    fn multip_inv(mut x: T, modu: T) -> T {
        if x >= modu {
            x = x % modu;
        }

        let (mut rem, mut rem_new) = (modu, x);
        let (mut inv, mut inv_new) = (T::zero(), T::one());

        while rem_new > T::zero() {
            let quo = rem / rem_new;

            let rem_temp = rem_new;
            rem_new = rem - quo * rem_new;
            rem = rem_temp;

            let inv_temp = inv_new;
            inv_new = Self::sub_mod_unsafe(inv, Self::mult_mod_unsafe(quo, inv_new, modu), modu);
            inv = inv_temp;
        }

        if rem > T::one() {
            // Inverse doesn't exist for x, gcd(x, modu) > 1
            return T::zero();
        }

        inv
    }

    /// Compute value of the Jacobi symbol `(x|n)`.
    ///
    /// Return value will be one of -1, 0 or 1.
    fn jacobi_symbol(mut x: T, mut n: T) -> i8 {
        if x >= n {
            x = x % n;
        }

        let mut par_t = 1;

        while x > T::zero() {
            while x & T::one() == T::zero() {
                x = x.signed_shr(1);

                let par_r = n & 7.into();
                if par_r == 3.into() || par_r == 5.into() {
                    par_t = -par_t;
                }
            }

            mem::swap(&mut x, &mut n);

            if (x & 3.into()) == 3.into() && (n & 3.into()) == 3.into() {
                par_t = -par_t;
            }
            x = x % n;
        }

        if n == T::one() {
            par_t
        } else {
            0
        }
    }

    /// Truncated square, `x * x`, for `x`.
    ///
    /// In other words wrapping isn't allowed and
    /// zero will be returned in that case.
    fn trunc_square(x: T) -> T {
        match x.cmp(&T::zero()) {
            Ordering::Greater => {
                if x < T::max_value() / x {
                    x * x
                } else {
                    T::zero()
                }
            }
            _ => T::zero(),
        }
    }
}

pub trait SignCast<S, T>
where
    S: PrimInt + Signed,
    T: PrimInt + Unsigned + TryFrom<S>,
{
    /// Cast `x` from signed type S to unsigned type T.
    ///
    /// This can be done since the `x` can always be given
    /// as the smallest nonnegative representative of its
    /// residue class. However, notice that if after the sign
    /// cast `x` equals to or is larger than `modu`, it is
    /// returned as `x` without taking modulo with `modu`.
    ///
    /// Cast fails if abs(`x`) cannot be computed.
    fn cast_to_unsigned(x: S, modu: T) -> Option<T> {
        if x >= S::zero() {
            return match T::try_from(x) {
                Ok(x) => Some(x),
                Err(_) => None,
            };
        }
        if x == S::min_value() {
            // No abs value
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
