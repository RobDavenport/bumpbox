#![cfg_attr(not(any(test, feature = "std")), no_std)]

pub mod queries;
pub mod queries3d;
pub mod scalar;
pub mod shapes;
pub mod shapes3d;
pub mod vec2;
pub mod vec3;

pub use queries::{
    aabb_overlaps, capsule_contains_point, circle_overlaps_aabb, closest_point_on_aabb,
    convex_polygon_contains_point, distance_squared_point_aabb, oriented_box_contains_point,
    raycast_aabb, RayHit,
};
pub use queries3d::{
    aabb3_overlaps, closest_point_on_aabb3, distance_squared_point_aabb3, raycast_aabb3,
    sphere_overlaps_aabb3, RayHit3,
};
pub use scalar::Fx32;
pub use shapes::{Aabb, Capsule, Circle, ConvexPolygon, OrientedBox, Ray, Segment};
pub use shapes3d::{Aabb3, Ray3, Sphere};
pub use vec2::Vec2;
pub use vec3::Vec3;
