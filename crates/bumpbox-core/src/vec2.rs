use core::ops::{Add, Mul, Neg, Sub};

use crate::Fx32;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Vec2 {
    pub x: Fx32,
    pub y: Fx32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: Fx32::ZERO, y: Fx32::ZERO };

    pub const fn new(x: Fx32, y: Fx32) -> Self {
        Self { x, y }
    }

    pub fn dot(self, other: Self) -> Fx32 {
        self.x * other.x + self.y * other.y
    }

    pub fn length_squared(self) -> Fx32 {
        self.dot(self)
    }

    pub fn component_min(self, other: Self) -> Self {
        Self::new(self.x.min(other.x), self.y.min(other.y))
    }

    pub fn component_max(self, other: Self) -> Self {
        Self::new(self.x.max(other.x), self.y.max(other.y))
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl Mul<Fx32> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: Fx32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}
