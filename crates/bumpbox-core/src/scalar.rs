use core::ops::{Add, Div, Mul, Neg, Sub};

use fixed::types::I16F16;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fx32(I16F16);

impl Fx32 {
    pub const FRAC_BITS: u32 = 16;
    pub const SCALE: i32 = 1 << Self::FRAC_BITS;
    pub const ZERO: Self = Self(I16F16::from_bits(0));
    pub const ONE: Self = Self(I16F16::from_bits(Self::SCALE));
    pub const MIN: Self = Self(I16F16::from_bits(i32::MIN));
    pub const MAX: Self = Self(I16F16::from_bits(i32::MAX));

    pub const fn from_raw(raw: i32) -> Self {
        Self(I16F16::from_bits(raw))
    }

    pub const fn raw(self) -> i32 {
        self.0.to_bits()
    }

    pub const fn from_int(value: i32) -> Self {
        Self(I16F16::from_bits(value << Self::FRAC_BITS))
    }

    pub fn from_ratio(num: i32, den: i32) -> Self {
        assert!(den != 0, "denominator must not be zero");
        let scaled = (num as i64) * (Self::SCALE as i64) / (den as i64);
        Self::from_raw(clamp_i64_to_i32(scaled))
    }

    pub fn floor_to_int(self) -> i32 {
        self.raw() >> Self::FRAC_BITS
    }

    pub fn abs(self) -> Self {
        Self::from_raw(self.raw().saturating_abs())
    }

    pub fn min(self, other: Self) -> Self {
        if self <= other {
            self
        } else {
            other
        }
    }

    pub fn max(self, other: Self) -> Self {
        if self >= other {
            self
        } else {
            other
        }
    }

    pub fn signum(self) -> i32 {
        if self.raw() < 0 {
            -1
        } else if self.raw() > 0 {
            1
        } else {
            0
        }
    }
}

const fn clamp_i64_to_i32(value: i64) -> i32 {
    if value < i32::MIN as i64 {
        i32::MIN
    } else if value > i32::MAX as i64 {
        i32::MAX
    } else {
        value as i32
    }
}

impl From<i32> for Fx32 {
    fn from(value: i32) -> Self {
        Self::from_int(value)
    }
}

impl Add for Fx32 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_raw(self.raw().saturating_add(rhs.raw()))
    }
}

impl Sub for Fx32 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_raw(self.raw().saturating_sub(rhs.raw()))
    }
}

impl Neg for Fx32 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::from_raw(self.raw().saturating_neg())
    }
}

impl Mul for Fx32 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let wide = (self.raw() as i64) * (rhs.raw() as i64);
        Self::from_raw(clamp_i64_to_i32(wide >> Self::FRAC_BITS))
    }
}

impl Div for Fx32 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        assert!(rhs.raw() != 0, "division by zero");
        let wide = ((self.raw() as i64) << Self::FRAC_BITS) / (rhs.raw() as i64);
        Self::from_raw(clamp_i64_to_i32(wide))
    }
}
