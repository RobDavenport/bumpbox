use crate::{Aabb, Capsule, Circle, ConvexPolygon, Fx32, OrientedBox, Ray, Vec2};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RayHit {
    pub toi: Fx32,
    pub point: Vec2,
    pub normal: Vec2,
}

pub fn aabb_overlaps(a: &Aabb, b: &Aabb) -> bool {
    a.min.x <= b.max.x && a.max.x >= b.min.x && a.min.y <= b.max.y && a.max.y >= b.min.y
}

pub fn closest_point_on_aabb(aabb: &Aabb, point: Vec2) -> Vec2 {
    Vec2::new(point.x.max(aabb.min.x).min(aabb.max.x), point.y.max(aabb.min.y).min(aabb.max.y))
}

pub fn distance_squared_point_aabb(aabb: &Aabb, point: Vec2) -> Fx32 {
    let closest = closest_point_on_aabb(aabb, point);
    let delta = point - closest;
    delta.length_squared()
}

pub fn circle_overlaps_aabb(circle: &Circle, aabb: &Aabb) -> bool {
    let radius_sq = circle.radius * circle.radius;
    distance_squared_point_aabb(aabb, circle.center) <= radius_sq
}

pub fn capsule_contains_point(capsule: &Capsule, point: Vec2) -> bool {
    let radius_sq = capsule.radius * capsule.radius;
    distance_squared_point_segment(capsule.segment.start, capsule.segment.end, point) <= radius_sq
}

pub fn convex_polygon_contains_point<const N: usize>(
    polygon: &ConvexPolygon<N>,
    point: Vec2,
) -> bool {
    let mut winding = 0i64;
    let mut index = 0usize;

    while index < N {
        let start = polygon.points[index];
        let end = polygon.points[(index + 1) % N];
        let cross = orient2d(start, end, point);

        if cross == 0 {
            if !point_on_segment(start, end, point) {
                return false;
            }
        } else {
            let sign = if cross > 0 { 1 } else { -1 };
            if winding == 0 {
                winding = sign;
            } else if sign != winding {
                return false;
            }
        }

        index += 1;
    }

    true
}

pub fn oriented_box_contains_point(oriented_box: &OrientedBox, point: Vec2) -> bool {
    let delta = point - oriented_box.center;
    let determinant = cross(oriented_box.axis_x, oriented_box.axis_y);

    if determinant == 0 {
        return false;
    }

    let local_x = ratio_i64_to_fx32(cross(delta, oriented_box.axis_y), determinant);
    let local_y = ratio_i64_to_fx32(cross(oriented_box.axis_x, delta), determinant);

    local_x.abs() <= oriented_box.half_extents.x && local_y.abs() <= oriented_box.half_extents.y
}

pub fn raycast_aabb(ray: &Ray, aabb: &Aabb, max_toi: Fx32) -> Option<RayHit> {
    let (tx_min, tx_max) = axis_interval(ray.origin.x, ray.dir.x, aabb.min.x, aabb.max.x)?;
    let (ty_min, ty_max) = axis_interval(ray.origin.y, ray.dir.y, aabb.min.y, aabb.max.y)?;

    let t_near = tx_min.max(ty_min);
    let t_far = tx_max.min(ty_max);

    if t_near > t_far || t_far < Fx32::ZERO || t_near > max_toi {
        return None;
    }

    let toi = if t_near < Fx32::ZERO { Fx32::ZERO } else { t_near };
    let point = ray.origin + (ray.dir * toi);

    let normal = if tx_min >= ty_min {
        if ray.dir.x.signum() >= 0 {
            Vec2::new(Fx32::from_int(-1), Fx32::ZERO)
        } else {
            Vec2::new(Fx32::from_int(1), Fx32::ZERO)
        }
    } else if ray.dir.y.signum() >= 0 {
        Vec2::new(Fx32::ZERO, Fx32::from_int(-1))
    } else {
        Vec2::new(Fx32::ZERO, Fx32::from_int(1))
    };

    Some(RayHit { toi, point, normal })
}

fn axis_interval(origin: Fx32, direction: Fx32, min: Fx32, max: Fx32) -> Option<(Fx32, Fx32)> {
    if direction == Fx32::ZERO {
        if origin < min || origin > max {
            return None;
        }
        return Some((Fx32::MIN, Fx32::MAX));
    }

    let t0 = (min - origin) / direction;
    let t1 = (max - origin) / direction;

    if t0 <= t1 {
        Some((t0, t1))
    } else {
        Some((t1, t0))
    }
}

fn distance_squared_point_segment(start: Vec2, end: Vec2, point: Vec2) -> Fx32 {
    let delta = end - start;
    let length_sq = delta.length_squared();

    if length_sq == Fx32::ZERO {
        return (point - start).length_squared();
    }

    let projected = (point - start).dot(delta) / length_sq;
    let t = projected.max(Fx32::ZERO).min(Fx32::ONE);
    let closest = start + (delta * t);
    (point - closest).length_squared()
}

fn orient2d(a: Vec2, b: Vec2, point: Vec2) -> i64 {
    let ab = b - a;
    let ap = point - a;
    (ab.x.raw() as i64 * ap.y.raw() as i64) - (ab.y.raw() as i64 * ap.x.raw() as i64)
}

fn cross(lhs: Vec2, rhs: Vec2) -> i64 {
    (lhs.x.raw() as i64 * rhs.y.raw() as i64) - (lhs.y.raw() as i64 * rhs.x.raw() as i64)
}

fn ratio_i64_to_fx32(num: i64, den: i64) -> Fx32 {
    let scaled = ((num as i128) << Fx32::FRAC_BITS) / den as i128;
    if scaled < i32::MIN as i128 {
        Fx32::MIN
    } else if scaled > i32::MAX as i128 {
        Fx32::MAX
    } else {
        Fx32::from_raw(scaled as i32)
    }
}

fn point_on_segment(start: Vec2, end: Vec2, point: Vec2) -> bool {
    point.x >= start.x.min(end.x)
        && point.x <= start.x.max(end.x)
        && point.y >= start.y.min(end.y)
        && point.y <= start.y.max(end.y)
}
