// 3D Projective Geometric Algebra
// Written by a generator written by enki.
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(clippy::op_ref)]
#![allow(clippy::too_many_arguments)]
//#![feature(const_slice_len)]

use std::fmt;
use std::ops::{Add, BitAnd, BitOr, BitXor, Index, IndexMut, Mul, Not, Sub};

type float_t = f64;

use std::f64::consts::PI;

#[repr(usize)]
pub enum Basis {
    one,
    e0,
    e1,
    e2,
    e3,
    e01,
    e02,
    e03,
    e12,
    e31,
    e23,
    e021,
    e013,
    e032,
    e123,
    e0123,
}

const basis: &[&str] = &[
    "1", "e0", "e1", "e2", "e3", "e01", "e02", "e03", "e12", "e31", "e23", "e021", "e013", "e032",
    "e123", "e0123",
];
const basis_count: usize = basis.len();

#[derive(Default)]
pub struct Zero {}

#[derive(Default)]
pub struct One {}

#[derive(Default)]
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

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PGA3D {
    mvec: [float_t; basis_count],
}

impl PGA3D {
    pub const fn zero() -> Self {
        Self {
            mvec: [0.0; basis_count],
        }
    }

    pub const fn new(f: float_t, idx: usize) -> Self {
        let mut ret = Self::zero();
        ret.mvec[idx] = f;
        ret
    }

    // basis vectors are available as methods
    pub const fn e0() -> Self {
        PGA3D::new(1.0, 1)
    }
    pub const fn e1() -> Self {
        PGA3D::new(1.0, 2)
    }
    pub const fn e2() -> Self {
        PGA3D::new(1.0, 3)
    }
    pub const fn e3() -> Self {
        PGA3D::new(1.0, 4)
    }
    pub const fn e01() -> Self {
        PGA3D::new(1.0, 5)
    }
    pub const fn e02() -> Self {
        PGA3D::new(1.0, 6)
    }
    pub const fn e03() -> Self {
        PGA3D::new(1.0, 7)
    }
    pub const fn e12() -> Self {
        PGA3D::new(1.0, 8)
    }
    pub const fn e31() -> Self {
        PGA3D::new(1.0, 9)
    }
    pub const fn e23() -> Self {
        PGA3D::new(1.0, 10)
    }
    pub const fn e021() -> Self {
        PGA3D::new(1.0, 11)
    }
    pub const fn e013() -> Self {
        PGA3D::new(1.0, 12)
    }
    pub const fn e032() -> Self {
        PGA3D::new(1.0, 13)
    }
    pub const fn e123() -> Self {
        PGA3D::new(1.0, 14)
    }
    pub const fn e0123() -> Self {
        PGA3D::new(1.0, 15)
    }
}

impl Index<usize> for PGA3D {
    type Output = float_t;

    fn index(&self, index: usize) -> &Self::Output {
        &self.mvec[index]
    }
}

impl IndexMut<usize> for PGA3D {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.mvec[index]
    }
}

impl fmt::Display for PGA3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut n = 0;
        let ret = self
            .mvec
            .iter()
            .enumerate()
            .filter_map(|(i, &coeff)| {
                if !(-0.00001..=0.00001).contains(&coeff) {
                    n = 1;
                    Some(format!(
                        "{}{}",
                        format!("{:.*}", 7, coeff)
                            .trim_end_matches('0')
                            .trim_end_matches('.'),
                        if i > 0 { basis[i] } else { "" }
                    ))
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
            .join(" + ");
        if n == 0 {
            write!(f, "0")
        } else {
            write!(f, "{}", ret)
        }
    }
}

macro_rules! define_binary_op(
    (
        // Operator, operator method, and scalar bounds.
        $Op: ident, $op: ident;
        // Argument identifiers and types + output.
        $lhs: ident: $Lhs: ty, $rhs: ident: $Rhs: ty, Output = $Result: ty;
        // Operator actual implementation.
        $action: expr;
        // Lifetime.
        $($lives: tt),*
    ) => {
       impl<$($lives ,)*> $Op<$Rhs> for $Lhs {
           type Output = $Result;

           #[inline]
           fn $op($lhs, $rhs: $Rhs) -> Self::Output {
               $action
           }
       }
    }
);

macro_rules! define_binary_op_all(
    (
        // Operator, operator method, and scalar bounds.
        $Op: ident, $op: ident;
        // Argument identifiers and types + output.
        $lhs: ident: $Lhs: ty, $rhs: ident: $Rhs: ty, Output = $Result: ty;
        // Operators actual implementations.
        [val val] => $action_val_val: expr;
        [ref val] => $action_ref_val: expr;
        [val ref] => $action_val_ref: expr;
        [ref ref] => $action_ref_ref: expr;
    ) => {
        define_binary_op!(
            $Op, $op;
            $lhs: $Lhs, $rhs: $Rhs, Output = $Result;
            $action_val_val;
        );

        define_binary_op!(
            $Op, $op;
            $lhs: &'a $Lhs, $rhs: $Rhs, Output = $Result;
            $action_ref_val;
            'a
        );

        define_binary_op!(
            $Op, $op;
            $lhs: $Lhs, $rhs: &'b $Rhs, Output = $Result;
            $action_val_ref;
            'b
        );

        define_binary_op!(
            $Op, $op;
            $lhs: &'a $Lhs, $rhs: &'b $Rhs, Output = $Result;
            $action_ref_ref;
            'a, 'b
        );
    }
);

// TODO define_unary_op

// Reverse
// Reverse the order of the basis blades.
impl PGA3D {
    pub fn Reverse(&self) -> PGA3D {
        let mut res = PGA3D::zero();
        let a = self;
        res[0] = a[0];
        res[1] = a[1];
        res[2] = a[2];
        res[3] = a[3];
        res[4] = a[4];
        res[5] = -a[5];
        res[6] = -a[6];
        res[7] = -a[7];
        res[8] = -a[8];
        res[9] = -a[9];
        res[10] = -a[10];
        res[11] = -a[11];
        res[12] = -a[12];
        res[13] = -a[13];
        res[14] = -a[14];
        res[15] = a[15];
        res
    }
}

// Dual
// Poincare duality operator.
impl PGA3D {
    pub fn Dual(&self) -> PGA3D {
        let mut res = PGA3D::zero();
        let a = self;
        res[0] = a[15];
        res[1] = a[14];
        res[2] = a[13];
        res[3] = a[12];
        res[4] = a[11];
        res[5] = a[10];
        res[6] = a[9];
        res[7] = a[8];
        res[8] = a[7];
        res[9] = a[6];
        res[10] = a[5];
        res[11] = a[4];
        res[12] = a[3];
        res[13] = a[2];
        res[14] = a[1];
        res[15] = a[0];
        res
    }
}

impl Not for &PGA3D {
    type Output = PGA3D;

    fn not(self) -> PGA3D {
        let mut res = PGA3D::zero();
        let a = self;
        res[0] = a[15];
        res[1] = a[14];
        res[2] = a[13];
        res[3] = a[12];
        res[4] = a[11];
        res[5] = a[10];
        res[6] = a[9];
        res[7] = a[8];
        res[8] = a[7];
        res[9] = a[6];
        res[10] = a[5];
        res[11] = a[4];
        res[12] = a[3];
        res[13] = a[2];
        res[14] = a[1];
        res[15] = a[0];
        res
    }
}

// Conjugate
// Clifford Conjugation
impl PGA3D {
    pub fn Conjugate(&self) -> PGA3D {
        let mut res = PGA3D::zero();
        let a = self;
        res[0] = a[0];
        res[1] = -a[1];
        res[2] = -a[2];
        res[3] = -a[3];
        res[4] = -a[4];
        res[5] = -a[5];
        res[6] = -a[6];
        res[7] = -a[7];
        res[8] = -a[8];
        res[9] = -a[9];
        res[10] = -a[10];
        res[11] = a[11];
        res[12] = a[12];
        res[13] = a[13];
        res[14] = a[14];
        res[15] = a[15];
        res
    }
}

// Involute
// Main involution
impl PGA3D {
    pub fn Involute(&self) -> PGA3D {
        let mut res = PGA3D::zero();
        let a = self;
        res[0] = a[0];
        res[1] = -a[1];
        res[2] = -a[2];
        res[3] = -a[3];
        res[4] = -a[4];
        res[5] = a[5];
        res[6] = a[6];
        res[7] = a[7];
        res[8] = a[8];
        res[9] = a[9];
        res[10] = a[10];
        res[11] = -a[11];
        res[12] = -a[12];
        res[13] = -a[13];
        res[14] = -a[14];
        res[15] = a[15];
        res
    }
}

// Mul
// The geometric product.

define_binary_op_all!(
    Mul,
    mul;
    self: PGA3D, b: PGA3D, Output = PGA3D;
    [val val] => &self * &b;
    [ref val] =>  self * &b;
    [val ref] => &self *  b;
    [ref ref] => {
        let mut res = PGA3D::zero();
        let a = self;
        res[0] = b[0]*a[0]+b[2]*a[2]+b[3]*a[3]+b[4]*a[4]-b[8]*a[8]-b[9]*a[9]-b[10]*a[10]-b[14]*a[14];
        res[1] = b[1]*a[0]+b[0]*a[1]-b[5]*a[2]-b[6]*a[3]-b[7]*a[4]+b[2]*a[5]+b[3]*a[6]+b[4]*a[7]+b[11]*a[8]+b[12]*a[9]+b[13]*a[10]+b[8]*a[11]+b[9]*a[12]+b[10]*a[13]+b[15]*a[14]-b[14]*a[15];
        res[2] = b[2]*a[0]+b[0]*a[2]-b[8]*a[3]+b[9]*a[4]+b[3]*a[8]-b[4]*a[9]-b[14]*a[10]-b[10]*a[14];
        res[3] = b[3]*a[0]+b[8]*a[2]+b[0]*a[3]-b[10]*a[4]-b[2]*a[8]-b[14]*a[9]+b[4]*a[10]-b[9]*a[14];
        res[4] = b[4]*a[0]-b[9]*a[2]+b[10]*a[3]+b[0]*a[4]-b[14]*a[8]+b[2]*a[9]-b[3]*a[10]-b[8]*a[14];
        res[5] = b[5]*a[0]+b[2]*a[1]-b[1]*a[2]-b[11]*a[3]+b[12]*a[4]+b[0]*a[5]-b[8]*a[6]+b[9]*a[7]+b[6]*a[8]-b[7]*a[9]-b[15]*a[10]-b[3]*a[11]+b[4]*a[12]+b[14]*a[13]-b[13]*a[14]-b[10]*a[15];
        res[6] = b[6]*a[0]+b[3]*a[1]+b[11]*a[2]-b[1]*a[3]-b[13]*a[4]+b[8]*a[5]+b[0]*a[6]-b[10]*a[7]-b[5]*a[8]-b[15]*a[9]+b[7]*a[10]+b[2]*a[11]+b[14]*a[12]-b[4]*a[13]-b[12]*a[14]-b[9]*a[15];
        res[7] = b[7]*a[0]+b[4]*a[1]-b[12]*a[2]+b[13]*a[3]-b[1]*a[4]-b[9]*a[5]+b[10]*a[6]+b[0]*a[7]-b[15]*a[8]+b[5]*a[9]-b[6]*a[10]+b[14]*a[11]-b[2]*a[12]+b[3]*a[13]-b[11]*a[14]-b[8]*a[15];
        res[8] = b[8]*a[0]+b[3]*a[2]-b[2]*a[3]+b[14]*a[4]+b[0]*a[8]+b[10]*a[9]-b[9]*a[10]+b[4]*a[14];
        res[9] = b[9]*a[0]-b[4]*a[2]+b[14]*a[3]+b[2]*a[4]-b[10]*a[8]+b[0]*a[9]+b[8]*a[10]+b[3]*a[14];
        res[10] = b[10]*a[0]+b[14]*a[2]+b[4]*a[3]-b[3]*a[4]+b[9]*a[8]-b[8]*a[9]+b[0]*a[10]+b[2]*a[14];
        res[11] = b[11]*a[0]-b[8]*a[1]+b[6]*a[2]-b[5]*a[3]+b[15]*a[4]-b[3]*a[5]+b[2]*a[6]-b[14]*a[7]-b[1]*a[8]+b[13]*a[9]-b[12]*a[10]+b[0]*a[11]+b[10]*a[12]-b[9]*a[13]+b[7]*a[14]-b[4]*a[15];
        res[12] = b[12]*a[0]-b[9]*a[1]-b[7]*a[2]+b[15]*a[3]+b[5]*a[4]+b[4]*a[5]-b[14]*a[6]-b[2]*a[7]-b[13]*a[8]-b[1]*a[9]+b[11]*a[10]-b[10]*a[11]+b[0]*a[12]+b[8]*a[13]+b[6]*a[14]-b[3]*a[15];
        res[13] = b[13]*a[0]-b[10]*a[1]+b[15]*a[2]+b[7]*a[3]-b[6]*a[4]-b[14]*a[5]-b[4]*a[6]+b[3]*a[7]+b[12]*a[8]-b[11]*a[9]-b[1]*a[10]+b[9]*a[11]-b[8]*a[12]+b[0]*a[13]+b[5]*a[14]-b[2]*a[15];
        res[14] = b[14]*a[0]+b[10]*a[2]+b[9]*a[3]+b[8]*a[4]+b[4]*a[8]+b[3]*a[9]+b[2]*a[10]+b[0]*a[14];
        res[15] = b[15]*a[0]+b[14]*a[1]+b[13]*a[2]+b[12]*a[3]+b[11]*a[4]+b[10]*a[5]+b[9]*a[6]+b[8]*a[7]+b[7]*a[8]+b[6]*a[9]+b[5]*a[10]-b[4]*a[11]-b[3]*a[12]-b[2]*a[13]-b[1]*a[14]+b[0]*a[15];
        res
    };
);

// Wedge
// The outer product. (MEET)

define_binary_op_all!(
    BitXor,
    bitxor;
    self: PGA3D, b: PGA3D, Output = PGA3D;
    [val val] => &self ^ &b;
    [ref val] =>  self ^ &b;
    [val ref] => &self ^  b;
    [ref ref] => {
        let mut res = PGA3D::zero();
        let a = self;
        res[0] = b[0]*a[0];
        res[1] = b[1]*a[0]+b[0]*a[1];
        res[2] = b[2]*a[0]+b[0]*a[2];
        res[3] = b[3]*a[0]+b[0]*a[3];
        res[4] = b[4]*a[0]+b[0]*a[4];
        res[5] = b[5]*a[0]+b[2]*a[1]-b[1]*a[2]+b[0]*a[5];
        res[6] = b[6]*a[0]+b[3]*a[1]-b[1]*a[3]+b[0]*a[6];
        res[7] = b[7]*a[0]+b[4]*a[1]-b[1]*a[4]+b[0]*a[7];
        res[8] = b[8]*a[0]+b[3]*a[2]-b[2]*a[3]+b[0]*a[8];
        res[9] = b[9]*a[0]-b[4]*a[2]+b[2]*a[4]+b[0]*a[9];
        res[10] = b[10]*a[0]+b[4]*a[3]-b[3]*a[4]+b[0]*a[10];
        res[11] = b[11]*a[0]-b[8]*a[1]+b[6]*a[2]-b[5]*a[3]-b[3]*a[5]+b[2]*a[6]-b[1]*a[8]+b[0]*a[11];
        res[12] = b[12]*a[0]-b[9]*a[1]-b[7]*a[2]+b[5]*a[4]+b[4]*a[5]-b[2]*a[7]-b[1]*a[9]+b[0]*a[12];
        res[13] = b[13]*a[0]-b[10]*a[1]+b[7]*a[3]-b[6]*a[4]-b[4]*a[6]+b[3]*a[7]-b[1]*a[10]+b[0]*a[13];
        res[14] = b[14]*a[0]+b[10]*a[2]+b[9]*a[3]+b[8]*a[4]+b[4]*a[8]+b[3]*a[9]+b[2]*a[10]+b[0]*a[14];
        res[15] = b[15]*a[0]+b[14]*a[1]+b[13]*a[2]+b[12]*a[3]+b[11]*a[4]+b[10]*a[5]+b[9]*a[6]+b[8]*a[7]+b[7]*a[8]+b[6]*a[9]+b[5]*a[10]-b[4]*a[11]-b[3]*a[12]-b[2]*a[13]-b[1]*a[14]+b[0]*a[15];
        res
    };
);

// Vee
// The regressive product. (JOIN)

define_binary_op_all!(
    BitAnd,
    bitand;
    self: PGA3D, b: PGA3D, Output = PGA3D;
    [val val] => &self & &b;
    [ref val] =>  self & &b;
    [val ref] => &self &  b;
    [ref ref] => {
        let mut res = PGA3D::zero();
        let a = self;
        res[15] = a[15]*b[15];
        res[14] = -(a[14]*-b[15]+a[15]*b[14]*-1.0);
        res[13] = -(a[13]*-b[15]+a[15]*b[13]*-1.0);
        res[12] = -(a[12]*-b[15]+a[15]*b[12]*-1.0);
        res[11] = -(a[11]*-b[15]+a[15]*b[11]*-1.0);
        res[10] = a[10]*b[15]+a[13]*-b[14]*-1.0-a[14]*-b[13]*-1.0+a[15]*b[10];
        res[9] = a[9]*b[15]+a[12]*-b[14]*-1.0-a[14]*-b[12]*-1.0+a[15]*b[9];
        res[8] = a[8]*b[15]+a[11]*-b[14]*-1.0-a[14]*-b[11]*-1.0+a[15]*b[8];
        res[7] = a[7]*b[15]+a[12]*-b[13]*-1.0-a[13]*-b[12]*-1.0+a[15]*b[7];
        res[6] = a[6]*b[15]-a[11]*-b[13]*-1.0+a[13]*-b[11]*-1.0+a[15]*b[6];
        res[5] = a[5]*b[15]+a[11]*-b[12]*-1.0-a[12]*-b[11]*-1.0+a[15]*b[5];
        res[4] = a[4]*b[15]-a[7]*b[14]*-1.0+a[9]*b[13]*-1.0-a[10]*b[12]*-1.0-a[12]*-b[10]+a[13]*-b[9]-a[14]*-b[7]+a[15]*b[4];
        res[3] = a[3]*b[15]-a[6]*b[14]*-1.0-a[8]*b[13]*-1.0+a[10]*b[11]*-1.0+a[11]*-b[10]-a[13]*-b[8]-a[14]*-b[6]+a[15]*b[3];
        res[2] = a[2]*b[15]-a[5]*b[14]*-1.0+a[8]*b[12]*-1.0-a[9]*b[11]*-1.0-a[11]*-b[9]+a[12]*-b[8]-a[14]*-b[5]+a[15]*b[2];
        res[1] = a[1]*b[15]+a[5]*b[13]*-1.0+a[6]*b[12]*-1.0+a[7]*b[11]*-1.0+a[11]*-b[7]+a[12]*-b[6]+a[13]*-b[5]+a[15]*b[1];
        res[0] = a[0]*b[15]+a[1]*b[14]*-1.0+a[2]*b[13]*-1.0+a[3]*b[12]*-1.0+a[4]*b[11]*-1.0+a[5]*b[10]+a[6]*b[9]+a[7]*b[8]+a[8]*b[7]+a[9]*b[6]+a[10]*b[5]-a[11]*-b[4]-a[12]*-b[3]-a[13]*-b[2]-a[14]*-b[1]+a[15]*b[0];
        res
    };
);

// Dot
// The inner product.

define_binary_op_all!(
    BitOr,
    bitor;
    self: PGA3D, b: PGA3D, Output = PGA3D;
    [val val] => &self | &b;
    [ref val] =>  self | &b;
    [val ref] => &self |  b;
    [ref ref] => {
        let mut res = PGA3D::zero();
        let a = self;
        res[0] = b[0]*a[0]+b[2]*a[2]+b[3]*a[3]+b[4]*a[4]-b[8]*a[8]-b[9]*a[9]-b[10]*a[10]-b[14]*a[14];
        res[1] = b[1]*a[0]+b[0]*a[1]-b[5]*a[2]-b[6]*a[3]-b[7]*a[4]+b[2]*a[5]+b[3]*a[6]+b[4]*a[7]+b[11]*a[8]+b[12]*a[9]+b[13]*a[10]+b[8]*a[11]+b[9]*a[12]+b[10]*a[13]+b[15]*a[14]-b[14]*a[15];
        res[2] = b[2]*a[0]+b[0]*a[2]-b[8]*a[3]+b[9]*a[4]+b[3]*a[8]-b[4]*a[9]-b[14]*a[10]-b[10]*a[14];
        res[3] = b[3]*a[0]+b[8]*a[2]+b[0]*a[3]-b[10]*a[4]-b[2]*a[8]-b[14]*a[9]+b[4]*a[10]-b[9]*a[14];
        res[4] = b[4]*a[0]-b[9]*a[2]+b[10]*a[3]+b[0]*a[4]-b[14]*a[8]+b[2]*a[9]-b[3]*a[10]-b[8]*a[14];
        res[5] = b[5]*a[0]-b[11]*a[3]+b[12]*a[4]+b[0]*a[5]-b[15]*a[10]-b[3]*a[11]+b[4]*a[12]-b[10]*a[15];
        res[6] = b[6]*a[0]+b[11]*a[2]-b[13]*a[4]+b[0]*a[6]-b[15]*a[9]+b[2]*a[11]-b[4]*a[13]-b[9]*a[15];
        res[7] = b[7]*a[0]-b[12]*a[2]+b[13]*a[3]+b[0]*a[7]-b[15]*a[8]-b[2]*a[12]+b[3]*a[13]-b[8]*a[15];
        res[8] = b[8]*a[0]+b[14]*a[4]+b[0]*a[8]+b[4]*a[14];
        res[9] = b[9]*a[0]+b[14]*a[3]+b[0]*a[9]+b[3]*a[14];
        res[10] = b[10]*a[0]+b[14]*a[2]+b[0]*a[10]+b[2]*a[14];
        res[11] = b[11]*a[0]+b[15]*a[4]+b[0]*a[11]-b[4]*a[15];
        res[12] = b[12]*a[0]+b[15]*a[3]+b[0]*a[12]-b[3]*a[15];
        res[13] = b[13]*a[0]+b[15]*a[2]+b[0]*a[13]-b[2]*a[15];
        res[14] = b[14]*a[0]+b[0]*a[14];
        res[15] = b[15]*a[0]+b[0]*a[15];
        res
    };
);

// Add
// Multivector addition

define_binary_op_all!(
    Add,
    add;
    self: PGA3D, b: PGA3D, Output = PGA3D;
    [val val] => &self + &b;
    [ref val] =>  self + &b;
    [val ref] => &self +  b;
    [ref ref] => {
        let mut res = PGA3D::zero();
        let a = self;
        res[0] = a[0]+b[0];
        res[1] = a[1]+b[1];
        res[2] = a[2]+b[2];
        res[3] = a[3]+b[3];
        res[4] = a[4]+b[4];
        res[5] = a[5]+b[5];
        res[6] = a[6]+b[6];
        res[7] = a[7]+b[7];
        res[8] = a[8]+b[8];
        res[9] = a[9]+b[9];
        res[10] = a[10]+b[10];
        res[11] = a[11]+b[11];
        res[12] = a[12]+b[12];
        res[13] = a[13]+b[13];
        res[14] = a[14]+b[14];
        res[15] = a[15]+b[15];
        res
    };
);

// Sub
// Multivector subtraction

define_binary_op_all!(
    Sub,
    sub;
    self: PGA3D, b: PGA3D, Output = PGA3D;
    [val val] => &self - &b;
    [ref val] =>  self - &b;
    [val ref] => &self -  b;
    [ref ref] => {
        let mut res = PGA3D::zero();
        let a = self;
        res[0] = a[0]-b[0];
        res[1] = a[1]-b[1];
        res[2] = a[2]-b[2];
        res[3] = a[3]-b[3];
        res[4] = a[4]-b[4];
        res[5] = a[5]-b[5];
        res[6] = a[6]-b[6];
        res[7] = a[7]-b[7];
        res[8] = a[8]-b[8];
        res[9] = a[9]-b[9];
        res[10] = a[10]-b[10];
        res[11] = a[11]-b[11];
        res[12] = a[12]-b[12];
        res[13] = a[13]-b[13];
        res[14] = a[14]-b[14];
        res[15] = a[15]-b[15];
        res
    };
);

// smul
// scalar/multivector multiplication

define_binary_op_all!(
    Mul,
    mul;
    self: float_t, b: PGA3D, Output = PGA3D;
    [val val] => &self * &b;
    [ref val] =>  self * &b;
    [val ref] => &self *  b;
    [ref ref] => {
        let mut res = PGA3D::zero();
        let a = self;
        res[0] = a*b[0];
        res[1] = a*b[1];
        res[2] = a*b[2];
        res[3] = a*b[3];
        res[4] = a*b[4];
        res[5] = a*b[5];
        res[6] = a*b[6];
        res[7] = a*b[7];
        res[8] = a*b[8];
        res[9] = a*b[9];
        res[10] = a*b[10];
        res[11] = a*b[11];
        res[12] = a*b[12];
        res[13] = a*b[13];
        res[14] = a*b[14];
        res[15] = a*b[15];
        res
    };
);

// muls
// multivector/scalar multiplication

define_binary_op_all!(
    Mul,
    mul;
    self: PGA3D, b: float_t, Output = PGA3D;
    [val val] => &self * &b;
    [ref val] =>  self * &b;
    [val ref] => &self *  b;
    [ref ref] => {
        let mut res = PGA3D::zero();
        let a = self;
        res[0] = a[0]*b;
        res[1] = a[1]*b;
        res[2] = a[2]*b;
        res[3] = a[3]*b;
        res[4] = a[4]*b;
        res[5] = a[5]*b;
        res[6] = a[6]*b;
        res[7] = a[7]*b;
        res[8] = a[8]*b;
        res[9] = a[9]*b;
        res[10] = a[10]*b;
        res[11] = a[11]*b;
        res[12] = a[12]*b;
        res[13] = a[13]*b;
        res[14] = a[14]*b;
        res[15] = a[15]*b;
        res
    };
);

// sadd
// scalar/multivector addition

define_binary_op_all!(
    Add,
    add;
    self: float_t, b: PGA3D, Output = PGA3D;
    [val val] => &self + &b;
    [ref val] =>  self + &b;
    [val ref] => &self +  b;
    [ref ref] => {
        let mut res = PGA3D::zero();
        let a = self;
        res[0] = a+b[0];
        res[1] = b[1];
        res[2] = b[2];
        res[3] = b[3];
        res[4] = b[4];
        res[5] = b[5];
        res[6] = b[6];
        res[7] = b[7];
        res[8] = b[8];
        res[9] = b[9];
        res[10] = b[10];
        res[11] = b[11];
        res[12] = b[12];
        res[13] = b[13];
        res[14] = b[14];
        res[15] = b[15];
        res
    };
);

// adds
// multivector/scalar addition

define_binary_op_all!(
    Add,
    add;
    self: PGA3D, b: float_t, Output = PGA3D;
    [val val] => &self + &b;
    [ref val] =>  self + &b;
    [val ref] => &self +  b;
    [ref ref] => {
        let mut res = PGA3D::zero();
        let a = self;
        res[0] = a[0]+b;
        res[1] = a[1];
        res[2] = a[2];
        res[3] = a[3];
        res[4] = a[4];
        res[5] = a[5];
        res[6] = a[6];
        res[7] = a[7];
        res[8] = a[8];
        res[9] = a[9];
        res[10] = a[10];
        res[11] = a[11];
        res[12] = a[12];
        res[13] = a[13];
        res[14] = a[14];
        res[15] = a[15];
        res
    };
);

// ssub
// scalar/multivector subtraction

define_binary_op_all!(
    Sub,
    sub;
    self: float_t, b: PGA3D, Output = PGA3D;
    [val val] => &self - &b;
    [ref val] =>  self - &b;
    [val ref] => &self -  b;
    [ref ref] => {
        let mut res = PGA3D::zero();
        let a = self;
        res[0] = a-b[0];
        res[1] = -b[1];
        res[2] = -b[2];
        res[3] = -b[3];
        res[4] = -b[4];
        res[5] = -b[5];
        res[6] = -b[6];
        res[7] = -b[7];
        res[8] = -b[8];
        res[9] = -b[9];
        res[10] = -b[10];
        res[11] = -b[11];
        res[12] = -b[12];
        res[13] = -b[13];
        res[14] = -b[14];
        res[15] = -b[15];
        res
    };
);

// subs
// multivector/scalar subtraction

define_binary_op_all!(
    Sub,
    sub;
    self: PGA3D, b: float_t, Output = PGA3D;
    [val val] => &self - &b;
    [ref val] =>  self - &b;
    [val ref] => &self -  b;
    [ref ref] => {
        let mut res = PGA3D::zero();
        let a = self;
        res[0] = a[0]-b;
        res[1] = a[1];
        res[2] = a[2];
        res[3] = a[3];
        res[4] = a[4];
        res[5] = a[5];
        res[6] = a[6];
        res[7] = a[7];
        res[8] = a[8];
        res[9] = a[9];
        res[10] = a[10];
        res[11] = a[11];
        res[12] = a[12];
        res[13] = a[13];
        res[14] = a[14];
        res[15] = a[15];
        res
    };
);

impl PGA3D {
    pub fn norm(&self) -> float_t {
        let scalar_part = (self * self.Conjugate())[0];

        scalar_part.abs().sqrt()
    }

    pub fn inorm(&self) -> float_t {
        self.Dual().norm()
    }

    pub fn normalized(&self) -> Self {
        self * (1.0 / self.norm())
    }

    // A rotor (Euclidean line) and translator (Ideal line)
    pub fn rotor(angle: float_t, line: &Self) -> Self {
        (angle / 2.0).cos() + (angle / 2.0).sin() * line.normalized()
    }

    pub fn translator(dist: float_t, line: &Self) -> Self {
        1.0 + dist / 2.0 * line
    }

    // A plane is defined using its homogenous equation ax + by + cz + d = 0
    pub fn plane(a: float_t, b: float_t, c: float_t, d: float_t) -> Self {
        a * Self::e1() + b * Self::e2() + c * Self::e3() + d * Self::e0()
    }

    // A point is just a homogeneous point, euclidean coordinates plus the origin
    pub const fn point(x: float_t, y: float_t, z: float_t) -> Self {
        // Self::e123() + x * Self::e032() + y * Self::e013() + z * Self::e021()
        PGA3D {
            mvec: [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, z, y, x, 1.0, 0.0,
            ],
        }
    }

    // for our toy problem (generate points on the surface of a torus)
    // we start with a function that generates motors.
    // circle(t) with t going from 0 to 1.
    pub fn circle(t: float_t, radius: float_t, line: &Self) -> Self {
        Self::rotor(t * 2.0 * PI, line) * Self::translator(radius, &(Self::e1() * Self::e0()))
    }

    // a torus is now the product of two circles.
    pub fn torus(s: float_t, t: float_t, r1: float_t, l1: &Self, r2: float_t, l2: &Self) -> Self {
        Self::circle(s, r2, l2) * Self::circle(t, r1, l1)
    }

    // and to sample its points we simply sandwich the origin ..
    pub fn point_on_torus(s: float_t, t: float_t) -> Self {
        let to = &Self::torus(s, t, 0.25, &Self::e12(), 0.6, &Self::e31());

        to * Self::e123() * to.Reverse()
    }
}

// Extensions based on https://enki.ws/ganja.js/examples/pga_dyn.html
impl PGA3D {
    pub fn normalize_motor_in_place(&mut self) {
        let m = &mut self.mvec;
        // R = r0 + r1 e01 + r2 e02 + r3 e03 + r4 e12 + r5 e31 + r6 e23 + r7 e0123
        let s = (m[0] * m[0] + m[4] * m[4] + m[5] * m[5] + m[6] * m[6])
            .sqrt()
            .recip();
        let d = (m[7] * m[0] - (m[1] * m[6] + m[2] * m[5] + m[3] * m[4])) * s * s;
        for element in &mut *m {
            *element *= s;
        }
        m[1] += m[6] * d;
        m[2] += m[5] * d;
        m[3] += m[4] * d;
        m[7] -= m[0] * d;
    }

    pub const fn from_elements(
        one: float_t,
        e0: float_t,
        e1: float_t,
        e2: float_t,
        e3: float_t,
        e01: float_t,
        e02: float_t,
        e03: float_t,
        e12: float_t,
        e31: float_t,
        e23: float_t,
        e021: float_t,
        e013: float_t,
        e032: float_t,
        e123: float_t,
        e0123: float_t,
    ) -> Self {
        PGA3D {
            mvec: [
                one, e0, e1, e2, e3, e01, e02, e03, e12, e31, e23, e021, e013, e032, e123, e0123,
            ],
        }
    }
}

fn main() {
    // Elements of the even subalgebra (scalar + bivector + pss) of unit length are motors
    let rot = &PGA3D::rotor(PI / 2.0, &(PGA3D::e1() * PGA3D::e2()));

    // The outer product ^ is the MEET. Here we intersect the yz (x=0) and xz (y=0) planes.
    let ax_z = &(PGA3D::e1() ^ PGA3D::e2());

    // line and plane meet in point. We intersect the line along the z-axis (x=0,y=0) with the xy (z=0) plane.
    let orig = &(ax_z ^ PGA3D::e3());

    // We can also easily create points and join them into a line using the regressive (vee, &) product.
    let px = &PGA3D::point(1.0, 0.0, 0.0);
    let line = &(orig & px);

    // Lets also create the plane with equation 2x + z - 3 = 0
    let p = &PGA3D::plane(2.0, 0.0, 1.0, -3.0);

    // rotations work on all elements
    let rotated_plane = rot * p * rot.Reverse();
    let rotated_line = rot * line * rot.Reverse();
    let rotated_point = rot * px * rot.Reverse();

    // See the 3D PGA Cheat sheet for a huge collection of useful formulas
    let point_on_plane = &((p | px) * p);

    // Some output
    println!("a point       : {}", px);
    println!("a line        : {}", line);
    println!("a plane       : {}", p);
    println!("a rotor       : {}", rot);
    println!("rotated line  : {}", rotated_line);
    println!("rotated point : {}", rotated_point);
    println!("rotated plane : {}", rotated_plane);
    println!("point on plane: {}", point_on_plane.normalized());
    println!("point on torus: {}", PGA3D::point_on_torus(0.0, 0.0));
    println!("{}", PGA3D::e0() - 1.0);
    println!("{}", 1.0 - PGA3D::e0());
}
