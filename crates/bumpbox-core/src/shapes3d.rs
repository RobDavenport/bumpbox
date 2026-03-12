use crate::{Fx32, Vec3};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Aabb3 {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb3 {
    pub fn try_new(min: Vec3, max: Vec3) -> Option<Self> {
        if min.x <= max.x && min.y <= max.y && min.z <= max.z {
            Some(Self { min, max })
        } else {
            None
        }
    }

    pub fn contains_point(&self, point: Vec3) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
            && point.z >= self.min.z
            && point.z <= self.max.z
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ray3 {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray3 {
    pub const fn new(origin: Vec3, dir: Vec3) -> Self {
        Self { origin, dir }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Segment3 {
    pub start: Vec3,
    pub end: Vec3,
}

impl Segment3 {
    pub const fn new(start: Vec3, end: Vec3) -> Self {
        Self { start, end }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: Fx32,
}

impl Sphere {
    pub fn try_new(center: Vec3, radius: Fx32) -> Option<Self> {
        if radius >= Fx32::ZERO {
            Some(Self { center, radius })
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Capsule3 {
    pub segment: Segment3,
    pub radius: Fx32,
}

impl Capsule3 {
    pub fn try_new(segment: Segment3, radius: Fx32) -> Option<Self> {
        if radius >= Fx32::ZERO {
            Some(Self { segment, radius })
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Triangle3 {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
}

impl Triangle3 {
    pub const fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self { a, b, c }
    }
}
