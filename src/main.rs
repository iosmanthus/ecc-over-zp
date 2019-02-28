use ecc::e256::E256;
use ecc::ec::{Ec, Point};
use ecc::gf256::Gf256;
use rayon::prelude::*;

fn main() {
    let a = Gf256::from(23);
    let b = Gf256::from(107);
    let ec = E256::new(a, b);
    ec.points().par_iter().for_each(|p| {
        println!(
            "{}",
            match p {
                Point::Identity => "".to_string(),
                Point::Ordinary(x, y) => {
                    format!("{},{}", u8::from(x.clone()), u8::from(y.clone()))
                }
            }
        );
    })
}
