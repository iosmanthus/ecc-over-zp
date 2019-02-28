use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Sub};

pub trait One {
    fn one() -> Self;
}

pub trait Zero {
    fn zero() -> Self;
}

impl One for i32 {
    fn one() -> Self {
        1
    }
}

impl Zero for i32 {
    fn zero() -> Self {
        0
    }
}

impl One for u8 {
    fn one() -> Self {
        1
    }
}

impl Zero for u8 {
    fn zero() -> Self {
        0
    }
}

pub trait Field:
    Sized
    + Clone
    + Zero
    + One
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + PartialEq
{
}

impl<T> Field for T where
    T: Sized
        + Clone
        + Zero
        + One
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + PartialEq
{}
