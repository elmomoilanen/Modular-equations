//!
//!
//!
use crate::{Int, UInt};

///
///
pub struct QuadEq<T: UInt> {
    pub a: T,
    pub b: T,
    pub c: T,
    pub d: T,
    pub modu: T,
}

///
///
pub struct QuadEqSigned<S: Int, T: UInt> {
    pub a: S,
    pub b: S,
    pub c: S,
    pub d: S,
    pub modu: T,
}
