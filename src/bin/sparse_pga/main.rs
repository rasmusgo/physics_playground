#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod coefficient;
mod elements;
mod multivector;

fn main() {
    let p = elements::Point::default();
    let m = elements::Line::default();
    let r = p + m;
    println!("{r:?}");
}
