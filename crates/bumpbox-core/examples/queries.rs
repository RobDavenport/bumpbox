use bumpbox_core::{circle_overlaps_aabb, raycast_aabb, Aabb, Circle, Fx32, Ray, Vec2};

fn v(x: i32, y: i32) -> Vec2 {
    Vec2::new(Fx32::from_int(x), Fx32::from_int(y))
}

fn main() {
    let aabb = Aabb::try_new(v(0, 0), v(4, 4)).expect("valid aabb");
    let circle = Circle::try_new(v(2, 2), Fx32::from_int(1)).expect("valid circle");
    let ray = Ray::new(v(-2, 2), v(1, 0));

    println!("circle overlaps box: {}", circle_overlaps_aabb(&circle, &aabb));

    if let Some(hit) = raycast_aabb(&ray, &aabb, Fx32::from_int(16)) {
        println!(
            "ray hit toi_raw={} point=({}, {})",
            hit.toi.raw(),
            hit.point.x.raw(),
            hit.point.y.raw()
        );
    }
}
