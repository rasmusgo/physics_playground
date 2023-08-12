#![allow(clippy::too_many_arguments)]
#![allow(non_camel_case_types)]

use std::ops::{Add, BitAnd, BitOr, BitXor, Index, IndexMut, Mul, Not, Sub};

#[derive(Default, Debug)]
pub struct Zero {}

#[derive(Default, Debug)]
pub struct One {}

#[derive(Default, Debug)]
pub struct MinusOne {}

pub type Float = f64;

// 0 + x = x
impl<T> Add<T> for Zero {
    type Output = T;

    fn add(self, rhs: T) -> T {
        rhs
    }
}

// x + 0 = x
impl Add<Zero> for One {
    type Output = One;

    fn add(self, _rhs: Zero) -> One {
        self
    }
}
impl Add<Zero> for MinusOne {
    type Output = MinusOne;

    fn add(self, _rhs: Zero) -> MinusOne {
        self
    }
}
impl Add<Zero> for Float {
    type Output = Float;

    fn add(self, _rhs: Zero) -> Float {
        self
    }
}

// 1 + -1 = 0
impl Add<MinusOne> for One {
    type Output = Zero;

    fn add(self, _rhs: MinusOne) -> Zero {
        Zero {}
    }
}

// -1 + 1 = 0
impl Add<One> for MinusOne {
    type Output = Zero;

    fn add(self, _rhs: One) -> Zero {
        Zero {}
    }
}

// 1 + 1 = 2
impl Add<One> for One {
    type Output = Float;

    fn add(self, _rhs: One) -> Float {
        2.0
    }
}

// -1 + -1 = -2
impl Add<MinusOne> for MinusOne {
    type Output = Float;

    fn add(self, _rhs: MinusOne) -> Float {
        -2.0
    }
}

// 1 + x
impl Add<Float> for One {
    type Output = Float;

    fn add(self, rhs: Float) -> Float {
        1.0 + rhs
    }
}

// x + 1
impl Add<One> for Float {
    type Output = Float;

    fn add(self, _rhs: One) -> Float {
        self + 1.0
    }
}

// -1 + x
impl Add<Float> for MinusOne {
    type Output = Float;

    fn add(self, rhs: Float) -> Float {
        -1.0 + rhs
    }
}

// x + -1
impl Add<MinusOne> for Float {
    type Output = Float;

    fn add(self, _rhs: MinusOne) -> Float {
        self + -1.0
    }
}

#[test]
fn test_add() {
    let _foo: Zero = Zero {} + Zero {};
    let _foo: Zero = One {} + MinusOne {};
    let _foo: Zero = MinusOne {} + One {};
    let _foo: One = One {} + Zero {};
    let _foo: One = Zero {} + One {};
    let _foo: MinusOne = MinusOne {} + Zero {};
    let _foo: MinusOne = Zero {} + MinusOne {};

    let foo: Float = MinusOne {} + MinusOne {};
    assert_eq!(foo, -2.0);

    let foo: Float = One {} + One {};
    assert_eq!(foo, 2.0);
}
