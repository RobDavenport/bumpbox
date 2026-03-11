use crate::{Fx32, Vec2};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Aabb {
    pub min: Vec2,
    pub max: Vec2,
}

impl Aabb {
    pub fn try_new(min: Vec2, max: Vec2) -> Option<Self> {
        if min.x <= max.x && min.y <= max.y {
            Some(Self { min, max })
        } else {
            None
        }
    }

    pub fn contains_point(&self, point: Vec2) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Segment {
    pub start: Vec2,
    pub end: Vec2,
}

impl Segment {
    pub const fn new(start: Vec2, end: Vec2) -> Self {
        Self { start, end }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ray {
    pub origin: Vec2,
    pub dir: Vec2,
}

impl Ray {
    pub const fn new(origin: Vec2, dir: Vec2) -> Self {
        Self { origin, dir }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Circle {
    pub center: Vec2,
    pub radius: Fx32,
}

impl Circle {
    pub fn try_new(center: Vec2, radius: Fx32) -> Option<Self> {
        if radius >= Fx32::ZERO {
            Some(Self { center, radius })
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Capsule {
    pub segment: Segment,
    pub radius: Fx32,
}

impl Capsule {
    pub fn try_new(segment: Segment, radius: Fx32) -> Option<Self> {
        if radius >= Fx32::ZERO {
            Some(Self { segment, radius })
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OrientedBox {
    pub center: Vec2,
    pub half_extents: Vec2,
    pub axis_x: Vec2,
    pub axis_y: Vec2,
}

impl OrientedBox {
    pub fn try_new(center: Vec2, half_extents: Vec2, axis_x: Vec2, axis_y: Vec2) -> Option<Self> {
        if half_extents.x < Fx32::ZERO || half_extents.y < Fx32::ZERO {
            return None;
        }
        if axis_x.length_squared() == Fx32::ZERO || axis_y.length_squared() == Fx32::ZERO {
            return None;
        }
        if cross(axis_x, axis_y) == 0 {
            return None;
        }
        Some(Self { center, half_extents, axis_x, axis_y })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConvexPolygon<const N: usize> {
    pub points: [Vec2; N],
}

impl<const N: usize> ConvexPolygon<N> {
    pub fn try_new(points: [Vec2; N]) -> Option<Self> {
        if N < 3 {
            return None;
        }
        Some(Self { points })
    }

    pub const fn vertex_count(&self) -> usize {
        N
    }
}

fn cross(lhs: Vec2, rhs: Vec2) -> i64 {
    (lhs.x.raw() as i64 * rhs.y.raw() as i64) - (lhs.y.raw() as i64 * rhs.x.raw() as i64)
}
