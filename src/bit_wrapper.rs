use super::field::{One, Zero};
use std::ops::{Add, BitAnd, Div, Mul, Shr, Sub};

pub trait IntoBit {
    fn into_bit(self) -> Vec<BitWrapper>;
}

impl<T> IntoBit for T
where
    T: Sized
        + One
        + Zero
        + BitAnd<Output = T>
        + Shr<usize, Output = T>
        + PartialEq
        + Clone,
{
    fn into_bit(self) -> Vec<BitWrapper> {
        let mut n = self;
        let mut vec = vec![];
        while {
            vec.push(BitWrapper(if n.clone() & T::one() == T::one() {
                true
            } else {
                false
            }));
            n = n >> 1;

            n != T::zero()
        } {}
        vec
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Eq)]
pub struct BitWrapper(bool);

impl One for BitWrapper {
    fn one() -> Self {
        BitWrapper(true)
    }
}

impl Zero for BitWrapper {
    fn zero() -> Self {
        BitWrapper(false)
    }
}

impl From<BitWrapper> for bool {
    fn from(bit: BitWrapper) -> Self {
        let BitWrapper(b) = bit;
        b
    }
}

impl Add for BitWrapper {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        BitWrapper(self.0 ^ rhs.0)
    }
}

impl Sub for BitWrapper {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        BitWrapper(self.0 ^ rhs.0)
    }
}

impl Mul for BitWrapper {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        BitWrapper(self.0 & rhs.0)
    }
}

impl Div for BitWrapper {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        assert_ne!(self, Self::zero());
        BitWrapper(self.0 & rhs.0)
    }
}
