//! Library implementing solvers for linear and quadratic modular equations.
//!
//! As it's described in Wikipedia, modular arithmetic is a system of arithmetic
//! where elements of the system (i.e., integers) wrap around after reaching
//! a specific boundary value called modulus.
//!
//! Before giving a concrete definition for such arithmetic system the following
//! relation needs to be stated: Given a positive integer M > 1, integer x is said
//! to be congruent to integer y, if M divides their difference (mathematically written
//! as M | (x - y)). In this case integers x and y are in a relation which is denoted
//! as x ≡ y (mod modu) and importantly this relation is an equivalence relation.
//!
//! Finally, the modular arithmetic system is constructed such that the elements of it
//! are so called residue or congruence classes [x] consisting of all the integers congruent
//! to x modulo M, or in other words all integers of the form {..., x - M, x, x + M, ...} =
//! {x + k * M}, k being a integer belonging to Z. Hence, in principle, all of these integers
//! are valid representatives of their residue class [x] but the common way is to use
//! the smallest nonnegative integer (modulo M) to represent the residue class. Congruence
//! relation was mentioned to be an equivalence relation and thus every integer can belong
//! to only one residue class modulo M.
//!
//! When listing all possible residue classes modulo M, a set of classes {[0], [1], ..., [M - 1]}
//! more precisely, and equipping this set with addition and multiplication operations (operations
//! are basically functions), the ring of integers modulo M (residue classes) is formed.
//! This ring is commonly denoted as Z/nZ where n the is modulo. Mentioned binary operations
//! are well-defined due to the fact that congruence relation is an equivalence relation. If the
//! modulo is a prime number, the ring becomes actually a field. These fields are more easier
//! to work with since every nonzero elements have a multiplicative inverse.
//!
//! In the context of rings or fields, it's meaningful to speak of equations and this library
//! implements solvers for linear and quadratic equations.
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
