use super::bit_wrapper::{BitWrapper, IntoBit};
use super::field::{One, Zero};
use super::operation::ModInv;
use super::poly::Poly;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, PartialEq, Eq)]
pub struct Gf256(Poly<BitWrapper>);

impl Gf256 {
    const PRIMITIVE: i32 = 285;
    fn primitive() -> Poly<BitWrapper> {
        Poly::new(Self::PRIMITIVE.into_bit())
    }
}

impl std::fmt::Debug for Gf256 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", u8::from(self.clone()))
    }
}

impl From<u8> for Gf256 {
    fn from(x: u8) -> Self {
        Gf256(Poly::new(x.into_bit()))
    }
}

impl From<Gf256> for u8 {
    fn from(poly: Gf256) -> Self {
        poly.0
            .iter()
            .enumerate()
            .map(|(i, &c)| if bool::from(c) { 1 << i } else { 0 })
            .sum()
    }
}

impl Zero for Gf256 {
    fn zero() -> Self {
        Gf256::from(0)
    }
}

impl One for Gf256 {
    fn one() -> Self {
        Gf256::from(1)
    }
}

impl Add for Gf256 {
    type Output = Self;
    fn add(self, rhs: Self) -> Gf256 {
        Gf256(self.0 + rhs.0)
    }
}

impl Sub for Gf256 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Gf256 {
        Gf256(self.0 - rhs.0)
    }
}

impl Mul for Gf256 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Gf256 {
        Gf256((self.0 * rhs.0) % Gf256::primitive())
    }
}

impl Div for Gf256 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Gf256((self.0 * rhs.0.modinv(Gf256::primitive())) % Gf256::primitive())
    }
}
