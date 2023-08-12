#![allow(clippy::too_many_arguments)]
#![allow(non_camel_case_types)]

use std::ops::{Add, BitAnd, BitOr, BitXor, Index, IndexMut, Mul, Not, Sub};

#[derive(Default, Debug)]
pub struct Const<const N: i32> {}

pub type Zero = Const<0>;
pub type One = Const<1>;
pub type MinusOne = Const<-1>;

pub type Float = f64;

pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}

pub enum AssertNumeric<const CHECK: i32> {}
impl<const N: i32> IsTrue for AssertNumeric<N> {}

// 0 + x = x
impl<T> Add<T> for Zero {
    type Output = T;

    #[inline]
    fn add(self, rhs: T) -> T {
        rhs
    }
}

// x + 0 = x
impl<const N: i32> Add<Zero> for Const<N>
where
    Assert<{ N != 0 }>: IsTrue,
{
    type Output = Const<N>;

    #[inline]
    fn add(self, _rhs: Zero) -> Const<N> {
        self
    }
}
impl Add<Zero> for Float {
    type Output = Float;

    #[inline]
    fn add(self, _rhs: Zero) -> Float {
        self
    }
}

// A + B
impl<const A: i32, const B: i32> Add<Const<B>> for Const<A>
where
    Assert<{ A != 0 }>: IsTrue,
    Assert<{ B != 0 }>: IsTrue,
    AssertNumeric<{ A + B }>: IsTrue,
{
    type Output = Const<{ A + B }>;

    #[inline]
    fn add(self, _rhs: Const<B>) -> Self::Output {
        Self::Output {}
    }
}

// N + x
impl<const N: i32> Add<Float> for Const<N>
where
    Assert<{ N != 0 }>: IsTrue,
{
    type Output = Float;

    #[inline]
    fn add(self, rhs: Float) -> Float {
        N as Float + rhs
    }
}

// x + N
impl<const N: i32> Add<Const<N>> for Float
where
    Assert<{ N != 0 }>: IsTrue,
{
    type Output = Float;

    #[inline]
    fn add(self, _rhs: Const<N>) -> Float {
        self + N as Float
    }
}

// A == B
impl<const A: i32, const B: i32> PartialEq<Const<B>> for Const<A> {
    #[inline]
    fn eq(&self, _other: &Const<B>) -> bool {
        A == B
    }
}
impl<const N: i32> PartialEq<Float> for Const<N> {
    #[inline]
    fn eq(&self, other: &Float) -> bool {
        N as Float == *other
    }
}
impl<const N: i32> PartialEq<Const<N>> for Float {
    #[inline]
    fn eq(&self, _other: &Const<N>) -> bool {
        *self == N as Float
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

    let foo = MinusOne {} + MinusOne {};
    assert_eq!(foo, -2.0);

    let foo = One {} + One {};
    assert_eq!(foo, 2.0);
}
