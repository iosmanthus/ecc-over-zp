use self::Point::*;
use super::ec::{Ec, Point};
use super::gf256::Gf256;
use rayon::prelude::*;
use std::collections::HashSet;
use std::fmt::{self, Debug};
use std::hash::{Hash, Hasher};

impl Hash for Point<Gf256> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Point::Identity => {
                283.hash(state);
                285.hash(state)
            }
            Point::Ordinary(x, y) => {
                u8::from(x.clone()).hash(state);
                u8::from(y.clone()).hash(state);
            }
        }
    }
}

#[derive(Debug)]
pub struct E256 {
    a: Gf256,
    b: Gf256,
    set: HashSet<Point<Gf256>>,
}

impl E256 {
    pub fn new(a: Gf256, b: Gf256) -> Self {
        let mut set = (0..256i32)
            .into_par_iter()
            .map(|i| {
                (0..256i32)
                    .into_par_iter()
                    .map(|j| (i, j))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .map(|(i, j)| {
                Point::new(Gf256::from(i as u8), Gf256::from(j as u8))
            })
            .filter(|p| Self::equation(a.clone(), b.clone(), p))
            .collect::<HashSet<_>>();
        let _ = set.insert(Point::Identity);
        Self { a, b, set }
    }
    fn equation(a: Gf256, b: Gf256, p: &Point<Gf256>) -> bool {
        match p {
            Identity => true,
            Ordinary(x, y) => {
                let (x, y) = (x.clone(), y.clone());
                let left = y.clone() * y.clone() + x.clone() * y.clone();
                let right = x.clone() * x.clone() * x.clone()
                    + a.clone() * x.clone() * x.clone()
                    + b.clone();
                left == right
            }
        }
    }

    pub fn order(&self) -> usize {
        self.set.len()
    }

    pub fn points(&self) -> HashSet<Point<Gf256>> {
        self.set.clone()
    }

    pub fn is_generator(&self, p: &Point<Gf256>) -> bool {
        let mut set = HashSet::new();
        let mut walk = Point::Identity;
        for _ in 0..self.order() {
            walk = self.add(walk.clone(), p.clone()).unwrap();
            if !set.insert(walk.clone()) {
                return false;
            }
        }
        true
    }
    fn contains(&self, p: &Point<Gf256>) -> bool {
        Self::equation(self.a.clone(), self.b.clone(), p)
    }
}

impl Ec<Gf256> for E256 {
    fn add(&self, p: Point<Gf256>, q: Point<Gf256>) -> Option<Point<Gf256>> {
        if !self.contains(&p) || !self.contains(&q) {
            return None;
        }
        if p == Identity {
            return Some(q);
        }
        if q == Identity {
            return Some(p);
        }
        if p == self.neg(q.clone()).unwrap() {
            return Some(Identity);
        }
        if let Ordinary(x0, y0) = p {
            if let Ordinary(x1, y1) = q {
                let k;
                if x0 == x1 && y0 == y1 {
                    k = x0.clone() + y0.clone() / x1.clone();
                } else {
                    k = (y0.clone() + y1.clone()) / (x0.clone() + x1.clone());
                }
                let x2 = k.clone() * k.clone()
                    + k.clone()
                    + x0.clone()
                    + x1.clone()
                    + self.a.clone();
                let y2 = k.clone() * (x0.clone() + x2.clone())
                    + x2.clone()
                    + y0.clone();
                return Some(Ordinary(x2, y2));
            }
        }
        unreachable!()
    }
    fn neg(&self, p: Point<Gf256>) -> Option<Point<Gf256>> {
        if !self.contains(&p) {
            return None;
        }
        match p {
            Identity => Some(Identity),
            Ordinary(x, y) => Some(Point::Ordinary(x.clone(), x + y)),
        }
    }
    fn mul(&self, mut n: isize, p: Point<Gf256>) -> Option<Point<Gf256>> {
        if !self.contains(&p) {
            return None;
        }
        match p {
            Identity => Some(Identity),
            _ => {
                if n == 0 {
                    return Some(Identity);
                }
                let sign = n < 0;
                n = n.abs();
                let mut sum = (0..n)
                    .map(|_| p.clone())
                    .fold(Identity, |acc, x| self.add(acc, x).unwrap());
                if sign {
                    sum = self.neg(sum.clone()).unwrap();
                }
                Some(sum)
            }
        }
    }
}
