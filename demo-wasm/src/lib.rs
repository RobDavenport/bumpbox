use bumpbox_core::{
    capsule3_contains_point, capsule3_overlaps_triangle3, capsule_contains_point,
    circle_overlaps_aabb, circle_overlaps_capsule, circle_overlaps_convex_polygon,
    circle_overlaps_oriented_box, closest_point_on_triangle3, closest_points_capsule3_triangle3,
    closest_points_segment3_triangle3, closest_points_sphere_triangle3,
    closest_points_triangle3_aabb3, closest_points_triangle3_triangle3,
    convex_polygon_contains_point, distance_squared_capsule3_triangle3,
    distance_squared_point_triangle3, distance_squared_sphere_triangle3,
    distance_squared_triangle3_aabb3, oriented_box_contains_point, raycast_aabb, raycast_aabb3,
    raycast_capsule, raycast_capsule3, raycast_circle, raycast_convex_polygon,
    raycast_oriented_box, raycast_sphere, raycast_triangle3, segment3_overlaps_triangle3,
    sphere_overlaps_aabb3, sphere_overlaps_capsule3, sphere_overlaps_triangle3,
    sweep_capsule3_triangle3, sweep_segment3_triangle3, sweep_sphere_triangle3,
    sweep_triangle3_triangle3, triangle3_overlaps_aabb3, triangle3_overlaps_triangle3, Aabb, Aabb3,
    Capsule, Capsule3, Circle, ConvexPolygon, Fx32, OrientedBox, Ray, Ray3, Segment, Segment3,
    Sphere, Triangle3, Vec2, Vec3,
};
use bumpbox_grid::{UniformGrid, UniformGrid3};
use serde::Serialize;
use wasm_bindgen::prelude::*;

const SCALE_F32: f32 = Fx32::SCALE as f32;

#[derive(Clone, Debug, PartialEq, Serialize)]
struct Aabb2View {
    min: [f32; 2],
    max: [f32; 2],
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct Aabb3View {
    min: [f32; 3],
    max: [f32; 3],
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct Segment2View {
    start: [f32; 2],
    end: [f32; 2],
    radius: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct Segment3View {
    start: [f32; 3],
    end: [f32; 3],
    radius: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct Ray2View {
    origin: [f32; 2],
    dir: [f32; 2],
    hit_point: Option<[f32; 2]>,
    hit_normal: Option<[f32; 2]>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct Ray3View {
    origin: [f32; 3],
    dir: [f32; 3],
    hit_point: Option<[f32; 3]>,
    hit_normal: Option<[f32; 3]>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct Scene2DFlags {
    circle_hits_aabb: bool,
    probe_in_capsule: bool,
    probe_in_oriented_box: bool,
    probe_in_polygon: bool,
    ray_hits_aabb: bool,
    ray_hits_circle: bool,
    ray_hits_capsule: bool,
    ray_hits_polygon: bool,
    ray_hits_oriented_box: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct Scene3DFlags {
    sphere_hits_aabb: bool,
    sphere_hits_capsule: bool,
    sphere_hits_triangle: bool,
    capsule_hits_triangle: bool,
    segment_hits_triangle: bool,
    triangle_hits_aabb: bool,
    triangle_hits_triangle: bool,
    probe_in_capsule: bool,
    probe_on_triangle: bool,
    ray_hits_aabb: bool,
    ray_hits_sphere: bool,
    ray_hits_capsule: bool,
    ray_hits_triangle: bool,
    sweep_hits_triangle: bool,
    capsule_sweep_hits_triangle: bool,
    segment_sweep_hits_triangle: bool,
    triangle_sweep_hits_triangle: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct Scene2DState {
    tick: u32,
    probe: [f32; 2],
    circle_center: [f32; 2],
    circle_radius: f32,
    aabb: Aabb2View,
    sensor: Aabb2View,
    capsule: Segment2View,
    oriented_box: [[f32; 2]; 4],
    polygon: Vec<[f32; 2]>,
    aabb_ray: Ray2View,
    circle_ray: Ray2View,
    capsule_ray: Ray2View,
    polygon_ray: Ray2View,
    oriented_box_ray: Ray2View,
    candidate_ids: Vec<u32>,
    flags: Scene2DFlags,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct Scene3DState {
    tick: u32,
    probe: [f32; 3],
    sphere_center: [f32; 3],
    sphere_radius: f32,
    aabb: Aabb3View,
    sensor: Aabb3View,
    capsule: Segment3View,
    triangle_segment: Segment3View,
    triangle_distance_segment: Segment3View,
    triangle_distance_sphere_center: [f32; 3],
    triangle_distance_sphere_radius: f32,
    triangle_distance_capsule: Segment3View,
    triangle_skew_distance_sphere_center: [f32; 3],
    triangle_skew_distance_sphere_radius: f32,
    triangle_skew_distance_capsule: Segment3View,
    triangle_vertex_distance_sphere_center: [f32; 3],
    triangle_vertex_distance_sphere_radius: f32,
    triangle: [[f32; 3]; 3],
    triangle_peer: [[f32; 3]; 3],
    triangle_distance_peer: [[f32; 3]; 3],
    triangle_distance_aabb: Aabb3View,
    triangle_vertex_distance_aabb: Aabb3View,
    triangle_sweep_triangle: [[f32; 3]; 3],
    triangle_sweep_peer: [[f32; 3]; 3],
    triangle_closest_point: [f32; 3],
    triangle_segment_closest_point: [f32; 3],
    triangle_distance_segment_point: [f32; 3],
    triangle_distance_triangle_point: [f32; 3],
    triangle_peer_distance_point_a: [f32; 3],
    triangle_peer_distance_point_b: [f32; 3],
    triangle_aabb_distance_point_triangle: [f32; 3],
    triangle_aabb_distance_point_aabb: [f32; 3],
    triangle_vertex_aabb_distance_point_triangle: [f32; 3],
    triangle_vertex_aabb_distance_point_aabb: [f32; 3],
    triangle_sphere_distance_point_sphere: [f32; 3],
    triangle_sphere_distance_point_triangle: [f32; 3],
    triangle_capsule_distance_point_capsule: [f32; 3],
    triangle_capsule_distance_point_triangle: [f32; 3],
    triangle_skew_sphere_distance_point_sphere: [f32; 3],
    triangle_skew_sphere_distance_point_triangle: [f32; 3],
    triangle_skew_capsule_distance_point_capsule: [f32; 3],
    triangle_skew_capsule_distance_point_triangle: [f32; 3],
    triangle_vertex_sphere_distance_point_sphere: [f32; 3],
    triangle_vertex_sphere_distance_point_triangle: [f32; 3],
    aabb_ray: Ray3View,
    sphere_ray: Ray3View,
    capsule_ray: Ray3View,
    triangle_ray: Ray3View,
    triangle_sweep: Ray3View,
    triangle_sweep_radius: f32,
    triangle_capsule_sweep_capsule: Segment3View,
    triangle_capsule_sweep: Ray3View,
    triangle_segment_sweep_segment: Segment3View,
    triangle_segment_sweep: Ray3View,
    triangle_triangle_sweep: Ray3View,
    candidate_ids: Vec<u32>,
    flags: Scene3DFlags,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct InteractiveScene2DFlags {
    circle_hits_aabb: bool,
    circle_hits_capsule: bool,
    circle_hits_polygon: bool,
    circle_hits_oriented_box: bool,
    probe_in_aabb: bool,
    probe_in_capsule: bool,
    probe_in_polygon: bool,
    probe_in_oriented_box: bool,
    ray_hits_aabb: bool,
    ray_hits_circle: bool,
    ray_hits_capsule: bool,
    ray_hits_polygon: bool,
    ray_hits_oriented_box: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct InteractiveHandles2D {
    probe: [f32; 2],
    circle: [f32; 2],
    aabb: [f32; 2],
    capsule: [f32; 2],
    oriented_box: [f32; 2],
    polygon: [f32; 2],
    sensor: [f32; 2],
    aabb_ray: [f32; 2],
    circle_ray: [f32; 2],
    capsule_ray: [f32; 2],
    polygon_ray: [f32; 2],
    oriented_box_ray: [f32; 2],
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct InteractiveScene2DState {
    probe: [f32; 2],
    circle_center: [f32; 2],
    circle_radius: f32,
    aabb: Aabb2View,
    sensor: Aabb2View,
    capsule: Segment2View,
    oriented_box: [[f32; 2]; 4],
    polygon: Vec<[f32; 2]>,
    aabb_ray: Ray2View,
    circle_ray: Ray2View,
    capsule_ray: Ray2View,
    polygon_ray: Ray2View,
    oriented_box_ray: Ray2View,
    candidate_ids: Vec<u32>,
    flags: InteractiveScene2DFlags,
    handles: InteractiveHandles2D,
}

#[derive(Clone, Debug, PartialEq)]
struct InteractiveScene2D {
    probe: Vec2,
    circle_center: Vec2,
    aabb_center: Vec2,
    capsule_center: Vec2,
    oriented_box_center: Vec2,
    polygon_offset: Vec2,
    sensor_center: Vec2,
    aabb_ray_origin: Vec2,
    circle_ray_origin: Vec2,
    capsule_ray_origin: Vec2,
    polygon_ray_origin: Vec2,
    oriented_box_ray_origin: Vec2,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct InteractiveScene3DFlags {
    sphere_hits_aabb: bool,
    sphere_hits_capsule: bool,
    sphere_hits_triangle: bool,
    probe_in_aabb: bool,
    probe_in_capsule: bool,
    probe_on_triangle: bool,
    ray_hits_aabb: bool,
    ray_hits_sphere: bool,
    ray_hits_capsule: bool,
    ray_hits_triangle: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct InteractiveHandles3D {
    probe: [f32; 3],
    sphere: [f32; 3],
    aabb: [f32; 3],
    capsule: [f32; 3],
    triangle: [f32; 3],
    sensor: [f32; 3],
    aabb_ray: [f32; 3],
    sphere_ray: [f32; 3],
    capsule_ray: [f32; 3],
    triangle_ray: [f32; 3],
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct InteractiveScene3DState {
    probe: [f32; 3],
    sphere_center: [f32; 3],
    sphere_radius: f32,
    aabb: Aabb3View,
    sensor: Aabb3View,
    capsule: Segment3View,
    triangle: [[f32; 3]; 3],
    triangle_closest_point: [f32; 3],
    aabb_ray: Ray3View,
    sphere_ray: Ray3View,
    capsule_ray: Ray3View,
    triangle_ray: Ray3View,
    candidate_ids: Vec<u32>,
    flags: InteractiveScene3DFlags,
    handles: InteractiveHandles3D,
}

#[derive(Clone, Debug, PartialEq)]
struct InteractiveScene3D {
    probe: Vec3,
    sphere_center: Vec3,
    aabb_center: Vec3,
    capsule_center: Vec3,
    triangle_offset: Vec3,
    sensor_center: Vec3,
    aabb_ray_origin: Vec3,
    sphere_ray_origin: Vec3,
    capsule_ray_origin: Vec3,
    triangle_ray_origin: Vec3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum InteractiveHandle2D {
    Probe = 0,
    Circle = 1,
    Aabb = 2,
    Capsule = 3,
    OrientedBox = 4,
    Polygon = 5,
    Sensor = 6,
    AabbRay = 7,
    CircleRay = 8,
    CapsuleRay = 9,
    PolygonRay = 10,
    OrientedBoxRay = 11,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum InteractiveHandle3D {
    Probe = 0,
    Sphere = 1,
    Aabb = 2,
    Capsule = 3,
    Triangle = 4,
    Sensor = 5,
    AabbRay = 6,
    SphereRay = 7,
    CapsuleRay = 8,
    TriangleRay = 9,
}

impl InteractiveHandle2D {
    fn from_raw(value: u32) -> Option<Self> {
        match value {
            0 => Some(Self::Probe),
            1 => Some(Self::Circle),
            2 => Some(Self::Aabb),
            3 => Some(Self::Capsule),
            4 => Some(Self::OrientedBox),
            5 => Some(Self::Polygon),
            6 => Some(Self::Sensor),
            7 => Some(Self::AabbRay),
            8 => Some(Self::CircleRay),
            9 => Some(Self::CapsuleRay),
            10 => Some(Self::PolygonRay),
            11 => Some(Self::OrientedBoxRay),
            _ => None,
        }
    }
}

impl InteractiveHandle3D {
    fn from_raw(value: u32) -> Option<Self> {
        match value {
            0 => Some(Self::Probe),
            1 => Some(Self::Sphere),
            2 => Some(Self::Aabb),
            3 => Some(Self::Capsule),
            4 => Some(Self::Triangle),
            5 => Some(Self::Sensor),
            6 => Some(Self::AabbRay),
            7 => Some(Self::SphereRay),
            8 => Some(Self::CapsuleRay),
            9 => Some(Self::TriangleRay),
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub struct Sandbox2D {
    scene: InteractiveScene2D,
}

#[wasm_bindgen]
pub struct Sandbox3D {
    scene: InteractiveScene3D,
}

#[wasm_bindgen]
pub struct Demo2D {
    tick: u32,
}

#[wasm_bindgen]
impl Demo2D {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { tick: 0 }
    }

    pub fn tick(&mut self) {
        self.tick = self.tick.wrapping_add(1);
    }

    pub fn reset(&mut self) {
        self.tick = 0;
    }

    pub fn render_state(&self) -> String {
        serde_json::to_string(&scene_2d_state(self.tick)).expect("scene state should serialize")
    }
}

impl Default for Demo2D {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
pub struct Demo3D {
    tick: u32,
}

#[wasm_bindgen]
impl Demo3D {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { tick: 0 }
    }

    pub fn tick(&mut self) {
        self.tick = self.tick.wrapping_add(1);
    }

    pub fn reset(&mut self) {
        self.tick = 0;
    }

    pub fn render_state(&self) -> String {
        serde_json::to_string(&scene_3d_state(self.tick)).expect("scene state should serialize")
    }
}

impl Default for Demo3D {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl Sandbox2D {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { scene: InteractiveScene2D::default() }
    }

    pub fn reset(&mut self) {
        self.scene = InteractiveScene2D::default();
    }

    pub fn move_handle(&mut self, handle: u32, x: f32, y: f32) {
        if let Some(handle) = InteractiveHandle2D::from_raw(handle) {
            self.scene.move_handle(handle, point_from_f32(x, y));
        }
    }

    pub fn render_state(&self) -> String {
        serde_json::to_string(&interactive_scene_2d_state(&self.scene))
            .expect("interactive scene state should serialize")
    }
}

impl Default for Sandbox2D {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl Sandbox3D {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { scene: InteractiveScene3D::default() }
    }

    pub fn reset(&mut self) {
        self.scene = InteractiveScene3D::default();
    }

    pub fn move_handle(&mut self, handle: u32, x: f32, y: f32, z: f32) {
        if let Some(handle) = InteractiveHandle3D::from_raw(handle) {
            self.scene.move_handle(handle, point3_from_f32(x, y, z));
        }
    }

    pub fn render_state(&self) -> String {
        serde_json::to_string(&interactive_scene_3d_state(&self.scene))
            .expect("interactive scene state should serialize")
    }
}

impl Default for Sandbox3D {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for InteractiveScene2D {
    fn default() -> Self {
        Self {
            probe: v2(43, 36),
            circle_center: v2(17, 16),
            aabb_center: v2(30, 21),
            capsule_center: v2(18, 41),
            oriented_box_center: v2(50, 22),
            polygon_offset: Vec2::ZERO,
            sensor_center: v2(17, 16),
            aabb_ray_origin: v2(4, 18),
            circle_ray_origin: v2(4, 16),
            capsule_ray_origin: v2(4, 41),
            polygon_ray_origin: v2(4, 44),
            oriented_box_ray_origin: v2(4, 22),
        }
    }
}

impl Default for InteractiveScene3D {
    fn default() -> Self {
        Self {
            probe: v3(24, 22, 20),
            sphere_center: v3(16, 16, 16),
            aabb_center: v3(26, 17, 17),
            capsule_center: v3(20, 34, 18),
            triangle_offset: Vec3::ZERO,
            sensor_center: v3(16, 16, 16),
            aabb_ray_origin: v3(4, 17, 17),
            sphere_ray_origin: v3(4, 16, 16),
            capsule_ray_origin: v3(4, 34, 18),
            triangle_ray_origin: v3(4, 22, 20),
        }
    }
}

impl InteractiveScene2D {
    fn move_handle(&mut self, handle: InteractiveHandle2D, target: Vec2) {
        match handle {
            InteractiveHandle2D::Probe => self.probe = clamp_free_point(target),
            InteractiveHandle2D::Circle => {
                self.circle_center = clamp_point_to_bounds(target, 6, 58, 6, 58)
            }
            InteractiveHandle2D::Aabb => {
                self.aabb_center = clamp_point_to_bounds(target, 10, 54, 9, 55)
            }
            InteractiveHandle2D::Capsule => {
                self.capsule_center = clamp_point_to_bounds(target, 15, 49, 10, 54)
            }
            InteractiveHandle2D::OrientedBox => {
                self.oriented_box_center = clamp_point_to_bounds(target, 10, 54, 10, 54)
            }
            InteractiveHandle2D::Polygon => {
                let delta = clamp_free_point(target) - self.polygon_handle();
                self.polygon_offset = clamp_polygon_offset(self.polygon_offset + delta);
            }
            InteractiveHandle2D::Sensor => {
                self.sensor_center = clamp_point_to_bounds(target, 8, 56, 8, 56)
            }
            InteractiveHandle2D::AabbRay => self.aabb_ray_origin = clamp_free_point(target),
            InteractiveHandle2D::CircleRay => self.circle_ray_origin = clamp_free_point(target),
            InteractiveHandle2D::CapsuleRay => self.capsule_ray_origin = clamp_free_point(target),
            InteractiveHandle2D::PolygonRay => self.polygon_ray_origin = clamp_free_point(target),
            InteractiveHandle2D::OrientedBoxRay => {
                self.oriented_box_ray_origin = clamp_free_point(target);
            }
        }
    }

    fn aabb(&self) -> Aabb {
        Aabb::try_new(
            self.aabb_center - Vec2::new(Fx32::from_int(8), Fx32::from_int(7)),
            self.aabb_center + Vec2::new(Fx32::from_int(8), Fx32::from_int(7)),
        )
        .expect("interactive aabb should remain valid")
    }

    fn circle(&self) -> Circle {
        Circle::try_new(self.circle_center, Fx32::from_int(6)).expect("interactive circle")
    }

    fn capsule(&self) -> Capsule {
        Capsule::try_new(
            Segment::new(self.capsule_center + v2(-10, -5), self.capsule_center + v2(10, 5)),
            Fx32::from_int(5),
        )
        .expect("interactive capsule")
    }

    fn oriented_box(&self) -> OrientedBox {
        OrientedBox::try_new(self.oriented_box_center, v2(6, 4), v2(1, 1), v2(-1, 1))
            .expect("interactive oriented box")
    }

    fn polygon(&self) -> ConvexPolygon<5> {
        ConvexPolygon::try_new([
            v2(40, 38) + self.polygon_offset,
            v2(51, 34) + self.polygon_offset,
            v2(58, 42) + self.polygon_offset,
            v2(54, 54) + self.polygon_offset,
            v2(42, 50) + self.polygon_offset,
        ])
        .expect("interactive polygon")
    }

    fn sensor(&self) -> Aabb {
        inflate_aabb(self.sensor_center, Fx32::from_int(8))
    }

    fn polygon_handle(&self) -> Vec2 {
        polygon_centroid(&self.polygon().points)
    }
}

impl InteractiveScene3D {
    fn move_handle(&mut self, handle: InteractiveHandle3D, target: Vec3) {
        match handle {
            InteractiveHandle3D::Probe => self.probe = clamp_free_point3(target),
            InteractiveHandle3D::Sphere => {
                self.sphere_center = clamp_point3_to_bounds(target, 5, 55, 5, 55, 5, 55)
            }
            InteractiveHandle3D::Aabb => {
                self.aabb_center = clamp_point3_to_bounds(target, 8, 52, 7, 57, 7, 57)
            }
            InteractiveHandle3D::Capsule => {
                self.capsule_center = clamp_point3_to_bounds(target, 14, 50, 4, 60, 12, 52)
            }
            InteractiveHandle3D::Triangle => {
                let delta = clamp_free_point3(target) - self.triangle_handle();
                self.triangle_offset = clamp_triangle_offset_3d(self.triangle_offset + delta);
            }
            InteractiveHandle3D::Sensor => {
                self.sensor_center = clamp_point3_to_bounds(target, 7, 53, 7, 53, 7, 53)
            }
            InteractiveHandle3D::AabbRay => self.aabb_ray_origin = clamp_free_point3(target),
            InteractiveHandle3D::SphereRay => self.sphere_ray_origin = clamp_free_point3(target),
            InteractiveHandle3D::CapsuleRay => {
                self.capsule_ray_origin = clamp_free_point3(target);
            }
            InteractiveHandle3D::TriangleRay => {
                self.triangle_ray_origin = clamp_free_point3(target);
            }
        }
    }

    fn sphere(&self) -> Sphere {
        Sphere::try_new(self.sphere_center, Fx32::from_int(5)).expect("interactive sphere")
    }

    fn aabb(&self) -> Aabb3 {
        Aabb3::try_new(
            self.aabb_center - Vec3::new(Fx32::from_int(8), Fx32::from_int(7), Fx32::from_int(7)),
            self.aabb_center + Vec3::new(Fx32::from_int(8), Fx32::from_int(7), Fx32::from_int(7)),
        )
        .expect("interactive aabb3 should remain valid")
    }

    fn capsule(&self) -> Capsule3 {
        Capsule3::try_new(
            Segment3::new(self.capsule_center + v3(-10, 0, -8), self.capsule_center + v3(10, 0, 8)),
            Fx32::from_int(4),
        )
        .expect("interactive capsule3")
    }

    fn triangle(&self) -> Triangle3 {
        Triangle3::new(
            v3(24, 16, 14) + self.triangle_offset,
            v3(24, 28, 14) + self.triangle_offset,
            v3(24, 22, 26) + self.triangle_offset,
        )
    }

    fn sensor(&self) -> Aabb3 {
        inflate_aabb3(self.sensor_center, Fx32::from_int(7))
    }

    fn triangle_handle(&self) -> Vec3 {
        triangle_reference_point(&self.triangle())
    }
}

fn interactive_scene_2d_state(scene: &InteractiveScene2D) -> InteractiveScene2DState {
    let aabb = scene.aabb();
    let circle = scene.circle();
    let capsule = scene.capsule();
    let oriented_box = scene.oriented_box();
    let polygon = scene.polygon();
    let sensor = scene.sensor();
    let max_toi = Fx32::from_int(96);

    let aabb_ray = Ray::new(scene.aabb_ray_origin, v2(1, 0));
    let circle_ray = Ray::new(scene.circle_ray_origin, v2(1, 0));
    let capsule_ray = Ray::new(scene.capsule_ray_origin, v2(1, 0));
    let polygon_ray = Ray::new(scene.polygon_ray_origin, v2(1, 0));
    let oriented_box_ray = Ray::new(scene.oriented_box_ray_origin, v2(1, 0));

    let aabb_ray_hit = raycast_aabb(&aabb_ray, &aabb, max_toi);
    let circle_ray_hit = raycast_circle(&circle_ray, &circle, max_toi);
    let capsule_ray_hit = raycast_capsule(&capsule_ray, &capsule, max_toi);
    let polygon_ray_hit = raycast_convex_polygon(&polygon_ray, &polygon, max_toi);
    let oriented_box_ray_hit = raycast_oriented_box(&oriented_box_ray, &oriented_box, max_toi);
    let candidate_ids = query_grid_2d(&aabb, &capsule, &oriented_box, &polygon, &sensor);

    InteractiveScene2DState {
        probe: vec2_to_arr(scene.probe),
        circle_center: vec2_to_arr(circle.center),
        circle_radius: fx_to_f32(circle.radius),
        aabb: aabb2_view(&aabb),
        sensor: aabb2_view(&sensor),
        capsule: Segment2View {
            start: vec2_to_arr(capsule.segment.start),
            end: vec2_to_arr(capsule.segment.end),
            radius: fx_to_f32(capsule.radius),
        },
        oriented_box: oriented_box_corners(&oriented_box),
        polygon: polygon.points.iter().copied().map(vec2_to_arr).collect(),
        aabb_ray: ray2_view(&aabb_ray, aabb_ray_hit),
        circle_ray: ray2_view(&circle_ray, circle_ray_hit),
        capsule_ray: ray2_view(&capsule_ray, capsule_ray_hit),
        polygon_ray: ray2_view(&polygon_ray, polygon_ray_hit),
        oriented_box_ray: ray2_view(&oriented_box_ray, oriented_box_ray_hit),
        candidate_ids,
        flags: InteractiveScene2DFlags {
            circle_hits_aabb: circle_overlaps_aabb(&circle, &aabb),
            circle_hits_capsule: circle_overlaps_capsule(&circle, &capsule),
            circle_hits_polygon: circle_overlaps_convex_polygon(&circle, &polygon),
            circle_hits_oriented_box: circle_overlaps_oriented_box(&circle, &oriented_box),
            probe_in_aabb: aabb.contains_point(scene.probe),
            probe_in_capsule: capsule_contains_point(&capsule, scene.probe),
            probe_in_polygon: convex_polygon_contains_point(&polygon, scene.probe),
            probe_in_oriented_box: oriented_box_contains_point(&oriented_box, scene.probe),
            ray_hits_aabb: aabb_ray_hit.is_some(),
            ray_hits_circle: circle_ray_hit.is_some(),
            ray_hits_capsule: capsule_ray_hit.is_some(),
            ray_hits_polygon: polygon_ray_hit.is_some(),
            ray_hits_oriented_box: oriented_box_ray_hit.is_some(),
        },
        handles: InteractiveHandles2D {
            probe: vec2_to_arr(scene.probe),
            circle: vec2_to_arr(scene.circle_center),
            aabb: vec2_to_arr(scene.aabb_center),
            capsule: vec2_to_arr(scene.capsule_center),
            oriented_box: vec2_to_arr(scene.oriented_box_center),
            polygon: vec2_to_arr(scene.polygon_handle()),
            sensor: vec2_to_arr(scene.sensor_center),
            aabb_ray: vec2_to_arr(scene.aabb_ray_origin),
            circle_ray: vec2_to_arr(scene.circle_ray_origin),
            capsule_ray: vec2_to_arr(scene.capsule_ray_origin),
            polygon_ray: vec2_to_arr(scene.polygon_ray_origin),
            oriented_box_ray: vec2_to_arr(scene.oriented_box_ray_origin),
        },
    }
}

fn interactive_scene_3d_state(scene: &InteractiveScene3D) -> InteractiveScene3DState {
    let sphere = scene.sphere();
    let aabb = scene.aabb();
    let capsule = scene.capsule();
    let triangle = scene.triangle();
    let sensor = scene.sensor();
    let triangle_closest_point = closest_point_on_triangle3(&triangle, scene.probe);
    let max_toi = Fx32::from_int(96);

    let aabb_ray = Ray3::new(scene.aabb_ray_origin, v3(1, 0, 0));
    let sphere_ray = Ray3::new(scene.sphere_ray_origin, v3(1, 0, 0));
    let capsule_ray = Ray3::new(scene.capsule_ray_origin, v3(1, 0, 0));
    let triangle_ray = Ray3::new(scene.triangle_ray_origin, v3(1, 0, 0));

    let aabb_ray_hit = raycast_aabb3(&aabb_ray, &aabb, max_toi);
    let sphere_ray_hit = raycast_sphere(&sphere_ray, &sphere, max_toi);
    let capsule_ray_hit = raycast_capsule3(&capsule_ray, &capsule, max_toi);
    let triangle_ray_hit = raycast_triangle3(&triangle_ray, &triangle, max_toi);
    let candidate_ids = query_grid_3d(&aabb, &capsule, &triangle, &sensor);

    InteractiveScene3DState {
        probe: vec3_to_arr(scene.probe),
        sphere_center: vec3_to_arr(sphere.center),
        sphere_radius: fx_to_f32(sphere.radius),
        aabb: aabb3_view(&aabb),
        sensor: aabb3_view(&sensor),
        capsule: Segment3View {
            start: vec3_to_arr(capsule.segment.start),
            end: vec3_to_arr(capsule.segment.end),
            radius: fx_to_f32(capsule.radius),
        },
        triangle: triangle3_view(&triangle),
        triangle_closest_point: vec3_to_arr(triangle_closest_point),
        aabb_ray: ray3_view(&aabb_ray, aabb_ray_hit),
        sphere_ray: ray3_view(&sphere_ray, sphere_ray_hit),
        capsule_ray: ray3_view(&capsule_ray, capsule_ray_hit),
        triangle_ray: ray3_view(&triangle_ray, triangle_ray_hit),
        candidate_ids,
        flags: InteractiveScene3DFlags {
            sphere_hits_aabb: sphere_overlaps_aabb3(&sphere, &aabb),
            sphere_hits_capsule: sphere_overlaps_capsule3(&sphere, &capsule),
            sphere_hits_triangle: sphere_overlaps_triangle3(&sphere, &triangle),
            probe_in_aabb: aabb.contains_point(scene.probe),
            probe_in_capsule: capsule3_contains_point(&capsule, scene.probe),
            probe_on_triangle: distance_squared_point_triangle3(&triangle, scene.probe)
                == Fx32::ZERO,
            ray_hits_aabb: aabb_ray_hit.is_some(),
            ray_hits_sphere: sphere_ray_hit.is_some(),
            ray_hits_capsule: capsule_ray_hit.is_some(),
            ray_hits_triangle: triangle_ray_hit.is_some(),
        },
        handles: InteractiveHandles3D {
            probe: vec3_to_arr(scene.probe),
            sphere: vec3_to_arr(scene.sphere_center),
            aabb: vec3_to_arr(scene.aabb_center),
            capsule: vec3_to_arr(scene.capsule_center),
            triangle: vec3_to_arr(scene.triangle_handle()),
            sensor: vec3_to_arr(scene.sensor_center),
            aabb_ray: vec3_to_arr(scene.aabb_ray_origin),
            sphere_ray: vec3_to_arr(scene.sphere_ray_origin),
            capsule_ray: vec3_to_arr(scene.capsule_ray_origin),
            triangle_ray: vec3_to_arr(scene.triangle_ray_origin),
        },
    }
}

fn scene_2d_state(tick: u32) -> Scene2DState {
    let probe = v2(triangle_wave(tick + 42, 120, 8, 58), triangle_wave(tick + 65, 96, 10, 50));
    let circle_center =
        v2(triangle_wave(tick + 10, 160, 12, 52), triangle_wave(tick + 98, 110, 8, 42));
    let circle_radius = Fx32::from_int(6);
    let circle = Circle::try_new(circle_center, circle_radius).expect("valid circle");

    let aabb = Aabb::try_new(v2(22, 14), v2(38, 28)).expect("valid aabb");
    let capsule = Capsule::try_new(Segment::new(v2(8, 36), v2(28, 46)), Fx32::from_int(5))
        .expect("valid capsule");
    let oriented_box =
        OrientedBox::try_new(v2(50, 22), v2(6, 4), v2(1, 1), v2(-1, 1)).expect("valid obb");
    let polygon =
        ConvexPolygon::try_new([v2(40, 38), v2(51, 34), v2(58, 42), v2(54, 54), v2(42, 50)])
            .expect("valid polygon");

    let sensor = inflate_aabb(circle_center, Fx32::from_int(8));
    let max_toi = Fx32::from_int(96);
    let aabb_ray = Ray::new(v2(4, triangle_wave(tick + 20, 120, 8, 34)), v2(1, 0));
    let aabb_ray_hit = raycast_aabb(&aabb_ray, &aabb, max_toi);
    let circle_ray = Ray::new(Vec2::new(Fx32::from_int(4), circle_center.y), v2(1, 0));
    let circle_ray_hit = raycast_circle(&circle_ray, &circle, max_toi);
    let capsule_mid_y = (capsule.segment.start.y + capsule.segment.end.y) / Fx32::from_int(2);
    let capsule_ray = Ray::new(Vec2::new(Fx32::from_int(4), capsule_mid_y), v2(1, 0));
    let capsule_ray_hit = raycast_capsule(&capsule_ray, &capsule, max_toi);
    let polygon_ray = Ray::new(v2(4, 44), v2(1, 0));
    let polygon_ray_hit = raycast_convex_polygon(&polygon_ray, &polygon, max_toi);
    let oriented_box_ray = Ray::new(v2(4, 22), v2(1, 0));
    let oriented_box_ray_hit = raycast_oriented_box(&oriented_box_ray, &oriented_box, max_toi);

    let candidate_ids = query_grid_2d(&aabb, &capsule, &oriented_box, &polygon, &sensor);

    Scene2DState {
        tick,
        probe: vec2_to_arr(probe),
        circle_center: vec2_to_arr(circle_center),
        circle_radius: fx_to_f32(circle_radius),
        aabb: aabb2_view(&aabb),
        sensor: aabb2_view(&sensor),
        capsule: Segment2View {
            start: vec2_to_arr(capsule.segment.start),
            end: vec2_to_arr(capsule.segment.end),
            radius: fx_to_f32(capsule.radius),
        },
        oriented_box: oriented_box_corners(&oriented_box),
        polygon: polygon.points.iter().copied().map(vec2_to_arr).collect(),
        aabb_ray: ray2_view(&aabb_ray, aabb_ray_hit),
        circle_ray: ray2_view(&circle_ray, circle_ray_hit),
        capsule_ray: ray2_view(&capsule_ray, capsule_ray_hit),
        polygon_ray: ray2_view(&polygon_ray, polygon_ray_hit),
        oriented_box_ray: ray2_view(&oriented_box_ray, oriented_box_ray_hit),
        candidate_ids,
        flags: Scene2DFlags {
            circle_hits_aabb: circle_overlaps_aabb(&circle, &aabb),
            probe_in_capsule: capsule_contains_point(&capsule, probe),
            probe_in_oriented_box: oriented_box_contains_point(&oriented_box, probe),
            probe_in_polygon: convex_polygon_contains_point(&polygon, probe),
            ray_hits_aabb: aabb_ray_hit.is_some(),
            ray_hits_circle: circle_ray_hit.is_some(),
            ray_hits_capsule: capsule_ray_hit.is_some(),
            ray_hits_polygon: polygon_ray_hit.is_some(),
            ray_hits_oriented_box: oriented_box_ray_hit.is_some(),
        },
    }
}

fn scene_3d_state(tick: u32) -> Scene3DState {
    let probe = v3(
        triangle_wave(tick + 28, 140, 8, 46),
        triangle_wave(tick + 80, 112, 8, 30),
        triangle_wave(tick + 50, 132, 6, 30),
    );
    let sphere_center = v3(
        triangle_wave(tick + 12, 180, 10, 44),
        triangle_wave(tick + 70, 126, 10, 28),
        triangle_wave(tick + 94, 156, 8, 28),
    );
    let sphere_radius = Fx32::from_int(5);
    let sphere = Sphere::try_new(sphere_center, sphere_radius).expect("valid sphere");

    let aabb = Aabb3::try_new(v3(18, 10, 10), v3(34, 24, 24)).expect("valid aabb3");
    let capsule =
        Capsule3::try_new(Segment3::new(v3(10, 34, 10), v3(30, 34, 26)), Fx32::from_int(4))
            .expect("valid capsule3");
    let triangle_segment = Segment3::new(v3(20, 22, 20), v3(28, 22, 20));
    let triangle_distance_segment = Segment3::new(v3(20, 20, 20), v3(20, 24, 20));
    let triangle_distance_sphere =
        Sphere::try_new(v3(32, 22, 20), Fx32::from_int(3)).expect("valid sphere");
    let triangle_distance_capsule =
        Capsule3::try_new(Segment3::new(v3(32, 22, 20), v3(32, 22, 20)), Fx32::from_int(2))
            .expect("valid capsule3");
    let triangle_skew_distance_sphere =
        Sphere::try_new(v3(33, 21, 19), Fx32::from_int(3)).expect("valid sphere");
    let triangle_skew_distance_capsule =
        Capsule3::try_new(Segment3::new(v3(32, 18, 18), v3(34, 26, 22)), Fx32::from_int(2))
            .expect("valid capsule3");
    let triangle_vertex_distance_sphere =
        Sphere::try_new(v3(27, 32, 14), Fx32::from_int(2)).expect("valid sphere");
    let triangle = Triangle3::new(v3(24, 16, 14), v3(24, 28, 14), v3(24, 22, 26));
    let triangle_peer = Triangle3::new(v3(24, 18, 16), v3(24, 30, 16), v3(24, 24, 28));
    let triangle_distance_peer = Triangle3::new(v3(30, 16, 14), v3(30, 18, 14), v3(30, 16, 16));
    let triangle_distance_aabb =
        Aabb3::try_new(v3(30, 18, 18), v3(34, 24, 24)).expect("valid aabb3");
    let triangle_vertex_distance_aabb =
        Aabb3::try_new(v3(28, 30, 12), v3(32, 34, 16)).expect("valid aabb3");
    let triangle_sweep_triangle = Triangle3::new(v3(4, 18, 18), v3(4, 24, 18), v3(4, 18, 24));
    let triangle_sweep_peer = Triangle3::new(v3(24, 18, 18), v3(24, 24, 18), v3(24, 18, 24));
    let sensor = inflate_aabb3(sphere_center, Fx32::from_int(7));
    let max_toi = Fx32::from_int(96);
    let aabb_ray = Ray3::new(v3(4, triangle_wave(tick + 18, 128, 10, 30), 16), v3(1, 0, 0));
    let aabb_ray_hit = raycast_aabb3(&aabb_ray, &aabb, max_toi);
    let sphere_ray =
        Ray3::new(Vec3::new(Fx32::from_int(4), sphere_center.y, sphere_center.z), v3(1, 0, 0));
    let sphere_ray_hit = raycast_sphere(&sphere_ray, &sphere, max_toi);
    let capsule_ray = Ray3::new(v3(4, 34, 10), v3(1, 0, 0));
    let capsule_ray_hit = raycast_capsule3(&capsule_ray, &capsule, max_toi);
    let triangle_ray = Ray3::new(v3(4, 22, 20), v3(1, 0, 0));
    let triangle_ray_hit = raycast_triangle3(&triangle_ray, &triangle, max_toi);
    let triangle_sweep_sphere =
        Sphere::try_new(v3(4, 22, 20), Fx32::from_int(3)).expect("valid sphere");
    let triangle_sweep_delta = v3(1, 0, 0);
    let triangle_sweep_hit = sweep_sphere_triangle3(
        &triangle_sweep_sphere,
        triangle_sweep_delta,
        &triangle,
        Fx32::from_int(32),
    );
    let triangle_capsule_sweep_capsule =
        Capsule3::try_new(Segment3::new(v3(4, 18, 20), v3(4, 26, 20)), Fx32::from_int(2))
            .expect("valid capsule3");
    let triangle_capsule_sweep_delta = v3(1, 0, 0);
    let triangle_capsule_sweep_hit = sweep_capsule3_triangle3(
        &triangle_capsule_sweep_capsule,
        triangle_capsule_sweep_delta,
        &triangle,
        Fx32::from_int(32),
    );
    let triangle_segment_sweep_segment = Segment3::new(v3(4, 20, 20), v3(4, 24, 20));
    let triangle_segment_sweep_delta = v3(1, 0, 0);
    let triangle_segment_sweep_hit = sweep_segment3_triangle3(
        &triangle_segment_sweep_segment,
        triangle_segment_sweep_delta,
        &triangle,
        Fx32::from_int(32),
    );
    let triangle_triangle_sweep_delta = v3(1, 0, 0);
    let triangle_triangle_sweep_hit = sweep_triangle3_triangle3(
        &triangle_sweep_triangle,
        triangle_triangle_sweep_delta,
        &triangle_sweep_peer,
        Fx32::from_int(32),
    );
    let triangle_closest_point = closest_point_on_triangle3(&triangle, probe);
    let triangle_segment_closest = closest_points_segment3_triangle3(&triangle_segment, &triangle);
    let triangle_distance_closest =
        closest_points_segment3_triangle3(&triangle_distance_segment, &triangle);
    let triangle_peer_distance =
        closest_points_triangle3_triangle3(&triangle, &triangle_distance_peer);
    let triangle_aabb_distance = closest_points_triangle3_aabb3(&triangle, &triangle_distance_aabb);
    let triangle_vertex_aabb_distance =
        closest_points_triangle3_aabb3(&triangle, &triangle_vertex_distance_aabb);
    let triangle_sphere_distance =
        closest_points_sphere_triangle3(&triangle_distance_sphere, &triangle);
    let triangle_capsule_distance =
        closest_points_capsule3_triangle3(&triangle_distance_capsule, &triangle);
    let triangle_skew_sphere_distance =
        closest_points_sphere_triangle3(&triangle_skew_distance_sphere, &triangle);
    let triangle_skew_capsule_distance =
        closest_points_capsule3_triangle3(&triangle_skew_distance_capsule, &triangle);
    let triangle_vertex_sphere_distance =
        closest_points_sphere_triangle3(&triangle_vertex_distance_sphere, &triangle);
    debug_assert!(
        distance_squared_triangle3_aabb3(&triangle, &triangle_distance_aabb) > Fx32::ZERO
    );
    debug_assert!(
        distance_squared_triangle3_aabb3(&triangle, &triangle_vertex_distance_aabb) > Fx32::ZERO
    );
    debug_assert!(
        distance_squared_sphere_triangle3(&triangle_distance_sphere, &triangle) > Fx32::ZERO
    );
    debug_assert!(
        distance_squared_capsule3_triangle3(&triangle_distance_capsule, &triangle) > Fx32::ZERO
    );
    debug_assert!(
        distance_squared_sphere_triangle3(&triangle_skew_distance_sphere, &triangle) > Fx32::ZERO
    );
    debug_assert!(
        distance_squared_capsule3_triangle3(&triangle_skew_distance_capsule, &triangle)
            > Fx32::ZERO
    );
    debug_assert!(
        distance_squared_sphere_triangle3(&triangle_vertex_distance_sphere, &triangle) > Fx32::ZERO
    );

    let candidate_ids = query_grid_3d(&aabb, &capsule, &triangle, &sensor);

    Scene3DState {
        tick,
        probe: vec3_to_arr(probe),
        sphere_center: vec3_to_arr(sphere_center),
        sphere_radius: fx_to_f32(sphere_radius),
        aabb: aabb3_view(&aabb),
        sensor: aabb3_view(&sensor),
        capsule: Segment3View {
            start: vec3_to_arr(capsule.segment.start),
            end: vec3_to_arr(capsule.segment.end),
            radius: fx_to_f32(capsule.radius),
        },
        triangle_segment: Segment3View {
            start: vec3_to_arr(triangle_segment.start),
            end: vec3_to_arr(triangle_segment.end),
            radius: 0.0,
        },
        triangle_distance_segment: Segment3View {
            start: vec3_to_arr(triangle_distance_segment.start),
            end: vec3_to_arr(triangle_distance_segment.end),
            radius: 0.0,
        },
        triangle_distance_sphere_center: vec3_to_arr(triangle_distance_sphere.center),
        triangle_distance_sphere_radius: fx_to_f32(triangle_distance_sphere.radius),
        triangle_distance_capsule: Segment3View {
            start: vec3_to_arr(triangle_distance_capsule.segment.start),
            end: vec3_to_arr(triangle_distance_capsule.segment.end),
            radius: fx_to_f32(triangle_distance_capsule.radius),
        },
        triangle_skew_distance_sphere_center: vec3_to_arr(triangle_skew_distance_sphere.center),
        triangle_skew_distance_sphere_radius: fx_to_f32(triangle_skew_distance_sphere.radius),
        triangle_skew_distance_capsule: Segment3View {
            start: vec3_to_arr(triangle_skew_distance_capsule.segment.start),
            end: vec3_to_arr(triangle_skew_distance_capsule.segment.end),
            radius: fx_to_f32(triangle_skew_distance_capsule.radius),
        },
        triangle_vertex_distance_sphere_center: vec3_to_arr(triangle_vertex_distance_sphere.center),
        triangle_vertex_distance_sphere_radius: fx_to_f32(triangle_vertex_distance_sphere.radius),
        triangle: triangle3_view(&triangle),
        triangle_peer: triangle3_view(&triangle_peer),
        triangle_distance_peer: triangle3_view(&triangle_distance_peer),
        triangle_distance_aabb: aabb3_view(&triangle_distance_aabb),
        triangle_vertex_distance_aabb: aabb3_view(&triangle_vertex_distance_aabb),
        triangle_sweep_triangle: triangle3_view(&triangle_sweep_triangle),
        triangle_sweep_peer: triangle3_view(&triangle_sweep_peer),
        triangle_closest_point: vec3_to_arr(triangle_closest_point),
        triangle_segment_closest_point: vec3_to_arr(triangle_segment_closest.segment_point),
        triangle_distance_segment_point: vec3_to_arr(triangle_distance_closest.segment_point),
        triangle_distance_triangle_point: vec3_to_arr(triangle_distance_closest.triangle_point),
        triangle_peer_distance_point_a: vec3_to_arr(triangle_peer_distance.point_a),
        triangle_peer_distance_point_b: vec3_to_arr(triangle_peer_distance.point_b),
        triangle_aabb_distance_point_triangle: vec3_to_arr(triangle_aabb_distance.point_a),
        triangle_aabb_distance_point_aabb: vec3_to_arr(triangle_aabb_distance.point_b),
        triangle_vertex_aabb_distance_point_triangle: vec3_to_arr(
            triangle_vertex_aabb_distance.point_a,
        ),
        triangle_vertex_aabb_distance_point_aabb: vec3_to_arr(
            triangle_vertex_aabb_distance.point_b,
        ),
        triangle_sphere_distance_point_sphere: vec3_to_arr(triangle_sphere_distance.point_a),
        triangle_sphere_distance_point_triangle: vec3_to_arr(triangle_sphere_distance.point_b),
        triangle_capsule_distance_point_capsule: vec3_to_arr(triangle_capsule_distance.point_a),
        triangle_capsule_distance_point_triangle: vec3_to_arr(triangle_capsule_distance.point_b),
        triangle_skew_sphere_distance_point_sphere: vec3_to_arr(
            triangle_skew_sphere_distance.point_a,
        ),
        triangle_skew_sphere_distance_point_triangle: vec3_to_arr(
            triangle_skew_sphere_distance.point_b,
        ),
        triangle_skew_capsule_distance_point_capsule: vec3_to_arr(
            triangle_skew_capsule_distance.point_a,
        ),
        triangle_skew_capsule_distance_point_triangle: vec3_to_arr(
            triangle_skew_capsule_distance.point_b,
        ),
        triangle_vertex_sphere_distance_point_sphere: vec3_to_arr(
            triangle_vertex_sphere_distance.point_a,
        ),
        triangle_vertex_sphere_distance_point_triangle: vec3_to_arr(
            triangle_vertex_sphere_distance.point_b,
        ),
        aabb_ray: ray3_view(&aabb_ray, aabb_ray_hit),
        sphere_ray: ray3_view(&sphere_ray, sphere_ray_hit),
        capsule_ray: ray3_view(&capsule_ray, capsule_ray_hit),
        triangle_ray: ray3_view(&triangle_ray, triangle_ray_hit),
        triangle_sweep: ray3_view(
            &Ray3::new(triangle_sweep_sphere.center, triangle_sweep_delta),
            triangle_sweep_hit,
        ),
        triangle_sweep_radius: fx_to_f32(triangle_sweep_sphere.radius),
        triangle_capsule_sweep_capsule: Segment3View {
            start: vec3_to_arr(triangle_capsule_sweep_capsule.segment.start),
            end: vec3_to_arr(triangle_capsule_sweep_capsule.segment.end),
            radius: fx_to_f32(triangle_capsule_sweep_capsule.radius),
        },
        triangle_capsule_sweep: ray3_view(
            &Ray3::new(
                (triangle_capsule_sweep_capsule.segment.start
                    + triangle_capsule_sweep_capsule.segment.end)
                    * Fx32::from_ratio(1, 2),
                triangle_capsule_sweep_delta,
            ),
            triangle_capsule_sweep_hit,
        ),
        triangle_segment_sweep_segment: Segment3View {
            start: vec3_to_arr(triangle_segment_sweep_segment.start),
            end: vec3_to_arr(triangle_segment_sweep_segment.end),
            radius: 0.0,
        },
        triangle_segment_sweep: ray3_view(
            &Ray3::new(
                (triangle_segment_sweep_segment.start + triangle_segment_sweep_segment.end)
                    * Fx32::from_ratio(1, 2),
                triangle_segment_sweep_delta,
            ),
            triangle_segment_sweep_hit,
        ),
        triangle_triangle_sweep: ray3_view(
            &Ray3::new(
                triangle_reference_point(&triangle_sweep_triangle),
                triangle_triangle_sweep_delta,
            ),
            triangle_triangle_sweep_hit,
        ),
        candidate_ids,
        flags: Scene3DFlags {
            sphere_hits_aabb: sphere_overlaps_aabb3(&sphere, &aabb),
            sphere_hits_capsule: sphere_overlaps_capsule3(&sphere, &capsule),
            sphere_hits_triangle: sphere_overlaps_triangle3(&sphere, &triangle),
            capsule_hits_triangle: capsule3_overlaps_triangle3(&capsule, &triangle),
            segment_hits_triangle: segment3_overlaps_triangle3(&triangle_segment, &triangle),
            triangle_hits_aabb: triangle3_overlaps_aabb3(&triangle, &aabb),
            triangle_hits_triangle: triangle3_overlaps_triangle3(&triangle, &triangle_peer),
            probe_in_capsule: capsule3_contains_point(&capsule, probe),
            probe_on_triangle: distance_squared_point_triangle3(&triangle, probe) == Fx32::ZERO,
            ray_hits_aabb: aabb_ray_hit.is_some(),
            ray_hits_sphere: sphere_ray_hit.is_some(),
            ray_hits_capsule: capsule_ray_hit.is_some(),
            ray_hits_triangle: triangle_ray_hit.is_some(),
            sweep_hits_triangle: triangle_sweep_hit.is_some(),
            capsule_sweep_hits_triangle: triangle_capsule_sweep_hit.is_some(),
            segment_sweep_hits_triangle: triangle_segment_sweep_hit.is_some(),
            triangle_sweep_hits_triangle: triangle_triangle_sweep_hit.is_some(),
        },
    }
}

fn query_grid_2d(
    aabb: &Aabb,
    capsule: &Capsule,
    oriented_box: &OrientedBox,
    polygon: &ConvexPolygon<5>,
    sensor: &Aabb,
) -> Vec<u32> {
    let mut grid: UniformGrid<64, 4> =
        UniformGrid::new(8, 8, Vec2::ZERO, Fx32::from_int(8)).expect("grid");
    let mut out = [0u32; 8];
    let obb_vertices = oriented_box_corners_vec(oriented_box);
    let polygon_vertices = polygon.points.to_vec();

    let capsule_bounds = aabb_from_points_2d(&[
        v2(
            capsule.segment.start.x.floor_to_int() - capsule.radius.floor_to_int(),
            capsule.segment.start.y.floor_to_int() - capsule.radius.floor_to_int(),
        ),
        v2(
            capsule.segment.start.x.floor_to_int() + capsule.radius.floor_to_int(),
            capsule.segment.start.y.floor_to_int() + capsule.radius.floor_to_int(),
        ),
        v2(
            capsule.segment.end.x.floor_to_int() - capsule.radius.floor_to_int(),
            capsule.segment.end.y.floor_to_int() - capsule.radius.floor_to_int(),
        ),
        v2(
            capsule.segment.end.x.floor_to_int() + capsule.radius.floor_to_int(),
            capsule.segment.end.y.floor_to_int() + capsule.radius.floor_to_int(),
        ),
    ]);
    let obb_bounds = aabb_from_points_2d(&obb_vertices);
    let polygon_bounds = aabb_from_points_2d(&polygon_vertices);

    grid.insert(11, aabb).expect("insert");
    grid.insert(22, &capsule_bounds).expect("insert");
    grid.insert(33, &obb_bounds).expect("insert");
    grid.insert(44, &polygon_bounds).expect("insert");

    let len = grid.query_aabb(sensor, &mut out).expect("query");
    out[..len].to_vec()
}

fn query_grid_3d(
    aabb: &Aabb3,
    capsule: &Capsule3,
    triangle: &Triangle3,
    sensor: &Aabb3,
) -> Vec<u32> {
    let mut grid: UniformGrid3<125, 4> =
        UniformGrid3::new(5, 5, 5, Vec3::ZERO, Fx32::from_int(12)).expect("grid3");
    let mut out = [0u32; 8];
    let capsule_bounds = aabb3_from_points(&[
        capsule.segment.start + Vec3::new(-capsule.radius, -capsule.radius, -capsule.radius),
        capsule.segment.start + Vec3::new(capsule.radius, capsule.radius, capsule.radius),
        capsule.segment.end + Vec3::new(-capsule.radius, -capsule.radius, -capsule.radius),
        capsule.segment.end + Vec3::new(capsule.radius, capsule.radius, capsule.radius),
    ]);
    let triangle_bounds = aabb3_from_points(&[triangle.a, triangle.b, triangle.c]);

    grid.insert(101, aabb).expect("insert");
    grid.insert(202, &capsule_bounds).expect("insert");
    grid.insert(303, &triangle_bounds).expect("insert");

    let len = grid.query_aabb(sensor, &mut out).expect("query");
    out[..len].to_vec()
}

fn oriented_box_corners(oriented_box: &OrientedBox) -> [[f32; 2]; 4] {
    let vertices = oriented_box_corners_vec(oriented_box);
    [
        vec2_to_arr(vertices[0]),
        vec2_to_arr(vertices[1]),
        vec2_to_arr(vertices[2]),
        vec2_to_arr(vertices[3]),
    ]
}

fn oriented_box_corners_vec(oriented_box: &OrientedBox) -> Vec<Vec2> {
    let hx = oriented_box.half_extents.x;
    let hy = oriented_box.half_extents.y;
    vec![
        oriented_box.center + oriented_box.axis_x * -hx + oriented_box.axis_y * -hy,
        oriented_box.center + oriented_box.axis_x * hx + oriented_box.axis_y * -hy,
        oriented_box.center + oriented_box.axis_x * hx + oriented_box.axis_y * hy,
        oriented_box.center + oriented_box.axis_x * -hx + oriented_box.axis_y * hy,
    ]
}

fn inflate_aabb(center: Vec2, half_extent: Fx32) -> Aabb {
    Aabb::try_new(
        Vec2::new(center.x - half_extent, center.y - half_extent),
        Vec2::new(center.x + half_extent, center.y + half_extent),
    )
    .expect("inflated aabb")
}

fn inflate_aabb3(center: Vec3, half_extent: Fx32) -> Aabb3 {
    Aabb3::try_new(
        Vec3::new(center.x - half_extent, center.y - half_extent, center.z - half_extent),
        Vec3::new(center.x + half_extent, center.y + half_extent, center.z + half_extent),
    )
    .expect("inflated aabb3")
}

fn aabb_from_points_2d(points: &[Vec2]) -> Aabb {
    let mut min = points[0];
    let mut max = points[0];
    for point in &points[1..] {
        min = min.component_min(*point);
        max = max.component_max(*point);
    }
    Aabb::try_new(min, max).expect("point bounds")
}

fn aabb3_from_points(points: &[Vec3]) -> Aabb3 {
    let mut min = points[0];
    let mut max = points[0];
    for point in &points[1..] {
        min = min.component_min(*point);
        max = max.component_max(*point);
    }
    Aabb3::try_new(min, max).expect("point bounds")
}

fn aabb2_view(aabb: &Aabb) -> Aabb2View {
    Aabb2View { min: vec2_to_arr(aabb.min), max: vec2_to_arr(aabb.max) }
}

fn aabb3_view(aabb: &Aabb3) -> Aabb3View {
    Aabb3View { min: vec3_to_arr(aabb.min), max: vec3_to_arr(aabb.max) }
}

fn triangle3_view(triangle: &Triangle3) -> [[f32; 3]; 3] {
    [vec3_to_arr(triangle.a), vec3_to_arr(triangle.b), vec3_to_arr(triangle.c)]
}

fn triangle_reference_point(triangle: &Triangle3) -> Vec3 {
    (triangle.a + triangle.b + triangle.c) * Fx32::from_ratio(1, 3)
}

fn ray2_view(ray: &Ray, hit: Option<bumpbox_core::RayHit>) -> Ray2View {
    Ray2View {
        origin: vec2_to_arr(ray.origin),
        dir: vec2_to_arr(ray.dir),
        hit_point: hit.map(|value| vec2_to_arr(value.point)),
        hit_normal: hit.map(|value| vec2_to_arr(value.normal)),
    }
}

fn ray3_view(ray: &Ray3, hit: Option<bumpbox_core::RayHit3>) -> Ray3View {
    Ray3View {
        origin: vec3_to_arr(ray.origin),
        dir: vec3_to_arr(ray.dir),
        hit_point: hit.map(|value| vec3_to_arr(value.point)),
        hit_normal: hit.map(|value| vec3_to_arr(value.normal)),
    }
}

fn point_from_f32(x: f32, y: f32) -> Vec2 {
    Vec2::new(f32_to_fx32(x), f32_to_fx32(y))
}

fn point3_from_f32(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(f32_to_fx32(x), f32_to_fx32(y), f32_to_fx32(z))
}

fn f32_to_fx32(value: f32) -> Fx32 {
    let scaled = (value * SCALE_F32).round();
    let raw = scaled.clamp(i32::MIN as f32, i32::MAX as f32) as i32;
    Fx32::from_raw(raw)
}

fn clamp_free_point(point: Vec2) -> Vec2 {
    clamp_point_to_bounds(point, 2, 62, 2, 62)
}

fn clamp_point_to_bounds(point: Vec2, min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> Vec2 {
    Vec2::new(
        point.x.max(Fx32::from_int(min_x)).min(Fx32::from_int(max_x)),
        point.y.max(Fx32::from_int(min_y)).min(Fx32::from_int(max_y)),
    )
}

fn clamp_polygon_offset(offset: Vec2) -> Vec2 {
    clamp_point_to_bounds(offset, -38, 4, -32, 8)
}

fn clamp_free_point3(point: Vec3) -> Vec3 {
    clamp_point3_to_bounds(point, 2, 62, 2, 62, 2, 62)
}

fn clamp_point3_to_bounds(
    point: Vec3,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    min_z: i32,
    max_z: i32,
) -> Vec3 {
    Vec3::new(
        point.x.max(Fx32::from_int(min_x)).min(Fx32::from_int(max_x)),
        point.y.max(Fx32::from_int(min_y)).min(Fx32::from_int(max_y)),
        point.z.max(Fx32::from_int(min_z)).min(Fx32::from_int(max_z)),
    )
}

fn clamp_triangle_offset_3d(offset: Vec3) -> Vec3 {
    clamp_point3_to_bounds(offset, -12, 12, -12, 12, -12, 12)
}

fn polygon_centroid<const N: usize>(points: &[Vec2; N]) -> Vec2 {
    let mut sum_x = 0i64;
    let mut sum_y = 0i64;

    for point in points {
        sum_x += point.x.raw() as i64;
        sum_y += point.y.raw() as i64;
    }

    Vec2::new(Fx32::from_raw((sum_x / N as i64) as i32), Fx32::from_raw((sum_y / N as i64) as i32))
}

fn v2(x: i32, y: i32) -> Vec2 {
    Vec2::new(Fx32::from_int(x), Fx32::from_int(y))
}

fn v3(x: i32, y: i32, z: i32) -> Vec3 {
    Vec3::new(Fx32::from_int(x), Fx32::from_int(y), Fx32::from_int(z))
}

fn vec2_to_arr(vec: Vec2) -> [f32; 2] {
    [fx_to_f32(vec.x), fx_to_f32(vec.y)]
}

fn vec3_to_arr(vec: Vec3) -> [f32; 3] {
    [fx_to_f32(vec.x), fx_to_f32(vec.y), fx_to_f32(vec.z)]
}

fn fx_to_f32(value: Fx32) -> f32 {
    value.raw() as f32 / SCALE_F32
}

fn triangle_wave(tick: u32, period: u32, min: i32, max: i32) -> i32 {
    let half = period / 2;
    let span = max - min;
    let phase = tick % period;
    if phase <= half {
        min + ((span as i64 * phase as i64) / half as i64) as i32
    } else {
        max - ((span as i64 * (phase - half) as i64) / half as i64) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::{
        interactive_scene_2d_state, interactive_scene_3d_state, point3_from_f32, point_from_f32,
        scene_2d_state, scene_3d_state, InteractiveHandle2D, InteractiveHandle3D,
        InteractiveScene2D, InteractiveScene3D,
    };

    #[test]
    fn interactive_scene_2d_is_deterministic_and_sorted() {
        let left = interactive_scene_2d_state(&InteractiveScene2D::default());
        let right = interactive_scene_2d_state(&InteractiveScene2D::default());

        assert_eq!(left, right);
        assert!(left.candidate_ids.windows(2).all(|pair| pair[0] <= pair[1]));
        assert!(left.flags.ray_hits_aabb);
        assert!(left.flags.ray_hits_circle);
        assert!(left.flags.ray_hits_capsule);
    }

    #[test]
    fn interactive_scene_moves_probe_circle_and_sensor() {
        let mut scene = InteractiveScene2D::default();
        scene.move_handle(InteractiveHandle2D::Circle, point_from_f32(30.0, 21.0));
        scene.move_handle(InteractiveHandle2D::Probe, point_from_f32(18.0, 41.0));
        scene.move_handle(InteractiveHandle2D::Sensor, point_from_f32(2.0, 2.0));

        let state = interactive_scene_2d_state(&scene);
        assert!(state.flags.circle_hits_aabb);
        assert!(state.flags.probe_in_capsule);
        assert!(state.candidate_ids.is_empty());
    }

    #[test]
    fn interactive_scene_3d_is_deterministic_and_sorted() {
        let left = interactive_scene_3d_state(&InteractiveScene3D::default());
        let right = interactive_scene_3d_state(&InteractiveScene3D::default());

        assert_eq!(left, right);
        assert!(left.candidate_ids.windows(2).all(|pair| pair[0] <= pair[1]));
        assert!(left.flags.ray_hits_aabb);
        assert!(left.flags.ray_hits_sphere);
        assert!(left.flags.ray_hits_capsule);
        assert!(left.flags.ray_hits_triangle);
    }

    #[test]
    fn interactive_scene_3d_moves_probe_sphere_and_sensor() {
        let mut scene = InteractiveScene3D::default();
        scene.move_handle(InteractiveHandle3D::Sphere, point3_from_f32(24.0, 22.0, 20.0));
        scene.move_handle(InteractiveHandle3D::Probe, point3_from_f32(20.0, 34.0, 18.0));
        scene.move_handle(InteractiveHandle3D::Sensor, point3_from_f32(53.0, 53.0, 53.0));

        let state = interactive_scene_3d_state(&scene);
        assert!(state.flags.sphere_hits_triangle);
        assert!(state.flags.probe_in_capsule);
        assert!(state.candidate_ids.is_empty());
    }

    #[test]
    fn scene_2d_snapshot_is_deterministic_and_sorted() {
        let left = scene_2d_state(12);
        let right = scene_2d_state(12);

        assert_eq!(left, right);
        assert!(left.candidate_ids.windows(2).all(|pair| pair[0] <= pair[1]));
        assert!(left.flags.ray_hits_aabb);
        assert!(left.flags.ray_hits_circle);
        assert!(left.flags.ray_hits_capsule);
        assert!(left.flags.ray_hits_polygon);
        assert!(left.flags.ray_hits_oriented_box);
    }

    #[test]
    fn scene_3d_snapshot_is_deterministic_and_sorted() {
        let left = scene_3d_state(18);
        let right = scene_3d_state(18);

        assert_eq!(left, right);
        assert!(left.candidate_ids.windows(2).all(|pair| pair[0] <= pair[1]));
        assert!(left.candidate_ids.contains(&303));
        assert!(left.flags.sphere_hits_triangle);
        assert!(left.flags.segment_hits_triangle);
        assert!(left.flags.triangle_hits_aabb);
        assert!(left.flags.triangle_hits_triangle);
        assert_eq!(left.triangle_segment_closest_point, [24.0, 22.0, 20.0]);
        assert_eq!(left.triangle_peer_distance_point_a, [24.0, 16.0, 14.0]);
        assert_eq!(left.triangle_peer_distance_point_b, [30.0, 16.0, 14.0]);
        assert!((left.triangle_aabb_distance_point_triangle[0] - 24.0).abs() < 0.01);
        assert!((left.triangle_aabb_distance_point_triangle[1] - 18.0).abs() < 0.01);
        assert!((left.triangle_aabb_distance_point_triangle[2] - 18.0).abs() < 0.01);
        assert!((left.triangle_aabb_distance_point_aabb[0] - 30.0).abs() < 0.01);
        assert!((left.triangle_aabb_distance_point_aabb[1] - 18.0).abs() < 0.01);
        assert!((left.triangle_aabb_distance_point_aabb[2] - 18.0).abs() < 0.01);
        assert!((left.triangle_vertex_aabb_distance_point_triangle[0] - 24.0).abs() < 0.01);
        assert!((left.triangle_vertex_aabb_distance_point_triangle[1] - 28.0).abs() < 0.01);
        assert!((left.triangle_vertex_aabb_distance_point_triangle[2] - 14.0).abs() < 0.01);
        assert!((left.triangle_vertex_aabb_distance_point_aabb[0] - 28.0).abs() < 0.01);
        assert!((left.triangle_vertex_aabb_distance_point_aabb[1] - 30.0).abs() < 0.01);
        assert!((left.triangle_vertex_aabb_distance_point_aabb[2] - 14.0).abs() < 0.01);
        assert_eq!(left.triangle_sphere_distance_point_sphere, [29.0, 22.0, 20.0]);
        assert_eq!(left.triangle_sphere_distance_point_triangle, [24.0, 22.0, 20.0]);
        assert_eq!(left.triangle_capsule_distance_point_capsule, [30.0, 22.0, 20.0]);
        assert_eq!(left.triangle_capsule_distance_point_triangle, [24.0, 22.0, 20.0]);
        assert!((left.triangle_skew_sphere_distance_point_sphere[0] - 30.0).abs() < 0.01);
        assert!((left.triangle_skew_sphere_distance_point_sphere[1] - 21.0).abs() < 0.01);
        assert!((left.triangle_skew_sphere_distance_point_sphere[2] - 19.0).abs() < 0.01);
        assert!((left.triangle_skew_sphere_distance_point_triangle[0] - 24.0).abs() < 0.01);
        assert!((left.triangle_skew_sphere_distance_point_triangle[1] - 21.0).abs() < 0.01);
        assert!((left.triangle_skew_sphere_distance_point_triangle[2] - 19.0).abs() < 0.01);
        assert!((left.triangle_skew_capsule_distance_point_capsule[0] - 30.0).abs() < 0.01);
        assert!((left.triangle_skew_capsule_distance_point_capsule[1] - 18.0).abs() < 0.01);
        assert!((left.triangle_skew_capsule_distance_point_capsule[2] - 18.0).abs() < 0.01);
        assert!((left.triangle_skew_capsule_distance_point_triangle[0] - 24.0).abs() < 0.01);
        assert!((left.triangle_skew_capsule_distance_point_triangle[1] - 18.0).abs() < 0.01);
        assert!((left.triangle_skew_capsule_distance_point_triangle[2] - 18.0).abs() < 0.01);
        assert!((left.triangle_vertex_sphere_distance_point_sphere[0] - 25.8).abs() < 0.02);
        assert!((left.triangle_vertex_sphere_distance_point_sphere[1] - 30.4).abs() < 0.02);
        assert!((left.triangle_vertex_sphere_distance_point_sphere[2] - 14.0).abs() < 0.01);
        assert!((left.triangle_vertex_sphere_distance_point_triangle[0] - 24.0).abs() < 0.01);
        assert!((left.triangle_vertex_sphere_distance_point_triangle[1] - 28.0).abs() < 0.01);
        assert!((left.triangle_vertex_sphere_distance_point_triangle[2] - 14.0).abs() < 0.01);
        assert!((left.triangle_distance_segment_point[0] - 20.0).abs() < 0.01);
        assert!((left.triangle_distance_segment_point[1] - 20.0).abs() < 0.01);
        assert!((left.triangle_distance_segment_point[2] - 20.0).abs() < 0.01);
        assert!((left.triangle_distance_triangle_point[0] - 24.0).abs() < 0.01);
        assert!((left.triangle_distance_triangle_point[1] - 20.0).abs() < 0.01);
        assert!((left.triangle_distance_triangle_point[2] - 20.0).abs() < 0.01);
        assert!(left.flags.ray_hits_aabb);
        assert!(left.flags.ray_hits_sphere);
        assert!(left.flags.ray_hits_capsule);
        assert!(left.flags.ray_hits_triangle);
        assert!(left.flags.sweep_hits_triangle);
        assert!(left.flags.capsule_sweep_hits_triangle);
        assert!(left.flags.segment_sweep_hits_triangle);
        assert!(left.flags.triangle_sweep_hits_triangle);
    }
}
