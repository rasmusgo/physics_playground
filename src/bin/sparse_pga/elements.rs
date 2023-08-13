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

pub type Motor = MultiVector<
    Float,
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
    Float,
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
fn test_add_motor_line() {
    let motor = Motor {
        one: 1.0,
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
    let result: Motor = motor + line;
}

#[test]
fn test_mul_motor_motor() {
    let motor1 = Motor {
        one: 1.0,
        e01: 0.0,
        e02: 0.0,
        e03: 0.0,
        e12: 0.0,
        e31: 0.0,
        e23: 0.0,
        ..Default::default()
    };
    let motor2 = Motor {
        one: 1.0,
        e01: 0.0,
        e02: 0.0,
        e03: 0.0,
        e12: 0.0,
        e31: 0.0,
        e23: 0.0,
        ..Default::default()
    };
    let result: Motor = motor1 * motor2;
}

#[test]
fn test_mul_motor_line() {
    let motor = Motor {
        one: 1.0,
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
    let result: Motor = motor * line;
}

#[test]
fn test_transform_line() {
    let motor = Motor {
        one: 1.0,
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
    let result: Motor = motor.clone() * line * motor.reverse();
}
