//! Library implementing solvers for linear and quadratic modular equations.
//!
//! As it's described in Wikipedia, modular arithmetic is a system of arithmetic
//! where elements of the system (i.e., integers) wrap around after reaching
//! a specific boundary value called modulus.
//!
//! Before giving a concrete definition for such arithmetic system the following congruence
//! relation needs to be introduced: Given a positive integer n > 1, integer x is said
//! to be congruent to integer y, if n divides their difference (written mathematically
//! as n | (x - y)). In this case integers x and y are in a relation which is denoted
//! by x ≡ y (mod n) and importantly this relation is an equivalence relation.
//!
//! Modular arithmetic system is constructed such that the elements of it are so
//! called residue or congruence classes \[x\], where one class \[x\] is consisting
//! of all the integers congruent to x modulo n, or in other words all integers of
//! the form {..., x - n, x, x + n, ...} = {x + k * n}, k being any integer. Hence,
//! in principle, all of these integers {x + k * n} are valid representatives of
//! their residue class \[x\] but the common way is to use the smallest nonnegative
//! integer (modulo n) to represent the residue class. As the congruence relation
//! is an equivalence relation, every integer can belong to only one residue class.
//!
//! When listing all possible residue classes modulo n, {\[0\], \[1\], ..., \[n - 1\]},
//! and equipping this set with addition and multiplication operations (operations are
//! basically functions), the ring of integers modulo n is formed. This ring is commonly
//! denoted by Z/nZ where n represents the modulo. Mentioned binary operations are
//! well-defined due to the fact that the congruence relation is an equivalence relation.
//! If the modulo is a prime number, the ring becomes actually a field. These fields
//! are more easier to work with since every nonzero element has a multiplicative inverse.
//!
//! Now in the context of rings and fields, it's meaningful to speak of equations
//! and this library implements solvers for linear and quadratic equations. Linear modular
//! equations are generally much easier to solve than their quadratic counterparts.
//! Structs `LinEq` and `LinEqSigned` defines linear equation types and their `solve`
//! methods actually solve the equations. Similarly for quadratic case, structs
//! `QuadEq` and `QuadEqSigned` define equation types and their `solve` methods can
//! be used to actually solve the equations.
//!
//! Next follows few examples of linear equations of the form ax + b = c (mod n).
//!
//! ```
//! use modular_equations::LinEq;
//!
//! let lin_eq = LinEq::<u32> {
//!     a: 13,
//!     b: 17,
//!     c: 5,
//!     modu: 29,
//! };
//! let sol = lin_eq.solve();
//!
//! // Residue class \[8\] is the correct solution (smallest nonnegative member)
//! assert!(sol.is_some() && sol.unwrap()[0] == 8);
//! ```
//!
//! Following linear equation doesn't have solution
//!
//! ```
//! use modular_equations::LinEqSigned;
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
//! If any of the coefficients (a, b, ...) is signed, one must use the signed type equation
//! `LinEqSigned` as above. Modulo must always be unsigned type. Every negative integer
//! in the ring can be turned to the smallest nonnegative representative of the
//! corresponding residue class \[x\]. Related to this fact there are few technical
//! restrictions, the first being that the used signed type (e.g. i32) must have
//! the arith::SignCast trait implemented and that trait requires the signed and
//! unsigned types to be compatible (i.e., have the same size in bytes). In addition,
//! as the smallest negative integer of each type doesn't have an absolute value in
//! two's complement, they will trigger immediate None return value if used as coefficients
//! in linear or quadratic equations.
//!
//! One important use case for linear equations is to find multiplicative inverses as
//! the following example tries to do for 17 in Z/255Z
//!
//! ```
//! use modular_equations::LinEq;
//!
//! let lin_eq = LinEq::<u8> {a: 17, b: 0, c: 1, modu: u8::MAX};
//!
//! // 17 doesn't have multiplicative inverse in this case
//! assert_eq!(lin_eq.solve(), None);
//! ```
//!
//! As mentioned above, quadratic equations of the form ax^2 + bx + c = d (mod n)
//! are typically much harder to solve than their linear counterparts. In particular,
//! this is the case when the modulo is a composite number as this requires
//! factorization of the modulo to its prime factors and solving the equation for
//! each of these prime factors before combining the final solution using the
//! Chinese remainder theorem.
//!
//! Consider now an example of solving a quadratic modular equation. As the d
//! coefficient is negative, the signed equation type `QuadEqSigned` must be used.
//!
//! ```
//! use modular_equations::QuadEqSigned;
//!
//! let quad_eq = QuadEqSigned::<i64, u64> {
//!     a: 1,
//!     b: 1,
//!     c: 1,
//!     d: -1,
//!     modu: 22,
//! };
//!
//! match quad_eq.solve() {
//!     Some(sols) if sols.len() == 4 => {
//!         // Correct solution consists of four residue classes
//!         assert_eq!(sols, vec![4, 6, 15, 17]);
//!     }
//!     _ => assert!(false),
//! }
//! ```
//!
//! An important use case for quadratic equations is to check whether a specific
//! integer q is a quadratic residue meaning that there exists an integer x s.t.
//! x^2 ≡ q (mod n) holds. Following example considers a case where for a relatively
//! large prime modulo it is checked whether 1 is quadratic residue.
//!
//! ```
//! use modular_equations::QuadEq;
//!
//! let quad_eq = QuadEq::<u128> {
//!     a: 1,
//!     b: 0,
//!     c: 0,
//!     d: 1,
//!     modu: 340_282_366_920_938_463_463_374_607_431_768_211_297,
//! };
//!
//! if let Some(x) = quad_eq.solve() {
//!     // There should be two solutions `x`, thus q=1 is a quadratic residue in this case
//!     assert_eq!(x.len(), 2);
//!     assert!(x[0] == 1 && x[1] == quad_eq.modu - 1);
//! } else {
//!     assert!(false);
//! }
//!
use std::convert::{From, Into};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::marker::{Send, Sync};

use num::{integer::Roots, PrimInt, Signed, Unsigned};

mod arith;
mod elliptic;
mod factor;
mod lin;
mod prime;
mod quad;
mod utils;

pub trait UInt:
    PrimInt + Unsigned + Roots + Display + Debug + From<u8> + Into<u128> + Hash + Send + Sync
{
}

impl<T> UInt for T where
    T: PrimInt + Unsigned + Roots + Display + Debug + From<u8> + Into<u128> + Hash + Send + Sync
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
