use bumpbox_core::{Aabb, Fx32, Vec2};
use bumpbox_grid::UniformGrid;

fn v(x: i32, y: i32) -> Vec2 {
    Vec2::new(Fx32::from_int(x), Fx32::from_int(y))
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
