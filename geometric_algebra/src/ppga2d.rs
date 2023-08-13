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
struct MultiVectorGroups {
    /// 1, e12, e1, e2
    g0: Simd32x4,
    /// e0, e012, e01, -e02
    g1: Simd32x4,
}

#[derive(Clone, Copy)]
pub union MultiVector {
    groups: MultiVectorGroups,
    /// 1, e12, e1, e2, e0, e012, e01, -e02
    elements: [f32; 8],
}

impl MultiVector {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(element0: f32, element1: f32, element2: f32, element3: f32, element4: f32, element5: f32, element6: f32, element7: f32) -> Self {
        Self { elements: [element0, element1, element2, element3, element4, element5, element6, element7] }
    }
    pub const fn from_groups(g0: Simd32x4, g1: Simd32x4) -> Self {
        Self { groups: MultiVectorGroups { g0, g1 } }
    }
    #[inline(always)]
    pub fn group0(&self) -> Simd32x4 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g0 }
    }
    #[inline(always)]
    pub fn group1(&self) -> Simd32x4 {
        unsafe { self.groups.g1 }
    }
    #[inline(always)]
    pub fn group1_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g1 }
    }
}

const MULTIVECTOR_INDEX_REMAP: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

impl std::ops::Index<usize> for MultiVector {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[MULTIVECTOR_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for MultiVector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[MULTIVECTOR_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<MultiVector> for [f32; 8] {
    fn from(vector: MultiVector) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7]] }
    }
}

impl std::convert::From<[f32; 8]> for MultiVector {
    fn from(array: [f32; 8]) -> Self {
        Self { elements: [array[0], array[1], array[2], array[3], array[4], array[5], array[6], array[7]] }
    }
}

impl std::fmt::Debug for MultiVector {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("MultiVector")
            .field("1", &self[0])
            .field("e12", &self[1])
            .field("e1", &self[2])
            .field("e2", &self[3])
            .field("e0", &self[4])
            .field("e012", &self[5])
            .field("e01", &self[6])
            .field("-e02", &self[7])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct RotorGroups {
    /// 1, e12
    g0: Simd32x2,
}

#[derive(Clone, Copy)]
pub union Rotor {
    groups: RotorGroups,
    /// 1, e12, 0, 0
    elements: [f32; 4],
}

impl Rotor {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(element0: f32, element1: f32) -> Self {
        Self { elements: [element0, element1, 0.0, 0.0] }
    }
    pub const fn from_groups(g0: Simd32x2) -> Self {
        Self { groups: RotorGroups { g0 } }
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

const ROTOR_INDEX_REMAP: [usize; 2] = [0, 1];

impl std::ops::Index<usize> for Rotor {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ROTOR_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for Rotor {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ROTOR_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<Rotor> for [f32; 2] {
    fn from(vector: Rotor) -> Self {
        unsafe { [vector.elements[0], vector.elements[1]] }
    }
}

impl std::convert::From<[f32; 2]> for Rotor {
    fn from(array: [f32; 2]) -> Self {
        Self { elements: [array[0], array[1], 0.0, 0.0] }
    }
}

impl std::fmt::Debug for Rotor {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("Rotor")
            .field("1", &self[0])
            .field("e12", &self[1])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct PointGroups {
    /// e12, e01, -e02
    g0: Simd32x3,
}

#[derive(Clone, Copy)]
pub union Point {
    groups: PointGroups,
    /// e12, e01, -e02, 0
    elements: [f32; 4],
}

impl Point {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(element0: f32, element1: f32, element2: f32) -> Self {
        Self { elements: [element0, element1, element2, 0.0] }
    }
    pub const fn from_groups(g0: Simd32x3) -> Self {
        Self { groups: PointGroups { g0 } }
    }
    #[inline(always)]
    pub fn group0(&self) -> Simd32x3 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g0 }
    }
}

const POINT_INDEX_REMAP: [usize; 3] = [0, 1, 2];

impl std::ops::Index<usize> for Point {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[POINT_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for Point {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[POINT_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<Point> for [f32; 3] {
    fn from(vector: Point) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2]] }
    }
}

impl std::convert::From<[f32; 3]> for Point {
    fn from(array: [f32; 3]) -> Self {
        Self { elements: [array[0], array[1], array[2], 0.0] }
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("Point")
            .field("e12", &self[0])
            .field("e01", &self[1])
            .field("-e02", &self[2])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct IdealPointGroups {
    /// e01, -e02
    g0: Simd32x2,
}

#[derive(Clone, Copy)]
pub union IdealPoint {
    groups: IdealPointGroups,
    /// e01, -e02, 0, 0
    elements: [f32; 4],
}

impl IdealPoint {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(element0: f32, element1: f32) -> Self {
        Self { elements: [element0, element1, 0.0, 0.0] }
    }
    pub const fn from_groups(g0: Simd32x2) -> Self {
        Self { groups: IdealPointGroups { g0 } }
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

const IDEALPOINT_INDEX_REMAP: [usize; 2] = [0, 1];

impl std::ops::Index<usize> for IdealPoint {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[IDEALPOINT_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for IdealPoint {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[IDEALPOINT_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<IdealPoint> for [f32; 2] {
    fn from(vector: IdealPoint) -> Self {
        unsafe { [vector.elements[0], vector.elements[1]] }
    }
}

impl std::convert::From<[f32; 2]> for IdealPoint {
    fn from(array: [f32; 2]) -> Self {
        Self { elements: [array[0], array[1], 0.0, 0.0] }
    }
}

impl std::fmt::Debug for IdealPoint {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("IdealPoint")
            .field("e01", &self[0])
            .field("-e02", &self[1])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct PlaneGroups {
    /// e0, e2, e1
    g0: Simd32x3,
}

#[derive(Clone, Copy)]
pub union Plane {
    groups: PlaneGroups,
    /// e0, e2, e1, 0
    elements: [f32; 4],
}

impl Plane {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(element0: f32, element1: f32, element2: f32) -> Self {
        Self { elements: [element0, element1, element2, 0.0] }
    }
    pub const fn from_groups(g0: Simd32x3) -> Self {
        Self { groups: PlaneGroups { g0 } }
    }
    #[inline(always)]
    pub fn group0(&self) -> Simd32x3 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g0 }
    }
}

const PLANE_INDEX_REMAP: [usize; 3] = [0, 1, 2];

impl std::ops::Index<usize> for Plane {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[PLANE_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for Plane {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[PLANE_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<Plane> for [f32; 3] {
    fn from(vector: Plane) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2]] }
    }
}

impl std::convert::From<[f32; 3]> for Plane {
    fn from(array: [f32; 3]) -> Self {
        Self { elements: [array[0], array[1], array[2], 0.0] }
    }
}

impl std::fmt::Debug for Plane {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("Plane")
            .field("e0", &self[0])
            .field("e2", &self[1])
            .field("e1", &self[2])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct TranslatorGroups {
    /// 1, e01, -e02
    g0: Simd32x3,
}

#[derive(Clone, Copy)]
pub union Translator {
    groups: TranslatorGroups,
    /// 1, e01, -e02, 0
    elements: [f32; 4],
}

impl Translator {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(element0: f32, element1: f32, element2: f32) -> Self {
        Self { elements: [element0, element1, element2, 0.0] }
    }
    pub const fn from_groups(g0: Simd32x3) -> Self {
        Self { groups: TranslatorGroups { g0 } }
    }
    #[inline(always)]
    pub fn group0(&self) -> Simd32x3 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g0 }
    }
}

const TRANSLATOR_INDEX_REMAP: [usize; 3] = [0, 1, 2];

impl std::ops::Index<usize> for Translator {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[TRANSLATOR_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for Translator {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[TRANSLATOR_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<Translator> for [f32; 3] {
    fn from(vector: Translator) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2]] }
    }
}

impl std::convert::From<[f32; 3]> for Translator {
    fn from(array: [f32; 3]) -> Self {
        Self { elements: [array[0], array[1], array[2], 0.0] }
    }
}

impl std::fmt::Debug for Translator {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("Translator")
            .field("1", &self[0])
            .field("e01", &self[1])
            .field("-e02", &self[2])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct MotorGroups {
    /// 1, e12, e01, -e02
    g0: Simd32x4,
}

#[derive(Clone, Copy)]
pub union Motor {
    groups: MotorGroups,
    /// 1, e12, e01, -e02
    elements: [f32; 4],
}

impl Motor {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(element0: f32, element1: f32, element2: f32, element3: f32) -> Self {
        Self { elements: [element0, element1, element2, element3] }
    }
    pub const fn from_groups(g0: Simd32x4) -> Self {
        Self { groups: MotorGroups { g0 } }
    }
    #[inline(always)]
    pub fn group0(&self) -> Simd32x4 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g0 }
    }
}

const MOTOR_INDEX_REMAP: [usize; 4] = [0, 1, 2, 3];

impl std::ops::Index<usize> for Motor {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[MOTOR_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for Motor {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[MOTOR_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<Motor> for [f32; 4] {
    fn from(vector: Motor) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3]] }
    }
}

impl std::convert::From<[f32; 4]> for Motor {
    fn from(array: [f32; 4]) -> Self {
        Self { elements: [array[0], array[1], array[2], array[3]] }
    }
}

impl std::fmt::Debug for Motor {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("Motor")
            .field("1", &self[0])
            .field("e12", &self[1])
            .field("e01", &self[2])
            .field("-e02", &self[3])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct MotorDualGroups {
    /// e012, e0, e2, e1
    g0: Simd32x4,
}

#[derive(Clone, Copy)]
pub union MotorDual {
    groups: MotorDualGroups,
    /// e012, e0, e2, e1
    elements: [f32; 4],
}

impl MotorDual {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(element0: f32, element1: f32, element2: f32, element3: f32) -> Self {
        Self { elements: [element0, element1, element2, element3] }
    }
    pub const fn from_groups(g0: Simd32x4) -> Self {
        Self { groups: MotorDualGroups { g0 } }
    }
    #[inline(always)]
    pub fn group0(&self) -> Simd32x4 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g0 }
    }
}

const MOTORDUAL_INDEX_REMAP: [usize; 4] = [0, 1, 2, 3];

impl std::ops::Index<usize> for MotorDual {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[MOTORDUAL_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for MotorDual {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[MOTORDUAL_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<MotorDual> for [f32; 4] {
    fn from(vector: MotorDual) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3]] }
    }
}

impl std::convert::From<[f32; 4]> for MotorDual {
    fn from(array: [f32; 4]) -> Self {
        Self { elements: [array[0], array[1], array[2], array[3]] }
    }
}

impl std::fmt::Debug for MotorDual {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("MotorDual")
            .field("e012", &self[0])
            .field("e0", &self[1])
            .field("e2", &self[2])
            .field("e1", &self[3])
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

impl Add<MultiVector> for Scalar {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + other.group0(), g1: other.group1() } }
    }
}

impl Sub<MultiVector> for Scalar {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) - other.group0(), g1: Simd32x4::from(0.0) - other.group1() } }
    }
}

impl GeometricProduct<MultiVector> for Scalar {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl RegressiveProduct<MultiVector> for Scalar {
    type Output = Scalar;

    fn regressive_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group1()[1] } }
    }
}

impl OuterProduct<MultiVector> for Scalar {
    type Output = MultiVector;

    fn outer_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl InnerProduct<MultiVector> for Scalar {
    type Output = MultiVector;

    fn inner_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl LeftContraction<MultiVector> for Scalar {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl RightContraction<MultiVector> for Scalar {
    type Output = Scalar;

    fn right_contraction(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl ScalarProduct<MultiVector> for Scalar {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl Add<Rotor> for Scalar {
    type Output = Rotor;

    fn add(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([1.0, 0.0]) + other.group0() } }
    }
}

impl Sub<Rotor> for Scalar {
    type Output = Rotor;

    fn sub(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([1.0, 0.0]) - other.group0() } }
    }
}

impl GeometricProduct<Rotor> for Scalar {
    type Output = Rotor;

    fn geometric_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x2::from(self.group0()) * other.group0() } }
    }
}

impl OuterProduct<Rotor> for Scalar {
    type Output = Rotor;

    fn outer_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x2::from(self.group0()) * other.group0() } }
    }
}

impl InnerProduct<Rotor> for Scalar {
    type Output = Rotor;

    fn inner_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x2::from(self.group0()) * other.group0() } }
    }
}

impl LeftContraction<Rotor> for Scalar {
    type Output = Rotor;

    fn left_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x2::from(self.group0()) * other.group0() } }
    }
}

impl RightContraction<Rotor> for Scalar {
    type Output = Scalar;

    fn right_contraction(self, other: Rotor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl ScalarProduct<Rotor> for Scalar {
    type Output = Scalar;

    fn scalar_product(self, other: Rotor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl Add<Point> for Scalar {
    type Output = Motor;

    fn add(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl Sub<Point> for Scalar {
    type Output = Motor;

    fn sub(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) - Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl GeometricProduct<Point> for Scalar {
    type Output = Point;

    fn geometric_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl OuterProduct<Point> for Scalar {
    type Output = Point;

    fn outer_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl InnerProduct<Point> for Scalar {
    type Output = Point;

    fn inner_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl LeftContraction<Point> for Scalar {
    type Output = Point;

    fn left_contraction(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl Add<IdealPoint> for Scalar {
    type Output = Translator;

    fn add(self, other: IdealPoint) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from(self.group0()) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from([other.group0()[0], other.group0()[0], other.group0()[1]]) * Simd32x3::from([0.0, 1.0, 1.0]) } }
    }
}

impl Sub<IdealPoint> for Scalar {
    type Output = Translator;

    fn sub(self, other: IdealPoint) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from(self.group0()) * Simd32x3::from([1.0, 0.0, 0.0]) - Simd32x3::from([other.group0()[0], other.group0()[0], other.group0()[1]]) * Simd32x3::from([0.0, 1.0, 1.0]) } }
    }
}

impl GeometricProduct<IdealPoint> for Scalar {
    type Output = IdealPoint;

    fn geometric_product(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x2::from(self.group0()) * other.group0() } }
    }
}

impl OuterProduct<IdealPoint> for Scalar {
    type Output = IdealPoint;

    fn outer_product(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x2::from(self.group0()) * other.group0() } }
    }
}

impl InnerProduct<IdealPoint> for Scalar {
    type Output = IdealPoint;

    fn inner_product(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x2::from(self.group0()) * other.group0() } }
    }
}

impl LeftContraction<IdealPoint> for Scalar {
    type Output = IdealPoint;

    fn left_contraction(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x2::from(self.group0()) * other.group0() } }
    }
}

impl GeometricProduct<Plane> for Scalar {
    type Output = Plane;

    fn geometric_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl OuterProduct<Plane> for Scalar {
    type Output = Plane;

    fn outer_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl InnerProduct<Plane> for Scalar {
    type Output = Plane;

    fn inner_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl LeftContraction<Plane> for Scalar {
    type Output = Plane;

    fn left_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl Add<Translator> for Scalar {
    type Output = Translator;

    fn add(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from(self.group0()) * Simd32x3::from([1.0, 0.0, 0.0]) + other.group0() } }
    }
}

impl Sub<Translator> for Scalar {
    type Output = Translator;

    fn sub(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from(self.group0()) * Simd32x3::from([1.0, 0.0, 0.0]) - other.group0() } }
    }
}

impl GeometricProduct<Translator> for Scalar {
    type Output = Translator;

    fn geometric_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl OuterProduct<Translator> for Scalar {
    type Output = Translator;

    fn outer_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl InnerProduct<Translator> for Scalar {
    type Output = Translator;

    fn inner_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl LeftContraction<Translator> for Scalar {
    type Output = Translator;

    fn left_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl RightContraction<Translator> for Scalar {
    type Output = Scalar;

    fn right_contraction(self, other: Translator) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl ScalarProduct<Translator> for Scalar {
    type Output = Scalar;

    fn scalar_product(self, other: Translator) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl Add<Motor> for Scalar {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + other.group0() } }
    }
}

impl Sub<Motor> for Scalar {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) - other.group0() } }
    }
}

impl GeometricProduct<Motor> for Scalar {
    type Output = Motor;

    fn geometric_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl OuterProduct<Motor> for Scalar {
    type Output = Motor;

    fn outer_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl InnerProduct<Motor> for Scalar {
    type Output = Motor;

    fn inner_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl LeftContraction<Motor> for Scalar {
    type Output = Motor;

    fn left_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl RightContraction<Motor> for Scalar {
    type Output = Scalar;

    fn right_contraction(self, other: Motor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl ScalarProduct<Motor> for Scalar {
    type Output = Scalar;

    fn scalar_product(self, other: Motor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl GeometricProduct<MotorDual> for Scalar {
    type Output = MotorDual;

    fn geometric_product(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl RegressiveProduct<MotorDual> for Scalar {
    type Output = Scalar;

    fn regressive_product(self, other: MotorDual) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl OuterProduct<MotorDual> for Scalar {
    type Output = MotorDual;

    fn outer_product(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl InnerProduct<MotorDual> for Scalar {
    type Output = MotorDual;

    fn inner_product(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl LeftContraction<MotorDual> for Scalar {
    type Output = MotorDual;

    fn left_contraction(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
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

impl Zero for MultiVector {
    fn zero() -> Self {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(0.0), g1: Simd32x4::from(0.0) } }
    }
}

impl One for MultiVector {
    fn one() -> Self {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(0.0) } }
    }
}

impl Neg for MultiVector {
    type Output = MultiVector;

    fn neg(self) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x4::from(-1.0), g1: self.group1() * Simd32x4::from(-1.0) } }
    }
}

impl Automorphism for MultiVector {
    type Output = MultiVector;

    fn automorphism(self) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x4::from([1.0, 1.0, -1.0, -1.0]), g1: self.group1() * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) } }
    }
}

impl Reversal for MultiVector {
    type Output = MultiVector;

    fn reversal(self) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x4::from([1.0, -1.0, 1.0, 1.0]), g1: self.group1() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) } }
    }
}

impl Conjugation for MultiVector {
    type Output = MultiVector;

    fn conjugation(self) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]), g1: self.group1() * Simd32x4::from([-1.0, 1.0, -1.0, -1.0]) } }
    }
}

impl Dual for MultiVector {
    type Output = MultiVector;

    fn dual(self) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: swizzle!(self.group1(), 1, 0, 3, 2), g1: swizzle!(self.group0(), 1, 0, 3, 2) } }
    }
}

impl Into<Scalar> for MultiVector {
    fn into(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] } }
    }
}

impl Add<Scalar> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Scalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + Simd32x4::from(other.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: self.group1() } }
    }
}

impl AddAssign<Scalar> for MultiVector {
    fn add_assign(&mut self, other: Scalar) {
        *self = (*self).add(other);
    }
}

impl Sub<Scalar> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Scalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - Simd32x4::from(other.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: self.group1() } }
    }
}

impl SubAssign<Scalar> for MultiVector {
    fn sub_assign(&mut self, other: Scalar) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Scalar> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Scalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl RegressiveProduct<Scalar> for MultiVector {
    type Output = Scalar;

    fn regressive_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group1()[1] * other.group0() } }
    }
}

impl OuterProduct<Scalar> for MultiVector {
    type Output = MultiVector;

    fn outer_product(self, other: Scalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for MultiVector {
    type Output = MultiVector;

    fn inner_product(self, other: Scalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl LeftContraction<Scalar> for MultiVector {
    type Output = Scalar;

    fn left_contraction(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl RightContraction<Scalar> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: Scalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl ScalarProduct<Scalar> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl Add<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + other.group0(), g1: self.group1() + other.group1() } }
    }
}

impl AddAssign<MultiVector> for MultiVector {
    fn add_assign(&mut self, other: MultiVector) {
        *self = (*self).add(other);
    }
}

impl Sub<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - other.group0(), g1: self.group1() - other.group1() } }
    }
}

impl SubAssign<MultiVector> for MultiVector {
    fn sub_assign(&mut self, other: MultiVector) {
        *self = (*self).sub(other);
    }
}

impl Mul<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn mul(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * other.group0(), g1: self.group1() * other.group1() } }
    }
}

impl MulAssign<MultiVector> for MultiVector {
    fn mul_assign(&mut self, other: MultiVector) {
        *self = (*self).mul(other);
    }
}

impl Div<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn div(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl DivAssign<MultiVector> for MultiVector {
    fn div_assign(&mut self, other: MultiVector) {
        *self = (*self).div(other);
    }
}

impl GeometricProduct<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 2, 3, 0, 1) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group1(), 3, 2, 1, 0) + Simd32x4::from(self.group1()[0]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl RegressiveProduct<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn regressive_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[1]) * other.group1() * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 3, 3, 1, 3) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group1(), 2, 2, 2, 1) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * other.group0() + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 1, 3) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 2, 2, 2, 1) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group1(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.group1()[1]) * other.group1() + Simd32x4::from(self.group1()[2]) * swizzle!(other.group1(), 3, 3, 1, 3) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group1(), 2, 2, 2, 1) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group1(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl OuterProduct<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn outer_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 0, 3) * Simd32x4::from([0.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 2, 2, 2, 0) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + swizzle!(self.group0(), 0, 1, 0, 0) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 3, 3, 0, 3) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group1(), 2, 2, 2, 0) * Simd32x4::from([0.0, 1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 0, 3) * Simd32x4::from([0.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 2, 2, 2, 0) * Simd32x4::from([0.0, 1.0, 0.0, 1.0]) + swizzle!(self.group0(), 0, 1, 0, 0) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn inner_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 1, 0) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + swizzle!(self.group0(), 2, 0, 2, 2) * swizzle!(other.group0(), 2, 0, 0, 1) * Simd32x4::from([1.0, 0.0, 1.0, 1.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 2, 2, 2, 1) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group1(), 3, 3, 1, 3) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group1(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl LeftContraction<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 2, 1) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 1, 3) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 2, 2, 2, 1) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group1(), 3, 3, 1, 3) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group1(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl RightContraction<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl ScalarProduct<MultiVector> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group0()[3] } }
    }
}

impl Into<Rotor> for MultiVector {
    fn into(self) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x2::from([self.group0()[0], self.group0()[1]]) } }
    }
}

impl Add<Rotor> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]), g1: self.group1() } }
    }
}

impl AddAssign<Rotor> for MultiVector {
    fn add_assign(&mut self, other: Rotor) {
        *self = (*self).add(other);
    }
}

impl Sub<Rotor> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]), g1: self.group1() } }
    }
}

impl SubAssign<Rotor> for MultiVector {
    fn sub_assign(&mut self, other: Rotor) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Rotor> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + swizzle!(self.group0(), 0, 0, 2, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[1]]), g1: Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) + swizzle!(self.group1(), 0, 0, 2, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<Rotor> for MultiVector {
    type Output = MultiVector;

    fn outer_product(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + swizzle!(self.group0(), 0, 0, 2, 3) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]), g1: Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + swizzle!(self.group1(), 0, 0, 2, 3) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) } }
    }
}

impl InnerProduct<Rotor> for MultiVector {
    type Output = MultiVector;

    fn inner_product(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + swizzle!(self.group0(), 0, 0, 2, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[1]]), g1: Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + swizzle!(self.group1(), 0, 0, 2, 3) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl RightContraction<Rotor> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + swizzle!(self.group0(), 0, 0, 2, 3) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 1.0, 1.0]), g1: Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + swizzle!(self.group1(), 0, 0, 2, 3) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl ScalarProduct<Rotor> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: Rotor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] } }
    }
}

impl Into<Point> for MultiVector {
    fn into(self) -> Point {
        Point { groups: PointGroups { g0: Simd32x3::from([self.group0()[1], self.group1()[2], self.group1()[3]]) } }
    }
}

impl Add<Point> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Point) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]), g1: self.group1() + Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl AddAssign<Point> for MultiVector {
    fn add_assign(&mut self, other: Point) {
        *self = (*self).add(other);
    }
}

impl Sub<Point> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Point) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]), g1: self.group1() - Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl SubAssign<Point> for MultiVector {
    fn sub_assign(&mut self, other: Point) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Point> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Point) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: swizzle!(self.group0(), 1, 0, 3, 2) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[2], other.group0()[1]]) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[2], other.group0()[1], other.group0()[2], other.group0()[2]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + swizzle!(self.group0(), 2, 2, 0, 0) * Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl ScalarProduct<Point> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: Point) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[1] * other.group0()[0] } }
    }
}

impl Into<IdealPoint> for MultiVector {
    fn into(self) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x2::from([self.group1()[2], self.group1()[3]]) } }
    }
}

impl Add<IdealPoint> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: IdealPoint) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0(), g1: self.group1() + Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[0], other.group0()[1]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl AddAssign<IdealPoint> for MultiVector {
    fn add_assign(&mut self, other: IdealPoint) {
        *self = (*self).add(other);
    }
}

impl Sub<IdealPoint> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: IdealPoint) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0(), g1: self.group1() - Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[0], other.group0()[1]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl SubAssign<IdealPoint> for MultiVector {
    fn sub_assign(&mut self, other: IdealPoint) {
        *self = (*self).sub(other);
    }
}

impl Into<Plane> for MultiVector {
    fn into(self) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from([self.group1()[0], self.group0()[3], self.group0()[2]]) } }
    }
}

impl Add<Plane> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]), g1: self.group1() + Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl AddAssign<Plane> for MultiVector {
    fn add_assign(&mut self, other: Plane) {
        *self = (*self).add(other);
    }
}

impl Sub<Plane> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]), g1: self.group1() - Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl SubAssign<Plane> for MultiVector {
    fn sub_assign(&mut self, other: Plane) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Plane> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[1]]) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + swizzle!(self.group0(), 2, 2, 0, 0) * Simd32x4::from([other.group0()[2], other.group0()[1], other.group0()[2], other.group0()[1]]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[2], other.group0()[1]]) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[2], other.group0()[1], other.group0()[2], other.group0()[2]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + self.group0() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl ScalarProduct<Plane> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group0()[1] } }
    }
}

impl Into<Translator> for MultiVector {
    fn into(self) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from([self.group0()[0], self.group1()[2], self.group1()[3]]) } }
    }
}

impl Add<Translator> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: self.group1() + Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl AddAssign<Translator> for MultiVector {
    fn add_assign(&mut self, other: Translator) {
        *self = (*self).add(other);
    }
}

impl Sub<Translator> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: self.group1() - Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl SubAssign<Translator> for MultiVector {
    fn sub_assign(&mut self, other: Translator) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Translator> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[2], other.group0()[1]]) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[2], other.group0()[1], other.group0()[2], other.group0()[2]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 2, 2, 0, 0) * Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl OuterProduct<Translator> for MultiVector {
    type Output = MultiVector;

    fn outer_product(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group1()[0], self.group0()[2], self.group0()[0], self.group0()[0]]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group0()[2]]) } }
    }
}

impl InnerProduct<Translator> for MultiVector {
    type Output = MultiVector;

    fn inner_product(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group0()[2], self.group1()[1], self.group0()[0], self.group0()[0]]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl RightContraction<Translator> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: self.group1() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl ScalarProduct<Translator> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: Translator) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] } }
    }
}

impl Into<Motor> for MultiVector {
    fn into(self) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group1()[2], self.group1()[3]]) } }
    }
}

impl Add<Motor> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + swizzle!(other.group0(), 0, 1, 0, 0) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]), g1: self.group1() + swizzle!(other.group0(), 0, 0, 2, 3) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl AddAssign<Motor> for MultiVector {
    fn add_assign(&mut self, other: Motor) {
        *self = (*self).add(other);
    }
}

impl Sub<Motor> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - swizzle!(other.group0(), 0, 1, 0, 0) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]), g1: self.group1() - swizzle!(other.group0(), 0, 0, 2, 3) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl SubAssign<Motor> for MultiVector {
    fn sub_assign(&mut self, other: Motor) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Motor> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 1, 1, 1, 0) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + swizzle!(self.group0(), 0, 0, 2, 2) * swizzle!(other.group0(), 0, 1, 0, 1), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 2, 3, 3) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 0, 1, 0, 0) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 0, 0, 0, 1) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 1, 1, 1, 0) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) + swizzle!(self.group0(), 2, 2, 0, 0) * swizzle!(other.group0(), 2, 3, 2, 3) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl OuterProduct<Motor> for MultiVector {
    type Output = MultiVector;

    fn outer_product(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + swizzle!(self.group0(), 0, 0, 2, 3) * swizzle!(other.group0(), 0, 1, 0, 0), g1: Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 0, 1, 0, 0) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 0, 2, 0, 0) * swizzle!(other.group0(), 0, 3, 2, 3) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl InnerProduct<Motor> for MultiVector {
    type Output = MultiVector;

    fn inner_product(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 1, 1, 1, 0) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + swizzle!(self.group0(), 0, 0, 2, 2) * swizzle!(other.group0(), 0, 1, 0, 1), g1: Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 2, 0, 0, 0) * swizzle!(other.group0(), 2, 0, 2, 3) * Simd32x4::from([-1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl RightContraction<Motor> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + swizzle!(self.group0(), 0, 0, 2, 3) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 1.0, 1.0]), g1: Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + swizzle!(self.group1(), 0, 0, 2, 3) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl ScalarProduct<Motor> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: Motor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] } }
    }
}

impl Into<MotorDual> for MultiVector {
    fn into(self) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from([self.group1()[1], self.group1()[0], self.group0()[3], self.group0()[2]]) } }
    }
}

impl Add<MotorDual> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: MotorDual) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + swizzle!(other.group0(), 0, 0, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]), g1: self.group1() + swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl AddAssign<MotorDual> for MultiVector {
    fn add_assign(&mut self, other: MotorDual) {
        *self = (*self).add(other);
    }
}

impl Sub<MotorDual> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: MotorDual) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - swizzle!(other.group0(), 0, 0, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]), g1: self.group1() - swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl SubAssign<MotorDual> for MultiVector {
    fn sub_assign(&mut self, other: MotorDual) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<MotorDual> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: MotorDual) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 2, 3) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 2, 3, 2, 2) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + swizzle!(self.group0(), 2, 2, 0, 0) * swizzle!(other.group0(), 3, 2, 3, 2), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 0, 1, 0, 0) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 0, 0, 0, 1) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 2, 3) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 2, 3, 3) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 2, 3, 2, 2) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + swizzle!(self.group0(), 0, 0, 2, 2) * swizzle!(other.group0(), 1, 0, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl RegressiveProduct<MotorDual> for MultiVector {
    type Output = MultiVector;

    fn regressive_product(self, other: MotorDual) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 0, 0, 2, 3) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 1.0, 1.0]), g1: Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + swizzle!(self.group1(), 0, 0, 2, 3) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl ScalarProduct<MotorDual> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: MotorDual) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[2] * other.group0()[3] + self.group0()[3] * other.group0()[2] } }
    }
}

impl SquaredMagnitude for MultiVector {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for MultiVector {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl Scale for MultiVector {
    type Output = MultiVector;

    fn scale(self, other: f32) -> MultiVector {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Signum for MultiVector {
    type Output = MultiVector;

    fn signum(self) -> MultiVector {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.magnitude().group0() } })
    }
}

impl Inverse for MultiVector {
    type Output = MultiVector;

    fn inverse(self) -> MultiVector {
        self.reversal().geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.squared_magnitude().group0() } })
    }
}

impl Zero for Rotor {
    fn zero() -> Self {
        Rotor { groups: RotorGroups { g0: Simd32x2::from(0.0) } }
    }
}

impl One for Rotor {
    fn one() -> Self {
        Rotor { groups: RotorGroups { g0: Simd32x2::from([1.0, 0.0]) } }
    }
}

impl Neg for Rotor {
    type Output = Rotor;

    fn neg(self) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x2::from(-1.0) } }
    }
}

impl Automorphism for Rotor {
    type Output = Rotor;

    fn automorphism(self) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() } }
    }
}

impl Reversal for Rotor {
    type Output = Rotor;

    fn reversal(self) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x2::from([1.0, -1.0]) } }
    }
}

impl Conjugation for Rotor {
    type Output = Rotor;

    fn conjugation(self) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x2::from([1.0, -1.0]) } }
    }
}

impl Into<Scalar> for Rotor {
    fn into(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] } }
    }
}

impl Add<Scalar> for Rotor {
    type Output = Rotor;

    fn add(self, other: Scalar) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() + Simd32x2::from(other.group0()) * Simd32x2::from([1.0, 0.0]) } }
    }
}

impl AddAssign<Scalar> for Rotor {
    fn add_assign(&mut self, other: Scalar) {
        *self = (*self).add(other);
    }
}

impl Sub<Scalar> for Rotor {
    type Output = Rotor;

    fn sub(self, other: Scalar) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() - Simd32x2::from(other.group0()) * Simd32x2::from([1.0, 0.0]) } }
    }
}

impl SubAssign<Scalar> for Rotor {
    fn sub_assign(&mut self, other: Scalar) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Scalar> for Rotor {
    type Output = Rotor;

    fn geometric_product(self, other: Scalar) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x2::from(other.group0()) } }
    }
}

impl OuterProduct<Scalar> for Rotor {
    type Output = Rotor;

    fn outer_product(self, other: Scalar) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x2::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for Rotor {
    type Output = Rotor;

    fn inner_product(self, other: Scalar) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x2::from(other.group0()) } }
    }
}

impl LeftContraction<Scalar> for Rotor {
    type Output = Scalar;

    fn left_contraction(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl RightContraction<Scalar> for Rotor {
    type Output = Rotor;

    fn right_contraction(self, other: Scalar) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x2::from(other.group0()) } }
    }
}

impl ScalarProduct<Scalar> for Rotor {
    type Output = Scalar;

    fn scalar_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl Add<MultiVector> for Rotor {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + other.group0(), g1: other.group1() } }
    }
}

impl Sub<MultiVector> for Rotor {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) - other.group0(), g1: Simd32x4::from(0.0) - other.group1() } }
    }
}

impl GeometricProduct<MultiVector> for Rotor {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl OuterProduct<MultiVector> for Rotor {
    type Output = MultiVector;

    fn outer_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<MultiVector> for Rotor {
    type Output = MultiVector;

    fn inner_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from([self.group0()[1], self.group0()[0], self.group0()[0], self.group0()[0]]) * swizzle!(other.group1(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl LeftContraction<MultiVector> for Rotor {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from([self.group0()[1], self.group0()[0], self.group0()[0], self.group0()[0]]) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from([self.group0()[1], self.group0()[0], self.group0()[0], self.group0()[0]]) * swizzle!(other.group1(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl ScalarProduct<MultiVector> for Rotor {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] } }
    }
}

impl Add<Rotor> for Rotor {
    type Output = Rotor;

    fn add(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() + other.group0() } }
    }
}

impl AddAssign<Rotor> for Rotor {
    fn add_assign(&mut self, other: Rotor) {
        *self = (*self).add(other);
    }
}

impl Sub<Rotor> for Rotor {
    type Output = Rotor;

    fn sub(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() - other.group0() } }
    }
}

impl SubAssign<Rotor> for Rotor {
    fn sub_assign(&mut self, other: Rotor) {
        *self = (*self).sub(other);
    }
}

impl Mul<Rotor> for Rotor {
    type Output = Rotor;

    fn mul(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * other.group0() } }
    }
}

impl MulAssign<Rotor> for Rotor {
    fn mul_assign(&mut self, other: Rotor) {
        *self = (*self).mul(other);
    }
}

impl Div<Rotor> for Rotor {
    type Output = Rotor;

    fn div(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x2::from([self.group0()[0], self.group0()[1]]) * Simd32x2::from([1.0, 1.0]) / Simd32x2::from([other.group0()[0], other.group0()[1]]) * Simd32x2::from([1.0, 1.0]) } }
    }
}

impl DivAssign<Rotor> for Rotor {
    fn div_assign(&mut self, other: Rotor) {
        *self = (*self).div(other);
    }
}

impl GeometricProduct<Rotor> for Rotor {
    type Output = Rotor;

    fn geometric_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() + Simd32x2::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0) * Simd32x2::from([-1.0, 1.0]) } }
    }
}

impl OuterProduct<Rotor> for Rotor {
    type Output = Rotor;

    fn outer_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() + self.group0() * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, 1.0]) } }
    }
}

impl InnerProduct<Rotor> for Rotor {
    type Output = Rotor;

    fn inner_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() + Simd32x2::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0) * Simd32x2::from([-1.0, 1.0]) } }
    }
}

impl LeftContraction<Rotor> for Rotor {
    type Output = Rotor;

    fn left_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() + swizzle!(self.group0(), 1, 0) * swizzle!(other.group0(), 1, 0) * Simd32x2::from([-1.0, 0.0]) } }
    }
}

impl RightContraction<Rotor> for Rotor {
    type Output = Rotor;

    fn right_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x2::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0) * Simd32x2::from([-1.0, 1.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([1.0, 0.0]) } }
    }
}

impl ScalarProduct<Rotor> for Rotor {
    type Output = Scalar;

    fn scalar_product(self, other: Rotor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] } }
    }
}

impl Add<Point> for Rotor {
    type Output = Motor;

    fn add(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl Sub<Point> for Rotor {
    type Output = Motor;

    fn sub(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) - Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl GeometricProduct<Point> for Rotor {
    type Output = Motor;

    fn geometric_product(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl OuterProduct<Point> for Rotor {
    type Output = Point;

    fn outer_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl InnerProduct<Point> for Rotor {
    type Output = Motor;

    fn inner_product(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[1], self.group0()[0], self.group0()[0], self.group0()[0]]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl LeftContraction<Point> for Rotor {
    type Output = Motor;

    fn left_contraction(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[1], self.group0()[0], self.group0()[0], self.group0()[0]]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl RightContraction<Point> for Rotor {
    type Output = Scalar;

    fn right_contraction(self, other: Point) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[1] * other.group0()[0] } }
    }
}

impl ScalarProduct<Point> for Rotor {
    type Output = Scalar;

    fn scalar_product(self, other: Point) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[1] * other.group0()[0] } }
    }
}

impl Add<IdealPoint> for Rotor {
    type Output = Motor;

    fn add(self, other: IdealPoint) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[0], other.group0()[1]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl Sub<IdealPoint> for Rotor {
    type Output = Motor;

    fn sub(self, other: IdealPoint) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) - Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[0], other.group0()[1]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl GeometricProduct<IdealPoint> for Rotor {
    type Output = IdealPoint;

    fn geometric_product(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() + Simd32x2::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0) * Simd32x2::from([-1.0, 1.0]) } }
    }
}

impl OuterProduct<IdealPoint> for Rotor {
    type Output = IdealPoint;

    fn outer_product(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() } }
    }
}

impl InnerProduct<IdealPoint> for Rotor {
    type Output = IdealPoint;

    fn inner_product(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() } }
    }
}

impl LeftContraction<IdealPoint> for Rotor {
    type Output = IdealPoint;

    fn left_contraction(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() } }
    }
}

impl GeometricProduct<Plane> for Rotor {
    type Output = MotorDual;

    fn geometric_product(self, other: Plane) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Plane> for Rotor {
    type Output = Scalar;

    fn regressive_product(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group0()[0] } }
    }
}

impl OuterProduct<Plane> for Rotor {
    type Output = MotorDual;

    fn outer_product(self, other: Plane) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from([self.group0()[1], self.group0()[0], self.group0()[0], self.group0()[0]]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl InnerProduct<Plane> for Rotor {
    type Output = Plane;

    fn inner_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[1]]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl LeftContraction<Plane> for Rotor {
    type Output = Plane;

    fn left_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl Add<Translator> for Rotor {
    type Output = Motor;

    fn add(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl Sub<Translator> for Rotor {
    type Output = Motor;

    fn sub(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) - Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl GeometricProduct<Translator> for Rotor {
    type Output = Motor;

    fn geometric_product(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl OuterProduct<Translator> for Rotor {
    type Output = Motor;

    fn outer_product(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl InnerProduct<Translator> for Rotor {
    type Output = Motor;

    fn inner_product(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl LeftContraction<Translator> for Rotor {
    type Output = Translator;

    fn left_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl RightContraction<Translator> for Rotor {
    type Output = Rotor;

    fn right_contraction(self, other: Translator) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x2::from(other.group0()[0]) } }
    }
}

impl ScalarProduct<Translator> for Rotor {
    type Output = Scalar;

    fn scalar_product(self, other: Translator) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] } }
    }
}

impl Add<Motor> for Rotor {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + other.group0() } }
    }
}

impl Sub<Motor> for Rotor {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) - other.group0() } }
    }
}

impl GeometricProduct<Motor> for Rotor {
    type Output = Motor;

    fn geometric_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl OuterProduct<Motor> for Rotor {
    type Output = Motor;

    fn outer_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<Motor> for Rotor {
    type Output = Motor;

    fn inner_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[0], self.group0()[0]]) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl LeftContraction<Motor> for Rotor {
    type Output = Motor;

    fn left_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from([self.group0()[1], self.group0()[0], self.group0()[0], self.group0()[0]]) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl RightContraction<Motor> for Rotor {
    type Output = Rotor;

    fn right_contraction(self, other: Motor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], other.group0()[0]]) * Simd32x2::from([-1.0, 1.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([1.0, 0.0]) } }
    }
}

impl ScalarProduct<Motor> for Rotor {
    type Output = Scalar;

    fn scalar_product(self, other: Motor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] } }
    }
}

impl GeometricProduct<MotorDual> for Rotor {
    type Output = MotorDual;

    fn geometric_product(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) } }
    }
}

impl RegressiveProduct<MotorDual> for Rotor {
    type Output = Rotor;

    fn regressive_product(self, other: MotorDual) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], other.group0()[0]]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([1.0, 0.0]) } }
    }
}

impl OuterProduct<MotorDual> for Rotor {
    type Output = MotorDual;

    fn outer_product(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from([self.group0()[1], self.group0()[0], self.group0()[0], self.group0()[0]]) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<MotorDual> for Rotor {
    type Output = MotorDual;

    fn inner_product(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[1], self.group0()[1]]) * swizzle!(other.group0(), 0, 0, 3, 2) * Simd32x4::from([0.0, -1.0, -1.0, 1.0]) } }
    }
}

impl LeftContraction<MotorDual> for Rotor {
    type Output = MotorDual;

    fn left_contraction(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, -1.0, 0.0, 0.0]) } }
    }
}

impl SquaredMagnitude for Rotor {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for Rotor {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl Scale for Rotor {
    type Output = Rotor;

    fn scale(self, other: f32) -> Rotor {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Signum for Rotor {
    type Output = Rotor;

    fn signum(self) -> Rotor {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.magnitude().group0() } })
    }
}

impl Inverse for Rotor {
    type Output = Rotor;

    fn inverse(self) -> Rotor {
        self.reversal().geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.squared_magnitude().group0() } })
    }
}

impl Zero for Point {
    fn zero() -> Self {
        Point { groups: PointGroups { g0: Simd32x3::from(0.0) } }
    }
}

impl One for Point {
    fn one() -> Self {
        Point { groups: PointGroups { g0: Simd32x3::from(0.0) } }
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x3::from(-1.0) } }
    }
}

impl Automorphism for Point {
    type Output = Point;

    fn automorphism(self) -> Point {
        Point { groups: PointGroups { g0: self.group0() } }
    }
}

impl Reversal for Point {
    type Output = Point;

    fn reversal(self) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x3::from(-1.0) } }
    }
}

impl Conjugation for Point {
    type Output = Point;

    fn conjugation(self) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x3::from(-1.0) } }
    }
}

impl Dual for Point {
    type Output = Plane;

    fn dual(self) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() } }
    }
}

impl Add<Scalar> for Point {
    type Output = Motor;

    fn add(self, other: Scalar) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + Simd32x4::from(other.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl Sub<Scalar> for Point {
    type Output = Motor;

    fn sub(self, other: Scalar) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) - Simd32x4::from(other.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl GeometricProduct<Scalar> for Point {
    type Output = Point;

    fn geometric_product(self, other: Scalar) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl OuterProduct<Scalar> for Point {
    type Output = Point;

    fn outer_product(self, other: Scalar) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for Point {
    type Output = Point;

    fn inner_product(self, other: Scalar) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl RightContraction<Scalar> for Point {
    type Output = Point;

    fn right_contraction(self, other: Scalar) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl Add<MultiVector> for Point {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + other.group0(), g1: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) + other.group1() } }
    }
}

impl Sub<MultiVector> for Point {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) - other.group0(), g1: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) - other.group1() } }
    }
}

impl GeometricProduct<MultiVector> for Point {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]), g1: Simd32x4::from(self.group0()[0]) * swizzle!(other.group1(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl ScalarProduct<MultiVector> for Point {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[1] } }
    }
}

impl Add<Rotor> for Point {
    type Output = Motor;

    fn add(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl Sub<Rotor> for Point {
    type Output = Motor;

    fn sub(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl GeometricProduct<Rotor> for Point {
    type Output = Motor;

    fn geometric_product(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) + Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[1]]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<Rotor> for Point {
    type Output = Point;

    fn outer_product(self, other: Rotor) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl InnerProduct<Rotor> for Point {
    type Output = Motor;

    fn inner_product(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[0], other.group0()[0]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl LeftContraction<Rotor> for Point {
    type Output = Scalar;

    fn left_contraction(self, other: Rotor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[1] } }
    }
}

impl RightContraction<Rotor> for Point {
    type Output = Motor;

    fn right_contraction(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[0], other.group0()[0]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl ScalarProduct<Rotor> for Point {
    type Output = Scalar;

    fn scalar_product(self, other: Rotor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[1] } }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: self.group0() + other.group0() } }
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, other: Point) {
        *self = (*self).add(other);
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: self.group0() - other.group0() } }
    }
}

impl SubAssign<Point> for Point {
    fn sub_assign(&mut self, other: Point) {
        *self = (*self).sub(other);
    }
}

impl Mul<Point> for Point {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: self.group0() * other.group0() } }
    }
}

impl MulAssign<Point> for Point {
    fn mul_assign(&mut self, other: Point) {
        *self = (*self).mul(other);
    }
}

impl Div<Point> for Point {
    type Output = Point;

    fn div(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) / Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) } }
    }
}

impl DivAssign<Point> for Point {
    fn div_assign(&mut self, other: Point) {
        *self = (*self).div(other);
    }
}

impl GeometricProduct<Point> for Point {
    type Output = Translator;

    fn geometric_product(self, other: Point) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([-1.0, -1.0, 1.0]) + swizzle!(self.group0(), 0, 2, 1) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl RegressiveProduct<Point> for Point {
    type Output = Plane;

    fn regressive_product(self, other: Point) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl InnerProduct<Point> for Point {
    type Output = Scalar;

    fn inner_product(self, other: Point) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] } }
    }
}

impl LeftContraction<Point> for Point {
    type Output = Scalar;

    fn left_contraction(self, other: Point) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] } }
    }
}

impl RightContraction<Point> for Point {
    type Output = Scalar;

    fn right_contraction(self, other: Point) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] } }
    }
}

impl ScalarProduct<Point> for Point {
    type Output = Scalar;

    fn scalar_product(self, other: Point) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] } }
    }
}

impl Into<IdealPoint> for Point {
    fn into(self) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x2::from([self.group0()[1], self.group0()[2]]) } }
    }
}

impl Add<IdealPoint> for Point {
    type Output = Point;

    fn add(self, other: IdealPoint) -> Point {
        Point { groups: PointGroups { g0: self.group0() + Simd32x3::from([other.group0()[0], other.group0()[0], other.group0()[1]]) * Simd32x3::from([0.0, 1.0, 1.0]) } }
    }
}

impl AddAssign<IdealPoint> for Point {
    fn add_assign(&mut self, other: IdealPoint) {
        *self = (*self).add(other);
    }
}

impl Sub<IdealPoint> for Point {
    type Output = Point;

    fn sub(self, other: IdealPoint) -> Point {
        Point { groups: PointGroups { g0: self.group0() - Simd32x3::from([other.group0()[0], other.group0()[0], other.group0()[1]]) * Simd32x3::from([0.0, 1.0, 1.0]) } }
    }
}

impl SubAssign<IdealPoint> for Point {
    fn sub_assign(&mut self, other: IdealPoint) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<IdealPoint> for Point {
    type Output = IdealPoint;

    fn geometric_product(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x2::from(self.group0()[0]) * swizzle!(other.group0(), 1, 0) * Simd32x2::from([-1.0, 1.0]) } }
    }
}

impl RegressiveProduct<IdealPoint> for Point {
    type Output = Plane;

    fn regressive_product(self, other: IdealPoint) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[2]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([1.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0) * Simd32x3::from([other.group0()[1], other.group0()[1], other.group0()[0]]) * Simd32x3::from([-1.0, 1.0, -1.0]) } }
    }
}

impl GeometricProduct<Plane> for Point {
    type Output = MotorDual;

    fn geometric_product(self, other: Plane) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[1]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[2], other.group0()[1], other.group0()[2], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Plane> for Point {
    type Output = Scalar;

    fn regressive_product(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl InnerProduct<Plane> for Point {
    type Output = Plane;

    fn inner_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[2]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([-1.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0) * swizzle!(other.group0(), 2, 2, 1) * Simd32x3::from([1.0, -1.0, 1.0]) } }
    }
}

impl RightContraction<Plane> for Point {
    type Output = Plane;

    fn right_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[2]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([-1.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0) * swizzle!(other.group0(), 2, 2, 1) * Simd32x3::from([1.0, -1.0, 1.0]) } }
    }
}

impl Add<Translator> for Point {
    type Output = Motor;

    fn add(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl Sub<Translator> for Point {
    type Output = Motor;

    fn sub(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) - Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl GeometricProduct<Translator> for Point {
    type Output = Point;

    fn geometric_product(self, other: Translator) -> Point {
        Point { groups: PointGroups { g0: Simd32x3::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([1.0, -1.0, 1.0]) + self.group0() * Simd32x3::from(other.group0()[0]) * Simd32x3::from([0.0, 1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Translator> for Point {
    type Output = Plane;

    fn regressive_product(self, other: Translator) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[2]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([1.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0) * swizzle!(other.group0(), 2, 2, 1) * Simd32x3::from([-1.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<Translator> for Point {
    type Output = Point;

    fn outer_product(self, other: Translator) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl InnerProduct<Translator> for Point {
    type Output = Point;

    fn inner_product(self, other: Translator) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl RightContraction<Translator> for Point {
    type Output = Point;

    fn right_contraction(self, other: Translator) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl Add<Motor> for Point {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + other.group0() } }
    }
}

impl Sub<Motor> for Point {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) - other.group0() } }
    }
}

impl GeometricProduct<Motor> for Point {
    type Output = Motor;

    fn geometric_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 1, 1, 0) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) + Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[1]]) * swizzle!(other.group0(), 0, 0, 0, 1) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) } }
    }
}

impl RegressiveProduct<Motor> for Point {
    type Output = Plane;

    fn regressive_product(self, other: Motor) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[3], other.group0()[3], other.group0()[1]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[2], other.group0()[1], other.group0()[2]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[3], other.group0()[2]]) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<Motor> for Point {
    type Output = Point;

    fn outer_product(self, other: Motor) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl InnerProduct<Motor> for Point {
    type Output = Motor;

    fn inner_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl LeftContraction<Motor> for Point {
    type Output = Scalar;

    fn left_contraction(self, other: Motor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[1] } }
    }
}

impl RightContraction<Motor> for Point {
    type Output = Motor;

    fn right_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl ScalarProduct<Motor> for Point {
    type Output = Scalar;

    fn scalar_product(self, other: Motor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[1] } }
    }
}

impl GeometricProduct<MotorDual> for Point {
    type Output = MotorDual;

    fn geometric_product(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 2, 3, 3) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[0], self.group0()[0]]) * swizzle!(other.group0(), 2, 3, 0, 0) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl RegressiveProduct<MotorDual> for Point {
    type Output = Motor;

    fn regressive_product(self, other: MotorDual) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<MotorDual> for Point {
    type Output = Plane;

    fn inner_product(self, other: MotorDual) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[3], other.group0()[2]]) * Simd32x3::from([-1.0, -1.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from(other.group0()[2]) * Simd32x3::from([-1.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0) * Simd32x3::from([other.group0()[3], other.group0()[0], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, 0.0]) } }
    }
}

impl RightContraction<MotorDual> for Point {
    type Output = Plane;

    fn right_contraction(self, other: MotorDual) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[2]) * Simd32x3::from(other.group0()[2]) * Simd32x3::from([-1.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0) * Simd32x3::from([other.group0()[3], other.group0()[3], other.group0()[2]]) * Simd32x3::from([1.0, -1.0, 1.0]) } }
    }
}

impl SquaredMagnitude for Point {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for Point {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl Scale for Point {
    type Output = Point;

    fn scale(self, other: f32) -> Point {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Signum for Point {
    type Output = Point;

    fn signum(self) -> Point {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.magnitude().group0() } })
    }
}

impl Inverse for Point {
    type Output = Point;

    fn inverse(self) -> Point {
        self.reversal().geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.squared_magnitude().group0() } })
    }
}

impl Zero for IdealPoint {
    fn zero() -> Self {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x2::from(0.0) } }
    }
}

impl One for IdealPoint {
    fn one() -> Self {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x2::from(0.0) } }
    }
}

impl Neg for IdealPoint {
    type Output = IdealPoint;

    fn neg(self) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x2::from(-1.0) } }
    }
}

impl Automorphism for IdealPoint {
    type Output = IdealPoint;

    fn automorphism(self) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() } }
    }
}

impl Reversal for IdealPoint {
    type Output = IdealPoint;

    fn reversal(self) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x2::from(-1.0) } }
    }
}

impl Conjugation for IdealPoint {
    type Output = IdealPoint;

    fn conjugation(self) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x2::from(-1.0) } }
    }
}

impl Add<Scalar> for IdealPoint {
    type Output = Translator;

    fn add(self, other: Scalar) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[0], self.group0()[1]]) * Simd32x3::from([0.0, 1.0, 1.0]) + Simd32x3::from(other.group0()) * Simd32x3::from([1.0, 0.0, 0.0]) } }
    }
}

impl Sub<Scalar> for IdealPoint {
    type Output = Translator;

    fn sub(self, other: Scalar) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[0], self.group0()[1]]) * Simd32x3::from([0.0, 1.0, 1.0]) - Simd32x3::from(other.group0()) * Simd32x3::from([1.0, 0.0, 0.0]) } }
    }
}

impl GeometricProduct<Scalar> for IdealPoint {
    type Output = IdealPoint;

    fn geometric_product(self, other: Scalar) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x2::from(other.group0()) } }
    }
}

impl OuterProduct<Scalar> for IdealPoint {
    type Output = IdealPoint;

    fn outer_product(self, other: Scalar) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x2::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for IdealPoint {
    type Output = IdealPoint;

    fn inner_product(self, other: Scalar) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x2::from(other.group0()) } }
    }
}

impl RightContraction<Scalar> for IdealPoint {
    type Output = IdealPoint;

    fn right_contraction(self, other: Scalar) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x2::from(other.group0()) } }
    }
}

impl Add<MultiVector> for IdealPoint {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: other.group0(), g1: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group0()[1]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) + other.group1() } }
    }
}

impl Sub<MultiVector> for IdealPoint {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(0.0) - other.group0(), g1: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group0()[1]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) - other.group1() } }
    }
}

impl Add<Rotor> for IdealPoint {
    type Output = Motor;

    fn add(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group0()[1]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl Sub<Rotor> for IdealPoint {
    type Output = Motor;

    fn sub(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group0()[1]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl GeometricProduct<Rotor> for IdealPoint {
    type Output = IdealPoint;

    fn geometric_product(self, other: Rotor) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() * Simd32x2::from([1.0, -1.0]) + Simd32x2::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0) } }
    }
}

impl OuterProduct<Rotor> for IdealPoint {
    type Output = IdealPoint;

    fn outer_product(self, other: Rotor) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x2::from(other.group0()[0]) } }
    }
}

impl InnerProduct<Rotor> for IdealPoint {
    type Output = IdealPoint;

    fn inner_product(self, other: Rotor) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x2::from(other.group0()[0]) } }
    }
}

impl RightContraction<Rotor> for IdealPoint {
    type Output = IdealPoint;

    fn right_contraction(self, other: Rotor) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x2::from(other.group0()[0]) } }
    }
}

impl Add<Point> for IdealPoint {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[0], self.group0()[1]]) * Simd32x3::from([0.0, 1.0, 1.0]) + other.group0() } }
    }
}

impl Sub<Point> for IdealPoint {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[0], self.group0()[1]]) * Simd32x3::from([0.0, 1.0, 1.0]) - other.group0() } }
    }
}

impl GeometricProduct<Point> for IdealPoint {
    type Output = IdealPoint;

    fn geometric_product(self, other: Point) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: swizzle!(self.group0(), 1, 0) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([1.0, -1.0]) } }
    }
}

impl RegressiveProduct<Point> for IdealPoint {
    type Output = Plane;

    fn regressive_product(self, other: Point) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * swizzle!(other.group0(), 2, 0, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) } }
    }
}

impl Add<IdealPoint> for IdealPoint {
    type Output = IdealPoint;

    fn add(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() + other.group0() } }
    }
}

impl AddAssign<IdealPoint> for IdealPoint {
    fn add_assign(&mut self, other: IdealPoint) {
        *self = (*self).add(other);
    }
}

impl Sub<IdealPoint> for IdealPoint {
    type Output = IdealPoint;

    fn sub(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() - other.group0() } }
    }
}

impl SubAssign<IdealPoint> for IdealPoint {
    fn sub_assign(&mut self, other: IdealPoint) {
        *self = (*self).sub(other);
    }
}

impl Mul<IdealPoint> for IdealPoint {
    type Output = IdealPoint;

    fn mul(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * other.group0() } }
    }
}

impl MulAssign<IdealPoint> for IdealPoint {
    fn mul_assign(&mut self, other: IdealPoint) {
        *self = (*self).mul(other);
    }
}

impl Div<IdealPoint> for IdealPoint {
    type Output = IdealPoint;

    fn div(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x2::from([self.group0()[0], self.group0()[1]]) * Simd32x2::from([1.0, 1.0]) / Simd32x2::from([other.group0()[0], other.group0()[1]]) * Simd32x2::from([1.0, 1.0]) } }
    }
}

impl DivAssign<IdealPoint> for IdealPoint {
    fn div_assign(&mut self, other: IdealPoint) {
        *self = (*self).div(other);
    }
}

impl RegressiveProduct<Plane> for IdealPoint {
    type Output = Scalar;

    fn regressive_product(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[1] + self.group0()[1] * other.group0()[2] } }
    }
}

impl Add<Translator> for IdealPoint {
    type Output = Translator;

    fn add(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[0], self.group0()[1]]) * Simd32x3::from([0.0, 1.0, 1.0]) + other.group0() } }
    }
}

impl Sub<Translator> for IdealPoint {
    type Output = Translator;

    fn sub(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[0], self.group0()[1]]) * Simd32x3::from([0.0, 1.0, 1.0]) - other.group0() } }
    }
}

impl GeometricProduct<Translator> for IdealPoint {
    type Output = IdealPoint;

    fn geometric_product(self, other: Translator) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x2::from(other.group0()[0]) } }
    }
}

impl OuterProduct<Translator> for IdealPoint {
    type Output = IdealPoint;

    fn outer_product(self, other: Translator) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x2::from(other.group0()[0]) } }
    }
}

impl InnerProduct<Translator> for IdealPoint {
    type Output = IdealPoint;

    fn inner_product(self, other: Translator) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x2::from(other.group0()[0]) } }
    }
}

impl RightContraction<Translator> for IdealPoint {
    type Output = IdealPoint;

    fn right_contraction(self, other: Translator) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x2::from(other.group0()[0]) } }
    }
}

impl Add<Motor> for IdealPoint {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group0()[1]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) + other.group0() } }
    }
}

impl Sub<Motor> for IdealPoint {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group0()[1]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) - other.group0() } }
    }
}

impl GeometricProduct<Motor> for IdealPoint {
    type Output = IdealPoint;

    fn geometric_product(self, other: Motor) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], other.group0()[1]]) * Simd32x2::from([1.0, -1.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], other.group0()[0]]) } }
    }
}

impl RegressiveProduct<Motor> for IdealPoint {
    type Output = Plane;

    fn regressive_product(self, other: Motor) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[1], other.group0()[2]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[3], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 0.0, 1.0]) } }
    }
}

impl OuterProduct<Motor> for IdealPoint {
    type Output = IdealPoint;

    fn outer_product(self, other: Motor) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x2::from(other.group0()[0]) } }
    }
}

impl InnerProduct<Motor> for IdealPoint {
    type Output = IdealPoint;

    fn inner_product(self, other: Motor) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x2::from(other.group0()[0]) } }
    }
}

impl RightContraction<Motor> for IdealPoint {
    type Output = IdealPoint;

    fn right_contraction(self, other: Motor) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x2::from(other.group0()[0]) } }
    }
}

impl RegressiveProduct<MotorDual> for IdealPoint {
    type Output = Translator;

    fn regressive_product(self, other: MotorDual) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[3], other.group0()[3], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[0]]) * Simd32x3::from([1.0, 1.0, 0.0]) } }
    }
}

impl Scale for IdealPoint {
    type Output = IdealPoint;

    fn scale(self, other: f32) -> IdealPoint {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Zero for Plane {
    fn zero() -> Self {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(0.0) } }
    }
}

impl One for Plane {
    fn one() -> Self {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(0.0) } }
    }
}

impl Neg for Plane {
    type Output = Plane;

    fn neg(self) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x3::from(-1.0) } }
    }
}

impl Automorphism for Plane {
    type Output = Plane;

    fn automorphism(self) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x3::from(-1.0) } }
    }
}

impl Reversal for Plane {
    type Output = Plane;

    fn reversal(self) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() } }
    }
}

impl Conjugation for Plane {
    type Output = Plane;

    fn conjugation(self) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x3::from(-1.0) } }
    }
}

impl Dual for Plane {
    type Output = Point;

    fn dual(self) -> Point {
        Point { groups: PointGroups { g0: self.group0() } }
    }
}

impl GeometricProduct<Scalar> for Plane {
    type Output = Plane;

    fn geometric_product(self, other: Scalar) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl OuterProduct<Scalar> for Plane {
    type Output = Plane;

    fn outer_product(self, other: Scalar) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for Plane {
    type Output = Plane;

    fn inner_product(self, other: Scalar) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl RightContraction<Scalar> for Plane {
    type Output = Plane;

    fn right_contraction(self, other: Scalar) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl Add<MultiVector> for Plane {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[2], self.group0()[1]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) + other.group0(), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + other.group1() } }
    }
}

impl Sub<MultiVector> for Plane {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[2], self.group0()[1]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) - other.group0(), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) - other.group1() } }
    }
}

impl GeometricProduct<MultiVector> for Plane {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 3, 0, 1), g1: Simd32x4::from(self.group0()[0]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 3, 2, 1, 0) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 2, 3, 0, 1) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl ScalarProduct<MultiVector> for Plane {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group0()[3] + self.group0()[2] * other.group0()[2] } }
    }
}

impl GeometricProduct<Rotor> for Plane {
    type Output = MotorDual;

    fn geometric_product(self, other: Rotor) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) + Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[1]]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[0], other.group0()[1]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RegressiveProduct<Rotor> for Plane {
    type Output = Scalar;

    fn regressive_product(self, other: Rotor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[1] } }
    }
}

impl OuterProduct<Rotor> for Plane {
    type Output = MotorDual;

    fn outer_product(self, other: Rotor) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[0], other.group0()[0]]) } }
    }
}

impl InnerProduct<Rotor> for Plane {
    type Output = Plane;

    fn inner_product(self, other: Rotor) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[1], other.group0()[0]]) * Simd32x3::from([0.0, 1.0, 1.0]) + swizzle!(self.group0(), 0, 1, 1) * Simd32x3::from([other.group0()[0], other.group0()[0], other.group0()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) } }
    }
}

impl RightContraction<Rotor> for Plane {
    type Output = Plane;

    fn right_contraction(self, other: Rotor) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl GeometricProduct<Point> for Plane {
    type Output = MotorDual;

    fn geometric_product(self, other: Point) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[2], other.group0()[1], other.group0()[0], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl RegressiveProduct<Point> for Plane {
    type Output = Scalar;

    fn regressive_product(self, other: Point) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl InnerProduct<Point> for Plane {
    type Output = Plane;

    fn inner_product(self, other: Point) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([-1.0, 1.0, 0.0]) + swizzle!(self.group0(), 1, 0, 1) * swizzle!(other.group0(), 2, 0, 0) * Simd32x3::from([1.0, 0.0, -1.0]) } }
    }
}

impl LeftContraction<Point> for Plane {
    type Output = Plane;

    fn left_contraction(self, other: Point) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([-1.0, 1.0, 0.0]) + swizzle!(self.group0(), 1, 0, 1) * swizzle!(other.group0(), 2, 0, 0) * Simd32x3::from([1.0, 0.0, -1.0]) } }
    }
}

impl RegressiveProduct<IdealPoint> for Plane {
    type Output = Scalar;

    fn regressive_product(self, other: IdealPoint) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group0()[0] + self.group0()[2] * other.group0()[1] } }
    }
}

impl Add<Plane> for Plane {
    type Output = Plane;

    fn add(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() + other.group0() } }
    }
}

impl AddAssign<Plane> for Plane {
    fn add_assign(&mut self, other: Plane) {
        *self = (*self).add(other);
    }
}

impl Sub<Plane> for Plane {
    type Output = Plane;

    fn sub(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() - other.group0() } }
    }
}

impl SubAssign<Plane> for Plane {
    fn sub_assign(&mut self, other: Plane) {
        *self = (*self).sub(other);
    }
}

impl Mul<Plane> for Plane {
    type Output = Plane;

    fn mul(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * other.group0() } }
    }
}

impl MulAssign<Plane> for Plane {
    fn mul_assign(&mut self, other: Plane) {
        *self = (*self).mul(other);
    }
}

impl Div<Plane> for Plane {
    type Output = Plane;

    fn div(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) / Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) } }
    }
}

impl DivAssign<Plane> for Plane {
    fn div_assign(&mut self, other: Plane) {
        *self = (*self).div(other);
    }
}

impl GeometricProduct<Plane> for Plane {
    type Output = Motor;

    fn geometric_product(self, other: Plane) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[2], other.group0()[1], other.group0()[0], other.group0()[2]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<Plane> for Plane {
    type Output = Point;

    fn outer_product(self, other: Plane) -> Point {
        Point { groups: PointGroups { g0: Simd32x3::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl InnerProduct<Plane> for Plane {
    type Output = Scalar;

    fn inner_product(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftContraction<Plane> for Plane {
    type Output = Scalar;

    fn left_contraction(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl RightContraction<Plane> for Plane {
    type Output = Scalar;

    fn right_contraction(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl ScalarProduct<Plane> for Plane {
    type Output = Scalar;

    fn scalar_product(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl GeometricProduct<Translator> for Plane {
    type Output = MotorDual;

    fn geometric_product(self, other: Translator) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[0], other.group0()[1]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[2], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) } }
    }
}

impl RegressiveProduct<Translator> for Plane {
    type Output = Scalar;

    fn regressive_product(self, other: Translator) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl OuterProduct<Translator> for Plane {
    type Output = MotorDual;

    fn outer_product(self, other: Translator) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group0()[1], self.group0()[0], self.group0()[1], self.group0()[0]]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[0], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl InnerProduct<Translator> for Plane {
    type Output = Plane;

    fn inner_product(self, other: Translator) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[1]) * swizzle!(other.group0(), 2, 0, 2) * Simd32x3::from([1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 1, 1, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([1.0, 0.0, 0.0]) } }
    }
}

impl RightContraction<Translator> for Plane {
    type Output = Plane;

    fn right_contraction(self, other: Translator) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl GeometricProduct<Motor> for Plane {
    type Output = MotorDual;

    fn geometric_product(self, other: Motor) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl RegressiveProduct<Motor> for Plane {
    type Output = Scalar;

    fn regressive_product(self, other: Motor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[1] + self.group0()[1] * other.group0()[2] + self.group0()[2] * other.group0()[3] } }
    }
}

impl OuterProduct<Motor> for Plane {
    type Output = MotorDual;

    fn outer_product(self, other: Motor) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<Motor> for Plane {
    type Output = Plane;

    fn inner_product(self, other: Motor) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[3], other.group0()[0], other.group0()[1]]) * Simd32x3::from([1.0, 1.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([1.0, 0.0, 0.0]) } }
    }
}

impl LeftContraction<Motor> for Plane {
    type Output = Plane;

    fn left_contraction(self, other: Motor) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[2], other.group0()[1], other.group0()[2]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + swizzle!(self.group0(), 1, 0, 1) * Simd32x3::from([other.group0()[3], other.group0()[0], other.group0()[1]]) * Simd32x3::from([1.0, 0.0, -1.0]) } }
    }
}

impl RightContraction<Motor> for Plane {
    type Output = Plane;

    fn right_contraction(self, other: Motor) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl Add<MotorDual> for Plane {
    type Output = MotorDual;

    fn add(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + other.group0() } }
    }
}

impl Sub<MotorDual> for Plane {
    type Output = MotorDual;

    fn sub(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) - other.group0() } }
    }
}

impl GeometricProduct<MotorDual> for Plane {
    type Output = Motor;

    fn geometric_product(self, other: MotorDual) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 0, 0, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) } }
    }
}

impl RegressiveProduct<MotorDual> for Plane {
    type Output = Plane;

    fn regressive_product(self, other: MotorDual) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl OuterProduct<MotorDual> for Plane {
    type Output = Point;

    fn outer_product(self, other: MotorDual) -> Point {
        Point { groups: PointGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[3], other.group0()[3], other.group0()[1]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[2], other.group0()[1], other.group0()[2]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[3], other.group0()[2]]) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl InnerProduct<MotorDual> for Plane {
    type Output = Translator;

    fn inner_product(self, other: MotorDual) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[3], other.group0()[3], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, 1.0]) + swizzle!(self.group0(), 1, 1, 0) * Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[0]]) * Simd32x3::from([1.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<MotorDual> for Plane {
    type Output = Translator;

    fn left_contraction(self, other: MotorDual) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[3], other.group0()[3], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, 1.0]) + swizzle!(self.group0(), 1, 1, 0) * Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[0]]) * Simd32x3::from([1.0, 1.0, 0.0]) } }
    }
}

impl RightContraction<MotorDual> for Plane {
    type Output = Scalar;

    fn right_contraction(self, other: MotorDual) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group0()[2] + self.group0()[2] * other.group0()[3] } }
    }
}

impl ScalarProduct<MotorDual> for Plane {
    type Output = Scalar;

    fn scalar_product(self, other: MotorDual) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group0()[2] + self.group0()[2] * other.group0()[3] } }
    }
}

impl SquaredMagnitude for Plane {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for Plane {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl Scale for Plane {
    type Output = Plane;

    fn scale(self, other: f32) -> Plane {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Signum for Plane {
    type Output = Plane;

    fn signum(self) -> Plane {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.magnitude().group0() } })
    }
}

impl Inverse for Plane {
    type Output = Plane;

    fn inverse(self) -> Plane {
        self.reversal().geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.squared_magnitude().group0() } })
    }
}

impl Zero for Translator {
    fn zero() -> Self {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from(0.0) } }
    }
}

impl One for Translator {
    fn one() -> Self {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from([1.0, 0.0, 0.0]) } }
    }
}

impl Neg for Translator {
    type Output = Translator;

    fn neg(self) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x3::from(-1.0) } }
    }
}

impl Automorphism for Translator {
    type Output = Translator;

    fn automorphism(self) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() } }
    }
}

impl Reversal for Translator {
    type Output = Translator;

    fn reversal(self) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x3::from([1.0, -1.0, -1.0]) } }
    }
}

impl Conjugation for Translator {
    type Output = Translator;

    fn conjugation(self) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x3::from([1.0, -1.0, -1.0]) } }
    }
}

impl Into<Scalar> for Translator {
    fn into(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] } }
    }
}

impl Add<Scalar> for Translator {
    type Output = Translator;

    fn add(self, other: Scalar) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() + Simd32x3::from(other.group0()) * Simd32x3::from([1.0, 0.0, 0.0]) } }
    }
}

impl AddAssign<Scalar> for Translator {
    fn add_assign(&mut self, other: Scalar) {
        *self = (*self).add(other);
    }
}

impl Sub<Scalar> for Translator {
    type Output = Translator;

    fn sub(self, other: Scalar) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() - Simd32x3::from(other.group0()) * Simd32x3::from([1.0, 0.0, 0.0]) } }
    }
}

impl SubAssign<Scalar> for Translator {
    fn sub_assign(&mut self, other: Scalar) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Scalar> for Translator {
    type Output = Translator;

    fn geometric_product(self, other: Scalar) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl OuterProduct<Scalar> for Translator {
    type Output = Translator;

    fn outer_product(self, other: Scalar) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for Translator {
    type Output = Translator;

    fn inner_product(self, other: Scalar) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl LeftContraction<Scalar> for Translator {
    type Output = Scalar;

    fn left_contraction(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl RightContraction<Scalar> for Translator {
    type Output = Translator;

    fn right_contraction(self, other: Scalar) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl ScalarProduct<Scalar> for Translator {
    type Output = Scalar;

    fn scalar_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl Add<MultiVector> for Translator {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + other.group0(), g1: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) + other.group1() } }
    }
}

impl Sub<MultiVector> for Translator {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) - other.group0(), g1: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) - other.group1() } }
    }
}

impl GeometricProduct<MultiVector> for Translator {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl OuterProduct<MultiVector> for Translator {
    type Output = MultiVector;

    fn outer_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 2, 0) * Simd32x4::from([0.0, 1.0, 0.0, 1.0]) + Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[1], self.group0()[0]]) * swizzle!(other.group0(), 0, 3, 0, 0) * Simd32x4::from([0.0, 1.0, 1.0, 0.0]) } }
    }
}

impl InnerProduct<MultiVector> for Translator {
    type Output = MultiVector;

    fn inner_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group0()[1], self.group0()[0], self.group0()[1], self.group0()[0]]) * swizzle!(other.group0(), 2, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<MultiVector> for Translator {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * other.group1() } }
    }
}

impl ScalarProduct<MultiVector> for Translator {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] } }
    }
}

impl Add<Rotor> for Translator {
    type Output = Motor;

    fn add(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([1.0, 0.0, 1.0, 1.0]) + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl Sub<Rotor> for Translator {
    type Output = Motor;

    fn sub(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([1.0, 0.0, 1.0, 1.0]) - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl GeometricProduct<Rotor> for Translator {
    type Output = Motor;

    fn geometric_product(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) + Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[1]]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<Rotor> for Translator {
    type Output = Motor;

    fn outer_product(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) } }
    }
}

impl InnerProduct<Rotor> for Translator {
    type Output = Motor;

    fn inner_product(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) } }
    }
}

impl LeftContraction<Rotor> for Translator {
    type Output = Rotor;

    fn left_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() } }
    }
}

impl RightContraction<Rotor> for Translator {
    type Output = Translator;

    fn right_contraction(self, other: Rotor) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl ScalarProduct<Rotor> for Translator {
    type Output = Scalar;

    fn scalar_product(self, other: Rotor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] } }
    }
}

impl Add<Point> for Translator {
    type Output = Motor;

    fn add(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([1.0, 0.0, 1.0, 1.0]) + Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl Sub<Point> for Translator {
    type Output = Motor;

    fn sub(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([1.0, 0.0, 1.0, 1.0]) - Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl GeometricProduct<Point> for Translator {
    type Output = Point;

    fn geometric_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() + swizzle!(self.group0(), 0, 2, 1) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl RegressiveProduct<Point> for Translator {
    type Output = Plane;

    fn regressive_product(self, other: Point) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + swizzle!(self.group0(), 1, 0, 1) * swizzle!(other.group0(), 2, 0, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) } }
    }
}

impl OuterProduct<Point> for Translator {
    type Output = Point;

    fn outer_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl InnerProduct<Point> for Translator {
    type Output = Point;

    fn inner_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl LeftContraction<Point> for Translator {
    type Output = Point;

    fn left_contraction(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl Into<IdealPoint> for Translator {
    fn into(self) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x2::from([self.group0()[1], self.group0()[2]]) } }
    }
}

impl Add<IdealPoint> for Translator {
    type Output = Translator;

    fn add(self, other: IdealPoint) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() + Simd32x3::from([other.group0()[0], other.group0()[0], other.group0()[1]]) * Simd32x3::from([0.0, 1.0, 1.0]) } }
    }
}

impl AddAssign<IdealPoint> for Translator {
    fn add_assign(&mut self, other: IdealPoint) {
        *self = (*self).add(other);
    }
}

impl Sub<IdealPoint> for Translator {
    type Output = Translator;

    fn sub(self, other: IdealPoint) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() - Simd32x3::from([other.group0()[0], other.group0()[0], other.group0()[1]]) * Simd32x3::from([0.0, 1.0, 1.0]) } }
    }
}

impl SubAssign<IdealPoint> for Translator {
    fn sub_assign(&mut self, other: IdealPoint) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<IdealPoint> for Translator {
    type Output = IdealPoint;

    fn geometric_product(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() } }
    }
}

impl OuterProduct<IdealPoint> for Translator {
    type Output = IdealPoint;

    fn outer_product(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() } }
    }
}

impl InnerProduct<IdealPoint> for Translator {
    type Output = IdealPoint;

    fn inner_product(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() } }
    }
}

impl LeftContraction<IdealPoint> for Translator {
    type Output = IdealPoint;

    fn left_contraction(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() } }
    }
}

impl GeometricProduct<Plane> for Translator {
    type Output = MotorDual;

    fn geometric_product(self, other: Plane) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[1]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[2], other.group0()[1], other.group0()[2], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Plane> for Translator {
    type Output = Scalar;

    fn regressive_product(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl OuterProduct<Plane> for Translator {
    type Output = MotorDual;

    fn outer_product(self, other: Plane) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from([self.group0()[1], self.group0()[0], self.group0()[0], self.group0()[0]]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl InnerProduct<Plane> for Translator {
    type Output = Plane;

    fn inner_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() + Simd32x3::from(self.group0()[2]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([-1.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0) * swizzle!(other.group0(), 2, 0, 0) * Simd32x3::from([1.0, 0.0, 0.0]) } }
    }
}

impl LeftContraction<Plane> for Translator {
    type Output = Plane;

    fn left_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl Add<Translator> for Translator {
    type Output = Translator;

    fn add(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() + other.group0() } }
    }
}

impl AddAssign<Translator> for Translator {
    fn add_assign(&mut self, other: Translator) {
        *self = (*self).add(other);
    }
}

impl Sub<Translator> for Translator {
    type Output = Translator;

    fn sub(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() - other.group0() } }
    }
}

impl SubAssign<Translator> for Translator {
    fn sub_assign(&mut self, other: Translator) {
        *self = (*self).sub(other);
    }
}

impl Mul<Translator> for Translator {
    type Output = Translator;

    fn mul(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * other.group0() } }
    }
}

impl MulAssign<Translator> for Translator {
    fn mul_assign(&mut self, other: Translator) {
        *self = (*self).mul(other);
    }
}

impl Div<Translator> for Translator {
    type Output = Translator;

    fn div(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) / Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) } }
    }
}

impl DivAssign<Translator> for Translator {
    fn div_assign(&mut self, other: Translator) {
        *self = (*self).div(other);
    }
}

impl GeometricProduct<Translator> for Translator {
    type Output = Translator;

    fn geometric_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() + self.group0() * Simd32x3::from(other.group0()[0]) * Simd32x3::from([0.0, 1.0, 1.0]) } }
    }
}

impl OuterProduct<Translator> for Translator {
    type Output = Translator;

    fn outer_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() + self.group0() * Simd32x3::from(other.group0()[0]) * Simd32x3::from([0.0, 1.0, 1.0]) } }
    }
}

impl InnerProduct<Translator> for Translator {
    type Output = Translator;

    fn inner_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() + self.group0() * Simd32x3::from(other.group0()[0]) * Simd32x3::from([0.0, 1.0, 1.0]) } }
    }
}

impl LeftContraction<Translator> for Translator {
    type Output = Translator;

    fn left_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl RightContraction<Translator> for Translator {
    type Output = Translator;

    fn right_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl ScalarProduct<Translator> for Translator {
    type Output = Scalar;

    fn scalar_product(self, other: Translator) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] } }
    }
}

impl Add<Motor> for Translator {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([1.0, 0.0, 1.0, 1.0]) + other.group0() } }
    }
}

impl Sub<Motor> for Translator {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([1.0, 0.0, 1.0, 1.0]) - other.group0() } }
    }
}

impl GeometricProduct<Motor> for Translator {
    type Output = Motor;

    fn geometric_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 1, 1, 0) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) + Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[1]]) * swizzle!(other.group0(), 0, 0, 0, 1) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) } }
    }
}

impl RegressiveProduct<Motor> for Translator {
    type Output = Plane;

    fn regressive_product(self, other: Motor) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[2], other.group0()[1], other.group0()[2]]) * Simd32x3::from([1.0, -1.0, 0.0]) + swizzle!(self.group0(), 1, 0, 1) * Simd32x3::from([other.group0()[3], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 0.0, 1.0]) } }
    }
}

impl OuterProduct<Motor> for Translator {
    type Output = Motor;

    fn outer_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl InnerProduct<Motor> for Translator {
    type Output = Motor;

    fn inner_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl LeftContraction<Motor> for Translator {
    type Output = Motor;

    fn left_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl RightContraction<Motor> for Translator {
    type Output = Translator;

    fn right_contraction(self, other: Motor) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl ScalarProduct<Motor> for Translator {
    type Output = Scalar;

    fn scalar_product(self, other: Motor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] } }
    }
}

impl GeometricProduct<MotorDual> for Translator {
    type Output = MotorDual;

    fn geometric_product(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 2, 3, 3) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[0], self.group0()[0]]) * swizzle!(other.group0(), 2, 3, 0, 0) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl RegressiveProduct<MotorDual> for Translator {
    type Output = Translator;

    fn regressive_product(self, other: MotorDual) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[2]]) * Simd32x3::from([1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[3], other.group0()[3], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([1.0, 0.0, 0.0]) } }
    }
}

impl OuterProduct<MotorDual> for Translator {
    type Output = MotorDual;

    fn outer_product(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from([self.group0()[1], self.group0()[0], self.group0()[0], self.group0()[0]]) * swizzle!(other.group0(), 2, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<MotorDual> for Translator {
    type Output = MotorDual;

    fn inner_product(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, -1.0, 0.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[0], self.group0()[0]]) * swizzle!(other.group0(), 0, 3, 0, 0) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) } }
    }
}

impl LeftContraction<MotorDual> for Translator {
    type Output = MotorDual;

    fn left_contraction(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl SquaredMagnitude for Translator {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for Translator {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl Scale for Translator {
    type Output = Translator;

    fn scale(self, other: f32) -> Translator {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Signum for Translator {
    type Output = Translator;

    fn signum(self) -> Translator {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.magnitude().group0() } })
    }
}

impl Inverse for Translator {
    type Output = Translator;

    fn inverse(self) -> Translator {
        self.reversal().geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.squared_magnitude().group0() } })
    }
}

impl Zero for Motor {
    fn zero() -> Self {
        Motor { groups: MotorGroups { g0: Simd32x4::from(0.0) } }
    }
}

impl One for Motor {
    fn one() -> Self {
        Motor { groups: MotorGroups { g0: Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl Neg for Motor {
    type Output = Motor;

    fn neg(self) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
    }
}

impl Automorphism for Motor {
    type Output = Motor;

    fn automorphism(self) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() } }
    }
}

impl Reversal for Motor {
    type Output = Motor;

    fn reversal(self) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) } }
    }
}

impl Conjugation for Motor {
    type Output = Motor;

    fn conjugation(self) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) } }
    }
}

impl Dual for Motor {
    type Output = MotorDual;

    fn dual(self) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: self.group0() } }
    }
}

impl Into<Scalar> for Motor {
    fn into(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] } }
    }
}

impl Add<Scalar> for Motor {
    type Output = Motor;

    fn add(self, other: Scalar) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() + Simd32x4::from(other.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl AddAssign<Scalar> for Motor {
    fn add_assign(&mut self, other: Scalar) {
        *self = (*self).add(other);
    }
}

impl Sub<Scalar> for Motor {
    type Output = Motor;

    fn sub(self, other: Scalar) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() - Simd32x4::from(other.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl SubAssign<Scalar> for Motor {
    fn sub_assign(&mut self, other: Scalar) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Scalar> for Motor {
    type Output = Motor;

    fn geometric_product(self, other: Scalar) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl OuterProduct<Scalar> for Motor {
    type Output = Motor;

    fn outer_product(self, other: Scalar) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for Motor {
    type Output = Motor;

    fn inner_product(self, other: Scalar) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl LeftContraction<Scalar> for Motor {
    type Output = Scalar;

    fn left_contraction(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl RightContraction<Scalar> for Motor {
    type Output = Motor;

    fn right_contraction(self, other: Scalar) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl ScalarProduct<Scalar> for Motor {
    type Output = Scalar;

    fn scalar_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl Add<MultiVector> for Motor {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: swizzle!(self.group0(), 0, 1, 0, 0) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + other.group0(), g1: swizzle!(self.group0(), 0, 0, 2, 3) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) + other.group1() } }
    }
}

impl Sub<MultiVector> for Motor {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: swizzle!(self.group0(), 0, 1, 0, 0) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) - other.group0(), g1: swizzle!(self.group0(), 0, 0, 2, 3) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) - other.group1() } }
    }
}

impl GeometricProduct<MultiVector> for Motor {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl OuterProduct<MultiVector> for Motor {
    type Output = MultiVector;

    fn outer_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + swizzle!(self.group0(), 0, 1, 0, 0) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 0, 3) * Simd32x4::from([0.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 2, 2, 2, 0) * Simd32x4::from([0.0, 1.0, 0.0, 1.0]) + swizzle!(self.group0(), 0, 1, 0, 0) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<MultiVector> for Motor {
    type Output = MultiVector;

    fn inner_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group1(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl LeftContraction<MultiVector> for Motor {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group1(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl ScalarProduct<MultiVector> for Motor {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] } }
    }
}

impl Into<Rotor> for Motor {
    fn into(self) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x2::from([self.group0()[0], self.group0()[1]]) } }
    }
}

impl Add<Rotor> for Motor {
    type Output = Motor;

    fn add(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl AddAssign<Rotor> for Motor {
    fn add_assign(&mut self, other: Rotor) {
        *self = (*self).add(other);
    }
}

impl Sub<Rotor> for Motor {
    type Output = Motor;

    fn sub(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl SubAssign<Rotor> for Motor {
    fn sub_assign(&mut self, other: Rotor) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Rotor> for Motor {
    type Output = Motor;

    fn geometric_product(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) + swizzle!(self.group0(), 0, 0, 2, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<Rotor> for Motor {
    type Output = Motor;

    fn outer_product(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + swizzle!(self.group0(), 0, 0, 2, 3) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) } }
    }
}

impl InnerProduct<Rotor> for Motor {
    type Output = Motor;

    fn inner_product(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + swizzle!(self.group0(), 0, 0, 2, 3) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) } }
    }
}

impl LeftContraction<Rotor> for Motor {
    type Output = Rotor;

    fn left_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() + Simd32x2::from([self.group0()[1], self.group0()[0]]) * swizzle!(other.group0(), 1, 0) * Simd32x2::from([-1.0, 0.0]) } }
    }
}

impl RightContraction<Rotor> for Motor {
    type Output = Motor;

    fn right_contraction(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + swizzle!(self.group0(), 0, 0, 2, 3) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl ScalarProduct<Rotor> for Motor {
    type Output = Scalar;

    fn scalar_product(self, other: Rotor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] } }
    }
}

impl Into<Point> for Motor {
    fn into(self) -> Point {
        Point { groups: PointGroups { g0: Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]) } }
    }
}

impl Add<Point> for Motor {
    type Output = Motor;

    fn add(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() + Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl AddAssign<Point> for Motor {
    fn add_assign(&mut self, other: Point) {
        *self = (*self).add(other);
    }
}

impl Sub<Point> for Motor {
    type Output = Motor;

    fn sub(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() - Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl SubAssign<Point> for Motor {
    fn sub_assign(&mut self, other: Point) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Point> for Motor {
    type Output = Motor;

    fn geometric_product(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Point> for Motor {
    type Output = Plane;

    fn regressive_product(self, other: Point) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[3]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[1]]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<Point> for Motor {
    type Output = Point;

    fn outer_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl InnerProduct<Point> for Motor {
    type Output = Motor;

    fn inner_product(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: swizzle!(self.group0(), 1, 0, 0, 0) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl LeftContraction<Point> for Motor {
    type Output = Motor;

    fn left_contraction(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: swizzle!(self.group0(), 1, 0, 0, 0) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl RightContraction<Point> for Motor {
    type Output = Scalar;

    fn right_contraction(self, other: Point) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[1] * other.group0()[0] } }
    }
}

impl ScalarProduct<Point> for Motor {
    type Output = Scalar;

    fn scalar_product(self, other: Point) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[1] * other.group0()[0] } }
    }
}

impl Into<IdealPoint> for Motor {
    fn into(self) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x2::from([self.group0()[2], self.group0()[3]]) } }
    }
}

impl Add<IdealPoint> for Motor {
    type Output = Motor;

    fn add(self, other: IdealPoint) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() + Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[0], other.group0()[1]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl AddAssign<IdealPoint> for Motor {
    fn add_assign(&mut self, other: IdealPoint) {
        *self = (*self).add(other);
    }
}

impl Sub<IdealPoint> for Motor {
    type Output = Motor;

    fn sub(self, other: IdealPoint) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() - Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[0], other.group0()[1]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl SubAssign<IdealPoint> for Motor {
    fn sub_assign(&mut self, other: IdealPoint) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<IdealPoint> for Motor {
    type Output = IdealPoint;

    fn geometric_product(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() + Simd32x2::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0) * Simd32x2::from([-1.0, 1.0]) } }
    }
}

impl RegressiveProduct<IdealPoint> for Motor {
    type Output = Plane;

    fn regressive_product(self, other: IdealPoint) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[3]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from([self.group0()[2], self.group0()[1], self.group0()[1]]) * Simd32x3::from([other.group0()[1], other.group0()[1], other.group0()[0]]) * Simd32x3::from([-1.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<IdealPoint> for Motor {
    type Output = IdealPoint;

    fn outer_product(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() } }
    }
}

impl InnerProduct<IdealPoint> for Motor {
    type Output = IdealPoint;

    fn inner_product(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() } }
    }
}

impl LeftContraction<IdealPoint> for Motor {
    type Output = IdealPoint;

    fn left_contraction(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() } }
    }
}

impl GeometricProduct<Plane> for Motor {
    type Output = MotorDual;

    fn geometric_product(self, other: Plane) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[1]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[2], other.group0()[1], other.group0()[2], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Plane> for Motor {
    type Output = Scalar;

    fn regressive_product(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group0()[0] + self.group0()[2] * other.group0()[1] + self.group0()[3] * other.group0()[2] } }
    }
}

impl OuterProduct<Plane> for Motor {
    type Output = MotorDual;

    fn outer_product(self, other: Plane) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl InnerProduct<Plane> for Motor {
    type Output = Plane;

    fn inner_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() + Simd32x3::from(self.group0()[3]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([-1.0, 0.0, 0.0]) + Simd32x3::from([self.group0()[2], self.group0()[1], self.group0()[1]]) * swizzle!(other.group0(), 2, 2, 1) * Simd32x3::from([1.0, -1.0, 1.0]) } }
    }
}

impl LeftContraction<Plane> for Motor {
    type Output = Plane;

    fn left_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl RightContraction<Plane> for Motor {
    type Output = Plane;

    fn right_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[3]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([-1.0, 0.0, 0.0]) + Simd32x3::from([self.group0()[2], self.group0()[1], self.group0()[1]]) * swizzle!(other.group0(), 2, 2, 1) * Simd32x3::from([1.0, -1.0, 1.0]) } }
    }
}

impl Into<Translator> for Motor {
    fn into(self) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[2], self.group0()[3]]) } }
    }
}

impl Add<Translator> for Motor {
    type Output = Motor;

    fn add(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() + Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl AddAssign<Translator> for Motor {
    fn add_assign(&mut self, other: Translator) {
        *self = (*self).add(other);
    }
}

impl Sub<Translator> for Motor {
    type Output = Motor;

    fn sub(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() - Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl SubAssign<Translator> for Motor {
    fn sub_assign(&mut self, other: Translator) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Translator> for Motor {
    type Output = Motor;

    fn geometric_product(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Translator> for Motor {
    type Output = Plane;

    fn regressive_product(self, other: Translator) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[3]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from([self.group0()[2], self.group0()[1], self.group0()[1]]) * swizzle!(other.group0(), 2, 2, 1) * Simd32x3::from([-1.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<Translator> for Motor {
    type Output = Motor;

    fn outer_product(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 0, 1, 0, 0) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl InnerProduct<Translator> for Motor {
    type Output = Motor;

    fn inner_product(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 0, 1, 0, 0) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl LeftContraction<Translator> for Motor {
    type Output = Translator;

    fn left_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl RightContraction<Translator> for Motor {
    type Output = Motor;

    fn right_contraction(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl ScalarProduct<Translator> for Motor {
    type Output = Scalar;

    fn scalar_product(self, other: Translator) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] } }
    }
}

impl Add<Motor> for Motor {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() + other.group0() } }
    }
}

impl AddAssign<Motor> for Motor {
    fn add_assign(&mut self, other: Motor) {
        *self = (*self).add(other);
    }
}

impl Sub<Motor> for Motor {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() - other.group0() } }
    }
}

impl SubAssign<Motor> for Motor {
    fn sub_assign(&mut self, other: Motor) {
        *self = (*self).sub(other);
    }
}

impl Mul<Motor> for Motor {
    type Output = Motor;

    fn mul(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * other.group0() } }
    }
}

impl MulAssign<Motor> for Motor {
    fn mul_assign(&mut self, other: Motor) {
        *self = (*self).mul(other);
    }
}

impl Div<Motor> for Motor {
    type Output = Motor;

    fn div(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl DivAssign<Motor> for Motor {
    fn div_assign(&mut self, other: Motor) {
        *self = (*self).div(other);
    }
}

impl GeometricProduct<Motor> for Motor {
    type Output = Motor;

    fn geometric_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 1, 1, 1, 0) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) + swizzle!(self.group0(), 0, 0, 2, 2) * swizzle!(other.group0(), 0, 0, 0, 1) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) } }
    }
}

impl RegressiveProduct<Motor> for Motor {
    type Output = Plane;

    fn regressive_product(self, other: Motor) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[3], other.group0()[3], other.group0()[1]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[2], other.group0()[1], other.group0()[2]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[1]]) * Simd32x3::from([other.group0()[0], other.group0()[3], other.group0()[2]]) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<Motor> for Motor {
    type Output = Motor;

    fn outer_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + self.group0() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl InnerProduct<Motor> for Motor {
    type Output = Motor;

    fn inner_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + swizzle!(self.group0(), 1, 1, 2, 3) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl LeftContraction<Motor> for Motor {
    type Output = Motor;

    fn left_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl RightContraction<Motor> for Motor {
    type Output = Motor;

    fn right_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + swizzle!(self.group0(), 0, 0, 2, 3) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl ScalarProduct<Motor> for Motor {
    type Output = Scalar;

    fn scalar_product(self, other: Motor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] } }
    }
}

impl Add<MotorDual> for Motor {
    type Output = MultiVector;

    fn add(self, other: MotorDual) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: swizzle!(self.group0(), 0, 1, 0, 0) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + swizzle!(other.group0(), 0, 0, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]), g1: swizzle!(self.group0(), 0, 0, 2, 3) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) + swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl Sub<MotorDual> for Motor {
    type Output = MultiVector;

    fn sub(self, other: MotorDual) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: swizzle!(self.group0(), 0, 1, 0, 0) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) - swizzle!(other.group0(), 0, 0, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]), g1: swizzle!(self.group0(), 0, 0, 2, 3) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) - swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl GeometricProduct<MotorDual> for Motor {
    type Output = MotorDual;

    fn geometric_product(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 2, 3, 3) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + swizzle!(self.group0(), 2, 2, 0, 0) * swizzle!(other.group0(), 2, 3, 0, 0) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl RegressiveProduct<MotorDual> for Motor {
    type Output = Motor;

    fn regressive_product(self, other: MotorDual) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl OuterProduct<MotorDual> for Motor {
    type Output = MotorDual;

    fn outer_product(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<MotorDual> for Motor {
    type Output = MotorDual;

    fn inner_product(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, -1.0, 0.0, 0.0]) + swizzle!(self.group0(), 0, 1, 1, 1) * swizzle!(other.group0(), 0, 0, 3, 2) * Simd32x4::from([0.0, -1.0, -1.0, 1.0]) } }
    }
}

impl LeftContraction<MotorDual> for Motor {
    type Output = MotorDual;

    fn left_contraction(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + swizzle!(self.group0(), 0, 1, 0, 0) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, -1.0, 0.0, 0.0]) } }
    }
}

impl RightContraction<MotorDual> for Motor {
    type Output = Plane;

    fn right_contraction(self, other: MotorDual) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[3]) * Simd32x3::from(other.group0()[2]) * Simd32x3::from([-1.0, 0.0, 0.0]) + Simd32x3::from([self.group0()[2], self.group0()[1], self.group0()[1]]) * Simd32x3::from([other.group0()[3], other.group0()[3], other.group0()[2]]) * Simd32x3::from([1.0, -1.0, 1.0]) } }
    }
}

impl SquaredMagnitude for Motor {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for Motor {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl Scale for Motor {
    type Output = Motor;

    fn scale(self, other: f32) -> Motor {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Signum for Motor {
    type Output = Motor;

    fn signum(self) -> Motor {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.magnitude().group0() } })
    }
}

impl Inverse for Motor {
    type Output = Motor;

    fn inverse(self) -> Motor {
        self.reversal().geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.squared_magnitude().group0() } })
    }
}

impl Zero for MotorDual {
    fn zero() -> Self {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(0.0) } }
    }
}

impl One for MotorDual {
    fn one() -> Self {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(0.0) } }
    }
}

impl Neg for MotorDual {
    type Output = MotorDual;

    fn neg(self) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
    }
}

impl Automorphism for MotorDual {
    type Output = MotorDual;

    fn automorphism(self) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
    }
}

impl Reversal for MotorDual {
    type Output = MotorDual;

    fn reversal(self) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl Conjugation for MotorDual {
    type Output = MotorDual;

    fn conjugation(self) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) } }
    }
}

impl Dual for MotorDual {
    type Output = Motor;

    fn dual(self) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() } }
    }
}

impl GeometricProduct<Scalar> for MotorDual {
    type Output = MotorDual;

    fn geometric_product(self, other: Scalar) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl RegressiveProduct<Scalar> for MotorDual {
    type Output = Scalar;

    fn regressive_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl OuterProduct<Scalar> for MotorDual {
    type Output = MotorDual;

    fn outer_product(self, other: Scalar) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for MotorDual {
    type Output = MotorDual;

    fn inner_product(self, other: Scalar) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl RightContraction<Scalar> for MotorDual {
    type Output = MotorDual;

    fn right_contraction(self, other: Scalar) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl Add<MultiVector> for MotorDual {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: swizzle!(self.group0(), 0, 0, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) + other.group0(), g1: swizzle!(self.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + other.group1() } }
    }
}

impl Sub<MultiVector> for MotorDual {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: swizzle!(self.group0(), 0, 0, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) - other.group0(), g1: swizzle!(self.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) - other.group1() } }
    }
}

impl GeometricProduct<MultiVector> for MotorDual {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 2, 3, 0, 1), g1: Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[1]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 3, 2, 1, 0) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group1(), 2, 3, 0, 1) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl RegressiveProduct<MultiVector> for MotorDual {
    type Output = MultiVector;

    fn regressive_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 2, 2, 2, 1) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group1(), 3, 3, 1, 3) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group1(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl ScalarProduct<MultiVector> for MotorDual {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[2] * other.group0()[3] + self.group0()[3] * other.group0()[2] } }
    }
}

impl GeometricProduct<Rotor> for MotorDual {
    type Output = MotorDual;

    fn geometric_product(self, other: Rotor) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) + swizzle!(self.group0(), 0, 0, 2, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl RegressiveProduct<Rotor> for MotorDual {
    type Output = Rotor;

    fn regressive_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() + Simd32x2::from([self.group0()[1], self.group0()[0]]) * swizzle!(other.group0(), 1, 0) * Simd32x2::from([1.0, 0.0]) } }
    }
}

impl OuterProduct<Rotor> for MotorDual {
    type Output = MotorDual;

    fn outer_product(self, other: Rotor) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + swizzle!(self.group0(), 0, 0, 2, 3) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 1.0, 1.0]) } }
    }
}

impl InnerProduct<Rotor> for MotorDual {
    type Output = MotorDual;

    fn inner_product(self, other: Rotor) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) + swizzle!(self.group0(), 0, 0, 2, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl RightContraction<Rotor> for MotorDual {
    type Output = MotorDual;

    fn right_contraction(self, other: Rotor) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + swizzle!(self.group0(), 0, 0, 2, 3) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[0], other.group0()[0]]) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) } }
    }
}

impl GeometricProduct<Point> for MotorDual {
    type Output = MotorDual;

    fn geometric_product(self, other: Point) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[2], other.group0()[1], other.group0()[0], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) } }
    }
}

impl RegressiveProduct<Point> for MotorDual {
    type Output = Motor;

    fn regressive_product(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl InnerProduct<Point> for MotorDual {
    type Output = Plane;

    fn inner_product(self, other: Point) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[3]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from([-1.0, 0.0, 0.0]) } }
    }
}

impl LeftContraction<Point> for MotorDual {
    type Output = Plane;

    fn left_contraction(self, other: Point) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[3]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[2]]) * swizzle!(other.group0(), 2, 0, 0) * Simd32x3::from([1.0, 0.0, -1.0]) } }
    }
}

impl RegressiveProduct<IdealPoint> for MotorDual {
    type Output = Translator;

    fn regressive_product(self, other: IdealPoint) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from(self.group0()[3]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[0]]) * Simd32x3::from([other.group0()[0], other.group0()[0], other.group0()[1]]) } }
    }
}

impl Into<Plane> for MotorDual {
    fn into(self) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]) } }
    }
}

impl Add<Plane> for MotorDual {
    type Output = MotorDual;

    fn add(self, other: Plane) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: self.group0() + Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl AddAssign<Plane> for MotorDual {
    fn add_assign(&mut self, other: Plane) {
        *self = (*self).add(other);
    }
}

impl Sub<Plane> for MotorDual {
    type Output = MotorDual;

    fn sub(self, other: Plane) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: self.group0() - Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl SubAssign<Plane> for MotorDual {
    fn sub_assign(&mut self, other: Plane) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Plane> for MotorDual {
    type Output = Motor;

    fn geometric_product(self, other: Plane) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[2], other.group0()[1]]) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[2], other.group0()[1], other.group0()[0], other.group0()[2]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Plane> for MotorDual {
    type Output = Plane;

    fn regressive_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl OuterProduct<Plane> for MotorDual {
    type Output = Point;

    fn outer_product(self, other: Plane) -> Point {
        Point { groups: PointGroups { g0: Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[3]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[1]]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl InnerProduct<Plane> for MotorDual {
    type Output = Translator;

    fn inner_product(self, other: Plane) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from(self.group0()[3]) * Simd32x3::from(other.group0()[2]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[0]]) * swizzle!(other.group0(), 1, 1, 2) } }
    }
}

impl LeftContraction<Plane> for MotorDual {
    type Output = Scalar;

    fn left_contraction(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[2] * other.group0()[1] + self.group0()[3] * other.group0()[2] } }
    }
}

impl RightContraction<Plane> for MotorDual {
    type Output = Translator;

    fn right_contraction(self, other: Plane) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from(self.group0()[3]) * Simd32x3::from(other.group0()[2]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[0]]) * swizzle!(other.group0(), 1, 1, 2) } }
    }
}

impl ScalarProduct<Plane> for MotorDual {
    type Output = Scalar;

    fn scalar_product(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[2] * other.group0()[1] + self.group0()[3] * other.group0()[2] } }
    }
}

impl GeometricProduct<Translator> for MotorDual {
    type Output = MotorDual;

    fn geometric_product(self, other: Translator) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[0], other.group0()[1]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[2], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + swizzle!(self.group0(), 0, 1, 0, 0) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl RegressiveProduct<Translator> for MotorDual {
    type Output = Translator;

    fn regressive_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() + Simd32x3::from(self.group0()[3]) * Simd32x3::from(other.group0()[2]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[0]]) * swizzle!(other.group0(), 1, 0, 0) * Simd32x3::from([1.0, 0.0, 0.0]) } }
    }
}

impl OuterProduct<Translator> for MotorDual {
    type Output = MotorDual;

    fn outer_product(self, other: Translator) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 0, 1, 0, 0) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<Translator> for MotorDual {
    type Output = MotorDual;

    fn inner_product(self, other: Translator) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + swizzle!(self.group0(), 0, 1, 0, 0) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl RightContraction<Translator> for MotorDual {
    type Output = MotorDual;

    fn right_contraction(self, other: Translator) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl Add<Motor> for MotorDual {
    type Output = MultiVector;

    fn add(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: swizzle!(self.group0(), 0, 0, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) + swizzle!(other.group0(), 0, 1, 0, 0) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]), g1: swizzle!(self.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + swizzle!(other.group0(), 0, 0, 2, 3) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl Sub<Motor> for MotorDual {
    type Output = MultiVector;

    fn sub(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: swizzle!(self.group0(), 0, 0, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) - swizzle!(other.group0(), 0, 1, 0, 0) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]), g1: swizzle!(self.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) - swizzle!(other.group0(), 0, 0, 2, 3) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl GeometricProduct<Motor> for MotorDual {
    type Output = MotorDual;

    fn geometric_product(self, other: Motor) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 0, 1, 0, 0) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) } }
    }
}

impl RegressiveProduct<Motor> for MotorDual {
    type Output = Motor;

    fn regressive_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl OuterProduct<Motor> for MotorDual {
    type Output = MotorDual;

    fn outer_product(self, other: Motor) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<Motor> for MotorDual {
    type Output = MotorDual;

    fn inner_product(self, other: Motor) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 0, 1) * Simd32x4::from([0.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 2, 2, 1, 0) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 0, 1, 0, 0) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) } }
    }
}

impl LeftContraction<Motor> for MotorDual {
    type Output = Plane;

    fn left_contraction(self, other: Motor) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[2], other.group0()[1], other.group0()[2]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[2]]) * Simd32x3::from([other.group0()[3], other.group0()[0], other.group0()[1]]) * Simd32x3::from([1.0, 0.0, -1.0]) } }
    }
}

impl RightContraction<Motor> for MotorDual {
    type Output = MotorDual;

    fn right_contraction(self, other: Motor) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + swizzle!(self.group0(), 0, 0, 2, 3) * swizzle!(other.group0(), 0, 1, 0, 0) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) } }
    }
}

impl Add<MotorDual> for MotorDual {
    type Output = MotorDual;

    fn add(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: self.group0() + other.group0() } }
    }
}

impl AddAssign<MotorDual> for MotorDual {
    fn add_assign(&mut self, other: MotorDual) {
        *self = (*self).add(other);
    }
}

impl Sub<MotorDual> for MotorDual {
    type Output = MotorDual;

    fn sub(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: self.group0() - other.group0() } }
    }
}

impl SubAssign<MotorDual> for MotorDual {
    fn sub_assign(&mut self, other: MotorDual) {
        *self = (*self).sub(other);
    }
}

impl Mul<MotorDual> for MotorDual {
    type Output = MotorDual;

    fn mul(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: self.group0() * other.group0() } }
    }
}

impl MulAssign<MotorDual> for MotorDual {
    fn mul_assign(&mut self, other: MotorDual) {
        *self = (*self).mul(other);
    }
}

impl Div<MotorDual> for MotorDual {
    type Output = MotorDual;

    fn div(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl DivAssign<MotorDual> for MotorDual {
    fn div_assign(&mut self, other: MotorDual) {
        *self = (*self).div(other);
    }
}

impl GeometricProduct<MotorDual> for MotorDual {
    type Output = Motor;

    fn geometric_product(self, other: MotorDual) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 0, 0, 2, 3) * Simd32x4::from([0.0, 0.0, 1.0, 1.0]) } }
    }
}

impl RegressiveProduct<MotorDual> for MotorDual {
    type Output = MotorDual;

    fn regressive_product(self, other: MotorDual) -> MotorDual {
        MotorDual { groups: MotorDualGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + self.group0() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl OuterProduct<MotorDual> for MotorDual {
    type Output = Point;

    fn outer_product(self, other: MotorDual) -> Point {
        Point { groups: PointGroups { g0: Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[3], other.group0()[3], other.group0()[1]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[2], other.group0()[1], other.group0()[2]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[1]]) * Simd32x3::from([other.group0()[0], other.group0()[3], other.group0()[2]]) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl InnerProduct<MotorDual> for MotorDual {
    type Output = Translator;

    fn inner_product(self, other: MotorDual) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[2]]) * Simd32x3::from([1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[3], other.group0()[3], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[3]]) * Simd32x3::from([0.0, 1.0, 1.0]) } }
    }
}

impl LeftContraction<MotorDual> for MotorDual {
    type Output = Translator;

    fn left_contraction(self, other: MotorDual) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[3], other.group0()[3], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, 1.0]) + Simd32x3::from([self.group0()[2], self.group0()[2], self.group0()[0]]) * Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[0]]) * Simd32x3::from([1.0, 1.0, 0.0]) } }
    }
}

impl RightContraction<MotorDual> for MotorDual {
    type Output = Translator;

    fn right_contraction(self, other: MotorDual) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x3::from(self.group0()[3]) * Simd32x3::from(other.group0()[3]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[0]]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[3]]) } }
    }
}

impl ScalarProduct<MotorDual> for MotorDual {
    type Output = Scalar;

    fn scalar_product(self, other: MotorDual) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group0()[3] } }
    }
}

impl SquaredMagnitude for MotorDual {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for MotorDual {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl Scale for MotorDual {
    type Output = MotorDual;

    fn scale(self, other: f32) -> MotorDual {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Signum for MotorDual {
    type Output = MotorDual;

    fn signum(self) -> MotorDual {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.magnitude().group0() } })
    }
}

impl Inverse for MotorDual {
    type Output = MotorDual;

    fn inverse(self) -> MotorDual {
        self.reversal().geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.squared_magnitude().group0() } })
    }
}

impl GeometricQuotient<Motor> for IdealPoint {
    type Output = IdealPoint;

    fn geometric_quotient(self, other: Motor) -> IdealPoint {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Point> for IdealPoint {
    type Output = IdealPoint;

    fn geometric_quotient(self, other: Point) -> IdealPoint {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Rotor> for IdealPoint {
    type Output = IdealPoint;

    fn geometric_quotient(self, other: Rotor) -> IdealPoint {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Scalar> for IdealPoint {
    type Output = IdealPoint;

    fn geometric_quotient(self, other: Scalar) -> IdealPoint {
        self.geometric_product(other.inverse())
    }
}

impl GeometricQuotient<Translator> for IdealPoint {
    type Output = IdealPoint;

    fn geometric_quotient(self, other: Translator) -> IdealPoint {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<IdealPoint> for Motor {
    type Output = IdealPoint;

    fn transformation(self, other: IdealPoint) -> IdealPoint {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Powi for Motor {
    type Output = Motor;

    fn powi(self, exponent: isize) -> Motor {
        if exponent == 0 {
            return Motor::one();
        }
        let mut x: Motor = if exponent < 0 { self.inverse() } else { self };
        let mut y: Motor = Motor::one();
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

impl GeometricQuotient<Motor> for Motor {
    type Output = Motor;

    fn geometric_quotient(self, other: Motor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Motor> for Motor {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MotorDual> for Motor {
    type Output = MotorDual;

    fn geometric_quotient(self, other: MotorDual) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MotorDual> for Motor {
    type Output = MotorDual;

    fn transformation(self, other: MotorDual) -> MotorDual {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MultiVector> for Motor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for Motor {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Plane> for Motor {
    type Output = MotorDual;

    fn geometric_quotient(self, other: Plane) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Plane> for Motor {
    type Output = Plane;

    fn transformation(self, other: Plane) -> Plane {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Point> for Motor {
    type Output = Motor;

    fn geometric_quotient(self, other: Point) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Point> for Motor {
    type Output = Point;

    fn transformation(self, other: Point) -> Point {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Rotor> for Motor {
    type Output = Motor;

    fn geometric_quotient(self, other: Rotor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Rotor> for Motor {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Scalar> for Motor {
    type Output = Motor;

    fn geometric_quotient(self, other: Scalar) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Scalar> for Motor {
    type Output = Scalar;

    fn transformation(self, other: Scalar) -> Scalar {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Translator> for Motor {
    type Output = Motor;

    fn geometric_quotient(self, other: Translator) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Translator> for Motor {
    type Output = Translator;

    fn transformation(self, other: Translator) -> Translator {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Motor> for MotorDual {
    type Output = MotorDual;

    fn geometric_quotient(self, other: Motor) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Motor> for MotorDual {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MotorDual> for MotorDual {
    type Output = Motor;

    fn geometric_quotient(self, other: MotorDual) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MotorDual> for MotorDual {
    type Output = MotorDual;

    fn transformation(self, other: MotorDual) -> MotorDual {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MultiVector> for MotorDual {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for MotorDual {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Plane> for MotorDual {
    type Output = Motor;

    fn geometric_quotient(self, other: Plane) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Plane> for MotorDual {
    type Output = Plane;

    fn transformation(self, other: Plane) -> Plane {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Point> for MotorDual {
    type Output = MotorDual;

    fn geometric_quotient(self, other: Point) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Point> for MotorDual {
    type Output = Point;

    fn transformation(self, other: Point) -> Point {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Rotor> for MotorDual {
    type Output = MotorDual;

    fn geometric_quotient(self, other: Rotor) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Rotor> for MotorDual {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Scalar> for MotorDual {
    type Output = MotorDual;

    fn geometric_quotient(self, other: Scalar) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Scalar> for MotorDual {
    type Output = Scalar;

    fn transformation(self, other: Scalar) -> Scalar {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Translator> for MotorDual {
    type Output = MotorDual;

    fn geometric_quotient(self, other: Translator) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Translator> for MotorDual {
    type Output = Translator;

    fn transformation(self, other: Translator) -> Translator {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Motor> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Motor> for MultiVector {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<MotorDual> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MotorDual) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MotorDual> for MultiVector {
    type Output = MotorDual;

    fn transformation(self, other: MotorDual) -> MotorDual {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Powi for MultiVector {
    type Output = MultiVector;

    fn powi(self, exponent: isize) -> MultiVector {
        if exponent == 0 {
            return MultiVector::one();
        }
        let mut x: MultiVector = if exponent < 0 { self.inverse() } else { self };
        let mut y: MultiVector = MultiVector::one();
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

impl GeometricQuotient<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Plane> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Plane> for MultiVector {
    type Output = Plane;

    fn transformation(self, other: Plane) -> Plane {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Point> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Point) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Point> for MultiVector {
    type Output = Point;

    fn transformation(self, other: Point) -> Point {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Rotor> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Rotor> for MultiVector {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Scalar> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Scalar) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Scalar> for MultiVector {
    type Output = Scalar;

    fn transformation(self, other: Scalar) -> Scalar {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Translator> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Translator> for MultiVector {
    type Output = Translator;

    fn transformation(self, other: Translator) -> Translator {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Motor> for Plane {
    type Output = MotorDual;

    fn geometric_quotient(self, other: Motor) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Motor> for Plane {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MotorDual> for Plane {
    type Output = Motor;

    fn geometric_quotient(self, other: MotorDual) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MotorDual> for Plane {
    type Output = MotorDual;

    fn transformation(self, other: MotorDual) -> MotorDual {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MultiVector> for Plane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for Plane {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Plane> for Plane {
    type Output = Motor;

    fn geometric_quotient(self, other: Plane) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Plane> for Plane {
    type Output = Plane;

    fn transformation(self, other: Plane) -> Plane {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Point> for Plane {
    type Output = MotorDual;

    fn geometric_quotient(self, other: Point) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Point> for Plane {
    type Output = Point;

    fn transformation(self, other: Point) -> Point {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Rotor> for Plane {
    type Output = MotorDual;

    fn geometric_quotient(self, other: Rotor) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Rotor> for Plane {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Scalar> for Plane {
    type Output = Plane;

    fn geometric_quotient(self, other: Scalar) -> Plane {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Scalar> for Plane {
    type Output = Scalar;

    fn transformation(self, other: Scalar) -> Scalar {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Translator> for Plane {
    type Output = MotorDual;

    fn geometric_quotient(self, other: Translator) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Translator> for Plane {
    type Output = Translator;

    fn transformation(self, other: Translator) -> Translator {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Transformation<IdealPoint> for Point {
    type Output = IdealPoint;

    fn transformation(self, other: IdealPoint) -> IdealPoint {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Motor> for Point {
    type Output = Motor;

    fn geometric_quotient(self, other: Motor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Motor> for Point {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MotorDual> for Point {
    type Output = MotorDual;

    fn geometric_quotient(self, other: MotorDual) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MotorDual> for Point {
    type Output = MotorDual;

    fn transformation(self, other: MotorDual) -> MotorDual {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MultiVector> for Point {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for Point {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Plane> for Point {
    type Output = MotorDual;

    fn geometric_quotient(self, other: Plane) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Plane> for Point {
    type Output = Plane;

    fn transformation(self, other: Plane) -> Plane {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Point> for Point {
    type Output = Translator;

    fn geometric_quotient(self, other: Point) -> Translator {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Point> for Point {
    type Output = Point;

    fn transformation(self, other: Point) -> Point {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Rotor> for Point {
    type Output = Motor;

    fn geometric_quotient(self, other: Rotor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Rotor> for Point {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Scalar> for Point {
    type Output = Point;

    fn geometric_quotient(self, other: Scalar) -> Point {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Scalar> for Point {
    type Output = Scalar;

    fn transformation(self, other: Scalar) -> Scalar {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Translator> for Point {
    type Output = Point;

    fn geometric_quotient(self, other: Translator) -> Point {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Translator> for Point {
    type Output = Translator;

    fn transformation(self, other: Translator) -> Translator {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Transformation<IdealPoint> for Rotor {
    type Output = IdealPoint;

    fn transformation(self, other: IdealPoint) -> IdealPoint {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Motor> for Rotor {
    type Output = Motor;

    fn geometric_quotient(self, other: Motor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Motor> for Rotor {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MotorDual> for Rotor {
    type Output = MotorDual;

    fn geometric_quotient(self, other: MotorDual) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MotorDual> for Rotor {
    type Output = MotorDual;

    fn transformation(self, other: MotorDual) -> MotorDual {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MultiVector> for Rotor {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for Rotor {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Plane> for Rotor {
    type Output = MotorDual;

    fn geometric_quotient(self, other: Plane) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Plane> for Rotor {
    type Output = Plane;

    fn transformation(self, other: Plane) -> Plane {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Point> for Rotor {
    type Output = Motor;

    fn geometric_quotient(self, other: Point) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Point> for Rotor {
    type Output = Point;

    fn transformation(self, other: Point) -> Point {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Powi for Rotor {
    type Output = Rotor;

    fn powi(self, exponent: isize) -> Rotor {
        if exponent == 0 {
            return Rotor::one();
        }
        let mut x: Rotor = if exponent < 0 { self.inverse() } else { self };
        let mut y: Rotor = Rotor::one();
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

impl GeometricQuotient<Rotor> for Rotor {
    type Output = Rotor;

    fn geometric_quotient(self, other: Rotor) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Rotor> for Rotor {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Scalar> for Rotor {
    type Output = Rotor;

    fn geometric_quotient(self, other: Scalar) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Scalar> for Rotor {
    type Output = Scalar;

    fn transformation(self, other: Scalar) -> Scalar {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Translator> for Rotor {
    type Output = Motor;

    fn geometric_quotient(self, other: Translator) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Translator> for Rotor {
    type Output = Translator;

    fn transformation(self, other: Translator) -> Translator {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Transformation<IdealPoint> for Scalar {
    type Output = IdealPoint;

    fn transformation(self, other: IdealPoint) -> IdealPoint {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Motor> for Scalar {
    type Output = Motor;

    fn geometric_quotient(self, other: Motor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Motor> for Scalar {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MotorDual> for Scalar {
    type Output = MotorDual;

    fn geometric_quotient(self, other: MotorDual) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MotorDual> for Scalar {
    type Output = MotorDual;

    fn transformation(self, other: MotorDual) -> MotorDual {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MultiVector> for Scalar {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for Scalar {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Plane> for Scalar {
    type Output = Plane;

    fn geometric_quotient(self, other: Plane) -> Plane {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Plane> for Scalar {
    type Output = Plane;

    fn transformation(self, other: Plane) -> Plane {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Point> for Scalar {
    type Output = Point;

    fn geometric_quotient(self, other: Point) -> Point {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Point> for Scalar {
    type Output = Point;

    fn transformation(self, other: Point) -> Point {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Rotor> for Scalar {
    type Output = Rotor;

    fn geometric_quotient(self, other: Rotor) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Rotor> for Scalar {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
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

impl GeometricQuotient<Translator> for Scalar {
    type Output = Translator;

    fn geometric_quotient(self, other: Translator) -> Translator {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Translator> for Scalar {
    type Output = Translator;

    fn transformation(self, other: Translator) -> Translator {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Transformation<IdealPoint> for Translator {
    type Output = IdealPoint;

    fn transformation(self, other: IdealPoint) -> IdealPoint {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Motor> for Translator {
    type Output = Motor;

    fn geometric_quotient(self, other: Motor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Motor> for Translator {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MotorDual> for Translator {
    type Output = MotorDual;

    fn geometric_quotient(self, other: MotorDual) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MotorDual> for Translator {
    type Output = MotorDual;

    fn transformation(self, other: MotorDual) -> MotorDual {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MultiVector> for Translator {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for Translator {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Plane> for Translator {
    type Output = MotorDual;

    fn geometric_quotient(self, other: Plane) -> MotorDual {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Plane> for Translator {
    type Output = Plane;

    fn transformation(self, other: Plane) -> Plane {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Point> for Translator {
    type Output = Point;

    fn geometric_quotient(self, other: Point) -> Point {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Point> for Translator {
    type Output = Point;

    fn transformation(self, other: Point) -> Point {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Rotor> for Translator {
    type Output = Motor;

    fn geometric_quotient(self, other: Rotor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Rotor> for Translator {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Scalar> for Translator {
    type Output = Translator;

    fn geometric_quotient(self, other: Scalar) -> Translator {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Scalar> for Translator {
    type Output = Scalar;

    fn transformation(self, other: Scalar) -> Scalar {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Powi for Translator {
    type Output = Translator;

    fn powi(self, exponent: isize) -> Translator {
        if exponent == 0 {
            return Translator::one();
        }
        let mut x: Translator = if exponent < 0 { self.inverse() } else { self };
        let mut y: Translator = Translator::one();
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

impl GeometricQuotient<Translator> for Translator {
    type Output = Translator;

    fn geometric_quotient(self, other: Translator) -> Translator {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Translator> for Translator {
    type Output = Translator;

    fn transformation(self, other: Translator) -> Translator {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

