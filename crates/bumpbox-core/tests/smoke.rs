use bumpbox_core::{
    aabb3_overlaps, aabb_overlaps, capsule_contains_point, closest_point_on_aabb,
    closest_point_on_aabb3, convex_polygon_contains_point, oriented_box_contains_point,
    raycast_aabb, raycast_aabb3, sphere_overlaps_aabb3, Aabb, Aabb3, Capsule, ConvexPolygon, Fx32,
    OrientedBox, Ray, Ray3, Segment, Sphere, Vec2, Vec3,
};

fn v(x: i32, y: i32) -> Vec2 {
    Vec2::new(Fx32::from_int(x), Fx32::from_int(y))
}

fn v3(x: i32, y: i32, z: i32) -> Vec3 {
    Vec3::new(Fx32::from_int(x), Fx32::from_int(y), Fx32::from_int(z))
}

#[test]
fn fixed_arithmetic_round_trips() {
    let half = Fx32::from_ratio(1, 2);
    let three = Fx32::from_int(3);
    let result = three * half;
    assert_eq!(result, Fx32::from_ratio(3, 2));

    let quotient = result / half;
    assert_eq!(quotient, three);
}

#[test]
fn fixed_raw_round_trip_and_saturation_are_stable() {
    let raw = Fx32::from_raw(0x0001_8000);
    assert_eq!(raw.raw(), 0x0001_8000);

    let saturated_add = Fx32::MAX + Fx32::from_int(1);
    assert_eq!(saturated_add, Fx32::MAX);

    let saturated_neg = -Fx32::MIN;
    assert_eq!(saturated_neg, Fx32::MAX);
}

#[test]
fn aabb_overlap_is_symmetric_and_boundary_touch_counts() {
    let a = Aabb::try_new(v(0, 0), v(2, 2)).unwrap();
    let b = Aabb::try_new(v(2, 0), v(4, 2)).unwrap();
    assert!(aabb_overlaps(&a, &b));
    assert!(aabb_overlaps(&b, &a));

    for ax in 0..=2 {
        for bx in 0..=2 {
            let left = Aabb::try_new(v(ax, 0), v(ax + 1, 1)).unwrap();
            let right = Aabb::try_new(v(bx, 0), v(bx + 1, 1)).unwrap();
            assert_eq!(aabb_overlaps(&left, &right), aabb_overlaps(&right, &left));
        }
    }
}

#[test]
fn closest_point_clamps_each_axis() {
    let aabb = Aabb::try_new(v(0, 0), v(3, 4)).unwrap();
    let point = v(5, -2);
    let closest = closest_point_on_aabb(&aabb, point);
    assert_eq!(closest, v(3, 0));
}

#[test]
fn raycast_corner_hit_prefers_x_normal_on_tie() {
    let aabb = Aabb::try_new(v(0, 0), v(2, 2)).unwrap();
    let ray = Ray::new(v(-1, -1), v(1, 1));
    let hit = raycast_aabb(&ray, &aabb, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.normal, v(-1, 0));
    assert_eq!(hit.point, v(0, 0));
}

#[test]
fn convex_polygon_contains_point_is_winding_agnostic_and_boundary_inclusive() {
    let ccw = ConvexPolygon::try_new([v(0, 0), v(4, 0), v(4, 4), v(0, 4)]).unwrap();
    let cw = ConvexPolygon::try_new([v(0, 0), v(0, 4), v(4, 4), v(4, 0)]).unwrap();

    assert!(convex_polygon_contains_point(&ccw, v(2, 2)));
    assert!(convex_polygon_contains_point(&cw, v(2, 2)));
    assert!(convex_polygon_contains_point(&ccw, v(4, 2)));
    assert!(convex_polygon_contains_point(&ccw, v(0, 0)));
    assert!(!convex_polygon_contains_point(&ccw, v(5, 2)));
}

#[test]
fn capsule_contains_point_counts_endcaps_and_degenerate_segments() {
    let capsule = Capsule::try_new(Segment::new(v(0, 0), v(4, 0)), Fx32::from_int(1)).unwrap();
    assert!(capsule_contains_point(&capsule, v(2, 1)));
    assert!(capsule_contains_point(&capsule, v(-1, 0)));
    assert!(!capsule_contains_point(&capsule, v(2, 2)));

    let degenerate = Capsule::try_new(Segment::new(v(1, 1), v(1, 1)), Fx32::from_int(2)).unwrap();
    assert!(capsule_contains_point(&degenerate, v(3, 1)));
    assert!(!capsule_contains_point(&degenerate, v(4, 1)));
}

#[test]
fn oriented_box_contains_point_respects_basis_coordinates_and_boundary() {
    let obb = OrientedBox::try_new(v(0, 0), v(2, 1), v(1, 1), v(-1, 1)).unwrap();

    assert!(oriented_box_contains_point(&obb, v(1, 1)));
    assert!(oriented_box_contains_point(&obb, v(1, 3)));
    assert!(!oriented_box_contains_point(&obb, v(3, 3)));
}

#[test]
fn oriented_box_rejects_parallel_axes() {
    let obb = OrientedBox::try_new(v(0, 0), v(1, 1), v(1, 0), v(2, 0));
    assert!(obb.is_none());
}

#[test]
fn aabb3_overlap_is_symmetric_and_boundary_touch_counts() {
    let a = Aabb3::try_new(v3(0, 0, 0), v3(2, 2, 2)).unwrap();
    let b = Aabb3::try_new(v3(2, 0, 0), v3(4, 2, 2)).unwrap();
    assert!(aabb3_overlaps(&a, &b));
    assert!(aabb3_overlaps(&b, &a));
}

#[test]
fn closest_point_on_aabb3_clamps_each_axis() {
    let aabb = Aabb3::try_new(v3(0, 0, 0), v3(3, 4, 5)).unwrap();
    let point = v3(5, -2, 7);
    let closest = closest_point_on_aabb3(&aabb, point);
    assert_eq!(closest, v3(3, 0, 5));
}

#[test]
fn sphere_overlaps_aabb3_counts_boundary_touch() {
    let aabb = Aabb3::try_new(v3(0, 0, 0), v3(2, 2, 2)).unwrap();
    let touching = Sphere::try_new(v3(3, 1, 1), Fx32::from_int(1)).unwrap();
    let separated = Sphere::try_new(v3(4, 1, 1), Fx32::from_int(1)).unwrap();

    assert!(sphere_overlaps_aabb3(&touching, &aabb));
    assert!(!sphere_overlaps_aabb3(&separated, &aabb));
}

#[test]
fn raycast_aabb3_corner_hit_prefers_x_normal_on_three_way_tie() {
    let aabb = Aabb3::try_new(v3(0, 0, 0), v3(2, 2, 2)).unwrap();
    let ray = Ray3::new(v3(-1, -1, -1), v3(1, 1, 1));
    let hit = raycast_aabb3(&ray, &aabb, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.normal, v3(-1, 0, 0));
    assert_eq!(hit.point, v3(0, 0, 0));
}
