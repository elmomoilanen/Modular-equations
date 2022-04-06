//!
//!
//!
use std::convert::{From, Into};
use std::fmt::{Debug, Display};

use num::{PrimInt, Signed, Unsigned};

mod arith;
mod lin;
mod prime;
mod quad;

///
pub trait UInt: PrimInt + Unsigned + Display + Debug + From<u8> + Into<u128> {}

impl<T> UInt for T where T: PrimInt + Unsigned + Display + Debug + From<u8> + Into<u128> {}

impl<T> arith::CoreArith<T> for T where T: UInt {}
impl<T> arith::Arith<T> for T where T: UInt {}

///
pub trait Int: PrimInt + Signed + Display + Debug + From<i8> + Into<i128> {}

impl<S> Int for S where S: PrimInt + Signed + Display + Debug + From<i8> + Into<i128> {}

impl arith::SignCast<i8, u8> for i8 {}
impl arith::SignCast<i16, u16> for i16 {}
impl arith::SignCast<i32, u32> for i32 {}
impl arith::SignCast<i64, u64> for i64 {}
impl arith::SignCast<i128, u128> for i128 {}
impl arith::SignCast<isize, usize> for isize {}

pub use lin::{LinEq, LinEqSigned};
