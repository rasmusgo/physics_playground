#![allow(clippy::too_many_arguments)]
#![allow(non_camel_case_types)]

use std::f64::consts::PI;
use std::fmt;
use std::ops::{Add, BitAnd, BitOr, BitXor, Index, IndexMut, Mul, Not, Sub};

#[derive(Default, Debug)]
pub struct Zero {}

#[derive(Default, Debug)]
pub struct One {}

#[derive(Default, Debug)]
pub struct MinusOne {}

type Float = f64;

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

#[test]
fn test_add_multivector() {
    let motor = UnitMotor {
        e01: 0.0,
        e02: 0.0,
        e03: 0.0,
        e12: 0.0,
        e31: 0.0,
        e23: 0.0,
        ..Default::default()
    };
    let line = Line {
        e01: 0.0,
        e02: 0.0,
        e03: 0.0,
        e12: 0.0,
        e31: 0.0,
        e23: 0.0,
        ..Default::default()
    };
    let result: UnitMotor = motor + line;
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct MultiVector<
    T_one,
    T_e0,
    T_e1,
    T_e2,
    T_e3,
    T_e01,
    T_e02,
    T_e03,
    T_e12,
    T_e31,
    T_e23,
    T_e021,
    T_e013,
    T_e032,
    T_e123,
    T_e0123,
> {
    pub one: T_one,
    pub e0: T_e0,
    pub e1: T_e1,
    pub e2: T_e2,
    pub e3: T_e3,
    pub e01: T_e01,
    pub e02: T_e02,
    pub e03: T_e03,
    pub e12: T_e12,
    pub e31: T_e31,
    pub e23: T_e23,
    pub e021: T_e021,
    pub e013: T_e013,
    pub e032: T_e032,
    pub e123: T_e123,
    pub e0123: T_e0123,
}

impl<
        L_one,
        L_e0,
        L_e1,
        L_e2,
        L_e3,
        L_e01,
        L_e02,
        L_e03,
        L_e12,
        L_e31,
        L_e23,
        L_e021,
        L_e013,
        L_e032,
        L_e123,
        L_e0123,
        R_one,
        R_e0,
        R_e1,
        R_e2,
        R_e3,
        R_e01,
        R_e02,
        R_e03,
        R_e12,
        R_e31,
        R_e23,
        R_e021,
        R_e013,
        R_e032,
        R_e123,
        R_e0123,
    >
    Add<
        MultiVector<
            R_one,
            R_e0,
            R_e1,
            R_e2,
            R_e3,
            R_e01,
            R_e02,
            R_e03,
            R_e12,
            R_e31,
            R_e23,
            R_e021,
            R_e013,
            R_e032,
            R_e123,
            R_e0123,
        >,
    >
    for MultiVector<
        L_one,
        L_e0,
        L_e1,
        L_e2,
        L_e3,
        L_e01,
        L_e02,
        L_e03,
        L_e12,
        L_e31,
        L_e23,
        L_e021,
        L_e013,
        L_e032,
        L_e123,
        L_e0123,
    >
where
    L_one: Add<R_one>,
    L_e0: Add<R_e0>,
    L_e1: Add<R_e1>,
    L_e2: Add<R_e2>,
    L_e3: Add<R_e3>,
    L_e01: Add<R_e01>,
    L_e02: Add<R_e02>,
    L_e03: Add<R_e03>,
    L_e12: Add<R_e12>,
    L_e31: Add<R_e31>,
    L_e23: Add<R_e23>,
    L_e021: Add<R_e021>,
    L_e013: Add<R_e013>,
    L_e032: Add<R_e032>,
    L_e123: Add<R_e123>,
    L_e0123: Add<R_e0123>,
{
    type Output = MultiVector<
        <L_one as Add<R_one>>::Output,
        <L_e0 as Add<R_e0>>::Output,
        <L_e1 as Add<R_e1>>::Output,
        <L_e2 as Add<R_e2>>::Output,
        <L_e3 as Add<R_e3>>::Output,
        <L_e01 as Add<R_e01>>::Output,
        <L_e02 as Add<R_e02>>::Output,
        <L_e03 as Add<R_e03>>::Output,
        <L_e12 as Add<R_e12>>::Output,
        <L_e31 as Add<R_e31>>::Output,
        <L_e23 as Add<R_e23>>::Output,
        <L_e021 as Add<R_e021>>::Output,
        <L_e013 as Add<R_e013>>::Output,
        <L_e032 as Add<R_e032>>::Output,
        <L_e123 as Add<R_e123>>::Output,
        <L_e0123 as Add<R_e0123>>::Output,
    >;
    fn add(
        self,
        rhs: MultiVector<
            R_one,
            R_e0,
            R_e1,
            R_e2,
            R_e3,
            R_e01,
            R_e02,
            R_e03,
            R_e12,
            R_e31,
            R_e23,
            R_e021,
            R_e013,
            R_e032,
            R_e123,
            R_e0123,
        >,
    ) -> Self::Output {
        Self::Output {
            one: self.one + rhs.one,
            e0: self.e0 + rhs.e0,
            e1: self.e1 + rhs.e1,
            e2: self.e2 + rhs.e2,
            e3: self.e3 + rhs.e3,
            e01: self.e01 + rhs.e01,
            e02: self.e02 + rhs.e02,
            e03: self.e03 + rhs.e03,
            e12: self.e12 + rhs.e12,
            e31: self.e31 + rhs.e31,
            e23: self.e23 + rhs.e23,
            e021: self.e021 + rhs.e021,
            e013: self.e013 + rhs.e013,
            e032: self.e032 + rhs.e032,
            e123: self.e123 + rhs.e123,
            e0123: self.e0123 + rhs.e0123,
        }
    }
}

pub type Line = MultiVector<
    Zero,
    Zero,
    Zero,
    Zero,
    Zero,
    Float,
    Float,
    Float,
    Float,
    Float,
    Float,
    Zero,
    Zero,
    Zero,
    Zero,
    Zero,
>;

pub type UnitMotor = MultiVector<
    One,
    Zero,
    Zero,
    Zero,
    Zero,
    Float,
    Float,
    Float,
    Float,
    Float,
    Float,
    Zero,
    Zero,
    Zero,
    Zero,
    Zero,
>;

pub type Point = MultiVector<
    Zero,
    Zero,
    Zero,
    Zero,
    Zero,
    Zero,
    Zero,
    Zero,
    Zero,
    Zero,
    Zero,
    Float,
    Float,
    Float,
    Float,
    Zero,
>;

pub type UnitPoint = MultiVector<
    Zero,
    Zero,
    Zero,
    Zero,
    Zero,
    Zero,
    Zero,
    Zero,
    Zero,
    Zero,
    Zero,
    Float,
    Float,
    Float,
    One,
    Zero,
>;
