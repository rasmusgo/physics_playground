#![allow(clippy::too_many_arguments)]
#![allow(non_camel_case_types)]

use std::ops::{Add, BitAnd, BitOr, BitXor, Index, IndexMut, Mul, Neg, Not, Sub};

#[derive(Default)]
pub struct Const<const N: i32> {}

impl<const N: i32> std::fmt::Debug for Const<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        N.fmt(f)
    }
}

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

// 0 - x = x
impl<T> Sub<T> for Zero
where
    T: Neg,
{
    type Output = <T as Neg>::Output;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        -rhs
    }
}

// x - 0 = x
impl<const N: i32> Sub<Zero> for Const<N>
where
    Assert<{ N != 0 }>: IsTrue,
{
    type Output = Const<N>;

    #[inline]
    fn sub(self, _rhs: Zero) -> Const<N> {
        self
    }
}
impl Sub<Zero> for Float {
    type Output = Float;

    #[inline]
    fn sub(self, _rhs: Zero) -> Float {
        self
    }
}

// A - B
impl<const A: i32, const B: i32> Sub<Const<B>> for Const<A>
where
    Assert<{ A != 0 }>: IsTrue,
    Assert<{ B != 0 }>: IsTrue,
    AssertNumeric<{ A - B }>: IsTrue,
{
    type Output = Const<{ A - B }>;

    #[inline]
    fn sub(self, _rhs: Const<B>) -> Self::Output {
        Self::Output {}
    }
}

// N - x
impl<const N: i32> Sub<Float> for Const<N>
where
    Assert<{ N != 0 }>: IsTrue,
{
    type Output = Float;

    #[inline]
    fn sub(self, rhs: Float) -> Float {
        N as Float - rhs
    }
}

// x - N
impl<const N: i32> Sub<Const<N>> for Float
where
    Assert<{ N != 0 }>: IsTrue,
{
    type Output = Float;

    #[inline]
    fn sub(self, _rhs: Const<N>) -> Float {
        self - N as Float
    }
}

impl<const N: i32> Neg for Const<N>
where
    AssertNumeric<{ -N }>: IsTrue,
{
    type Output = Const<{ -N }>;

    fn neg(self) -> Self::Output {
        Self::Output {}
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

// 0 * x = 0
impl<T> Mul<T> for Zero {
    type Output = Zero;

    #[inline]
    fn mul(self, _rhs: T) -> Zero {
        self
    }
}

// x * 0 = 0
impl<const N: i32> Mul<Zero> for Const<N>
where
    Assert<{ N != 0 }>: IsTrue,
{
    type Output = Zero;

    #[inline]
    fn mul(self, rhs: Zero) -> Zero {
        rhs
    }
}
impl Mul<Zero> for Float {
    type Output = Zero;

    #[inline]
    fn mul(self, rhs: Zero) -> Zero {
        rhs
    }
}

// A * B
impl<const A: i32, const B: i32> Mul<Const<B>> for Const<A>
where
    Assert<{ A != 0 }>: IsTrue,
    Assert<{ B != 0 }>: IsTrue,
    AssertNumeric<{ A * B }>: IsTrue,
{
    type Output = Const<{ A * B }>;

    #[inline]
    fn mul(self, _rhs: Const<B>) -> Self::Output {
        Self::Output {}
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
    let _foo: Const<-2> = MinusOne {} + MinusOne {};
    let _foo: Const<2> = One {} + One {};
}

#[test]
fn test_sub() {
    let _foo: Zero = Zero {} - Zero {};
    let _foo: Const<2> = One {} - MinusOne {};
    let _foo: Const<-2> = MinusOne {} - One {};
    let _foo: One = One {} - Zero {};
    let _foo: MinusOne = Zero {} - One {};
    let _foo: MinusOne = MinusOne {} - Zero {};
    let _foo: One = Zero {} - MinusOne {};
    let _foo: Zero = MinusOne {} - MinusOne {};
    let _foo: Zero = One {} - One {};
}

#[test]
fn test_mul() {
    let _foo: Zero = Zero {} * Zero {};
    let _foo: MinusOne = One {} * MinusOne {};
    let _foo: MinusOne = MinusOne {} * One {};
    let _foo: Zero = One {} * Zero {};
    let _foo: Zero = Zero {} * One {};
    let _foo: Zero = MinusOne {} * Zero {};
    let _foo: Zero = Zero {} * MinusOne {};
    let _foo: One = MinusOne {} * MinusOne {};
    let _foo: One = One {} * One {};
}
