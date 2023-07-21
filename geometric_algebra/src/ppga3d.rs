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
    /// 1, e23, -e13, e12
    g0: Simd32x4,
    /// e0, -e023, e013, -e012
    g1: Simd32x4,
    /// e123, e1, e2, e3
    g2: Simd32x4,
    /// e0123, e01, e02, e03
    g3: Simd32x4,
}

#[derive(Clone, Copy)]
pub union MultiVector {
    groups: MultiVectorGroups,
    /// 1, e23, -e13, e12, e0, -e023, e013, -e012, e123, e1, e2, e3, e0123, e01, e02, e03
    elements: [f32; 16],
}

impl MultiVector {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(element0: f32, element1: f32, element2: f32, element3: f32, element4: f32, element5: f32, element6: f32, element7: f32, element8: f32, element9: f32, element10: f32, element11: f32, element12: f32, element13: f32, element14: f32, element15: f32) -> Self {
        Self { elements: [element0, element1, element2, element3, element4, element5, element6, element7, element8, element9, element10, element11, element12, element13, element14, element15] }
    }
    pub const fn from_groups(g0: Simd32x4, g1: Simd32x4, g2: Simd32x4, g3: Simd32x4) -> Self {
        Self { groups: MultiVectorGroups { g0, g1, g2, g3 } }
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
    #[inline(always)]
    pub fn group2(&self) -> Simd32x4 {
        unsafe { self.groups.g2 }
    }
    #[inline(always)]
    pub fn group2_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g2 }
    }
    #[inline(always)]
    pub fn group3(&self) -> Simd32x4 {
        unsafe { self.groups.g3 }
    }
    #[inline(always)]
    pub fn group3_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g3 }
    }
}

const MULTIVECTOR_INDEX_REMAP: [usize; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

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

impl std::convert::From<MultiVector> for [f32; 16] {
    fn from(vector: MultiVector) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7], vector.elements[8], vector.elements[9], vector.elements[10], vector.elements[11], vector.elements[12], vector.elements[13], vector.elements[14], vector.elements[15]] }
    }
}

impl std::convert::From<[f32; 16]> for MultiVector {
    fn from(array: [f32; 16]) -> Self {
        Self { elements: [array[0], array[1], array[2], array[3], array[4], array[5], array[6], array[7], array[8], array[9], array[10], array[11], array[12], array[13], array[14], array[15]] }
    }
}

impl std::fmt::Debug for MultiVector {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("MultiVector")
            .field("1", &self[0])
            .field("e23", &self[1])
            .field("-e13", &self[2])
            .field("e12", &self[3])
            .field("e0", &self[4])
            .field("-e023", &self[5])
            .field("e013", &self[6])
            .field("-e012", &self[7])
            .field("e123", &self[8])
            .field("e1", &self[9])
            .field("e2", &self[10])
            .field("e3", &self[11])
            .field("e0123", &self[12])
            .field("e01", &self[13])
            .field("e02", &self[14])
            .field("e03", &self[15])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct RotorGroups {
    /// 1, e23, -e13, e12
    g0: Simd32x4,
}

#[derive(Clone, Copy)]
pub union Rotor {
    groups: RotorGroups,
    /// 1, e23, -e13, e12
    elements: [f32; 4],
}

impl Rotor {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(element0: f32, element1: f32, element2: f32, element3: f32) -> Self {
        Self { elements: [element0, element1, element2, element3] }
    }
    pub const fn from_groups(g0: Simd32x4) -> Self {
        Self { groups: RotorGroups { g0 } }
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

const ROTOR_INDEX_REMAP: [usize; 4] = [0, 1, 2, 3];

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

impl std::convert::From<Rotor> for [f32; 4] {
    fn from(vector: Rotor) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3]] }
    }
}

impl std::convert::From<[f32; 4]> for Rotor {
    fn from(array: [f32; 4]) -> Self {
        Self { elements: [array[0], array[1], array[2], array[3]] }
    }
}

impl std::fmt::Debug for Rotor {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("Rotor")
            .field("1", &self[0])
            .field("e23", &self[1])
            .field("-e13", &self[2])
            .field("e12", &self[3])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct PointGroups {
    /// e123, -e023, e013, -e012
    g0: Simd32x4,
}

#[derive(Clone, Copy)]
pub union Point {
    groups: PointGroups,
    /// e123, -e023, e013, -e012
    elements: [f32; 4],
}

impl Point {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(element0: f32, element1: f32, element2: f32, element3: f32) -> Self {
        Self { elements: [element0, element1, element2, element3] }
    }
    pub const fn from_groups(g0: Simd32x4) -> Self {
        Self { groups: PointGroups { g0 } }
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

const POINT_INDEX_REMAP: [usize; 4] = [0, 1, 2, 3];

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

impl std::convert::From<Point> for [f32; 4] {
    fn from(vector: Point) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3]] }
    }
}

impl std::convert::From<[f32; 4]> for Point {
    fn from(array: [f32; 4]) -> Self {
        Self { elements: [array[0], array[1], array[2], array[3]] }
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("Point")
            .field("e123", &self[0])
            .field("-e023", &self[1])
            .field("e013", &self[2])
            .field("-e012", &self[3])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct IdealPointGroups {
    /// e01, e02, e03
    g0: Simd32x3,
}

#[derive(Clone, Copy)]
pub union IdealPoint {
    groups: IdealPointGroups,
    /// e01, e02, e03, 0
    elements: [f32; 4],
}

impl IdealPoint {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(element0: f32, element1: f32, element2: f32) -> Self {
        Self { elements: [element0, element1, element2, 0.0] }
    }
    pub const fn from_groups(g0: Simd32x3) -> Self {
        Self { groups: IdealPointGroups { g0 } }
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

const IDEALPOINT_INDEX_REMAP: [usize; 3] = [0, 1, 2];

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

impl std::convert::From<IdealPoint> for [f32; 3] {
    fn from(vector: IdealPoint) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2]] }
    }
}

impl std::convert::From<[f32; 3]> for IdealPoint {
    fn from(array: [f32; 3]) -> Self {
        Self { elements: [array[0], array[1], array[2], 0.0] }
    }
}

impl std::fmt::Debug for IdealPoint {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("IdealPoint")
            .field("e01", &self[0])
            .field("e02", &self[1])
            .field("e03", &self[2])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct PlaneGroups {
    /// e0, e1, e2, e3
    g0: Simd32x4,
}

#[derive(Clone, Copy)]
pub union Plane {
    groups: PlaneGroups,
    /// e0, e1, e2, e3
    elements: [f32; 4],
}

impl Plane {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(element0: f32, element1: f32, element2: f32, element3: f32) -> Self {
        Self { elements: [element0, element1, element2, element3] }
    }
    pub const fn from_groups(g0: Simd32x4) -> Self {
        Self { groups: PlaneGroups { g0 } }
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

const PLANE_INDEX_REMAP: [usize; 4] = [0, 1, 2, 3];

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

impl std::convert::From<Plane> for [f32; 4] {
    fn from(vector: Plane) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3]] }
    }
}

impl std::convert::From<[f32; 4]> for Plane {
    fn from(array: [f32; 4]) -> Self {
        Self { elements: [array[0], array[1], array[2], array[3]] }
    }
}

impl std::fmt::Debug for Plane {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("Plane")
            .field("e0", &self[0])
            .field("e1", &self[1])
            .field("e2", &self[2])
            .field("e3", &self[3])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct LineGroups {
    /// e01, e02, e03
    g0: Simd32x3,
    /// e23, -e13, e12
    g1: Simd32x3,
}

#[derive(Clone, Copy)]
pub union Line {
    groups: LineGroups,
    /// e01, e02, e03, 0, e23, -e13, e12, 0
    elements: [f32; 8],
}

impl Line {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(element0: f32, element1: f32, element2: f32, element3: f32, element4: f32, element5: f32) -> Self {
        Self { elements: [element0, element1, element2, 0.0, element3, element4, element5, 0.0] }
    }
    pub const fn from_groups(g0: Simd32x3, g1: Simd32x3) -> Self {
        Self { groups: LineGroups { g0, g1 } }
    }
    #[inline(always)]
    pub fn group0(&self) -> Simd32x3 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g0 }
    }
    #[inline(always)]
    pub fn group1(&self) -> Simd32x3 {
        unsafe { self.groups.g1 }
    }
    #[inline(always)]
    pub fn group1_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g1 }
    }
}

const LINE_INDEX_REMAP: [usize; 6] = [0, 1, 2, 4, 5, 6];

impl std::ops::Index<usize> for Line {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[LINE_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for Line {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[LINE_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<Line> for [f32; 6] {
    fn from(vector: Line) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[4], vector.elements[5], vector.elements[6]] }
    }
}

impl std::convert::From<[f32; 6]> for Line {
    fn from(array: [f32; 6]) -> Self {
        Self { elements: [array[0], array[1], array[2], 0.0, array[3], array[4], array[5], 0.0] }
    }
}

impl std::fmt::Debug for Line {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("Line")
            .field("e01", &self[0])
            .field("e02", &self[1])
            .field("e03", &self[2])
            .field("e23", &self[3])
            .field("-e13", &self[4])
            .field("e12", &self[5])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct TranslatorGroups {
    /// 1, e01, e02, e03
    g0: Simd32x4,
}

#[derive(Clone, Copy)]
pub union Translator {
    groups: TranslatorGroups,
    /// 1, e01, e02, e03
    elements: [f32; 4],
}

impl Translator {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(element0: f32, element1: f32, element2: f32, element3: f32) -> Self {
        Self { elements: [element0, element1, element2, element3] }
    }
    pub const fn from_groups(g0: Simd32x4) -> Self {
        Self { groups: TranslatorGroups { g0 } }
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

const TRANSLATOR_INDEX_REMAP: [usize; 4] = [0, 1, 2, 3];

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

impl std::convert::From<Translator> for [f32; 4] {
    fn from(vector: Translator) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3]] }
    }
}

impl std::convert::From<[f32; 4]> for Translator {
    fn from(array: [f32; 4]) -> Self {
        Self { elements: [array[0], array[1], array[2], array[3]] }
    }
}

impl std::fmt::Debug for Translator {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("Translator")
            .field("1", &self[0])
            .field("e01", &self[1])
            .field("e02", &self[2])
            .field("e03", &self[3])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct MotorGroups {
    /// 1, e23, -e13, e12
    g0: Simd32x4,
    /// e0123, e01, e02, e03
    g1: Simd32x4,
}

#[derive(Clone, Copy)]
pub union Motor {
    groups: MotorGroups,
    /// 1, e23, -e13, e12, e0123, e01, e02, e03
    elements: [f32; 8],
}

impl Motor {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(element0: f32, element1: f32, element2: f32, element3: f32, element4: f32, element5: f32, element6: f32, element7: f32) -> Self {
        Self { elements: [element0, element1, element2, element3, element4, element5, element6, element7] }
    }
    pub const fn from_groups(g0: Simd32x4, g1: Simd32x4) -> Self {
        Self { groups: MotorGroups { g0, g1 } }
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

const MOTOR_INDEX_REMAP: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

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

impl std::convert::From<Motor> for [f32; 8] {
    fn from(vector: Motor) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7]] }
    }
}

impl std::convert::From<[f32; 8]> for Motor {
    fn from(array: [f32; 8]) -> Self {
        Self { elements: [array[0], array[1], array[2], array[3], array[4], array[5], array[6], array[7]] }
    }
}

impl std::fmt::Debug for Motor {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("Motor")
            .field("1", &self[0])
            .field("e23", &self[1])
            .field("-e13", &self[2])
            .field("e12", &self[3])
            .field("e0123", &self[4])
            .field("e01", &self[5])
            .field("e02", &self[6])
            .field("e03", &self[7])
            .finish()
    }
}

#[derive(Clone, Copy)]
struct PointAndPlaneGroups {
    /// e123, -e023, e013, -e012
    g0: Simd32x4,
    /// e0, e1, e2, e3
    g1: Simd32x4,
}

#[derive(Clone, Copy)]
pub union PointAndPlane {
    groups: PointAndPlaneGroups,
    /// e123, -e023, e013, -e012, e0, e1, e2, e3
    elements: [f32; 8],
}

impl PointAndPlane {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(element0: f32, element1: f32, element2: f32, element3: f32, element4: f32, element5: f32, element6: f32, element7: f32) -> Self {
        Self { elements: [element0, element1, element2, element3, element4, element5, element6, element7] }
    }
    pub const fn from_groups(g0: Simd32x4, g1: Simd32x4) -> Self {
        Self { groups: PointAndPlaneGroups { g0, g1 } }
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

const POINTANDPLANE_INDEX_REMAP: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

impl std::ops::Index<usize> for PointAndPlane {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[POINTANDPLANE_INDEX_REMAP[index]] }
    }
}

impl std::ops::IndexMut<usize> for PointAndPlane {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[POINTANDPLANE_INDEX_REMAP[index]] }
    }
}

impl std::convert::From<PointAndPlane> for [f32; 8] {
    fn from(vector: PointAndPlane) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7]] }
    }
}

impl std::convert::From<[f32; 8]> for PointAndPlane {
    fn from(array: [f32; 8]) -> Self {
        Self { elements: [array[0], array[1], array[2], array[3], array[4], array[5], array[6], array[7]] }
    }
}

impl std::fmt::Debug for PointAndPlane {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("PointAndPlane")
            .field("e123", &self[0])
            .field("-e023", &self[1])
            .field("e013", &self[2])
            .field("-e012", &self[3])
            .field("e0", &self[4])
            .field("e1", &self[5])
            .field("e2", &self[6])
            .field("e3", &self[7])
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
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + other.group0(), g1: other.group1(), g2: other.group2(), g3: other.group3() } }
    }
}

impl Sub<MultiVector> for Scalar {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) - other.group0(), g1: Simd32x4::from(0.0) - other.group1(), g2: Simd32x4::from(0.0) - other.group2(), g3: Simd32x4::from(0.0) - other.group3() } }
    }
}

impl GeometricProduct<MultiVector> for Scalar {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1(), g2: Simd32x4::from(self.group0()) * other.group2(), g3: Simd32x4::from(self.group0()) * other.group3() } }
    }
}

impl RegressiveProduct<MultiVector> for Scalar {
    type Output = Scalar;

    fn regressive_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group3()[0] } }
    }
}

impl OuterProduct<MultiVector> for Scalar {
    type Output = MultiVector;

    fn outer_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1(), g2: Simd32x4::from(self.group0()) * other.group2(), g3: Simd32x4::from(self.group0()) * other.group3() } }
    }
}

impl InnerProduct<MultiVector> for Scalar {
    type Output = MultiVector;

    fn inner_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1(), g2: Simd32x4::from(self.group0()) * other.group2(), g3: Simd32x4::from(self.group0()) * other.group3() } }
    }
}

impl LeftContraction<MultiVector> for Scalar {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1(), g2: Simd32x4::from(self.group0()) * other.group2(), g3: Simd32x4::from(self.group0()) * other.group3() } }
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
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + other.group0() } }
    }
}

impl Sub<Rotor> for Scalar {
    type Output = Rotor;

    fn sub(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) - other.group0() } }
    }
}

impl GeometricProduct<Rotor> for Scalar {
    type Output = Rotor;

    fn geometric_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl OuterProduct<Rotor> for Scalar {
    type Output = Rotor;

    fn outer_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl InnerProduct<Rotor> for Scalar {
    type Output = Rotor;

    fn inner_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl LeftContraction<Rotor> for Scalar {
    type Output = Rotor;

    fn left_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
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

impl GeometricProduct<Point> for Scalar {
    type Output = Point;

    fn geometric_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl OuterProduct<Point> for Scalar {
    type Output = Point;

    fn outer_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl InnerProduct<Point> for Scalar {
    type Output = Point;

    fn inner_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl LeftContraction<Point> for Scalar {
    type Output = Point;

    fn left_contraction(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl Add<IdealPoint> for Scalar {
    type Output = Translator;

    fn add(self, other: IdealPoint) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl Sub<IdealPoint> for Scalar {
    type Output = Translator;

    fn sub(self, other: IdealPoint) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) - Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl GeometricProduct<IdealPoint> for Scalar {
    type Output = IdealPoint;

    fn geometric_product(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl OuterProduct<IdealPoint> for Scalar {
    type Output = IdealPoint;

    fn outer_product(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl InnerProduct<IdealPoint> for Scalar {
    type Output = IdealPoint;

    fn inner_product(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl LeftContraction<IdealPoint> for Scalar {
    type Output = IdealPoint;

    fn left_contraction(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl GeometricProduct<Plane> for Scalar {
    type Output = Plane;

    fn geometric_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl OuterProduct<Plane> for Scalar {
    type Output = Plane;

    fn outer_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl InnerProduct<Plane> for Scalar {
    type Output = Plane;

    fn inner_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl LeftContraction<Plane> for Scalar {
    type Output = Plane;

    fn left_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl GeometricProduct<Line> for Scalar {
    type Output = Line;

    fn geometric_product(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl OuterProduct<Line> for Scalar {
    type Output = Line;

    fn outer_product(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl InnerProduct<Line> for Scalar {
    type Output = Line;

    fn inner_product(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl LeftContraction<Line> for Scalar {
    type Output = Line;

    fn left_contraction(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl Add<Translator> for Scalar {
    type Output = Translator;

    fn add(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + other.group0() } }
    }
}

impl Sub<Translator> for Scalar {
    type Output = Translator;

    fn sub(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) - other.group0() } }
    }
}

impl GeometricProduct<Translator> for Scalar {
    type Output = Translator;

    fn geometric_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl OuterProduct<Translator> for Scalar {
    type Output = Translator;

    fn outer_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl InnerProduct<Translator> for Scalar {
    type Output = Translator;

    fn inner_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl LeftContraction<Translator> for Scalar {
    type Output = Translator;

    fn left_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
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
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + other.group0(), g1: other.group1() } }
    }
}

impl Sub<Motor> for Scalar {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) - other.group0(), g1: Simd32x4::from(0.0) - other.group1() } }
    }
}

impl GeometricProduct<Motor> for Scalar {
    type Output = Motor;

    fn geometric_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl RegressiveProduct<Motor> for Scalar {
    type Output = Scalar;

    fn regressive_product(self, other: Motor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group1()[0] } }
    }
}

impl OuterProduct<Motor> for Scalar {
    type Output = Motor;

    fn outer_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl InnerProduct<Motor> for Scalar {
    type Output = Motor;

    fn inner_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl LeftContraction<Motor> for Scalar {
    type Output = Motor;

    fn left_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
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

impl GeometricProduct<PointAndPlane> for Scalar {
    type Output = PointAndPlane;

    fn geometric_product(self, other: PointAndPlane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl OuterProduct<PointAndPlane> for Scalar {
    type Output = PointAndPlane;

    fn outer_product(self, other: PointAndPlane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl InnerProduct<PointAndPlane> for Scalar {
    type Output = PointAndPlane;

    fn inner_product(self, other: PointAndPlane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl LeftContraction<PointAndPlane> for Scalar {
    type Output = PointAndPlane;

    fn left_contraction(self, other: PointAndPlane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
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
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(0.0), g1: Simd32x4::from(0.0), g2: Simd32x4::from(0.0), g3: Simd32x4::from(0.0) } }
    }
}

impl One for MultiVector {
    fn one() -> Self {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x4::from(0.0), g3: Simd32x4::from(0.0) } }
    }
}

impl Neg for MultiVector {
    type Output = MultiVector;

    fn neg(self) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x4::from(-1.0), g1: self.group1() * Simd32x4::from(-1.0), g2: self.group2() * Simd32x4::from(-1.0), g3: self.group3() * Simd32x4::from(-1.0) } }
    }
}

impl Automorphism for MultiVector {
    type Output = MultiVector;

    fn automorphism(self) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0(), g1: self.group1() * Simd32x4::from(-1.0), g2: self.group2() * Simd32x4::from(-1.0), g3: self.group3() } }
    }
}

impl Reversal for MultiVector {
    type Output = MultiVector;

    fn reversal(self) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]), g1: self.group1() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]), g2: self.group2() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]), g3: self.group3() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) } }
    }
}

impl Conjugation for MultiVector {
    type Output = MultiVector;

    fn conjugation(self) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]), g1: self.group1() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]), g2: self.group2() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]), g3: self.group3() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) } }
    }
}

impl Dual for MultiVector {
    type Output = MultiVector;

    fn dual(self) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group3(), g1: self.group2() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]), g2: self.group1() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]), g3: self.group0() } }
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
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + Simd32x4::from(other.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: self.group1(), g2: self.group2(), g3: self.group3() } }
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
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - Simd32x4::from(other.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: self.group1(), g2: self.group2(), g3: self.group3() } }
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
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()), g2: self.group2() * Simd32x4::from(other.group0()), g3: self.group3() * Simd32x4::from(other.group0()) } }
    }
}

impl RegressiveProduct<Scalar> for MultiVector {
    type Output = Scalar;

    fn regressive_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group3()[0] * other.group0() } }
    }
}

impl OuterProduct<Scalar> for MultiVector {
    type Output = MultiVector;

    fn outer_product(self, other: Scalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()), g2: self.group2() * Simd32x4::from(other.group0()), g3: self.group3() * Simd32x4::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for MultiVector {
    type Output = MultiVector;

    fn inner_product(self, other: Scalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()), g2: self.group2() * Simd32x4::from(other.group0()), g3: self.group3() * Simd32x4::from(other.group0()) } }
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
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()), g2: self.group2() * Simd32x4::from(other.group0()), g3: self.group3() * Simd32x4::from(other.group0()) } }
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
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + other.group0(), g1: self.group1() + other.group1(), g2: self.group2() + other.group2(), g3: self.group3() + other.group3() } }
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
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - other.group0(), g1: self.group1() - other.group1(), g2: self.group2() - other.group2(), g3: self.group3() - other.group3() } }
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
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * other.group0(), g1: self.group1() * other.group1(), g2: self.group2() * other.group2(), g3: self.group3() * other.group3() } }
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
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]), g2: Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group2()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group2()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]), g3: Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group3()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group3()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) } }
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
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group2()[0]) * other.group2() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) + Simd32x4::from(self.group2()[1]) * swizzle!(other.group2(), 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group2(), 2, 3, 0, 1) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group2()[3]) * swizzle!(other.group2(), 3, 2, 1, 0) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group1(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group2()[0]) * other.group3() + Simd32x4::from(self.group2()[1]) * swizzle!(other.group3(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group3(), 2, 3, 0, 1) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group2()[3]) * swizzle!(other.group3(), 3, 2, 1, 0) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) - Simd32x4::from(self.group3()[0]) * other.group2() + Simd32x4::from(self.group3()[1]) * swizzle!(other.group2(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group3()[2]) * swizzle!(other.group2(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group3()[3]) * swizzle!(other.group2(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]), g2: Simd32x4::from(self.group0()[0]) * other.group2() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group2(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group2(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group2(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group2()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group2()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group2()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]), g3: Simd32x4::from(self.group0()[0]) * other.group3() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group3(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group3(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group3(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * other.group2() + Simd32x4::from(self.group1()[1]) * swizzle!(other.group2(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group2(), 2, 3, 0, 1) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group2(), 3, 2, 1, 0) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) - Simd32x4::from(self.group2()[0]) * other.group1() + Simd32x4::from(self.group2()[1]) * swizzle!(other.group1(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group1(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group2()[3]) * swizzle!(other.group1(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group3()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group3()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group3()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group3()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl RegressiveProduct<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn regressive_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group3(), 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group3(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group3(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group2()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group2(), 1, 0, 1, 1) * Simd32x4::from([-1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group2(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group2(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group2()[0]) * other.group1() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group2()[3]) * Simd32x4::from(other.group1()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[0]) * other.group0() + Simd32x4::from(self.group3()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group3()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.group1()[1]) * swizzle!(other.group3(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group3(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group3(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group3()[0]) * other.group1() + Simd32x4::from(self.group3()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[3]) * Simd32x4::from(other.group1()[3]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group3()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group1(), 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group2()[0]) * other.group3() + Simd32x4::from(self.group2()[2]) * Simd32x4::from(other.group3()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group2()[3]) * Simd32x4::from(other.group3()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group3()[0]) * other.group2() + Simd32x4::from(self.group3()[1]) * Simd32x4::from(other.group2()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from(other.group2()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group3()[3]) * Simd32x4::from(other.group2()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group0()[0], self.group2()[1], self.group0()[1], self.group0()[1]]) * Simd32x4::from([other.group1()[0], other.group3()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]), g3: Simd32x4::from(self.group1()[2]) * swizzle!(other.group1(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group1(), 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group3()[0]) * other.group3() + Simd32x4::from(self.group3()[2]) * Simd32x4::from(other.group3()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group3()[3]) * Simd32x4::from(other.group3()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group1()[0], self.group3()[1], self.group1()[1], self.group1()[1]]) * Simd32x4::from([other.group1()[0], other.group3()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]) } }
    }
}

impl OuterProduct<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn outer_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group2()[1]) * swizzle!(other.group2(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group2(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group2()[3]) * swizzle!(other.group2(), 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + self.group0() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group1()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group2()[1]) * swizzle!(other.group3(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group3(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group2()[3]) * swizzle!(other.group3(), 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group3()[1]) * swizzle!(other.group2(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group3()[2]) * swizzle!(other.group2(), 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group3()[3]) * swizzle!(other.group2(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + self.group0() * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]), g2: Simd32x4::from(self.group0()[0]) * other.group2() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group2()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group2()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group2()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group2()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group2(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g3: Simd32x4::from(self.group0()[0]) * other.group3() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group3()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group3()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * other.group2() + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group2()[1]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group2()[2]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group2()[3]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group2()[1]) * swizzle!(other.group1(), 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group1(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group2()[3]) * swizzle!(other.group1(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group3()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group3(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn inner_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group2()[0]) * other.group2() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) + Simd32x4::from(self.group2()[1]) * swizzle!(other.group2(), 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group2(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group2()[3]) * swizzle!(other.group2(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 1, 1, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group1()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from(other.group3()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group2()[1]) * swizzle!(other.group3(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group3(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group2()[3]) * swizzle!(other.group3(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) - Simd32x4::from(self.group3()[0]) * other.group2() + Simd32x4::from(self.group3()[1]) * Simd32x4::from(other.group2()[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from(other.group2()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[3]) * Simd32x4::from(other.group2()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group1(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: Simd32x4::from(self.group0()[0]) * other.group2() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group2(), 3, 3, 0, 1) * Simd32x4::from([0.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group2(), 2, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group2()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group2()[1]) * swizzle!(other.group0(), 0, 0, 3, 2) * Simd32x4::from([0.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group0(), 3, 3, 0, 1) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group2()[3]) * swizzle!(other.group0(), 2, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]) + swizzle!(self.group0(), 0, 1, 1, 1) * swizzle!(other.group2(), 0, 0, 3, 2) * Simd32x4::from([0.0, -1.0, 1.0, -1.0]), g3: Simd32x4::from(self.group0()[0]) * other.group3() + Simd32x4::from(self.group1()[1]) * swizzle!(other.group2(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group2(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group2(), 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group2()[1]) * swizzle!(other.group1(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group1(), 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group2()[3]) * swizzle!(other.group1(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group3()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group3()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + self.group0() * Simd32x4::from(other.group3()[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) } }
    }
}

impl LeftContraction<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from(other.group2()[0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group2()[1]) * swizzle!(other.group2(), 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group2(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group2()[3]) * swizzle!(other.group2(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group1()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from(other.group3()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group2()[1]) * swizzle!(other.group3(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group3(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group2()[3]) * swizzle!(other.group3(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group1(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: Simd32x4::from(self.group0()[0]) * other.group2() + Simd32x4::from(self.group2()[1]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group2()[3]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + self.group0() * Simd32x4::from(other.group2()[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]), g3: Simd32x4::from(self.group0()[0]) * other.group3() + Simd32x4::from(self.group2()[1]) * swizzle!(other.group1(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group1(), 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group2()[3]) * swizzle!(other.group1(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + self.group0() * Simd32x4::from(other.group3()[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) } }
    }
}

impl RightContraction<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group2()[0]) * other.group2() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from(other.group2()[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from(other.group2()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group2()[3]) * Simd32x4::from(other.group2()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) - Simd32x4::from(self.group3()[0]) * other.group2() + Simd32x4::from(self.group3()[1]) * Simd32x4::from(other.group2()[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from(other.group2()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[3]) * Simd32x4::from(other.group2()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: Simd32x4::from(self.group0()[2]) * swizzle!(other.group2(), 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group2(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group2()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group2()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group0()[0], self.group2()[1], self.group0()[1], self.group0()[1]]) * Simd32x4::from([other.group2()[0], other.group0()[0], other.group2()[3], other.group2()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, -1.0]), g3: Simd32x4::from(self.group1()[2]) * swizzle!(other.group2(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group2(), 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group3()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group3()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group1()[0], self.group3()[1], self.group1()[1], self.group1()[1]]) * Simd32x4::from([other.group2()[0], other.group0()[0], other.group2()[3], other.group2()[2]]) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]) } }
    }
}

impl ScalarProduct<MultiVector> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3] - self.group2()[0] * other.group2()[0] + self.group2()[1] * other.group2()[1] + self.group2()[2] * other.group2()[2] + self.group2()[3] * other.group2()[3] } }
    }
}

impl Into<Rotor> for MultiVector {
    fn into(self) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() } }
    }
}

impl Add<Rotor> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + other.group0(), g1: self.group1(), g2: self.group2(), g3: self.group3() } }
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
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - other.group0(), g1: self.group1(), g2: self.group2(), g3: self.group3() } }
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
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]), g1: Simd32x4::from(self.group1()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]), g2: Simd32x4::from(self.group2()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group2()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group2()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]), g3: Simd32x4::from(self.group3()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group3()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group3()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group3()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl OuterProduct<Rotor> for MultiVector {
    type Output = MultiVector;

    fn outer_product(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + self.group0() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.group1()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + self.group1() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g2: Simd32x4::from(self.group2()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group2()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g3: Simd32x4::from(self.group3()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group3()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<Rotor> for MultiVector {
    type Output = MultiVector;

    fn inner_product(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 1, 1, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]), g1: Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: Simd32x4::from(self.group2()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group0(), 3, 3, 0, 1) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group2()[3]) * swizzle!(other.group0(), 2, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]) + swizzle!(self.group2(), 0, 1, 1, 1) * swizzle!(other.group0(), 0, 0, 3, 2) * Simd32x4::from([0.0, 1.0, 1.0, -1.0]), g3: Simd32x4::from(self.group3()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + self.group3() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl RightContraction<Rotor> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: Simd32x4::from(self.group2()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + self.group2() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g3: Simd32x4::from(self.group3()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + self.group3() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl ScalarProduct<Rotor> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: Rotor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3] } }
    }
}

impl Into<Point> for MultiVector {
    fn into(self) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from([self.group2()[0], self.group1()[1], self.group1()[2], self.group1()[3]]) } }
    }
}

impl Add<Point> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Point) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0(), g1: self.group1() + other.group0() * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g2: self.group2() + Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g3: self.group3() } }
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
        MultiVector { groups: MultiVectorGroups { g0: self.group0(), g1: self.group1() - other.group0() * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g2: self.group2() - Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g3: self.group3() } }
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
        MultiVector { groups: MultiVectorGroups { g0: self.group2() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 1, 3, 2) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 3, 2, 1) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 2, 1, 3) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group3()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from([self.group3()[0], self.group0()[0], self.group0()[0], self.group0()[0]]) * other.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]), g2: self.group0() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]), g3: Simd32x4::from(self.group2()[0]) * swizzle!(other.group0(), 1, 1, 2, 3) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group2()[1]) * swizzle!(other.group0(), 1, 1, 3, 2) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group0(), 2, 3, 2, 1) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group2()[3]) * swizzle!(other.group0(), 3, 2, 1, 3) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + self.group1() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl ScalarProduct<Point> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: Point) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group2()[0] * other.group0()[0] } }
    }
}

impl Into<IdealPoint> for MultiVector {
    fn into(self) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x3::from([self.group3()[1], self.group3()[2], self.group3()[3]]) } }
    }
}

impl Add<IdealPoint> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: IdealPoint) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0(), g1: self.group1(), g2: self.group2(), g3: self.group3() + Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
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
        MultiVector { groups: MultiVectorGroups { g0: self.group0(), g1: self.group1(), g2: self.group2(), g3: self.group3() - Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl SubAssign<IdealPoint> for MultiVector {
    fn sub_assign(&mut self, other: IdealPoint) {
        *self = (*self).sub(other);
    }
}

impl Into<Plane> for MultiVector {
    fn into(self) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from([self.group1()[0], self.group2()[1], self.group2()[2], self.group2()[3]]) } }
    }
}

impl Add<Plane> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0(), g1: self.group1() + Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: self.group2() + other.group0() * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g3: self.group3() } }
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
        MultiVector { groups: MultiVectorGroups { g0: self.group0(), g1: self.group1() - Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: self.group2() - other.group0() * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g3: self.group3() } }
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
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group2()[1]) * swizzle!(other.group0(), 1, 1, 3, 2) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group0(), 2, 3, 2, 1) * Simd32x4::from([1.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group2()[3]) * swizzle!(other.group0(), 3, 2, 1, 3) * Simd32x4::from([1.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group2()[0]) * other.group0() * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.group3()[0]) * swizzle!(other.group0(), 1, 1, 2, 3) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group3()[1]) * swizzle!(other.group0(), 1, 1, 3, 2) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group3()[2]) * swizzle!(other.group0(), 2, 3, 2, 1) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group3()[3]) * swizzle!(other.group0(), 3, 2, 1, 3) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + self.group0() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]), g2: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 1, 3, 2) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 3, 2, 1) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 2, 1, 3) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * other.group0() * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g3: Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 1, 1, 3, 2) * Simd32x4::from([-1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 3, 2, 1) * Simd32x4::from([-1.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 3, 2, 1, 3) * Simd32x4::from([-1.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group2()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from([self.group2()[0], self.group1()[0], self.group1()[0], self.group1()[0]]) * other.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl ScalarProduct<Plane> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group2()[1] * other.group0()[1] + self.group2()[2] * other.group0()[2] + self.group2()[3] * other.group0()[3] } }
    }
}

impl Into<Line> for MultiVector {
    fn into(self) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from([self.group3()[1], self.group3()[2], self.group3()[3]]), g1: Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]) } }
    }
}

impl Add<Line> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + Simd32x4::from([other.group0()[0], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: self.group1(), g2: self.group2(), g3: self.group3() + Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl AddAssign<Line> for MultiVector {
    fn add_assign(&mut self, other: Line) {
        *self = (*self).add(other);
    }
}

impl Sub<Line> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - Simd32x4::from([other.group0()[0], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: self.group1(), g2: self.group2(), g3: self.group3() - Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl SubAssign<Line> for MultiVector {
    fn sub_assign(&mut self, other: Line) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Line> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[2], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[2], other.group1()[1], other.group1()[0]]) * Simd32x4::from([-1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[2], other.group1()[1], other.group1()[0], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[2], other.group1()[1]]) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], other.group1()[2], other.group1()[1], other.group1()[0]]) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group1()[2], other.group1()[1], other.group1()[0], other.group1()[2]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([-1.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group0()[2], other.group0()[1], other.group0()[0], other.group0()[2]]) * Simd32x4::from([-1.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]), g2: Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[2], other.group1()[1]]) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([other.group1()[1], other.group1()[2], other.group1()[1], other.group1()[0]]) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group1()[2], other.group1()[1], other.group1()[0], other.group1()[2]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]), g3: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[2], other.group0()[1], other.group0()[0], other.group0()[2]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[2], other.group1()[1]]) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([other.group1()[1], other.group1()[2], other.group1()[1], other.group1()[0]]) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group3()[3]) * Simd32x4::from([other.group1()[2], other.group1()[1], other.group1()[0], other.group1()[2]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl ScalarProduct<Line> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: Line) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[1] * other.group1()[0] - self.group0()[2] * other.group1()[1] - self.group0()[3] * other.group1()[2] } }
    }
}

impl Into<Translator> for MultiVector {
    fn into(self) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from([self.group0()[0], self.group3()[1], self.group3()[2], self.group3()[3]]) } }
    }
}

impl Add<Translator> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: self.group1(), g2: self.group2(), g3: self.group3() + other.group0() * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
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
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: self.group1(), g2: self.group2(), g3: self.group3() - other.group0() * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
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
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: Simd32x4::from(self.group2()[0]) * swizzle!(other.group0(), 1, 1, 2, 3) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + Simd32x4::from(self.group2()[1]) * swizzle!(other.group0(), 1, 1, 3, 2) * Simd32x4::from([-1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group0(), 2, 3, 2, 1) * Simd32x4::from([-1.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group2()[3]) * swizzle!(other.group0(), 3, 2, 1, 3) * Simd32x4::from([-1.0, -1.0, 1.0, 0.0]) + self.group1() * Simd32x4::from(other.group0()[0]), g2: self.group2() * Simd32x4::from(other.group0()[0]), g3: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 1, 3, 2) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 3, 2, 1) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 2, 1, 3) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group3()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group3()[0], self.group0()[0], self.group0()[0], self.group0()[0]]) * other.group0() } }
    }
}

impl OuterProduct<Translator> for MultiVector {
    type Output = MultiVector;

    fn outer_product(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: Simd32x4::from(self.group2()[1]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group2()[3]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + self.group1() * Simd32x4::from(other.group0()[0]), g2: self.group2() * Simd32x4::from(other.group0()[0]), g3: Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group3()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 1, 2, 3) } }
    }
}

impl InnerProduct<Translator> for MultiVector {
    type Output = MultiVector;

    fn inner_product(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: Simd32x4::from(self.group2()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group2()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + self.group1() * Simd32x4::from(other.group0()[0]), g2: self.group2() * Simd32x4::from(other.group0()[0]), g3: Simd32x4::from(self.group3()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group3()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group3()[0], self.group0()[0], self.group0()[0], self.group0()[0]]) * other.group0() } }
    }
}

impl RightContraction<Translator> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: self.group1() * Simd32x4::from(other.group0()[0]), g2: self.group2() * Simd32x4::from(other.group0()[0]), g3: self.group3() * Simd32x4::from(other.group0()[0]) } }
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
        Motor { groups: MotorGroups { g0: self.group0(), g1: self.group3() } }
    }
}

impl Add<Motor> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + other.group0(), g1: self.group1(), g2: self.group2(), g3: self.group3() + other.group1() } }
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
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - other.group0(), g1: self.group1(), g2: self.group2(), g3: self.group3() - other.group1() } }
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
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]), g1: Simd32x4::from(self.group1()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group2()[0]) * other.group1() + Simd32x4::from(self.group2()[1]) * swizzle!(other.group1(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group1(), 2, 3, 0, 1) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group2()[3]) * swizzle!(other.group1(), 3, 2, 1, 0) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]), g2: Simd32x4::from(self.group2()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group2()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group2()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]), g3: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group1(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group3()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group3()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group3()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group3()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Motor> for MultiVector {
    type Output = MultiVector;

    fn regressive_product(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group1(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group3()[0]) * other.group0() + Simd32x4::from(self.group3()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.group1()[1]) * swizzle!(other.group1(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group1(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group1(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group2()[0]) * other.group1() + Simd32x4::from(self.group2()[2]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group2()[3]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group1()[0], self.group2()[1], self.group1()[1], self.group1()[1]]) * Simd32x4::from([other.group0()[0], other.group1()[0], other.group0()[3], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, -1.0]), g3: Simd32x4::from(self.group3()[0]) * other.group1() + self.group3() * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl OuterProduct<Motor> for MultiVector {
    type Output = MultiVector;

    fn outer_product(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + self.group0() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.group1()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group2()[1]) * swizzle!(other.group1(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group1(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group2()[3]) * swizzle!(other.group1(), 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + self.group1() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g2: Simd32x4::from(self.group2()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group2()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g3: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group1()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group3()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group1(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<Motor> for MultiVector {
    type Output = MultiVector;

    fn inner_product(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 1, 1, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]), g1: Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group2()[1]) * swizzle!(other.group1(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group1(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group2()[3]) * swizzle!(other.group1(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: Simd32x4::from(self.group2()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group0(), 3, 3, 0, 1) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group2()[3]) * swizzle!(other.group0(), 2, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]) + swizzle!(self.group2(), 0, 1, 1, 1) * swizzle!(other.group0(), 0, 0, 3, 2) * Simd32x4::from([0.0, 1.0, 1.0, -1.0]), g3: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group3()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group3()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + self.group0() * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) } }
    }
}

impl RightContraction<Motor> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: Simd32x4::from(self.group2()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + self.group2() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g3: Simd32x4::from(self.group3()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + self.group3() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl ScalarProduct<Motor> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: Motor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3] } }
    }
}

impl Into<PointAndPlane> for MultiVector {
    fn into(self) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from([self.group2()[0], self.group1()[1], self.group1()[2], self.group1()[3]]), g1: Simd32x4::from([self.group1()[0], self.group2()[1], self.group2()[2], self.group2()[3]]) } }
    }
}

impl Add<PointAndPlane> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: PointAndPlane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0(), g1: self.group1() + Simd32x4::from([other.group1()[0], other.group0()[1], other.group0()[2], other.group0()[3]]), g2: self.group2() + Simd32x4::from([other.group0()[0], other.group1()[1], other.group1()[2], other.group1()[3]]), g3: self.group3() } }
    }
}

impl AddAssign<PointAndPlane> for MultiVector {
    fn add_assign(&mut self, other: PointAndPlane) {
        *self = (*self).add(other);
    }
}

impl Sub<PointAndPlane> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: PointAndPlane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0(), g1: self.group1() - Simd32x4::from([other.group1()[0], other.group0()[1], other.group0()[2], other.group0()[3]]), g2: self.group2() - Simd32x4::from([other.group0()[0], other.group1()[1], other.group1()[2], other.group1()[3]]), g3: self.group3() } }
    }
}

impl SubAssign<PointAndPlane> for MultiVector {
    fn sub_assign(&mut self, other: PointAndPlane) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<PointAndPlane> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: PointAndPlane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group0()[0], other.group1()[1], other.group1()[2], other.group1()[3]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group1()[1], other.group0()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([other.group1()[2], other.group1()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group1()[3], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group0()[1], other.group0()[2], other.group0()[3]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[1], other.group1()[0], other.group0()[3], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[2], other.group0()[3], other.group1()[0], other.group0()[1]]) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[3], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) - Simd32x4::from(self.group3()[0]) * Simd32x4::from([other.group0()[0], other.group1()[1], other.group1()[2], other.group1()[3]]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([other.group1()[1], other.group0()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([other.group1()[2], other.group1()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group3()[3]) * Simd32x4::from([other.group1()[3], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]), g2: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group1()[1], other.group1()[2], other.group1()[3]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[1], other.group0()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[2], other.group1()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[3], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]), g3: Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[0], other.group1()[1], other.group1()[2], other.group1()[3]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[1], other.group0()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[2], other.group1()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group1()[3], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) - Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group1()[0], other.group0()[1], other.group0()[2], other.group0()[3]]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group0()[1], other.group1()[0], other.group0()[3], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([other.group0()[2], other.group0()[3], other.group1()[0], other.group0()[1]]) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group0()[3], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) } }
    }
}

impl ScalarProduct<PointAndPlane> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: PointAndPlane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group2()[0] * other.group0()[0] + self.group2()[1] * other.group1()[1] + self.group2()[2] * other.group1()[2] + self.group2()[3] * other.group1()[3] } }
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
        Rotor { groups: RotorGroups { g0: Simd32x4::from(0.0) } }
    }
}

impl One for Rotor {
    fn one() -> Self {
        Rotor { groups: RotorGroups { g0: Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl Neg for Rotor {
    type Output = Rotor;

    fn neg(self) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
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
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) } }
    }
}

impl Conjugation for Rotor {
    type Output = Rotor;

    fn conjugation(self) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) } }
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
        Rotor { groups: RotorGroups { g0: self.group0() + Simd32x4::from(other.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
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
        Rotor { groups: RotorGroups { g0: self.group0() - Simd32x4::from(other.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
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
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl OuterProduct<Scalar> for Rotor {
    type Output = Rotor;

    fn outer_product(self, other: Scalar) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for Rotor {
    type Output = Rotor;

    fn inner_product(self, other: Scalar) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
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
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
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
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + other.group0(), g1: other.group1(), g2: other.group2(), g3: other.group3() } }
    }
}

impl Sub<MultiVector> for Rotor {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - other.group0(), g1: Simd32x4::from(0.0) - other.group1(), g2: Simd32x4::from(0.0) - other.group2(), g3: Simd32x4::from(0.0) - other.group3() } }
    }
}

impl GeometricProduct<MultiVector> for Rotor {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group1(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]), g2: Simd32x4::from(self.group0()[0]) * other.group2() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group2(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group2(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group2(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]), g3: Simd32x4::from(self.group0()[0]) * other.group3() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group3(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group3(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group3(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) } }
    }
}

impl OuterProduct<MultiVector> for Rotor {
    type Output = MultiVector;

    fn outer_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + self.group0() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + self.group0() * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]), g2: Simd32x4::from(self.group0()[0]) * other.group2() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group2()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group2()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group2(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g3: Simd32x4::from(self.group0()[0]) * other.group3() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group3()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group3()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group3(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<MultiVector> for Rotor {
    type Output = MultiVector;

    fn inner_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 1, 1, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group1()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group1(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: Simd32x4::from(self.group0()[0]) * other.group2() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group2(), 3, 3, 0, 1) * Simd32x4::from([0.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group2(), 2, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) + swizzle!(self.group0(), 0, 1, 1, 1) * swizzle!(other.group2(), 0, 0, 3, 2) * Simd32x4::from([0.0, -1.0, 1.0, -1.0]), g3: Simd32x4::from(self.group0()[0]) * other.group3() + self.group0() * Simd32x4::from(other.group3()[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) } }
    }
}

impl LeftContraction<MultiVector> for Rotor {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group1()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group1(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: Simd32x4::from(self.group0()[0]) * other.group2() + self.group0() * Simd32x4::from(other.group2()[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]), g3: Simd32x4::from(self.group0()[0]) * other.group3() + self.group0() * Simd32x4::from(other.group3()[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) } }
    }
}

impl ScalarProduct<MultiVector> for Rotor {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3] } }
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
        Rotor { groups: RotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) } }
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
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl OuterProduct<Rotor> for Rotor {
    type Output = Rotor;

    fn outer_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + self.group0() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl InnerProduct<Rotor> for Rotor {
    type Output = Rotor;

    fn inner_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 1, 1, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl LeftContraction<Rotor> for Rotor {
    type Output = Rotor;

    fn left_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl RightContraction<Rotor> for Rotor {
    type Output = Rotor;

    fn right_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl ScalarProduct<Rotor> for Rotor {
    type Output = Scalar;

    fn scalar_product(self, other: Rotor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3] } }
    }
}

impl GeometricProduct<Point> for Rotor {
    type Output = PointAndPlane;

    fn geometric_product(self, other: Point) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + swizzle!(self.group0(), 0, 0, 1, 1) * swizzle!(other.group0(), 0, 0, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]), g1: Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 1, 1, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) } }
    }
}

impl OuterProduct<Point> for Rotor {
    type Output = Point;

    fn outer_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl InnerProduct<Point> for Rotor {
    type Output = PointAndPlane;

    fn inner_product(self, other: Point) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 1, 1, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) } }
    }
}

impl LeftContraction<Point> for Rotor {
    type Output = PointAndPlane;

    fn left_contraction(self, other: Point) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 1, 1, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) } }
    }
}

impl RegressiveProduct<IdealPoint> for Rotor {
    type Output = Scalar;

    fn regressive_product(self, other: IdealPoint) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group0()[0] + self.group0()[2] * other.group0()[1] + self.group0()[3] * other.group0()[2] } }
    }
}

impl InnerProduct<IdealPoint> for Rotor {
    type Output = IdealPoint;

    fn inner_product(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl LeftContraction<IdealPoint> for Rotor {
    type Output = IdealPoint;

    fn left_contraction(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl GeometricProduct<Plane> for Rotor {
    type Output = PointAndPlane;

    fn geometric_product(self, other: Plane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 1, 1, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + swizzle!(self.group0(), 0, 0, 1, 1) * swizzle!(other.group0(), 0, 0, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<Plane> for Rotor {
    type Output = PointAndPlane;

    fn outer_product(self, other: Plane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 1, 1, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl InnerProduct<Plane> for Rotor {
    type Output = Plane;

    fn inner_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + swizzle!(self.group0(), 0, 0, 1, 1) * swizzle!(other.group0(), 0, 0, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) } }
    }
}

impl LeftContraction<Plane> for Rotor {
    type Output = Plane;

    fn left_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl GeometricProduct<Line> for Rotor {
    type Output = Motor;

    fn geometric_product(self, other: Line) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[2], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[2], other.group1()[1], other.group1()[0]]) * Simd32x4::from([-1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[2], other.group1()[1], other.group1()[0], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[2], other.group0()[1], other.group0()[0], other.group0()[2]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Line> for Rotor {
    type Output = Scalar;

    fn regressive_product(self, other: Line) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group0()[0] + self.group0()[2] * other.group0()[1] + self.group0()[3] * other.group0()[2] } }
    }
}

impl RightContraction<Line> for Rotor {
    type Output = Scalar;

    fn right_contraction(self, other: Line) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[1] * other.group1()[0] - self.group0()[2] * other.group1()[1] - self.group0()[3] * other.group1()[2] } }
    }
}

impl ScalarProduct<Line> for Rotor {
    type Output = Scalar;

    fn scalar_product(self, other: Line) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[1] * other.group1()[0] - self.group0()[2] * other.group1()[1] - self.group0()[3] * other.group1()[2] } }
    }
}

impl GeometricProduct<Translator> for Rotor {
    type Output = Motor;

    fn geometric_product(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 1, 3, 2) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 3, 2, 1) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 2, 1, 3) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * other.group0() * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Translator> for Rotor {
    type Output = Scalar;

    fn regressive_product(self, other: Translator) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group0()[3] } }
    }
}

impl OuterProduct<Translator> for Rotor {
    type Output = Motor;

    fn outer_product(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 1, 2, 3) } }
    }
}

impl LeftContraction<Translator> for Rotor {
    type Output = Translator;

    fn left_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl RightContraction<Translator> for Rotor {
    type Output = Rotor;

    fn right_contraction(self, other: Translator) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) } }
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
        Motor { groups: MotorGroups { g0: self.group0() + other.group0(), g1: other.group1() } }
    }
}

impl Sub<Motor> for Rotor {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() - other.group0(), g1: Simd32x4::from(0.0) - other.group1() } }
    }
}

impl GeometricProduct<Motor> for Rotor {
    type Output = Motor;

    fn geometric_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group1(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) } }
    }
}

impl RegressiveProduct<Motor> for Rotor {
    type Output = Rotor;

    fn regressive_product(self, other: Motor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group1(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl OuterProduct<Motor> for Rotor {
    type Output = Motor;

    fn outer_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + self.group0() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group1()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group1(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<Motor> for Rotor {
    type Output = Motor;

    fn inner_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 1, 1, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + self.group0() * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) } }
    }
}

impl LeftContraction<Motor> for Rotor {
    type Output = Motor;

    fn left_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + self.group0() * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) } }
    }
}

impl RightContraction<Motor> for Rotor {
    type Output = Rotor;

    fn right_contraction(self, other: Motor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl ScalarProduct<Motor> for Rotor {
    type Output = Scalar;

    fn scalar_product(self, other: Motor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3] } }
    }
}

impl GeometricProduct<PointAndPlane> for Rotor {
    type Output = PointAndPlane;

    fn geometric_product(self, other: PointAndPlane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group0()[3], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[2], other.group0()[3], other.group1()[0], other.group0()[1]]) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[3], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[2], other.group1()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[3], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) } }
    }
}

impl OuterProduct<PointAndPlane> for Rotor {
    type Output = PointAndPlane;

    fn outer_product(self, other: PointAndPlane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group1(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 1, 1, 0, 0) * swizzle!(other.group1(), 1, 0, 0, 0) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() } }
    }
}

impl InnerProduct<PointAndPlane> for Rotor {
    type Output = PointAndPlane;

    fn inner_product(self, other: PointAndPlane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[2], other.group1()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[3], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) } }
    }
}

impl LeftContraction<PointAndPlane> for Rotor {
    type Output = PointAndPlane;

    fn left_contraction(self, other: PointAndPlane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 1, 1, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) } }
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
        Point { groups: PointGroups { g0: Simd32x4::from(0.0) } }
    }
}

impl One for Point {
    fn one() -> Self {
        Point { groups: PointGroups { g0: Simd32x4::from(0.0) } }
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
    }
}

impl Automorphism for Point {
    type Output = Point;

    fn automorphism(self) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
    }
}

impl Reversal for Point {
    type Output = Point;

    fn reversal(self) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
    }
}

impl Conjugation for Point {
    type Output = Point;

    fn conjugation(self) -> Point {
        Point { groups: PointGroups { g0: self.group0() } }
    }
}

impl Dual for Point {
    type Output = Plane;

    fn dual(self) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
    }
}

impl GeometricProduct<Scalar> for Point {
    type Output = Point;

    fn geometric_product(self, other: Scalar) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl OuterProduct<Scalar> for Point {
    type Output = Point;

    fn outer_product(self, other: Scalar) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for Point {
    type Output = Point;

    fn inner_product(self, other: Scalar) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl RightContraction<Scalar> for Point {
    type Output = Point;

    fn right_contraction(self, other: Scalar) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl Add<MultiVector> for Point {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: other.group0(), g1: self.group0() * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + other.group1(), g2: Simd32x4::from(self.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + other.group2(), g3: other.group3() } }
    }
}

impl Sub<MultiVector> for Point {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(0.0) - other.group0(), g1: self.group0() * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) - other.group1(), g2: Simd32x4::from(self.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) - other.group2(), g3: Simd32x4::from(0.0) - other.group3() } }
    }
}

impl GeometricProduct<MultiVector> for Point {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group2() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.group0()[0]) * other.group3() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]), g2: Simd32x4::from(self.group0()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]), g3: Simd32x4::from(0.0) - Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group2(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group2(), 2, 3, 0, 1) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group2(), 3, 2, 1, 0) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) } }
    }
}

impl ScalarProduct<MultiVector> for Point {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group2()[0] } }
    }
}

impl GeometricProduct<Rotor> for Point {
    type Output = PointAndPlane;

    fn geometric_product(self, other: Rotor) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 0, 1) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 2, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]) + swizzle!(self.group0(), 0, 1, 1, 1) * swizzle!(other.group0(), 0, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]), g1: Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 1, 2, 3) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) } }
    }
}

impl OuterProduct<Rotor> for Point {
    type Output = Point;

    fn outer_product(self, other: Rotor) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl InnerProduct<Rotor> for Point {
    type Output = PointAndPlane;

    fn inner_product(self, other: Rotor) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 1, 2, 3) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) } }
    }
}

impl RightContraction<Rotor> for Point {
    type Output = PointAndPlane;

    fn right_contraction(self, other: Rotor) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 1, 2, 3) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) } }
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
        Point { groups: PointGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) } }
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
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(0.0) - Simd32x4::from(self.group0()[0]) * other.group0() + self.group0() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Point> for Point {
    type Output = Line;

    fn regressive_product(self, other: Point) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[3], other.group0()[3], other.group0()[1]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[2], other.group0()[1], other.group0()[2]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[1]]) * Simd32x3::from([other.group0()[0], other.group0()[3], other.group0()[2]]) * Simd32x3::from([0.0, -1.0, 1.0]), g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[3]]) + Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from(-1.0) } }
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

impl RegressiveProduct<IdealPoint> for Point {
    type Output = Plane;

    fn regressive_product(self, other: IdealPoint) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl Add<Plane> for Point {
    type Output = PointAndPlane;

    fn add(self, other: Plane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group0(), g1: other.group0() } }
    }
}

impl Sub<Plane> for Point {
    type Output = PointAndPlane;

    fn sub(self, other: Plane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group0(), g1: Simd32x4::from(0.0) - other.group0() } }
    }
}

impl RegressiveProduct<Plane> for Point {
    type Output = Scalar;

    fn regressive_product(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3] } }
    }
}

impl InnerProduct<Plane> for Point {
    type Output = Line;

    fn inner_product(self, other: Plane) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[3], other.group0()[3], other.group0()[1]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[2], other.group0()[1], other.group0()[2]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[1]]) * Simd32x3::from([other.group0()[0], other.group0()[3], other.group0()[2]]) * Simd32x3::from([0.0, -1.0, 1.0]), g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[3]]) } }
    }
}

impl RightContraction<Plane> for Point {
    type Output = Line;

    fn right_contraction(self, other: Plane) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[3], other.group0()[3], other.group0()[1]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[2], other.group0()[1], other.group0()[2]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[1]]) * Simd32x3::from([other.group0()[0], other.group0()[3], other.group0()[2]]) * Simd32x3::from([0.0, -1.0, 1.0]), g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[3]]) } }
    }
}

impl RegressiveProduct<Line> for Point {
    type Output = Plane;

    fn regressive_product(self, other: Line) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group1()[2], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group1()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([-1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[2], other.group1()[1], other.group1()[0], other.group0()[2]]) * Simd32x4::from([-1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl InnerProduct<Line> for Point {
    type Output = Plane;

    fn inner_product(self, other: Line) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) } }
    }
}

impl RightContraction<Line> for Point {
    type Output = Plane;

    fn right_contraction(self, other: Line) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) } }
    }
}

impl GeometricProduct<Translator> for Point {
    type Output = Point;

    fn geometric_product(self, other: Translator) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + self.group0() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Translator> for Point {
    type Output = Plane;

    fn regressive_product(self, other: Translator) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 1, 2, 3) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl OuterProduct<Translator> for Point {
    type Output = Point;

    fn outer_product(self, other: Translator) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl InnerProduct<Translator> for Point {
    type Output = Point;

    fn inner_product(self, other: Translator) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl RightContraction<Translator> for Point {
    type Output = Point;

    fn right_contraction(self, other: Translator) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl GeometricProduct<Motor> for Point {
    type Output = PointAndPlane;

    fn geometric_product(self, other: Motor) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group1()[1], other.group1()[2], other.group1()[3]]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 0, 1) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 2, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]) + swizzle!(self.group0(), 0, 1, 1, 1) * swizzle!(other.group0(), 0, 0, 3, 2) * Simd32x4::from([0.0, 1.0, 1.0, -1.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group0()[1], other.group0()[2], other.group0()[3]]) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl RegressiveProduct<Motor> for Point {
    type Output = PointAndPlane;

    fn regressive_product(self, other: Motor) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group0() * Simd32x4::from(other.group1()[0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[1], other.group1()[1], other.group0()[3], other.group0()[2]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[2], other.group0()[3], other.group1()[2], other.group0()[1]]) * Simd32x4::from([-1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[3], other.group0()[2], other.group0()[1], other.group1()[3]]) * Simd32x4::from([-1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * other.group1() * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl OuterProduct<Motor> for Point {
    type Output = Point;

    fn outer_product(self, other: Motor) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl InnerProduct<Motor> for Point {
    type Output = PointAndPlane;

    fn inner_product(self, other: Motor) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group0()[1], other.group0()[2], other.group0()[3]]) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl RightContraction<Motor> for Point {
    type Output = PointAndPlane;

    fn right_contraction(self, other: Motor) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 1, 2, 3) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) } }
    }
}

impl Add<PointAndPlane> for Point {
    type Output = PointAndPlane;

    fn add(self, other: PointAndPlane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group0() + other.group0(), g1: other.group1() } }
    }
}

impl Sub<PointAndPlane> for Point {
    type Output = PointAndPlane;

    fn sub(self, other: PointAndPlane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group0() - other.group0(), g1: Simd32x4::from(0.0) - other.group1() } }
    }
}

impl GeometricProduct<PointAndPlane> for Point {
    type Output = Motor;

    fn geometric_product(self, other: PointAndPlane) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group1()[1], other.group1()[2], other.group1()[3]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(0.0) - Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group0()[1], other.group0()[2], other.group0()[3]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[1], other.group0()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[2], other.group1()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[3], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) } }
    }
}

impl LeftContraction<PointAndPlane> for Point {
    type Output = Scalar;

    fn left_contraction(self, other: PointAndPlane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] } }
    }
}

impl ScalarProduct<PointAndPlane> for Point {
    type Output = Scalar;

    fn scalar_product(self, other: PointAndPlane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] } }
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
        IdealPoint { groups: IdealPointGroups { g0: Simd32x3::from(0.0) } }
    }
}

impl One for IdealPoint {
    fn one() -> Self {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x3::from(0.0) } }
    }
}

impl Neg for IdealPoint {
    type Output = IdealPoint;

    fn neg(self) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x3::from(-1.0) } }
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
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x3::from(-1.0) } }
    }
}

impl Conjugation for IdealPoint {
    type Output = IdealPoint;

    fn conjugation(self) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x3::from(-1.0) } }
    }
}

impl Add<Scalar> for IdealPoint {
    type Output = Translator;

    fn add(self, other: Scalar) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + Simd32x4::from(other.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl Sub<Scalar> for IdealPoint {
    type Output = Translator;

    fn sub(self, other: Scalar) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) - Simd32x4::from(other.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl GeometricProduct<Scalar> for IdealPoint {
    type Output = IdealPoint;

    fn geometric_product(self, other: Scalar) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl OuterProduct<Scalar> for IdealPoint {
    type Output = IdealPoint;

    fn outer_product(self, other: Scalar) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for IdealPoint {
    type Output = IdealPoint;

    fn inner_product(self, other: Scalar) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl RightContraction<Scalar> for IdealPoint {
    type Output = IdealPoint;

    fn right_contraction(self, other: Scalar) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl Add<MultiVector> for IdealPoint {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: other.group0(), g1: other.group1(), g2: other.group2(), g3: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + other.group3() } }
    }
}

impl Sub<MultiVector> for IdealPoint {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(0.0) - other.group0(), g1: Simd32x4::from(0.0) - other.group1(), g2: Simd32x4::from(0.0) - other.group2(), g3: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) - other.group3() } }
    }
}

impl RegressiveProduct<Rotor> for IdealPoint {
    type Output = Scalar;

    fn regressive_product(self, other: Rotor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[1] + self.group0()[1] * other.group0()[2] + self.group0()[2] * other.group0()[3] } }
    }
}

impl InnerProduct<Rotor> for IdealPoint {
    type Output = IdealPoint;

    fn inner_product(self, other: Rotor) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl RightContraction<Rotor> for IdealPoint {
    type Output = IdealPoint;

    fn right_contraction(self, other: Rotor) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl RegressiveProduct<Point> for IdealPoint {
    type Output = Plane;

    fn regressive_product(self, other: Point) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) } }
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
        IdealPoint { groups: IdealPointGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) / Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) } }
    }
}

impl DivAssign<IdealPoint> for IdealPoint {
    fn div_assign(&mut self, other: IdealPoint) {
        *self = (*self).div(other);
    }
}

impl Add<Line> for IdealPoint {
    type Output = Line;

    fn add(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: self.group0() + other.group0(), g1: other.group1() } }
    }
}

impl Sub<Line> for IdealPoint {
    type Output = Line;

    fn sub(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: self.group0() - other.group0(), g1: Simd32x3::from(0.0) - other.group1() } }
    }
}

impl RegressiveProduct<Line> for IdealPoint {
    type Output = Scalar;

    fn regressive_product(self, other: Line) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2] } }
    }
}

impl Add<Translator> for IdealPoint {
    type Output = Translator;

    fn add(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + other.group0() } }
    }
}

impl Sub<Translator> for IdealPoint {
    type Output = Translator;

    fn sub(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) - other.group0() } }
    }
}

impl GeometricProduct<Translator> for IdealPoint {
    type Output = IdealPoint;

    fn geometric_product(self, other: Translator) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl OuterProduct<Translator> for IdealPoint {
    type Output = IdealPoint;

    fn outer_product(self, other: Translator) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl InnerProduct<Translator> for IdealPoint {
    type Output = IdealPoint;

    fn inner_product(self, other: Translator) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl RightContraction<Translator> for IdealPoint {
    type Output = IdealPoint;

    fn right_contraction(self, other: Translator) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl Add<Motor> for IdealPoint {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: other.group0(), g1: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + other.group1() } }
    }
}

impl Sub<Motor> for IdealPoint {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(0.0) - other.group0(), g1: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) - other.group1() } }
    }
}

impl RegressiveProduct<Motor> for IdealPoint {
    type Output = Translator;

    fn regressive_product(self, other: Motor) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group1()[0], other.group0()[2]]) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group1()[0]]) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[1], other.group1()[0], other.group0()[0], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<Motor> for IdealPoint {
    type Output = IdealPoint;

    fn inner_product(self, other: Motor) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl RightContraction<Motor> for IdealPoint {
    type Output = IdealPoint;

    fn right_contraction(self, other: Motor) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl RegressiveProduct<PointAndPlane> for IdealPoint {
    type Output = Plane;

    fn regressive_product(self, other: PointAndPlane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) } }
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
        Plane { groups: PlaneGroups { g0: Simd32x4::from(0.0) } }
    }
}

impl One for Plane {
    fn one() -> Self {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(0.0) } }
    }
}

impl Neg for Plane {
    type Output = Plane;

    fn neg(self) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
    }
}

impl Automorphism for Plane {
    type Output = Plane;

    fn automorphism(self) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
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
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
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
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl OuterProduct<Scalar> for Plane {
    type Output = Plane;

    fn outer_product(self, other: Scalar) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for Plane {
    type Output = Plane;

    fn inner_product(self, other: Scalar) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl RightContraction<Scalar> for Plane {
    type Output = Plane;

    fn right_contraction(self, other: Scalar) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl Add<MultiVector> for Plane {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: other.group0(), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + other.group1(), g2: self.group0() * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + other.group2(), g3: other.group3() } }
    }
}

impl Sub<MultiVector> for Plane {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(0.0) - other.group0(), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) - other.group1(), g2: self.group0() * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) - other.group2(), g3: Simd32x4::from(0.0) - other.group3() } }
    }
}

impl GeometricProduct<MultiVector> for Plane {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group2(), 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group2(), 2, 3, 0, 1) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group2(), 3, 2, 1, 0) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]), g1: Simd32x4::from(self.group0()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * swizzle!(other.group3(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group3(), 2, 3, 0, 1) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group3(), 3, 2, 1, 0) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]), g2: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]), g3: Simd32x4::from(self.group0()[0]) * other.group2() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group1(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) } }
    }
}

impl ScalarProduct<MultiVector> for Plane {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group2()[1] + self.group0()[2] * other.group2()[2] + self.group0()[3] * other.group2()[3] } }
    }
}

impl GeometricProduct<Rotor> for Plane {
    type Output = PointAndPlane;

    fn geometric_product(self, other: Rotor) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 1, 2, 3) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]), g1: Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 0, 1) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 2, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]) + swizzle!(self.group0(), 0, 1, 1, 1) * swizzle!(other.group0(), 0, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<Rotor> for Plane {
    type Output = PointAndPlane;

    fn outer_product(self, other: Rotor) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 1, 2, 3) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]), g1: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl InnerProduct<Rotor> for Plane {
    type Output = Plane;

    fn inner_product(self, other: Rotor) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 0, 1) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 2, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]) + swizzle!(self.group0(), 0, 1, 1, 1) * swizzle!(other.group0(), 0, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightContraction<Rotor> for Plane {
    type Output = Plane;

    fn right_contraction(self, other: Rotor) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl Add<Point> for Plane {
    type Output = PointAndPlane;

    fn add(self, other: Point) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: other.group0(), g1: self.group0() } }
    }
}

impl Sub<Point> for Plane {
    type Output = PointAndPlane;

    fn sub(self, other: Point) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(0.0) - other.group0(), g1: self.group0() } }
    }
}

impl RegressiveProduct<Point> for Plane {
    type Output = Scalar;

    fn regressive_product(self, other: Point) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group0()[3] } }
    }
}

impl InnerProduct<Point> for Plane {
    type Output = Line;

    fn inner_product(self, other: Point) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[3], other.group0()[3], other.group0()[1]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[2], other.group0()[1], other.group0()[2]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[1]]) * Simd32x3::from([other.group0()[0], other.group0()[3], other.group0()[2]]) * Simd32x3::from([0.0, 1.0, -1.0]), g1: Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x3::from(other.group0()[0]) } }
    }
}

impl LeftContraction<Point> for Plane {
    type Output = Line;

    fn left_contraction(self, other: Point) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[3], other.group0()[3], other.group0()[1]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[2], other.group0()[1], other.group0()[2]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[1]]) * Simd32x3::from([other.group0()[0], other.group0()[3], other.group0()[2]]) * Simd32x3::from([0.0, 1.0, -1.0]), g1: Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x3::from(other.group0()[0]) } }
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
        Plane { groups: PlaneGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl DivAssign<Plane> for Plane {
    fn div_assign(&mut self, other: Plane) {
        *self = (*self).div(other);
    }
}

impl OuterProduct<Plane> for Plane {
    type Output = Line;

    fn outer_product(self, other: Plane) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[3]]) + Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from(-1.0), g1: Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[3], other.group0()[3], other.group0()[1]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[2], other.group0()[1], other.group0()[2]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[1]]) * Simd32x3::from([other.group0()[0], other.group0()[3], other.group0()[2]]) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl InnerProduct<Plane> for Plane {
    type Output = Scalar;

    fn inner_product(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group0()[3] } }
    }
}

impl LeftContraction<Plane> for Plane {
    type Output = Scalar;

    fn left_contraction(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group0()[3] } }
    }
}

impl RightContraction<Plane> for Plane {
    type Output = Scalar;

    fn right_contraction(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group0()[3] } }
    }
}

impl ScalarProduct<Plane> for Plane {
    type Output = Scalar;

    fn scalar_product(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group0()[3] } }
    }
}

impl GeometricProduct<Line> for Plane {
    type Output = PointAndPlane;

    fn geometric_product(self, other: Line) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group0()[2], other.group0()[1]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group0()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[2], other.group0()[1], other.group0()[0], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]), g1: Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group1()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([-1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[2], other.group1()[1], other.group1()[0], other.group0()[2]]) * Simd32x4::from([-1.0, 1.0, -1.0, 0.0]) + swizzle!(self.group0(), 1, 0, 1, 1) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group1()[2], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<Line> for Plane {
    type Output = Point;

    fn outer_product(self, other: Line) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group0()[2], other.group0()[1]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group0()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[2], other.group0()[1], other.group0()[0], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) } }
    }
}

impl InnerProduct<Line> for Plane {
    type Output = Plane;

    fn inner_product(self, other: Line) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group1()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([-1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[2], other.group1()[1], other.group1()[0], other.group0()[2]]) * Simd32x4::from([-1.0, 1.0, -1.0, 0.0]) + swizzle!(self.group0(), 1, 0, 1, 1) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group1()[2], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) } }
    }
}

impl LeftContraction<Line> for Plane {
    type Output = Plane;

    fn left_contraction(self, other: Line) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group1()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([-1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[2], other.group1()[1], other.group1()[0], other.group0()[2]]) * Simd32x4::from([-1.0, 1.0, -1.0, 0.0]) + swizzle!(self.group0(), 1, 0, 1, 1) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group1()[2], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) } }
    }
}

impl InnerProduct<Translator> for Plane {
    type Output = Plane;

    fn inner_product(self, other: Translator) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl RightContraction<Translator> for Plane {
    type Output = Plane;

    fn right_contraction(self, other: Translator) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl GeometricProduct<Motor> for Plane {
    type Output = PointAndPlane;

    fn geometric_product(self, other: Motor) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[1], other.group1()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[2], other.group1()[3], other.group1()[0], other.group1()[1]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[3], other.group1()[2], other.group1()[1], other.group1()[0]]) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[0]) * other.group0() * Simd32x4::from([0.0, -1.0, -1.0, -1.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[1], other.group0()[0], other.group0()[3], other.group0()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[2], other.group0()[3], other.group0()[0], other.group0()[1]]) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[3], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl RegressiveProduct<Motor> for Plane {
    type Output = Plane;

    fn regressive_product(self, other: Motor) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group1()[0]) } }
    }
}

impl OuterProduct<Motor> for Plane {
    type Output = PointAndPlane;

    fn outer_product(self, other: Motor) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group1()[3], other.group1()[2]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[2], other.group1()[3], other.group0()[2], other.group1()[1]]) * Simd32x4::from([1.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[3], other.group1()[2], other.group1()[1], other.group0()[3]]) * Simd32x4::from([1.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * other.group0() * Simd32x4::from([0.0, -1.0, -1.0, -1.0]), g1: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl RightContraction<Motor> for Plane {
    type Output = Plane;

    fn right_contraction(self, other: Motor) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl Add<PointAndPlane> for Plane {
    type Output = PointAndPlane;

    fn add(self, other: PointAndPlane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: other.group0(), g1: self.group0() + other.group1() } }
    }
}

impl Sub<PointAndPlane> for Plane {
    type Output = PointAndPlane;

    fn sub(self, other: PointAndPlane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(0.0) - other.group0(), g1: self.group0() - other.group1() } }
    }
}

impl GeometricProduct<PointAndPlane> for Plane {
    type Output = Motor;

    fn geometric_product(self, other: PointAndPlane) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[1], other.group0()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[2], other.group1()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[3], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group1()[1], other.group1()[2], other.group1()[3]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[1], other.group1()[0], other.group0()[3], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[2], other.group0()[3], other.group1()[0], other.group0()[1]]) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[3], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) } }
    }
}

impl RegressiveProduct<PointAndPlane> for Plane {
    type Output = Scalar;

    fn regressive_product(self, other: PointAndPlane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group0()[3] } }
    }
}

impl RightContraction<PointAndPlane> for Plane {
    type Output = Scalar;

    fn right_contraction(self, other: PointAndPlane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2] + self.group0()[3] * other.group1()[3] } }
    }
}

impl ScalarProduct<PointAndPlane> for Plane {
    type Output = Scalar;

    fn scalar_product(self, other: PointAndPlane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2] + self.group0()[3] * other.group1()[3] } }
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

impl Zero for Line {
    fn zero() -> Self {
        Line { groups: LineGroups { g0: Simd32x3::from(0.0), g1: Simd32x3::from(0.0) } }
    }
}

impl One for Line {
    fn one() -> Self {
        Line { groups: LineGroups { g0: Simd32x3::from(0.0), g1: Simd32x3::from(0.0) } }
    }
}

impl Neg for Line {
    type Output = Line;

    fn neg(self) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(-1.0), g1: self.group1() * Simd32x3::from(-1.0) } }
    }
}

impl Automorphism for Line {
    type Output = Line;

    fn automorphism(self) -> Line {
        Line { groups: LineGroups { g0: self.group0(), g1: self.group1() } }
    }
}

impl Reversal for Line {
    type Output = Line;

    fn reversal(self) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(-1.0), g1: self.group1() * Simd32x3::from(-1.0) } }
    }
}

impl Conjugation for Line {
    type Output = Line;

    fn conjugation(self) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(-1.0), g1: self.group1() * Simd32x3::from(-1.0) } }
    }
}

impl Dual for Line {
    type Output = Line;

    fn dual(self) -> Line {
        Line { groups: LineGroups { g0: self.group1(), g1: self.group0() } }
    }
}

impl GeometricProduct<Scalar> for Line {
    type Output = Line;

    fn geometric_product(self, other: Scalar) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl OuterProduct<Scalar> for Line {
    type Output = Line;

    fn outer_product(self, other: Scalar) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for Line {
    type Output = Line;

    fn inner_product(self, other: Scalar) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl RightContraction<Scalar> for Line {
    type Output = Line;

    fn right_contraction(self, other: Scalar) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl Add<MultiVector> for Line {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from([self.group0()[0], self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + other.group0(), g1: other.group1(), g2: other.group2(), g3: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + other.group3() } }
    }
}

impl Sub<MultiVector> for Line {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from([self.group0()[0], self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) - other.group0(), g1: Simd32x4::from(0.0) - other.group1(), g2: Simd32x4::from(0.0) - other.group2(), g3: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) - other.group3() } }
    }
}

impl GeometricProduct<MultiVector> for Line {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]), g1: Simd32x4::from(self.group0()[0]) * swizzle!(other.group2(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * swizzle!(other.group2(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group2(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group1(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group1(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group1(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]), g2: Simd32x4::from(self.group1()[0]) * swizzle!(other.group2(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group2(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group2(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]), g3: Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group3(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group3(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group3(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) } }
    }
}

impl ScalarProduct<MultiVector> for Line {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group0()[1] - self.group1()[1] * other.group0()[2] - self.group1()[2] * other.group0()[3] } }
    }
}

impl GeometricProduct<Rotor> for Line {
    type Output = Motor;

    fn geometric_product(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]), g1: Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Rotor> for Line {
    type Output = Scalar;

    fn regressive_product(self, other: Rotor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[1] + self.group0()[1] * other.group0()[2] + self.group0()[2] * other.group0()[3] } }
    }
}

impl LeftContraction<Rotor> for Line {
    type Output = Scalar;

    fn left_contraction(self, other: Rotor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group0()[1] - self.group1()[1] * other.group0()[2] - self.group1()[2] * other.group0()[3] } }
    }
}

impl ScalarProduct<Rotor> for Line {
    type Output = Scalar;

    fn scalar_product(self, other: Rotor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group0()[1] - self.group1()[1] * other.group0()[2] - self.group1()[2] * other.group0()[3] } }
    }
}

impl RegressiveProduct<Point> for Line {
    type Output = Plane;

    fn regressive_product(self, other: Point) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group0()[0], self.group1()[0], self.group1()[0]]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl InnerProduct<Point> for Line {
    type Output = Plane;

    fn inner_product(self, other: Point) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) } }
    }
}

impl LeftContraction<Point> for Line {
    type Output = Plane;

    fn left_contraction(self, other: Point) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) } }
    }
}

impl Into<IdealPoint> for Line {
    fn into(self) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: self.group0() } }
    }
}

impl Add<IdealPoint> for Line {
    type Output = Line;

    fn add(self, other: IdealPoint) -> Line {
        Line { groups: LineGroups { g0: self.group0() + other.group0(), g1: self.group1() } }
    }
}

impl AddAssign<IdealPoint> for Line {
    fn add_assign(&mut self, other: IdealPoint) {
        *self = (*self).add(other);
    }
}

impl Sub<IdealPoint> for Line {
    type Output = Line;

    fn sub(self, other: IdealPoint) -> Line {
        Line { groups: LineGroups { g0: self.group0() - other.group0(), g1: self.group1() } }
    }
}

impl SubAssign<IdealPoint> for Line {
    fn sub_assign(&mut self, other: IdealPoint) {
        *self = (*self).sub(other);
    }
}

impl RegressiveProduct<IdealPoint> for Line {
    type Output = Scalar;

    fn regressive_product(self, other: IdealPoint) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2] } }
    }
}

impl GeometricProduct<Plane> for Line {
    type Output = PointAndPlane;

    fn geometric_product(self, other: Plane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + Simd32x4::from([self.group1()[0], self.group1()[0], self.group0()[0], self.group0()[0]]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group0()[0], self.group1()[0], self.group1()[0]]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) } }
    }
}

impl OuterProduct<Plane> for Line {
    type Output = Point;

    fn outer_product(self, other: Plane) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + Simd32x4::from([self.group1()[0], self.group1()[0], self.group0()[0], self.group0()[0]]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl InnerProduct<Plane> for Line {
    type Output = Plane;

    fn inner_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group0()[0], self.group1()[0], self.group1()[0]]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) } }
    }
}

impl RightContraction<Plane> for Line {
    type Output = Plane;

    fn right_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group0()[0], self.group1()[0], self.group1()[0]]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) } }
    }
}

impl Add<Line> for Line {
    type Output = Line;

    fn add(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: self.group0() + other.group0(), g1: self.group1() + other.group1() } }
    }
}

impl AddAssign<Line> for Line {
    fn add_assign(&mut self, other: Line) {
        *self = (*self).add(other);
    }
}

impl Sub<Line> for Line {
    type Output = Line;

    fn sub(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: self.group0() - other.group0(), g1: self.group1() - other.group1() } }
    }
}

impl SubAssign<Line> for Line {
    fn sub_assign(&mut self, other: Line) {
        *self = (*self).sub(other);
    }
}

impl Mul<Line> for Line {
    type Output = Line;

    fn mul(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: self.group0() * other.group0(), g1: self.group1() * other.group1() } }
    }
}

impl MulAssign<Line> for Line {
    fn mul_assign(&mut self, other: Line) {
        *self = (*self).mul(other);
    }
}

impl Div<Line> for Line {
    type Output = Line;

    fn div(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) / Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]), g1: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) / Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) } }
    }
}

impl DivAssign<Line> for Line {
    fn div_assign(&mut self, other: Line) {
        *self = (*self).div(other);
    }
}

impl GeometricProduct<Line> for Line {
    type Output = Motor;

    fn geometric_product(self, other: Line) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[1], other.group1()[2], other.group1()[1], other.group1()[0]]) * Simd32x4::from([-1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[2], other.group1()[1], other.group1()[0], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[2], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[1], other.group1()[2], other.group1()[1], other.group1()[0]]) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[2], other.group1()[1], other.group1()[0], other.group1()[2]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[2], other.group0()[1], other.group0()[0], other.group0()[2]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[2], other.group1()[1]]) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) } }
    }
}

impl RegressiveProduct<Line> for Line {
    type Output = Scalar;

    fn regressive_product(self, other: Line) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2] + self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2] } }
    }
}

impl InnerProduct<Line> for Line {
    type Output = Scalar;

    fn inner_product(self, other: Line) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2] } }
    }
}

impl LeftContraction<Line> for Line {
    type Output = Scalar;

    fn left_contraction(self, other: Line) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2] } }
    }
}

impl RightContraction<Line> for Line {
    type Output = Scalar;

    fn right_contraction(self, other: Line) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2] } }
    }
}

impl ScalarProduct<Line> for Line {
    type Output = Scalar;

    fn scalar_product(self, other: Line) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2] } }
    }
}

impl RegressiveProduct<Translator> for Line {
    type Output = Scalar;

    fn regressive_product(self, other: Translator) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group1()[0] * other.group0()[1] + self.group1()[1] * other.group0()[2] + self.group1()[2] * other.group0()[3] } }
    }
}

impl InnerProduct<Translator> for Line {
    type Output = Line;

    fn inner_product(self, other: Translator) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]), g1: self.group1() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl RightContraction<Translator> for Line {
    type Output = Line;

    fn right_contraction(self, other: Translator) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]), g1: self.group1() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl Add<Motor> for Line {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + other.group0(), g1: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + other.group1() } }
    }
}

impl Sub<Motor> for Line {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) - other.group0(), g1: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) - other.group1() } }
    }
}

impl GeometricProduct<Motor> for Line {
    type Output = Motor;

    fn geometric_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]), g1: Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group1(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group1(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group1(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) } }
    }
}

impl LeftContraction<Motor> for Line {
    type Output = Translator;

    fn left_contraction(self, other: Motor) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group1()[0], other.group0()[2]]) * Simd32x4::from([-1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group1()[0]]) * Simd32x4::from([-1.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[1], other.group1()[0], other.group0()[0], other.group0()[0]]) * Simd32x4::from([-1.0, -1.0, 0.0, 0.0]) } }
    }
}

impl ScalarProduct<Motor> for Line {
    type Output = Scalar;

    fn scalar_product(self, other: Motor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group0()[1] - self.group1()[1] * other.group0()[2] - self.group1()[2] * other.group0()[3] } }
    }
}

impl GeometricProduct<PointAndPlane> for Line {
    type Output = PointAndPlane;

    fn geometric_product(self, other: PointAndPlane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[3], other.group1()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([0.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group0()[3], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], other.group1()[0], other.group0()[1]]) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[3], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([0.0, -1.0, 1.0, -1.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group1()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[3], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group1(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl RegressiveProduct<PointAndPlane> for Line {
    type Output = Plane;

    fn regressive_product(self, other: PointAndPlane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group0()[0], self.group1()[0], self.group1()[0]]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl OuterProduct<PointAndPlane> for Line {
    type Output = Point;

    fn outer_product(self, other: PointAndPlane) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group1(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group1(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + Simd32x4::from([self.group1()[0], self.group1()[0], self.group0()[0], self.group0()[0]]) * swizzle!(other.group1(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl InnerProduct<PointAndPlane> for Line {
    type Output = Plane;

    fn inner_product(self, other: PointAndPlane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group1()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[3], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group1(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl LeftContraction<PointAndPlane> for Line {
    type Output = Plane;

    fn left_contraction(self, other: PointAndPlane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) } }
    }
}

impl RightContraction<PointAndPlane> for Line {
    type Output = Plane;

    fn right_contraction(self, other: PointAndPlane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group1(), 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group1(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group0()[0], self.group1()[0], self.group1()[0]]) * swizzle!(other.group1(), 1, 0, 3, 2) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) } }
    }
}

impl SquaredMagnitude for Line {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for Line {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl Scale for Line {
    type Output = Line;

    fn scale(self, other: f32) -> Line {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Signum for Line {
    type Output = Line;

    fn signum(self) -> Line {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.magnitude().group0() } })
    }
}

impl Inverse for Line {
    type Output = Line;

    fn inverse(self) -> Line {
        self.reversal().geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.squared_magnitude().group0() } })
    }
}

impl Zero for Translator {
    fn zero() -> Self {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(0.0) } }
    }
}

impl One for Translator {
    fn one() -> Self {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl Neg for Translator {
    type Output = Translator;

    fn neg(self) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(-1.0) } }
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
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) } }
    }
}

impl Conjugation for Translator {
    type Output = Translator;

    fn conjugation(self) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) } }
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
        Translator { groups: TranslatorGroups { g0: self.group0() + Simd32x4::from(other.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
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
        Translator { groups: TranslatorGroups { g0: self.group0() - Simd32x4::from(other.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
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
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl OuterProduct<Scalar> for Translator {
    type Output = Translator;

    fn outer_product(self, other: Scalar) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for Translator {
    type Output = Translator;

    fn inner_product(self, other: Scalar) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
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
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
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
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + other.group0(), g1: other.group1(), g2: other.group2(), g3: self.group0() * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + other.group3() } }
    }
}

impl Sub<MultiVector> for Translator {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) - other.group0(), g1: Simd32x4::from(0.0) - other.group1(), g2: Simd32x4::from(0.0) - other.group2(), g3: self.group0() * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) - other.group3() } }
    }
}

impl GeometricProduct<MultiVector> for Translator {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group2(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group2(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group2(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]), g2: Simd32x4::from(self.group0()[0]) * other.group2(), g3: Simd32x4::from(self.group0()[0]) * other.group3() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl OuterProduct<MultiVector> for Translator {
    type Output = MultiVector;

    fn outer_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group2(), 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group2(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + swizzle!(self.group0(), 0, 0, 1, 1) * swizzle!(other.group2(), 0, 0, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]), g2: Simd32x4::from(self.group0()[0]) * other.group2(), g3: Simd32x4::from(self.group0()[0]) * other.group3() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 1, 1, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<MultiVector> for Translator {
    type Output = MultiVector;

    fn inner_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group2()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group2()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group2(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: Simd32x4::from(self.group0()[0]) * other.group2(), g3: Simd32x4::from(self.group0()[0]) * other.group3() + self.group0() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl LeftContraction<MultiVector> for Translator {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * other.group1(), g2: Simd32x4::from(self.group0()[0]) * other.group2(), g3: Simd32x4::from(self.group0()[0]) * other.group3() } }
    }
}

impl ScalarProduct<MultiVector> for Translator {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] } }
    }
}

impl GeometricProduct<Rotor> for Translator {
    type Output = Motor;

    fn geometric_product(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Rotor> for Translator {
    type Output = Scalar;

    fn regressive_product(self, other: Rotor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] + self.group0()[3] * other.group0()[3] } }
    }
}

impl OuterProduct<Rotor> for Translator {
    type Output = Motor;

    fn outer_product(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 1, 1, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl LeftContraction<Rotor> for Translator {
    type Output = Rotor;

    fn left_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl RightContraction<Rotor> for Translator {
    type Output = Translator;

    fn right_contraction(self, other: Rotor) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl ScalarProduct<Rotor> for Translator {
    type Output = Scalar;

    fn scalar_product(self, other: Rotor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] } }
    }
}

impl GeometricProduct<Point> for Translator {
    type Output = Point;

    fn geometric_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + self.group0() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) } }
    }
}

impl RegressiveProduct<Point> for Translator {
    type Output = Plane;

    fn regressive_product(self, other: Point) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 1, 1, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl OuterProduct<Point> for Translator {
    type Output = Point;

    fn outer_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl InnerProduct<Point> for Translator {
    type Output = Point;

    fn inner_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl LeftContraction<Point> for Translator {
    type Output = Point;

    fn left_contraction(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl Into<IdealPoint> for Translator {
    fn into(self) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]) } }
    }
}

impl Add<IdealPoint> for Translator {
    type Output = Translator;

    fn add(self, other: IdealPoint) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() + Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
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
        Translator { groups: TranslatorGroups { g0: self.group0() - Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
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
        IdealPoint { groups: IdealPointGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl OuterProduct<IdealPoint> for Translator {
    type Output = IdealPoint;

    fn outer_product(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl InnerProduct<IdealPoint> for Translator {
    type Output = IdealPoint;

    fn inner_product(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl LeftContraction<IdealPoint> for Translator {
    type Output = IdealPoint;

    fn left_contraction(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl InnerProduct<Plane> for Translator {
    type Output = Plane;

    fn inner_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl LeftContraction<Plane> for Translator {
    type Output = Plane;

    fn left_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl RegressiveProduct<Line> for Translator {
    type Output = Scalar;

    fn regressive_product(self, other: Line) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group1()[0] + self.group0()[2] * other.group1()[1] + self.group0()[3] * other.group1()[2] } }
    }
}

impl InnerProduct<Line> for Translator {
    type Output = Line;

    fn inner_product(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0(), g1: Simd32x3::from(self.group0()[0]) * other.group1() } }
    }
}

impl LeftContraction<Line> for Translator {
    type Output = Line;

    fn left_contraction(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0(), g1: Simd32x3::from(self.group0()[0]) * other.group1() } }
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
        Translator { groups: TranslatorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) } }
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
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + self.group0() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl OuterProduct<Translator> for Translator {
    type Output = Translator;

    fn outer_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + self.group0() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl InnerProduct<Translator> for Translator {
    type Output = Translator;

    fn inner_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + self.group0() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl LeftContraction<Translator> for Translator {
    type Output = Translator;

    fn left_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl RightContraction<Translator> for Translator {
    type Output = Translator;

    fn right_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) } }
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
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + other.group0(), g1: self.group0() * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + other.group1() } }
    }
}

impl Sub<Motor> for Translator {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) - other.group0(), g1: self.group0() * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) - other.group1() } }
    }
}

impl GeometricProduct<Motor> for Translator {
    type Output = Motor;

    fn geometric_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Motor> for Translator {
    type Output = Translator;

    fn regressive_product(self, other: Motor) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[1], other.group1()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group1()[0], other.group0()[2]]) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group1()[0]]) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl OuterProduct<Motor> for Translator {
    type Output = Motor;

    fn outer_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 1, 1, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<Motor> for Translator {
    type Output = Motor;

    fn inner_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * other.group1() + self.group0() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl LeftContraction<Motor> for Translator {
    type Output = Motor;

    fn left_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * other.group1() } }
    }
}

impl RightContraction<Motor> for Translator {
    type Output = Translator;

    fn right_contraction(self, other: Motor) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl ScalarProduct<Motor> for Translator {
    type Output = Scalar;

    fn scalar_product(self, other: Motor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] } }
    }
}

impl GeometricProduct<PointAndPlane> for Translator {
    type Output = PointAndPlane;

    fn geometric_product(self, other: PointAndPlane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[3], other.group1()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([0.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) + swizzle!(self.group0(), 0, 1, 1, 1) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([0.0, -1.0, 1.0, -1.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group1()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group1(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl RegressiveProduct<PointAndPlane> for Translator {
    type Output = Plane;

    fn regressive_product(self, other: PointAndPlane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 1, 1, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) } }
    }
}

impl OuterProduct<PointAndPlane> for Translator {
    type Output = PointAndPlane;

    fn outer_product(self, other: PointAndPlane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group1(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + swizzle!(self.group0(), 0, 0, 1, 1) * swizzle!(other.group1(), 0, 0, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() } }
    }
}

impl InnerProduct<PointAndPlane> for Translator {
    type Output = PointAndPlane;

    fn inner_product(self, other: PointAndPlane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group1()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group1(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl LeftContraction<PointAndPlane> for Translator {
    type Output = PointAndPlane;

    fn left_contraction(self, other: PointAndPlane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * other.group1() } }
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
        Motor { groups: MotorGroups { g0: Simd32x4::from(0.0), g1: Simd32x4::from(0.0) } }
    }
}

impl One for Motor {
    fn one() -> Self {
        Motor { groups: MotorGroups { g0: Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(0.0) } }
    }
}

impl Neg for Motor {
    type Output = Motor;

    fn neg(self) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(-1.0), g1: self.group1() * Simd32x4::from(-1.0) } }
    }
}

impl Automorphism for Motor {
    type Output = Motor;

    fn automorphism(self) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0(), g1: self.group1() } }
    }
}

impl Reversal for Motor {
    type Output = Motor;

    fn reversal(self) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]), g1: self.group1() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) } }
    }
}

impl Conjugation for Motor {
    type Output = Motor;

    fn conjugation(self) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]), g1: self.group1() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) } }
    }
}

impl Dual for Motor {
    type Output = Motor;

    fn dual(self) -> Motor {
        Motor { groups: MotorGroups { g0: self.group1(), g1: self.group0() } }
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
        Motor { groups: MotorGroups { g0: self.group0() + Simd32x4::from(other.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: self.group1() } }
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
        Motor { groups: MotorGroups { g0: self.group0() - Simd32x4::from(other.group0()) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: self.group1() } }
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
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl RegressiveProduct<Scalar> for Motor {
    type Output = Scalar;

    fn regressive_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group1()[0] * other.group0() } }
    }
}

impl OuterProduct<Scalar> for Motor {
    type Output = Motor;

    fn outer_product(self, other: Scalar) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for Motor {
    type Output = Motor;

    fn inner_product(self, other: Scalar) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()) } }
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
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()) } }
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
        MultiVector { groups: MultiVectorGroups { g0: self.group0() + other.group0(), g1: other.group1(), g2: other.group2(), g3: self.group1() + other.group3() } }
    }
}

impl Sub<MultiVector> for Motor {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() - other.group0(), g1: Simd32x4::from(0.0) - other.group1(), g2: Simd32x4::from(0.0) - other.group2(), g3: self.group1() - other.group3() } }
    }
}

impl GeometricProduct<MultiVector> for Motor {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group1(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) - Simd32x4::from(self.group1()[0]) * other.group2() + Simd32x4::from(self.group1()[1]) * swizzle!(other.group2(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group2(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group2(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]), g2: Simd32x4::from(self.group0()[0]) * other.group2() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group2(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group2(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group2(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]), g3: Simd32x4::from(self.group0()[0]) * other.group3() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group3(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group3(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group3(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl RegressiveProduct<MultiVector> for Motor {
    type Output = MultiVector;

    fn regressive_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group3(), 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group3(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group3(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * other.group0() + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group3()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.group1()[0]) * other.group1() + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group1()[3]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group1(), 1, 0, 0, 0) * swizzle!(other.group1(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]), g2: Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group1(), 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[0]) * other.group2() + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group2()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group2()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group0()[0], self.group1()[1], self.group0()[1], self.group0()[1]]) * Simd32x4::from([other.group1()[0], other.group2()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]), g3: Simd32x4::from(self.group1()[0]) * other.group3() + self.group1() * Simd32x4::from(other.group3()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl OuterProduct<MultiVector> for Motor {
    type Output = MultiVector;

    fn outer_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + self.group0() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group1()[1]) * swizzle!(other.group2(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group2(), 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group2(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + self.group0() * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]), g2: Simd32x4::from(self.group0()[0]) * other.group2() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group2()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group2()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group2(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g3: Simd32x4::from(self.group0()[0]) * other.group3() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group3()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group3()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group3(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<MultiVector> for Motor {
    type Output = MultiVector;

    fn inner_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 1, 1, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group1()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) - Simd32x4::from(self.group1()[0]) * other.group2() + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group2()[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group2()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group2()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group1(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: Simd32x4::from(self.group0()[0]) * other.group2() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group2(), 3, 3, 0, 1) * Simd32x4::from([0.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group2(), 2, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) + swizzle!(self.group0(), 0, 1, 1, 1) * swizzle!(other.group2(), 0, 0, 3, 2) * Simd32x4::from([0.0, -1.0, 1.0, -1.0]), g3: Simd32x4::from(self.group0()[0]) * other.group3() + Simd32x4::from(self.group1()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + self.group0() * Simd32x4::from(other.group3()[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) } }
    }
}

impl LeftContraction<MultiVector> for Motor {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group1()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group1(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: Simd32x4::from(self.group0()[0]) * other.group2() + self.group0() * Simd32x4::from(other.group2()[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]), g3: Simd32x4::from(self.group0()[0]) * other.group3() + self.group0() * Simd32x4::from(other.group3()[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) } }
    }
}

impl ScalarProduct<MultiVector> for Motor {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3] } }
    }
}

impl Into<Rotor> for Motor {
    fn into(self) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() } }
    }
}

impl Add<Rotor> for Motor {
    type Output = Motor;

    fn add(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() + other.group0(), g1: self.group1() } }
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
        Motor { groups: MotorGroups { g0: self.group0() - other.group0(), g1: self.group1() } }
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
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]), g1: Simd32x4::from(self.group1()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Rotor> for Motor {
    type Output = Rotor;

    fn regressive_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group1()[0]) * other.group0() + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group1(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl OuterProduct<Rotor> for Motor {
    type Output = Motor;

    fn outer_product(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + self.group0() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<Rotor> for Motor {
    type Output = Motor;

    fn inner_product(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 1, 1, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]), g1: Simd32x4::from(self.group1()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + self.group1() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl LeftContraction<Rotor> for Motor {
    type Output = Rotor;

    fn left_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl RightContraction<Rotor> for Motor {
    type Output = Motor;

    fn right_contraction(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.group1()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + self.group1() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl ScalarProduct<Rotor> for Motor {
    type Output = Scalar;

    fn scalar_product(self, other: Rotor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3] } }
    }
}

impl GeometricProduct<Point> for Motor {
    type Output = PointAndPlane;

    fn geometric_product(self, other: Point) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from([self.group0()[0], self.group1()[1], self.group0()[1], self.group0()[1]]) * swizzle!(other.group0(), 0, 0, 3, 2) * Simd32x4::from([0.0, -1.0, 1.0, -1.0]), g1: Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 1, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) } }
    }
}

impl RegressiveProduct<Point> for Motor {
    type Output = PointAndPlane;

    fn regressive_product(self, other: Point) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group1()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group1()[1], self.group1()[1], self.group0()[1], self.group0()[1]]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl OuterProduct<Point> for Motor {
    type Output = Point;

    fn outer_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl InnerProduct<Point> for Motor {
    type Output = PointAndPlane;

    fn inner_product(self, other: Point) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 1, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) } }
    }
}

impl LeftContraction<Point> for Motor {
    type Output = PointAndPlane;

    fn left_contraction(self, other: Point) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 1, 1, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) } }
    }
}

impl Into<IdealPoint> for Motor {
    fn into(self) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x3::from([self.group1()[1], self.group1()[2], self.group1()[3]]) } }
    }
}

impl Add<IdealPoint> for Motor {
    type Output = Motor;

    fn add(self, other: IdealPoint) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0(), g1: self.group1() + Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
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
        Motor { groups: MotorGroups { g0: self.group0(), g1: self.group1() - Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl SubAssign<IdealPoint> for Motor {
    fn sub_assign(&mut self, other: IdealPoint) {
        *self = (*self).sub(other);
    }
}

impl RegressiveProduct<IdealPoint> for Motor {
    type Output = Translator;

    fn regressive_product(self, other: IdealPoint) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from([self.group0()[1], self.group1()[0], self.group1()[0], self.group1()[0]]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl InnerProduct<IdealPoint> for Motor {
    type Output = IdealPoint;

    fn inner_product(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl LeftContraction<IdealPoint> for Motor {
    type Output = IdealPoint;

    fn left_contraction(self, other: IdealPoint) -> IdealPoint {
        IdealPoint { groups: IdealPointGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl GeometricProduct<Plane> for Motor {
    type Output = PointAndPlane;

    fn geometric_product(self, other: Plane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 1, 1, 2, 3) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from([self.group0()[1], self.group0()[1], self.group1()[1], self.group1()[1]]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]), g1: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from([self.group1()[1], self.group0()[0], self.group0()[1], self.group0()[1]]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) } }
    }
}

impl RegressiveProduct<Plane> for Motor {
    type Output = Plane;

    fn regressive_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group1()[0]) * other.group0() } }
    }
}

impl OuterProduct<Plane> for Motor {
    type Output = PointAndPlane;

    fn outer_product(self, other: Plane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from([self.group0()[1], self.group0()[1], self.group1()[1], self.group1()[1]]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]), g1: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl LeftContraction<Plane> for Motor {
    type Output = Plane;

    fn left_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl Into<Line> for Motor {
    fn into(self) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from([self.group1()[1], self.group1()[2], self.group1()[3]]), g1: Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]) } }
    }
}

impl Add<Line> for Motor {
    type Output = Motor;

    fn add(self, other: Line) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() + Simd32x4::from([other.group0()[0], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: self.group1() + Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl AddAssign<Line> for Motor {
    fn add_assign(&mut self, other: Line) {
        *self = (*self).add(other);
    }
}

impl Sub<Line> for Motor {
    type Output = Motor;

    fn sub(self, other: Line) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() - Simd32x4::from([other.group0()[0], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: self.group1() - Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl SubAssign<Line> for Motor {
    fn sub_assign(&mut self, other: Line) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Line> for Motor {
    type Output = Motor;

    fn geometric_product(self, other: Line) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[2], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[2], other.group1()[1], other.group1()[0]]) * Simd32x4::from([-1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[2], other.group1()[1], other.group1()[0], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[2], other.group0()[1], other.group0()[0], other.group0()[2]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[2], other.group1()[1]]) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], other.group1()[2], other.group1()[1], other.group1()[0]]) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group1()[2], other.group1()[1], other.group1()[0], other.group1()[2]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl RightContraction<Line> for Motor {
    type Output = Translator;

    fn right_contraction(self, other: Line) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from([self.group0()[1], self.group1()[0], self.group1()[0], self.group1()[0]]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from(-1.0) } }
    }
}

impl ScalarProduct<Line> for Motor {
    type Output = Scalar;

    fn scalar_product(self, other: Line) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[1] * other.group1()[0] - self.group0()[2] * other.group1()[1] - self.group0()[3] * other.group1()[2] } }
    }
}

impl Into<Translator> for Motor {
    fn into(self) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from([self.group0()[0], self.group1()[1], self.group1()[2], self.group1()[3]]) } }
    }
}

impl Add<Translator> for Motor {
    type Output = Motor;

    fn add(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() + Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: self.group1() + other.group0() * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
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
        Motor { groups: MotorGroups { g0: self.group0() - Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: self.group1() - other.group0() * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
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
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 1, 3, 2) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 3, 2, 1) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 2, 1, 3) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group1()[0], self.group0()[0], self.group0()[0], self.group0()[0]]) * other.group0() } }
    }
}

impl RegressiveProduct<Translator> for Motor {
    type Output = Translator;

    fn regressive_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * other.group0() + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl OuterProduct<Translator> for Motor {
    type Output = Motor;

    fn outer_product(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 1, 2, 3) } }
    }
}

impl InnerProduct<Translator> for Motor {
    type Output = Motor;

    fn inner_product(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group1()[0], self.group0()[0], self.group0()[0], self.group0()[0]]) * other.group0() } }
    }
}

impl LeftContraction<Translator> for Motor {
    type Output = Translator;

    fn left_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl RightContraction<Translator> for Motor {
    type Output = Motor;

    fn right_contraction(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: self.group1() * Simd32x4::from(other.group0()[0]) } }
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
        Motor { groups: MotorGroups { g0: self.group0() + other.group0(), g1: self.group1() + other.group1() } }
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
        Motor { groups: MotorGroups { g0: self.group0() - other.group0(), g1: self.group1() - other.group1() } }
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
        Motor { groups: MotorGroups { g0: self.group0() * other.group0(), g1: self.group1() * other.group1() } }
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
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) } }
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
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group1(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Motor> for Motor {
    type Output = Motor;

    fn regressive_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group1(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * other.group0() + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.group1()[0]) * other.group1() + self.group1() * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl OuterProduct<Motor> for Motor {
    type Output = Motor;

    fn outer_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + self.group0() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group1()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group1(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl InnerProduct<Motor> for Motor {
    type Output = Motor;

    fn inner_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 1, 1, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group1()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + self.group0() * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) } }
    }
}

impl LeftContraction<Motor> for Motor {
    type Output = Motor;

    fn left_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + self.group0() * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) } }
    }
}

impl RightContraction<Motor> for Motor {
    type Output = Motor;

    fn right_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.group1()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + self.group1() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl ScalarProduct<Motor> for Motor {
    type Output = Scalar;

    fn scalar_product(self, other: Motor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3] } }
    }
}

impl Add<PointAndPlane> for Motor {
    type Output = MultiVector;

    fn add(self, other: PointAndPlane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0(), g1: Simd32x4::from([other.group1()[0], other.group0()[1], other.group0()[2], other.group0()[3]]), g2: Simd32x4::from([other.group0()[0], other.group1()[1], other.group1()[2], other.group1()[3]]), g3: self.group1() } }
    }
}

impl Sub<PointAndPlane> for Motor {
    type Output = MultiVector;

    fn sub(self, other: PointAndPlane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0(), g1: Simd32x4::from(0.0) - Simd32x4::from([other.group1()[0], other.group0()[1], other.group0()[2], other.group0()[3]]), g2: Simd32x4::from(0.0) - Simd32x4::from([other.group0()[0], other.group1()[1], other.group1()[2], other.group1()[3]]), g3: self.group1() } }
    }
}

impl GeometricProduct<PointAndPlane> for Motor {
    type Output = PointAndPlane;

    fn geometric_product(self, other: PointAndPlane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group0()[3], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[2], other.group0()[3], other.group1()[0], other.group0()[1]]) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[3], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([0.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[3], other.group1()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([0.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * other.group1() * Simd32x4::from([0.0, -1.0, -1.0, -1.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[2], other.group1()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[3], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group1()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl RegressiveProduct<PointAndPlane> for Motor {
    type Output = PointAndPlane;

    fn regressive_product(self, other: PointAndPlane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group1()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[0]) * other.group1() + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group1()[1], self.group1()[1], self.group0()[1], self.group0()[1]]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) } }
    }
}

impl OuterProduct<PointAndPlane> for Motor {
    type Output = PointAndPlane;

    fn outer_product(self, other: PointAndPlane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group1(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group1(), 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group1(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from([self.group0()[1], self.group0()[1], self.group1()[1], self.group1()[1]]) * swizzle!(other.group1(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() } }
    }
}

impl InnerProduct<PointAndPlane> for Motor {
    type Output = PointAndPlane;

    fn inner_product(self, other: PointAndPlane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group1()[0]) * other.group1() * Simd32x4::from([0.0, -1.0, -1.0, -1.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[2], other.group1()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[3], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group1()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl LeftContraction<PointAndPlane> for Motor {
    type Output = PointAndPlane;

    fn left_contraction(self, other: PointAndPlane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 1, 1, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) } }
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

impl Zero for PointAndPlane {
    fn zero() -> Self {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(0.0), g1: Simd32x4::from(0.0) } }
    }
}

impl One for PointAndPlane {
    fn one() -> Self {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(0.0), g1: Simd32x4::from(0.0) } }
    }
}

impl Neg for PointAndPlane {
    type Output = PointAndPlane;

    fn neg(self) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group0() * Simd32x4::from(-1.0), g1: self.group1() * Simd32x4::from(-1.0) } }
    }
}

impl Automorphism for PointAndPlane {
    type Output = PointAndPlane;

    fn automorphism(self) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group0() * Simd32x4::from(-1.0), g1: self.group1() * Simd32x4::from(-1.0) } }
    }
}

impl Reversal for PointAndPlane {
    type Output = PointAndPlane;

    fn reversal(self) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group0() * Simd32x4::from(-1.0), g1: self.group1() } }
    }
}

impl Conjugation for PointAndPlane {
    type Output = PointAndPlane;

    fn conjugation(self) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group0(), g1: self.group1() * Simd32x4::from(-1.0) } }
    }
}

impl Dual for PointAndPlane {
    type Output = PointAndPlane;

    fn dual(self) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group1(), g1: self.group0() * Simd32x4::from(-1.0) } }
    }
}

impl GeometricProduct<Scalar> for PointAndPlane {
    type Output = PointAndPlane;

    fn geometric_product(self, other: Scalar) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl OuterProduct<Scalar> for PointAndPlane {
    type Output = PointAndPlane;

    fn outer_product(self, other: Scalar) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl InnerProduct<Scalar> for PointAndPlane {
    type Output = PointAndPlane;

    fn inner_product(self, other: Scalar) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl RightContraction<Scalar> for PointAndPlane {
    type Output = PointAndPlane;

    fn right_contraction(self, other: Scalar) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl Add<MultiVector> for PointAndPlane {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: other.group0(), g1: Simd32x4::from([self.group1()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) + other.group1(), g2: Simd32x4::from([self.group0()[0], self.group1()[1], self.group1()[2], self.group1()[3]]) + other.group2(), g3: other.group3() } }
    }
}

impl Sub<MultiVector> for PointAndPlane {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(0.0) - other.group0(), g1: Simd32x4::from([self.group1()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) - other.group1(), g2: Simd32x4::from([self.group0()[0], self.group1()[1], self.group1()[2], self.group1()[3]]) - other.group2(), g3: Simd32x4::from(0.0) - other.group3() } }
    }
}

impl GeometricProduct<MultiVector> for PointAndPlane {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group2() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group2(), 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group2(), 2, 3, 0, 1) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group2(), 3, 2, 1, 0) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]), g1: Simd32x4::from(self.group0()[0]) * other.group3() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group3(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group3(), 2, 3, 0, 1) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group3(), 3, 2, 1, 0) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]), g2: Simd32x4::from(self.group0()[0]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]), g3: Simd32x4::from(0.0) - Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[1]) * swizzle!(other.group2(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group2(), 2, 3, 0, 1) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group2(), 3, 2, 1, 0) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[0]) * other.group2() + Simd32x4::from(self.group1()[1]) * swizzle!(other.group1(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group1(), 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group1(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) } }
    }
}

impl ScalarProduct<MultiVector> for PointAndPlane {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group2()[0] + self.group1()[1] * other.group2()[1] + self.group1()[2] * other.group2()[2] + self.group1()[3] * other.group2()[3] } }
    }
}

impl GeometricProduct<Rotor> for PointAndPlane {
    type Output = PointAndPlane;

    fn geometric_product(self, other: Rotor) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 0, 1) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 2, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 1, 1, 2, 3) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 0, 1, 1, 1) * swizzle!(other.group0(), 0, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]), g1: Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 0, 0, 3, 2) * Simd32x4::from([0.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 0, 1) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 2, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 1, 2, 3) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) } }
    }
}

impl OuterProduct<Rotor> for PointAndPlane {
    type Output = PointAndPlane;

    fn outer_product(self, other: Rotor) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 1, 1, 2, 3) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + self.group0() * Simd32x4::from(other.group0()[0]), g1: self.group1() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl InnerProduct<Rotor> for PointAndPlane {
    type Output = PointAndPlane;

    fn inner_product(self, other: Rotor) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 0, 0, 3, 2) * Simd32x4::from([0.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 0, 1) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 2, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 1, 2, 3) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) } }
    }
}

impl RightContraction<Rotor> for PointAndPlane {
    type Output = PointAndPlane;

    fn right_contraction(self, other: Rotor) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 1, 2, 3) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) } }
    }
}

impl Into<Point> for PointAndPlane {
    fn into(self) -> Point {
        Point { groups: PointGroups { g0: self.group0() } }
    }
}

impl Add<Point> for PointAndPlane {
    type Output = PointAndPlane;

    fn add(self, other: Point) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group0() + other.group0(), g1: self.group1() } }
    }
}

impl AddAssign<Point> for PointAndPlane {
    fn add_assign(&mut self, other: Point) {
        *self = (*self).add(other);
    }
}

impl Sub<Point> for PointAndPlane {
    type Output = PointAndPlane;

    fn sub(self, other: Point) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group0() - other.group0(), g1: self.group1() } }
    }
}

impl SubAssign<Point> for PointAndPlane {
    fn sub_assign(&mut self, other: Point) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Point> for PointAndPlane {
    type Output = Motor;

    fn geometric_product(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[0], self.group1()[1], self.group1()[2], self.group1()[3]]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 1, 1, 3, 2) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 3, 2, 1) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 3, 2, 1, 3) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from([self.group1()[0], self.group0()[0], self.group0()[0], self.group0()[0]]) * other.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) } }
    }
}

impl RightContraction<Point> for PointAndPlane {
    type Output = Scalar;

    fn right_contraction(self, other: Point) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] } }
    }
}

impl ScalarProduct<Point> for PointAndPlane {
    type Output = Scalar;

    fn scalar_product(self, other: Point) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] } }
    }
}

impl RegressiveProduct<IdealPoint> for PointAndPlane {
    type Output = Plane;

    fn regressive_product(self, other: IdealPoint) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl Into<Plane> for PointAndPlane {
    fn into(self) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group1() } }
    }
}

impl Add<Plane> for PointAndPlane {
    type Output = PointAndPlane;

    fn add(self, other: Plane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group0(), g1: self.group1() + other.group0() } }
    }
}

impl AddAssign<Plane> for PointAndPlane {
    fn add_assign(&mut self, other: Plane) {
        *self = (*self).add(other);
    }
}

impl Sub<Plane> for PointAndPlane {
    type Output = PointAndPlane;

    fn sub(self, other: Plane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group0(), g1: self.group1() - other.group0() } }
    }
}

impl SubAssign<Plane> for PointAndPlane {
    fn sub_assign(&mut self, other: Plane) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Plane> for PointAndPlane {
    type Output = Motor;

    fn geometric_product(self, other: Plane) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 1, 1, 3, 2) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 3, 2, 1) * Simd32x4::from([1.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 3, 2, 1, 3) * Simd32x4::from([1.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * other.group0() * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 1, 1, 3, 2) * Simd32x4::from([-1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 2, 3, 2, 1) * Simd32x4::from([-1.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 3, 2, 1, 3) * Simd32x4::from([-1.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from([self.group0()[0], self.group1()[0], self.group1()[0], self.group1()[0]]) * other.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl RegressiveProduct<Plane> for PointAndPlane {
    type Output = Scalar;

    fn regressive_product(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] - self.group0()[3] * other.group0()[3] } }
    }
}

impl LeftContraction<Plane> for PointAndPlane {
    type Output = Scalar;

    fn left_contraction(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2] + self.group1()[3] * other.group0()[3] } }
    }
}

impl ScalarProduct<Plane> for PointAndPlane {
    type Output = Scalar;

    fn scalar_product(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2] + self.group1()[3] * other.group0()[3] } }
    }
}

impl GeometricProduct<Line> for PointAndPlane {
    type Output = PointAndPlane;

    fn geometric_product(self, other: Line) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[2], other.group1()[1]]) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[2], other.group1()[0]]) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[1], other.group1()[1], other.group1()[0], other.group1()[1]]) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group0()[2], other.group0()[1]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], other.group0()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group1()[2], other.group0()[1], other.group0()[0], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group1()[2], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], other.group1()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([-1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[2], other.group1()[1], other.group1()[0], other.group0()[2]]) * Simd32x4::from([-1.0, 1.0, -1.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) } }
    }
}

impl RegressiveProduct<Line> for PointAndPlane {
    type Output = Plane;

    fn regressive_product(self, other: Line) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group1()[2], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group1()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([-1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[2], other.group1()[1], other.group1()[0], other.group0()[2]]) * Simd32x4::from([-1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) } }
    }
}

impl OuterProduct<Line> for PointAndPlane {
    type Output = Point;

    fn outer_product(self, other: Line) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group0()[2], other.group0()[1]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], other.group0()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group1()[2], other.group0()[1], other.group0()[0], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) } }
    }
}

impl InnerProduct<Line> for PointAndPlane {
    type Output = Plane;

    fn inner_product(self, other: Line) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group1()[2], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], other.group1()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([-1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[2], other.group1()[1], other.group1()[0], other.group0()[2]]) * Simd32x4::from([-1.0, 1.0, -1.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) } }
    }
}

impl LeftContraction<Line> for PointAndPlane {
    type Output = Plane;

    fn left_contraction(self, other: Line) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], other.group1()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([-1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[2], other.group1()[1], other.group1()[0], other.group0()[2]]) * Simd32x4::from([-1.0, 1.0, -1.0, 0.0]) + swizzle!(self.group1(), 1, 0, 1, 1) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group1()[2], other.group1()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) } }
    }
}

impl RightContraction<Line> for PointAndPlane {
    type Output = Plane;

    fn right_contraction(self, other: Line) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * Simd32x4::from([other.group1()[0], other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) } }
    }
}

impl GeometricProduct<Translator> for PointAndPlane {
    type Output = PointAndPlane;

    fn geometric_product(self, other: Translator) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + self.group0() * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl RegressiveProduct<Translator> for PointAndPlane {
    type Output = Plane;

    fn regressive_product(self, other: Translator) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 1, 2, 3) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl OuterProduct<Translator> for PointAndPlane {
    type Output = PointAndPlane;

    fn outer_product(self, other: Translator) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + self.group0() * Simd32x4::from(other.group0()[0]), g1: self.group1() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl InnerProduct<Translator> for PointAndPlane {
    type Output = PointAndPlane;

    fn inner_product(self, other: Translator) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl RightContraction<Translator> for PointAndPlane {
    type Output = PointAndPlane;

    fn right_contraction(self, other: Translator) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: self.group1() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl Add<Motor> for PointAndPlane {
    type Output = MultiVector;

    fn add(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: other.group0(), g1: Simd32x4::from([self.group1()[0], self.group0()[1], self.group0()[2], self.group0()[3]]), g2: Simd32x4::from([self.group0()[0], self.group1()[1], self.group1()[2], self.group1()[3]]), g3: other.group1() } }
    }
}

impl Sub<Motor> for PointAndPlane {
    type Output = MultiVector;

    fn sub(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x4::from(0.0) - other.group0(), g1: Simd32x4::from([self.group1()[0], self.group0()[1], self.group0()[2], self.group0()[3]]), g2: Simd32x4::from([self.group0()[0], self.group1()[1], self.group1()[2], self.group1()[3]]), g3: Simd32x4::from(0.0) - other.group1() } }
    }
}

impl GeometricProduct<Motor> for PointAndPlane {
    type Output = PointAndPlane;

    fn geometric_product(self, other: Motor) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group1()[1], other.group1()[2], other.group1()[3]]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 0, 1) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group0()[3]) * swizzle!(other.group0(), 2, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 1, 1, 2, 3) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[1], other.group1()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[2], other.group1()[3], other.group1()[0], other.group1()[1]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[3], other.group1()[2], other.group1()[1], other.group1()[0]]) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + swizzle!(self.group0(), 0, 1, 1, 1) * swizzle!(other.group0(), 0, 0, 3, 2) * Simd32x4::from([0.0, 1.0, 1.0, -1.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group0()[1], other.group0()[2], other.group0()[3]]) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[1], other.group0()[0], other.group0()[3], other.group0()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[2], other.group0()[3], other.group0()[0], other.group0()[1]]) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group1()[3], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl RegressiveProduct<Motor> for PointAndPlane {
    type Output = PointAndPlane;

    fn regressive_product(self, other: Motor) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group0() * Simd32x4::from(other.group1()[0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[1], other.group1()[1], other.group0()[3], other.group0()[2]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[2], other.group0()[3], other.group1()[2], other.group0()[1]]) * Simd32x4::from([-1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[3], other.group0()[2], other.group0()[1], other.group1()[3]]) * Simd32x4::from([-1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group1()[0], self.group0()[0], self.group0()[0], self.group0()[0]]) * other.group1() } }
    }
}

impl OuterProduct<Motor> for PointAndPlane {
    type Output = PointAndPlane;

    fn outer_product(self, other: Motor) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 1, 1, 2, 3) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group1()[3], other.group1()[2]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[2], other.group1()[3], other.group0()[2], other.group1()[1]]) * Simd32x4::from([1.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[3], other.group1()[2], other.group1()[1], other.group0()[3]]) * Simd32x4::from([1.0, -1.0, 1.0, 0.0]) + self.group0() * Simd32x4::from(other.group0()[0]), g1: self.group1() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl InnerProduct<Motor> for PointAndPlane {
    type Output = PointAndPlane;

    fn inner_product(self, other: Motor) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + self.group0() * Simd32x4::from(other.group0()[0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group0()[1], other.group0()[2], other.group0()[3]]) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[1], other.group0()[0], other.group0()[3], other.group0()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[2], other.group0()[3], other.group0()[0], other.group0()[1]]) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group1()[3], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) } }
    }
}

impl RightContraction<Motor> for PointAndPlane {
    type Output = PointAndPlane;

    fn right_contraction(self, other: Motor) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 1, 0, 0, 0) * swizzle!(other.group0(), 1, 1, 2, 3) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) } }
    }
}

impl Add<PointAndPlane> for PointAndPlane {
    type Output = PointAndPlane;

    fn add(self, other: PointAndPlane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group0() + other.group0(), g1: self.group1() + other.group1() } }
    }
}

impl AddAssign<PointAndPlane> for PointAndPlane {
    fn add_assign(&mut self, other: PointAndPlane) {
        *self = (*self).add(other);
    }
}

impl Sub<PointAndPlane> for PointAndPlane {
    type Output = PointAndPlane;

    fn sub(self, other: PointAndPlane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group0() - other.group0(), g1: self.group1() - other.group1() } }
    }
}

impl SubAssign<PointAndPlane> for PointAndPlane {
    fn sub_assign(&mut self, other: PointAndPlane) {
        *self = (*self).sub(other);
    }
}

impl Mul<PointAndPlane> for PointAndPlane {
    type Output = PointAndPlane;

    fn mul(self, other: PointAndPlane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: self.group0() * other.group0(), g1: self.group1() * other.group1() } }
    }
}

impl MulAssign<PointAndPlane> for PointAndPlane {
    fn mul_assign(&mut self, other: PointAndPlane) {
        *self = (*self).mul(other);
    }
}

impl Div<PointAndPlane> for PointAndPlane {
    type Output = PointAndPlane;

    fn div(self, other: PointAndPlane) -> PointAndPlane {
        PointAndPlane { groups: PointAndPlaneGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) / Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl DivAssign<PointAndPlane> for PointAndPlane {
    fn div_assign(&mut self, other: PointAndPlane) {
        *self = (*self).div(other);
    }
}

impl GeometricProduct<PointAndPlane> for PointAndPlane {
    type Output = Motor;

    fn geometric_product(self, other: PointAndPlane) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group1()[1], other.group1()[2], other.group1()[3]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[1], other.group0()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[2], other.group1()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group1()[3], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]), g1: Simd32x4::from(0.0) - Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group0()[1], other.group0()[2], other.group0()[3]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[1], other.group0()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[2], other.group1()[3], other.group0()[0], other.group1()[1]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[3], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[0], other.group1()[1], other.group1()[2], other.group1()[3]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[1], other.group1()[0], other.group0()[3], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[2], other.group0()[3], other.group1()[0], other.group0()[1]]) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[3], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) } }
    }
}

impl ScalarProduct<PointAndPlane> for PointAndPlane {
    type Output = Scalar;

    fn scalar_product(self, other: PointAndPlane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] + self.group1()[1] * other.group1()[1] + self.group1()[2] * other.group1()[2] + self.group1()[3] * other.group1()[3] } }
    }
}

impl SquaredMagnitude for PointAndPlane {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for PointAndPlane {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.squared_magnitude().group0().sqrt() } }
    }
}

impl Scale for PointAndPlane {
    type Output = PointAndPlane;

    fn scale(self, other: f32) -> PointAndPlane {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: other } })
    }
}

impl Signum for PointAndPlane {
    type Output = PointAndPlane;

    fn signum(self) -> PointAndPlane {
        self.geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.magnitude().group0() } })
    }
}

impl Inverse for PointAndPlane {
    type Output = PointAndPlane;

    fn inverse(self) -> PointAndPlane {
        self.reversal().geometric_product(Scalar { groups: ScalarGroups { g0: 1.0 / self.squared_magnitude().group0() } })
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

impl GeometricQuotient<Line> for Line {
    type Output = Motor;

    fn geometric_quotient(self, other: Line) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Line> for Line {
    type Output = Line;

    fn transformation(self, other: Line) -> Line {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Motor> for Line {
    type Output = Motor;

    fn geometric_quotient(self, other: Motor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Motor> for Line {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MultiVector> for Line {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for Line {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Plane> for Line {
    type Output = PointAndPlane;

    fn geometric_quotient(self, other: Plane) -> PointAndPlane {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Plane> for Line {
    type Output = Plane;

    fn transformation(self, other: Plane) -> Plane {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<PointAndPlane> for Line {
    type Output = PointAndPlane;

    fn geometric_quotient(self, other: PointAndPlane) -> PointAndPlane {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<PointAndPlane> for Line {
    type Output = PointAndPlane;

    fn transformation(self, other: PointAndPlane) -> PointAndPlane {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Rotor> for Line {
    type Output = Motor;

    fn geometric_quotient(self, other: Rotor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Rotor> for Line {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Scalar> for Line {
    type Output = Line;

    fn geometric_quotient(self, other: Scalar) -> Line {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Scalar> for Line {
    type Output = Scalar;

    fn transformation(self, other: Scalar) -> Scalar {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Line> for Motor {
    type Output = Motor;

    fn geometric_quotient(self, other: Line) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Line> for Motor {
    type Output = Line;

    fn transformation(self, other: Line) -> Line {
        self.geometric_product(other).geometric_product(self.reversal()).into()
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
    type Output = PointAndPlane;

    fn geometric_quotient(self, other: Plane) -> PointAndPlane {
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
    type Output = PointAndPlane;

    fn geometric_quotient(self, other: Point) -> PointAndPlane {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Point> for Motor {
    type Output = Point;

    fn transformation(self, other: Point) -> Point {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<PointAndPlane> for Motor {
    type Output = PointAndPlane;

    fn geometric_quotient(self, other: PointAndPlane) -> PointAndPlane {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<PointAndPlane> for Motor {
    type Output = PointAndPlane;

    fn transformation(self, other: PointAndPlane) -> PointAndPlane {
        self.geometric_product(other).geometric_product(self.reversal())
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

impl GeometricQuotient<Line> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Line> for MultiVector {
    type Output = Line;

    fn transformation(self, other: Line) -> Line {
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

impl GeometricQuotient<PointAndPlane> for MultiVector {
    type Output = MultiVector;

    fn geometric_quotient(self, other: PointAndPlane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<PointAndPlane> for MultiVector {
    type Output = PointAndPlane;

    fn transformation(self, other: PointAndPlane) -> PointAndPlane {
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

impl GeometricQuotient<Line> for Plane {
    type Output = PointAndPlane;

    fn geometric_quotient(self, other: Line) -> PointAndPlane {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Line> for Plane {
    type Output = Line;

    fn transformation(self, other: Line) -> Line {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Motor> for Plane {
    type Output = PointAndPlane;

    fn geometric_quotient(self, other: Motor) -> PointAndPlane {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Motor> for Plane {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
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

impl GeometricQuotient<PointAndPlane> for Plane {
    type Output = Motor;

    fn geometric_quotient(self, other: PointAndPlane) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<PointAndPlane> for Plane {
    type Output = PointAndPlane;

    fn transformation(self, other: PointAndPlane) -> PointAndPlane {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Rotor> for Plane {
    type Output = PointAndPlane;

    fn geometric_quotient(self, other: Rotor) -> PointAndPlane {
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

impl GeometricQuotient<Motor> for Point {
    type Output = PointAndPlane;

    fn geometric_quotient(self, other: Motor) -> PointAndPlane {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Motor> for Point {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
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

impl GeometricQuotient<PointAndPlane> for Point {
    type Output = Motor;

    fn geometric_quotient(self, other: PointAndPlane) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<PointAndPlane> for Point {
    type Output = PointAndPlane;

    fn transformation(self, other: PointAndPlane) -> PointAndPlane {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Rotor> for Point {
    type Output = PointAndPlane;

    fn geometric_quotient(self, other: Rotor) -> PointAndPlane {
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

impl GeometricQuotient<Line> for PointAndPlane {
    type Output = PointAndPlane;

    fn geometric_quotient(self, other: Line) -> PointAndPlane {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Line> for PointAndPlane {
    type Output = Line;

    fn transformation(self, other: Line) -> Line {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Motor> for PointAndPlane {
    type Output = PointAndPlane;

    fn geometric_quotient(self, other: Motor) -> PointAndPlane {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Motor> for PointAndPlane {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<MultiVector> for PointAndPlane {
    type Output = MultiVector;

    fn geometric_quotient(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for PointAndPlane {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Plane> for PointAndPlane {
    type Output = Motor;

    fn geometric_quotient(self, other: Plane) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Plane> for PointAndPlane {
    type Output = Plane;

    fn transformation(self, other: Plane) -> Plane {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Point> for PointAndPlane {
    type Output = Motor;

    fn geometric_quotient(self, other: Point) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Point> for PointAndPlane {
    type Output = Point;

    fn transformation(self, other: Point) -> Point {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<PointAndPlane> for PointAndPlane {
    type Output = Motor;

    fn geometric_quotient(self, other: PointAndPlane) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<PointAndPlane> for PointAndPlane {
    type Output = PointAndPlane;

    fn transformation(self, other: PointAndPlane) -> PointAndPlane {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl GeometricQuotient<Rotor> for PointAndPlane {
    type Output = PointAndPlane;

    fn geometric_quotient(self, other: Rotor) -> PointAndPlane {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Rotor> for PointAndPlane {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Scalar> for PointAndPlane {
    type Output = PointAndPlane;

    fn geometric_quotient(self, other: Scalar) -> PointAndPlane {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Scalar> for PointAndPlane {
    type Output = Scalar;

    fn transformation(self, other: Scalar) -> Scalar {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Translator> for PointAndPlane {
    type Output = PointAndPlane;

    fn geometric_quotient(self, other: Translator) -> PointAndPlane {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Translator> for PointAndPlane {
    type Output = Translator;

    fn transformation(self, other: Translator) -> Translator {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<Line> for Rotor {
    type Output = Motor;

    fn geometric_quotient(self, other: Line) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Line> for Rotor {
    type Output = Line;

    fn transformation(self, other: Line) -> Line {
        self.geometric_product(other).geometric_product(self.reversal()).into()
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
    type Output = PointAndPlane;

    fn geometric_quotient(self, other: Plane) -> PointAndPlane {
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
    type Output = PointAndPlane;

    fn geometric_quotient(self, other: Point) -> PointAndPlane {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Point> for Rotor {
    type Output = Point;

    fn transformation(self, other: Point) -> Point {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl GeometricQuotient<PointAndPlane> for Rotor {
    type Output = PointAndPlane;

    fn geometric_quotient(self, other: PointAndPlane) -> PointAndPlane {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<PointAndPlane> for Rotor {
    type Output = PointAndPlane;

    fn transformation(self, other: PointAndPlane) -> PointAndPlane {
        self.geometric_product(other).geometric_product(self.reversal())
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

impl GeometricQuotient<Line> for Scalar {
    type Output = Line;

    fn geometric_quotient(self, other: Line) -> Line {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Line> for Scalar {
    type Output = Line;

    fn transformation(self, other: Line) -> Line {
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

impl GeometricQuotient<PointAndPlane> for Scalar {
    type Output = PointAndPlane;

    fn geometric_quotient(self, other: PointAndPlane) -> PointAndPlane {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<PointAndPlane> for Scalar {
    type Output = PointAndPlane;

    fn transformation(self, other: PointAndPlane) -> PointAndPlane {
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

impl GeometricQuotient<PointAndPlane> for Translator {
    type Output = PointAndPlane;

    fn geometric_quotient(self, other: PointAndPlane) -> PointAndPlane {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<PointAndPlane> for Translator {
    type Output = PointAndPlane;

    fn transformation(self, other: PointAndPlane) -> PointAndPlane {
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

