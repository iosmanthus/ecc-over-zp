//use super::field::{One, Zero};
//use std::ops::{Add, BitAnd, Shl, Shr, Sub};

pub trait DivRem<RHS = Self>: Sized {
    type Output;
    fn divrem(self, rhs: RHS) -> (Self::Output, Self::Output);
}

pub trait Egcd<RHS = Self> {
    type Output;
    fn egcd(self, rhs: RHS) -> (Self::Output, Self::Output);
}

pub trait ModInv<MOD = Self> {
    type Output;
    fn modinv(self, modulo: MOD) -> Self::Output;
}

impl<T> ModInv for T
where
    T: Egcd<Output = T>,
{
    type Output = Self;
    fn modinv(self, modulo: Self) -> Self::Output {
        self.egcd(modulo).0
    }
}
