use super::field::Field;

pub trait Ec<F: Field> {
    fn add(&self, p: Point<F>, q: Point<F>) -> Option<Point<F>>;
    fn neg(&self, p: Point<F>) -> Option<Point<F>>;
    fn mul(&self, n: isize, p: Point<F>) -> Option<Point<F>>;
}

#[derive(PartialEq, Clone, Eq, Debug)]
pub enum Point<F: Field> {
    Identity,
    Ordinary(F, F),
}

impl<F: Field> Point<F> {
    pub fn new(x: F, y: F) -> Self {
        Point::Ordinary(x, y)
    }
}
