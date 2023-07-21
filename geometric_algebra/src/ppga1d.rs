#![allow(clippy::assign_op_pattern)]
use crate::{simd::*, *};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy)]
struct ScalarGroups {
    /// 1
    g0: f32,
}

#[derive(Clone, Copy)]
pub union Scalar {
    groups: ScalarGroups,
    /// 1
    elements: [f32; 1],
}

impl Scalar {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(element0: f32) -> Self {
        Self { elements: [element0] }
    }
    pub const fn from_groups(g0: f32) -> Self {
        Self { groups: ScalarGroups { g0 } }
    }
    #[inline(always)]
    pub fn group0(&self) -> f32 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut f32 {
        unsafe { &mut self.groups.g0 }
    }
}

const SCALAR_INDEX_REMAP: [usize; 1] = [0];

impl std::ops::Index<usize> for Scalar {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[SCALAR_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for Scalar {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[SCALAR_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<Scalar> for [f32; 1] {
    fn from(vector: Scalar) -> Self {
        unsafe { [vector.elements[0]] }
    }
}

impl std::convert::From<[f32; 1]> for Scalar {
    fn from(array: [f32; 1]) -> Self {
        Self { elements: [array[0]] }
    }
}

impl std::fmt::Debug for Scalar {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("Scalar")
            .field("1", &self[0])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct DualNumberGroups {
    /// 1, e01
    g0: Simd32x2,
}

#[derive(Clone, Copy)]
pub union DualNumber {
    groups: DualNumberGroups,
    /// 1, e01, 0, 0
    elements: [f32; 4],
}

impl DualNumber {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(element0: f32, element1: f32) -> Self {
        Self { elements: [element0, element1, 0.0, 0.0] }
    }
    pub const fn from_groups(g0: Simd32x2) -> Self {
        Self { groups: DualNumberGroups { g0 } }
    }
    #[inline(always)]
    pub fn group0(&self) -> Simd32x2 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Simd32x2 {
        unsafe { &mut self.groups.g0 }
    }
}

const DUALNUMBER_INDEX_REMAP: [usize; 2] = [0, 1];

impl std::ops::Index<usize> for DualNumber {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[DUALNUMBER_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for DualNumber {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[DUALNUMBER_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<DualNumber> for [f32; 2] {
    fn from(vector: DualNumber) -> Self {
        unsafe { [vector.elements[0], vector.elements[1]] }
    }
}

impl std::convert::From<[f32; 2]> for DualNumber {
    fn from(array: [f32; 2]) -> Self {
        Self { elements: [array[0], array[1], 0.0, 0.0] }
    }
}

impl std::fmt::Debug for DualNumber {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("DualNumber")
            .field("1", &self[0])
            .field("e01", &self[1])
            .finish()
    }
}

impl Zero for Scalar {
    fn zero() -> Self {
        Scalar { groups: ScalarGroups { g0: 0.0 } }
    }
}

impl One for Scalar {
    fn one() -> Self {
        Scalar { groups: ScalarGroups { g0: 1.0 } }
    }
}

impl Neg for Scalar {
    type Output = Scalar;

    fn neg(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * -1.0 } }
    }
}

impl Automorphism for Scalar {
    type Output = Scalar;

    fn automorphism(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() } }
    }
}

impl Reversal for Scalar {
    type Output = Scalar;

    fn reversal(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() } }
    }
}

impl Conjugation for Scalar {
    type Output = Scalar;

    fn conjugation(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() } }
    }
}

impl Add<Scalar> for Scalar {
    type Output = Scalar;

    fn add(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() + other.group0() } }
    }
}

impl AddAssign<Scalar> for Scalar {
    fn add_assign(&mut self, other: Scalar) {
        *self = (*self).add(other);
    }
}

impl Sub<Scalar> for Scalar {
    type Output = Scalar;

    fn sub(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() - other.group0() } }
    }
}

impl SubAssign<Scalar> for Scalar {
    fn sub_assign(&mut self, other: Scalar) {
        *self = (*self).sub(other);
    }
}

impl Mul<Scalar> for Scalar {
    type Output = Scalar;

    fn mul(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl MulAssign<Scalar> for Scalar {
    fn mul_assign(&mut self, other: Scalar) {
        *self = (*self).mul(other);
    }
}

impl Div<Scalar> for Scalar {
    type Output = Scalar;

    fn div(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * 1.0 / other.group0() * 1.0 } }
    }
}

impl DivAssign<Scalar> for Scalar {
    fn div_assign(&mut self, other: Scalar) {
        *self = (*self).div(other);
    }
}

impl GeometricProduct<Scalar> for Scalar {
    type Output = Scalar;

    fn geometric_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl OuterProduct<Scalar> for Scalar {
    type Output = Scalar;

    fn outer_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl InnerProduct<Scalar> for Scalar {
    type Output = Scalar;

    fn inner_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl LeftContraction<Scalar> for Scalar {
    type Output = Scalar;

    fn left_contraction(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl RightContraction<Scalar> for Scalar {
    type Output = Scalar;

    fn right_contraction(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl ScalarProduct<Scalar> for Scalar {
    type Output = Scalar;

    fn scalar_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl Add<DualNumber> for Scalar {
    type Output = DualNumber;

    fn add(self, other: DualNumber) -> DualNumber {
        DualNumber { groups: DualNumberGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([1.0, 0.0]) + other.group0() } }
    }
}

impl Sub<DualNumber> for Scalar {
    type Output = DualNumber;

    fn sub(self, other: DualNumber) -> DualNumber {
        DualNumber { groups: DualNumberGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([1.0, 0.0]) - other.group0() } }
    }
}

impl GeometricProduct<DualNumber> for Scalar {
    type Output = DualNumber;

    fn geometric_product(self, other: DualNumber) -> DualNumber {
        DualNumber { groups: DualNumberGroups { g0: Simd32x2::from(self.group0()) * other.group0() } }
    }
}

impl RegressiveProduct<DualNumber> for Scalar {
    type Output = Scalar;

    fn regressive_product(self, other: DualNumber) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[1] } }
    }
}

impl OuterProduct<DualNumber> for Scalar {
    type Output = DualNumber;

    fn outer_product(self, other: DualNumber) -> DualNumber {
        DualNumber { groups: DualNumberGroups { g0: Simd32x2::from(self.group0()) * other.group0() } }
    }
}

impl InnerProduct<DualNumber> for Scalar {
    type Output = DualNumber;

    fn inner_product(self, other: DualNumber) -> DualNumber {
        DualNumber { groups: DualNumberGroups { g0: Simd32x2::from(self.group0()) * other.group0() } }
    }
}

impl LeftContraction<DualNumber> for Scalar {
    type Output = DualNumber;

    fn left_contraction(self, other: DualNumber) -> DualNumber {
        DualNumber { groups: DualNumberGroups { g0: Simd32x2::from(self.group0()) * other.group0() } }
    }
}

impl RightContraction<DualNumber> for Scalar {
    type Output = Scalar;

    fn right_contraction(self, other: DualNumber) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl ScalarProduct<DualNumber> for Scalar {
    type Output = Scalar;

    fn scalar_product(self, other: DualNumber) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl SquaredMagnitude for Scalar {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for Scalar {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl Scale for Scalar {
    type Output = Scalar;

    fn scale(self, other: f32) -> Scalar {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Signum for Scalar {
    type Output = Scalar;

    fn signum(self) -> Scalar {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.magnitude().group0() } })
    }
}

impl Inverse for Scalar {
    type Output = Scalar;

    fn inverse(self) -> Scalar {
        self.reversal().geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.squared_magnitude().group0() } })
    }
}

impl Zero for DualNumber {
    fn zero() -> Self {
        DualNumber { groups: DualNumberGroups { g0: Simd32x2::from(0.0) } }
    }
}

impl One for DualNumber {
    fn one() -> Self {
        DualNumber { groups: DualNumberGroups { g0: Simd32x2::from([1.0, 0.0]) } }
    }
}

impl Neg for DualNumber {
    type Output = DualNumber;

    fn neg(self) -> DualNumber {
        DualNumber { groups: DualNumberGroups { g0: self.group0() * Simd32x2::from(-1.0) } }
    }
}

impl Automorphism for DualNumber {
    type Output = DualNumber;

    fn automorphism(self) -> DualNumber {
        DualNumber { groups: DualNumberGroups { g0: self.group0() } }
    }
}

impl Reversal for DualNumber {
    type Output = DualNumber;

    fn reversal(self) -> DualNumber {
        DualNumber { groups: DualNumberGroups { g0: self.group0() * Simd32x2::from([1.0, -1.0]) } }
    }
}

impl Conjugation for DualNumber {
    type Output = DualNumber;

    fn conjugation(self) -> DualNumber {
        DualNumber { groups: DualNumberGroups { g0: self.group0() * Simd32x2::from([1.0, -1.0]) } }
    }
}

impl Dual for DualNumber {
    type Output = DualNumber;

    fn dual(self) -> DualNumber {
        DualNumber { groups: DualNumberGroups { g0: swizzle!(self.group0(), 1, 0) } }
    }
}

impl Into<Scalar> for DualNumber {
    fn into(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] } }
    }
}

impl Add<Scalar> for DualNumber {
    type Output = DualNumber;

    fn add(self, other: Scalar) -> DualNumber {
        DualNumber { groups: DualNumberGroups { g0: self.group0() + Simd32x2::from(other.group0()) * Simd32x2::from([1.0, 0.0]) } }
    }
}

impl AddAssign<Scalar> for DualNumber {
    fn add_assign(&mut self, other: Scalar) {
        *self = (*self).add(other);
    }
}

impl Sub<Scalar> for DualNumber {
    type Output = DualNumber;

    fn sub(self, other: Scalar) -> DualNumber {
        DualNumber { groups: DualNumberGroups { g0: self.group0() - Simd32x2::from(other.group0()) * Simd32x2::from([1.0, 0.0]) } }
    }
}

impl SubAssign<Scalar> for DualNumber {
    fn sub_assign(&mut self, other: Scalar) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Scalar> for DualNumber {
    type Output = DualNumber;

    fn geometric_product(self, other: Scalar) -> DualNumber {
        DualNumber { groups: DualNumberGroups { g0: self.group0() * Simd32x2::from(other.group0()) } }
    }
}

impl RegressiveProduct<Scalar> for DualNumber {
    type Output = Scalar;

    fn regressive_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group0() } }
    }
}

impl OuterProduct<Scalar> for DualNumber {
    type Output = DualNumber;

    fn outer_product(self, other: Scalar) -> DualNumber {
        DualNumber { groups: DualNumberGroups { g0: self.group0() * Simd32x2::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for DualNumber {
    type Output = DualNumber;

    fn inner_product(self, other: Scalar) -> DualNumber {
        DualNumber { groups: DualNumberGroups { g0: self.group0() * Simd32x2::from(other.group0()) } }
    }
}

impl LeftContraction<Scalar> for DualNumber {
    type Output = Scalar;

    fn left_contraction(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl RightContraction<Scalar> for DualNumber {
    type Output = DualNumber;

    fn right_contraction(self, other: Scalar) -> DualNumber {
        DualNumber { groups: DualNumberGroups { g0: self.group0() * Simd32x2::from(other.group0()) } }
    }
}

impl ScalarProduct<Scalar> for DualNumber {
    type Output = Scalar;

    fn scalar_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl Add<DualNumber> for DualNumber {
    type Output = DualNumber;

    fn add(self, other: DualNumber) -> DualNumber {
        DualNumber { groups: DualNumberGroups { g0: self.group0() + other.group0() } }
    }
}

impl AddAssign<DualNumber> for DualNumber {
    fn add_assign(&mut self, other: DualNumber) {
        *self = (*self).add(other);
    }
}

impl Sub<DualNumber> for DualNumber {
    type Output = DualNumber;

    fn sub(self, other: DualNumber) -> DualNumber {
        DualNumber { groups: DualNumberGroups { g0: self.group0() - other.group0() } }
    }
}

impl SubAssign<DualNumber> for DualNumber {
    fn sub_assign(&mut self, other: DualNumber) {
        *self = (*self).sub(other);
    }
}

impl Mul<DualNumber> for DualNumber {
    type Output = DualNumber;

    fn mul(self, other: DualNumber) -> DualNumber {
        DualNumber { groups: DualNumberGroups { g0: self.group0() * other.group0() } }
    }
}

impl MulAssign<DualNumber> for DualNumber {
    fn mul_assign(&mut self, other: DualNumber) {
        *self = (*self).mul(other);
    }
}

impl Div<DualNumber> for DualNumber {
    type Output = DualNumber;

    fn div(self, other: DualNumber) -> DualNumber {
        DualNumber { groups: DualNumberGroups { g0: Simd32x2::from([self.group0()[0], self.group0()[1]]) * Simd32x2::from([1.0, 1.0]) / Simd32x2::from([other.group0()[0], other.group0()[1]]) * Simd32x2::from([1.0, 1.0]) } }
    }
}

impl DivAssign<DualNumber> for DualNumber {
    fn div_assign(&mut self, other: DualNumber) {
        *self = (*self).div(other);
    }
}

impl GeometricProduct<DualNumber> for DualNumber {
    type Output = DualNumber;

    fn geometric_product(self, other: DualNumber) -> DualNumber {
        DualNumber { groups: DualNumberGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() + self.group0() * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, 1.0]) } }
    }
}

impl RegressiveProduct<DualNumber> for DualNumber {
    type Output = DualNumber;

    fn regressive_product(self, other: DualNumber) -> DualNumber {
        DualNumber { groups: DualNumberGroups { g0: Simd32x2::from(self.group0()[1]) * other.group0() + Simd32x2::from(self.group0()[0]) * swizzle!(other.group0(), 1, 0) * Simd32x2::from([1.0, 0.0]) } }
    }
}

impl OuterProduct<DualNumber> for DualNumber {
    type Output = DualNumber;

    fn outer_product(self, other: DualNumber) -> DualNumber {
        DualNumber { groups: DualNumberGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() + self.group0() * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, 1.0]) } }
    }
}

impl InnerProduct<DualNumber> for DualNumber {
    type Output = DualNumber;

    fn inner_product(self, other: DualNumber) -> DualNumber {
        DualNumber { groups: DualNumberGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() + self.group0() * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, 1.0]) } }
    }
}

impl LeftContraction<DualNumber> for DualNumber {
    type Output = DualNumber;

    fn left_contraction(self, other: DualNumber) -> DualNumber {
        DualNumber { groups: DualNumberGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() } }
    }
}

impl RightContraction<DualNumber> for DualNumber {
    type Output = DualNumber;

    fn right_contraction(self, other: DualNumber) -> DualNumber {
        DualNumber { groups: DualNumberGroups { g0: self.group0() * Simd32x2::from(other.group0()[0]) } }
    }
}

impl ScalarProduct<DualNumber> for DualNumber {
    type Output = Scalar;

    fn scalar_product(self, other: DualNumber) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] } }
    }
}

impl SquaredMagnitude for DualNumber {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for DualNumber {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl Scale for DualNumber {
    type Output = DualNumber;

    fn scale(self, other: f32) -> DualNumber {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Signum for DualNumber {
    type Output = DualNumber;

    fn signum(self) -> DualNumber {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.magnitude().group0() } })
    }
}

impl Inverse for DualNumber {
    type Output = DualNumber;

    fn inverse(self) -> DualNumber {
        self.reversal().geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.squared_magnitude().group0() } })
    }
}

impl Powi for DualNumber {
    type Output = DualNumber;

    fn powi(self, exponent: isize) -> DualNumber {
        if exponent == 0 {
            return DualNumber::one();
        }
        let mut x: DualNumber = if exponent < 0 { self.inverse() } else { self };
        let mut y: DualNumber = DualNumber::one();
        let mut n: isize = exponent.abs();
        while 1 < n {
            if n & 1 == 1 {
                y = x.geometric_product(y);
            }
            x = x.geometric_product(x);
            n = n >> 1;
        }
        x.geometric_product(y)
    }
}

impl GeometricQuotient<DualNumber> for DualNumber {
    type Output = DualNumber;

    fn geometric_quotient(self, other: DualNumber) -> DualNumber {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<DualNumber> for DualNumber {
    type Output = DualNumber;

    fn transformation(self, other: DualNumber) -> DualNumber {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Scalar> for DualNumber {
    type Output = DualNumber;

    fn geometric_quotient(self, other: Scalar) -> DualNumber {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Scalar> for DualNumber {
    type Output = Scalar;

    fn transformation(self, other: Scalar) -> Scalar {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<DualNumber> for Scalar {
    type Output = DualNumber;

    fn geometric_quotient(self, other: DualNumber) -> DualNumber {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<DualNumber> for Scalar {
    type Output = DualNumber;

    fn transformation(self, other: DualNumber) -> DualNumber {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Powi for Scalar {
    type Output = Scalar;

    fn powi(self, exponent: isize) -> Scalar {
        if exponent == 0 {
            return Scalar::one();
        }
        let mut x: Scalar = if exponent < 0 { self.inverse() } else { self };
        let mut y: Scalar = Scalar::one();
        let mut n: isize = exponent.abs();
        while 1 < n {
            if n & 1 == 1 {
                y = x.geometric_product(y);
            }
            x = x.geometric_product(x);
            n = n >> 1;
        }
        x.geometric_product(y)
    }
}

impl GeometricQuotient<Scalar> for Scalar {
    type Output = Scalar;

    fn geometric_quotient(self, other: Scalar) -> Scalar {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Scalar> for Scalar {
    type Output = Scalar;

    fn transformation(self, other: Scalar) -> Scalar {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

