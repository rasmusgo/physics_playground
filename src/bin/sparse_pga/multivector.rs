#![allow(clippy::too_many_arguments)]
#![allow(non_camel_case_types)]

use crate::coefficient::*;

use std::ops::{Add, BitAnd, BitOr, BitXor, Index, IndexMut, Mul, Neg, Not, Sub};

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

    #[inline]
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
    Sub<
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
    L_one: Sub<R_one>,
    L_e0: Sub<R_e0>,
    L_e1: Sub<R_e1>,
    L_e2: Sub<R_e2>,
    L_e3: Sub<R_e3>,
    L_e01: Sub<R_e01>,
    L_e02: Sub<R_e02>,
    L_e03: Sub<R_e03>,
    L_e12: Sub<R_e12>,
    L_e31: Sub<R_e31>,
    L_e23: Sub<R_e23>,
    L_e021: Sub<R_e021>,
    L_e013: Sub<R_e013>,
    L_e032: Sub<R_e032>,
    L_e123: Sub<R_e123>,
    L_e0123: Sub<R_e0123>,
{
    type Output = MultiVector<
        <L_one as Sub<R_one>>::Output,
        <L_e0 as Sub<R_e0>>::Output,
        <L_e1 as Sub<R_e1>>::Output,
        <L_e2 as Sub<R_e2>>::Output,
        <L_e3 as Sub<R_e3>>::Output,
        <L_e01 as Sub<R_e01>>::Output,
        <L_e02 as Sub<R_e02>>::Output,
        <L_e03 as Sub<R_e03>>::Output,
        <L_e12 as Sub<R_e12>>::Output,
        <L_e31 as Sub<R_e31>>::Output,
        <L_e23 as Sub<R_e23>>::Output,
        <L_e021 as Sub<R_e021>>::Output,
        <L_e013 as Sub<R_e013>>::Output,
        <L_e032 as Sub<R_e032>>::Output,
        <L_e123 as Sub<R_e123>>::Output,
        <L_e0123 as Sub<R_e0123>>::Output,
    >;

    #[inline]
    fn sub(
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
            one: self.one - rhs.one,
            e0: self.e0 - rhs.e0,
            e1: self.e1 - rhs.e1,
            e2: self.e2 - rhs.e2,
            e3: self.e3 - rhs.e3,
            e01: self.e01 - rhs.e01,
            e02: self.e02 - rhs.e02,
            e03: self.e03 - rhs.e03,
            e12: self.e12 - rhs.e12,
            e31: self.e31 - rhs.e31,
            e23: self.e23 - rhs.e23,
            e021: self.e021 - rhs.e021,
            e013: self.e013 - rhs.e013,
            e032: self.e032 - rhs.e032,
            e123: self.e123 - rhs.e123,
            e0123: self.e0123 - rhs.e0123,
        }
    }
}

impl<
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
    > Neg
    for MultiVector<
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
    >
where
    T_one: Neg,
    T_e0: Neg,
    T_e1: Neg,
    T_e2: Neg,
    T_e3: Neg,
    T_e01: Neg,
    T_e02: Neg,
    T_e03: Neg,
    T_e12: Neg,
    T_e31: Neg,
    T_e23: Neg,
    T_e021: Neg,
    T_e013: Neg,
    T_e032: Neg,
    T_e123: Neg,
    T_e0123: Neg,
{
    type Output = MultiVector<
        <T_one as Neg>::Output,
        <T_e0 as Neg>::Output,
        <T_e1 as Neg>::Output,
        <T_e2 as Neg>::Output,
        <T_e3 as Neg>::Output,
        <T_e01 as Neg>::Output,
        <T_e02 as Neg>::Output,
        <T_e03 as Neg>::Output,
        <T_e12 as Neg>::Output,
        <T_e31 as Neg>::Output,
        <T_e23 as Neg>::Output,
        <T_e021 as Neg>::Output,
        <T_e013 as Neg>::Output,
        <T_e032 as Neg>::Output,
        <T_e123 as Neg>::Output,
        <T_e0123 as Neg>::Output,
    >;
    fn neg(self) -> Self::Output {
        Self::Output {
            one: -self.one,
            e0: -self.e0,
            e1: -self.e1,
            e2: -self.e2,
            e3: -self.e3,
            e01: -self.e01,
            e02: -self.e02,
            e03: -self.e03,
            e12: -self.e12,
            e31: -self.e31,
            e23: -self.e23,
            e021: -self.e021,
            e013: -self.e013,
            e032: -self.e032,
            e123: -self.e123,
            e0123: -self.e0123,
        }
    }
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
    Mul<
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
    L_one: Copy
        + Mul<R_one>
        + Mul<R_e0>
        + Mul<R_e1>
        + Mul<R_e2>
        + Mul<R_e3>
        + Mul<R_e01>
        + Mul<R_e02>
        + Mul<R_e03>
        + Mul<R_e12>
        + Mul<R_e31>
        + Mul<R_e23>
        + Mul<R_e021>
        + Mul<R_e013>
        + Mul<R_e032>
        + Mul<R_e123>
        + Mul<R_e0123>,
    L_e0: Copy
        + Mul<R_one>
        + Mul<R_e0>
        + Mul<R_e1>
        + Mul<R_e2>
        + Mul<R_e3>
        + Mul<R_e01>
        + Mul<R_e02>
        + Mul<R_e03>
        + Mul<R_e12>
        + Mul<R_e31>
        + Mul<R_e23>
        + Mul<R_e021>
        + Mul<R_e013>
        + Mul<R_e032>
        + Mul<R_e123>
        + Mul<R_e0123>,
    L_e1: Copy
        + Mul<R_one>
        + Mul<R_e0>
        + Mul<R_e1>
        + Mul<R_e2>
        + Mul<R_e3>
        + Mul<R_e01>
        + Mul<R_e02>
        + Mul<R_e03>
        + Mul<R_e12>
        + Mul<R_e31>
        + Mul<R_e23>
        + Mul<R_e021>
        + Mul<R_e013>
        + Mul<R_e032>
        + Mul<R_e123>
        + Mul<R_e0123>,
    L_e2: Copy
        + Mul<R_one>
        + Mul<R_e0>
        + Mul<R_e1>
        + Mul<R_e2>
        + Mul<R_e3>
        + Mul<R_e01>
        + Mul<R_e02>
        + Mul<R_e03>
        + Mul<R_e12>
        + Mul<R_e31>
        + Mul<R_e23>
        + Mul<R_e021>
        + Mul<R_e013>
        + Mul<R_e032>
        + Mul<R_e123>
        + Mul<R_e0123>,
    L_e3: Copy
        + Mul<R_one>
        + Mul<R_e0>
        + Mul<R_e1>
        + Mul<R_e2>
        + Mul<R_e3>
        + Mul<R_e01>
        + Mul<R_e02>
        + Mul<R_e03>
        + Mul<R_e12>
        + Mul<R_e31>
        + Mul<R_e23>
        + Mul<R_e021>
        + Mul<R_e013>
        + Mul<R_e032>
        + Mul<R_e123>
        + Mul<R_e0123>,
    L_e01: Copy
        + Mul<R_one>
        + Mul<R_e0>
        + Mul<R_e1>
        + Mul<R_e2>
        + Mul<R_e3>
        + Mul<R_e01>
        + Mul<R_e02>
        + Mul<R_e03>
        + Mul<R_e12>
        + Mul<R_e31>
        + Mul<R_e23>
        + Mul<R_e021>
        + Mul<R_e013>
        + Mul<R_e032>
        + Mul<R_e123>
        + Mul<R_e0123>,
    L_e02: Copy
        + Mul<R_one>
        + Mul<R_e0>
        + Mul<R_e1>
        + Mul<R_e2>
        + Mul<R_e3>
        + Mul<R_e01>
        + Mul<R_e02>
        + Mul<R_e03>
        + Mul<R_e12>
        + Mul<R_e31>
        + Mul<R_e23>
        + Mul<R_e021>
        + Mul<R_e013>
        + Mul<R_e032>
        + Mul<R_e123>
        + Mul<R_e0123>,
    L_e03: Copy
        + Mul<R_one>
        + Mul<R_e0>
        + Mul<R_e1>
        + Mul<R_e2>
        + Mul<R_e3>
        + Mul<R_e01>
        + Mul<R_e02>
        + Mul<R_e03>
        + Mul<R_e12>
        + Mul<R_e31>
        + Mul<R_e23>
        + Mul<R_e021>
        + Mul<R_e013>
        + Mul<R_e032>
        + Mul<R_e123>
        + Mul<R_e0123>,
    L_e12: Copy
        + Mul<R_one>
        + Mul<R_e0>
        + Mul<R_e1>
        + Mul<R_e2>
        + Mul<R_e3>
        + Mul<R_e01>
        + Mul<R_e02>
        + Mul<R_e03>
        + Mul<R_e12>
        + Mul<R_e31>
        + Mul<R_e23>
        + Mul<R_e021>
        + Mul<R_e013>
        + Mul<R_e032>
        + Mul<R_e123>
        + Mul<R_e0123>,
    L_e31: Copy
        + Mul<R_one>
        + Mul<R_e0>
        + Mul<R_e1>
        + Mul<R_e2>
        + Mul<R_e3>
        + Mul<R_e01>
        + Mul<R_e02>
        + Mul<R_e03>
        + Mul<R_e12>
        + Mul<R_e31>
        + Mul<R_e23>
        + Mul<R_e021>
        + Mul<R_e013>
        + Mul<R_e032>
        + Mul<R_e123>
        + Mul<R_e0123>,
    L_e23: Copy
        + Mul<R_one>
        + Mul<R_e0>
        + Mul<R_e1>
        + Mul<R_e2>
        + Mul<R_e3>
        + Mul<R_e01>
        + Mul<R_e02>
        + Mul<R_e03>
        + Mul<R_e12>
        + Mul<R_e31>
        + Mul<R_e23>
        + Mul<R_e021>
        + Mul<R_e013>
        + Mul<R_e032>
        + Mul<R_e123>
        + Mul<R_e0123>,
    L_e021: Copy
        + Mul<R_one>
        + Mul<R_e0>
        + Mul<R_e1>
        + Mul<R_e2>
        + Mul<R_e3>
        + Mul<R_e01>
        + Mul<R_e02>
        + Mul<R_e03>
        + Mul<R_e12>
        + Mul<R_e31>
        + Mul<R_e23>
        + Mul<R_e021>
        + Mul<R_e013>
        + Mul<R_e032>
        + Mul<R_e123>
        + Mul<R_e0123>,
    L_e013: Copy
        + Mul<R_one>
        + Mul<R_e0>
        + Mul<R_e1>
        + Mul<R_e2>
        + Mul<R_e3>
        + Mul<R_e01>
        + Mul<R_e02>
        + Mul<R_e03>
        + Mul<R_e12>
        + Mul<R_e31>
        + Mul<R_e23>
        + Mul<R_e021>
        + Mul<R_e013>
        + Mul<R_e032>
        + Mul<R_e123>
        + Mul<R_e0123>,
    L_e032: Copy
        + Mul<R_one>
        + Mul<R_e0>
        + Mul<R_e1>
        + Mul<R_e2>
        + Mul<R_e3>
        + Mul<R_e01>
        + Mul<R_e02>
        + Mul<R_e03>
        + Mul<R_e12>
        + Mul<R_e31>
        + Mul<R_e23>
        + Mul<R_e021>
        + Mul<R_e013>
        + Mul<R_e032>
        + Mul<R_e123>
        + Mul<R_e0123>,
    L_e123: Copy
        + Mul<R_one>
        + Mul<R_e0>
        + Mul<R_e1>
        + Mul<R_e2>
        + Mul<R_e3>
        + Mul<R_e01>
        + Mul<R_e02>
        + Mul<R_e03>
        + Mul<R_e12>
        + Mul<R_e31>
        + Mul<R_e23>
        + Mul<R_e021>
        + Mul<R_e013>
        + Mul<R_e032>
        + Mul<R_e123>
        + Mul<R_e0123>,
    L_e0123: Copy
        + Mul<R_one>
        + Mul<R_e0>
        + Mul<R_e1>
        + Mul<R_e2>
        + Mul<R_e3>
        + Mul<R_e01>
        + Mul<R_e02>
        + Mul<R_e03>
        + Mul<R_e12>
        + Mul<R_e31>
        + Mul<R_e23>
        + Mul<R_e021>
        + Mul<R_e013>
        + Mul<R_e032>
        + Mul<R_e123>
        + Mul<R_e0123>,
    R_one: Copy,
    R_e0: Copy,
    R_e1: Copy,
    R_e2: Copy,
    R_e3: Copy,
    R_e01: Copy,
    R_e02: Copy,
    R_e03: Copy,
    R_e12: Copy,
    R_e31: Copy,
    R_e23: Copy,
    R_e021: Copy,
    R_e013: Copy,
    R_e032: Copy,
    R_e123: Copy,
    R_e0123: Copy,

    <L_one as Mul<R_one>>::Output: Add<<L_e1 as Mul<R_e1>>::Output>,
    <<L_one as Mul<R_one>>::Output as Add<<L_e1 as Mul<R_e1>>::Output>>::Output:
        Add<<L_e2 as Mul<R_e2>>::Output>,
    <L_one as Mul<R_e0123>>::Output: Add<<L_e0 as Mul<R_e123>>::Output>,

    <<<L_one as Mul<R_one>>::Output as Add<<L_e1 as Mul<R_e1>>::Output>>::Output as Add<
        <L_e2 as Mul<R_e2>>::Output,
    >>::Output: Add<<L_e3 as Mul<R_e3>>::Output>,
    <<<<L_one as Mul<R_one>>::Output as Add<<L_e1 as Mul<R_e1>>::Output>>::Output as Add<
        <L_e2 as Mul<R_e2>>::Output,
    >>::Output as Add<<L_e3 as Mul<R_e3>>::Output>>::Output: Sub<<L_e12 as Mul<R_e12>>::Output>,

    <<<<<L_one as Mul<R_one>>::Output as Add<<L_e1 as Mul<R_e1>>::Output>>::Output as Add<
        <L_e2 as Mul<R_e2>>::Output,
    >>::Output as Add<<L_e3 as Mul<R_e3>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e12>>::Output,
    >>::Output: Sub<<L_e31 as Mul<R_e31>>::Output>,
    <<<<<<L_one as Mul<R_one>>::Output as Add<<L_e1 as Mul<R_e1>>::Output>>::Output as Add<
        <L_e2 as Mul<R_e2>>::Output,
    >>::Output as Add<<L_e3 as Mul<R_e3>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e12>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e31>>::Output>>::Output: Sub<<L_e23 as Mul<R_e23>>::Output>,
    <<<<<<<L_one as Mul<R_one>>::Output as Add<<L_e1 as Mul<R_e1>>::Output>>::Output as Add<
        <L_e2 as Mul<R_e2>>::Output,
    >>::Output as Add<<L_e3 as Mul<R_e3>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e12>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e31>>::Output>>::Output as Sub<
        <L_e23 as Mul<R_e23>>::Output,
    >>::Output: Sub<<L_e123 as Mul<R_e123>>::Output>,
    <L_one as Mul<R_e0>>::Output: Add<<L_e0 as Mul<R_one>>::Output>,
    <<L_one as Mul<R_e0>>::Output as Add<<L_e0 as Mul<R_one>>::Output>>::Output:
        Sub<<L_e1 as Mul<R_e01>>::Output>,
    <<<L_one as Mul<R_e0>>::Output as Add<<L_e0 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e01>>::Output,
    >>::Output: Sub<<L_e2 as Mul<R_e02>>::Output>,
    <<<<L_one as Mul<R_e0>>::Output as Add<<L_e0 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e01>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e02>>::Output>>::Output: Sub<<L_e3 as Mul<R_e03>>::Output>,
    <<<<<L_one as Mul<R_e0>>::Output as Add<<L_e0 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e01>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e02>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e03>>::Output,
    >>::Output: Add<<L_e01 as Mul<R_e1>>::Output>,
    <<<<<<L_one as Mul<R_e0>>::Output as Add<<L_e0 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e01>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e02>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e1>>::Output>>::Output: Add<<L_e02 as Mul<R_e2>>::Output>,
    <<<<<<<L_one as Mul<R_e0>>::Output as Add<<L_e0 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e01>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e02>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e1>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e2>>::Output,
    >>::Output: Add<<L_e03 as Mul<R_e3>>::Output>,
    <<<<<<<<L_one as Mul<R_e0>>::Output as Add<<L_e0 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e01>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e02>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e1>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e2>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e3>>::Output>>::Output: Add<<L_e12 as Mul<R_e021>>::Output>,
    <<<<<<<<<L_one as Mul<R_e0>>::Output as Add<<L_e0 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e01>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e02>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e1>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e2>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e3>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e021>>::Output,
    >>::Output: Add<<L_e31 as Mul<R_e013>>::Output>,
    <<<<<<<<<<L_one as Mul<R_e0>>::Output as Add<<L_e0 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e01>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e02>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e1>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e2>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e3>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e021>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_e013>>::Output>>::Output: Add<<L_e23 as Mul<R_e032>>::Output>,
    <<<<<<<<<<<L_one as Mul<R_e0>>::Output as Add<<L_e0 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e01>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e02>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e1>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e2>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e3>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e021>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_e013>>::Output>>::Output as Add<
        <L_e23 as Mul<R_e032>>::Output,
    >>::Output: Add<<L_e021 as Mul<R_e12>>::Output>,
    <<<<<<<<<<<<L_one as Mul<R_e0>>::Output as Add<<L_e0 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e01>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e02>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e1>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e2>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e3>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e021>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_e013>>::Output>>::Output as Add<
        <L_e23 as Mul<R_e032>>::Output,
    >>::Output as Add<<L_e021 as Mul<R_e12>>::Output>>::Output: Add<<L_e013 as Mul<R_e31>>::Output>,
    <<<<<<<<<<<<<L_one as Mul<R_e0>>::Output as Add<<L_e0 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e01>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e02>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e1>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e2>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e3>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e021>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_e013>>::Output>>::Output as Add<
        <L_e23 as Mul<R_e032>>::Output,
    >>::Output as Add<<L_e021 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e013 as Mul<R_e31>>::Output,
    >>::Output: Add<<L_e032 as Mul<R_e23>>::Output>,
    <<<<<<<<<<<<<<L_one as Mul<R_e0>>::Output as Add<<L_e0 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e01>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e02>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e1>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e2>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e3>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e021>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_e013>>::Output>>::Output as Add<
        <L_e23 as Mul<R_e032>>::Output,
    >>::Output as Add<<L_e021 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e013 as Mul<R_e31>>::Output,
    >>::Output as Add<<L_e032 as Mul<R_e23>>::Output>>::Output:
        Add<<L_e123 as Mul<R_e0123>>::Output>,
    <<<<<<<<<<<<<<<L_one as Mul<R_e0>>::Output as Add<<L_e0 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e01>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e02>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e1>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e2>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e3>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e021>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_e013>>::Output>>::Output as Add<
        <L_e23 as Mul<R_e032>>::Output,
    >>::Output as Add<<L_e021 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e013 as Mul<R_e31>>::Output,
    >>::Output as Add<<L_e032 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e123 as Mul<R_e0123>>::Output,
    >>::Output: Sub<<L_e0123 as Mul<R_e123>>::Output>,
    <L_one as Mul<R_e1>>::Output: Add<<L_e1 as Mul<R_one>>::Output>,
    <<L_one as Mul<R_e1>>::Output as Add<<L_e1 as Mul<R_one>>::Output>>::Output:
        Sub<<L_e2 as Mul<R_e12>>::Output>,
    <<<L_one as Mul<R_e1>>::Output as Add<<L_e1 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e2 as Mul<R_e12>>::Output,
    >>::Output: Add<<L_e3 as Mul<R_e31>>::Output>,
    <<<<L_one as Mul<R_e1>>::Output as Add<<L_e1 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e2 as Mul<R_e12>>::Output,
    >>::Output as Add<<L_e3 as Mul<R_e31>>::Output>>::Output: Add<<L_e12 as Mul<R_e2>>::Output>,
    <<<<<L_one as Mul<R_e1>>::Output as Add<<L_e1 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e2 as Mul<R_e12>>::Output,
    >>::Output as Add<<L_e3 as Mul<R_e31>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e2>>::Output,
    >>::Output: Sub<<L_e31 as Mul<R_e3>>::Output>,
    <<<<<<L_one as Mul<R_e1>>::Output as Add<<L_e1 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e2 as Mul<R_e12>>::Output,
    >>::Output as Add<<L_e3 as Mul<R_e31>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e2>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e3>>::Output>>::Output: Sub<<L_e23 as Mul<R_e123>>::Output>,
    <<<<<<<L_one as Mul<R_e1>>::Output as Add<<L_e1 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e2 as Mul<R_e12>>::Output,
    >>::Output as Add<<L_e3 as Mul<R_e31>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e2>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e3>>::Output>>::Output as Sub<
        <L_e23 as Mul<R_e123>>::Output,
    >>::Output: Sub<<L_e123 as Mul<R_e23>>::Output>,
    <L_one as Mul<R_e2>>::Output: Add<<L_e1 as Mul<R_e12>>::Output>,
    <<L_one as Mul<R_e2>>::Output as Add<<L_e1 as Mul<R_e12>>::Output>>::Output:
        Add<<L_e2 as Mul<R_one>>::Output>,
    <<<L_one as Mul<R_e2>>::Output as Add<<L_e1 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e2 as Mul<R_one>>::Output,
    >>::Output: Sub<<L_e3 as Mul<R_e23>>::Output>,
    <<<<L_one as Mul<R_e2>>::Output as Add<<L_e1 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e2 as Mul<R_one>>::Output,
    >>::Output as Sub<<L_e3 as Mul<R_e23>>::Output>>::Output: Sub<<L_e12 as Mul<R_e1>>::Output>,
    <<<<<L_one as Mul<R_e2>>::Output as Add<<L_e1 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e2 as Mul<R_one>>::Output,
    >>::Output as Sub<<L_e3 as Mul<R_e23>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e1>>::Output,
    >>::Output: Sub<<L_e31 as Mul<R_e123>>::Output>,
    <<<<<<L_one as Mul<R_e2>>::Output as Add<<L_e1 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e2 as Mul<R_one>>::Output,
    >>::Output as Sub<<L_e3 as Mul<R_e23>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e1>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e123>>::Output>>::Output: Add<<L_e23 as Mul<R_e3>>::Output>,
    <L_one as Mul<R_e032>>::Output: Sub<<L_e0 as Mul<R_e23>>::Output>,
    <<<<<<<L_one as Mul<R_e2>>::Output as Add<<L_e1 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e2 as Mul<R_one>>::Output,
    >>::Output as Sub<<L_e3 as Mul<R_e23>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e1>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e123>>::Output>>::Output as Add<
        <L_e23 as Mul<R_e3>>::Output,
    >>::Output: Sub<<L_e123 as Mul<R_e31>>::Output>,
    <L_one as Mul<R_e3>>::Output: Sub<<L_e1 as Mul<R_e31>>::Output>,
    <<L_one as Mul<R_e3>>::Output as Sub<<L_e1 as Mul<R_e31>>::Output>>::Output:
        Add<<L_e2 as Mul<R_e23>>::Output>,
    <<<L_one as Mul<R_e3>>::Output as Sub<<L_e1 as Mul<R_e31>>::Output>>::Output as Add<
        <L_e2 as Mul<R_e23>>::Output,
    >>::Output: Add<<L_e3 as Mul<R_one>>::Output>,
    <<<<L_one as Mul<R_e3>>::Output as Sub<<L_e1 as Mul<R_e31>>::Output>>::Output as Add<
        <L_e2 as Mul<R_e23>>::Output,
    >>::Output as Add<<L_e3 as Mul<R_one>>::Output>>::Output: Sub<<L_e12 as Mul<R_e123>>::Output>,
    <<<<<L_one as Mul<R_e3>>::Output as Sub<<L_e1 as Mul<R_e31>>::Output>>::Output as Add<
        <L_e2 as Mul<R_e23>>::Output,
    >>::Output as Add<<L_e3 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e123>>::Output,
    >>::Output: Add<<L_e31 as Mul<R_e1>>::Output>,
    <<<<<<L_one as Mul<R_e3>>::Output as Sub<<L_e1 as Mul<R_e31>>::Output>>::Output as Add<
        <L_e2 as Mul<R_e23>>::Output,
    >>::Output as Add<<L_e3 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e123>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_e1>>::Output>>::Output: Sub<<L_e23 as Mul<R_e2>>::Output>,
    <<<<<<<L_one as Mul<R_e3>>::Output as Sub<<L_e1 as Mul<R_e31>>::Output>>::Output as Add<
        <L_e2 as Mul<R_e23>>::Output,
    >>::Output as Add<<L_e3 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e123>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_e1>>::Output>>::Output as Sub<
        <L_e23 as Mul<R_e2>>::Output,
    >>::Output: Sub<<L_e123 as Mul<R_e12>>::Output>,
    <L_one as Mul<R_e01>>::Output: Add<<L_e0 as Mul<R_e1>>::Output>,
    <<L_one as Mul<R_e01>>::Output as Add<<L_e0 as Mul<R_e1>>::Output>>::Output:
        Sub<<L_e1 as Mul<R_e0>>::Output>,
    <<<L_one as Mul<R_e01>>::Output as Add<<L_e0 as Mul<R_e1>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e0>>::Output,
    >>::Output: Sub<<L_e2 as Mul<R_e021>>::Output>,
    <<<<L_one as Mul<R_e01>>::Output as Add<<L_e0 as Mul<R_e1>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e0>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e021>>::Output>>::Output: Add<<L_e3 as Mul<R_e013>>::Output>,
    <<<<<L_one as Mul<R_e01>>::Output as Add<<L_e0 as Mul<R_e1>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e0>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e021>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e013>>::Output,
    >>::Output: Add<<L_e01 as Mul<R_one>>::Output>,
    <<<<<<L_one as Mul<R_e01>>::Output as Add<<L_e0 as Mul<R_e1>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e0>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e021>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e013>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_one>>::Output>>::Output: Sub<<L_e02 as Mul<R_e12>>::Output>,
    <<<<<<<L_one as Mul<R_e01>>::Output as Add<<L_e0 as Mul<R_e1>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e0>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e021>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e013>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e02 as Mul<R_e12>>::Output,
    >>::Output: Add<<L_e03 as Mul<R_e31>>::Output>,
    <<<<<<<<L_one as Mul<R_e01>>::Output as Add<<L_e0 as Mul<R_e1>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e0>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e021>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e013>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e02 as Mul<R_e12>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e31>>::Output>>::Output: Add<<L_e12 as Mul<R_e02>>::Output>,
    <<<<<<<<<L_one as Mul<R_e01>>::Output as Add<<L_e0 as Mul<R_e1>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e0>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e021>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e013>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e02 as Mul<R_e12>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e31>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e02>>::Output,
    >>::Output: Sub<<L_e31 as Mul<R_e03>>::Output>,
    <<<<<<<<<<L_one as Mul<R_e01>>::Output as Add<<L_e0 as Mul<R_e1>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e0>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e021>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e013>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e02 as Mul<R_e12>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e31>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e02>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e03>>::Output>>::Output: Sub<<L_e23 as Mul<R_e0123>>::Output>,
    <<<<<<<<<<<L_one as Mul<R_e01>>::Output as Add<<L_e0 as Mul<R_e1>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e0>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e021>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e013>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e02 as Mul<R_e12>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e31>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e02>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e03>>::Output>>::Output as Sub<
        <L_e23 as Mul<R_e0123>>::Output,
    >>::Output: Sub<<L_e021 as Mul<R_e2>>::Output>,
    <<<<<<<<<<<<L_one as Mul<R_e01>>::Output as Add<<L_e0 as Mul<R_e1>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e0>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e021>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e013>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e02 as Mul<R_e12>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e31>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e02>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e03>>::Output>>::Output as Sub<
        <L_e23 as Mul<R_e0123>>::Output,
    >>::Output as Sub<<L_e021 as Mul<R_e2>>::Output>>::Output: Add<<L_e013 as Mul<R_e3>>::Output>,
    <<<<<<<<<<<<<L_one as Mul<R_e01>>::Output as Add<<L_e0 as Mul<R_e1>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e0>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e021>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e013>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e02 as Mul<R_e12>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e31>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e02>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e03>>::Output>>::Output as Sub<
        <L_e23 as Mul<R_e0123>>::Output,
    >>::Output as Sub<<L_e021 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e013 as Mul<R_e3>>::Output,
    >>::Output: Add<<L_e032 as Mul<R_e123>>::Output>,
    <<<<<<<<<<<<<<L_one as Mul<R_e01>>::Output as Add<<L_e0 as Mul<R_e1>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e0>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e021>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e013>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e02 as Mul<R_e12>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e31>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e02>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e03>>::Output>>::Output as Sub<
        <L_e23 as Mul<R_e0123>>::Output,
    >>::Output as Sub<<L_e021 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e013 as Mul<R_e3>>::Output,
    >>::Output as Add<<L_e032 as Mul<R_e123>>::Output>>::Output:
        Sub<<L_e123 as Mul<R_e032>>::Output>,
    <<<<<<<<<<<<<<<L_one as Mul<R_e01>>::Output as Add<<L_e0 as Mul<R_e1>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e0>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e021>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e013>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e02 as Mul<R_e12>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e31>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e02>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e03>>::Output>>::Output as Sub<
        <L_e23 as Mul<R_e0123>>::Output,
    >>::Output as Sub<<L_e021 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e013 as Mul<R_e3>>::Output,
    >>::Output as Add<<L_e032 as Mul<R_e123>>::Output>>::Output as Sub<
        <L_e123 as Mul<R_e032>>::Output,
    >>::Output: Sub<<L_e0123 as Mul<R_e23>>::Output>,
    <L_one as Mul<R_e02>>::Output: Add<<L_e0 as Mul<R_e2>>::Output>,
    <<L_one as Mul<R_e02>>::Output as Add<<L_e0 as Mul<R_e2>>::Output>>::Output:
        Add<<L_e1 as Mul<R_e021>>::Output>,
    <<<L_one as Mul<R_e02>>::Output as Add<<L_e0 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e021>>::Output,
    >>::Output: Sub<<L_e2 as Mul<R_e0>>::Output>,
    <<<<L_one as Mul<R_e02>>::Output as Add<<L_e0 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e021>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e0>>::Output>>::Output: Sub<<L_e3 as Mul<R_e032>>::Output>,
    <<<<<L_one as Mul<R_e02>>::Output as Add<<L_e0 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e021>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e0>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e032>>::Output,
    >>::Output: Add<<L_e01 as Mul<R_e12>>::Output>,
    <<<<<<L_one as Mul<R_e02>>::Output as Add<<L_e0 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e021>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e0>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e032>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e12>>::Output>>::Output: Add<<L_e02 as Mul<R_one>>::Output>,
    <<<<<<<L_one as Mul<R_e02>>::Output as Add<<L_e0 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e021>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e0>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e032>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e02 as Mul<R_one>>::Output,
    >>::Output: Sub<<L_e03 as Mul<R_e23>>::Output>,
    <<<<<<<<L_one as Mul<R_e02>>::Output as Add<<L_e0 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e021>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e0>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e032>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e02 as Mul<R_one>>::Output,
    >>::Output as Sub<<L_e03 as Mul<R_e23>>::Output>>::Output: Sub<<L_e12 as Mul<R_e01>>::Output>,
    <<<<<<<<<L_one as Mul<R_e02>>::Output as Add<<L_e0 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e021>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e0>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e032>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e02 as Mul<R_one>>::Output,
    >>::Output as Sub<<L_e03 as Mul<R_e23>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e01>>::Output,
    >>::Output: Sub<<L_e31 as Mul<R_e0123>>::Output>,
    <<<<<<<<<<L_one as Mul<R_e02>>::Output as Add<<L_e0 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e021>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e0>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e032>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e02 as Mul<R_one>>::Output,
    >>::Output as Sub<<L_e03 as Mul<R_e23>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e01>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e0123>>::Output>>::Output: Add<<L_e23 as Mul<R_e03>>::Output>,
    <<<<<<<<<<<L_one as Mul<R_e02>>::Output as Add<<L_e0 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e021>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e0>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e032>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e02 as Mul<R_one>>::Output,
    >>::Output as Sub<<L_e03 as Mul<R_e23>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e01>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e0123>>::Output>>::Output as Add<
        <L_e23 as Mul<R_e03>>::Output,
    >>::Output: Add<<L_e021 as Mul<R_e1>>::Output>,
    <<<<<<<<<<<<L_one as Mul<R_e02>>::Output as Add<<L_e0 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e021>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e0>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e032>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e02 as Mul<R_one>>::Output,
    >>::Output as Sub<<L_e03 as Mul<R_e23>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e01>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e0123>>::Output>>::Output as Add<
        <L_e23 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e021 as Mul<R_e1>>::Output>>::Output: Add<<L_e013 as Mul<R_e123>>::Output>,
    <<<<<<<<<<<<<L_one as Mul<R_e02>>::Output as Add<<L_e0 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e021>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e0>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e032>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e02 as Mul<R_one>>::Output,
    >>::Output as Sub<<L_e03 as Mul<R_e23>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e01>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e0123>>::Output>>::Output as Add<
        <L_e23 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e021 as Mul<R_e1>>::Output>>::Output as Add<
        <L_e013 as Mul<R_e123>>::Output,
    >>::Output: Sub<<L_e032 as Mul<R_e3>>::Output>,
    <<<<<<<<<<<<<<L_one as Mul<R_e02>>::Output as Add<<L_e0 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e021>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e0>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e032>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e02 as Mul<R_one>>::Output,
    >>::Output as Sub<<L_e03 as Mul<R_e23>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e01>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e0123>>::Output>>::Output as Add<
        <L_e23 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e021 as Mul<R_e1>>::Output>>::Output as Add<
        <L_e013 as Mul<R_e123>>::Output,
    >>::Output as Sub<<L_e032 as Mul<R_e3>>::Output>>::Output: Sub<<L_e123 as Mul<R_e013>>::Output>,
    <<<<<<<<<<<<<<<L_one as Mul<R_e02>>::Output as Add<<L_e0 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e021>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e0>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e032>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e02 as Mul<R_one>>::Output,
    >>::Output as Sub<<L_e03 as Mul<R_e23>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e01>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e0123>>::Output>>::Output as Add<
        <L_e23 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e021 as Mul<R_e1>>::Output>>::Output as Add<
        <L_e013 as Mul<R_e123>>::Output,
    >>::Output as Sub<<L_e032 as Mul<R_e3>>::Output>>::Output as Sub<
        <L_e123 as Mul<R_e013>>::Output,
    >>::Output: Sub<<L_e0123 as Mul<R_e31>>::Output>,
    <L_one as Mul<R_e03>>::Output: Add<<L_e0 as Mul<R_e3>>::Output>,
    <<L_one as Mul<R_e03>>::Output as Add<<L_e0 as Mul<R_e3>>::Output>>::Output:
        Sub<<L_e1 as Mul<R_e013>>::Output>,
    <<<L_one as Mul<R_e03>>::Output as Add<<L_e0 as Mul<R_e3>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e013>>::Output,
    >>::Output: Add<<L_e2 as Mul<R_e032>>::Output>,
    <<<<L_one as Mul<R_e03>>::Output as Add<<L_e0 as Mul<R_e3>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e013>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e032>>::Output>>::Output: Sub<<L_e3 as Mul<R_e0>>::Output>,
    <<<<<L_one as Mul<R_e03>>::Output as Add<<L_e0 as Mul<R_e3>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e013>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e032>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e0>>::Output,
    >>::Output: Sub<<L_e01 as Mul<R_e31>>::Output>,
    <<<<<<L_one as Mul<R_e03>>::Output as Add<<L_e0 as Mul<R_e3>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e013>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e032>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e0>>::Output,
    >>::Output as Sub<<L_e01 as Mul<R_e31>>::Output>>::Output: Add<<L_e02 as Mul<R_e23>>::Output>,
    <<<<<<<L_one as Mul<R_e03>>::Output as Add<<L_e0 as Mul<R_e3>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e013>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e032>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e0>>::Output,
    >>::Output as Sub<<L_e01 as Mul<R_e31>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e23>>::Output,
    >>::Output: Add<<L_e03 as Mul<R_one>>::Output>,
    <<<<<<<<L_one as Mul<R_e03>>::Output as Add<<L_e0 as Mul<R_e3>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e013>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e032>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e0>>::Output,
    >>::Output as Sub<<L_e01 as Mul<R_e31>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e23>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_one>>::Output>>::Output: Sub<<L_e12 as Mul<R_e0123>>::Output>,
    <<<<<<<<<L_one as Mul<R_e03>>::Output as Add<<L_e0 as Mul<R_e3>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e013>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e032>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e0>>::Output,
    >>::Output as Sub<<L_e01 as Mul<R_e31>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e23>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e0123>>::Output,
    >>::Output: Add<<L_e31 as Mul<R_e01>>::Output>,
    <<<<<<<<<<L_one as Mul<R_e03>>::Output as Add<<L_e0 as Mul<R_e3>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e013>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e032>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e0>>::Output,
    >>::Output as Sub<<L_e01 as Mul<R_e31>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e23>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e0123>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_e01>>::Output>>::Output: Sub<<L_e23 as Mul<R_e02>>::Output>,
    <<<<<<<<<<<L_one as Mul<R_e03>>::Output as Add<<L_e0 as Mul<R_e3>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e013>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e032>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e0>>::Output,
    >>::Output as Sub<<L_e01 as Mul<R_e31>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e23>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e0123>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_e01>>::Output>>::Output as Sub<
        <L_e23 as Mul<R_e02>>::Output,
    >>::Output: Add<<L_e021 as Mul<R_e123>>::Output>,
    <<<<<<<<<<<<L_one as Mul<R_e03>>::Output as Add<<L_e0 as Mul<R_e3>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e013>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e032>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e0>>::Output,
    >>::Output as Sub<<L_e01 as Mul<R_e31>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e23>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e0123>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_e01>>::Output>>::Output as Sub<
        <L_e23 as Mul<R_e02>>::Output,
    >>::Output as Add<<L_e021 as Mul<R_e123>>::Output>>::Output: Sub<<L_e013 as Mul<R_e1>>::Output>,
    <<<<<<<<<<<<<L_one as Mul<R_e03>>::Output as Add<<L_e0 as Mul<R_e3>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e013>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e032>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e0>>::Output,
    >>::Output as Sub<<L_e01 as Mul<R_e31>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e23>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e0123>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_e01>>::Output>>::Output as Sub<
        <L_e23 as Mul<R_e02>>::Output,
    >>::Output as Add<<L_e021 as Mul<R_e123>>::Output>>::Output as Sub<
        <L_e013 as Mul<R_e1>>::Output,
    >>::Output: Add<<L_e032 as Mul<R_e2>>::Output>,
    <<<<<<<<<<<<<<L_one as Mul<R_e03>>::Output as Add<<L_e0 as Mul<R_e3>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e013>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e032>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e0>>::Output,
    >>::Output as Sub<<L_e01 as Mul<R_e31>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e23>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e0123>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_e01>>::Output>>::Output as Sub<
        <L_e23 as Mul<R_e02>>::Output,
    >>::Output as Add<<L_e021 as Mul<R_e123>>::Output>>::Output as Sub<
        <L_e013 as Mul<R_e1>>::Output,
    >>::Output as Add<<L_e032 as Mul<R_e2>>::Output>>::Output: Sub<<L_e123 as Mul<R_e021>>::Output>,
    <<<<<<<<<<<<<<<L_one as Mul<R_e03>>::Output as Add<<L_e0 as Mul<R_e3>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e013>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e032>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e0>>::Output,
    >>::Output as Sub<<L_e01 as Mul<R_e31>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e23>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_one>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e0123>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_e01>>::Output>>::Output as Sub<
        <L_e23 as Mul<R_e02>>::Output,
    >>::Output as Add<<L_e021 as Mul<R_e123>>::Output>>::Output as Sub<
        <L_e013 as Mul<R_e1>>::Output,
    >>::Output as Add<<L_e032 as Mul<R_e2>>::Output>>::Output as Sub<
        <L_e123 as Mul<R_e021>>::Output,
    >>::Output: Sub<<L_e0123 as Mul<R_e12>>::Output>,
    <L_one as Mul<R_e12>>::Output: Add<<L_e1 as Mul<R_e2>>::Output>,
    <<L_one as Mul<R_e12>>::Output as Add<<L_e1 as Mul<R_e2>>::Output>>::Output:
        Sub<<L_e2 as Mul<R_e1>>::Output>,
    <<<L_one as Mul<R_e12>>::Output as Add<<L_e1 as Mul<R_e2>>::Output>>::Output as Sub<
        <L_e2 as Mul<R_e1>>::Output,
    >>::Output: Add<<L_e3 as Mul<R_e123>>::Output>,
    <<<<L_one as Mul<R_e12>>::Output as Add<<L_e1 as Mul<R_e2>>::Output>>::Output as Sub<
        <L_e2 as Mul<R_e1>>::Output,
    >>::Output as Add<<L_e3 as Mul<R_e123>>::Output>>::Output: Add<<L_e12 as Mul<R_one>>::Output>,
    <<<<<L_one as Mul<R_e12>>::Output as Add<<L_e1 as Mul<R_e2>>::Output>>::Output as Sub<
        <L_e2 as Mul<R_e1>>::Output,
    >>::Output as Add<<L_e3 as Mul<R_e123>>::Output>>::Output as Add<
        <L_e12 as Mul<R_one>>::Output,
    >>::Output: Add<<L_e31 as Mul<R_e23>>::Output>,
    <<<<<<L_one as Mul<R_e12>>::Output as Add<<L_e1 as Mul<R_e2>>::Output>>::Output as Sub<
        <L_e2 as Mul<R_e1>>::Output,
    >>::Output as Add<<L_e3 as Mul<R_e123>>::Output>>::Output as Add<
        <L_e12 as Mul<R_one>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_e23>>::Output>>::Output: Sub<<L_e23 as Mul<R_e31>>::Output>,
    <<<<<<<L_one as Mul<R_e12>>::Output as Add<<L_e1 as Mul<R_e2>>::Output>>::Output as Sub<
        <L_e2 as Mul<R_e1>>::Output,
    >>::Output as Add<<L_e3 as Mul<R_e123>>::Output>>::Output as Add<
        <L_e12 as Mul<R_one>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_e23>>::Output>>::Output as Sub<
        <L_e23 as Mul<R_e31>>::Output,
    >>::Output: Add<<L_e123 as Mul<R_e3>>::Output>,
    <L_one as Mul<R_e31>>::Output: Sub<<L_e1 as Mul<R_e3>>::Output>,
    <<L_one as Mul<R_e31>>::Output as Sub<<L_e1 as Mul<R_e3>>::Output>>::Output:
        Add<<L_e2 as Mul<R_e123>>::Output>,
    <<<L_one as Mul<R_e31>>::Output as Sub<<L_e1 as Mul<R_e3>>::Output>>::Output as Add<
        <L_e2 as Mul<R_e123>>::Output,
    >>::Output: Add<<L_e3 as Mul<R_e1>>::Output>,
    <<<<L_one as Mul<R_e31>>::Output as Sub<<L_e1 as Mul<R_e3>>::Output>>::Output as Add<
        <L_e2 as Mul<R_e123>>::Output,
    >>::Output as Add<<L_e3 as Mul<R_e1>>::Output>>::Output: Sub<<L_e12 as Mul<R_e23>>::Output>,
    <<<<<L_one as Mul<R_e31>>::Output as Sub<<L_e1 as Mul<R_e3>>::Output>>::Output as Add<
        <L_e2 as Mul<R_e123>>::Output,
    >>::Output as Add<<L_e3 as Mul<R_e1>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e23>>::Output,
    >>::Output: Add<<L_e31 as Mul<R_one>>::Output>,
    <<<<<<L_one as Mul<R_e31>>::Output as Sub<<L_e1 as Mul<R_e3>>::Output>>::Output as Add<
        <L_e2 as Mul<R_e123>>::Output,
    >>::Output as Add<<L_e3 as Mul<R_e1>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e23>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_one>>::Output>>::Output: Add<<L_e23 as Mul<R_e12>>::Output>,
    <<<<<<<L_one as Mul<R_e31>>::Output as Sub<<L_e1 as Mul<R_e3>>::Output>>::Output as Add<
        <L_e2 as Mul<R_e123>>::Output,
    >>::Output as Add<<L_e3 as Mul<R_e1>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e23>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_one>>::Output>>::Output as Add<
        <L_e23 as Mul<R_e12>>::Output,
    >>::Output: Add<<L_e123 as Mul<R_e2>>::Output>,
    <L_one as Mul<R_e23>>::Output: Add<<L_e1 as Mul<R_e123>>::Output>,
    <<L_one as Mul<R_e23>>::Output as Add<<L_e1 as Mul<R_e123>>::Output>>::Output:
        Add<<L_e2 as Mul<R_e3>>::Output>,
    <<<L_one as Mul<R_e23>>::Output as Add<<L_e1 as Mul<R_e123>>::Output>>::Output as Add<
        <L_e2 as Mul<R_e3>>::Output,
    >>::Output: Sub<<L_e3 as Mul<R_e2>>::Output>,
    <<<<L_one as Mul<R_e23>>::Output as Add<<L_e1 as Mul<R_e123>>::Output>>::Output as Add<
        <L_e2 as Mul<R_e3>>::Output,
    >>::Output as Sub<<L_e3 as Mul<R_e2>>::Output>>::Output: Add<<L_e12 as Mul<R_e31>>::Output>,
    <<<<<L_one as Mul<R_e23>>::Output as Add<<L_e1 as Mul<R_e123>>::Output>>::Output as Add<
        <L_e2 as Mul<R_e3>>::Output,
    >>::Output as Sub<<L_e3 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e31>>::Output,
    >>::Output: Sub<<L_e31 as Mul<R_e12>>::Output>,
    <<<<<<L_one as Mul<R_e23>>::Output as Add<<L_e1 as Mul<R_e123>>::Output>>::Output as Add<
        <L_e2 as Mul<R_e3>>::Output,
    >>::Output as Sub<<L_e3 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e31>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e12>>::Output>>::Output: Add<<L_e23 as Mul<R_one>>::Output>,
    <<<<<<<L_one as Mul<R_e23>>::Output as Add<<L_e1 as Mul<R_e123>>::Output>>::Output as Add<
        <L_e2 as Mul<R_e3>>::Output,
    >>::Output as Sub<<L_e3 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e31>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e23 as Mul<R_one>>::Output,
    >>::Output: Add<<L_e123 as Mul<R_e1>>::Output>,
    <L_one as Mul<R_e021>>::Output: Sub<<L_e0 as Mul<R_e12>>::Output>,
    <<L_one as Mul<R_e021>>::Output as Sub<<L_e0 as Mul<R_e12>>::Output>>::Output:
        Add<<L_e1 as Mul<R_e02>>::Output>,
    <<<L_one as Mul<R_e021>>::Output as Sub<<L_e0 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e02>>::Output,
    >>::Output: Sub<<L_e2 as Mul<R_e01>>::Output>,
    <<<<L_one as Mul<R_e021>>::Output as Sub<<L_e0 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e02>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e01>>::Output>>::Output: Add<<L_e3 as Mul<R_e0123>>::Output>,
    <<<<<L_one as Mul<R_e021>>::Output as Sub<<L_e0 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e02>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e01>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e0123>>::Output,
    >>::Output: Sub<<L_e01 as Mul<R_e2>>::Output>,
    <<<<<<L_one as Mul<R_e021>>::Output as Sub<<L_e0 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e02>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e01>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e0123>>::Output,
    >>::Output as Sub<<L_e01 as Mul<R_e2>>::Output>>::Output: Add<<L_e02 as Mul<R_e1>>::Output>,
    <<<<<<<L_one as Mul<R_e021>>::Output as Sub<<L_e0 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e02>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e01>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e0123>>::Output,
    >>::Output as Sub<<L_e01 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e1>>::Output,
    >>::Output: Sub<<L_e03 as Mul<R_e123>>::Output>,
    <<<<<<<<L_one as Mul<R_e021>>::Output as Sub<<L_e0 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e02>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e01>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e0123>>::Output,
    >>::Output as Sub<<L_e01 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e1>>::Output,
    >>::Output as Sub<<L_e03 as Mul<R_e123>>::Output>>::Output: Sub<<L_e12 as Mul<R_e0>>::Output>,
    <<<<<<<<<L_one as Mul<R_e021>>::Output as Sub<<L_e0 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e02>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e01>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e0123>>::Output,
    >>::Output as Sub<<L_e01 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e1>>::Output,
    >>::Output as Sub<<L_e03 as Mul<R_e123>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e0>>::Output,
    >>::Output: Add<<L_e31 as Mul<R_e032>>::Output>,
    <<<<<<<<<<L_one as Mul<R_e021>>::Output as Sub<<L_e0 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e02>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e01>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e0123>>::Output,
    >>::Output as Sub<<L_e01 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e1>>::Output,
    >>::Output as Sub<<L_e03 as Mul<R_e123>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e0>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_e032>>::Output>>::Output: Sub<<L_e23 as Mul<R_e013>>::Output>,
    <<<<<<<<<<<L_one as Mul<R_e021>>::Output as Sub<<L_e0 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e02>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e01>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e0123>>::Output,
    >>::Output as Sub<<L_e01 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e1>>::Output,
    >>::Output as Sub<<L_e03 as Mul<R_e123>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e0>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_e032>>::Output>>::Output as Sub<
        <L_e23 as Mul<R_e013>>::Output,
    >>::Output: Add<<L_e021 as Mul<R_one>>::Output>,
    <<<<<<<<<<<<L_one as Mul<R_e021>>::Output as Sub<<L_e0 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e02>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e01>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e0123>>::Output,
    >>::Output as Sub<<L_e01 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e1>>::Output,
    >>::Output as Sub<<L_e03 as Mul<R_e123>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e0>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_e032>>::Output>>::Output as Sub<
        <L_e23 as Mul<R_e013>>::Output,
    >>::Output as Add<<L_e021 as Mul<R_one>>::Output>>::Output: Add<<L_e013 as Mul<R_e23>>::Output>,
    <<<<<<<<<<<<<L_one as Mul<R_e021>>::Output as Sub<<L_e0 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e02>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e01>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e0123>>::Output,
    >>::Output as Sub<<L_e01 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e1>>::Output,
    >>::Output as Sub<<L_e03 as Mul<R_e123>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e0>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_e032>>::Output>>::Output as Sub<
        <L_e23 as Mul<R_e013>>::Output,
    >>::Output as Add<<L_e021 as Mul<R_one>>::Output>>::Output as Add<
        <L_e013 as Mul<R_e23>>::Output,
    >>::Output: Sub<<L_e032 as Mul<R_e31>>::Output>,
    <<<<<<<<<<<<<<L_one as Mul<R_e021>>::Output as Sub<<L_e0 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e02>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e01>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e0123>>::Output,
    >>::Output as Sub<<L_e01 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e1>>::Output,
    >>::Output as Sub<<L_e03 as Mul<R_e123>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e0>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_e032>>::Output>>::Output as Sub<
        <L_e23 as Mul<R_e013>>::Output,
    >>::Output as Add<<L_e021 as Mul<R_one>>::Output>>::Output as Add<
        <L_e013 as Mul<R_e23>>::Output,
    >>::Output as Sub<<L_e032 as Mul<R_e31>>::Output>>::Output: Add<<L_e123 as Mul<R_e03>>::Output>,
    <<<<<<<<<<<<<<<L_one as Mul<R_e021>>::Output as Sub<<L_e0 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e02>>::Output,
    >>::Output as Sub<<L_e2 as Mul<R_e01>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e0123>>::Output,
    >>::Output as Sub<<L_e01 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e1>>::Output,
    >>::Output as Sub<<L_e03 as Mul<R_e123>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e0>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_e032>>::Output>>::Output as Sub<
        <L_e23 as Mul<R_e013>>::Output,
    >>::Output as Add<<L_e021 as Mul<R_one>>::Output>>::Output as Add<
        <L_e013 as Mul<R_e23>>::Output,
    >>::Output as Sub<<L_e032 as Mul<R_e31>>::Output>>::Output as Add<
        <L_e123 as Mul<R_e03>>::Output,
    >>::Output: Sub<<L_e0123 as Mul<R_e3>>::Output>,
    <L_one as Mul<R_e013>>::Output: Sub<<L_e0 as Mul<R_e31>>::Output>,
    <<L_one as Mul<R_e013>>::Output as Sub<<L_e0 as Mul<R_e31>>::Output>>::Output:
        Sub<<L_e1 as Mul<R_e03>>::Output>,
    <<<L_one as Mul<R_e013>>::Output as Sub<<L_e0 as Mul<R_e31>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e03>>::Output,
    >>::Output: Add<<L_e2 as Mul<R_e0123>>::Output>,
    <<<<L_one as Mul<R_e013>>::Output as Sub<<L_e0 as Mul<R_e31>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e0123>>::Output>>::Output: Add<<L_e3 as Mul<R_e01>>::Output>,
    <<<<<L_one as Mul<R_e013>>::Output as Sub<<L_e0 as Mul<R_e31>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e0123>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e01>>::Output,
    >>::Output: Add<<L_e01 as Mul<R_e3>>::Output>,
    <<<<<<L_one as Mul<R_e013>>::Output as Sub<<L_e0 as Mul<R_e31>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e0123>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e01>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e3>>::Output>>::Output: Sub<<L_e02 as Mul<R_e123>>::Output>,
    <<<<<<<L_one as Mul<R_e013>>::Output as Sub<<L_e0 as Mul<R_e31>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e0123>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e01>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e3>>::Output>>::Output as Sub<
        <L_e02 as Mul<R_e123>>::Output,
    >>::Output: Sub<<L_e03 as Mul<R_e1>>::Output>,
    <<<<<<<<L_one as Mul<R_e013>>::Output as Sub<<L_e0 as Mul<R_e31>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e0123>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e01>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e3>>::Output>>::Output as Sub<
        <L_e02 as Mul<R_e123>>::Output,
    >>::Output as Sub<<L_e03 as Mul<R_e1>>::Output>>::Output: Sub<<L_e12 as Mul<R_e032>>::Output>,
    <<<<<<<<<L_one as Mul<R_e013>>::Output as Sub<<L_e0 as Mul<R_e31>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e0123>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e01>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e3>>::Output>>::Output as Sub<
        <L_e02 as Mul<R_e123>>::Output,
    >>::Output as Sub<<L_e03 as Mul<R_e1>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e032>>::Output,
    >>::Output: Sub<<L_e31 as Mul<R_e0>>::Output>,
    <<<<<<<<<<L_one as Mul<R_e013>>::Output as Sub<<L_e0 as Mul<R_e31>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e0123>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e01>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e3>>::Output>>::Output as Sub<
        <L_e02 as Mul<R_e123>>::Output,
    >>::Output as Sub<<L_e03 as Mul<R_e1>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e032>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e0>>::Output>>::Output: Add<<L_e23 as Mul<R_e021>>::Output>,
    <<<<<<<<<<<L_one as Mul<R_e013>>::Output as Sub<<L_e0 as Mul<R_e31>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e0123>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e01>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e3>>::Output>>::Output as Sub<
        <L_e02 as Mul<R_e123>>::Output,
    >>::Output as Sub<<L_e03 as Mul<R_e1>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e032>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e0>>::Output>>::Output as Add<
        <L_e23 as Mul<R_e021>>::Output,
    >>::Output: Sub<<L_e021 as Mul<R_e23>>::Output>,
    <<<<<<<<<<<<L_one as Mul<R_e013>>::Output as Sub<<L_e0 as Mul<R_e31>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e0123>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e01>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e3>>::Output>>::Output as Sub<
        <L_e02 as Mul<R_e123>>::Output,
    >>::Output as Sub<<L_e03 as Mul<R_e1>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e032>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e0>>::Output>>::Output as Add<
        <L_e23 as Mul<R_e021>>::Output,
    >>::Output as Sub<<L_e021 as Mul<R_e23>>::Output>>::Output: Add<<L_e013 as Mul<R_one>>::Output>,
    <<<<<<<<<<<<<L_one as Mul<R_e013>>::Output as Sub<<L_e0 as Mul<R_e31>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e0123>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e01>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e3>>::Output>>::Output as Sub<
        <L_e02 as Mul<R_e123>>::Output,
    >>::Output as Sub<<L_e03 as Mul<R_e1>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e032>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e0>>::Output>>::Output as Add<
        <L_e23 as Mul<R_e021>>::Output,
    >>::Output as Sub<<L_e021 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e013 as Mul<R_one>>::Output,
    >>::Output: Add<<L_e032 as Mul<R_e12>>::Output>,
    <<<<<<<<<<<<<<L_one as Mul<R_e013>>::Output as Sub<<L_e0 as Mul<R_e31>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e0123>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e01>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e3>>::Output>>::Output as Sub<
        <L_e02 as Mul<R_e123>>::Output,
    >>::Output as Sub<<L_e03 as Mul<R_e1>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e032>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e0>>::Output>>::Output as Add<
        <L_e23 as Mul<R_e021>>::Output,
    >>::Output as Sub<<L_e021 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e013 as Mul<R_one>>::Output,
    >>::Output as Add<<L_e032 as Mul<R_e12>>::Output>>::Output: Add<<L_e123 as Mul<R_e02>>::Output>,
    <<<<<<<<<<<<<<<L_one as Mul<R_e013>>::Output as Sub<<L_e0 as Mul<R_e31>>::Output>>::Output as Sub<
        <L_e1 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e0123>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e01>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e3>>::Output>>::Output as Sub<
        <L_e02 as Mul<R_e123>>::Output,
    >>::Output as Sub<<L_e03 as Mul<R_e1>>::Output>>::Output as Sub<
        <L_e12 as Mul<R_e032>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e0>>::Output>>::Output as Add<
        <L_e23 as Mul<R_e021>>::Output,
    >>::Output as Sub<<L_e021 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e013 as Mul<R_one>>::Output,
    >>::Output as Add<<L_e032 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e123 as Mul<R_e02>>::Output,
    >>::Output: Sub<<L_e0123 as Mul<R_e2>>::Output>,
    <<L_one as Mul<R_e032>>::Output as Sub<<L_e0 as Mul<R_e23>>::Output>>::Output:
        Add<<L_e1 as Mul<R_e0123>>::Output>,
    <<<L_one as Mul<R_e032>>::Output as Sub<<L_e0 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e0123>>::Output,
    >>::Output: Add<<L_e2 as Mul<R_e03>>::Output>,
    <<<<L_one as Mul<R_e032>>::Output as Sub<<L_e0 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e0123>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e03>>::Output>>::Output: Sub<<L_e3 as Mul<R_e02>>::Output>,
    <<<<<L_one as Mul<R_e032>>::Output as Sub<<L_e0 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e0123>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e03>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e02>>::Output,
    >>::Output: Sub<<L_e01 as Mul<R_e123>>::Output>,
    <<<<<<L_one as Mul<R_e032>>::Output as Sub<<L_e0 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e0123>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e03>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e02>>::Output,
    >>::Output as Sub<<L_e01 as Mul<R_e123>>::Output>>::Output: Sub<<L_e02 as Mul<R_e3>>::Output>,
    <<<<<<<L_one as Mul<R_e032>>::Output as Sub<<L_e0 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e0123>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e03>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e02>>::Output,
    >>::Output as Sub<<L_e01 as Mul<R_e123>>::Output>>::Output as Sub<
        <L_e02 as Mul<R_e3>>::Output,
    >>::Output: Add<<L_e03 as Mul<R_e2>>::Output>,
    <<<<<<<<L_one as Mul<R_e032>>::Output as Sub<<L_e0 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e0123>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e03>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e02>>::Output,
    >>::Output as Sub<<L_e01 as Mul<R_e123>>::Output>>::Output as Sub<
        <L_e02 as Mul<R_e3>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e2>>::Output>>::Output: Add<<L_e12 as Mul<R_e013>>::Output>,
    <<<<<<<<<L_one as Mul<R_e032>>::Output as Sub<<L_e0 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e0123>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e03>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e02>>::Output,
    >>::Output as Sub<<L_e01 as Mul<R_e123>>::Output>>::Output as Sub<
        <L_e02 as Mul<R_e3>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e013>>::Output,
    >>::Output: Sub<<L_e31 as Mul<R_e021>>::Output>,
    <<<<<<<<<<L_one as Mul<R_e032>>::Output as Sub<<L_e0 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e0123>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e03>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e02>>::Output,
    >>::Output as Sub<<L_e01 as Mul<R_e123>>::Output>>::Output as Sub<
        <L_e02 as Mul<R_e3>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e013>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e021>>::Output>>::Output: Sub<<L_e23 as Mul<R_e0>>::Output>,
    <<<<<<<<<<<L_one as Mul<R_e032>>::Output as Sub<<L_e0 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e0123>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e03>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e02>>::Output,
    >>::Output as Sub<<L_e01 as Mul<R_e123>>::Output>>::Output as Sub<
        <L_e02 as Mul<R_e3>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e013>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e021>>::Output>>::Output as Sub<
        <L_e23 as Mul<R_e0>>::Output,
    >>::Output: Add<<L_e021 as Mul<R_e31>>::Output>,
    <<<<<<<<<<<<L_one as Mul<R_e032>>::Output as Sub<<L_e0 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e0123>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e03>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e02>>::Output,
    >>::Output as Sub<<L_e01 as Mul<R_e123>>::Output>>::Output as Sub<
        <L_e02 as Mul<R_e3>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e013>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e021>>::Output>>::Output as Sub<
        <L_e23 as Mul<R_e0>>::Output,
    >>::Output as Add<<L_e021 as Mul<R_e31>>::Output>>::Output: Sub<<L_e013 as Mul<R_e12>>::Output>,
    <<<<<<<<<<<<<L_one as Mul<R_e032>>::Output as Sub<<L_e0 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e0123>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e03>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e02>>::Output,
    >>::Output as Sub<<L_e01 as Mul<R_e123>>::Output>>::Output as Sub<
        <L_e02 as Mul<R_e3>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e013>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e021>>::Output>>::Output as Sub<
        <L_e23 as Mul<R_e0>>::Output,
    >>::Output as Add<<L_e021 as Mul<R_e31>>::Output>>::Output as Sub<
        <L_e013 as Mul<R_e12>>::Output,
    >>::Output: Add<<L_e032 as Mul<R_one>>::Output>,
    <<<<<<<<<<<<<<L_one as Mul<R_e032>>::Output as Sub<<L_e0 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e0123>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e03>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e02>>::Output,
    >>::Output as Sub<<L_e01 as Mul<R_e123>>::Output>>::Output as Sub<
        <L_e02 as Mul<R_e3>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e013>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e021>>::Output>>::Output as Sub<
        <L_e23 as Mul<R_e0>>::Output,
    >>::Output as Add<<L_e021 as Mul<R_e31>>::Output>>::Output as Sub<
        <L_e013 as Mul<R_e12>>::Output,
    >>::Output as Add<<L_e032 as Mul<R_one>>::Output>>::Output: Add<<L_e123 as Mul<R_e01>>::Output>,
    <<<<<<<<<<<<<<<L_one as Mul<R_e032>>::Output as Sub<<L_e0 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e0123>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e03>>::Output>>::Output as Sub<
        <L_e3 as Mul<R_e02>>::Output,
    >>::Output as Sub<<L_e01 as Mul<R_e123>>::Output>>::Output as Sub<
        <L_e02 as Mul<R_e3>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e013>>::Output,
    >>::Output as Sub<<L_e31 as Mul<R_e021>>::Output>>::Output as Sub<
        <L_e23 as Mul<R_e0>>::Output,
    >>::Output as Add<<L_e021 as Mul<R_e31>>::Output>>::Output as Sub<
        <L_e013 as Mul<R_e12>>::Output,
    >>::Output as Add<<L_e032 as Mul<R_one>>::Output>>::Output as Add<
        <L_e123 as Mul<R_e01>>::Output,
    >>::Output: Sub<<L_e0123 as Mul<R_e1>>::Output>,
    <L_one as Mul<R_e123>>::Output: Add<<L_e1 as Mul<R_e23>>::Output>,
    <<L_one as Mul<R_e123>>::Output as Add<<L_e1 as Mul<R_e23>>::Output>>::Output:
        Add<<L_e2 as Mul<R_e31>>::Output>,
    <<<L_one as Mul<R_e123>>::Output as Add<<L_e1 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e2 as Mul<R_e31>>::Output,
    >>::Output: Add<<L_e3 as Mul<R_e12>>::Output>,
    <<<<L_one as Mul<R_e123>>::Output as Add<<L_e1 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e2 as Mul<R_e31>>::Output,
    >>::Output as Add<<L_e3 as Mul<R_e12>>::Output>>::Output: Add<<L_e12 as Mul<R_e3>>::Output>,
    <<<<<L_one as Mul<R_e123>>::Output as Add<<L_e1 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e2 as Mul<R_e31>>::Output,
    >>::Output as Add<<L_e3 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e3>>::Output,
    >>::Output: Add<<L_e31 as Mul<R_e2>>::Output>,
    <<<<<<L_one as Mul<R_e123>>::Output as Add<<L_e1 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e2 as Mul<R_e31>>::Output,
    >>::Output as Add<<L_e3 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e3>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_e2>>::Output>>::Output: Add<<L_e23 as Mul<R_e1>>::Output>,
    <<<<<<<L_one as Mul<R_e123>>::Output as Add<<L_e1 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e2 as Mul<R_e31>>::Output,
    >>::Output as Add<<L_e3 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e3>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_e2>>::Output>>::Output as Add<
        <L_e23 as Mul<R_e1>>::Output,
    >>::Output: Add<<L_e123 as Mul<R_one>>::Output>,
    <<L_one as Mul<R_e0123>>::Output as Add<<L_e0 as Mul<R_e123>>::Output>>::Output:
        Add<<L_e1 as Mul<R_e032>>::Output>,
    <<<L_one as Mul<R_e0123>>::Output as Add<<L_e0 as Mul<R_e123>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e032>>::Output,
    >>::Output: Add<<L_e2 as Mul<R_e013>>::Output>,
    <<<<L_one as Mul<R_e0123>>::Output as Add<<L_e0 as Mul<R_e123>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e032>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e013>>::Output>>::Output: Add<<L_e3 as Mul<R_e021>>::Output>,
    <<<<<L_one as Mul<R_e0123>>::Output as Add<<L_e0 as Mul<R_e123>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e032>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e013>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e021>>::Output,
    >>::Output: Add<<L_e01 as Mul<R_e23>>::Output>,
    <<<<<<L_one as Mul<R_e0123>>::Output as Add<<L_e0 as Mul<R_e123>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e032>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e013>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e021>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e23>>::Output>>::Output: Add<<L_e02 as Mul<R_e31>>::Output>,
    <<<<<<<L_one as Mul<R_e0123>>::Output as Add<<L_e0 as Mul<R_e123>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e032>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e013>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e021>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e31>>::Output,
    >>::Output: Add<<L_e03 as Mul<R_e12>>::Output>,
    <<<<<<<<L_one as Mul<R_e0123>>::Output as Add<<L_e0 as Mul<R_e123>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e032>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e013>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e021>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e31>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e12>>::Output>>::Output: Add<<L_e12 as Mul<R_e03>>::Output>,
    <<<<<<<<<L_one as Mul<R_e0123>>::Output as Add<<L_e0 as Mul<R_e123>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e032>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e013>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e021>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e31>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e03>>::Output,
    >>::Output: Add<<L_e31 as Mul<R_e02>>::Output>,
    <<<<<<<<<<L_one as Mul<R_e0123>>::Output as Add<<L_e0 as Mul<R_e123>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e032>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e013>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e021>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e31>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_e02>>::Output>>::Output: Add<<L_e23 as Mul<R_e01>>::Output>,
    <<<<<<<<<<<L_one as Mul<R_e0123>>::Output as Add<<L_e0 as Mul<R_e123>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e032>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e013>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e021>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e31>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_e02>>::Output>>::Output as Add<
        <L_e23 as Mul<R_e01>>::Output,
    >>::Output: Sub<<L_e021 as Mul<R_e3>>::Output>,
    <<<<<<<<<<<<L_one as Mul<R_e0123>>::Output as Add<<L_e0 as Mul<R_e123>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e032>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e013>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e021>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e31>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_e02>>::Output>>::Output as Add<
        <L_e23 as Mul<R_e01>>::Output,
    >>::Output as Sub<<L_e021 as Mul<R_e3>>::Output>>::Output: Sub<<L_e013 as Mul<R_e2>>::Output>,
    <<<<<<<<<<<<<L_one as Mul<R_e0123>>::Output as Add<<L_e0 as Mul<R_e123>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e032>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e013>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e021>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e31>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_e02>>::Output>>::Output as Add<
        <L_e23 as Mul<R_e01>>::Output,
    >>::Output as Sub<<L_e021 as Mul<R_e3>>::Output>>::Output as Sub<
        <L_e013 as Mul<R_e2>>::Output,
    >>::Output: Sub<<L_e032 as Mul<R_e1>>::Output>,
    <<<<<<<<<<<<<<L_one as Mul<R_e0123>>::Output as Add<<L_e0 as Mul<R_e123>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e032>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e013>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e021>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e31>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_e02>>::Output>>::Output as Add<
        <L_e23 as Mul<R_e01>>::Output,
    >>::Output as Sub<<L_e021 as Mul<R_e3>>::Output>>::Output as Sub<
        <L_e013 as Mul<R_e2>>::Output,
    >>::Output as Sub<<L_e032 as Mul<R_e1>>::Output>>::Output: Sub<<L_e123 as Mul<R_e0>>::Output>,
    <<<<<<<<<<<<<<<L_one as Mul<R_e0123>>::Output as Add<<L_e0 as Mul<R_e123>>::Output>>::Output as Add<
        <L_e1 as Mul<R_e032>>::Output,
    >>::Output as Add<<L_e2 as Mul<R_e013>>::Output>>::Output as Add<
        <L_e3 as Mul<R_e021>>::Output,
    >>::Output as Add<<L_e01 as Mul<R_e23>>::Output>>::Output as Add<
        <L_e02 as Mul<R_e31>>::Output,
    >>::Output as Add<<L_e03 as Mul<R_e12>>::Output>>::Output as Add<
        <L_e12 as Mul<R_e03>>::Output,
    >>::Output as Add<<L_e31 as Mul<R_e02>>::Output>>::Output as Add<
        <L_e23 as Mul<R_e01>>::Output,
    >>::Output as Sub<<L_e021 as Mul<R_e3>>::Output>>::Output as Sub<
        <L_e013 as Mul<R_e2>>::Output,
    >>::Output as Sub<<L_e032 as Mul<R_e1>>::Output>>::Output as Sub<
        <L_e123 as Mul<R_e0>>::Output,
    >>::Output: Add<<L_e0123 as Mul<R_one>>::Output>,
{
    type Output = MultiVector<
        <<<<<<<<L_one as Mul<R_one>>::Output as Add<<L_e1 as Mul<R_e1>>::Output>>::Output as Add<
            <L_e2 as Mul<R_e2>>::Output,
        >>::Output as Add<<L_e3 as Mul<R_e3>>::Output>>::Output as Sub<
            <L_e12 as Mul<R_e12>>::Output,
        >>::Output as Sub<<L_e31 as Mul<R_e31>>::Output>>::Output as Sub<
            <L_e23 as Mul<R_e23>>::Output,
        >>::Output as Sub<<L_e123 as Mul<R_e123>>::Output>>::Output,
        <<<<<<<<<<<<<<<<L_one as Mul<R_e0>>::Output as Add<<L_e0 as Mul<R_one>>::Output>>::Output as Sub<
            <L_e1 as Mul<R_e01>>::Output,
        >>::Output as Sub<<L_e2 as Mul<R_e02>>::Output>>::Output as Sub<
            <L_e3 as Mul<R_e03>>::Output,
        >>::Output as Add<<L_e01 as Mul<R_e1>>::Output>>::Output as Add<
            <L_e02 as Mul<R_e2>>::Output,
        >>::Output as Add<<L_e03 as Mul<R_e3>>::Output>>::Output as Add<
            <L_e12 as Mul<R_e021>>::Output,
        >>::Output as Add<<L_e31 as Mul<R_e013>>::Output>>::Output as Add<
            <L_e23 as Mul<R_e032>>::Output,
        >>::Output as Add<<L_e021 as Mul<R_e12>>::Output>>::Output as Add<
            <L_e013 as Mul<R_e31>>::Output,
        >>::Output as Add<<L_e032 as Mul<R_e23>>::Output>>::Output as Add<
            <L_e123 as Mul<R_e0123>>::Output,
        >>::Output as Sub<<L_e0123 as Mul<R_e123>>::Output>>::Output,
        <<<<<<<<L_one as Mul<R_e1>>::Output as Add<<L_e1 as Mul<R_one>>::Output>>::Output as Sub<
            <L_e2 as Mul<R_e12>>::Output,
        >>::Output as Add<<L_e3 as Mul<R_e31>>::Output>>::Output as Add<
            <L_e12 as Mul<R_e2>>::Output,
        >>::Output as Sub<<L_e31 as Mul<R_e3>>::Output>>::Output as Sub<
            <L_e23 as Mul<R_e123>>::Output,
        >>::Output as Sub<<L_e123 as Mul<R_e23>>::Output>>::Output,
        <<<<<<<<L_one as Mul<R_e2>>::Output as Add<<L_e1 as Mul<R_e12>>::Output>>::Output as Add<
            <L_e2 as Mul<R_one>>::Output,
        >>::Output as Sub<<L_e3 as Mul<R_e23>>::Output>>::Output as Sub<
            <L_e12 as Mul<R_e1>>::Output,
        >>::Output as Sub<<L_e31 as Mul<R_e123>>::Output>>::Output as Add<
            <L_e23 as Mul<R_e3>>::Output,
        >>::Output as Sub<<L_e123 as Mul<R_e31>>::Output>>::Output,
        <<<<<<<<L_one as Mul<R_e3>>::Output as Sub<<L_e1 as Mul<R_e31>>::Output>>::Output as Add<
            <L_e2 as Mul<R_e23>>::Output,
        >>::Output as Add<<L_e3 as Mul<R_one>>::Output>>::Output as Sub<
            <L_e12 as Mul<R_e123>>::Output,
        >>::Output as Add<<L_e31 as Mul<R_e1>>::Output>>::Output as Sub<
            <L_e23 as Mul<R_e2>>::Output,
        >>::Output as Sub<<L_e123 as Mul<R_e12>>::Output>>::Output,
        <<<<<<<<<<<<<<<<L_one as Mul<R_e01>>::Output as Add<<L_e0 as Mul<R_e1>>::Output>>::Output as Sub<
            <L_e1 as Mul<R_e0>>::Output,
        >>::Output as Sub<<L_e2 as Mul<R_e021>>::Output>>::Output as Add<
            <L_e3 as Mul<R_e013>>::Output,
        >>::Output as Add<<L_e01 as Mul<R_one>>::Output>>::Output as Sub<
            <L_e02 as Mul<R_e12>>::Output,
        >>::Output as Add<<L_e03 as Mul<R_e31>>::Output>>::Output as Add<
            <L_e12 as Mul<R_e02>>::Output,
        >>::Output as Sub<<L_e31 as Mul<R_e03>>::Output>>::Output as Sub<
            <L_e23 as Mul<R_e0123>>::Output,
        >>::Output as Sub<<L_e021 as Mul<R_e2>>::Output>>::Output as Add<
            <L_e013 as Mul<R_e3>>::Output,
        >>::Output as Add<<L_e032 as Mul<R_e123>>::Output>>::Output as Sub<
            <L_e123 as Mul<R_e032>>::Output,
        >>::Output as Sub<<L_e0123 as Mul<R_e23>>::Output>>::Output,
        <<<<<<<<<<<<<<<<L_one as Mul<R_e02>>::Output as Add<<L_e0 as Mul<R_e2>>::Output>>::Output as Add<
            <L_e1 as Mul<R_e021>>::Output,
        >>::Output as Sub<<L_e2 as Mul<R_e0>>::Output>>::Output as Sub<
            <L_e3 as Mul<R_e032>>::Output,
        >>::Output as Add<<L_e01 as Mul<R_e12>>::Output>>::Output as Add<
            <L_e02 as Mul<R_one>>::Output,
        >>::Output as Sub<<L_e03 as Mul<R_e23>>::Output>>::Output as Sub<
            <L_e12 as Mul<R_e01>>::Output,
        >>::Output as Sub<<L_e31 as Mul<R_e0123>>::Output>>::Output as Add<
            <L_e23 as Mul<R_e03>>::Output,
        >>::Output as Add<<L_e021 as Mul<R_e1>>::Output>>::Output as Add<
            <L_e013 as Mul<R_e123>>::Output,
        >>::Output as Sub<<L_e032 as Mul<R_e3>>::Output>>::Output as Sub<
            <L_e123 as Mul<R_e013>>::Output,
        >>::Output as Sub<<L_e0123 as Mul<R_e31>>::Output>>::Output,
        <<<<<<<<<<<<<<<<L_one as Mul<R_e03>>::Output as Add<<L_e0 as Mul<R_e3>>::Output>>::Output as Sub<
            <L_e1 as Mul<R_e013>>::Output,
        >>::Output as Add<<L_e2 as Mul<R_e032>>::Output>>::Output as Sub<
            <L_e3 as Mul<R_e0>>::Output,
        >>::Output as Sub<<L_e01 as Mul<R_e31>>::Output>>::Output as Add<
            <L_e02 as Mul<R_e23>>::Output,
        >>::Output as Add<<L_e03 as Mul<R_one>>::Output>>::Output as Sub<
            <L_e12 as Mul<R_e0123>>::Output,
        >>::Output as Add<<L_e31 as Mul<R_e01>>::Output>>::Output as Sub<
            <L_e23 as Mul<R_e02>>::Output,
        >>::Output as Add<<L_e021 as Mul<R_e123>>::Output>>::Output as Sub<
            <L_e013 as Mul<R_e1>>::Output,
        >>::Output as Add<<L_e032 as Mul<R_e2>>::Output>>::Output as Sub<
            <L_e123 as Mul<R_e021>>::Output,
        >>::Output as Sub<<L_e0123 as Mul<R_e12>>::Output>>::Output,
        <<<<<<<<L_one as Mul<R_e12>>::Output as Add<<L_e1 as Mul<R_e2>>::Output>>::Output as Sub<
            <L_e2 as Mul<R_e1>>::Output,
        >>::Output as Add<<L_e3 as Mul<R_e123>>::Output>>::Output as Add<
            <L_e12 as Mul<R_one>>::Output,
        >>::Output as Add<<L_e31 as Mul<R_e23>>::Output>>::Output as Sub<
            <L_e23 as Mul<R_e31>>::Output,
        >>::Output as Add<<L_e123 as Mul<R_e3>>::Output>>::Output,
        <<<<<<<<L_one as Mul<R_e31>>::Output as Sub<<L_e1 as Mul<R_e3>>::Output>>::Output as Add<
            <L_e2 as Mul<R_e123>>::Output,
        >>::Output as Add<<L_e3 as Mul<R_e1>>::Output>>::Output as Sub<
            <L_e12 as Mul<R_e23>>::Output,
        >>::Output as Add<<L_e31 as Mul<R_one>>::Output>>::Output as Add<
            <L_e23 as Mul<R_e12>>::Output,
        >>::Output as Add<<L_e123 as Mul<R_e2>>::Output>>::Output,
        <<<<<<<<L_one as Mul<R_e23>>::Output as Add<<L_e1 as Mul<R_e123>>::Output>>::Output as Add<
            <L_e2 as Mul<R_e3>>::Output,
        >>::Output as Sub<<L_e3 as Mul<R_e2>>::Output>>::Output as Add<
            <L_e12 as Mul<R_e31>>::Output,
        >>::Output as Sub<<L_e31 as Mul<R_e12>>::Output>>::Output as Add<
            <L_e23 as Mul<R_one>>::Output,
        >>::Output as Add<<L_e123 as Mul<R_e1>>::Output>>::Output,
        <<<<<<<<<<<<<<<<L_one as Mul<R_e021>>::Output as Sub<<L_e0 as Mul<R_e12>>::Output>>::Output as Add<
            <L_e1 as Mul<R_e02>>::Output,
        >>::Output as Sub<<L_e2 as Mul<R_e01>>::Output>>::Output as Add<
            <L_e3 as Mul<R_e0123>>::Output,
        >>::Output as Sub<<L_e01 as Mul<R_e2>>::Output>>::Output as Add<
            <L_e02 as Mul<R_e1>>::Output,
        >>::Output as Sub<<L_e03 as Mul<R_e123>>::Output>>::Output as Sub<
            <L_e12 as Mul<R_e0>>::Output,
        >>::Output as Add<<L_e31 as Mul<R_e032>>::Output>>::Output as Sub<
            <L_e23 as Mul<R_e013>>::Output,
        >>::Output as Add<<L_e021 as Mul<R_one>>::Output>>::Output as Add<
            <L_e013 as Mul<R_e23>>::Output,
        >>::Output as Sub<<L_e032 as Mul<R_e31>>::Output>>::Output as Add<
            <L_e123 as Mul<R_e03>>::Output,
        >>::Output as Sub<<L_e0123 as Mul<R_e3>>::Output>>::Output,
        <<<<<<<<<<<<<<<<L_one as Mul<R_e013>>::Output as Sub<<L_e0 as Mul<R_e31>>::Output>>::Output as Sub<
            <L_e1 as Mul<R_e03>>::Output,
        >>::Output as Add<<L_e2 as Mul<R_e0123>>::Output>>::Output as Add<
            <L_e3 as Mul<R_e01>>::Output,
        >>::Output as Add<<L_e01 as Mul<R_e3>>::Output>>::Output as Sub<
            <L_e02 as Mul<R_e123>>::Output,
        >>::Output as Sub<<L_e03 as Mul<R_e1>>::Output>>::Output as Sub<
            <L_e12 as Mul<R_e032>>::Output,
        >>::Output as Sub<<L_e31 as Mul<R_e0>>::Output>>::Output as Add<
            <L_e23 as Mul<R_e021>>::Output,
        >>::Output as Sub<<L_e021 as Mul<R_e23>>::Output>>::Output as Add<
            <L_e013 as Mul<R_one>>::Output,
        >>::Output as Add<<L_e032 as Mul<R_e12>>::Output>>::Output as Add<
            <L_e123 as Mul<R_e02>>::Output,
        >>::Output as Sub<<L_e0123 as Mul<R_e2>>::Output>>::Output,
        <<<<<<<<<<<<<<<<L_one as Mul<R_e032>>::Output as Sub<<L_e0 as Mul<R_e23>>::Output>>::Output as Add<
            <L_e1 as Mul<R_e0123>>::Output,
        >>::Output as Add<<L_e2 as Mul<R_e03>>::Output>>::Output as Sub<
            <L_e3 as Mul<R_e02>>::Output,
        >>::Output as Sub<<L_e01 as Mul<R_e123>>::Output>>::Output as Sub<
            <L_e02 as Mul<R_e3>>::Output,
        >>::Output as Add<<L_e03 as Mul<R_e2>>::Output>>::Output as Add<
            <L_e12 as Mul<R_e013>>::Output,
        >>::Output as Sub<<L_e31 as Mul<R_e021>>::Output>>::Output as Sub<
            <L_e23 as Mul<R_e0>>::Output,
        >>::Output as Add<<L_e021 as Mul<R_e31>>::Output>>::Output as Sub<
            <L_e013 as Mul<R_e12>>::Output,
        >>::Output as Add<<L_e032 as Mul<R_one>>::Output>>::Output as Add<
            <L_e123 as Mul<R_e01>>::Output,
        >>::Output as Sub<<L_e0123 as Mul<R_e1>>::Output>>::Output,
        <<<<<<<<L_one as Mul<R_e123>>::Output as Add<<L_e1 as Mul<R_e23>>::Output>>::Output as Add<
            <L_e2 as Mul<R_e31>>::Output,
        >>::Output as Add<<L_e3 as Mul<R_e12>>::Output>>::Output as Add<
            <L_e12 as Mul<R_e3>>::Output,
        >>::Output as Add<<L_e31 as Mul<R_e2>>::Output>>::Output as Add<
            <L_e23 as Mul<R_e1>>::Output,
        >>::Output as Add<<L_e123 as Mul<R_one>>::Output>>::Output,
        <<<<<<<<<<<<<<<<L_one as Mul<R_e0123>>::Output as Add<<L_e0 as Mul<R_e123>>::Output>>::Output as Add<
            <L_e1 as Mul<R_e032>>::Output,
        >>::Output as Add<<L_e2 as Mul<R_e013>>::Output>>::Output as Add<
            <L_e3 as Mul<R_e021>>::Output,
        >>::Output as Add<<L_e01 as Mul<R_e23>>::Output>>::Output as Add<
            <L_e02 as Mul<R_e31>>::Output,
        >>::Output as Add<<L_e03 as Mul<R_e12>>::Output>>::Output as Add<
            <L_e12 as Mul<R_e03>>::Output,
        >>::Output as Add<<L_e31 as Mul<R_e02>>::Output>>::Output as Add<
            <L_e23 as Mul<R_e01>>::Output,
        >>::Output as Sub<<L_e021 as Mul<R_e3>>::Output>>::Output as Sub<
            <L_e013 as Mul<R_e2>>::Output,
        >>::Output as Sub<<L_e032 as Mul<R_e1>>::Output>>::Output as Sub<
            <L_e123 as Mul<R_e0>>::Output,
        >>::Output as Add<<L_e0123 as Mul<R_one>>::Output>>::Output,
    >;

    #[inline]
    fn mul(
        self,
        b: MultiVector<
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
        let a = self;
        Self::Output {
            one: a.one * b.one + a.e1 * b.e1 + a.e2 * b.e2 + a.e3 * b.e3
                - a.e12 * b.e12
                - a.e31 * b.e31
                - a.e23 * b.e23
                - a.e123 * b.e123,
            e0: a.one * b.e0 + a.e0 * b.one - a.e1 * b.e01 - a.e2 * b.e02 - a.e3 * b.e03
                + a.e01 * b.e1
                + a.e02 * b.e2
                + a.e03 * b.e3
                + a.e12 * b.e021
                + a.e31 * b.e013
                + a.e23 * b.e032
                + a.e021 * b.e12
                + a.e013 * b.e31
                + a.e032 * b.e23
                + a.e123 * b.e0123
                - a.e0123 * b.e123,
            e1: a.one * b.e1 + a.e1 * b.one - a.e2 * b.e12 + a.e3 * b.e31 + a.e12 * b.e2
                - a.e31 * b.e3
                - a.e23 * b.e123
                - a.e123 * b.e23,
            e2: a.one * b.e2 + a.e1 * b.e12 + a.e2 * b.one
                - a.e3 * b.e23
                - a.e12 * b.e1
                - a.e31 * b.e123
                + a.e23 * b.e3
                - a.e123 * b.e31,
            e3: a.one * b.e3 - a.e1 * b.e31 + a.e2 * b.e23 + a.e3 * b.one - a.e12 * b.e123
                + a.e31 * b.e1
                - a.e23 * b.e2
                - a.e123 * b.e12,
            e01: a.one * b.e01 + a.e0 * b.e1 - a.e1 * b.e0 - a.e2 * b.e021
                + a.e3 * b.e013
                + a.e01 * b.one
                - a.e02 * b.e12
                + a.e03 * b.e31
                + a.e12 * b.e02
                - a.e31 * b.e03
                - a.e23 * b.e0123
                - a.e021 * b.e2
                + a.e013 * b.e3
                + a.e032 * b.e123
                - a.e123 * b.e032
                - a.e0123 * b.e23,
            e02: a.one * b.e02 + a.e0 * b.e2 + a.e1 * b.e021 - a.e2 * b.e0 - a.e3 * b.e032
                + a.e01 * b.e12
                + a.e02 * b.one
                - a.e03 * b.e23
                - a.e12 * b.e01
                - a.e31 * b.e0123
                + a.e23 * b.e03
                + a.e021 * b.e1
                + a.e013 * b.e123
                - a.e032 * b.e3
                - a.e123 * b.e013
                - a.e0123 * b.e31,
            e03: a.one * b.e03 + a.e0 * b.e3 - a.e1 * b.e013 + a.e2 * b.e032
                - a.e3 * b.e0
                - a.e01 * b.e31
                + a.e02 * b.e23
                + a.e03 * b.one
                - a.e12 * b.e0123
                + a.e31 * b.e01
                - a.e23 * b.e02
                + a.e021 * b.e123
                - a.e013 * b.e1
                + a.e032 * b.e2
                - a.e123 * b.e021
                - a.e0123 * b.e12,
            e12: a.one * b.e12 + a.e1 * b.e2 - a.e2 * b.e1
                + a.e3 * b.e123
                + a.e12 * b.one
                + a.e31 * b.e23
                - a.e23 * b.e31
                + a.e123 * b.e3,
            e31: a.one * b.e31 - a.e1 * b.e3 + a.e2 * b.e123 + a.e3 * b.e1 - a.e12 * b.e23
                + a.e31 * b.one
                + a.e23 * b.e12
                + a.e123 * b.e2,
            e23: a.one * b.e23 + a.e1 * b.e123 + a.e2 * b.e3 - a.e3 * b.e2 + a.e12 * b.e31
                - a.e31 * b.e12
                + a.e23 * b.one
                + a.e123 * b.e1,
            e021: a.one * b.e021 - a.e0 * b.e12 + a.e1 * b.e02 - a.e2 * b.e01 + a.e3 * b.e0123
                - a.e01 * b.e2
                + a.e02 * b.e1
                - a.e03 * b.e123
                - a.e12 * b.e0
                + a.e31 * b.e032
                - a.e23 * b.e013
                + a.e021 * b.one
                + a.e013 * b.e23
                - a.e032 * b.e31
                + a.e123 * b.e03
                - a.e0123 * b.e3,
            e013: a.one * b.e013 - a.e0 * b.e31 - a.e1 * b.e03
                + a.e2 * b.e0123
                + a.e3 * b.e01
                + a.e01 * b.e3
                - a.e02 * b.e123
                - a.e03 * b.e1
                - a.e12 * b.e032
                - a.e31 * b.e0
                + a.e23 * b.e021
                - a.e021 * b.e23
                + a.e013 * b.one
                + a.e032 * b.e12
                + a.e123 * b.e02
                - a.e0123 * b.e2,
            e032: a.one * b.e032 - a.e0 * b.e23 + a.e1 * b.e0123 + a.e2 * b.e03
                - a.e3 * b.e02
                - a.e01 * b.e123
                - a.e02 * b.e3
                + a.e03 * b.e2
                + a.e12 * b.e013
                - a.e31 * b.e021
                - a.e23 * b.e0
                + a.e021 * b.e31
                - a.e013 * b.e12
                + a.e032 * b.one
                + a.e123 * b.e01
                - a.e0123 * b.e1,
            e123: a.one * b.e123
                + a.e1 * b.e23
                + a.e2 * b.e31
                + a.e3 * b.e12
                + a.e12 * b.e3
                + a.e31 * b.e2
                + a.e23 * b.e1
                + a.e123 * b.one,
            e0123: a.one * b.e0123
                + a.e0 * b.e123
                + a.e1 * b.e032
                + a.e2 * b.e013
                + a.e3 * b.e021
                + a.e01 * b.e23
                + a.e02 * b.e31
                + a.e03 * b.e12
                + a.e12 * b.e03
                + a.e31 * b.e02
                + a.e23 * b.e01
                - a.e021 * b.e3
                - a.e013 * b.e2
                - a.e032 * b.e1
                - a.e123 * b.e0
                + a.e0123 * b.one,
        }
    }
}

#[test]
fn test_sub_multivector() {
    let a = MultiVector::<
        One,
        Zero,
        Zero,
        Zero,
        Zero,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        Zero,
        Zero,
        Zero,
        Zero,
        Zero,
    >::default();
    let b = MultiVector::<
        One,
        Zero,
        Zero,
        Zero,
        Zero,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        Zero,
        Zero,
        Zero,
        Zero,
        Zero,
    >::default();
    let result = a - b;
    let _: Const<0> = result.one;
}

#[test]
fn test_add_neg_multivector() {
    let a = MultiVector::<
        One,
        Zero,
        Zero,
        Zero,
        Zero,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        Zero,
        Zero,
        Zero,
        Zero,
        Zero,
    >::default();
    let b = MultiVector::<
        One,
        Zero,
        Zero,
        Zero,
        Zero,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        Zero,
        Zero,
        Zero,
        Zero,
        Zero,
    >::default();
    let result = a + -b;
    let _: Const<0> = result.one;
}

#[test]
fn test_mul_multivector() {
    let a = MultiVector::<
        One,
        Zero,
        Zero,
        Zero,
        Zero,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        Zero,
        Zero,
        Zero,
        Zero,
        Zero,
    >::default();
    let b = MultiVector::<
        One,
        Zero,
        Zero,
        Zero,
        Zero,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        Zero,
        Zero,
        Zero,
        Zero,
        Zero,
    >::default();
    let result = a * b;
    let _: Float = result.one;
}
