use std::ops::{Add, Mul, Div, AddAssign, MulAssign, DivAssign};
use std::fmt::Display;
use std::iter::Sum;

use core::cmp::{PartialEq, PartialOrd};

pub trait Identity {
    fn zero() -> Self;
    fn one() -> Self;
    fn negative() -> Self;
}

macro_rules! usign_impl_identity {
    ($($t:ty)*) => {
        $(
            impl Identity for $t {
                fn zero() -> Self { 0 as $t }
                fn one() -> Self { 1 as $t }
                fn negative() -> Self { 1 as $t }
            }
        )*
    };
}

macro_rules! sign_impl_identity {
    ($($t:ty)*) => {
        $(
            impl Identity for $t {
                fn zero() -> Self { 0 as $t }
                fn one() -> Self { 1 as $t }
                fn negative() -> Self { -1 as $t }
            }
        )*
    };
}

sign_impl_identity!(i8 i16 i32 i64 i128 isize f32 f64);
usign_impl_identity!(u8 u16 u32 u64 u128 usize);


pub trait Numeric: Display + 
    Identity + Copy + Mul<Output = Self> + Add<Output = Self> + PartialOrd + 
    PartialEq + Div<Output = Self> + AddAssign + MulAssign + DivAssign + Sum<Self> + PartialOrd
{
    fn eps() -> Self;
    fn abs(self) -> Self;
    fn sqrt(self) -> Self;
    fn is_zero(self) -> bool;
    fn is_negative(self) -> bool;
    fn ground_if_zero(self) -> Self;
    fn to_float(self) -> f64;
    fn to_int(self) -> isize;
    fn from_float(num: f64) -> Self;
    fn sign(self) -> Self;
}
macro_rules! impl_numeric {
    ($($t:ty)*) => {
        $(
            impl Numeric for $t {
                fn eps() -> Self { 1e-10 as $t }
                fn abs(self) -> Self{
                    if self < Self::zero() {
                        self * Self::negative()
                    }else{
                        self
                    }
                }

                fn sqrt(self) -> Self{
                    if self < Self::zero() {
                        panic!("Cannot compute square root of a negative number! -- {:?}", self);
                    }

                    (self as f64).sqrt() as $t
                }

                fn is_zero(self) -> bool{
                    self.abs() <= Self::eps()
                }

                fn is_negative(self) -> bool{
                    self < Self::zero()
                }

                fn ground_if_zero(self) -> Self{
                    if self.is_zero() {
                        Self::zero()
                    }else{
                        self
                    }
                }

                fn to_float(self) -> f64{
                    self as f64
                }

                fn to_int(self) -> isize{
                    self as isize
                }

                fn from_float(num: f64) -> Self{
                    num as Self
                }

                fn sign(self) -> Self{
                    if Self::zero() < self {
                        Self::one()
                    }else{
                        Self::negative()
                    }
                }
            }
        )*
    };
}
impl_numeric!(i8 i16 i32 i64 i128 isize f32 f64 u8 u16 u32 u64 u128 usize);
