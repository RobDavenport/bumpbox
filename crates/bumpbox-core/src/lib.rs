#![cfg_attr(not(any(test, feature = "std")), no_std)]

pub mod queries;
pub mod queries3d;
pub mod scalar;
pub mod shapes;
pub mod shapes3d;
pub mod vec2;
pub mod vec3;

pub use queries::{
    aabb_overlaps, capsule_contains_point, capsule_overlaps_aabb, capsule_overlaps_capsule,
    capsule_overlaps_convex_polygon, capsule_overlaps_oriented_box, capsule_overlaps_segment,
    circle_overlaps_aabb, circle_overlaps_capsule, circle_overlaps_convex_polygon,
    circle_overlaps_oriented_box, circle_overlaps_segment, closest_point_on_aabb,
    closest_point_on_capsule, closest_point_on_circle, closest_point_on_convex_polygon,
    closest_point_on_oriented_box, closest_point_on_segment, convex_polygon_contains_point,
    convex_polygon_overlaps_aabb, convex_polygon_overlaps_convex_polygon,
    distance_squared_point_aabb, distance_squared_point_capsule, distance_squared_point_circle,
    distance_squared_point_convex_polygon, distance_squared_point_oriented_box,
    distance_squared_point_segment, oriented_box_contains_point, oriented_box_overlaps_aabb,
    oriented_box_overlaps_oriented_box, raycast_aabb, raycast_capsule, raycast_circle,
    raycast_convex_polygon, raycast_oriented_box, segment_intersects_segment, sweep_capsule_aabb,
    sweep_capsule_capsule, sweep_capsule_circle, sweep_capsule_convex_polygon,
    sweep_capsule_oriented_box, sweep_capsule_segment, sweep_circle_aabb, sweep_circle_capsule,
    sweep_circle_circle, sweep_circle_convex_polygon, sweep_circle_oriented_box,
    sweep_circle_segment, RayHit,
};
pub use queries3d::{
    aabb3_overlaps, capsule3_contains_point, capsule3_overlaps_aabb3, capsule3_overlaps_capsule3,
    capsule3_overlaps_triangle3, closest_point_on_aabb3, closest_point_on_capsule3,
    closest_point_on_segment3, closest_point_on_sphere, closest_point_on_triangle3,
    closest_points_capsule3_triangle3, closest_points_segment3_triangle3,
    closest_points_sphere_triangle3, closest_points_triangle3_aabb3,
    closest_points_triangle3_triangle3, distance_squared_capsule3_triangle3,
    distance_squared_point_aabb3, distance_squared_point_capsule3, distance_squared_point_segment3,
    distance_squared_point_sphere, distance_squared_point_triangle3,
    distance_squared_segment3_triangle3, distance_squared_sphere_triangle3,
    distance_squared_triangle3_aabb3, distance_squared_triangle3_triangle3, raycast_aabb3,
    raycast_capsule3, raycast_sphere, raycast_triangle3, segment3_overlaps_aabb3,
    segment3_overlaps_triangle3, sphere_overlaps_aabb3, sphere_overlaps_capsule3,
    sphere_overlaps_segment3, sphere_overlaps_sphere, sphere_overlaps_triangle3,
    sweep_capsule3_aabb3, sweep_capsule3_capsule3, sweep_capsule3_segment3, sweep_capsule3_sphere,
    sweep_capsule3_triangle3, sweep_segment3_aabb3, sweep_segment3_triangle3, sweep_sphere_aabb3,
    sweep_sphere_capsule3, sweep_sphere_segment3, sweep_sphere_sphere, sweep_sphere_triangle3,
    sweep_triangle3_triangle3, triangle3_overlaps_aabb3, triangle3_overlaps_triangle3,
    ClosestPair3, ClosestPoints3, RayHit3,
};
pub use scalar::Fx32;
pub use shapes::{Aabb, Capsule, Circle, ConvexPolygon, OrientedBox, Ray, Segment};
pub use shapes3d::{Aabb3, Capsule3, Ray3, Segment3, Sphere, Triangle3};
pub use vec2::Vec2;
pub use vec3::Vec3;
