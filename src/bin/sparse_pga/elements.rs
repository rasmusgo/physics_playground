#![allow(clippy::too_many_arguments)]
#![allow(non_camel_case_types)]

use crate::coefficient::*;
use crate::multivector::*;

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
