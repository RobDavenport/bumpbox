use bumpbox_core::{
    aabb3_overlaps, aabb_overlaps, capsule3_contains_point, capsule3_overlaps_aabb3,
    capsule3_overlaps_capsule3, capsule3_overlaps_triangle3, capsule_contains_point,
    capsule_overlaps_aabb, capsule_overlaps_capsule, capsule_overlaps_convex_polygon,
    capsule_overlaps_oriented_box, capsule_overlaps_segment, circle_overlaps_capsule,
    circle_overlaps_convex_polygon, circle_overlaps_oriented_box, circle_overlaps_segment,
    closest_point_on_aabb, closest_point_on_aabb3, closest_point_on_capsule,
    closest_point_on_capsule3, closest_point_on_circle, closest_point_on_convex_polygon,
    closest_point_on_oriented_box, closest_point_on_segment, closest_point_on_segment3,
    closest_point_on_sphere, closest_point_on_triangle3, closest_points_capsule3_triangle3,
    closest_points_segment3_triangle3, closest_points_sphere_triangle3,
    closest_points_triangle3_aabb3, closest_points_triangle3_triangle3,
    convex_polygon_contains_point, convex_polygon_overlaps_aabb,
    convex_polygon_overlaps_convex_polygon, distance_squared_capsule3_triangle3,
    distance_squared_point_capsule, distance_squared_point_capsule3, distance_squared_point_circle,
    distance_squared_point_convex_polygon, distance_squared_point_oriented_box,
    distance_squared_point_segment, distance_squared_point_segment3, distance_squared_point_sphere,
    distance_squared_point_triangle3, distance_squared_segment3_triangle3,
    distance_squared_sphere_triangle3, distance_squared_triangle3_aabb3,
    distance_squared_triangle3_triangle3, oriented_box_contains_point, oriented_box_overlaps_aabb,
    oriented_box_overlaps_oriented_box, raycast_aabb, raycast_aabb3, raycast_capsule,
    raycast_capsule3, raycast_circle, raycast_convex_polygon, raycast_oriented_box, raycast_sphere,
    raycast_triangle3, segment3_overlaps_aabb3, segment3_overlaps_triangle3,
    segment_intersects_segment, sphere_overlaps_aabb3, sphere_overlaps_capsule3,
    sphere_overlaps_segment3, sphere_overlaps_sphere, sphere_overlaps_triangle3,
    sweep_capsule3_aabb3, sweep_capsule3_capsule3, sweep_capsule3_segment3, sweep_capsule3_sphere,
    sweep_capsule3_triangle3, sweep_capsule_aabb, sweep_capsule_capsule, sweep_capsule_circle,
    sweep_capsule_convex_polygon, sweep_capsule_oriented_box, sweep_capsule_segment,
    sweep_circle_aabb, sweep_circle_capsule, sweep_circle_circle, sweep_circle_convex_polygon,
    sweep_circle_oriented_box, sweep_circle_segment, sweep_segment3_aabb3,
    sweep_segment3_triangle3, sweep_sphere_aabb3, sweep_sphere_capsule3, sweep_sphere_segment3,
    sweep_sphere_sphere, sweep_sphere_triangle3, sweep_triangle3_triangle3,
    triangle3_overlaps_aabb3, triangle3_overlaps_triangle3, Aabb, Aabb3, Capsule, Capsule3, Circle,
    ConvexPolygon, Fx32, OrientedBox, Ray, Ray3, Segment, Segment3, Sphere, Triangle3, Vec2, Vec3,
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
fn segment_closest_point_and_distance_clamp_to_endpoints_and_interior() {
    let segment = Segment::new(v(0, 0), v(4, 0));
    assert_eq!(closest_point_on_segment(&segment, v(2, 3)), v(2, 0));
    assert_eq!(distance_squared_point_segment(&segment, v(2, 3)), Fx32::from_int(9));
    assert_eq!(closest_point_on_segment(&segment, v(-1, 1)), v(0, 0));

    let degenerate = Segment::new(v(1, 1), v(1, 1));
    assert_eq!(closest_point_on_segment(&degenerate, v(4, 5)), v(1, 1));
    assert_eq!(distance_squared_point_segment(&degenerate, v(4, 5)), Fx32::from_int(25));
}

#[test]
fn circle_closest_point_and_distance_are_zero_inside_and_boundary_clamped() {
    let circle = Circle::try_new(v(0, 0), Fx32::from_int(2)).unwrap();
    assert_eq!(closest_point_on_circle(&circle, v(1, 0)), v(1, 0));
    assert_eq!(distance_squared_point_circle(&circle, v(1, 0)), Fx32::ZERO);
    assert_eq!(closest_point_on_circle(&circle, v(5, 0)), v(2, 0));
    assert_eq!(distance_squared_point_circle(&circle, v(5, 0)), Fx32::from_int(9));

    let degenerate = Circle::try_new(v(1, 1), Fx32::ZERO).unwrap();
    assert_eq!(closest_point_on_circle(&degenerate, v(4, 5)), v(1, 1));
    assert_eq!(distance_squared_point_circle(&degenerate, v(4, 5)), Fx32::from_int(25));
}

#[test]
fn capsule_closest_point_and_distance_handle_inside_and_degenerate_segments() {
    let capsule = Capsule::try_new(Segment::new(v(0, 0), v(4, 0)), Fx32::from_int(1)).unwrap();
    assert_eq!(closest_point_on_capsule(&capsule, v(2, 0)), v(2, 0));
    assert_eq!(distance_squared_point_capsule(&capsule, v(2, 0)), Fx32::ZERO);
    assert_eq!(closest_point_on_capsule(&capsule, v(2, 3)), v(2, 1));
    assert_eq!(distance_squared_point_capsule(&capsule, v(2, 3)), Fx32::from_int(4));

    let degenerate = Capsule::try_new(Segment::new(v(1, 1), v(1, 1)), Fx32::from_int(2)).unwrap();
    assert_eq!(closest_point_on_capsule(&degenerate, v(5, 1)), v(3, 1));
    assert_eq!(distance_squared_point_capsule(&degenerate, v(5, 1)), Fx32::from_int(4));
}

#[test]
fn convex_polygon_closest_point_and_distance_are_winding_agnostic() {
    let ccw = ConvexPolygon::try_new([v(0, 0), v(4, 0), v(4, 4), v(0, 4)]).unwrap();
    let cw = ConvexPolygon::try_new([v(0, 0), v(0, 4), v(4, 4), v(4, 0)]).unwrap();
    assert_eq!(closest_point_on_convex_polygon(&ccw, v(2, 2)), v(2, 2));
    assert_eq!(distance_squared_point_convex_polygon(&ccw, v(2, 2)), Fx32::ZERO);
    assert_eq!(closest_point_on_convex_polygon(&ccw, v(6, 2)), v(4, 2));
    assert_eq!(distance_squared_point_convex_polygon(&ccw, v(6, 2)), Fx32::from_int(4));
    assert_eq!(closest_point_on_convex_polygon(&cw, v(6, 2)), v(4, 2));
}

#[test]
fn oriented_box_closest_point_and_distance_handle_rotated_boxes() {
    let axis_aligned = OrientedBox::try_new(v(0, 0), v(2, 1), v(1, 0), v(0, 1)).unwrap();
    assert_eq!(closest_point_on_oriented_box(&axis_aligned, v(0, 0)), v(0, 0));
    assert_eq!(distance_squared_point_oriented_box(&axis_aligned, v(0, 0)), Fx32::ZERO);
    assert_eq!(closest_point_on_oriented_box(&axis_aligned, v(5, 0)), v(2, 0));
    assert_eq!(distance_squared_point_oriented_box(&axis_aligned, v(5, 0)), Fx32::from_int(9));

    let rotated = OrientedBox::try_new(v(0, 0), v(1, 1), v(1, 1), v(-1, 1)).unwrap();
    assert_eq!(closest_point_on_oriented_box(&rotated, v(0, 0)), v(0, 0));
    assert!(distance_squared_point_oriented_box(&rotated, v(4, 0)) > Fx32::ZERO);
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
fn raycast_circle_is_boundary_inclusive_and_reports_zero_normal_from_inside() {
    let circle = Circle::try_new(v(3, 0), Fx32::from_int(2)).unwrap();
    let incoming = Ray::new(v(0, 0), v(1, 0));
    let hit = raycast_circle(&incoming, &circle, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(1));
    assert_eq!(hit.point, v(1, 0));
    assert_eq!(hit.normal, v(-1, 0));

    let inside = Ray::new(v(3, 0), v(1, 0));
    let inside_hit = raycast_circle(&inside, &circle, Fx32::from_int(10)).unwrap();
    assert_eq!(inside_hit.toi, Fx32::ZERO);
    assert_eq!(inside_hit.point, v(3, 0));
    assert_eq!(inside_hit.normal, Vec2::ZERO);

    let too_short = Ray::new(v(0, 0), v(1, 0));
    assert!(raycast_circle(&too_short, &circle, Fx32::from_ratio(1, 2)).is_none());
}

#[test]
fn raycast_capsule_is_boundary_inclusive_and_reports_zero_normal_from_inside() {
    let capsule = Capsule::try_new(Segment::new(v(3, -1), v(3, 1)), Fx32::from_int(1)).unwrap();
    let incoming = Ray::new(v(0, 0), v(1, 0));
    let hit = raycast_capsule(&incoming, &capsule, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(2));
    assert_eq!(hit.point, v(2, 0));
    assert_eq!(hit.normal, v(-1, 0));

    let inside = Ray::new(v(3, 0), v(1, 0));
    let inside_hit = raycast_capsule(&inside, &capsule, Fx32::from_int(10)).unwrap();
    assert_eq!(inside_hit.toi, Fx32::ZERO);
    assert_eq!(inside_hit.point, v(3, 0));
    assert_eq!(inside_hit.normal, Vec2::ZERO);

    let too_short = Ray::new(v(0, 0), v(1, 0));
    assert!(raycast_capsule(&too_short, &capsule, Fx32::from_int(1)).is_none());
}

#[test]
fn raycast_convex_polygon_is_boundary_inclusive_and_reports_zero_normal_from_inside() {
    let square = ConvexPolygon::try_new([v(2, -2), v(6, -2), v(6, 2), v(2, 2)]).unwrap();
    let incoming = Ray::new(v(0, 0), v(1, 0));
    let hit = raycast_convex_polygon(&incoming, &square, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(2));
    assert_eq!(hit.point, v(2, 0));
    assert_eq!(hit.normal, v(-1, 0));

    let inside = Ray::new(v(3, 0), v(1, 0));
    let inside_hit = raycast_convex_polygon(&inside, &square, Fx32::from_int(10)).unwrap();
    assert_eq!(inside_hit.toi, Fx32::ZERO);
    assert_eq!(inside_hit.point, v(3, 0));
    assert_eq!(inside_hit.normal, Vec2::ZERO);

    let too_short = Ray::new(v(0, 0), v(1, 0));
    assert!(raycast_convex_polygon(&too_short, &square, Fx32::from_int(1)).is_none());
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
fn convex_polygon_overlap_is_winding_agnostic_and_boundary_inclusive() {
    let square = ConvexPolygon::try_new([v(0, 0), v(4, 0), v(4, 4), v(0, 4)]).unwrap();
    let touching = ConvexPolygon::try_new([v(4, 1), v(6, 1), v(6, 3), v(4, 3)]).unwrap();
    let touching_cw = ConvexPolygon::try_new([v(4, 1), v(4, 3), v(6, 3), v(6, 1)]).unwrap();
    let separated = ConvexPolygon::try_new([v(5, 1), v(7, 1), v(7, 3), v(5, 3)]).unwrap();

    assert!(convex_polygon_overlaps_convex_polygon(&square, &touching));
    assert!(convex_polygon_overlaps_convex_polygon(&touching, &square));
    assert!(convex_polygon_overlaps_convex_polygon(&square, &touching_cw));
    assert!(!convex_polygon_overlaps_convex_polygon(&square, &separated));
}

#[test]
fn circle_overlaps_convex_polygon_counts_boundary_touch_and_containment() {
    let square = ConvexPolygon::try_new([v(0, 0), v(4, 0), v(4, 4), v(0, 4)]).unwrap();
    let contained = Circle::try_new(v(2, 2), Fx32::from_int(1)).unwrap();
    let touching = Circle::try_new(v(6, 2), Fx32::from_int(2)).unwrap();
    let separated = Circle::try_new(v(7, 2), Fx32::from_int(2)).unwrap();

    assert!(circle_overlaps_convex_polygon(&contained, &square));
    assert!(circle_overlaps_convex_polygon(&touching, &square));
    assert!(!circle_overlaps_convex_polygon(&separated, &square));
}

#[test]
fn capsule_overlaps_convex_polygon_counts_boundary_touch_and_degenerate_capsules() {
    let square = ConvexPolygon::try_new([v(0, 0), v(4, 0), v(4, 4), v(0, 4)]).unwrap();
    let touching = Capsule::try_new(Segment::new(v(6, 2), v(8, 2)), Fx32::from_int(2)).unwrap();
    let contained = Capsule::try_new(Segment::new(v(1, 1), v(3, 1)), Fx32::from_int(0)).unwrap();
    let degenerate = Capsule::try_new(Segment::new(v(6, 2), v(6, 2)), Fx32::from_int(2)).unwrap();
    let separated = Capsule::try_new(Segment::new(v(7, 2), v(9, 2)), Fx32::from_int(2)).unwrap();

    assert!(capsule_overlaps_convex_polygon(&touching, &square));
    assert!(capsule_overlaps_convex_polygon(&contained, &square));
    assert!(capsule_overlaps_convex_polygon(&degenerate, &square));
    assert!(!capsule_overlaps_convex_polygon(&separated, &square));
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
fn circle_overlaps_capsule_counts_boundary_touch() {
    let capsule = Capsule::try_new(Segment::new(v(0, 0), v(4, 0)), Fx32::from_int(1)).unwrap();
    let touching = Circle::try_new(v(2, 3), Fx32::from_int(2)).unwrap();
    let separated = Circle::try_new(v(2, 4), Fx32::from_int(2)).unwrap();

    assert!(circle_overlaps_capsule(&touching, &capsule));
    assert!(!circle_overlaps_capsule(&separated, &capsule));
}

#[test]
fn circle_overlaps_segment_counts_boundary_touch_and_degenerate_segments() {
    let segment = Segment::new(v(0, 0), v(4, 0));
    let touching = Circle::try_new(v(2, 3), Fx32::from_int(3)).unwrap();
    let separated = Circle::try_new(v(2, 4), Fx32::from_int(3)).unwrap();
    let degenerate_segment = Segment::new(v(6, 0), v(6, 0));
    let degenerate_touching = Circle::try_new(v(8, 0), Fx32::from_int(2)).unwrap();

    assert!(circle_overlaps_segment(&touching, &segment));
    assert!(!circle_overlaps_segment(&separated, &segment));
    assert!(circle_overlaps_segment(&degenerate_touching, &degenerate_segment));
}

#[test]
fn capsule_overlaps_capsule_is_symmetric_and_handles_degenerate_capsules() {
    let horizontal = Capsule::try_new(Segment::new(v(0, 0), v(4, 0)), Fx32::from_int(1)).unwrap();
    let touching = Capsule::try_new(Segment::new(v(0, 4), v(4, 4)), Fx32::from_int(3)).unwrap();
    let degenerate = Capsule::try_new(Segment::new(v(9, 0), v(9, 0)), Fx32::from_int(3)).unwrap();

    assert!(capsule_overlaps_capsule(&horizontal, &touching));
    assert!(capsule_overlaps_capsule(&touching, &horizontal));
    assert!(!capsule_overlaps_capsule(&horizontal, &degenerate));
}

#[test]
fn capsule_overlaps_segment_counts_boundary_touch_and_degenerate_capsules() {
    let segment = Segment::new(v(0, 0), v(4, 0));
    let touching = Capsule::try_new(Segment::new(v(2, 3), v(2, 5)), Fx32::from_int(3)).unwrap();
    let separated = Capsule::try_new(Segment::new(v(2, 4), v(2, 6)), Fx32::from_int(3)).unwrap();
    let degenerate = Capsule::try_new(Segment::new(v(6, 0), v(6, 0)), Fx32::from_int(2)).unwrap();

    assert!(capsule_overlaps_segment(&touching, &segment));
    assert!(!capsule_overlaps_segment(&separated, &segment));
    assert!(capsule_overlaps_segment(&degenerate, &Segment::new(v(8, 0), v(8, 0))));
}

#[test]
fn segment_intersects_segment_is_symmetric_and_boundary_inclusive() {
    let horizontal = Segment::new(v(0, 0), v(4, 0));
    let crossing = Segment::new(v(2, -1), v(2, 1));
    let touching = Segment::new(v(4, 0), v(6, 0));
    let separated = Segment::new(v(5, 0), v(7, 0));
    let degenerate = Segment::new(v(4, 0), v(4, 0));

    assert!(segment_intersects_segment(&horizontal, &crossing));
    assert!(segment_intersects_segment(&crossing, &horizontal));
    assert!(segment_intersects_segment(&horizontal, &touching));
    assert!(!segment_intersects_segment(&horizontal, &separated));
    assert!(segment_intersects_segment(&horizontal, &degenerate));
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
fn oriented_box_overlap_is_symmetric_and_boundary_inclusive() {
    let left = OrientedBox::try_new(v(0, 0), v(2, 1), v(1, 0), v(0, 1)).unwrap();
    let touching = OrientedBox::try_new(v(4, 0), v(2, 1), v(1, 0), v(0, 1)).unwrap();
    let rotated = OrientedBox::try_new(v(1, 0), v(1, 1), v(1, 1), v(-1, 1)).unwrap();
    let separated = OrientedBox::try_new(v(5, 0), v(2, 1), v(1, 0), v(0, 1)).unwrap();

    assert!(oriented_box_overlaps_oriented_box(&left, &touching));
    assert!(oriented_box_overlaps_oriented_box(&touching, &left));
    assert!(oriented_box_overlaps_oriented_box(&left, &rotated));
    assert!(!oriented_box_overlaps_oriented_box(&left, &separated));
}

#[test]
fn circle_overlaps_oriented_box_counts_boundary_touch_and_rotated_boxes() {
    let axis_aligned = OrientedBox::try_new(v(0, 0), v(2, 1), v(1, 0), v(0, 1)).unwrap();
    let rotated = OrientedBox::try_new(v(0, 0), v(1, 1), v(1, 1), v(-1, 1)).unwrap();
    let touching = Circle::try_new(v(4, 0), Fx32::from_int(2)).unwrap();
    let contained = Circle::try_new(v(0, 0), Fx32::from_int(1)).unwrap();
    let separated = Circle::try_new(v(5, 0), Fx32::from_int(2)).unwrap();

    assert!(circle_overlaps_oriented_box(&touching, &axis_aligned));
    assert!(circle_overlaps_oriented_box(&contained, &rotated));
    assert!(!circle_overlaps_oriented_box(&separated, &axis_aligned));
}

#[test]
fn capsule_overlaps_oriented_box_counts_boundary_touch_and_rotated_boxes() {
    let axis_aligned = OrientedBox::try_new(v(0, 0), v(2, 1), v(1, 0), v(0, 1)).unwrap();
    let rotated = OrientedBox::try_new(v(0, 0), v(1, 1), v(1, 1), v(-1, 1)).unwrap();
    let touching = Capsule::try_new(Segment::new(v(4, 0), v(6, 0)), Fx32::from_int(2)).unwrap();
    let contained = Capsule::try_new(Segment::new(v(-1, 0), v(1, 0)), Fx32::from_int(0)).unwrap();
    let separated = Capsule::try_new(Segment::new(v(7, 0), v(9, 0)), Fx32::from_int(2)).unwrap();

    assert!(capsule_overlaps_oriented_box(&touching, &axis_aligned));
    assert!(capsule_overlaps_oriented_box(&contained, &rotated));
    assert!(!capsule_overlaps_oriented_box(&separated, &axis_aligned));
}

#[test]
fn capsule_overlaps_aabb_counts_boundary_touch_and_degenerate_capsules() {
    let aabb = Aabb::try_new(v(0, 0), v(4, 4)).unwrap();
    let touching = Capsule::try_new(Segment::new(v(6, 2), v(8, 2)), Fx32::from_int(2)).unwrap();
    let contained = Capsule::try_new(Segment::new(v(1, 1), v(3, 1)), Fx32::ZERO).unwrap();
    let degenerate = Capsule::try_new(Segment::new(v(6, 2), v(6, 2)), Fx32::from_int(2)).unwrap();
    let separated = Capsule::try_new(Segment::new(v(7, 2), v(9, 2)), Fx32::from_int(2)).unwrap();

    assert!(capsule_overlaps_aabb(&touching, &aabb));
    assert!(capsule_overlaps_aabb(&contained, &aabb));
    assert!(capsule_overlaps_aabb(&degenerate, &aabb));
    assert!(!capsule_overlaps_aabb(&separated, &aabb));
}

#[test]
fn convex_polygon_overlaps_aabb_is_winding_agnostic_and_boundary_inclusive() {
    let aabb = Aabb::try_new(v(0, 0), v(4, 4)).unwrap();
    let touching = ConvexPolygon::try_new([v(4, 1), v(6, 1), v(6, 3), v(4, 3)]).unwrap();
    let touching_cw = ConvexPolygon::try_new([v(4, 1), v(4, 3), v(6, 3), v(6, 1)]).unwrap();
    let separated = ConvexPolygon::try_new([v(5, 1), v(7, 1), v(7, 3), v(5, 3)]).unwrap();

    assert!(convex_polygon_overlaps_aabb(&touching, &aabb));
    assert!(convex_polygon_overlaps_aabb(&touching_cw, &aabb));
    assert!(!convex_polygon_overlaps_aabb(&separated, &aabb));
}

#[test]
fn oriented_box_overlaps_aabb_is_symmetric_and_boundary_inclusive() {
    let aabb = Aabb::try_new(v(0, 0), v(4, 4)).unwrap();
    let touching = OrientedBox::try_new(v(6, 2), v(2, 1), v(1, 0), v(0, 1)).unwrap();
    let rotated = OrientedBox::try_new(v(3, 2), v(1, 1), v(1, 1), v(-1, 1)).unwrap();
    let separated = OrientedBox::try_new(v(7, 2), v(2, 1), v(1, 0), v(0, 1)).unwrap();

    assert!(oriented_box_overlaps_aabb(&touching, &aabb));
    assert!(oriented_box_overlaps_aabb(&rotated, &aabb));
    assert!(!oriented_box_overlaps_aabb(&separated, &aabb));
}

#[test]
fn raycast_oriented_box_is_boundary_inclusive_and_reports_zero_normal_from_inside() {
    let axis_aligned = OrientedBox::try_new(v(4, 0), v(2, 2), v(1, 0), v(0, 1)).unwrap();
    let incoming = Ray::new(v(0, 0), v(1, 0));
    let hit = raycast_oriented_box(&incoming, &axis_aligned, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(2));
    assert_eq!(hit.point, v(2, 0));
    assert_eq!(hit.normal, v(-1, 0));

    let inside = Ray::new(v(4, 0), v(1, 0));
    let inside_hit = raycast_oriented_box(&inside, &axis_aligned, Fx32::from_int(10)).unwrap();
    assert_eq!(inside_hit.toi, Fx32::ZERO);
    assert_eq!(inside_hit.point, v(4, 0));
    assert_eq!(inside_hit.normal, Vec2::ZERO);

    let rotated = OrientedBox::try_new(v(4, 0), v(2, 2), v(1, 1), v(-1, 1)).unwrap();
    assert!(raycast_oriented_box(&incoming, &rotated, Fx32::from_int(10)).is_some());

    let too_short = Ray::new(v(0, 0), v(1, 0));
    assert!(raycast_oriented_box(&too_short, &axis_aligned, Fx32::from_int(1)).is_none());
}

#[test]
fn sweep_circle_aabb_is_boundary_inclusive_and_reports_zero_normal_from_inside() {
    let aabb = Aabb::try_new(v(4, -2), v(8, 2)).unwrap();
    let circle = Circle::try_new(v(0, 0), Fx32::from_int(2)).unwrap();
    let hit = sweep_circle_aabb(&circle, v(1, 0), &aabb, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(2));
    assert_eq!(hit.point, v(2, 0));
    assert_eq!(hit.normal, v(-1, 0));

    let overlapping = Circle::try_new(v(5, 0), Fx32::from_int(2)).unwrap();
    let overlap_hit = sweep_circle_aabb(&overlapping, v(1, 0), &aabb, Fx32::from_int(10)).unwrap();
    assert_eq!(overlap_hit.toi, Fx32::ZERO);
    assert_eq!(overlap_hit.point, v(5, 0));
    assert_eq!(overlap_hit.normal, Vec2::ZERO);

    assert!(sweep_circle_aabb(&circle, v(1, 0), &aabb, Fx32::from_int(1)).is_none());
}

#[test]
fn sweep_circle_capsule_is_boundary_inclusive_and_reports_zero_normal_from_inside() {
    let capsule = Capsule::try_new(Segment::new(v(6, -1), v(6, 1)), Fx32::from_int(1)).unwrap();
    let circle = Circle::try_new(v(0, 0), Fx32::from_int(2)).unwrap();
    let hit = sweep_circle_capsule(&circle, v(1, 0), &capsule, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(3));
    assert_eq!(hit.point, v(3, 0));
    assert_eq!(hit.normal, v(-1, 0));

    let overlapping = Circle::try_new(v(6, 0), Fx32::from_int(2)).unwrap();
    let overlap_hit =
        sweep_circle_capsule(&overlapping, v(1, 0), &capsule, Fx32::from_int(10)).unwrap();
    assert_eq!(overlap_hit.toi, Fx32::ZERO);
    assert_eq!(overlap_hit.point, v(6, 0));
    assert_eq!(overlap_hit.normal, Vec2::ZERO);

    assert!(sweep_circle_capsule(&circle, v(1, 0), &capsule, Fx32::from_int(2)).is_none());
}

#[test]
fn sweep_circle_segment_is_boundary_inclusive_and_reports_zero_normal_from_inside() {
    let segment = Segment::new(v(6, -1), v(6, 1));
    let circle = Circle::try_new(v(0, 0), Fx32::from_int(2)).unwrap();
    let hit = sweep_circle_segment(&circle, v(1, 0), &segment, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(4));
    assert_eq!(hit.point, v(4, 0));
    assert_eq!(hit.normal, v(-1, 0));

    let overlapping = Circle::try_new(v(6, 0), Fx32::from_int(2)).unwrap();
    let overlap_hit =
        sweep_circle_segment(&overlapping, v(1, 0), &segment, Fx32::from_int(10)).unwrap();
    assert_eq!(overlap_hit.toi, Fx32::ZERO);
    assert_eq!(overlap_hit.point, v(6, 0));
    assert_eq!(overlap_hit.normal, Vec2::ZERO);

    assert!(sweep_circle_segment(&circle, v(1, 0), &segment, Fx32::from_int(3)).is_none());
}

#[test]
fn sweep_circle_circle_is_boundary_inclusive_and_reports_zero_normal_from_inside() {
    let target = Circle::try_new(v(8, 0), Fx32::from_int(1)).unwrap();
    let circle = Circle::try_new(v(0, 0), Fx32::from_int(2)).unwrap();
    let hit = sweep_circle_circle(&circle, v(1, 0), &target, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(5));
    assert_eq!(hit.point, v(5, 0));
    assert_eq!(hit.normal, v(-1, 0));

    let overlapping = Circle::try_new(v(8, 0), Fx32::from_int(2)).unwrap();
    let overlap_hit =
        sweep_circle_circle(&overlapping, v(1, 0), &target, Fx32::from_int(10)).unwrap();
    assert_eq!(overlap_hit.toi, Fx32::ZERO);
    assert_eq!(overlap_hit.point, v(8, 0));
    assert_eq!(overlap_hit.normal, Vec2::ZERO);

    assert!(sweep_circle_circle(&circle, v(1, 0), &target, Fx32::from_int(4)).is_none());
}

#[test]
fn sweep_circle_convex_polygon_is_boundary_inclusive_and_reports_zero_normal_from_inside() {
    let polygon = ConvexPolygon::try_new([v(6, -2), v(10, -2), v(10, 2), v(6, 2)]).unwrap();
    let circle = Circle::try_new(v(0, 0), Fx32::from_int(2)).unwrap();
    let edge_capsule =
        Capsule::try_new(Segment::new(v(6, 2), v(6, -2)), Fx32::from_int(2)).unwrap();
    let edge_hit =
        raycast_capsule(&Ray::new(v(0, 0), v(1, 0)), &edge_capsule, Fx32::from_int(10)).unwrap();
    assert_eq!(edge_hit.toi, Fx32::from_int(4));
    let hit = sweep_circle_convex_polygon(&circle, v(1, 0), &polygon, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(4));
    assert_eq!(hit.point, v(4, 0));
    assert_eq!(hit.normal, v(-1, 0));

    let overlapping = Circle::try_new(v(7, 0), Fx32::from_int(2)).unwrap();
    let overlap_hit =
        sweep_circle_convex_polygon(&overlapping, v(1, 0), &polygon, Fx32::from_int(10)).unwrap();
    assert_eq!(overlap_hit.toi, Fx32::ZERO);
    assert_eq!(overlap_hit.point, v(7, 0));
    assert_eq!(overlap_hit.normal, Vec2::ZERO);

    assert!(sweep_circle_convex_polygon(&circle, v(1, 0), &polygon, Fx32::from_int(3)).is_none());
}

#[test]
fn sweep_circle_oriented_box_is_boundary_inclusive_and_reports_zero_normal_from_inside() {
    let oriented_box = OrientedBox::try_new(v(8, 0), v(2, 2), v(1, 0), v(0, 1)).unwrap();
    let circle = Circle::try_new(v(0, 0), Fx32::from_int(2)).unwrap();
    let hit =
        sweep_circle_oriented_box(&circle, v(1, 0), &oriented_box, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(4));
    assert_eq!(hit.point, v(4, 0));
    assert_eq!(hit.normal, v(-1, 0));

    let overlapping = Circle::try_new(v(8, 0), Fx32::from_int(2)).unwrap();
    let overlap_hit =
        sweep_circle_oriented_box(&overlapping, v(1, 0), &oriented_box, Fx32::from_int(10))
            .unwrap();
    assert_eq!(overlap_hit.toi, Fx32::ZERO);
    assert_eq!(overlap_hit.point, v(8, 0));
    assert_eq!(overlap_hit.normal, Vec2::ZERO);

    let rotated = OrientedBox::try_new(v(8, 0), v(2, 2), v(1, 1), v(-1, 1)).unwrap();
    assert!(sweep_circle_oriented_box(&circle, v(1, 0), &rotated, Fx32::from_int(10)).is_some());
    assert!(sweep_circle_oriented_box(&circle, v(1, 0), &oriented_box, Fx32::from_int(3)).is_none());
}

#[test]
fn sweep_capsule_circle_is_boundary_inclusive_and_uses_midpoint_reference() {
    let capsule = Capsule::try_new(Segment::new(v(-1, 0), v(1, 0)), Fx32::from_int(1)).unwrap();
    let target = Circle::try_new(v(8, 0), Fx32::from_int(1)).unwrap();
    let hit = sweep_capsule_circle(&capsule, v(1, 0), &target, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(5));
    assert_eq!(hit.point, v(5, 0));
    assert_eq!(hit.normal, v(-1, 0));

    let overlapping = Capsule::try_new(Segment::new(v(7, 0), v(9, 0)), Fx32::from_int(1)).unwrap();
    let overlap_hit =
        sweep_capsule_circle(&overlapping, v(1, 0), &target, Fx32::from_int(10)).unwrap();
    assert_eq!(overlap_hit.toi, Fx32::ZERO);
    assert_eq!(overlap_hit.point, v(8, 0));
    assert_eq!(overlap_hit.normal, Vec2::ZERO);

    assert!(sweep_capsule_circle(&capsule, v(1, 0), &target, Fx32::from_int(4)).is_none());
}

#[test]
fn sweep_capsule_segment_is_boundary_inclusive_and_uses_midpoint_reference() {
    let capsule = Capsule::try_new(Segment::new(v(-1, 0), v(1, 0)), Fx32::from_int(1)).unwrap();
    let target = Segment::new(v(8, -1), v(8, 1));
    let hit = sweep_capsule_segment(&capsule, v(1, 0), &target, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(6));
    assert_eq!(hit.point, v(6, 0));
    assert_eq!(hit.normal, v(-1, 0));

    let overlapping = Capsule::try_new(Segment::new(v(7, 0), v(9, 0)), Fx32::from_int(1)).unwrap();
    let overlap_hit =
        sweep_capsule_segment(&overlapping, v(1, 0), &target, Fx32::from_int(10)).unwrap();
    assert_eq!(overlap_hit.toi, Fx32::ZERO);
    assert_eq!(overlap_hit.point, v(8, 0));
    assert_eq!(overlap_hit.normal, Vec2::ZERO);

    assert!(sweep_capsule_segment(&capsule, v(1, 0), &target, Fx32::from_int(5)).is_none());
}

#[test]
fn sweep_capsule_capsule_is_boundary_inclusive_and_uses_midpoint_reference() {
    let moving = Capsule::try_new(Segment::new(v(-1, 0), v(1, 0)), Fx32::from_int(1)).unwrap();
    let target = Capsule::try_new(Segment::new(v(10, -1), v(10, 1)), Fx32::from_int(1)).unwrap();
    let hit = sweep_capsule_capsule(&moving, v(1, 0), &target, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(7));
    assert_eq!(hit.point, v(7, 0));
    assert_eq!(hit.normal, v(-1, 0));

    let overlapping = Capsule::try_new(Segment::new(v(9, 0), v(11, 0)), Fx32::from_int(1)).unwrap();
    let overlap_hit =
        sweep_capsule_capsule(&overlapping, v(1, 0), &target, Fx32::from_int(10)).unwrap();
    assert_eq!(overlap_hit.toi, Fx32::ZERO);
    assert_eq!(overlap_hit.point, v(10, 0));
    assert_eq!(overlap_hit.normal, Vec2::ZERO);

    assert!(sweep_capsule_capsule(&moving, v(1, 0), &target, Fx32::from_int(6)).is_none());
}

#[test]
fn sweep_capsule_aabb_is_boundary_inclusive_and_uses_midpoint_reference() {
    let capsule = Capsule::try_new(Segment::new(v(-1, 0), v(1, 0)), Fx32::from_int(1)).unwrap();
    let target = Aabb::try_new(v(8, -2), v(12, 2)).unwrap();
    let hit = sweep_capsule_aabb(&capsule, v(1, 0), &target, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(6));
    assert_eq!(hit.point, v(6, 0));
    assert_eq!(hit.normal, v(-1, 0));

    let overlapping = Capsule::try_new(Segment::new(v(8, 0), v(10, 0)), Fx32::from_int(1)).unwrap();
    let overlap_hit =
        sweep_capsule_aabb(&overlapping, v(1, 0), &target, Fx32::from_int(10)).unwrap();
    assert_eq!(overlap_hit.toi, Fx32::ZERO);
    assert_eq!(overlap_hit.point, v(9, 0));
    assert_eq!(overlap_hit.normal, Vec2::ZERO);

    assert!(sweep_capsule_aabb(&capsule, v(1, 0), &target, Fx32::from_int(5)).is_none());
}

#[test]
fn sweep_capsule_convex_polygon_is_boundary_inclusive_and_uses_midpoint_reference() {
    let capsule = Capsule::try_new(Segment::new(v(-1, 0), v(1, 0)), Fx32::from_int(1)).unwrap();
    let target = ConvexPolygon::try_new([v(8, -2), v(12, -2), v(12, 2), v(8, 2)]).unwrap();
    let hit = sweep_capsule_convex_polygon(&capsule, v(1, 0), &target, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(6));
    assert_eq!(hit.point, v(6, 0));
    assert_eq!(hit.normal, v(-1, 0));

    let overlapping = Capsule::try_new(Segment::new(v(8, 0), v(10, 0)), Fx32::from_int(1)).unwrap();
    let overlap_hit =
        sweep_capsule_convex_polygon(&overlapping, v(1, 0), &target, Fx32::from_int(10)).unwrap();
    assert_eq!(overlap_hit.toi, Fx32::ZERO);
    assert_eq!(overlap_hit.point, v(9, 0));
    assert_eq!(overlap_hit.normal, Vec2::ZERO);

    assert!(sweep_capsule_convex_polygon(&capsule, v(1, 0), &target, Fx32::from_int(5)).is_none());
}

#[test]
fn sweep_capsule_oriented_box_is_boundary_inclusive_and_uses_midpoint_reference() {
    let capsule = Capsule::try_new(Segment::new(v(-1, 0), v(1, 0)), Fx32::from_int(1)).unwrap();
    let target = OrientedBox::try_new(v(10, 0), v(2, 2), v(1, 0), v(0, 1)).unwrap();
    let hit = sweep_capsule_oriented_box(&capsule, v(1, 0), &target, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(6));
    assert_eq!(hit.point, v(6, 0));
    assert_eq!(hit.normal, v(-1, 0));

    let overlapping = Capsule::try_new(Segment::new(v(8, 0), v(10, 0)), Fx32::from_int(1)).unwrap();
    let overlap_hit =
        sweep_capsule_oriented_box(&overlapping, v(1, 0), &target, Fx32::from_int(10)).unwrap();
    assert_eq!(overlap_hit.toi, Fx32::ZERO);
    assert_eq!(overlap_hit.point, v(9, 0));
    assert_eq!(overlap_hit.normal, Vec2::ZERO);

    let rotated = OrientedBox::try_new(v(10, 0), v(2, 2), v(1, 1), v(-1, 1)).unwrap();
    assert!(sweep_capsule_oriented_box(&capsule, v(1, 0), &rotated, Fx32::from_int(10)).is_some());
    assert!(sweep_capsule_oriented_box(&capsule, v(1, 0), &target, Fx32::from_int(5)).is_none());
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
fn segment3_closest_point_and_distance_clamp_to_endpoints_and_interior() {
    let segment = Segment3::new(v3(0, 0, 0), v3(4, 0, 0));
    assert_eq!(closest_point_on_segment3(&segment, v3(2, 3, 0)), v3(2, 0, 0));
    assert_eq!(distance_squared_point_segment3(&segment, v3(2, 3, 0)), Fx32::from_int(9));
    assert_eq!(closest_point_on_segment3(&segment, v3(-1, 1, 0)), v3(0, 0, 0));

    let degenerate = Segment3::new(v3(1, 1, 1), v3(1, 1, 1));
    assert_eq!(closest_point_on_segment3(&degenerate, v3(4, 5, 1)), v3(1, 1, 1));
    assert_eq!(distance_squared_point_segment3(&degenerate, v3(4, 5, 1)), Fx32::from_int(25));
}

#[test]
fn sphere_closest_point_and_distance_are_zero_inside_and_boundary_clamped() {
    let sphere = Sphere::try_new(v3(0, 0, 0), Fx32::from_int(2)).unwrap();
    assert_eq!(closest_point_on_sphere(&sphere, v3(1, 0, 0)), v3(1, 0, 0));
    assert_eq!(distance_squared_point_sphere(&sphere, v3(1, 0, 0)), Fx32::ZERO);
    assert_eq!(closest_point_on_sphere(&sphere, v3(5, 0, 0)), v3(2, 0, 0));
    assert_eq!(distance_squared_point_sphere(&sphere, v3(5, 0, 0)), Fx32::from_int(9));

    let degenerate = Sphere::try_new(v3(1, 1, 1), Fx32::ZERO).unwrap();
    assert_eq!(closest_point_on_sphere(&degenerate, v3(4, 5, 1)), v3(1, 1, 1));
    assert_eq!(distance_squared_point_sphere(&degenerate, v3(4, 5, 1)), Fx32::from_int(25));
}

#[test]
fn capsule3_closest_point_and_distance_handle_inside_and_degenerate_segments() {
    let capsule =
        Capsule3::try_new(Segment3::new(v3(0, 0, 0), v3(4, 0, 0)), Fx32::from_int(1)).unwrap();
    assert_eq!(closest_point_on_capsule3(&capsule, v3(2, 0, 0)), v3(2, 0, 0));
    assert_eq!(distance_squared_point_capsule3(&capsule, v3(2, 0, 0)), Fx32::ZERO);
    assert_eq!(closest_point_on_capsule3(&capsule, v3(2, 3, 0)), v3(2, 1, 0));
    assert_eq!(distance_squared_point_capsule3(&capsule, v3(2, 3, 0)), Fx32::from_int(4));

    let degenerate =
        Capsule3::try_new(Segment3::new(v3(1, 1, 1), v3(1, 1, 1)), Fx32::from_int(2)).unwrap();
    assert_eq!(closest_point_on_capsule3(&degenerate, v3(5, 1, 1)), v3(3, 1, 1));
    assert_eq!(distance_squared_point_capsule3(&degenerate, v3(5, 1, 1)), Fx32::from_int(4));
}

#[test]
fn triangle3_closest_point_and_distance_handle_inside_and_degenerate_triangles() {
    let triangle = Triangle3::new(v3(0, 0, 0), v3(4, 0, 0), v3(0, 4, 0));
    assert_eq!(closest_point_on_triangle3(&triangle, v3(1, 1, 0)), v3(1, 1, 0));
    assert_eq!(distance_squared_point_triangle3(&triangle, v3(1, 1, 0)), Fx32::ZERO);
    assert_eq!(closest_point_on_triangle3(&triangle, v3(3, 3, 0)), v3(2, 2, 0));
    assert_eq!(distance_squared_point_triangle3(&triangle, v3(3, 3, 0)), Fx32::from_int(2));

    let degenerate = Triangle3::new(v3(0, 0, 0), v3(4, 0, 0), v3(8, 0, 0));
    assert_eq!(closest_point_on_triangle3(&degenerate, v3(5, 2, 0)), v3(5, 0, 0));
    assert_eq!(distance_squared_point_triangle3(&degenerate, v3(5, 2, 0)), Fx32::from_int(4));
}

#[test]
fn sphere_overlaps_triangle3_handles_face_interior_and_line_degenerates() {
    let triangle = Triangle3::new(v3(0, 0, 0), v3(4, 0, 0), v3(0, 4, 0));
    let touching = Sphere::try_new(v3(1, 1, 2), Fx32::from_int(2)).unwrap();
    let separated = Sphere::try_new(v3(5, 5, 1), Fx32::from_int(1)).unwrap();
    let degenerate = Triangle3::new(v3(0, 0, 0), v3(4, 0, 0), v3(8, 0, 0));
    let degenerate_touching = Sphere::try_new(v3(5, 2, 0), Fx32::from_int(2)).unwrap();

    assert!(sphere_overlaps_triangle3(&touching, &triangle));
    assert!(!sphere_overlaps_triangle3(&separated, &triangle));
    assert!(sphere_overlaps_triangle3(&degenerate_touching, &degenerate));
}

#[test]
fn capsule3_overlaps_triangle3_handles_face_crossings_and_line_degenerates() {
    let triangle = Triangle3::new(v3(0, 0, 0), v3(4, 0, 0), v3(0, 4, 0));
    let touching =
        Capsule3::try_new(Segment3::new(v3(1, 1, 2), v3(1, 1, 4)), Fx32::from_int(2)).unwrap();
    let crossing = Capsule3::try_new(Segment3::new(v3(1, 1, -1), v3(1, 1, 1)), Fx32::ZERO).unwrap();
    let separated =
        Capsule3::try_new(Segment3::new(v3(5, 5, 2), v3(5, 5, 4)), Fx32::from_int(1)).unwrap();
    let degenerate = Triangle3::new(v3(0, 0, 0), v3(4, 0, 0), v3(8, 0, 0));
    let degenerate_touching =
        Capsule3::try_new(Segment3::new(v3(5, 2, 0), v3(5, 2, 0)), Fx32::from_int(2)).unwrap();

    assert!(capsule3_overlaps_triangle3(&touching, &triangle));
    assert!(capsule3_overlaps_triangle3(&crossing, &triangle));
    assert!(!capsule3_overlaps_triangle3(&separated, &triangle));
    assert!(capsule3_overlaps_triangle3(&degenerate_touching, &degenerate));
}

#[test]
fn triangle3_overlaps_aabb3_counts_boundary_touch_and_face_slice_cases() {
    let aabb = Aabb3::try_new(v3(0, 0, 0), v3(2, 2, 2)).unwrap();
    let touching = Triangle3::new(v3(2, 0, 0), v3(2, 2, 0), v3(2, 0, 2));
    let contained = Triangle3::new(v3(1, 1, 1), v3(2, 1, 1), v3(1, 2, 1));
    let sliced = Triangle3::new(v3(1, -1, -1), v3(1, 3, -1), v3(1, -1, 3));
    let separated = Triangle3::new(v3(3, 0, 0), v3(3, 2, 0), v3(3, 0, 2));

    assert!(triangle3_overlaps_aabb3(&touching, &aabb));
    assert!(triangle3_overlaps_aabb3(&contained, &aabb));
    assert!(triangle3_overlaps_aabb3(&sliced, &aabb));
    assert!(!triangle3_overlaps_aabb3(&separated, &aabb));
}

#[test]
fn sphere_overlaps_triangle3_counts_boundary_touch_and_degenerate_triangles() {
    let triangle = Triangle3::new(v3(0, 0, 0), v3(4, 0, 0), v3(0, 4, 0));
    let touching = Sphere::try_new(v3(2, 2, 1), Fx32::from_int(1)).unwrap();
    let separated = Sphere::try_new(v3(2, 2, 2), Fx32::from_int(1)).unwrap();
    let degenerate = Triangle3::new(v3(6, 0, 0), v3(10, 0, 0), v3(14, 0, 0));
    let degenerate_touching = Sphere::try_new(v3(8, 2, 0), Fx32::from_int(2)).unwrap();

    assert!(sphere_overlaps_triangle3(&touching, &triangle));
    assert!(!sphere_overlaps_triangle3(&separated, &triangle));
    assert!(sphere_overlaps_triangle3(&degenerate_touching, &degenerate));
}

#[test]
fn capsule3_overlaps_triangle3_counts_boundary_touch_and_degenerate_triangles() {
    let triangle = Triangle3::new(v3(8, -2, -2), v3(8, 2, -2), v3(8, 0, 2));
    let touching =
        Capsule3::try_new(Segment3::new(v3(4, 0, 0), v3(6, 0, 0)), Fx32::from_int(2)).unwrap();
    let penetrating =
        Capsule3::try_new(Segment3::new(v3(6, 0, 0), v3(10, 0, 0)), Fx32::from_int(0)).unwrap();
    let separated =
        Capsule3::try_new(Segment3::new(v3(4, 4, 0), v3(6, 4, 0)), Fx32::from_int(1)).unwrap();
    let degenerate = Triangle3::new(v3(0, 6, 0), v3(4, 6, 0), v3(8, 6, 0));
    let degenerate_touching =
        Capsule3::try_new(Segment3::new(v3(4, 9, 0), v3(4, 11, 0)), Fx32::from_int(3)).unwrap();

    assert!(bumpbox_core::capsule3_overlaps_triangle3(&touching, &triangle));
    assert!(bumpbox_core::capsule3_overlaps_triangle3(&penetrating, &triangle));
    assert!(!bumpbox_core::capsule3_overlaps_triangle3(&separated, &triangle));
    assert!(bumpbox_core::capsule3_overlaps_triangle3(&degenerate_touching, &degenerate));
}

#[test]
fn segment3_overlaps_triangle3_counts_boundary_touch_and_degenerate_triangles() {
    let triangle = Triangle3::new(v3(8, -2, -2), v3(8, 2, -2), v3(8, 0, 2));
    let touching = Segment3::new(v3(4, 0, 0), v3(8, 0, 0));
    let separated = Segment3::new(v3(4, 4, 0), v3(6, 4, 0));
    let degenerate = Triangle3::new(v3(0, 6, 0), v3(4, 6, 0), v3(8, 6, 0));
    let degenerate_touching = Segment3::new(v3(4, 4, 0), v3(4, 8, 0));

    assert!(segment3_overlaps_triangle3(&touching, &triangle));
    assert!(!segment3_overlaps_triangle3(&separated, &triangle));
    assert!(segment3_overlaps_triangle3(&degenerate_touching, &degenerate));
}

#[test]
fn segment3_overlaps_triangle3_handles_face_crossings_and_point_degenerates() {
    let triangle = Triangle3::new(v3(8, -2, -2), v3(8, 2, -2), v3(8, 0, 2));
    let crossing = Segment3::new(v3(6, 0, 0), v3(10, 0, 0));
    let separated = Segment3::new(v3(6, 0, 4), v3(10, 0, 4));
    let point_touch = Segment3::new(v3(8, 0, 0), v3(8, 0, 0));

    assert!(segment3_overlaps_triangle3(&crossing, &triangle));
    assert!(!segment3_overlaps_triangle3(&separated, &triangle));
    assert!(segment3_overlaps_triangle3(&point_touch, &triangle));
}

#[test]
fn closest_points_segment3_triangle3_handle_face_interiors_and_degenerate_triangles() {
    let triangle = Triangle3::new(v3(8, -2, -2), v3(8, 2, -2), v3(8, 0, 2));
    let face_offset = Segment3::new(v3(4, 0, 1), v3(4, 4, 1));
    let face_pair = closest_points_segment3_triangle3(&face_offset, &triangle);
    assert_eq!(face_pair.segment_point, v3(4, 0, 1));
    assert_eq!(face_pair.triangle_point, v3(8, 0, 1));

    let degenerate = Triangle3::new(v3(0, 6, 0), v3(4, 6, 0), v3(8, 6, 0));
    let degenerate_offset = Segment3::new(v3(4, 8, 0), v3(4, 10, 0));
    let degenerate_pair = closest_points_segment3_triangle3(&degenerate_offset, &degenerate);
    assert_eq!(degenerate_pair.segment_point, v3(4, 8, 0));
    assert_eq!(degenerate_pair.triangle_point, v3(4, 6, 0));
}

#[test]
fn distance_squared_segment3_triangle3_is_zero_on_touch_and_crossing() {
    let triangle = Triangle3::new(v3(0, 0, 0), v3(4, 0, 0), v3(0, 4, 0));
    let touching = Segment3::new(v3(4, 0, 0), v3(6, 0, 0));
    let crossing = Segment3::new(v3(1, 1, -1), v3(1, 1, 1));
    let separated = Segment3::new(v3(1, 1, 2), v3(1, 1, 4));
    let degenerate = Triangle3::new(v3(0, 6, 0), v3(4, 6, 0), v3(8, 6, 0));
    let degenerate_crossing = Segment3::new(v3(4, 4, 0), v3(4, 8, 0));

    assert_eq!(distance_squared_segment3_triangle3(&touching, &triangle), Fx32::ZERO);
    assert_eq!(distance_squared_segment3_triangle3(&crossing, &triangle), Fx32::ZERO);
    assert_eq!(distance_squared_segment3_triangle3(&separated, &triangle), Fx32::from_int(4));
    assert_eq!(distance_squared_segment3_triangle3(&degenerate_crossing, &degenerate), Fx32::ZERO);
}

#[test]
fn closest_points_sphere_triangle3_handle_face_offsets_and_degenerate_triangles() {
    let triangle = Triangle3::new(v3(8, -2, -2), v3(8, 2, -2), v3(8, 0, 2));
    let sphere = Sphere::try_new(v3(2, 0, 1), Fx32::from_int(2)).unwrap();
    let pair = closest_points_sphere_triangle3(&sphere, &triangle);
    assert_eq!(pair.point_a, v3(4, 0, 1));
    assert_eq!(pair.point_b, v3(8, 0, 1));

    let degenerate = Triangle3::new(v3(0, 6, 0), v3(4, 6, 0), v3(8, 6, 0));
    let touching = Sphere::try_new(v3(4, 8, 0), Fx32::from_int(2)).unwrap();
    let degenerate_pair = closest_points_sphere_triangle3(&touching, &degenerate);
    assert_eq!(degenerate_pair.point_a, v3(4, 6, 0));
    assert_eq!(degenerate_pair.point_b, v3(4, 6, 0));
}

#[test]
fn distance_squared_sphere_triangle3_is_zero_on_touch_and_overlap() {
    let triangle = Triangle3::new(v3(0, 0, 0), v3(4, 0, 0), v3(0, 4, 0));
    let touching = Sphere::try_new(v3(1, 1, 2), Fx32::from_int(2)).unwrap();
    let overlapping = Sphere::try_new(v3(1, 1, 1), Fx32::from_int(2)).unwrap();
    let separated = Sphere::try_new(v3(1, 1, 4), Fx32::from_int(1)).unwrap();

    assert_eq!(distance_squared_sphere_triangle3(&touching, &triangle), Fx32::ZERO);
    assert_eq!(distance_squared_sphere_triangle3(&overlapping, &triangle), Fx32::ZERO);
    assert_eq!(distance_squared_sphere_triangle3(&separated, &triangle), Fx32::from_int(9));
}

#[test]
fn closest_points_sphere_triangle3_collapse_to_contact_point_on_touch_and_overlap() {
    let triangle = Triangle3::new(v3(0, 0, 0), v3(4, 0, 0), v3(0, 4, 0));
    let touching = Sphere::try_new(v3(1, 1, 2), Fx32::from_int(2)).unwrap();
    let overlapping = Sphere::try_new(v3(1, 1, 1), Fx32::from_int(2)).unwrap();

    let touching_pair = closest_points_sphere_triangle3(&touching, &triangle);
    assert_eq!(touching_pair.point_a, v3(1, 1, 0));
    assert_eq!(touching_pair.point_b, v3(1, 1, 0));

    let overlapping_pair = closest_points_sphere_triangle3(&overlapping, &triangle);
    assert_eq!(overlapping_pair.point_a, v3(1, 1, 0));
    assert_eq!(overlapping_pair.point_b, v3(1, 1, 0));
}

#[test]
fn closest_points_sphere_triangle3_handle_skew_separated_spheres() {
    let triangle = Triangle3::new(v3(0, 0, 0), v3(3, 0, -3), v3(0, 6, -3));
    let sphere = Sphere::try_new(v3(3, 3, 0), Fx32::ONE).unwrap();

    let pair = closest_points_sphere_triangle3(&sphere, &triangle);
    let expected_point_b = closest_point_on_triangle3(&triangle, sphere.center);
    let expected_point_a = closest_point_on_sphere(&sphere, expected_point_b);
    assert_eq!(pair.point_a, expected_point_a);
    assert_eq!(pair.point_b, expected_point_b);
    assert_eq!(
        distance_squared_sphere_triangle3(&sphere, &triangle),
        (expected_point_a - expected_point_b).length_squared()
    );
}

#[test]
fn closest_points_capsule3_triangle3_handle_face_offsets_and_degenerate_triangles() {
    let triangle = Triangle3::new(v3(8, -2, -2), v3(8, 2, -2), v3(8, 0, 2));
    let capsule =
        Capsule3::try_new(Segment3::new(v3(2, 0, 1), v3(4, 0, 1)), Fx32::from_int(2)).unwrap();
    let pair = closest_points_capsule3_triangle3(&capsule, &triangle);
    assert_eq!(pair.point_a, v3(6, 0, 1));
    assert_eq!(pair.point_b, v3(8, 0, 1));

    let degenerate = Triangle3::new(v3(0, 6, 0), v3(4, 6, 0), v3(8, 6, 0));
    let touching =
        Capsule3::try_new(Segment3::new(v3(4, 8, 0), v3(4, 10, 0)), Fx32::from_int(2)).unwrap();
    let degenerate_pair = closest_points_capsule3_triangle3(&touching, &degenerate);
    assert_eq!(degenerate_pair.point_a, v3(4, 6, 0));
    assert_eq!(degenerate_pair.point_b, v3(4, 6, 0));
}

#[test]
fn distance_squared_capsule3_triangle3_is_zero_on_touch_and_crossing() {
    let triangle = Triangle3::new(v3(0, 0, 0), v3(4, 0, 0), v3(0, 4, 0));
    let touching =
        Capsule3::try_new(Segment3::new(v3(1, 1, 4), v3(1, 1, 6)), Fx32::from_int(4)).unwrap();
    let crossing = Capsule3::try_new(Segment3::new(v3(1, 1, -1), v3(1, 1, 1)), Fx32::ZERO).unwrap();
    let separated =
        Capsule3::try_new(Segment3::new(v3(1, 1, 4), v3(1, 1, 6)), Fx32::from_int(1)).unwrap();

    assert_eq!(distance_squared_capsule3_triangle3(&touching, &triangle), Fx32::ZERO);
    assert_eq!(distance_squared_capsule3_triangle3(&crossing, &triangle), Fx32::ZERO);
    assert_eq!(distance_squared_capsule3_triangle3(&separated, &triangle), Fx32::from_int(9));
}

#[test]
fn closest_points_capsule3_triangle3_collapse_to_contact_point_on_touch_and_crossing() {
    let triangle = Triangle3::new(v3(0, 0, 0), v3(4, 0, 0), v3(0, 4, 0));
    let touching =
        Capsule3::try_new(Segment3::new(v3(1, 1, 4), v3(1, 1, 6)), Fx32::from_int(4)).unwrap();
    let crossing = Capsule3::try_new(Segment3::new(v3(1, 1, -1), v3(1, 1, 1)), Fx32::ZERO).unwrap();

    let touching_pair = closest_points_capsule3_triangle3(&touching, &triangle);
    assert_eq!(touching_pair.point_a, v3(1, 1, 0));
    assert_eq!(touching_pair.point_b, v3(1, 1, 0));

    let crossing_pair = closest_points_capsule3_triangle3(&crossing, &triangle);
    assert_eq!(crossing_pair.point_a, v3(1, 1, 0));
    assert_eq!(crossing_pair.point_b, v3(1, 1, 0));
}

#[test]
fn closest_points_capsule3_triangle3_handle_skew_separated_capsules() {
    let triangle = Triangle3::new(v3(0, 0, 0), v3(0, 0, 4), v3(4, -3, 0));
    let capsule =
        Capsule3::try_new(Segment3::new(v3(3, 4, 2), v3(6, 8, 2)), Fx32::from_int(1)).unwrap();

    let pair = closest_points_capsule3_triangle3(&capsule, &triangle);
    let inward_step = Fx32::from_int(1) / Fx32::from_int(5);
    let expected_point_a = Vec3::new(
        Fx32::from_int(3) - Fx32::from_int(3) * inward_step,
        Fx32::from_int(4) - Fx32::from_int(4) * inward_step,
        Fx32::from_int(2),
    );
    let expected_point_b = v3(0, 0, 2);
    assert_eq!(pair.point_a, expected_point_a);
    assert_eq!(pair.point_b, expected_point_b);
    assert_eq!(
        distance_squared_capsule3_triangle3(&capsule, &triangle),
        (expected_point_a - expected_point_b).length_squared()
    );
}

#[test]
fn triangle3_overlaps_aabb3_counts_boundary_touch_and_degenerate_triangles() {
    let aabb = Aabb3::try_new(v3(0, 0, 0), v3(4, 4, 4)).unwrap();
    let touching = Triangle3::new(v3(4, 1, 1), v3(6, 1, 1), v3(4, 3, 1));
    let crossing = Triangle3::new(v3(2, 2, -1), v3(6, 2, 2), v3(2, 6, 2));
    let separated = Triangle3::new(v3(5, 1, 1), v3(7, 1, 1), v3(5, 3, 1));
    let degenerate = Triangle3::new(v3(-2, 2, 2), v3(0, 2, 2), v3(2, 2, 2));

    assert!(triangle3_overlaps_aabb3(&touching, &aabb));
    assert!(triangle3_overlaps_aabb3(&crossing, &aabb));
    assert!(!triangle3_overlaps_aabb3(&separated, &aabb));
    assert!(triangle3_overlaps_aabb3(&degenerate, &aabb));
}

#[test]
fn triangle3_overlaps_triangle3_counts_boundary_touch_and_degenerate_triangles() {
    let triangle = Triangle3::new(v3(0, 0, 0), v3(4, 0, 0), v3(0, 4, 0));
    let touching = Triangle3::new(v3(4, 0, 0), v3(6, 0, 0), v3(4, 2, 0));
    let crossing = Triangle3::new(v3(1, 1, -1), v3(3, 1, 1), v3(1, 3, 1));
    let separated = Triangle3::new(v3(5, 0, 0), v3(7, 0, 0), v3(5, 2, 0));
    let degenerate = Triangle3::new(v3(0, 6, 0), v3(4, 6, 0), v3(8, 6, 0));
    let degenerate_touching = Triangle3::new(v3(4, 4, 0), v3(4, 8, 0), v3(4, 12, 0));

    assert!(triangle3_overlaps_triangle3(&triangle, &touching));
    assert!(triangle3_overlaps_triangle3(&triangle, &crossing));
    assert!(!triangle3_overlaps_triangle3(&triangle, &separated));
    assert!(triangle3_overlaps_triangle3(&degenerate, &degenerate_touching));
}

#[test]
fn triangle3_overlaps_triangle3_handles_containment_and_symmetry() {
    let outer = Triangle3::new(v3(0, 0, 0), v3(6, 0, 0), v3(0, 6, 0));
    let inner = Triangle3::new(v3(1, 1, 0), v3(2, 1, 0), v3(1, 2, 0));
    let skew = Triangle3::new(v3(1, 1, -1), v3(2, 1, 1), v3(1, 2, 1));
    let separated = Triangle3::new(v3(7, 1, 0), v3(8, 1, 0), v3(7, 2, 0));

    assert!(triangle3_overlaps_triangle3(&outer, &inner));
    assert!(triangle3_overlaps_triangle3(&inner, &outer));
    assert!(triangle3_overlaps_triangle3(&outer, &skew));
    assert!(!triangle3_overlaps_triangle3(&outer, &separated));
}

#[test]
fn closest_points_triangle3_triangle3_handle_separated_and_degenerate_triangles() {
    let triangle = Triangle3::new(v3(8, -2, -2), v3(8, 2, -2), v3(8, 0, 2));
    let point_triangle = Triangle3::new(v3(14, 0, 1), v3(14, 0, 1), v3(14, 0, 1));
    let pair = closest_points_triangle3_triangle3(&triangle, &point_triangle);
    assert_eq!(pair.point_a, v3(8, 0, 1));
    assert_eq!(pair.point_b, v3(14, 0, 1));

    let line_triangle = Triangle3::new(v3(0, 6, 0), v3(4, 6, 0), v3(8, 6, 0));
    let offset_triangle = Triangle3::new(v3(4, 10, 0), v3(4, 12, 0), v3(4, 14, 0));
    let degenerate_pair = closest_points_triangle3_triangle3(&line_triangle, &offset_triangle);
    assert_eq!(degenerate_pair.point_a, v3(4, 6, 0));
    assert_eq!(degenerate_pair.point_b, v3(4, 10, 0));
}

#[test]
fn distance_squared_triangle3_triangle3_is_zero_on_overlap_and_positive_when_separated() {
    let triangle = Triangle3::new(v3(0, 0, 0), v3(4, 0, 0), v3(0, 4, 0));
    let touching = Triangle3::new(v3(4, 0, 0), v3(6, 0, 0), v3(4, 2, 0));
    let crossing = Triangle3::new(v3(1, 1, -1), v3(3, 1, 1), v3(1, 3, 1));
    let separated = Triangle3::new(v3(6, 0, 1), v3(8, 0, 1), v3(6, 2, 1));

    assert_eq!(distance_squared_triangle3_triangle3(&triangle, &touching), Fx32::ZERO);
    assert_eq!(distance_squared_triangle3_triangle3(&triangle, &crossing), Fx32::ZERO);
    assert_eq!(distance_squared_triangle3_triangle3(&triangle, &separated), Fx32::from_int(5));
}

#[test]
fn closest_points_triangle3_aabb3_handle_separated_and_degenerate_triangles() {
    let triangle = Triangle3::new(v3(8, -2, -2), v3(8, 2, -2), v3(8, 0, 2));
    let aabb = Aabb3::try_new(v3(0, -1, -1), v3(4, 1, 1)).unwrap();
    let pair = closest_points_triangle3_aabb3(&triangle, &aabb);
    assert_eq!(pair.point_a, v3(8, -1, -1));
    assert_eq!(pair.point_b, v3(4, -1, -1));

    let degenerate = Triangle3::new(v3(6, 6, 1), v3(6, 6, 1), v3(6, 6, 1));
    let degenerate_pair = closest_points_triangle3_aabb3(&degenerate, &aabb);
    assert_eq!(degenerate_pair.point_a, v3(6, 6, 1));
    assert_eq!(degenerate_pair.point_b, v3(4, 1, 1));
}

#[test]
fn closest_points_triangle3_aabb3_handle_face_interior_offsets() {
    let triangle = Triangle3::new(v3(0, 0, 0), v3(0, 4, 0), v3(0, 0, 4));
    let aabb = Aabb3::try_new(v3(3, 0, 0), v3(5, 2, 2)).unwrap();

    let pair = closest_points_triangle3_aabb3(&triangle, &aabb);
    assert_eq!(pair.point_a, v3(0, 1, 1));
    assert_eq!(pair.point_b, v3(3, 1, 1));
    assert_eq!(distance_squared_triangle3_aabb3(&triangle, &aabb), Fx32::from_int(9));
}

#[test]
fn closest_points_triangle3_aabb3_handle_face_interior_offsets_on_multiple_axes() {
    let cases = [
        (
            Triangle3::new(v3(6, 5, 7), v3(6, 9, 7), v3(6, 5, 11)),
            Aabb3::try_new(v3(9, 5, 7), v3(11, 7, 9)).unwrap(),
            v3(6, 6, 8),
            v3(9, 6, 8),
        ),
        (
            Triangle3::new(v3(-4, 3, -2), v3(-4, 7, -2), v3(-4, 3, 2)),
            Aabb3::try_new(v3(-1, 3, -2), v3(1, 5, 0)).unwrap(),
            v3(-4, 4, -1),
            v3(-1, 4, -1),
        ),
    ];

    for (triangle, aabb, expected_triangle_point, expected_aabb_point) in cases {
        let pair = closest_points_triangle3_aabb3(&triangle, &aabb);
        assert_eq!(pair.point_a, expected_triangle_point);
        assert_eq!(pair.point_b, expected_aabb_point);
        assert_eq!(distance_squared_triangle3_aabb3(&triangle, &aabb), Fx32::from_int(9));
    }
}

#[test]
fn closest_points_triangle3_aabb3_handle_degenerate_boxes() {
    let triangle = Triangle3::new(v3(8, -2, -2), v3(8, 2, -2), v3(8, 0, 2));
    let point_aabb = Aabb3::try_new(v3(4, 1, 1), v3(4, 1, 1)).unwrap();

    let pair = closest_points_triangle3_aabb3(&triangle, &point_aabb);
    assert_eq!(pair.point_b, v3(4, 1, 1));
    assert_eq!(pair.point_a, closest_point_on_triangle3(&triangle, pair.point_b));
    assert_eq!(
        distance_squared_triangle3_aabb3(&triangle, &point_aabb),
        (pair.point_a - pair.point_b).length_squared()
    );
}

#[test]
fn closest_points_triangle3_aabb3_collapse_to_contact_point_on_touch_and_overlap() {
    let aabb = Aabb3::try_new(v3(0, 0, 0), v3(4, 4, 4)).unwrap();
    let touching = Triangle3::new(v3(4, 1, 1), v3(4, 3, 1), v3(4, 1, 3));
    let crossing = Triangle3::new(v3(2, -1, -1), v3(2, 5, -1), v3(2, -1, 5));

    let touching_pair = closest_points_triangle3_aabb3(&touching, &aabb);
    assert_eq!(touching_pair.point_a, v3(4, 1, 1));
    assert_eq!(touching_pair.point_b, touching_pair.point_a);

    let crossing_pair = closest_points_triangle3_aabb3(&crossing, &aabb);
    assert_eq!(crossing_pair.point_a, v3(2, 0, 0));
    assert_eq!(crossing_pair.point_b, crossing_pair.point_a);
}

#[test]
fn distance_squared_triangle3_aabb3_is_zero_on_touch_and_overlap() {
    let aabb = Aabb3::try_new(v3(0, 0, 0), v3(4, 4, 4)).unwrap();
    let touching = Triangle3::new(v3(4, 1, 1), v3(4, 3, 1), v3(4, 1, 3));
    let crossing = Triangle3::new(v3(2, -1, -1), v3(2, 5, -1), v3(2, -1, 5));
    let separated = Triangle3::new(v3(8, -2, -2), v3(8, 2, -2), v3(8, 0, 2));

    assert_eq!(distance_squared_triangle3_aabb3(&touching, &aabb), Fx32::ZERO);
    assert_eq!(distance_squared_triangle3_aabb3(&crossing, &aabb), Fx32::ZERO);
    assert_eq!(distance_squared_triangle3_aabb3(&separated, &aabb), Fx32::from_int(16));
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

#[test]
fn raycast_sphere_is_boundary_inclusive_and_reports_zero_normal_from_inside() {
    let sphere = Sphere::try_new(v3(3, 0, 0), Fx32::from_int(2)).unwrap();
    let incoming = Ray3::new(v3(0, 0, 0), v3(1, 0, 0));
    let hit = raycast_sphere(&incoming, &sphere, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(1));
    assert_eq!(hit.point, v3(1, 0, 0));
    assert_eq!(hit.normal, v3(-1, 0, 0));

    let inside = Ray3::new(v3(3, 0, 0), v3(1, 0, 0));
    let inside_hit = raycast_sphere(&inside, &sphere, Fx32::from_int(10)).unwrap();
    assert_eq!(inside_hit.toi, Fx32::ZERO);
    assert_eq!(inside_hit.point, v3(3, 0, 0));
    assert_eq!(inside_hit.normal, Vec3::ZERO);

    let too_short = Ray3::new(v3(0, 0, 0), v3(1, 0, 0));
    assert!(raycast_sphere(&too_short, &sphere, Fx32::from_ratio(1, 2)).is_none());
}

#[test]
fn raycast_capsule3_is_boundary_inclusive_and_reports_zero_normal_from_inside() {
    let capsule =
        Capsule3::try_new(Segment3::new(v3(3, -1, 0), v3(3, 1, 0)), Fx32::from_int(1)).unwrap();
    let incoming = Ray3::new(v3(0, 0, 0), v3(1, 0, 0));
    let hit = raycast_capsule3(&incoming, &capsule, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(2));
    assert_eq!(hit.point, v3(2, 0, 0));
    assert_eq!(hit.normal, v3(-1, 0, 0));

    let inside = Ray3::new(v3(3, 0, 0), v3(1, 0, 0));
    let inside_hit = raycast_capsule3(&inside, &capsule, Fx32::from_int(10)).unwrap();
    assert_eq!(inside_hit.toi, Fx32::ZERO);
    assert_eq!(inside_hit.point, v3(3, 0, 0));
    assert_eq!(inside_hit.normal, Vec3::ZERO);

    let too_short = Ray3::new(v3(0, 0, 0), v3(1, 0, 0));
    assert!(raycast_capsule3(&too_short, &capsule, Fx32::from_int(1)).is_none());
}

#[test]
fn raycast_triangle3_is_boundary_inclusive_and_reports_zero_normal_on_surface() {
    let triangle = Triangle3::new(v3(3, -1, -1), v3(3, 1, -1), v3(3, 0, 1));
    let incoming = Ray3::new(v3(0, 0, 0), v3(1, 0, 0));
    let hit = raycast_triangle3(&incoming, &triangle, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(3));
    assert_eq!(hit.point, v3(3, 0, 0));
    assert_eq!(hit.normal, v3(-1, 0, 0));

    let on_surface = Ray3::new(v3(3, 0, 0), v3(1, 0, 0));
    let surface_hit = raycast_triangle3(&on_surface, &triangle, Fx32::from_int(10)).unwrap();
    assert_eq!(surface_hit.toi, Fx32::ZERO);
    assert_eq!(surface_hit.point, v3(3, 0, 0));
    assert_eq!(surface_hit.normal, Vec3::ZERO);

    assert!(raycast_triangle3(&incoming, &triangle, Fx32::from_int(2)).is_none());
}

#[test]
fn sweep_sphere_aabb3_is_boundary_inclusive_and_reports_zero_normal_from_inside() {
    let aabb = Aabb3::try_new(v3(4, -2, -2), v3(8, 2, 2)).unwrap();
    let sphere = Sphere::try_new(v3(0, 0, 0), Fx32::from_int(2)).unwrap();
    let hit = sweep_sphere_aabb3(&sphere, v3(1, 0, 0), &aabb, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(2));
    assert_eq!(hit.point, v3(2, 0, 0));
    assert_eq!(hit.normal, v3(-1, 0, 0));

    let overlapping = Sphere::try_new(v3(5, 0, 0), Fx32::from_int(2)).unwrap();
    let overlap_hit =
        sweep_sphere_aabb3(&overlapping, v3(1, 0, 0), &aabb, Fx32::from_int(10)).unwrap();
    assert_eq!(overlap_hit.toi, Fx32::ZERO);
    assert_eq!(overlap_hit.point, v3(5, 0, 0));
    assert_eq!(overlap_hit.normal, Vec3::ZERO);

    assert!(sweep_sphere_aabb3(&sphere, v3(1, 0, 0), &aabb, Fx32::from_int(1)).is_none());
}

#[test]
fn sweep_sphere_capsule3_is_boundary_inclusive_and_reports_zero_normal_from_inside() {
    let capsule =
        Capsule3::try_new(Segment3::new(v3(6, -1, 0), v3(6, 1, 0)), Fx32::from_int(1)).unwrap();
    let sphere = Sphere::try_new(v3(0, 0, 0), Fx32::from_int(2)).unwrap();
    let hit = sweep_sphere_capsule3(&sphere, v3(1, 0, 0), &capsule, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(3));
    assert_eq!(hit.point, v3(3, 0, 0));
    assert_eq!(hit.normal, v3(-1, 0, 0));

    let overlapping = Sphere::try_new(v3(6, 0, 0), Fx32::from_int(2)).unwrap();
    let overlap_hit =
        sweep_sphere_capsule3(&overlapping, v3(1, 0, 0), &capsule, Fx32::from_int(10)).unwrap();
    assert_eq!(overlap_hit.toi, Fx32::ZERO);
    assert_eq!(overlap_hit.point, v3(6, 0, 0));
    assert_eq!(overlap_hit.normal, Vec3::ZERO);

    assert!(sweep_sphere_capsule3(&sphere, v3(1, 0, 0), &capsule, Fx32::from_int(2)).is_none());
}

#[test]
fn sweep_sphere_segment3_is_boundary_inclusive_and_reports_zero_normal_from_inside() {
    let segment = Segment3::new(v3(6, -1, 0), v3(6, 1, 0));
    let sphere = Sphere::try_new(v3(0, 0, 0), Fx32::from_int(2)).unwrap();
    let hit = sweep_sphere_segment3(&sphere, v3(1, 0, 0), &segment, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(4));
    assert_eq!(hit.point, v3(4, 0, 0));
    assert_eq!(hit.normal, v3(-1, 0, 0));

    let overlapping = Sphere::try_new(v3(6, 0, 0), Fx32::from_int(2)).unwrap();
    let overlap_hit =
        sweep_sphere_segment3(&overlapping, v3(1, 0, 0), &segment, Fx32::from_int(10)).unwrap();
    assert_eq!(overlap_hit.toi, Fx32::ZERO);
    assert_eq!(overlap_hit.point, v3(6, 0, 0));
    assert_eq!(overlap_hit.normal, Vec3::ZERO);

    assert!(sweep_sphere_segment3(&sphere, v3(1, 0, 0), &segment, Fx32::from_int(3)).is_none());
}

#[test]
fn sweep_sphere_sphere_is_boundary_inclusive_and_reports_zero_normal_from_inside() {
    let target = Sphere::try_new(v3(8, 0, 0), Fx32::from_int(1)).unwrap();
    let sphere = Sphere::try_new(v3(0, 0, 0), Fx32::from_int(2)).unwrap();
    let hit = sweep_sphere_sphere(&sphere, v3(1, 0, 0), &target, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(5));
    assert_eq!(hit.point, v3(5, 0, 0));
    assert_eq!(hit.normal, v3(-1, 0, 0));

    let overlapping = Sphere::try_new(v3(8, 0, 0), Fx32::from_int(2)).unwrap();
    let overlap_hit =
        sweep_sphere_sphere(&overlapping, v3(1, 0, 0), &target, Fx32::from_int(10)).unwrap();
    assert_eq!(overlap_hit.toi, Fx32::ZERO);
    assert_eq!(overlap_hit.point, v3(8, 0, 0));
    assert_eq!(overlap_hit.normal, Vec3::ZERO);

    assert!(sweep_sphere_sphere(&sphere, v3(1, 0, 0), &target, Fx32::from_int(4)).is_none());
}

#[test]
fn sweep_sphere_triangle3_is_boundary_inclusive_and_reports_zero_normal_from_inside() {
    let triangle = Triangle3::new(v3(6, -2, -2), v3(6, 2, -2), v3(6, 0, 2));
    let sphere = Sphere::try_new(v3(0, 0, 0), Fx32::from_int(2)).unwrap();
    let hit = sweep_sphere_triangle3(&sphere, v3(1, 0, 0), &triangle, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(4));
    assert_eq!(hit.point, v3(4, 0, 0));
    assert_eq!(hit.normal, v3(-1, 0, 0));

    let overlapping = Sphere::try_new(v3(6, 0, 0), Fx32::from_int(2)).unwrap();
    let overlap_hit =
        sweep_sphere_triangle3(&overlapping, v3(1, 0, 0), &triangle, Fx32::from_int(10)).unwrap();
    assert_eq!(overlap_hit.toi, Fx32::ZERO);
    assert_eq!(overlap_hit.point, v3(6, 0, 0));
    assert_eq!(overlap_hit.normal, Vec3::ZERO);

    assert!(sweep_sphere_triangle3(&sphere, v3(1, 0, 0), &triangle, Fx32::from_int(3)).is_none());
}

#[test]
fn sweep_segment3_triangle3_is_boundary_inclusive_and_uses_midpoint_reference() {
    let segment = Segment3::new(v3(-1, 0, 0), v3(1, 0, 0));
    let triangle = Triangle3::new(v3(8, -2, -2), v3(8, 2, -2), v3(8, 0, 2));
    let hit =
        sweep_segment3_triangle3(&segment, v3(1, 0, 0), &triangle, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(7));
    assert_eq!(hit.point, v3(7, 0, 0));
    assert_eq!(hit.normal, v3(-1, 0, 0));

    let overlap_hit = sweep_segment3_triangle3(
        &Segment3::new(v3(8, 0, 0), v3(10, 0, 0)),
        v3(1, 0, 0),
        &triangle,
        Fx32::from_int(10),
    )
    .unwrap();
    assert_eq!(overlap_hit.toi, Fx32::ZERO);
    assert_eq!(overlap_hit.point, v3(9, 0, 0));
    assert_eq!(overlap_hit.normal, Vec3::ZERO);

    assert!(sweep_segment3_triangle3(&segment, v3(1, 0, 0), &triangle, Fx32::from_int(6)).is_none());
}

#[test]
fn sweep_segment3_triangle3_hits_face_interior_when_endpoints_miss_edges() {
    let segment = Segment3::new(v3(0, -3, 0), v3(0, 3, 0));
    let triangle = Triangle3::new(v3(6, -1, -2), v3(6, 1, -2), v3(6, 0, 2));
    let hit =
        sweep_segment3_triangle3(&segment, v3(1, 0, 0), &triangle, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(6));
    assert_eq!(hit.point, v3(6, 0, 0));
    assert_eq!(hit.normal, v3(-1, 0, 0));
}

#[test]
fn sweep_triangle3_triangle3_is_boundary_inclusive_and_uses_reference_point() {
    let triangle = Triangle3::new(v3(0, 0, 0), v3(0, 3, 0), v3(0, 0, 3));
    let target = Triangle3::new(v3(6, 0, 0), v3(6, 3, 0), v3(6, 0, 3));
    let reference = (triangle.a + triangle.b + triangle.c) * Fx32::from_ratio(1, 3);
    let hit =
        sweep_triangle3_triangle3(&triangle, v3(1, 0, 0), &target, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(6));
    assert_eq!(hit.point, reference + v3(6, 0, 0));
    assert_eq!(hit.normal, v3(-1, 0, 0));

    let overlapping = Triangle3::new(v3(6, 0, 0), v3(6, 3, 0), v3(6, 0, 3));
    let overlap_reference =
        (overlapping.a + overlapping.b + overlapping.c) * Fx32::from_ratio(1, 3);
    let overlap_hit =
        sweep_triangle3_triangle3(&overlapping, v3(1, 0, 0), &target, Fx32::from_int(10)).unwrap();
    assert_eq!(overlap_hit.toi, Fx32::ZERO);
    assert_eq!(overlap_hit.point, overlap_reference);
    assert_eq!(overlap_hit.normal, Vec3::ZERO);

    assert!(sweep_triangle3_triangle3(&triangle, v3(1, 0, 0), &target, Fx32::from_int(5)).is_none());
}

#[test]
fn sweep_triangle3_triangle3_hits_face_interior_when_peer_edges_miss() {
    let triangle = Triangle3::new(v3(0, 0, 0), v3(0, 6, 0), v3(0, 0, 6));
    let point_triangle = Triangle3::new(v3(6, 2, 2), v3(6, 2, 2), v3(6, 2, 2));
    let reference = (triangle.a + triangle.b + triangle.c) * Fx32::from_ratio(1, 3);
    let hit =
        sweep_triangle3_triangle3(&triangle, v3(1, 0, 0), &point_triangle, Fx32::from_int(10))
            .unwrap();
    assert_eq!(hit.toi, Fx32::from_int(6));
    assert_eq!(hit.point, reference + v3(6, 0, 0));
    assert_eq!(hit.normal, v3(1, 0, 0));
}

#[test]
fn sweep_capsule3_sphere_is_boundary_inclusive_and_uses_midpoint_reference() {
    let capsule =
        Capsule3::try_new(Segment3::new(v3(-1, 0, 0), v3(1, 0, 0)), Fx32::from_int(1)).unwrap();
    let target = Sphere::try_new(v3(8, 0, 0), Fx32::from_int(1)).unwrap();
    let hit = sweep_capsule3_sphere(&capsule, v3(1, 0, 0), &target, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(5));
    assert_eq!(hit.point, v3(5, 0, 0));
    assert_eq!(hit.normal, v3(-1, 0, 0));

    let overlapping =
        Capsule3::try_new(Segment3::new(v3(7, 0, 0), v3(9, 0, 0)), Fx32::from_int(1)).unwrap();
    let overlap_hit =
        sweep_capsule3_sphere(&overlapping, v3(1, 0, 0), &target, Fx32::from_int(10)).unwrap();
    assert_eq!(overlap_hit.toi, Fx32::ZERO);
    assert_eq!(overlap_hit.point, v3(8, 0, 0));
    assert_eq!(overlap_hit.normal, Vec3::ZERO);

    assert!(sweep_capsule3_sphere(&capsule, v3(1, 0, 0), &target, Fx32::from_int(4)).is_none());
}

#[test]
fn sweep_capsule3_triangle3_is_boundary_inclusive_and_uses_midpoint_reference() {
    let capsule =
        Capsule3::try_new(Segment3::new(v3(-1, 0, 0), v3(1, 0, 0)), Fx32::from_int(1)).unwrap();
    let triangle = Triangle3::new(v3(8, -2, -2), v3(8, 2, -2), v3(8, 0, 2));
    let hit =
        sweep_capsule3_triangle3(&capsule, v3(1, 0, 0), &triangle, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(6));
    assert_eq!(hit.point, v3(6, 0, 0));
    assert_eq!(hit.normal, v3(-1, 0, 0));

    let overlapping =
        Capsule3::try_new(Segment3::new(v3(7, 0, 0), v3(9, 0, 0)), Fx32::from_int(1)).unwrap();
    let overlap_hit =
        sweep_capsule3_triangle3(&overlapping, v3(1, 0, 0), &triangle, Fx32::from_int(10)).unwrap();
    assert_eq!(overlap_hit.toi, Fx32::ZERO);
    assert_eq!(overlap_hit.point, v3(8, 0, 0));
    assert_eq!(overlap_hit.normal, Vec3::ZERO);

    assert!(sweep_capsule3_triangle3(&capsule, v3(1, 0, 0), &triangle, Fx32::from_int(5)).is_none());
}

#[test]
fn sweep_capsule3_triangle3_hits_face_interior_when_endpoints_miss_edges() {
    let capsule =
        Capsule3::try_new(Segment3::new(v3(0, -3, 0), v3(0, 3, 0)), Fx32::from_int(1)).unwrap();
    let triangle = Triangle3::new(v3(6, -1, -2), v3(6, 1, -2), v3(6, 0, 2));
    let hit =
        sweep_capsule3_triangle3(&capsule, v3(1, 0, 0), &triangle, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(5));
    assert_eq!(hit.point, v3(5, 0, 0));
    assert_eq!(hit.normal, v3(-1, 0, 0));
}

#[test]
fn sweep_capsule3_segment3_is_boundary_inclusive_and_uses_midpoint_reference() {
    let capsule =
        Capsule3::try_new(Segment3::new(v3(-1, 0, 0), v3(1, 0, 0)), Fx32::from_int(1)).unwrap();
    let target = Segment3::new(v3(8, -1, 0), v3(8, 1, 0));
    let hit = sweep_capsule3_segment3(&capsule, v3(1, 0, 0), &target, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(6));
    assert_eq!(hit.point, v3(6, 0, 0));
    assert_eq!(hit.normal, v3(-1, 0, 0));

    let overlapping =
        Capsule3::try_new(Segment3::new(v3(8, 0, 0), v3(10, 0, 0)), Fx32::from_int(1)).unwrap();
    let overlap_hit =
        sweep_capsule3_segment3(&overlapping, v3(1, 0, 0), &target, Fx32::from_int(10)).unwrap();
    assert_eq!(overlap_hit.toi, Fx32::ZERO);
    assert_eq!(overlap_hit.point, v3(9, 0, 0));
    assert_eq!(overlap_hit.normal, Vec3::ZERO);

    assert!(sweep_capsule3_segment3(&capsule, v3(1, 0, 0), &target, Fx32::from_int(5)).is_none());
}

#[test]
fn sweep_capsule3_segment3_hits_face_interior_for_parallel_segments() {
    let capsule =
        Capsule3::try_new(Segment3::new(v3(0, -1, -5), v3(0, 1, -5)), Fx32::from_int(1)).unwrap();
    let target = Segment3::new(v3(0, -1, 0), v3(0, 1, 0));
    let hit = sweep_capsule3_segment3(&capsule, v3(0, 0, 1), &target, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(4));
    assert_eq!(hit.point, v3(0, 0, -1));
    assert_eq!(hit.normal, v3(0, 0, -1));
}

#[test]
fn sweep_capsule3_capsule3_is_boundary_inclusive_and_uses_midpoint_reference() {
    let moving =
        Capsule3::try_new(Segment3::new(v3(-1, 0, 0), v3(1, 0, 0)), Fx32::from_int(1)).unwrap();
    let target =
        Capsule3::try_new(Segment3::new(v3(10, -1, 0), v3(10, 1, 0)), Fx32::from_int(1)).unwrap();
    let hit = sweep_capsule3_capsule3(&moving, v3(1, 0, 0), &target, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(7));
    assert_eq!(hit.point, v3(7, 0, 0));
    assert_eq!(hit.normal, v3(-1, 0, 0));

    let overlapping =
        Capsule3::try_new(Segment3::new(v3(9, 0, 0), v3(11, 0, 0)), Fx32::from_int(1)).unwrap();
    let overlap_hit =
        sweep_capsule3_capsule3(&overlapping, v3(1, 0, 0), &target, Fx32::from_int(10)).unwrap();
    assert_eq!(overlap_hit.toi, Fx32::ZERO);
    assert_eq!(overlap_hit.point, v3(10, 0, 0));
    assert_eq!(overlap_hit.normal, Vec3::ZERO);

    assert!(sweep_capsule3_capsule3(&moving, v3(1, 0, 0), &target, Fx32::from_int(6)).is_none());
}

#[test]
fn sweep_capsule3_aabb3_is_boundary_inclusive_and_uses_midpoint_reference() {
    let capsule =
        Capsule3::try_new(Segment3::new(v3(-1, 0, 0), v3(1, 0, 0)), Fx32::from_int(1)).unwrap();
    let target = Aabb3::try_new(v3(8, -2, -2), v3(12, 2, 2)).unwrap();
    let hit = sweep_capsule3_aabb3(&capsule, v3(1, 0, 0), &target, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(6));
    assert_eq!(hit.point, v3(6, 0, 0));
    assert_eq!(hit.normal, v3(-1, 0, 0));

    let overlapping =
        Capsule3::try_new(Segment3::new(v3(8, 0, 0), v3(10, 0, 0)), Fx32::from_int(1)).unwrap();
    let overlap_hit =
        sweep_capsule3_aabb3(&overlapping, v3(1, 0, 0), &target, Fx32::from_int(10)).unwrap();
    assert_eq!(overlap_hit.toi, Fx32::ZERO);
    assert_eq!(overlap_hit.point, v3(9, 0, 0));
    assert_eq!(overlap_hit.normal, Vec3::ZERO);

    assert!(sweep_capsule3_aabb3(&capsule, v3(1, 0, 0), &target, Fx32::from_int(5)).is_none());
}

#[test]
fn sweep_segment3_aabb3_is_boundary_inclusive_and_uses_midpoint_reference() {
    let segment = Segment3::new(v3(-1, 0, 0), v3(1, 0, 0));
    let target = Aabb3::try_new(v3(8, -2, -2), v3(12, 2, 2)).unwrap();
    let hit = sweep_segment3_aabb3(&segment, v3(1, 0, 0), &target, Fx32::from_int(10)).unwrap();
    assert_eq!(hit.toi, Fx32::from_int(7));
    assert_eq!(hit.point, v3(7, 0, 0));
    assert_eq!(hit.normal, v3(-1, 0, 0));

    let overlap_hit = sweep_segment3_aabb3(
        &Segment3::new(v3(8, 0, 0), v3(10, 0, 0)),
        v3(1, 0, 0),
        &target,
        Fx32::from_int(10),
    )
    .unwrap();
    assert_eq!(overlap_hit.toi, Fx32::ZERO);
    assert_eq!(overlap_hit.point, v3(9, 0, 0));
    assert_eq!(overlap_hit.normal, Vec3::ZERO);

    assert!(sweep_segment3_aabb3(&segment, v3(1, 0, 0), &target, Fx32::from_int(6)).is_none());
}

#[test]
fn segment3_overlaps_aabb3_counts_boundary_touch_and_degenerate_segments() {
    let aabb = Aabb3::try_new(v3(0, 0, 0), v3(4, 4, 4)).unwrap();
    let crossing = Segment3::new(v3(-1, 2, 2), v3(5, 2, 2));
    let touching = Segment3::new(v3(4, 2, 2), v3(6, 2, 2));
    let separated = Segment3::new(v3(5, 2, 2), v3(7, 2, 2));
    let degenerate = Segment3::new(v3(4, 2, 2), v3(4, 2, 2));

    assert!(segment3_overlaps_aabb3(&crossing, &aabb));
    assert!(segment3_overlaps_aabb3(&touching, &aabb));
    assert!(!segment3_overlaps_aabb3(&separated, &aabb));
    assert!(segment3_overlaps_aabb3(&degenerate, &aabb));
}

#[test]
fn capsule3_contains_point_counts_endcaps_and_degenerate_segments() {
    let capsule =
        Capsule3::try_new(Segment3::new(v3(0, 0, 0), v3(4, 0, 0)), Fx32::from_int(1)).unwrap();
    assert!(capsule3_contains_point(&capsule, v3(2, 1, 0)));
    assert!(capsule3_contains_point(&capsule, v3(-1, 0, 0)));
    assert!(!capsule3_contains_point(&capsule, v3(2, 2, 0)));

    let degenerate =
        Capsule3::try_new(Segment3::new(v3(1, 1, 1), v3(1, 1, 1)), Fx32::from_int(2)).unwrap();
    assert!(capsule3_contains_point(&degenerate, v3(3, 1, 1)));
    assert!(!capsule3_contains_point(&degenerate, v3(4, 1, 1)));
}

#[test]
fn sphere_overlaps_capsule3_counts_boundary_touch() {
    let capsule =
        Capsule3::try_new(Segment3::new(v3(0, 0, 0), v3(4, 0, 0)), Fx32::from_int(1)).unwrap();
    let touching = Sphere::try_new(v3(2, 3, 0), Fx32::from_int(2)).unwrap();
    let separated = Sphere::try_new(v3(2, 4, 0), Fx32::from_int(2)).unwrap();

    assert!(sphere_overlaps_capsule3(&touching, &capsule));
    assert!(!sphere_overlaps_capsule3(&separated, &capsule));
}

#[test]
fn sphere_overlaps_segment3_counts_boundary_touch_and_degenerate_segments() {
    let segment = Segment3::new(v3(0, 0, 0), v3(4, 0, 0));
    let touching = Sphere::try_new(v3(2, 3, 0), Fx32::from_int(3)).unwrap();
    let separated = Sphere::try_new(v3(2, 4, 0), Fx32::from_int(3)).unwrap();
    let degenerate = Segment3::new(v3(8, 0, 0), v3(8, 0, 0));
    let degenerate_touching = Sphere::try_new(v3(10, 0, 0), Fx32::from_int(2)).unwrap();

    assert!(sphere_overlaps_segment3(&touching, &segment));
    assert!(!sphere_overlaps_segment3(&separated, &segment));
    assert!(sphere_overlaps_segment3(&degenerate_touching, &degenerate));
}

#[test]
fn sphere_overlaps_sphere_is_symmetric_and_boundary_inclusive() {
    let a = Sphere::try_new(v3(0, 0, 0), Fx32::from_int(2)).unwrap();
    let touching = Sphere::try_new(v3(5, 0, 0), Fx32::from_int(3)).unwrap();
    let separated = Sphere::try_new(v3(6, 0, 0), Fx32::from_int(3)).unwrap();

    assert!(sphere_overlaps_sphere(&a, &touching));
    assert!(sphere_overlaps_sphere(&touching, &a));
    assert!(!sphere_overlaps_sphere(&a, &separated));
}

#[test]
fn capsule3_overlaps_capsule3_is_symmetric_and_handles_skew_segments() {
    let axis_x =
        Capsule3::try_new(Segment3::new(v3(0, 0, 0), v3(4, 0, 0)), Fx32::from_int(1)).unwrap();
    let skew_touching =
        Capsule3::try_new(Segment3::new(v3(2, 3, -1), v3(2, 3, 1)), Fx32::from_int(2)).unwrap();
    let skew_separated =
        Capsule3::try_new(Segment3::new(v3(2, 4, -1), v3(2, 4, 1)), Fx32::from_int(2)).unwrap();
    let degenerate =
        Capsule3::try_new(Segment3::new(v3(9, 0, 0), v3(9, 0, 0)), Fx32::from_int(3)).unwrap();

    assert!(capsule3_overlaps_capsule3(&axis_x, &skew_touching));
    assert!(capsule3_overlaps_capsule3(&skew_touching, &axis_x));
    assert!(!capsule3_overlaps_capsule3(&axis_x, &skew_separated));
    assert!(!capsule3_overlaps_capsule3(&axis_x, &degenerate));
}

#[test]
fn capsule3_overlaps_aabb3_counts_boundary_touch_and_degenerate_capsules() {
    let aabb = Aabb3::try_new(v3(0, 0, 0), v3(4, 4, 4)).unwrap();
    let touching =
        Capsule3::try_new(Segment3::new(v3(6, 2, 2), v3(8, 2, 2)), Fx32::from_int(2)).unwrap();
    let contained = Capsule3::try_new(Segment3::new(v3(1, 1, 1), v3(3, 1, 1)), Fx32::ZERO).unwrap();
    let degenerate =
        Capsule3::try_new(Segment3::new(v3(6, 2, 2), v3(6, 2, 2)), Fx32::from_int(2)).unwrap();
    let separated =
        Capsule3::try_new(Segment3::new(v3(7, 2, 2), v3(9, 2, 2)), Fx32::from_int(2)).unwrap();

    assert!(capsule3_overlaps_aabb3(&touching, &aabb));
    assert!(capsule3_overlaps_aabb3(&contained, &aabb));
    assert!(capsule3_overlaps_aabb3(&degenerate, &aabb));
    assert!(!capsule3_overlaps_aabb3(&separated, &aabb));
}
