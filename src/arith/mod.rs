//!
//!
//!
use std::{cmp, mem};

use num::{PrimInt, Unsigned};

pub trait CoreArith<T: PrimInt + Unsigned> {
    fn add_unsafe(x: T, y: T, modu: T) -> T {
        if x < modu - y {
            x + y
        } else {
            cmp::min(x, y) - (modu - cmp::max(x, y))
        }
    }

    fn sub_unsafe(x: T, y: T, modu: T) -> T {
        if x >= y {
            x - y
        } else {
            modu - (y - x)
        }
    }

    fn mult_unsafe(mut x: T, mut y: T, modu: T) -> T {
        let zero = T::zero();
        let one = T::one();

        if x == zero || y == zero {
            return zero;
        }

        let mut res = zero;

        while y > zero {
            if y & one == one {
                res = Self::add_unsafe(res, x, modu);
            }
            y = y.unsigned_shr(1);
            x = Self::add_unsafe(x, x, modu);
        }

        res
    }

    fn exp_unsafe(mut base: T, mut ex: T, modu: T) -> T {
        let zero = T::zero();
        let one = T::one();

        if base == zero {
            return zero;
        }

        let mut res = one;

        while ex > zero {
            if ex & one == one {
                res = Self::mult_unsafe(res, base, modu);
            }
            ex = ex.unsigned_shr(1);
            base = Self::mult_unsafe(base, base, modu);
        }

        res
    }
}

pub trait Arith<T: PrimInt + Unsigned>: CoreArith<T> {
    fn add(x: T, y: T, modu: T) -> T {
        if x < modu && y < modu {
            Self::add_unsafe(x, y, modu)
        } else {
            Self::add_unsafe(x % modu, y % modu, modu)
        }
    }

    fn sub(x: T, y: T, modu: T) -> T {
        if x < modu && y < modu {
            Self::sub_unsafe(x, y, modu)
        } else {
            Self::sub_unsafe(x % modu, y % modu, modu)
        }
    }

    fn mult(x: T, y: T, modu: T) -> T {
        if x < modu && y < modu {
            Self::mult_unsafe(x, y, modu)
        } else {
            Self::mult_unsafe(x % modu, y % modu, modu)
        }
    }

    fn exp(base: T, ex: T, modu: T) -> T {
        if base < modu {
            Self::exp_unsafe(base, ex, modu)
        } else {
            Self::exp_unsafe(base % modu, ex, modu)
        }
    }

    fn gcd(mut x: T, mut y: T) -> T {
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
            inv_new = Self::sub_unsafe(inv, Self::mult_unsafe(quo, inv_new, modu), modu);
            inv = inv_temp;
        }

        if rem > one {
            // inverse doesn't exist, gcd(x, modu) > 1
            return zero;
        }

        inv
    }
}

#[cfg(test)]
mod tests;
