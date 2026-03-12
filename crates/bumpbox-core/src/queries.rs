use crate::{
    scalar::{smallest_non_negative_quadratic_root, sqrt_non_negative},
    Aabb, Capsule, Circle, ConvexPolygon, Fx32, OrientedBox, Ray, Segment, Vec2,
};

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

pub fn closest_point_on_segment(segment: &Segment, point: Vec2) -> Vec2 {
    closest_point_on_segment_points(segment.start, segment.end, point)
}

pub fn distance_squared_point_segment(segment: &Segment, point: Vec2) -> Fx32 {
    let closest = closest_point_on_segment(segment, point);
    let delta = point - closest;
    delta.length_squared()
}

pub fn closest_point_on_circle(circle: &Circle, point: Vec2) -> Vec2 {
    let delta = point - circle.center;
    let distance_sq = delta.length_squared();
    let radius_sq = circle.radius * circle.radius;

    if distance_sq <= radius_sq {
        return point;
    }

    if let Some(snapped) = snap_axis_aligned_boundary_point2(circle.center, delta, circle.radius) {
        return snapped;
    }

    let distance = sqrt_non_negative(distance_sq).expect("distance squared is non-negative");
    if distance == Fx32::ZERO {
        circle.center
    } else {
        circle.center + (delta * (circle.radius / distance))
    }
}

pub fn distance_squared_point_circle(circle: &Circle, point: Vec2) -> Fx32 {
    let delta = point - closest_point_on_circle(circle, point);
    delta.length_squared()
}

pub fn closest_point_on_capsule(capsule: &Capsule, point: Vec2) -> Vec2 {
    let segment_point =
        closest_point_on_segment_points(capsule.segment.start, capsule.segment.end, point);
    let delta = point - segment_point;
    let distance_sq = delta.length_squared();
    let radius_sq = capsule.radius * capsule.radius;

    if distance_sq <= radius_sq {
        return point;
    }

    if let Some(snapped) = snap_axis_aligned_boundary_point2(segment_point, delta, capsule.radius) {
        return snapped;
    }

    let distance = sqrt_non_negative(distance_sq).expect("distance squared is non-negative");
    if distance == Fx32::ZERO {
        segment_point
    } else {
        segment_point + (delta * (capsule.radius / distance))
    }
}

pub fn distance_squared_point_capsule(capsule: &Capsule, point: Vec2) -> Fx32 {
    let delta = point - closest_point_on_capsule(capsule, point);
    delta.length_squared()
}

pub fn closest_point_on_convex_polygon<const N: usize>(
    polygon: &ConvexPolygon<N>,
    point: Vec2,
) -> Vec2 {
    closest_point_on_polygon_points(&polygon.points, point)
}

pub fn distance_squared_point_convex_polygon<const N: usize>(
    polygon: &ConvexPolygon<N>,
    point: Vec2,
) -> Fx32 {
    let delta = point - closest_point_on_convex_polygon(polygon, point);
    delta.length_squared()
}

pub fn closest_point_on_oriented_box(oriented_box: &OrientedBox, point: Vec2) -> Vec2 {
    let points = oriented_box_corners(oriented_box);
    closest_point_on_polygon_points(&points, point)
}

pub fn distance_squared_point_oriented_box(oriented_box: &OrientedBox, point: Vec2) -> Fx32 {
    let delta = point - closest_point_on_oriented_box(oriented_box, point);
    delta.length_squared()
}

pub fn circle_overlaps_aabb(circle: &Circle, aabb: &Aabb) -> bool {
    let radius_sq = circle.radius * circle.radius;
    distance_squared_point_aabb(aabb, circle.center) <= radius_sq
}

pub fn raycast_circle(ray: &Ray, circle: &Circle, max_toi: Fx32) -> Option<RayHit> {
    let offset = ray.origin - circle.center;
    let radius_sq = circle.radius * circle.radius;
    let c = offset.length_squared() - radius_sq;

    if c <= Fx32::ZERO {
        return Some(RayHit { toi: Fx32::ZERO, point: ray.origin, normal: Vec2::ZERO });
    }

    let a = ray.dir.length_squared();
    if a == Fx32::ZERO {
        return None;
    }

    let b = offset.dot(ray.dir);
    if b > Fx32::ZERO {
        return None;
    }

    let toi = smallest_non_negative_quadratic_root(a, b * Fx32::from_int(2), c)?;

    if toi < Fx32::ZERO || toi > max_toi {
        return None;
    }

    let point = ray.origin + (ray.dir * toi);
    let normal = if circle.radius == Fx32::ZERO {
        Vec2::ZERO
    } else {
        snap_axis_aligned_normal2((point - circle.center) * (Fx32::ONE / circle.radius))
    };

    Some(RayHit { toi, point, normal })
}

pub fn raycast_capsule(ray: &Ray, capsule: &Capsule, max_toi: Fx32) -> Option<RayHit> {
    if capsule_contains_point(capsule, ray.origin) {
        return Some(RayHit { toi: Fx32::ZERO, point: ray.origin, normal: Vec2::ZERO });
    }

    let mut best = raycast_capsule_side(ray, capsule, max_toi);
    let start_hit = raycast_circle(
        &Ray::new(ray.origin, ray.dir),
        &Circle { center: capsule.segment.start, radius: capsule.radius },
        max_toi,
    );
    best = pick_earlier_hit(best, start_hit);

    let end_hit = raycast_circle(
        &Ray::new(ray.origin, ray.dir),
        &Circle { center: capsule.segment.end, radius: capsule.radius },
        max_toi,
    );
    pick_earlier_hit(best, end_hit)
}

pub fn raycast_convex_polygon<const N: usize>(
    ray: &Ray,
    polygon: &ConvexPolygon<N>,
    max_toi: Fx32,
) -> Option<RayHit> {
    raycast_polygon_points(ray, &polygon.points, max_toi)
}

pub fn raycast_oriented_box(
    ray: &Ray,
    oriented_box: &OrientedBox,
    max_toi: Fx32,
) -> Option<RayHit> {
    let points = oriented_box_corners(oriented_box);
    raycast_polygon_points(ray, &points, max_toi)
}

pub fn sweep_circle_aabb(
    circle: &Circle,
    delta: Vec2,
    aabb: &Aabb,
    max_toi: Fx32,
) -> Option<RayHit> {
    if circle_overlaps_aabb(circle, aabb) {
        return Some(RayHit { toi: Fx32::ZERO, point: circle.center, normal: Vec2::ZERO });
    }

    let inflated = inflate_aabb(aabb, circle.radius);
    raycast_aabb(&Ray::new(circle.center, delta), &inflated, max_toi)
}

pub fn sweep_circle_capsule(
    circle: &Circle,
    delta: Vec2,
    capsule: &Capsule,
    max_toi: Fx32,
) -> Option<RayHit> {
    if circle_overlaps_capsule(circle, capsule) {
        return Some(RayHit { toi: Fx32::ZERO, point: circle.center, normal: Vec2::ZERO });
    }

    let expanded =
        Capsule::try_new(capsule.segment, capsule.radius + circle.radius).expect("valid capsule");
    raycast_capsule(&Ray::new(circle.center, delta), &expanded, max_toi)
}

pub fn sweep_circle_segment(
    circle: &Circle,
    delta: Vec2,
    segment: &Segment,
    max_toi: Fx32,
) -> Option<RayHit> {
    if circle_overlaps_segment(circle, segment) {
        return Some(RayHit { toi: Fx32::ZERO, point: circle.center, normal: Vec2::ZERO });
    }

    let expanded = Capsule::try_new(*segment, circle.radius).expect("valid capsule");
    raycast_capsule(&Ray::new(circle.center, delta), &expanded, max_toi)
}

pub fn sweep_circle_circle(
    circle: &Circle,
    delta: Vec2,
    other: &Circle,
    max_toi: Fx32,
) -> Option<RayHit> {
    let combined_radius = circle.radius + other.radius;
    let combined_radius_sq = combined_radius * combined_radius;
    if (circle.center - other.center).length_squared() <= combined_radius_sq {
        return Some(RayHit { toi: Fx32::ZERO, point: circle.center, normal: Vec2::ZERO });
    }

    let expanded = Circle::try_new(other.center, combined_radius).expect("valid circle");
    raycast_circle(&Ray::new(circle.center, delta), &expanded, max_toi)
}

pub fn sweep_circle_convex_polygon<const N: usize>(
    circle: &Circle,
    delta: Vec2,
    polygon: &ConvexPolygon<N>,
    max_toi: Fx32,
) -> Option<RayHit> {
    sweep_circle_polygon_points(circle, delta, &polygon.points, max_toi)
}

pub fn sweep_circle_oriented_box(
    circle: &Circle,
    delta: Vec2,
    oriented_box: &OrientedBox,
    max_toi: Fx32,
) -> Option<RayHit> {
    let points = oriented_box_corners(oriented_box);
    sweep_circle_polygon_points(circle, delta, &points, max_toi)
}

pub fn sweep_capsule_circle(
    capsule: &Capsule,
    delta: Vec2,
    circle: &Circle,
    max_toi: Fx32,
) -> Option<RayHit> {
    if circle_overlaps_capsule(circle, capsule) {
        return Some(RayHit {
            toi: Fx32::ZERO,
            point: capsule_midpoint(capsule),
            normal: Vec2::ZERO,
        });
    }

    let transformed_segment =
        Segment::new(circle.center - capsule.segment.start, circle.center - capsule.segment.end);
    let expanded = Capsule::try_new(transformed_segment, capsule.radius + circle.radius)
        .expect("valid capsule");
    let hit = raycast_capsule(&Ray::new(Vec2::ZERO, delta), &expanded, max_toi)?;
    Some(RayHit {
        toi: hit.toi,
        point: capsule_midpoint(capsule) + (delta * hit.toi),
        normal: hit.normal,
    })
}

pub fn sweep_capsule_segment(
    capsule: &Capsule,
    delta: Vec2,
    segment: &Segment,
    max_toi: Fx32,
) -> Option<RayHit> {
    if capsule_overlaps_segment(capsule, segment) {
        return Some(RayHit {
            toi: Fx32::ZERO,
            point: capsule_midpoint(capsule),
            normal: Vec2::ZERO,
        });
    }

    let points = segment_difference_quad(capsule.segment, *segment);
    let sweep = Circle::try_new(Vec2::ZERO, capsule.radius).expect("valid circle");
    let hit = sweep_circle_polygon_points(&sweep, delta, &points, max_toi)?;
    Some(RayHit {
        toi: hit.toi,
        point: capsule_midpoint(capsule) + (delta * hit.toi),
        normal: hit.normal,
    })
}

pub fn sweep_capsule_capsule(
    moving: &Capsule,
    delta: Vec2,
    target: &Capsule,
    max_toi: Fx32,
) -> Option<RayHit> {
    if capsule_overlaps_capsule(moving, target) {
        return Some(RayHit {
            toi: Fx32::ZERO,
            point: capsule_midpoint(moving),
            normal: Vec2::ZERO,
        });
    }

    let points = segment_difference_quad(moving.segment, target.segment);
    let sweep = Circle::try_new(Vec2::ZERO, moving.radius + target.radius).expect("valid circle");
    let hit = sweep_circle_polygon_points(&sweep, delta, &points, max_toi)?;
    Some(RayHit {
        toi: hit.toi,
        point: capsule_midpoint(moving) + (delta * hit.toi),
        normal: hit.normal,
    })
}

pub fn sweep_capsule_aabb(
    capsule: &Capsule,
    delta: Vec2,
    aabb: &Aabb,
    max_toi: Fx32,
) -> Option<RayHit> {
    let points = aabb_corners(aabb);
    sweep_capsule_polygon_points(capsule, delta, &points, max_toi)
}

pub fn sweep_capsule_convex_polygon<const N: usize>(
    capsule: &Capsule,
    delta: Vec2,
    polygon: &ConvexPolygon<N>,
    max_toi: Fx32,
) -> Option<RayHit> {
    sweep_capsule_polygon_points(capsule, delta, &polygon.points, max_toi)
}

pub fn sweep_capsule_oriented_box(
    capsule: &Capsule,
    delta: Vec2,
    oriented_box: &OrientedBox,
    max_toi: Fx32,
) -> Option<RayHit> {
    let points = oriented_box_corners(oriented_box);
    sweep_capsule_polygon_points(capsule, delta, &points, max_toi)
}

pub fn circle_overlaps_segment(circle: &Circle, segment: &Segment) -> bool {
    let radius_sq = circle.radius * circle.radius;
    distance_squared_point_segment_points(segment.start, segment.end, circle.center) <= radius_sq
}

pub fn circle_overlaps_capsule(circle: &Circle, capsule: &Capsule) -> bool {
    let radius = circle.radius + capsule.radius;
    let radius_sq = radius * radius;
    distance_squared_point_segment_points(capsule.segment.start, capsule.segment.end, circle.center)
        <= radius_sq
}

pub fn circle_overlaps_convex_polygon<const N: usize>(
    circle: &Circle,
    polygon: &ConvexPolygon<N>,
) -> bool {
    circle_overlaps_polygon_points(circle, &polygon.points)
}

pub fn capsule_contains_point(capsule: &Capsule, point: Vec2) -> bool {
    let radius_sq = capsule.radius * capsule.radius;
    distance_squared_point_segment_points(capsule.segment.start, capsule.segment.end, point)
        <= radius_sq
}

pub fn capsule_overlaps_capsule(a: &Capsule, b: &Capsule) -> bool {
    let radius = a.radius + b.radius;
    let radius_sq = radius * radius;
    distance_squared_segment_segment(a.segment.start, a.segment.end, b.segment.start, b.segment.end)
        <= radius_sq
}

pub fn capsule_overlaps_convex_polygon<const N: usize>(
    capsule: &Capsule,
    polygon: &ConvexPolygon<N>,
) -> bool {
    capsule_overlaps_polygon_points(capsule, &polygon.points)
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

pub fn convex_polygon_overlaps_convex_polygon<const N: usize, const M: usize>(
    a: &ConvexPolygon<N>,
    b: &ConvexPolygon<M>,
) -> bool {
    convex_polygons_overlap(&a.points, &b.points)
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

pub fn oriented_box_overlaps_oriented_box(a: &OrientedBox, b: &OrientedBox) -> bool {
    let a_points = oriented_box_corners(a);
    let b_points = oriented_box_corners(b);
    convex_polygons_overlap(&a_points, &b_points)
}

pub fn circle_overlaps_oriented_box(circle: &Circle, oriented_box: &OrientedBox) -> bool {
    let points = oriented_box_corners(oriented_box);
    circle_overlaps_polygon_points(circle, &points)
}

pub fn capsule_overlaps_aabb(capsule: &Capsule, aabb: &Aabb) -> bool {
    let points = aabb_corners(aabb);
    capsule_overlaps_polygon_points(capsule, &points)
}

pub fn capsule_overlaps_segment(capsule: &Capsule, segment: &Segment) -> bool {
    let radius_sq = capsule.radius * capsule.radius;
    distance_squared_segment_segment(
        capsule.segment.start,
        capsule.segment.end,
        segment.start,
        segment.end,
    ) <= radius_sq
}

pub fn capsule_overlaps_oriented_box(capsule: &Capsule, oriented_box: &OrientedBox) -> bool {
    let points = oriented_box_corners(oriented_box);
    capsule_overlaps_polygon_points(capsule, &points)
}

pub fn segment_intersects_segment(a: &Segment, b: &Segment) -> bool {
    segments_intersect(a.start, a.end, b.start, b.end)
}

pub fn convex_polygon_overlaps_aabb<const N: usize>(
    polygon: &ConvexPolygon<N>,
    aabb: &Aabb,
) -> bool {
    let points = aabb_corners(aabb);
    convex_polygons_overlap(&polygon.points, &points)
}

pub fn oriented_box_overlaps_aabb(oriented_box: &OrientedBox, aabb: &Aabb) -> bool {
    let oriented_box_points = oriented_box_corners(oriented_box);
    let aabb_points = aabb_corners(aabb);
    convex_polygons_overlap(&oriented_box_points, &aabb_points)
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

fn raycast_capsule_side(ray: &Ray, capsule: &Capsule, max_toi: Fx32) -> Option<RayHit> {
    let axis = capsule.segment.end - capsule.segment.start;
    let axis_length_sq = axis.length_squared();
    if axis_length_sq == Fx32::ZERO {
        return None;
    }

    let relative_origin = ray.origin - capsule.segment.start;
    let ray_axis_dot = ray.dir.dot(axis);
    let origin_axis_dot = relative_origin.dot(axis);
    let radius_sq = capsule.radius * capsule.radius;
    let quad_a = (axis_length_sq * ray.dir.length_squared()) - (ray_axis_dot * ray_axis_dot);
    let quad_b = ((axis_length_sq * relative_origin.dot(ray.dir))
        - (origin_axis_dot * ray_axis_dot))
        * Fx32::from_int(2);
    let quad_c = (axis_length_sq * relative_origin.length_squared())
        - (origin_axis_dot * origin_axis_dot)
        - (radius_sq * axis_length_sq);
    let toi = smallest_non_negative_quadratic_root(quad_a, quad_b, quad_c)?;
    if toi > max_toi {
        return None;
    }

    let axis_t = origin_axis_dot + (ray_axis_dot * toi);
    if axis_t < Fx32::ZERO || axis_t > axis_length_sq {
        return None;
    }

    let point = ray.origin + (ray.dir * toi);
    let closest = capsule.segment.start + (axis * (axis_t / axis_length_sq));
    let normal = if capsule.radius == Fx32::ZERO {
        Vec2::ZERO
    } else {
        snap_axis_aligned_normal2((point - closest) * (Fx32::ONE / capsule.radius))
    };

    Some(RayHit { toi, point, normal })
}

fn raycast_polygon_points(ray: &Ray, points: &[Vec2], max_toi: Fx32) -> Option<RayHit> {
    if convex_polygon_contains_slice(points, ray.origin) {
        return Some(RayHit { toi: Fx32::ZERO, point: ray.origin, normal: Vec2::ZERO });
    }

    let centroid = polygon_centroid(points);
    let mut t_enter = Fx32::ZERO;
    let mut t_exit = max_toi;
    let mut entering_normal = None;
    let mut index = 0usize;

    while index < points.len() {
        let start = points[index];
        let end = points[(index + 1) % points.len()];
        let edge = end - start;
        if edge == Vec2::ZERO {
            index += 1;
            continue;
        }

        let normal = outward_edge_normal(start, end, centroid);
        let plane_offset = normal.dot(start);
        let numer = plane_offset - normal.dot(ray.origin);
        let denom = normal.dot(ray.dir);

        if denom == Fx32::ZERO {
            if numer < Fx32::ZERO {
                return None;
            }
            index += 1;
            continue;
        }

        let t = numer / denom;
        if denom < Fx32::ZERO {
            if t > t_enter
                || (t == t_enter && preferred_normal(normal, entering_normal.unwrap_or(Vec2::ZERO)))
            {
                t_enter = t;
                entering_normal = Some(normal);
            }
        } else if t < t_exit {
            t_exit = t;
        }

        if t_enter > t_exit {
            return None;
        }

        index += 1;
    }

    if t_enter < Fx32::ZERO || t_enter > max_toi {
        return None;
    }

    Some(RayHit {
        toi: t_enter,
        point: ray.origin + (ray.dir * t_enter),
        normal: entering_normal.unwrap_or(Vec2::ZERO),
    })
}

fn closest_point_on_segment_points(start: Vec2, end: Vec2, point: Vec2) -> Vec2 {
    let delta = end - start;
    let length_sq = delta.length_squared();

    if length_sq == Fx32::ZERO {
        return start;
    }

    let projected = (point - start).dot(delta) / length_sq;
    let t = projected.max(Fx32::ZERO).min(Fx32::ONE);
    start + (delta * t)
}

fn distance_squared_point_segment_points(start: Vec2, end: Vec2, point: Vec2) -> Fx32 {
    let closest = closest_point_on_segment_points(start, end, point);
    (point - closest).length_squared()
}

fn distance_squared_segment_segment(
    a_start: Vec2,
    a_end: Vec2,
    b_start: Vec2,
    b_end: Vec2,
) -> Fx32 {
    if segments_intersect(a_start, a_end, b_start, b_end) {
        return Fx32::ZERO;
    }

    let a_to_b_start = distance_squared_point_segment_points(a_start, a_end, b_start);
    let a_to_b_end = distance_squared_point_segment_points(a_start, a_end, b_end);
    let b_to_a_start = distance_squared_point_segment_points(b_start, b_end, a_start);
    let b_to_a_end = distance_squared_point_segment_points(b_start, b_end, a_end);

    a_to_b_start.min(a_to_b_end).min(b_to_a_start).min(b_to_a_end)
}

fn segments_intersect(a_start: Vec2, a_end: Vec2, b_start: Vec2, b_end: Vec2) -> bool {
    let ab_start = orient2d(a_start, a_end, b_start);
    let ab_end = orient2d(a_start, a_end, b_end);
    let ba_start = orient2d(b_start, b_end, a_start);
    let ba_end = orient2d(b_start, b_end, a_end);

    if ab_start == 0 && point_on_segment(a_start, a_end, b_start) {
        return true;
    }
    if ab_end == 0 && point_on_segment(a_start, a_end, b_end) {
        return true;
    }
    if ba_start == 0 && point_on_segment(b_start, b_end, a_start) {
        return true;
    }
    if ba_end == 0 && point_on_segment(b_start, b_end, a_end) {
        return true;
    }

    orientation_signs_differ(ab_start, ab_end) && orientation_signs_differ(ba_start, ba_end)
}

fn convex_polygons_overlap(a_points: &[Vec2], b_points: &[Vec2]) -> bool {
    let mut tested_axis = false;

    if !polygon_axes_overlap(a_points, b_points, &mut tested_axis) {
        return false;
    }
    if !polygon_axes_overlap(b_points, a_points, &mut tested_axis) {
        return false;
    }

    tested_axis
}

fn circle_overlaps_polygon_points(circle: &Circle, points: &[Vec2]) -> bool {
    if convex_polygon_contains_slice(points, circle.center) {
        return true;
    }

    let radius_sq = circle.radius * circle.radius;
    let mut index = 0usize;

    while index < points.len() {
        let start = points[index];
        let end = points[(index + 1) % points.len()];

        if distance_squared_point_segment_points(start, end, circle.center) <= radius_sq {
            return true;
        }

        index += 1;
    }

    false
}

fn capsule_overlaps_polygon_points(capsule: &Capsule, points: &[Vec2]) -> bool {
    if circle_overlaps_polygon_points(
        &Circle { center: capsule.segment.start, radius: capsule.radius },
        points,
    ) {
        return true;
    }
    if circle_overlaps_polygon_points(
        &Circle { center: capsule.segment.end, radius: capsule.radius },
        points,
    ) {
        return true;
    }

    let radius_sq = capsule.radius * capsule.radius;
    let mut index = 0usize;

    while index < points.len() {
        let start = points[index];
        let end = points[(index + 1) % points.len()];

        if distance_squared_segment_segment(capsule.segment.start, capsule.segment.end, start, end)
            <= radius_sq
        {
            return true;
        }

        index += 1;
    }

    false
}

fn closest_point_on_polygon_points(points: &[Vec2], point: Vec2) -> Vec2 {
    if convex_polygon_contains_slice(points, point) {
        return point;
    }

    let mut best = closest_point_on_segment_points(points[0], points[1 % points.len()], point);
    let mut best_distance_sq = (point - best).length_squared();
    let mut index = 1usize;

    while index < points.len() {
        let start = points[index];
        let end = points[(index + 1) % points.len()];
        let candidate = closest_point_on_segment_points(start, end, point);
        let candidate_distance_sq = (point - candidate).length_squared();

        if candidate_distance_sq < best_distance_sq
            || (candidate_distance_sq == best_distance_sq && preferred_point(candidate, best))
        {
            best = candidate;
            best_distance_sq = candidate_distance_sq;
        }

        index += 1;
    }

    best
}

fn sweep_circle_polygon_points(
    circle: &Circle,
    delta: Vec2,
    points: &[Vec2],
    max_toi: Fx32,
) -> Option<RayHit> {
    if circle_overlaps_polygon_points(circle, points) {
        return Some(RayHit { toi: Fx32::ZERO, point: circle.center, normal: Vec2::ZERO });
    }

    let ray = Ray::new(circle.center, delta);
    let mut best = None;
    let mut index = 0usize;

    while index < points.len() {
        let start = points[index];
        let end = points[(index + 1) % points.len()];
        let edge =
            Capsule::try_new(Segment::new(start, end), circle.radius).expect("valid capsule");
        best = pick_preferred_hit(best, raycast_capsule(&ray, &edge, max_toi));
        index += 1;
    }

    best
}

fn sweep_capsule_polygon_points(
    capsule: &Capsule,
    delta: Vec2,
    points: &[Vec2],
    max_toi: Fx32,
) -> Option<RayHit> {
    if capsule_overlaps_polygon_points(capsule, points) {
        return Some(RayHit {
            toi: Fx32::ZERO,
            point: capsule_midpoint(capsule),
            normal: Vec2::ZERO,
        });
    }

    let mut best = None;
    let mut index = 0usize;

    while index < points.len() {
        let start = points[index];
        let end = points[(index + 1) % points.len()];
        let edge = Segment::new(start, end);
        best = pick_preferred_hit(best, sweep_capsule_segment(capsule, delta, &edge, max_toi));

        let vertex = Circle::try_new(start, Fx32::ZERO).expect("valid circle");
        best = pick_preferred_hit(best, sweep_capsule_circle(capsule, delta, &vertex, max_toi));
        index += 1;
    }

    best
}

fn capsule_midpoint(capsule: &Capsule) -> Vec2 {
    (capsule.segment.start + capsule.segment.end) * Fx32::from_ratio(1, 2)
}

fn segment_difference_quad(moving: Segment, target: Segment) -> [Vec2; 4] {
    [
        target.start - moving.start,
        target.end - moving.start,
        target.end - moving.end,
        target.start - moving.end,
    ]
}

fn convex_polygon_contains_slice(points: &[Vec2], point: Vec2) -> bool {
    let mut winding = 0i64;
    let mut index = 0usize;

    while index < points.len() {
        let start = points[index];
        let end = points[(index + 1) % points.len()];
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

fn polygon_centroid(points: &[Vec2]) -> Vec2 {
    let mut sum_x = 0i64;
    let mut sum_y = 0i64;

    for point in points {
        sum_x += point.x.raw() as i64;
        sum_y += point.y.raw() as i64;
    }

    Vec2::new(
        Fx32::from_raw((sum_x / points.len() as i64) as i32),
        Fx32::from_raw((sum_y / points.len() as i64) as i32),
    )
}

fn outward_edge_normal(start: Vec2, end: Vec2, centroid: Vec2) -> Vec2 {
    let edge = end - start;
    let midpoint = (start + end) * Fx32::from_ratio(1, 2);
    let mut normal = perpendicular(edge);

    if normal.dot(midpoint - centroid) < Fx32::ZERO {
        normal = -normal;
    }

    normalize_face_normal(normal)
}

fn polygon_axes_overlap(
    source_points: &[Vec2],
    other_points: &[Vec2],
    tested_axis: &mut bool,
) -> bool {
    let mut index = 0usize;

    while index < source_points.len() {
        let start = source_points[index];
        let end = source_points[(index + 1) % source_points.len()];
        let edge = end - start;

        if edge != Vec2::ZERO {
            if separated_on_axis(source_points, other_points, edge) {
                return false;
            }

            let normal = perpendicular(edge);
            if separated_on_axis(source_points, other_points, normal) {
                return false;
            }

            *tested_axis = true;
        }

        index += 1;
    }

    true
}

fn separated_on_axis(a_points: &[Vec2], b_points: &[Vec2], axis: Vec2) -> bool {
    if axis == Vec2::ZERO {
        return false;
    }

    let (a_min, a_max) = projection_bounds(a_points, axis);
    let (b_min, b_max) = projection_bounds(b_points, axis);

    a_max < b_min || b_max < a_min
}

fn projection_bounds(points: &[Vec2], axis: Vec2) -> (i128, i128) {
    let mut index = 1usize;
    let mut min = projection(axis, points[0]);
    let mut max = min;

    while index < points.len() {
        let value = projection(axis, points[index]);
        if value < min {
            min = value;
        }
        if value > max {
            max = value;
        }
        index += 1;
    }

    (min, max)
}

fn oriented_box_corners(oriented_box: &OrientedBox) -> [Vec2; 4] {
    let x_extent = oriented_box.axis_x * oriented_box.half_extents.x;
    let y_extent = oriented_box.axis_y * oriented_box.half_extents.y;

    [
        oriented_box.center - x_extent - y_extent,
        oriented_box.center + x_extent - y_extent,
        oriented_box.center + x_extent + y_extent,
        oriented_box.center - x_extent + y_extent,
    ]
}

fn aabb_corners(aabb: &Aabb) -> [Vec2; 4] {
    [aabb.min, Vec2::new(aabb.max.x, aabb.min.y), aabb.max, Vec2::new(aabb.min.x, aabb.max.y)]
}

fn inflate_aabb(aabb: &Aabb, amount: Fx32) -> Aabb {
    Aabb::try_new(
        Vec2::new(aabb.min.x - amount, aabb.min.y - amount),
        Vec2::new(aabb.max.x + amount, aabb.max.y + amount),
    )
    .expect("inflated aabb remains valid")
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

fn orientation_signs_differ(lhs: i64, rhs: i64) -> bool {
    (lhs > 0 && rhs < 0) || (lhs < 0 && rhs > 0)
}

fn perpendicular(value: Vec2) -> Vec2 {
    Vec2::new(-value.y, value.x)
}

fn projection(axis: Vec2, point: Vec2) -> i128 {
    (axis.x.raw() as i128 * point.x.raw() as i128) + (axis.y.raw() as i128 * point.y.raw() as i128)
}

fn preferred_normal(candidate: Vec2, current: Vec2) -> bool {
    candidate.x < current.x || (candidate.x == current.x && candidate.y < current.y)
}

fn preferred_point(candidate: Vec2, current: Vec2) -> bool {
    candidate.x < current.x || (candidate.x == current.x && candidate.y < current.y)
}

fn normalize_face_normal(normal: Vec2) -> Vec2 {
    let scale = normal.x.abs().max(normal.y.abs());
    if scale == Fx32::ZERO {
        Vec2::ZERO
    } else {
        normal * (Fx32::ONE / scale)
    }
}

fn snap_axis_aligned_normal2(normal: Vec2) -> Vec2 {
    if normal.y == Fx32::ZERO && normal.x != Fx32::ZERO {
        Vec2::new(Fx32::from_int(normal.x.signum()), Fx32::ZERO)
    } else if normal.x == Fx32::ZERO && normal.y != Fx32::ZERO {
        Vec2::new(Fx32::ZERO, Fx32::from_int(normal.y.signum()))
    } else {
        normal
    }
}

fn snap_axis_aligned_boundary_point2(origin: Vec2, delta: Vec2, radius: Fx32) -> Option<Vec2> {
    if delta.y == Fx32::ZERO && delta.x != Fx32::ZERO {
        Some(origin + Vec2::new(Fx32::from_int(delta.x.signum()) * radius, Fx32::ZERO))
    } else if delta.x == Fx32::ZERO && delta.y != Fx32::ZERO {
        Some(origin + Vec2::new(Fx32::ZERO, Fx32::from_int(delta.y.signum()) * radius))
    } else {
        None
    }
}

fn pick_earlier_hit(current: Option<RayHit>, candidate: Option<RayHit>) -> Option<RayHit> {
    match (current, candidate) {
        (None, None) => None,
        (Some(hit), None) | (None, Some(hit)) => Some(hit),
        (Some(current_hit), Some(candidate_hit)) => {
            if candidate_hit.toi < current_hit.toi {
                Some(candidate_hit)
            } else {
                Some(current_hit)
            }
        }
    }
}

fn pick_preferred_hit(current: Option<RayHit>, candidate: Option<RayHit>) -> Option<RayHit> {
    match (current, candidate) {
        (None, None) => None,
        (Some(hit), None) | (None, Some(hit)) => Some(hit),
        (Some(current_hit), Some(candidate_hit)) => {
            if candidate_hit.toi < current_hit.toi
                || (candidate_hit.toi == current_hit.toi
                    && preferred_normal(candidate_hit.normal, current_hit.normal))
            {
                Some(candidate_hit)
            } else {
                Some(current_hit)
            }
        }
    }
}
