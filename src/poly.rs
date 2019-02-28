use super::field::{Field, One, Zero};
use super::operation::{DivRem, Egcd};
//use std::cmp::Ordering;
use std::collections::VecDeque;
use std::iter::{Product, Sum};
use std::ops::{Add, BitAnd, Deref, Div, Mul, Neg, Rem, Shl, Shr, Sub};

#[derive(Debug, Clone)]
pub struct Poly<T> {
    data: VecDeque<T>,
}

impl<T> Deref for Poly<T> {
    type Target = VecDeque<T>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Zero> Zero for Poly<T> {
    fn zero() -> Self {
        Self {
            data: VecDeque::from(vec![T::zero()]),
        }
    }
}

impl<T: One> One for Poly<T> {
    fn one() -> Self {
        Self {
            data: VecDeque::from(vec![T::one()]),
        }
    }
}

impl<T> PartialEq for Poly<T>
where
    T: Zero + PartialEq,
{
    fn eq(&self, other: &Poly<T>) -> bool {
        let (mut i, l_len, r_len) = (0, self.data.len(), other.data.len());
        while i < l_len && i < r_len {
            if self.data[i] != other.data[i] {
                return false;
            }
            i += 1;
        }

        let mut j = i;
        while j < r_len {
            if other.data[j] != T::zero() {
                return false;
            }
            j += 1;
        }

        while i < l_len {
            if self.data[i] != T::zero() {
                return false;
            }
            i += 1;
        }
        true
    }
}

impl<T> Eq for Poly<T> where T: Zero + PartialEq {}

impl<T> Poly<T>
where
    T: Zero + PartialEq,
{
    fn order(&self) -> usize {
        let mut ord = 0;
        for (i, x) in self.data.iter().enumerate() {
            if *x != T::zero() {
                ord = i;
            }
        }
        ord
    }
}

impl<T> Poly<T> {
    pub fn new(coefficient: Vec<T>) -> Self {
        Self {
            data: VecDeque::from(coefficient),
        }
    }
}

impl<T> Poly<T>
where
    T: Zero + Clone,
{
    pub fn sub_item(&self, index: usize) -> Self {
        match self.data.get(index) {
            Some(c) => Self::new(vec![c.clone()]) << index,
            _ => Self::zero(),
        }
    }

    pub fn coefficient(&self, index: usize) -> T {
        match self.data.get(index) {
            Some(c) => c.clone(),
            _ => T::zero(),
        }
    }

    pub fn first_coefficient(&self) -> T {
        self.coefficient(0)
    }
}
impl<T> Poly<T>
where
    T: Zero + PartialEq + Clone,
{
    pub fn last_coefficient(&self) -> T {
        self.coefficient(self.order())
    }
}

impl<T> Poly<T>
where
    T: Clone,
{
    fn align(&mut self, rhs: &mut Self, padding: T) {
        let l = self.data.len();
        let r = rhs.data.len();
        if l > r {
            rhs.data.resize(l, padding);
        } else {
            self.data.resize(r, padding);
        }
    }
}

impl<T> Add for Poly<T>
where
    T: Zero + Add<Output = T> + Clone,
{
    type Output = Self;
    fn add(mut self, mut rhs: Self) -> Self::Output {
        self.align(&mut rhs, T::zero());
        Poly {
            data: self
                .data
                .into_iter()
                .zip(rhs.data)
                .map(|(x, y)| x + y)
                .collect(),
        }
    }
}

impl<T> Sub for Poly<T>
where
    T: Zero + Sub<Output = T> + Clone,
{
    type Output = Self;
    fn sub(mut self, mut rhs: Self) -> Self::Output {
        self.align(&mut rhs, T::zero());
        Poly {
            data: self
                .data
                .into_iter()
                .zip(rhs.data)
                .map(|(x, y)| x - y)
                .collect(),
        }
    }
}

impl<T> Neg for Poly<T>
where
    T: Zero + Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Poly {
            data: self.data.into_iter().map(|x| -x).collect(),
        }
    }
}

impl<T> Shl<usize> for Poly<T>
where
    T: Zero,
{
    type Output = Self;
    fn shl(mut self, rhs: usize) -> Self::Output {
        for _ in 0..rhs {
            self.data.push_front(T::zero());
        }
        self
    }
}

impl<T> Shr<usize> for Poly<T>
where
    T: Zero,
{
    type Output = Self;
    fn shr(mut self, rhs: usize) -> Self::Output {
        for _ in 0..rhs {
            let _ = self.data.pop_front().unwrap();
            if self.data.is_empty() {
                self.data.push_front(T::zero());
                break;
            }
        }
        self
    }
}

impl<T> BitAnd for Poly<T>
where
    T: BitAnd<Output = T> + Zero + Clone,
{
    type Output = Self;
    fn bitand(mut self, mut rhs: Self) -> Self::Output {
        self.align(&mut rhs, T::zero());
        Poly {
            data: self
                .data
                .into_iter()
                .zip(rhs.data)
                .map(|(x, y)| x & y)
                .collect(),
        }
    }
}

impl<T> Mul for Poly<T>
where
    T: Zero + Add<Output = T> + Mul<Output = T> + Clone,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        rhs.data
            .iter()
            .enumerate()
            .map(|(i, b)| {
                Poly {
                    data: self
                        .data
                        .iter()
                        .map(|a| a.clone() * b.clone())
                        .collect(),
                } << i
            })
            .sum()
    }
}

impl<T> Sum for Poly<T>
where
    T: Zero + Add<Output = T> + Clone,
{
    fn sum<I: Iterator>(iter: I) -> Self
    where
        I: Iterator<Item = Poly<T>>,
    {
        iter.fold(Poly::zero(), |a, b| a + b)
    }
}

impl<T> Product for Poly<T>
where
    T: One + Zero + Add<Output = T> + Mul<Output = T> + Clone,
{
    fn product<I: Iterator>(iter: I) -> Self
    where
        I: Iterator<Item = Poly<T>>,
    {
        iter.fold(Poly::one(), |acc, x| acc * x)
    }
}

impl<T> DivRem for Poly<T>
where
    T: Field,
{
    type Output = Self;
    fn divrem(self, rhs: Self) -> (Self::Output, Self::Output) {
        if self == Self::zero() {
            (Self::zero(), Self::zero())
        } else {
            let (q, r) = (self.clone() >> 1).divrem(rhs.clone());
            let (q, r) = (q << 1, r << 1);
            let r = r + self.sub_item(0);

            if r.order() >= rhs.order() {
                let c =
                    T::one() / rhs.last_coefficient() * r.last_coefficient();
                let c = Self::new(vec![c]);
                return (q + c.clone(), r - rhs * c);
            }
            (q, r)
        }
    }
}

impl<T> Div for Poly<T>
where
    T: Field,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self.divrem(rhs).0
    }
}

impl<T> Rem for Poly<T>
where
    T: Field,
{
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        self.divrem(rhs).1
    }
}

impl<T> Egcd for Poly<T>
where
    T: Field,
{
    type Output = Self;
    fn egcd(self, rhs: Self) -> (Self::Output, Self::Output) {
        if rhs == Self::zero() {
            (Self::one(), Self::zero())
        } else {
            let (q, r) = self.divrem(rhs.clone());
            let (x, y) = rhs.egcd(r);
            (y.clone(), x - q * y)
        }
    }
}
