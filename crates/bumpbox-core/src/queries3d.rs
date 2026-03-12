use crate::{
    scalar::{smallest_non_negative_quadratic_root, sqrt_non_negative},
    Aabb3, Capsule3, Fx32, Ray3, Segment3, Sphere, Triangle3, Vec3,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RayHit3 {
    pub toi: Fx32,
    pub point: Vec3,
    pub normal: Vec3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClosestPoints3 {
    pub segment_point: Vec3,
    pub triangle_point: Vec3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClosestPair3 {
    pub point_a: Vec3,
    pub point_b: Vec3,
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

pub fn closest_point_on_segment3(segment: &Segment3, point: Vec3) -> Vec3 {
    closest_point_on_segment3_points(segment.start, segment.end, point)
}

pub fn distance_squared_point_segment3(segment: &Segment3, point: Vec3) -> Fx32 {
    let closest = closest_point_on_segment3(segment, point);
    let delta = point - closest;
    delta.length_squared()
}

pub fn closest_points_segment3_triangle3(
    segment: &Segment3,
    triangle: &Triangle3,
) -> ClosestPoints3 {
    closest_points_segment_triangle3(*segment, triangle)
}

pub fn distance_squared_segment3_triangle3(segment: &Segment3, triangle: &Triangle3) -> Fx32 {
    let closest = closest_points_segment3_triangle3(segment, triangle);
    (closest.segment_point - closest.triangle_point).length_squared()
}

pub fn closest_point_on_sphere(sphere: &Sphere, point: Vec3) -> Vec3 {
    closest_point_on_round_anchor3(sphere.center, sphere.radius, point)
}

pub fn distance_squared_point_sphere(sphere: &Sphere, point: Vec3) -> Fx32 {
    let delta = point - closest_point_on_sphere(sphere, point);
    delta.length_squared()
}

pub fn closest_point_on_capsule3(capsule: &Capsule3, point: Vec3) -> Vec3 {
    let segment_point =
        closest_point_on_segment3_points(capsule.segment.start, capsule.segment.end, point);
    closest_point_on_round_anchor3(segment_point, capsule.radius, point)
}

pub fn distance_squared_point_capsule3(capsule: &Capsule3, point: Vec3) -> Fx32 {
    let delta = point - closest_point_on_capsule3(capsule, point);
    delta.length_squared()
}

pub fn closest_point_on_triangle3(triangle: &Triangle3, point: Vec3) -> Vec3 {
    let ab = triangle.b - triangle.a;
    let ac = triangle.c - triangle.a;
    let normal = cross3(ab, ac);
    if normal.length_squared() == Fx32::ZERO {
        return closest_point_on_triangle3_degenerate(triangle, point);
    }

    let ap = point - triangle.a;
    let d1 = ab.dot(ap);
    let d2 = ac.dot(ap);
    if d1 <= Fx32::ZERO && d2 <= Fx32::ZERO {
        return triangle.a;
    }

    let bp = point - triangle.b;
    let d3 = ab.dot(bp);
    let d4 = ac.dot(bp);
    if d3 >= Fx32::ZERO && d4 <= d3 {
        return triangle.b;
    }

    let vc = (d1 * d4) - (d3 * d2);
    if vc <= Fx32::ZERO && d1 >= Fx32::ZERO && d3 <= Fx32::ZERO {
        let v = d1 / (d1 - d3);
        return triangle.a + (ab * v);
    }

    let cp = point - triangle.c;
    let d5 = ab.dot(cp);
    let d6 = ac.dot(cp);
    if d6 >= Fx32::ZERO && d5 <= d6 {
        return triangle.c;
    }

    let vb = (d5 * d2) - (d1 * d6);
    if vb <= Fx32::ZERO && d2 >= Fx32::ZERO && d6 <= Fx32::ZERO {
        let w = d2 / (d2 - d6);
        return triangle.a + (ac * w);
    }

    let bc = triangle.c - triangle.b;
    let va = (d3 * d6) - (d5 * d4);
    if va <= Fx32::ZERO && (d4 - d3) >= Fx32::ZERO && (d5 - d6) >= Fx32::ZERO {
        let w = (d4 - d3) / ((d4 - d3) + (d5 - d6));
        return triangle.b + (bc * w);
    }

    let denom = va + vb + vc;
    if denom == Fx32::ZERO {
        return closest_point_on_triangle3_degenerate(triangle, point);
    }

    let v = vb / denom;
    let w = vc / denom;
    triangle.a + (ab * v) + (ac * w)
}

pub fn distance_squared_point_triangle3(triangle: &Triangle3, point: Vec3) -> Fx32 {
    let delta = point - closest_point_on_triangle3(triangle, point);
    delta.length_squared()
}

pub fn closest_points_triangle3_triangle3(a: &Triangle3, b: &Triangle3) -> ClosestPair3 {
    let mut best = None;

    for edge in triangle3_edges(*a) {
        let candidate = closest_points_segment_triangle3(edge, b);
        best = pick_preferred_closest_pair3(
            best,
            Some(ClosestPair3 {
                point_a: candidate.segment_point,
                point_b: candidate.triangle_point,
            }),
        );
    }

    for edge in triangle3_edges(*b) {
        let candidate = closest_points_segment_triangle3(edge, a);
        best = pick_preferred_closest_pair3(
            best,
            Some(ClosestPair3 {
                point_a: candidate.triangle_point,
                point_b: candidate.segment_point,
            }),
        );
    }

    best.expect("triangle edge candidates exist")
}

pub fn closest_points_sphere_triangle3(sphere: &Sphere, triangle: &Triangle3) -> ClosestPair3 {
    let triangle_point = closest_point_on_triangle3(triangle, sphere.center);
    ClosestPair3 {
        point_a: closest_point_on_round_anchor3(sphere.center, sphere.radius, triangle_point),
        point_b: triangle_point,
    }
}

pub fn distance_squared_sphere_triangle3(sphere: &Sphere, triangle: &Triangle3) -> Fx32 {
    let closest = closest_points_sphere_triangle3(sphere, triangle);
    (closest.point_a - closest.point_b).length_squared()
}

pub fn closest_points_capsule3_triangle3(capsule: &Capsule3, triangle: &Triangle3) -> ClosestPair3 {
    let closest = closest_points_segment3_triangle3(&capsule.segment, triangle);
    ClosestPair3 {
        point_a: closest_point_on_round_anchor3(
            closest.segment_point,
            capsule.radius,
            closest.triangle_point,
        ),
        point_b: closest.triangle_point,
    }
}

pub fn distance_squared_capsule3_triangle3(capsule: &Capsule3, triangle: &Triangle3) -> Fx32 {
    let closest = closest_points_capsule3_triangle3(capsule, triangle);
    (closest.point_a - closest.point_b).length_squared()
}

pub fn closest_points_triangle3_aabb3(triangle: &Triangle3, aabb: &Aabb3) -> ClosestPair3 {
    let mut best = None;
    let mut best_face_interior = None;
    let triangle_normal = cross3(triangle.b - triangle.a, triangle.c - triangle.a);

    for edge in triangle3_edges(*triangle) {
        let candidate = closest_points_segment_aabb3(aabb, edge.start, edge.end);
        best = pick_preferred_closest_pair3(
            best,
            Some(ClosestPair3 {
                point_a: candidate.segment_point,
                point_b: candidate.triangle_point,
            }),
        );
    }

    for edge in aabb3_edges(*aabb) {
        let candidate = closest_points_segment_triangle3(edge, triangle);
        best = pick_preferred_closest_pair3(
            best,
            Some(ClosestPair3 {
                point_a: candidate.triangle_point,
                point_b: candidate.segment_point,
            }),
        );
    }

    for (face_center, axis) in aabb3_face_centers(*aabb) {
        let triangle_point = closest_point_on_triangle3(triangle, face_center);
        if point_in_triangle3_strict(triangle_point, triangle)
            && point_matches_face_axis3(triangle_point, face_center, axis)
            && aabb3_face_projects_inside_triangle(
                *aabb,
                face_center,
                axis,
                triangle,
                triangle_point,
                triangle_normal,
            )
        {
            best_face_interior = pick_preferred_closest_pair3(
                best_face_interior,
                Some(ClosestPair3 { point_a: triangle_point, point_b: face_center }),
            );
        }
    }

    for face in aabb3_face_triangles(*aabb) {
        let candidate = closest_points_triangle3_triangle3(triangle, &face);
        best = pick_preferred_closest_pair3(
            best,
            Some(ClosestPair3 { point_a: candidate.point_a, point_b: candidate.point_b }),
        );
    }

    let best = best.expect("triangle and aabb edge candidates exist");
    match best_face_interior {
        Some(face_candidate) => {
            let best_distance = (best.point_a - best.point_b).length_squared();
            let face_distance = (face_candidate.point_a - face_candidate.point_b).length_squared();
            if face_distance > Fx32::ZERO
                && (face_distance < best_distance
                    || (face_distance == best_distance
                        && aabb3_face_projects_inside_triangle(
                            *aabb,
                            face_candidate.point_b,
                            shared_axis3(face_candidate.point_a, face_candidate.point_b)
                                .expect("face candidate matches one face axis"),
                            triangle,
                            face_candidate.point_a,
                            triangle_normal,
                        )))
            {
                face_candidate
            } else {
                best
            }
        }
        None => best,
    }
}

pub fn distance_squared_triangle3_aabb3(triangle: &Triangle3, aabb: &Aabb3) -> Fx32 {
    let closest = closest_points_triangle3_aabb3(triangle, aabb);
    (closest.point_a - closest.point_b).length_squared()
}

pub fn distance_squared_triangle3_triangle3(a: &Triangle3, b: &Triangle3) -> Fx32 {
    let closest = closest_points_triangle3_triangle3(a, b);
    (closest.point_a - closest.point_b).length_squared()
}

pub fn sphere_overlaps_aabb3(sphere: &Sphere, aabb: &Aabb3) -> bool {
    let radius_sq = sphere.radius * sphere.radius;
    distance_squared_point_aabb3(aabb, sphere.center) <= radius_sq
}

pub fn raycast_sphere(ray: &Ray3, sphere: &Sphere, max_toi: Fx32) -> Option<RayHit3> {
    let offset = ray.origin - sphere.center;
    let radius_sq = sphere.radius * sphere.radius;
    let c = offset.length_squared() - radius_sq;

    if c <= Fx32::ZERO {
        return Some(RayHit3 { toi: Fx32::ZERO, point: ray.origin, normal: Vec3::ZERO });
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
    let normal = if sphere.radius == Fx32::ZERO {
        Vec3::ZERO
    } else {
        snap_axis_aligned_normal3((point - sphere.center) * (Fx32::ONE / sphere.radius))
    };

    Some(RayHit3 { toi, point, normal })
}

pub fn raycast_capsule3(ray: &Ray3, capsule: &Capsule3, max_toi: Fx32) -> Option<RayHit3> {
    if capsule3_contains_point(capsule, ray.origin) {
        return Some(RayHit3 { toi: Fx32::ZERO, point: ray.origin, normal: Vec3::ZERO });
    }

    let mut best = raycast_capsule3_side(ray, capsule, max_toi);
    let start_hit = raycast_sphere(
        &Ray3::new(ray.origin, ray.dir),
        &Sphere { center: capsule.segment.start, radius: capsule.radius },
        max_toi,
    );
    best = pick_earlier_hit3(best, start_hit);

    let end_hit = raycast_sphere(
        &Ray3::new(ray.origin, ray.dir),
        &Sphere { center: capsule.segment.end, radius: capsule.radius },
        max_toi,
    );
    pick_earlier_hit3(best, end_hit)
}

pub fn raycast_triangle3(ray: &Ray3, triangle: &Triangle3, max_toi: Fx32) -> Option<RayHit3> {
    if point_on_triangle3(triangle, ray.origin) {
        return Some(RayHit3 { toi: Fx32::ZERO, point: ray.origin, normal: Vec3::ZERO });
    }

    let ab = triangle.b - triangle.a;
    let ac = triangle.c - triangle.a;
    let normal = cross3(ab, ac);
    if normal.length_squared() == Fx32::ZERO {
        return None;
    }

    let denom = normal.dot(ray.dir);
    if denom == Fx32::ZERO {
        return None;
    }

    let toi = normal.dot(triangle.a - ray.origin) / denom;
    if toi < Fx32::ZERO || toi > max_toi {
        return None;
    }

    let point = ray.origin + (ray.dir * toi);
    if !point_in_triangle3(point, triangle, normal) {
        return None;
    }

    let face_normal = normalize_face_normal3(normal);
    let normal = if denom < Fx32::ZERO { face_normal } else { -face_normal };
    Some(RayHit3 { toi, point, normal })
}

pub fn sweep_sphere_aabb3(
    sphere: &Sphere,
    delta: Vec3,
    aabb: &Aabb3,
    max_toi: Fx32,
) -> Option<RayHit3> {
    if sphere_overlaps_aabb3(sphere, aabb) {
        return Some(RayHit3 { toi: Fx32::ZERO, point: sphere.center, normal: Vec3::ZERO });
    }

    let inflated = inflate_aabb3(aabb, sphere.radius);
    raycast_aabb3(&Ray3::new(sphere.center, delta), &inflated, max_toi)
}

pub fn sweep_sphere_capsule3(
    sphere: &Sphere,
    delta: Vec3,
    capsule: &Capsule3,
    max_toi: Fx32,
) -> Option<RayHit3> {
    if sphere_overlaps_capsule3(sphere, capsule) {
        return Some(RayHit3 { toi: Fx32::ZERO, point: sphere.center, normal: Vec3::ZERO });
    }

    let expanded =
        Capsule3::try_new(capsule.segment, capsule.radius + sphere.radius).expect("valid capsule3");
    raycast_capsule3(&Ray3::new(sphere.center, delta), &expanded, max_toi)
}

pub fn sweep_sphere_segment3(
    sphere: &Sphere,
    delta: Vec3,
    segment: &Segment3,
    max_toi: Fx32,
) -> Option<RayHit3> {
    if sphere_overlaps_segment3(sphere, segment) {
        return Some(RayHit3 { toi: Fx32::ZERO, point: sphere.center, normal: Vec3::ZERO });
    }

    let expanded = Capsule3::try_new(*segment, sphere.radius).expect("valid capsule3");
    raycast_capsule3(&Ray3::new(sphere.center, delta), &expanded, max_toi)
}

pub fn sweep_sphere_sphere(
    sphere: &Sphere,
    delta: Vec3,
    other: &Sphere,
    max_toi: Fx32,
) -> Option<RayHit3> {
    if sphere_overlaps_sphere(sphere, other) {
        return Some(RayHit3 { toi: Fx32::ZERO, point: sphere.center, normal: Vec3::ZERO });
    }

    let expanded =
        Sphere::try_new(other.center, other.radius + sphere.radius).expect("valid sphere");
    raycast_sphere(&Ray3::new(sphere.center, delta), &expanded, max_toi)
}

pub fn sweep_sphere_triangle3(
    sphere: &Sphere,
    delta: Vec3,
    triangle: &Triangle3,
    max_toi: Fx32,
) -> Option<RayHit3> {
    if sphere_overlaps_triangle3(sphere, triangle) {
        return Some(RayHit3 { toi: Fx32::ZERO, point: sphere.center, normal: Vec3::ZERO });
    }

    let mut best =
        raycast_sphere_triangle3_face(sphere.center, delta, sphere.radius, triangle, max_toi);
    for edge in triangle3_edges(*triangle) {
        best = pick_preferred_hit3(best, sweep_sphere_segment3(sphere, delta, &edge, max_toi));
    }
    best
}

pub fn sweep_capsule3_triangle3(
    capsule: &Capsule3,
    delta: Vec3,
    triangle: &Triangle3,
    max_toi: Fx32,
) -> Option<RayHit3> {
    if capsule3_overlaps_triangle3(capsule, triangle) {
        return Some(RayHit3 {
            toi: Fx32::ZERO,
            point: capsule3_midpoint(capsule),
            normal: Vec3::ZERO,
        });
    }

    let prism = segment3_triangle3_difference_prism(capsule.segment, *triangle);
    let sweep = Sphere::try_new(Vec3::ZERO, capsule.radius).expect("valid sphere");
    let hit = sweep_sphere_prism3(&sweep, delta, &prism, max_toi)?;
    Some(RayHit3 {
        toi: hit.toi,
        point: capsule3_midpoint(capsule) + (delta * hit.toi),
        normal: hit.normal,
    })
}

pub fn sweep_segment3_triangle3(
    segment: &Segment3,
    delta: Vec3,
    triangle: &Triangle3,
    max_toi: Fx32,
) -> Option<RayHit3> {
    let capsule = Capsule3::try_new(*segment, Fx32::ZERO).expect("valid capsule3");
    sweep_capsule3_triangle3(&capsule, delta, triangle, max_toi)
}

pub fn sweep_triangle3_triangle3(
    triangle: &Triangle3,
    delta: Vec3,
    other: &Triangle3,
    max_toi: Fx32,
) -> Option<RayHit3> {
    let reference = triangle3_reference_point(*triangle);
    if triangle3_overlaps_triangle3(triangle, other) {
        return Some(RayHit3 { toi: Fx32::ZERO, point: reference, normal: Vec3::ZERO });
    }

    let mut best = None;
    for edge in triangle3_edges(*triangle) {
        let candidate = sweep_segment3_triangle3(&edge, delta, other, max_toi).map(|hit| RayHit3 {
            toi: hit.toi,
            point: reference + (delta * hit.toi),
            normal: hit.normal,
        });
        best = pick_preferred_hit3(best, candidate);
    }

    for edge in triangle3_edges(*other) {
        let candidate =
            sweep_segment3_triangle3(&edge, -delta, triangle, max_toi).map(|hit| RayHit3 {
                toi: hit.toi,
                point: reference + (delta * hit.toi),
                normal: if hit.normal == Vec3::ZERO { Vec3::ZERO } else { -hit.normal },
            });
        best = pick_preferred_hit3(best, candidate);
    }

    best
}

pub fn sweep_capsule3_sphere(
    capsule: &Capsule3,
    delta: Vec3,
    sphere: &Sphere,
    max_toi: Fx32,
) -> Option<RayHit3> {
    if sphere_overlaps_capsule3(sphere, capsule) {
        return Some(RayHit3 {
            toi: Fx32::ZERO,
            point: capsule3_midpoint(capsule),
            normal: Vec3::ZERO,
        });
    }

    let transformed_segment =
        Segment3::new(sphere.center - capsule.segment.start, sphere.center - capsule.segment.end);
    let expanded = Capsule3::try_new(transformed_segment, capsule.radius + sphere.radius)
        .expect("valid capsule3");
    let hit = raycast_capsule3(&Ray3::new(Vec3::ZERO, delta), &expanded, max_toi)?;
    Some(RayHit3 {
        toi: hit.toi,
        point: capsule3_midpoint(capsule) + (delta * hit.toi),
        normal: hit.normal,
    })
}

pub fn sweep_capsule3_aabb3(
    capsule: &Capsule3,
    delta: Vec3,
    aabb: &Aabb3,
    max_toi: Fx32,
) -> Option<RayHit3> {
    if capsule3_overlaps_aabb3(capsule, aabb) {
        return Some(RayHit3 {
            toi: Fx32::ZERO,
            point: capsule3_midpoint(capsule),
            normal: Vec3::ZERO,
        });
    }

    let inflated = inflate_aabb3(aabb, capsule.radius);
    let hit = sweep_segment_aabb3(capsule.segment, delta, &inflated, max_toi)?;
    Some(RayHit3 {
        toi: hit.toi,
        point: capsule3_midpoint(capsule) + (delta * hit.toi),
        normal: hit.normal,
    })
}

pub fn sweep_segment3_aabb3(
    segment: &Segment3,
    delta: Vec3,
    aabb: &Aabb3,
    max_toi: Fx32,
) -> Option<RayHit3> {
    sweep_segment_aabb3(*segment, delta, aabb, max_toi)
}

pub fn sweep_capsule3_segment3(
    capsule: &Capsule3,
    delta: Vec3,
    segment: &Segment3,
    max_toi: Fx32,
) -> Option<RayHit3> {
    if sphere_overlaps_capsule3(
        &Sphere { center: capsule.segment.start, radius: capsule.radius },
        &Capsule3::try_new(*segment, Fx32::ZERO).expect("valid capsule3"),
    ) || sphere_overlaps_capsule3(
        &Sphere { center: capsule.segment.end, radius: capsule.radius },
        &Capsule3::try_new(*segment, Fx32::ZERO).expect("valid capsule3"),
    ) || distance_squared_segment_segment3(
        capsule.segment.start,
        capsule.segment.end,
        segment.start,
        segment.end,
    ) <= (capsule.radius * capsule.radius)
    {
        return Some(RayHit3 {
            toi: Fx32::ZERO,
            point: capsule3_midpoint(capsule),
            normal: Vec3::ZERO,
        });
    }

    let points = segment3_difference_quad(capsule.segment, *segment);
    let sweep = Sphere::try_new(Vec3::ZERO, capsule.radius).expect("valid sphere");
    let hit = sweep_sphere_quad3_points(&sweep, delta, &points, max_toi)?;
    Some(RayHit3 {
        toi: hit.toi,
        point: capsule3_midpoint(capsule) + (delta * hit.toi),
        normal: hit.normal,
    })
}

pub fn sweep_capsule3_capsule3(
    moving: &Capsule3,
    delta: Vec3,
    target: &Capsule3,
    max_toi: Fx32,
) -> Option<RayHit3> {
    if capsule3_overlaps_capsule3(moving, target) {
        return Some(RayHit3 {
            toi: Fx32::ZERO,
            point: capsule3_midpoint(moving),
            normal: Vec3::ZERO,
        });
    }

    let points = segment3_difference_quad(moving.segment, target.segment);
    let sweep = Sphere::try_new(Vec3::ZERO, moving.radius + target.radius).expect("valid sphere");
    let hit = sweep_sphere_quad3_points(&sweep, delta, &points, max_toi)?;
    Some(RayHit3 {
        toi: hit.toi,
        point: capsule3_midpoint(moving) + (delta * hit.toi),
        normal: hit.normal,
    })
}

pub fn segment3_overlaps_aabb3(segment: &Segment3, aabb: &Aabb3) -> bool {
    let delta = segment.end - segment.start;
    if delta.length_squared() == Fx32::ZERO {
        return aabb.contains_point(segment.start);
    }

    raycast_aabb3(&Ray3::new(segment.start, delta), aabb, Fx32::ONE).is_some()
}

pub fn sphere_overlaps_sphere(a: &Sphere, b: &Sphere) -> bool {
    let radius = a.radius + b.radius;
    let radius_sq = radius * radius;
    (a.center - b.center).length_squared() <= radius_sq
}

pub fn capsule3_contains_point(capsule: &Capsule3, point: Vec3) -> bool {
    let radius_sq = capsule.radius * capsule.radius;
    distance_squared_point_segment3_points(capsule.segment.start, capsule.segment.end, point)
        <= radius_sq
}

pub fn sphere_overlaps_capsule3(sphere: &Sphere, capsule: &Capsule3) -> bool {
    let combined_radius = sphere.radius + capsule.radius;
    let radius_sq = combined_radius * combined_radius;
    distance_squared_point_segment3_points(
        capsule.segment.start,
        capsule.segment.end,
        sphere.center,
    ) <= radius_sq
}

pub fn sphere_overlaps_triangle3(sphere: &Sphere, triangle: &Triangle3) -> bool {
    let radius_sq = sphere.radius * sphere.radius;
    distance_squared_point_triangle3(triangle, sphere.center) <= radius_sq
}

pub fn sphere_overlaps_segment3(sphere: &Sphere, segment: &Segment3) -> bool {
    let radius_sq = sphere.radius * sphere.radius;
    distance_squared_point_segment3_points(segment.start, segment.end, sphere.center) <= radius_sq
}

pub fn capsule3_overlaps_capsule3(a: &Capsule3, b: &Capsule3) -> bool {
    let radius = a.radius + b.radius;
    let radius_sq = radius * radius;
    distance_squared_segment_segment3(
        a.segment.start,
        a.segment.end,
        b.segment.start,
        b.segment.end,
    ) <= radius_sq
}

pub fn capsule3_overlaps_triangle3(capsule: &Capsule3, triangle: &Triangle3) -> bool {
    let radius_sq = capsule.radius * capsule.radius;
    distance_squared_segment3_triangle3(&capsule.segment, triangle) <= radius_sq
}

pub fn segment3_overlaps_triangle3(segment: &Segment3, triangle: &Triangle3) -> bool {
    distance_squared_segment3_triangle3(segment, triangle) == Fx32::ZERO
}

pub fn triangle3_overlaps_triangle3(a: &Triangle3, b: &Triangle3) -> bool {
    distance_squared_triangle3_triangle3(a, b) == Fx32::ZERO
}

pub fn capsule3_overlaps_aabb3(capsule: &Capsule3, aabb: &Aabb3) -> bool {
    let radius_sq = capsule.radius * capsule.radius;
    distance_squared_segment_aabb3(aabb, capsule.segment.start, capsule.segment.end) <= radius_sq
}

pub fn triangle3_overlaps_aabb3(triangle: &Triangle3, aabb: &Aabb3) -> bool {
    if aabb.contains_point(triangle.a)
        || aabb.contains_point(triangle.b)
        || aabb.contains_point(triangle.c)
    {
        return true;
    }

    let edges = triangle3_edges(*triangle);
    if edges.iter().any(|edge| segment3_overlaps_aabb3(edge, aabb)) {
        return true;
    }

    let normal = cross3(triangle.b - triangle.a, triangle.c - triangle.a);
    if normal.length_squared() == Fx32::ZERO {
        return false;
    }

    if aabb3_corners(*aabb).iter().copied().any(|corner| point_on_triangle3(triangle, corner)) {
        return true;
    }

    aabb3_edges(*aabb).iter().any(|edge| segment_intersects_triangle3(*edge, triangle))
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

fn raycast_capsule3_side(ray: &Ray3, capsule: &Capsule3, max_toi: Fx32) -> Option<RayHit3> {
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
        Vec3::ZERO
    } else {
        snap_axis_aligned_normal3((point - closest) * (Fx32::ONE / capsule.radius))
    };

    Some(RayHit3 { toi, point, normal })
}

fn closest_point_on_segment3_points(start: Vec3, end: Vec3, point: Vec3) -> Vec3 {
    let delta = end - start;
    let length_sq = delta.length_squared();

    if length_sq == Fx32::ZERO {
        return start;
    }

    let projected = (point - start).dot(delta) / length_sq;
    let t = projected.max(Fx32::ZERO).min(Fx32::ONE);
    start + (delta * t)
}

fn distance_squared_point_segment3_points(start: Vec3, end: Vec3, point: Vec3) -> Fx32 {
    let closest = closest_point_on_segment3_points(start, end, point);
    (point - closest).length_squared()
}

fn closest_points_segment_aabb3(aabb: &Aabb3, start: Vec3, end: Vec3) -> ClosestPoints3 {
    if aabb.contains_point(start) {
        return ClosestPoints3 { segment_point: start, triangle_point: start };
    }

    let delta = end - start;
    if delta.length_squared() == Fx32::ZERO {
        return ClosestPoints3 {
            segment_point: start,
            triangle_point: closest_point_on_aabb3(aabb, start),
        };
    }

    if let Some(hit) = raycast_aabb3(&Ray3::new(start, delta), aabb, Fx32::ONE) {
        return ClosestPoints3 { segment_point: hit.point, triangle_point: hit.point };
    }

    let mut breakpoints = [Fx32::ZERO; 8];
    let mut breakpoint_count = 0usize;

    push_breakpoint(&mut breakpoints, &mut breakpoint_count, Fx32::ZERO);
    push_breakpoint(&mut breakpoints, &mut breakpoint_count, Fx32::ONE);
    axis_breakpoints(
        start.x,
        delta.x,
        aabb.min.x,
        aabb.max.x,
        &mut breakpoints,
        &mut breakpoint_count,
    );
    axis_breakpoints(
        start.y,
        delta.y,
        aabb.min.y,
        aabb.max.y,
        &mut breakpoints,
        &mut breakpoint_count,
    );
    axis_breakpoints(
        start.z,
        delta.z,
        aabb.min.z,
        aabb.max.z,
        &mut breakpoints,
        &mut breakpoint_count,
    );

    let mut best = ClosestPoints3 {
        segment_point: start,
        triangle_point: closest_point_on_aabb3(aabb, start),
    };
    best = pick_preferred_closest_points3(
        Some(best),
        Some(ClosestPoints3 {
            segment_point: end,
            triangle_point: closest_point_on_aabb3(aabb, end),
        }),
    )
    .expect("endpoint candidates exist");

    let two = Fx32::from_int(2);
    let mut index = 0usize;

    while index + 1 < breakpoint_count {
        let t0 = breakpoints[index];
        let t1 = breakpoints[index + 1];

        let point0 = start + (delta * t0);
        best = pick_preferred_closest_points3(
            Some(best),
            Some(ClosestPoints3 {
                segment_point: point0,
                triangle_point: closest_point_on_aabb3(aabb, point0),
            }),
        )
        .expect("candidate exists");

        let point1 = start + (delta * t1);
        best = pick_preferred_closest_points3(
            Some(best),
            Some(ClosestPoints3 {
                segment_point: point1,
                triangle_point: closest_point_on_aabb3(aabb, point1),
            }),
        )
        .expect("candidate exists");

        if t1 > t0 {
            let midpoint = (t0 + t1) / two;
            let mut quad_a = Fx32::ZERO;
            let mut quad_b = Fx32::ZERO;
            let mut quad_c = Fx32::ZERO;

            accumulate_axis_quadratic(
                start.x,
                delta.x,
                midpoint,
                aabb.min.x,
                aabb.max.x,
                &mut quad_a,
                &mut quad_b,
                &mut quad_c,
            );
            accumulate_axis_quadratic(
                start.y,
                delta.y,
                midpoint,
                aabb.min.y,
                aabb.max.y,
                &mut quad_a,
                &mut quad_b,
                &mut quad_c,
            );
            accumulate_axis_quadratic(
                start.z,
                delta.z,
                midpoint,
                aabb.min.z,
                aabb.max.z,
                &mut quad_a,
                &mut quad_b,
                &mut quad_c,
            );

            if quad_a > Fx32::ZERO {
                let stationary = (-quad_b) / (quad_a * two);
                if stationary > t0 && stationary < t1 {
                    let point = start + (delta * stationary);
                    best = pick_preferred_closest_points3(
                        Some(best),
                        Some(ClosestPoints3 {
                            segment_point: point,
                            triangle_point: closest_point_on_aabb3(aabb, point),
                        }),
                    )
                    .expect("candidate exists");
                }
            }
        }

        index += 1;
    }

    best
}

fn distance_squared_segment_aabb3(aabb: &Aabb3, start: Vec3, end: Vec3) -> Fx32 {
    if segment3_overlaps_aabb3(&Segment3::new(start, end), aabb) {
        return Fx32::ZERO;
    }

    let delta = end - start;
    let mut breakpoints = [Fx32::ZERO; 8];
    let mut breakpoint_count = 0usize;

    push_breakpoint(&mut breakpoints, &mut breakpoint_count, Fx32::ZERO);
    push_breakpoint(&mut breakpoints, &mut breakpoint_count, Fx32::ONE);
    axis_breakpoints(
        start.x,
        delta.x,
        aabb.min.x,
        aabb.max.x,
        &mut breakpoints,
        &mut breakpoint_count,
    );
    axis_breakpoints(
        start.y,
        delta.y,
        aabb.min.y,
        aabb.max.y,
        &mut breakpoints,
        &mut breakpoint_count,
    );
    axis_breakpoints(
        start.z,
        delta.z,
        aabb.min.z,
        aabb.max.z,
        &mut breakpoints,
        &mut breakpoint_count,
    );

    let mut best =
        distance_squared_point_aabb3(aabb, start).min(distance_squared_point_aabb3(aabb, end));
    let two = Fx32::from_int(2);
    let mut index = 0usize;

    while index + 1 < breakpoint_count {
        let t0 = breakpoints[index];
        let t1 = breakpoints[index + 1];

        best = best
            .min(distance_squared_point_aabb3(aabb, start + (delta * t0)))
            .min(distance_squared_point_aabb3(aabb, start + (delta * t1)));

        if t1 > t0 {
            let midpoint = (t0 + t1) / two;
            let mut quad_a = Fx32::ZERO;
            let mut quad_b = Fx32::ZERO;
            let mut quad_c = Fx32::ZERO;

            accumulate_axis_quadratic(
                start.x,
                delta.x,
                midpoint,
                aabb.min.x,
                aabb.max.x,
                &mut quad_a,
                &mut quad_b,
                &mut quad_c,
            );
            accumulate_axis_quadratic(
                start.y,
                delta.y,
                midpoint,
                aabb.min.y,
                aabb.max.y,
                &mut quad_a,
                &mut quad_b,
                &mut quad_c,
            );
            accumulate_axis_quadratic(
                start.z,
                delta.z,
                midpoint,
                aabb.min.z,
                aabb.max.z,
                &mut quad_a,
                &mut quad_b,
                &mut quad_c,
            );

            if quad_a > Fx32::ZERO {
                let stationary = (-quad_b) / (quad_a * two);
                if stationary > t0 && stationary < t1 {
                    best =
                        best.min(distance_squared_point_aabb3(aabb, start + (delta * stationary)));
                }
            }
        }

        index += 1;
    }

    best
}

fn distance_squared_segment_segment3(
    a_start: Vec3,
    a_end: Vec3,
    b_start: Vec3,
    b_end: Vec3,
) -> Fx32 {
    let closest = closest_points_segment_segment3(a_start, a_end, b_start, b_end);
    (closest.segment_point - closest.triangle_point).length_squared()
}

fn closest_points_segment_segment3(
    a_start: Vec3,
    a_end: Vec3,
    b_start: Vec3,
    b_end: Vec3,
) -> ClosestPoints3 {
    let u = a_end - a_start;
    let v = b_end - b_start;
    let w = a_start - b_start;

    let a = u.dot(u);
    let b = u.dot(v);
    let c = v.dot(v);
    let d = u.dot(w);
    let e = v.dot(w);

    if a == Fx32::ZERO {
        return ClosestPoints3 {
            segment_point: a_start,
            triangle_point: closest_point_on_segment3_points(b_start, b_end, a_start),
        };
    }
    if c == Fx32::ZERO {
        return ClosestPoints3 {
            segment_point: closest_point_on_segment3_points(a_start, a_end, b_start),
            triangle_point: b_start,
        };
    }

    let mut s_numer;
    let mut s_denom = (a * c) - (b * b);
    let mut t_numer;
    let mut t_denom = s_denom;

    if s_denom == Fx32::ZERO {
        s_numer = Fx32::ZERO;
        s_denom = Fx32::ONE;
        t_numer = e;
        t_denom = c;
    } else {
        s_numer = (b * e) - (c * d);
        t_numer = (a * e) - (b * d);

        if s_numer < Fx32::ZERO {
            s_numer = Fx32::ZERO;
            t_numer = e;
            t_denom = c;
        } else if s_numer > s_denom {
            s_numer = s_denom;
            t_numer = e + b;
            t_denom = c;
        }
    }

    if t_numer < Fx32::ZERO {
        t_numer = Fx32::ZERO;

        if -d < Fx32::ZERO {
            s_numer = Fx32::ZERO;
            s_denom = Fx32::ONE;
        } else if -d > a {
            s_numer = a;
            s_denom = a;
        } else {
            s_numer = -d;
            s_denom = a;
        }
    } else if t_numer > t_denom {
        t_numer = t_denom;

        if (-d + b) < Fx32::ZERO {
            s_numer = Fx32::ZERO;
            s_denom = Fx32::ONE;
        } else if (-d + b) > a {
            s_numer = a;
            s_denom = a;
        } else {
            s_numer = -d + b;
            s_denom = a;
        }
    }

    let s = if s_numer == Fx32::ZERO { Fx32::ZERO } else { s_numer / s_denom };
    let t = if t_numer == Fx32::ZERO { Fx32::ZERO } else { t_numer / t_denom };
    ClosestPoints3 { segment_point: a_start + (u * s), triangle_point: b_start + (v * t) }
}

fn closest_points_segment_triangle3(segment: Segment3, triangle: &Triangle3) -> ClosestPoints3 {
    let delta = segment.end - segment.start;
    if delta.length_squared() == Fx32::ZERO {
        let triangle_point = closest_point_on_triangle3(triangle, segment.start);
        return ClosestPoints3 { segment_point: segment.start, triangle_point };
    }

    if let Some(hit) = raycast_triangle3(&Ray3::new(segment.start, delta), triangle, Fx32::ONE) {
        return ClosestPoints3 { segment_point: hit.point, triangle_point: hit.point };
    }

    let mut best = ClosestPoints3 {
        segment_point: segment.start,
        triangle_point: closest_point_on_triangle3(triangle, segment.start),
    };
    best = pick_preferred_closest_points3(
        Some(best),
        Some(ClosestPoints3 {
            segment_point: segment.end,
            triangle_point: closest_point_on_triangle3(triangle, segment.end),
        }),
    )
    .expect("closest pair exists");

    for triangle_point in [triangle.a, triangle.b, triangle.c] {
        best = pick_preferred_closest_points3(
            Some(best),
            Some(ClosestPoints3 {
                segment_point: closest_point_on_segment3_points(
                    segment.start,
                    segment.end,
                    triangle_point,
                ),
                triangle_point,
            }),
        )
        .expect("closest pair exists");
    }

    for edge in triangle3_edges(*triangle) {
        best = pick_preferred_closest_points3(
            Some(best),
            Some(closest_points_segment_segment3(segment.start, segment.end, edge.start, edge.end)),
        )
        .expect("closest pair exists");
    }

    let normal = cross3(triangle.b - triangle.a, triangle.c - triangle.a);
    let normal_length_sq = normal.length_squared();
    if normal_length_sq != Fx32::ZERO {
        best = pick_preferred_closest_points3(
            Some(best),
            closest_points_segment_triangle_face(
                segment.start,
                segment.end,
                triangle,
                normal,
                normal_length_sq,
            ),
        )
        .expect("closest pair exists");
    }

    best
}

fn axis_breakpoints(
    start: Fx32,
    delta: Fx32,
    min: Fx32,
    max: Fx32,
    breakpoints: &mut [Fx32; 8],
    breakpoint_count: &mut usize,
) {
    if delta == Fx32::ZERO {
        return;
    }

    push_breakpoint(breakpoints, breakpoint_count, (min - start) / delta);
    push_breakpoint(breakpoints, breakpoint_count, (max - start) / delta);
}

fn push_breakpoint(breakpoints: &mut [Fx32; 8], breakpoint_count: &mut usize, value: Fx32) {
    if value < Fx32::ZERO || value > Fx32::ONE {
        return;
    }

    let mut index = 0usize;
    while index < *breakpoint_count {
        if breakpoints[index] == value {
            return;
        }
        if breakpoints[index] > value {
            let mut shift = *breakpoint_count;
            while shift > index {
                breakpoints[shift] = breakpoints[shift - 1];
                shift -= 1;
            }
            breakpoints[index] = value;
            *breakpoint_count += 1;
            return;
        }
        index += 1;
    }

    breakpoints[*breakpoint_count] = value;
    *breakpoint_count += 1;
}

fn accumulate_axis_quadratic(
    start: Fx32,
    delta: Fx32,
    sample_t: Fx32,
    min: Fx32,
    max: Fx32,
    quad_a: &mut Fx32,
    quad_b: &mut Fx32,
    quad_c: &mut Fx32,
) {
    let sample_value = start + (delta * sample_t);
    let offset = if sample_value < min {
        start - min
    } else if sample_value > max {
        start - max
    } else {
        return;
    };

    *quad_a = *quad_a + (delta * delta);
    *quad_b = *quad_b + ((offset * delta) * Fx32::from_int(2));
    *quad_c = *quad_c + (offset * offset);
}

fn inflate_aabb3(aabb: &Aabb3, amount: Fx32) -> Aabb3 {
    Aabb3::try_new(
        Vec3::new(aabb.min.x - amount, aabb.min.y - amount, aabb.min.z - amount),
        Vec3::new(aabb.max.x + amount, aabb.max.y + amount, aabb.max.z + amount),
    )
    .expect("inflated aabb3 remains valid")
}

fn sweep_segment_aabb3(
    segment: Segment3,
    delta: Vec3,
    aabb: &Aabb3,
    max_toi: Fx32,
) -> Option<RayHit3> {
    if segment3_overlaps_aabb3(&segment, aabb) {
        return Some(RayHit3 {
            toi: Fx32::ZERO,
            point: (segment.start + segment.end) * Fx32::from_ratio(1, 2),
            normal: Vec3::ZERO,
        });
    }

    let segment_center = (segment.start + segment.end) * Fx32::from_ratio(1, 2);
    let segment_half = (segment.end - segment.start) * Fx32::from_ratio(1, 2);
    let box_center = (aabb.min + aabb.max) * Fx32::from_ratio(1, 2);
    let box_half = (aabb.max - aabb.min) * Fx32::from_ratio(1, 2);
    let relative_center = segment_center - box_center;

    let mut t_enter = Fx32::ZERO;
    let mut t_exit = max_toi;
    let mut enter_normal = None;

    if !accumulate_sweep_constraint(
        relative_center.x,
        delta.x,
        box_half.x + segment_half.x.abs(),
        Vec3::new(Fx32::from_int(-1), Fx32::ZERO, Fx32::ZERO),
        Vec3::new(Fx32::from_int(1), Fx32::ZERO, Fx32::ZERO),
        &mut t_enter,
        &mut t_exit,
        &mut enter_normal,
    ) {
        return None;
    }
    if !accumulate_sweep_constraint(
        relative_center.y,
        delta.y,
        box_half.y + segment_half.y.abs(),
        Vec3::new(Fx32::ZERO, Fx32::from_int(-1), Fx32::ZERO),
        Vec3::new(Fx32::ZERO, Fx32::from_int(1), Fx32::ZERO),
        &mut t_enter,
        &mut t_exit,
        &mut enter_normal,
    ) {
        return None;
    }
    if !accumulate_sweep_constraint(
        relative_center.z,
        delta.z,
        box_half.z + segment_half.z.abs(),
        Vec3::new(Fx32::ZERO, Fx32::ZERO, Fx32::from_int(-1)),
        Vec3::new(Fx32::ZERO, Fx32::ZERO, Fx32::from_int(1)),
        &mut t_enter,
        &mut t_exit,
        &mut enter_normal,
    ) {
        return None;
    }

    let cross_x = Vec3::new(Fx32::ZERO, -segment_half.z, segment_half.y);
    if !accumulate_sweep_constraint(
        relative_center.dot(cross_x),
        delta.dot(cross_x),
        project_extent_on_axis(box_half, cross_x),
        -normalize_contact_axis3(cross_x),
        normalize_contact_axis3(cross_x),
        &mut t_enter,
        &mut t_exit,
        &mut enter_normal,
    ) {
        return None;
    }

    let cross_y = Vec3::new(segment_half.z, Fx32::ZERO, -segment_half.x);
    if !accumulate_sweep_constraint(
        relative_center.dot(cross_y),
        delta.dot(cross_y),
        project_extent_on_axis(box_half, cross_y),
        -normalize_contact_axis3(cross_y),
        normalize_contact_axis3(cross_y),
        &mut t_enter,
        &mut t_exit,
        &mut enter_normal,
    ) {
        return None;
    }

    let cross_z = Vec3::new(-segment_half.y, segment_half.x, Fx32::ZERO);
    if !accumulate_sweep_constraint(
        relative_center.dot(cross_z),
        delta.dot(cross_z),
        project_extent_on_axis(box_half, cross_z),
        -normalize_contact_axis3(cross_z),
        normalize_contact_axis3(cross_z),
        &mut t_enter,
        &mut t_exit,
        &mut enter_normal,
    ) {
        return None;
    }

    if t_enter < Fx32::ZERO || t_enter > max_toi {
        return None;
    }

    Some(RayHit3 {
        toi: t_enter,
        point: segment_center + (delta * t_enter),
        normal: enter_normal.unwrap_or(Vec3::ZERO),
    })
}

fn sweep_sphere_quad3_points(
    sphere: &Sphere,
    delta: Vec3,
    points: &[Vec3; 4],
    max_toi: Fx32,
) -> Option<RayHit3> {
    let ray = Ray3::new(sphere.center, delta);
    let mut best = raycast_quad3_face(&ray, sphere.radius, points, max_toi);
    let mut index = 0usize;

    while index < points.len() {
        let start = points[index];
        let end = points[(index + 1) % points.len()];
        let edge = Segment3::new(start, end);
        best = pick_preferred_hit3(best, sweep_sphere_segment3(sphere, delta, &edge, max_toi));
        index += 1;
    }

    best
}

fn raycast_quad3_face(
    ray: &Ray3,
    radius: Fx32,
    points: &[Vec3; 4],
    max_toi: Fx32,
) -> Option<RayHit3> {
    let normal = cross3(points[1] - points[0], points[3] - points[0]);
    let normal_length_sq = normal.length_squared();
    if normal_length_sq == Fx32::ZERO {
        return None;
    }

    let normal_length =
        sqrt_non_negative(normal_length_sq).expect("normal length squared is non-negative");
    if normal_length == Fx32::ZERO {
        return None;
    }

    let denom = normal.dot(ray.dir);
    if denom == Fx32::ZERO {
        return None;
    }

    let normal_unit = snap_axis_aligned_normal3(normal * (Fx32::ONE / normal_length));
    let plane_offset = normal.dot(points[0]);
    let radius_offset = radius * normal_length;
    let mut best = None;

    for signed_offset in [radius_offset, -radius_offset] {
        let toi = (plane_offset + signed_offset - normal.dot(ray.origin)) / denom;
        if toi < Fx32::ZERO || toi > max_toi {
            continue;
        }

        let center = ray.origin + (ray.dir * toi);
        let signed_distance = normal.dot(center - points[0]);
        let sign = signed_distance.signum();
        let contact_point = center - (normal_unit * (radius * Fx32::from_int(sign)));

        if point_in_convex_quad3(contact_point, points, normal) {
            let candidate = RayHit3 {
                toi,
                point: center,
                normal: if sign >= 0 { normal_unit } else { -normal_unit },
            };
            best = pick_preferred_hit3(best, Some(candidate));
        }
    }

    best
}

fn raycast_sphere_triangle3_face(
    origin: Vec3,
    delta: Vec3,
    radius: Fx32,
    triangle: &Triangle3,
    max_toi: Fx32,
) -> Option<RayHit3> {
    let normal = cross3(triangle.b - triangle.a, triangle.c - triangle.a);
    let normal_length_sq = normal.length_squared();
    if normal_length_sq == Fx32::ZERO {
        return None;
    }

    let normal_length =
        sqrt_non_negative(normal_length_sq).expect("normal length squared is non-negative");
    if normal_length == Fx32::ZERO {
        return None;
    }

    let denom = normal.dot(delta);
    if denom == Fx32::ZERO {
        return None;
    }

    let normal_unit = snap_axis_aligned_normal3(normal * (Fx32::ONE / normal_length));
    let plane_offset = normal.dot(triangle.a);
    let radius_offset = radius * normal_length;
    let mut best = None;

    for signed_offset in [radius_offset, -radius_offset] {
        let toi = (plane_offset + signed_offset - normal.dot(origin)) / denom;
        if toi < Fx32::ZERO || toi > max_toi {
            continue;
        }

        let center = origin + (delta * toi);
        let signed_distance = normal.dot(center - triangle.a);
        let sign = signed_distance.signum();
        let normal_sign = if sign == 0 {
            if signed_offset < Fx32::ZERO {
                -1
            } else {
                1
            }
        } else {
            sign
        };
        let contact_point = center - (normal_unit * (radius * Fx32::from_int(normal_sign)));

        if point_in_triangle3(contact_point, triangle, normal) {
            let candidate = RayHit3 {
                toi,
                point: center,
                normal: if normal_sign >= 0 { normal_unit } else { -normal_unit },
            };
            best = pick_preferred_hit3(best, Some(candidate));
        }
    }

    best
}

fn sweep_sphere_prism3(
    sphere: &Sphere,
    delta: Vec3,
    prism: &[Vec3; 6],
    max_toi: Fx32,
) -> Option<RayHit3> {
    let front = Triangle3::new(prism[0], prism[1], prism[2]);
    let back = Triangle3::new(prism[3], prism[5], prism[4]);
    let quads = [
        [prism[0], prism[1], prism[4], prism[3]],
        [prism[1], prism[2], prism[5], prism[4]],
        [prism[2], prism[0], prism[3], prism[5]],
    ];
    let edges = [
        Segment3::new(prism[0], prism[1]),
        Segment3::new(prism[1], prism[2]),
        Segment3::new(prism[2], prism[0]),
        Segment3::new(prism[3], prism[4]),
        Segment3::new(prism[4], prism[5]),
        Segment3::new(prism[5], prism[3]),
        Segment3::new(prism[0], prism[3]),
        Segment3::new(prism[1], prism[4]),
        Segment3::new(prism[2], prism[5]),
    ];

    let mut best =
        raycast_sphere_triangle3_face(sphere.center, delta, sphere.radius, &front, max_toi);
    best = pick_preferred_hit3(
        best,
        raycast_sphere_triangle3_face(sphere.center, delta, sphere.radius, &back, max_toi),
    );

    for quad in quads {
        best = pick_preferred_hit3(
            best,
            raycast_quad3_face(&Ray3::new(sphere.center, delta), sphere.radius, &quad, max_toi),
        );
    }

    for edge in edges {
        best = pick_preferred_hit3(best, sweep_sphere_segment3(sphere, delta, &edge, max_toi));
    }

    best
}

fn capsule3_midpoint(capsule: &Capsule3) -> Vec3 {
    (capsule.segment.start + capsule.segment.end) * Fx32::from_ratio(1, 2)
}

fn triangle3_reference_point(triangle: Triangle3) -> Vec3 {
    (triangle.a + triangle.b + triangle.c) * Fx32::from_ratio(1, 3)
}

fn segment3_difference_quad(moving: Segment3, target: Segment3) -> [Vec3; 4] {
    [
        target.start - moving.start,
        target.end - moving.start,
        target.end - moving.end,
        target.start - moving.end,
    ]
}

fn segment3_triangle3_difference_prism(moving: Segment3, target: Triangle3) -> [Vec3; 6] {
    [
        target.a - moving.start,
        target.b - moving.start,
        target.c - moving.start,
        target.a - moving.end,
        target.b - moving.end,
        target.c - moving.end,
    ]
}

fn triangle3_edges(triangle: Triangle3) -> [Segment3; 3] {
    [
        Segment3::new(triangle.a, triangle.b),
        Segment3::new(triangle.b, triangle.c),
        Segment3::new(triangle.c, triangle.a),
    ]
}

fn aabb3_corners(aabb: Aabb3) -> [Vec3; 8] {
    let min = aabb.min;
    let max = aabb.max;
    [
        Vec3::new(min.x, min.y, min.z),
        Vec3::new(max.x, min.y, min.z),
        Vec3::new(max.x, max.y, min.z),
        Vec3::new(min.x, max.y, min.z),
        Vec3::new(min.x, min.y, max.z),
        Vec3::new(max.x, min.y, max.z),
        Vec3::new(max.x, max.y, max.z),
        Vec3::new(min.x, max.y, max.z),
    ]
}

fn aabb3_edges(aabb: Aabb3) -> [Segment3; 12] {
    let corners = aabb3_corners(aabb);
    [
        Segment3::new(corners[0], corners[1]),
        Segment3::new(corners[1], corners[2]),
        Segment3::new(corners[2], corners[3]),
        Segment3::new(corners[3], corners[0]),
        Segment3::new(corners[4], corners[5]),
        Segment3::new(corners[5], corners[6]),
        Segment3::new(corners[6], corners[7]),
        Segment3::new(corners[7], corners[4]),
        Segment3::new(corners[0], corners[4]),
        Segment3::new(corners[1], corners[5]),
        Segment3::new(corners[2], corners[6]),
        Segment3::new(corners[3], corners[7]),
    ]
}

fn aabb3_face_triangles(aabb: Aabb3) -> [Triangle3; 12] {
    let corners = aabb3_corners(aabb);
    [
        Triangle3::new(corners[0], corners[1], corners[2]),
        Triangle3::new(corners[0], corners[2], corners[3]),
        Triangle3::new(corners[4], corners[5], corners[6]),
        Triangle3::new(corners[4], corners[6], corners[7]),
        Triangle3::new(corners[0], corners[1], corners[5]),
        Triangle3::new(corners[0], corners[5], corners[4]),
        Triangle3::new(corners[1], corners[2], corners[6]),
        Triangle3::new(corners[1], corners[6], corners[5]),
        Triangle3::new(corners[2], corners[3], corners[7]),
        Triangle3::new(corners[2], corners[7], corners[6]),
        Triangle3::new(corners[3], corners[0], corners[4]),
        Triangle3::new(corners[3], corners[4], corners[7]),
    ]
}

fn aabb3_face_centers(aabb: Aabb3) -> [(Vec3, usize); 6] {
    let center = (aabb.min + aabb.max) * (Fx32::ONE / Fx32::from_int(2));
    [
        (Vec3::new(aabb.min.x, center.y, center.z), 0),
        (Vec3::new(aabb.max.x, center.y, center.z), 0),
        (Vec3::new(center.x, aabb.min.y, center.z), 1),
        (Vec3::new(center.x, aabb.max.y, center.z), 1),
        (Vec3::new(center.x, center.y, aabb.min.z), 2),
        (Vec3::new(center.x, center.y, aabb.max.z), 2),
    ]
}

fn aabb3_face_corners(aabb: Aabb3, face_point: Vec3, axis: usize) -> [Vec3; 4] {
    match axis {
        0 => [
            Vec3::new(face_point.x, aabb.min.y, aabb.min.z),
            Vec3::new(face_point.x, aabb.max.y, aabb.min.z),
            Vec3::new(face_point.x, aabb.max.y, aabb.max.z),
            Vec3::new(face_point.x, aabb.min.y, aabb.max.z),
        ],
        1 => [
            Vec3::new(aabb.min.x, face_point.y, aabb.min.z),
            Vec3::new(aabb.max.x, face_point.y, aabb.min.z),
            Vec3::new(aabb.max.x, face_point.y, aabb.max.z),
            Vec3::new(aabb.min.x, face_point.y, aabb.max.z),
        ],
        _ => [
            Vec3::new(aabb.min.x, aabb.min.y, face_point.z),
            Vec3::new(aabb.max.x, aabb.min.y, face_point.z),
            Vec3::new(aabb.max.x, aabb.max.y, face_point.z),
            Vec3::new(aabb.min.x, aabb.max.y, face_point.z),
        ],
    }
}

fn point_matches_face_axis3(point: Vec3, face_point: Vec3, axis: usize) -> bool {
    match axis {
        0 => point.y == face_point.y && point.z == face_point.z,
        1 => point.x == face_point.x && point.z == face_point.z,
        _ => point.x == face_point.x && point.y == face_point.y,
    }
}

fn point_in_convex_quad3(point: Vec3, points: &[Vec3; 4], normal: Vec3) -> bool {
    let mut winding = 0i32;
    let mut index = 0usize;

    while index < points.len() {
        let start = points[index];
        let end = points[(index + 1) % points.len()];
        let edge_cross = normal.dot(cross3(end - start, point - start));

        if edge_cross == Fx32::ZERO {
            index += 1;
            continue;
        }

        let sign = if edge_cross > Fx32::ZERO { 1 } else { -1 };
        if winding == 0 {
            winding = sign;
        } else if sign != winding {
            return false;
        }

        index += 1;
    }

    true
}

fn segment_intersects_triangle3(segment: Segment3, triangle: &Triangle3) -> bool {
    let delta = segment.end - segment.start;
    if delta.length_squared() == Fx32::ZERO {
        return point_on_triangle3(triangle, segment.start);
    }

    raycast_triangle3(&Ray3::new(segment.start, delta), triangle, Fx32::ONE).is_some()
}

fn cross3(a: Vec3, b: Vec3) -> Vec3 {
    Vec3::new((a.y * b.z) - (a.z * b.y), (a.z * b.x) - (a.x * b.z), (a.x * b.y) - (a.y * b.x))
}

fn closest_point_on_triangle3_degenerate(triangle: &Triangle3, point: Vec3) -> Vec3 {
    let ab = closest_point_on_segment3_points(triangle.a, triangle.b, point);
    let bc = closest_point_on_segment3_points(triangle.b, triangle.c, point);
    let ca = closest_point_on_segment3_points(triangle.c, triangle.a, point);

    let ab_distance = (point - ab).length_squared();
    let bc_distance = (point - bc).length_squared();
    let ca_distance = (point - ca).length_squared();

    let mut best = ab;
    let mut best_distance = ab_distance;
    if bc_distance < best_distance || (bc_distance == best_distance && preferred_point3(bc, best)) {
        best = bc;
        best_distance = bc_distance;
    }
    if ca_distance < best_distance || (ca_distance == best_distance && preferred_point3(ca, best)) {
        best = ca;
    }
    best
}

fn closest_points_segment_triangle_face(
    start: Vec3,
    end: Vec3,
    triangle: &Triangle3,
    normal: Vec3,
    normal_length_sq: Fx32,
) -> Option<ClosestPoints3> {
    let delta = end - start;
    let non_negative = projected_point_interval_on_triangle(start, delta, triangle, normal, true)?;
    let non_positive = projected_point_interval_on_triangle(start, delta, triangle, normal, false);

    let mut best = None;
    for interval in [Some(non_negative), non_positive] {
        let Some((t_min, t_max)) = interval else {
            continue;
        };
        best = pick_preferred_closest_points3(
            best,
            Some(closest_points_segment_plane_interval(
                start,
                delta,
                triangle.a,
                normal,
                normal_length_sq,
                t_min,
                t_max,
            )),
        );
    }

    best
}

fn projected_point_interval_on_triangle(
    start: Vec3,
    delta: Vec3,
    triangle: &Triangle3,
    normal: Vec3,
    non_negative: bool,
) -> Option<(Fx32, Fx32)> {
    let mut t_min = Fx32::ZERO;
    let mut t_max = Fx32::ONE;
    let edges = [(triangle.a, triangle.b), (triangle.b, triangle.c), (triangle.c, triangle.a)];

    for (edge_start, edge_end) in edges {
        let edge = edge_end - edge_start;
        let start_value = normal.dot(cross3(edge, start - edge_start));
        let delta_value = normal.dot(cross3(edge, delta));
        if !intersect_linear_halfspace(
            start_value,
            delta_value,
            non_negative,
            &mut t_min,
            &mut t_max,
        ) {
            return None;
        }
    }

    Some((t_min, t_max))
}

fn intersect_linear_halfspace(
    start_value: Fx32,
    delta_value: Fx32,
    non_negative: bool,
    t_min: &mut Fx32,
    t_max: &mut Fx32,
) -> bool {
    let value0 = if non_negative { start_value } else { -start_value };
    let value_delta = if non_negative { delta_value } else { -delta_value };

    if value_delta == Fx32::ZERO {
        return value0 >= Fx32::ZERO;
    }

    let root = (-value0) / value_delta;
    if value_delta > Fx32::ZERO {
        if root > *t_min {
            *t_min = root;
        }
    } else if root < *t_max {
        *t_max = root;
    }

    *t_min <= *t_max && *t_max >= Fx32::ZERO && *t_min <= Fx32::ONE
}

fn closest_points_segment_plane_interval(
    start: Vec3,
    delta: Vec3,
    plane_point: Vec3,
    normal: Vec3,
    normal_length_sq: Fx32,
    t_min: Fx32,
    t_max: Fx32,
) -> ClosestPoints3 {
    let plane_offset0 = normal.dot(start - plane_point);
    let plane_offset_delta = normal.dot(delta);
    let mut best_t = t_min;
    let mut best_pair =
        closest_points_segment_plane_at(start, delta, plane_point, normal, normal_length_sq, t_min);
    let mut best_distance =
        plane_distance_squared_at(plane_offset0, plane_offset_delta, normal_length_sq, t_min);

    let end_pair =
        closest_points_segment_plane_at(start, delta, plane_point, normal, normal_length_sq, t_max);
    let end_distance =
        plane_distance_squared_at(plane_offset0, plane_offset_delta, normal_length_sq, t_max);
    if end_distance < best_distance
        || (end_distance == best_distance && preferred_closest_points3(end_pair, best_pair))
    {
        best_t = t_max;
        best_pair = end_pair;
        best_distance = end_distance;
    }

    if plane_offset_delta != Fx32::ZERO {
        let stationary = (-plane_offset0) / plane_offset_delta;
        if stationary >= t_min && stationary <= t_max {
            let stationary_pair = closest_points_segment_plane_at(
                start,
                delta,
                plane_point,
                normal,
                normal_length_sq,
                stationary,
            );
            let stationary_distance = plane_distance_squared_at(
                plane_offset0,
                plane_offset_delta,
                normal_length_sq,
                stationary,
            );
            if stationary_distance < best_distance
                || (stationary_distance == best_distance
                    && preferred_closest_points3(stationary_pair, best_pair))
            {
                best_t = stationary;
            }
        }
    }

    closest_points_segment_plane_at(start, delta, plane_point, normal, normal_length_sq, best_t)
}

fn closest_points_segment_plane_at(
    start: Vec3,
    delta: Vec3,
    plane_point: Vec3,
    normal: Vec3,
    normal_length_sq: Fx32,
    t: Fx32,
) -> ClosestPoints3 {
    let segment_point = start + (delta * t);
    let signed_distance = normal.dot(segment_point - plane_point);
    let triangle_point =
        if normal.y == Fx32::ZERO && normal.z == Fx32::ZERO && normal.x != Fx32::ZERO {
            Vec3::new(plane_point.x, segment_point.y, segment_point.z)
        } else if normal.x == Fx32::ZERO && normal.z == Fx32::ZERO && normal.y != Fx32::ZERO {
            Vec3::new(segment_point.x, plane_point.y, segment_point.z)
        } else if normal.x == Fx32::ZERO && normal.y == Fx32::ZERO && normal.z != Fx32::ZERO {
            Vec3::new(segment_point.x, segment_point.y, plane_point.z)
        } else {
            segment_point - (normal * (signed_distance / normal_length_sq))
        };
    ClosestPoints3 { segment_point, triangle_point }
}

fn plane_distance_squared_at(
    plane_offset0: Fx32,
    plane_offset_delta: Fx32,
    normal_length_sq: Fx32,
    t: Fx32,
) -> Fx32 {
    let signed_distance = plane_offset0 + (plane_offset_delta * t);
    (signed_distance * signed_distance) / normal_length_sq
}

fn point_on_triangle3(triangle: &Triangle3, point: Vec3) -> bool {
    let normal = cross3(triangle.b - triangle.a, triangle.c - triangle.a);
    if normal.length_squared() == Fx32::ZERO {
        return closest_point_on_triangle3_degenerate(triangle, point) == point;
    }

    normal.dot(point - triangle.a) == Fx32::ZERO && point_in_triangle3(point, triangle, normal)
}

fn point_in_triangle3(point: Vec3, triangle: &Triangle3, normal: Vec3) -> bool {
    let ab = normal.dot(cross3(triangle.b - triangle.a, point - triangle.a));
    let bc = normal.dot(cross3(triangle.c - triangle.b, point - triangle.b));
    let ca = normal.dot(cross3(triangle.a - triangle.c, point - triangle.c));

    let has_neg = ab < Fx32::ZERO || bc < Fx32::ZERO || ca < Fx32::ZERO;
    let has_pos = ab > Fx32::ZERO || bc > Fx32::ZERO || ca > Fx32::ZERO;
    !(has_neg && has_pos)
}

fn point_in_triangle3_strict(point: Vec3, triangle: &Triangle3) -> bool {
    let normal = cross3(triangle.b - triangle.a, triangle.c - triangle.a);
    if normal.length_squared() == Fx32::ZERO || normal.dot(point - triangle.a) != Fx32::ZERO {
        return false;
    }

    let ab = normal.dot(cross3(triangle.b - triangle.a, point - triangle.a));
    let bc = normal.dot(cross3(triangle.c - triangle.b, point - triangle.b));
    let ca = normal.dot(cross3(triangle.a - triangle.c, point - triangle.c));

    (ab > Fx32::ZERO && bc > Fx32::ZERO && ca > Fx32::ZERO)
        || (ab < Fx32::ZERO && bc < Fx32::ZERO && ca < Fx32::ZERO)
}

fn aabb3_face_projects_inside_triangle(
    aabb: Aabb3,
    face_point: Vec3,
    axis: usize,
    triangle: &Triangle3,
    triangle_point: Vec3,
    triangle_normal: Vec3,
) -> bool {
    if triangle_normal.length_squared() == Fx32::ZERO
        || !normal_is_axis_aligned_to3(triangle_normal, axis)
    {
        return false;
    }

    for corner in aabb3_face_corners(aabb, face_point, axis) {
        let projected = match axis {
            0 => Vec3::new(triangle_point.x, corner.y, corner.z),
            1 => Vec3::new(corner.x, triangle_point.y, corner.z),
            _ => Vec3::new(corner.x, corner.y, triangle_point.z),
        };
        if !point_on_triangle3(triangle, projected) {
            return false;
        }
    }

    true
}

fn normal_is_axis_aligned_to3(normal: Vec3, axis: usize) -> bool {
    match axis {
        0 => normal.y == Fx32::ZERO && normal.z == Fx32::ZERO && normal.x != Fx32::ZERO,
        1 => normal.x == Fx32::ZERO && normal.z == Fx32::ZERO && normal.y != Fx32::ZERO,
        _ => normal.x == Fx32::ZERO && normal.y == Fx32::ZERO && normal.z != Fx32::ZERO,
    }
}

fn shared_axis3(a: Vec3, b: Vec3) -> Option<usize> {
    if a.y == b.y && a.z == b.z && a.x != b.x {
        Some(0)
    } else if a.x == b.x && a.z == b.z && a.y != b.y {
        Some(1)
    } else if a.x == b.x && a.y == b.y && a.z != b.z {
        Some(2)
    } else {
        None
    }
}

fn normalize_face_normal3(normal: Vec3) -> Vec3 {
    let scale = normal.x.abs().max(normal.y.abs()).max(normal.z.abs());
    if scale == Fx32::ZERO {
        Vec3::ZERO
    } else {
        snap_axis_aligned_normal3(normal * (Fx32::ONE / scale))
    }
}

fn accumulate_sweep_constraint(
    offset: Fx32,
    velocity: Fx32,
    bound: Fx32,
    negative_normal: Vec3,
    positive_normal: Vec3,
    t_enter: &mut Fx32,
    t_exit: &mut Fx32,
    enter_normal: &mut Option<Vec3>,
) -> bool {
    if velocity == Fx32::ZERO {
        return offset.abs() <= bound;
    }

    let mut near = (-bound - offset) / velocity;
    let mut far = (bound - offset) / velocity;
    let mut near_normal = negative_normal;

    if near > far {
        core::mem::swap(&mut near, &mut far);
        near_normal = positive_normal;
    }

    if near > *t_enter
        || (near == *t_enter && preferred_normal3(near_normal, enter_normal.unwrap_or(Vec3::ZERO)))
    {
        *t_enter = near;
        *enter_normal = Some(near_normal);
    }
    if far < *t_exit {
        *t_exit = far;
    }

    *t_enter <= *t_exit && *t_exit >= Fx32::ZERO
}

fn project_extent_on_axis(extent: Vec3, axis: Vec3) -> Fx32 {
    (extent.x * axis.x.abs()) + (extent.y * axis.y.abs()) + (extent.z * axis.z.abs())
}

fn normalize_contact_axis3(axis: Vec3) -> Vec3 {
    let length_sq = axis.length_squared();
    if length_sq == Fx32::ZERO {
        return Vec3::ZERO;
    }

    let length = sqrt_non_negative(length_sq).expect("axis length squared is non-negative");
    if length == Fx32::ZERO {
        Vec3::ZERO
    } else {
        snap_axis_aligned_normal3(axis * (Fx32::ONE / length))
    }
}

fn snap_axis_aligned_normal3(normal: Vec3) -> Vec3 {
    if normal.y == Fx32::ZERO && normal.z == Fx32::ZERO && normal.x != Fx32::ZERO {
        Vec3::new(Fx32::from_int(normal.x.signum()), Fx32::ZERO, Fx32::ZERO)
    } else if normal.x == Fx32::ZERO && normal.z == Fx32::ZERO && normal.y != Fx32::ZERO {
        Vec3::new(Fx32::ZERO, Fx32::from_int(normal.y.signum()), Fx32::ZERO)
    } else if normal.x == Fx32::ZERO && normal.y == Fx32::ZERO && normal.z != Fx32::ZERO {
        Vec3::new(Fx32::ZERO, Fx32::ZERO, Fx32::from_int(normal.z.signum()))
    } else {
        normal
    }
}

fn snap_axis_aligned_boundary_point3(origin: Vec3, delta: Vec3, radius: Fx32) -> Option<Vec3> {
    if delta.y == Fx32::ZERO && delta.z == Fx32::ZERO && delta.x != Fx32::ZERO {
        Some(origin + Vec3::new(Fx32::from_int(delta.x.signum()) * radius, Fx32::ZERO, Fx32::ZERO))
    } else if delta.x == Fx32::ZERO && delta.z == Fx32::ZERO && delta.y != Fx32::ZERO {
        Some(origin + Vec3::new(Fx32::ZERO, Fx32::from_int(delta.y.signum()) * radius, Fx32::ZERO))
    } else if delta.x == Fx32::ZERO && delta.y == Fx32::ZERO && delta.z != Fx32::ZERO {
        Some(origin + Vec3::new(Fx32::ZERO, Fx32::ZERO, Fx32::from_int(delta.z.signum()) * radius))
    } else {
        None
    }
}

fn closest_point_on_round_anchor3(anchor: Vec3, radius: Fx32, point: Vec3) -> Vec3 {
    let delta = point - anchor;
    let distance_sq = delta.length_squared();
    let radius_sq = radius * radius;

    if distance_sq <= radius_sq {
        return point;
    }

    if let Some(snapped) = snap_axis_aligned_boundary_point3(anchor, delta, radius) {
        return snapped;
    }

    let distance = sqrt_non_negative(distance_sq).expect("distance squared is non-negative");
    if distance == Fx32::ZERO {
        anchor
    } else {
        anchor + (delta * (radius / distance))
    }
}

fn pick_earlier_hit3(current: Option<RayHit3>, candidate: Option<RayHit3>) -> Option<RayHit3> {
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

fn preferred_normal3(candidate: Vec3, current: Vec3) -> bool {
    candidate.x < current.x
        || (candidate.x == current.x
            && (candidate.y < current.y || (candidate.y == current.y && candidate.z < current.z)))
}

fn preferred_point3(candidate: Vec3, current: Vec3) -> bool {
    candidate.x < current.x
        || (candidate.x == current.x
            && (candidate.y < current.y || (candidate.y == current.y && candidate.z < current.z)))
}

fn preferred_closest_points3(candidate: ClosestPoints3, current: ClosestPoints3) -> bool {
    preferred_point3(candidate.segment_point, current.segment_point)
        || (candidate.segment_point == current.segment_point
            && preferred_point3(candidate.triangle_point, current.triangle_point))
}

fn preferred_closest_pair3(candidate: ClosestPair3, current: ClosestPair3) -> bool {
    preferred_point3(candidate.point_a, current.point_a)
        || (candidate.point_a == current.point_a
            && preferred_point3(candidate.point_b, current.point_b))
}

fn pick_preferred_closest_points3(
    current: Option<ClosestPoints3>,
    candidate: Option<ClosestPoints3>,
) -> Option<ClosestPoints3> {
    match (current, candidate) {
        (None, None) => None,
        (Some(pair), None) | (None, Some(pair)) => Some(pair),
        (Some(current_pair), Some(candidate_pair)) => {
            let current_distance =
                (current_pair.segment_point - current_pair.triangle_point).length_squared();
            let candidate_distance =
                (candidate_pair.segment_point - candidate_pair.triangle_point).length_squared();
            if candidate_distance < current_distance
                || (candidate_distance == current_distance
                    && preferred_closest_points3(candidate_pair, current_pair))
            {
                Some(candidate_pair)
            } else {
                Some(current_pair)
            }
        }
    }
}

fn pick_preferred_closest_pair3(
    current: Option<ClosestPair3>,
    candidate: Option<ClosestPair3>,
) -> Option<ClosestPair3> {
    match (current, candidate) {
        (None, None) => None,
        (Some(pair), None) | (None, Some(pair)) => Some(pair),
        (Some(current_pair), Some(candidate_pair)) => {
            let current_distance = (current_pair.point_a - current_pair.point_b).length_squared();
            let candidate_distance =
                (candidate_pair.point_a - candidate_pair.point_b).length_squared();
            if candidate_distance < current_distance
                || (candidate_distance == current_distance
                    && preferred_closest_pair3(candidate_pair, current_pair))
            {
                Some(candidate_pair)
            } else {
                Some(current_pair)
            }
        }
    }
}

fn pick_preferred_hit3(current: Option<RayHit3>, candidate: Option<RayHit3>) -> Option<RayHit3> {
    match (current, candidate) {
        (None, None) => None,
        (Some(hit), None) | (None, Some(hit)) => Some(hit),
        (Some(current_hit), Some(candidate_hit)) => {
            if candidate_hit.toi < current_hit.toi
                || (candidate_hit.toi == current_hit.toi
                    && preferred_normal3(candidate_hit.normal, current_hit.normal))
            {
                Some(candidate_hit)
            } else {
                Some(current_hit)
            }
        }
    }
}
