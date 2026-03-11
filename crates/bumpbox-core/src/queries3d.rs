use crate::{Aabb3, Fx32, Ray3, Sphere, Vec3};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RayHit3 {
    pub toi: Fx32,
    pub point: Vec3,
    pub normal: Vec3,
}

pub fn aabb3_overlaps(a: &Aabb3, b: &Aabb3) -> bool {
    a.min.x <= b.max.x
        && a.max.x >= b.min.x
        && a.min.y <= b.max.y
        && a.max.y >= b.min.y
        && a.min.z <= b.max.z
        && a.max.z >= b.min.z
}

pub fn closest_point_on_aabb3(aabb: &Aabb3, point: Vec3) -> Vec3 {
    Vec3::new(
        point.x.max(aabb.min.x).min(aabb.max.x),
        point.y.max(aabb.min.y).min(aabb.max.y),
        point.z.max(aabb.min.z).min(aabb.max.z),
    )
}

pub fn distance_squared_point_aabb3(aabb: &Aabb3, point: Vec3) -> Fx32 {
    let closest = closest_point_on_aabb3(aabb, point);
    let delta = point - closest;
    delta.length_squared()
}

pub fn sphere_overlaps_aabb3(sphere: &Sphere, aabb: &Aabb3) -> bool {
    let radius_sq = sphere.radius * sphere.radius;
    distance_squared_point_aabb3(aabb, sphere.center) <= radius_sq
}

pub fn raycast_aabb3(ray: &Ray3, aabb: &Aabb3, max_toi: Fx32) -> Option<RayHit3> {
    let (tx_min, tx_max) = axis_interval(ray.origin.x, ray.dir.x, aabb.min.x, aabb.max.x)?;
    let (ty_min, ty_max) = axis_interval(ray.origin.y, ray.dir.y, aabb.min.y, aabb.max.y)?;
    let (tz_min, tz_max) = axis_interval(ray.origin.z, ray.dir.z, aabb.min.z, aabb.max.z)?;

    let t_near = tx_min.max(ty_min).max(tz_min);
    let t_far = tx_max.min(ty_max).min(tz_max);

    if t_near > t_far || t_far < Fx32::ZERO || t_near > max_toi {
        return None;
    }

    let toi = if t_near < Fx32::ZERO { Fx32::ZERO } else { t_near };
    let point = ray.origin + (ray.dir * toi);

    let normal = if tx_min >= ty_min && tx_min >= tz_min {
        if ray.dir.x.signum() >= 0 {
            Vec3::new(Fx32::from_int(-1), Fx32::ZERO, Fx32::ZERO)
        } else {
            Vec3::new(Fx32::from_int(1), Fx32::ZERO, Fx32::ZERO)
        }
    } else if ty_min >= tz_min {
        if ray.dir.y.signum() >= 0 {
            Vec3::new(Fx32::ZERO, Fx32::from_int(-1), Fx32::ZERO)
        } else {
            Vec3::new(Fx32::ZERO, Fx32::from_int(1), Fx32::ZERO)
        }
    } else if ray.dir.z.signum() >= 0 {
        Vec3::new(Fx32::ZERO, Fx32::ZERO, Fx32::from_int(-1))
    } else {
        Vec3::new(Fx32::ZERO, Fx32::ZERO, Fx32::from_int(1))
    };

    Some(RayHit3 { toi, point, normal })
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
