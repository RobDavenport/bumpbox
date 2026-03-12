use bumpbox_core::{Aabb, Aabb3, Fx32, Vec2, Vec3};
use bumpbox_grid::{UniformGrid, UniformGrid3};

fn v(x: i32, y: i32) -> Vec2 {
    Vec2::new(Fx32::from_int(x), Fx32::from_int(y))
}

fn v3(x: i32, y: i32, z: i32) -> Vec3 {
    Vec3::new(Fx32::from_int(x), Fx32::from_int(y), Fx32::from_int(z))
}

#[test]
fn grid_query_is_sorted_and_deduplicated() {
    let mut grid: UniformGrid<16, 4> =
        UniformGrid::new(4, 4, Vec2::ZERO, Fx32::from_int(1)).unwrap();

    let wide = Aabb::try_new(v(0, 0), v(2, 2)).unwrap();
    let right = Aabb::try_new(v(2, 0), v(3, 1)).unwrap();

    grid.insert(7, &wide).unwrap();
    grid.insert(3, &right).unwrap();

    let query = Aabb::try_new(v(0, 0), v(3, 2)).unwrap();
    let mut out = [0u32; 8];
    let len = grid.query_aabb(&query, &mut out).unwrap();

    assert_eq!(&out[..len], &[3, 7]);
}

#[test]
fn grid3_query_is_sorted_and_deduplicated() {
    let mut grid: UniformGrid3<27, 4> =
        UniformGrid3::new(3, 3, 3, Vec3::ZERO, Fx32::from_int(1)).unwrap();

    let slab = Aabb3::try_new(v3(0, 0, 0), v3(2, 2, 2)).unwrap();
    let corner = Aabb3::try_new(v3(2, 2, 2), v3(3, 3, 3)).unwrap();

    grid.insert(9, &slab).unwrap();
    grid.insert(4, &corner).unwrap();

    let query = Aabb3::try_new(v3(0, 0, 0), v3(3, 3, 3)).unwrap();
    let mut out = [0u32; 8];
    let len = grid.query_aabb(&query, &mut out).unwrap();

    assert_eq!(&out[..len], &[4, 9]);
}

#[test]
fn grid3_insert_rejects_out_of_bounds_items() {
    let mut grid: UniformGrid3<8, 2> =
        UniformGrid3::new(2, 2, 2, Vec3::ZERO, Fx32::from_int(1)).unwrap();
    let outside = Aabb3::try_new(v3(-1, 0, 0), v3(0, 1, 1)).unwrap();

    assert!(grid.insert(1, &outside).is_err());
}
