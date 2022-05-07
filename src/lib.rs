//! Library implementing solvers for linear and quadratic modular equations.
//!
//! As it's described in Wikipedia, modular arithmetic is a system of arithmetic
//! where elements of the system (i.e., integers) wrap around after reaching
//! a specific boundary value called modulus.
//!
//! Before giving a concrete definition for such arithmetic system the following congruence
//! relation needs to be introduced: Given a positive integer M > 1, integer x is said
//! to be congruent to integer y, if M divides their difference (written mathematically
//! as M | (x - y)). In this case integers x and y are in a relation which is denoted
//! as x ≡ y (mod M) and importantly this relation is an equivalence relation.
//!
//! Finally, the modular arithmetic system is constructed such that the elements of it
//! are so called residue or congruence classes \[x\], where one class \[x\] is consisting
//! of all the integers congruent to x modulo M, or in other words all integers of the form
//! {..., x - M, x, x + M, ...} = {x + k * M}, k being a integer belonging to Z. Hence,
//! in principle, all of these integers {x + k * M} are valid representatives of their
//! residue class \[x\] but the common way is to use the smallest nonnegative integer (modulo M)
//! to represent the residue class. As the congruence relation is an equivalence relation,
//! every integer can belong to only one residue class modulo M.
//!
//! When listing all possible residue classes modulo M, a set of classes {\[0\], \[1\], ..., \[M - 1\]}
//! more precisely, and equipping this set with addition and multiplication operations (operations
//! are basically functions), the ring of integers modulo M (residue classes) is formed.
//! This ring is commonly denoted as Z/nZ where n is the modulo. Mentioned binary operations
//! are well-defined due to the fact that the congruence relation is an equivalence relation. If the
//! modulo is a prime number, the ring becomes actually a field. These fields are more easier
//! to work with since every nonzero element has a multiplicative inverse.
//!
//! Now in the context of rings and fields, it's meaningful to speak of equations and this library
//! implements solvers for linear and quadratic equations. Next follows few examples of linear
//! equations of the form ax + b = c (mod M).
//!
//! ```
//! use modular_arithmetic::LinEq;
//!
//! let lin_eq = LinEq::<u32> {
//!     a: 13,
//!     b: 17,
//!     c: 5,
//!     modu: 29,
//! };
//! let sol = lin_eq.solve();
//!
//! // residue class [8] is the correct solution (smallest nonnegative member)
//! assert!(sol.is_some() && sol.unwrap()[0] == 8);
//! ```
//!
//! Following linear equation doesn't have solution
//!
//! ```
//! use modular_arithmetic::LinEqSigned;
//!
//! let lin_eq = LinEqSigned::<i8, u8> {
//!     a: -3,
//!     b: -1,
//!     c: 3,
//!     modu: 9
//! };
//!
//! assert_eq!(lin_eq.solve(), None);
//! ```
//!
//! If any of the coefficients is signed, one must use the signed type equation
//! `LinEqSigned`. Notice however that in this case, the modulo must still be unsigned.
//! Every negative integer in the ring can be turned to the smallest nonnegative
//! representative of the corresponding residue class \[x\]. With respect to this there
//! are few technical restrictions, the first being that the used signed type (e.g. i32)
//! must have the arith::SignCast trait implemented and that trait requires the signed
//! and unsigned types to be compatible (i.e., same size e.g. i32 and u32). In addition,
//! as the smallest negative integer of each type doesn't have an absolute value in
//! two's complement, they will trigger panic if used as coefficients in equations.
//!
use std::convert::{From, Into};
use std::fmt::{Debug, Display};
use std::marker::{Send, Sync};

use num::{integer::Roots, PrimInt, Signed, Unsigned};

mod arith;
mod elliptic;
mod factor;
mod lin;
mod prime;
mod quad;

pub trait UInt:
    PrimInt + Unsigned + Roots + Display + Debug + From<u8> + Into<u128> + Send + Sync
{
}

impl<T> UInt for T where
    T: PrimInt + Unsigned + Roots + Display + Debug + From<u8> + Into<u128> + Send + Sync
{
}

impl<T> arith::CoreArith<T> for T where T: UInt {}
impl<T> arith::Arith<T> for T where T: UInt {}

pub trait Int: PrimInt + Signed + Display + Debug + From<i8> + Into<i128> {}

impl<S> Int for S where S: PrimInt + Signed + Display + Debug + From<i8> + Into<i128> {}

impl arith::SignCast<i8, u8> for i8 {}
impl arith::SignCast<i16, u16> for i16 {}
impl arith::SignCast<i32, u32> for i32 {}
impl arith::SignCast<i64, u64> for i64 {}
impl arith::SignCast<i128, u128> for i128 {}
impl arith::SignCast<isize, usize> for isize {}

pub use lin::{LinEq, LinEqSigned};
pub use quad::{QuadEq, QuadEqSigned};
