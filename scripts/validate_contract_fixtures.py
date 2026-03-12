#!/usr/bin/env python3
from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / "fixtures" / "contracts"

def fail(message: str) -> None:
    print(f"[contract-validation] {message}", file=sys.stderr)
    raise SystemExit(1)

def load(name: str):
    with (FIXTURES / name).open("r", encoding="utf-8") as handle:
        return json.load(handle)

def is_point2(value) -> bool:
    return isinstance(value, list) and len(value) == 2 and all(isinstance(component, int) for component in value)

def is_point3(value) -> bool:
    return isinstance(value, list) and len(value) == 3 and all(isinstance(component, int) for component in value)

def is_circle(value) -> bool:
    return (
        isinstance(value, dict)
        and is_point2(value.get("center"))
        and isinstance(value.get("radius"), int)
    )

def is_segment(value) -> bool:
    return (
        isinstance(value, dict)
        and is_point2(value.get("start"))
        and is_point2(value.get("end"))
    )

def is_segment3(value) -> bool:
    return (
        isinstance(value, dict)
        and is_point3(value.get("start"))
        and is_point3(value.get("end"))
    )

def is_capsule(value) -> bool:
    return (
        isinstance(value, dict)
        and is_point2(value.get("start"))
        and is_point2(value.get("end"))
        and isinstance(value.get("radius"), int)
    )

def is_polygon(value) -> bool:
    points = value.get("points") if isinstance(value, dict) else None
    return isinstance(points, list) and len(points) >= 3 and all(is_point2(point) for point in points)

def is_oriented_box(value) -> bool:
    return (
        isinstance(value, dict)
        and is_point2(value.get("center"))
        and is_point2(value.get("half_extents"))
        and is_point2(value.get("axis_x"))
        and is_point2(value.get("axis_y"))
    )

def is_sphere(value) -> bool:
    return (
        isinstance(value, dict)
        and is_point3(value.get("center"))
        and isinstance(value.get("radius"), int)
    )

def is_capsule3(value) -> bool:
    return (
        isinstance(value, dict)
        and is_point3(value.get("start"))
        and is_point3(value.get("end"))
        and isinstance(value.get("radius"), int)
    )

def is_triangle3(value) -> bool:
    return (
        isinstance(value, dict)
        and is_point3(value.get("a"))
        and is_point3(value.get("b"))
        and is_point3(value.get("c"))
    )

def validate_query_cases(payload, expect_valid: bool) -> None:
    ok = True
    if not isinstance(payload, dict) or not isinstance(payload.get("cases"), list) or not payload["cases"]:
        ok = False
    else:
        for case in payload["cases"]:
            if not isinstance(case, dict):
                ok = False
                continue
            if not isinstance(case.get("name"), str) or not case["name"]:
                ok = False
            kind = case.get("kind")
            if kind not in {
                "point_in_aabb",
                "closest_point_segment",
                "distance_squared_point_segment",
                "closest_point_circle",
                "distance_squared_point_circle",
                "closest_point_capsule",
                "distance_squared_point_capsule",
                "closest_point_polygon",
                "distance_squared_point_polygon",
                "closest_point_oriented_box",
                "distance_squared_point_oriented_box",
                "raycast_aabb",
                "raycast_circle",
                "raycast_capsule",
                "raycast_polygon",
                "raycast_oriented_box",
                "sweep_circle_aabb",
                "sweep_circle_capsule",
                "sweep_circle_segment",
                "sweep_circle_circle",
                "sweep_circle_polygon",
                "sweep_circle_oriented_box",
                "sweep_capsule_aabb",
                "sweep_capsule_circle",
                "sweep_capsule_segment",
                "sweep_capsule_capsule",
                "sweep_capsule_polygon",
                "sweep_capsule_oriented_box",
                "segment_intersects_segment",
                "circle_overlaps_segment",
                "capsule_overlaps_segment",
                "circle_overlaps_capsule",
                "circle_overlaps_polygon",
                "circle_overlaps_oriented_box",
                "capsule_overlaps_aabb",
                "capsule_overlaps_capsule",
                "capsule_overlaps_polygon",
                "capsule_overlaps_oriented_box",
                "polygon_overlaps_aabb",
                "polygon_overlaps_polygon",
                "oriented_box_overlaps_aabb",
                "oriented_box_overlaps_oriented_box",
                "point_in_aabb3",
                "closest_point_segment3",
                "distance_squared_point_segment3",
                "closest_point_sphere",
                "distance_squared_point_sphere",
                "closest_point_capsule3",
                "distance_squared_point_capsule3",
                "closest_point_triangle3",
                "distance_squared_point_triangle3",
                "closest_points_segment3_triangle3",
                "distance_squared_segment3_triangle3",
                "closest_points_sphere_triangle3",
                "distance_squared_sphere_triangle3",
                "closest_points_capsule3_triangle3",
                "distance_squared_capsule3_triangle3",
                "closest_points_triangle3_triangle3",
                "distance_squared_triangle3_triangle3",
                "closest_points_triangle3_aabb3",
                "distance_squared_triangle3_aabb3",
                "raycast_aabb3",
                "raycast_sphere",
                "raycast_capsule3",
                "raycast_triangle3",
                "sweep_segment3_aabb3",
                "sweep_segment3_triangle3",
                "sweep_triangle3_triangle3",
                "sweep_sphere_aabb3",
                "sweep_sphere_capsule3",
                "sweep_sphere_segment3",
                "sweep_sphere_sphere",
                "sweep_sphere_triangle3",
                "sweep_capsule3_sphere",
                "sweep_capsule3_triangle3",
                "sweep_capsule3_aabb3",
                "sweep_capsule3_segment3",
                "sweep_capsule3_capsule3",
                "segment3_overlaps_aabb3",
                "segment3_overlaps_triangle3",
                "sphere_overlaps_segment3",
                "sphere_overlaps_triangle3",
                "sphere_overlaps_sphere",
                "capsule3_overlaps_triangle3",
                "capsule3_overlaps_aabb3",
                "capsule3_overlaps_capsule3",
                "triangle3_overlaps_triangle3",
                "triangle3_overlaps_aabb3",
            }:
                ok = False
                continue
            if kind == "segment_intersects_segment":
                if not is_segment(case.get("segment")) or not is_segment(case.get("other_segment")):
                    ok = False
                if not isinstance(case.get("expected_overlap"), bool):
                    ok = False
            if kind == "circle_overlaps_segment":
                if not is_circle(case.get("circle")) or not is_segment(case.get("segment")):
                    ok = False
                if not isinstance(case.get("expected_overlap"), bool):
                    ok = False
            if kind == "capsule_overlaps_segment":
                if not is_capsule(case.get("capsule")) or not is_segment(case.get("segment")):
                    ok = False
                if not isinstance(case.get("expected_overlap"), bool):
                    ok = False
            if kind == "point_in_aabb":
                aabb = case.get("aabb")
                if not isinstance(aabb, dict) or not is_point2(aabb.get("min")) or not is_point2(aabb.get("max")):
                    ok = False
                if not is_point2(case.get("point")) or not isinstance(case.get("expected_contains"), bool):
                    ok = False
            if kind == "closest_point_segment":
                if not is_segment(case.get("segment")) or not is_point2(case.get("point")):
                    ok = False
                if not is_point2(case.get("expected_closest_point")):
                    ok = False
            if kind == "distance_squared_point_segment":
                if not is_segment(case.get("segment")) or not is_point2(case.get("point")):
                    ok = False
                if not isinstance(case.get("expected_distance_sq"), int):
                    ok = False
            if kind == "closest_point_circle":
                if not is_circle(case.get("circle")) or not is_point2(case.get("point")):
                    ok = False
                if not is_point2(case.get("expected_closest_point")):
                    ok = False
            if kind == "distance_squared_point_circle":
                if not is_circle(case.get("circle")) or not is_point2(case.get("point")):
                    ok = False
                if not isinstance(case.get("expected_distance_sq"), int):
                    ok = False
            if kind == "closest_point_capsule":
                if not is_capsule(case.get("capsule")) or not is_point2(case.get("point")):
                    ok = False
                if not is_point2(case.get("expected_closest_point")):
                    ok = False
            if kind == "distance_squared_point_capsule":
                if not is_capsule(case.get("capsule")) or not is_point2(case.get("point")):
                    ok = False
                if not isinstance(case.get("expected_distance_sq"), int):
                    ok = False
            if kind == "closest_point_polygon":
                if not is_polygon(case.get("polygon")) or not is_point2(case.get("point")):
                    ok = False
                if not is_point2(case.get("expected_closest_point")):
                    ok = False
            if kind == "distance_squared_point_polygon":
                if not is_polygon(case.get("polygon")) or not is_point2(case.get("point")):
                    ok = False
                if not isinstance(case.get("expected_distance_sq"), int):
                    ok = False
            if kind == "closest_point_oriented_box":
                if not is_oriented_box(case.get("oriented_box")) or not is_point2(case.get("point")):
                    ok = False
                if not is_point2(case.get("expected_closest_point")):
                    ok = False
            if kind == "distance_squared_point_oriented_box":
                if not is_oriented_box(case.get("oriented_box")) or not is_point2(case.get("point")):
                    ok = False
                if not isinstance(case.get("expected_distance_sq"), int):
                    ok = False
            if kind == "raycast_aabb":
                aabb = case.get("aabb")
                if not isinstance(aabb, dict) or not is_point2(aabb.get("min")) or not is_point2(aabb.get("max")):
                    ok = False
                ray = case.get("ray")
                if not isinstance(ray, dict) or not is_point2(ray.get("origin")) or not is_point2(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "raycast_circle":
                ray = case.get("ray")
                if not is_circle(case.get("circle")):
                    ok = False
                if not isinstance(ray, dict) or not is_point2(ray.get("origin")) or not is_point2(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "raycast_capsule":
                ray = case.get("ray")
                if not is_capsule(case.get("capsule")):
                    ok = False
                if not isinstance(ray, dict) or not is_point2(ray.get("origin")) or not is_point2(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "raycast_polygon":
                ray = case.get("ray")
                if not is_polygon(case.get("polygon")):
                    ok = False
                if not isinstance(ray, dict) or not is_point2(ray.get("origin")) or not is_point2(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "raycast_oriented_box":
                ray = case.get("ray")
                if not is_oriented_box(case.get("oriented_box")):
                    ok = False
                if not isinstance(ray, dict) or not is_point2(ray.get("origin")) or not is_point2(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "sweep_circle_aabb":
                aabb = case.get("aabb")
                ray = case.get("ray")
                if not is_circle(case.get("circle")) or not isinstance(aabb, dict) or not is_point2(aabb.get("min")) or not is_point2(aabb.get("max")):
                    ok = False
                if not isinstance(ray, dict) or not is_point2(ray.get("origin")) or not is_point2(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "sweep_circle_capsule":
                ray = case.get("ray")
                if not is_circle(case.get("circle")) or not is_capsule(case.get("capsule")):
                    ok = False
                if not isinstance(ray, dict) or not is_point2(ray.get("origin")) or not is_point2(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "sweep_circle_segment":
                ray = case.get("ray")
                if not is_circle(case.get("circle")) or not is_segment(case.get("segment")):
                    ok = False
                if not isinstance(ray, dict) or not is_point2(ray.get("origin")) or not is_point2(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "sweep_circle_circle":
                ray = case.get("ray")
                if not is_circle(case.get("circle")) or not is_circle(case.get("other_circle")):
                    ok = False
                if not isinstance(ray, dict) or not is_point2(ray.get("origin")) or not is_point2(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "sweep_circle_polygon":
                ray = case.get("ray")
                if not is_circle(case.get("circle")) or not is_polygon(case.get("polygon")):
                    ok = False
                if not isinstance(ray, dict) or not is_point2(ray.get("origin")) or not is_point2(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "sweep_circle_oriented_box":
                ray = case.get("ray")
                if not is_circle(case.get("circle")) or not is_oriented_box(case.get("oriented_box")):
                    ok = False
                if not isinstance(ray, dict) or not is_point2(ray.get("origin")) or not is_point2(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "sweep_capsule_aabb":
                aabb = case.get("aabb")
                ray = case.get("ray")
                if not is_capsule(case.get("capsule")) or not isinstance(aabb, dict) or not is_point2(aabb.get("min")) or not is_point2(aabb.get("max")):
                    ok = False
                if not isinstance(ray, dict) or not is_point2(ray.get("origin")) or not is_point2(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "sweep_capsule_circle":
                ray = case.get("ray")
                if not is_capsule(case.get("capsule")) or not is_circle(case.get("circle")):
                    ok = False
                if not isinstance(ray, dict) or not is_point2(ray.get("origin")) or not is_point2(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "sweep_capsule_segment":
                ray = case.get("ray")
                if not is_capsule(case.get("capsule")) or not is_segment(case.get("segment")):
                    ok = False
                if not isinstance(ray, dict) or not is_point2(ray.get("origin")) or not is_point2(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "sweep_capsule_capsule":
                ray = case.get("ray")
                if not is_capsule(case.get("capsule")) or not is_capsule(case.get("other_capsule")):
                    ok = False
                if not isinstance(ray, dict) or not is_point2(ray.get("origin")) or not is_point2(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "sweep_capsule_polygon":
                ray = case.get("ray")
                if not is_capsule(case.get("capsule")) or not is_polygon(case.get("polygon")):
                    ok = False
                if not isinstance(ray, dict) or not is_point2(ray.get("origin")) or not is_point2(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "sweep_capsule_oriented_box":
                ray = case.get("ray")
                if not is_capsule(case.get("capsule")) or not is_oriented_box(case.get("oriented_box")):
                    ok = False
                if not isinstance(ray, dict) or not is_point2(ray.get("origin")) or not is_point2(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "circle_overlaps_capsule":
                if not is_circle(case.get("circle")) or not is_capsule(case.get("capsule")):
                    ok = False
                if not isinstance(case.get("expected_overlap"), bool):
                    ok = False
            if kind == "circle_overlaps_polygon":
                if not is_circle(case.get("circle")) or not is_polygon(case.get("polygon")):
                    ok = False
                if not isinstance(case.get("expected_overlap"), bool):
                    ok = False
            if kind == "circle_overlaps_oriented_box":
                if not is_circle(case.get("circle")) or not is_oriented_box(case.get("oriented_box")):
                    ok = False
                if not isinstance(case.get("expected_overlap"), bool):
                    ok = False
            if kind == "capsule_overlaps_capsule":
                if not is_capsule(case.get("capsule")) or not is_capsule(case.get("other_capsule")):
                    ok = False
                if not isinstance(case.get("expected_overlap"), bool):
                    ok = False
            if kind == "capsule_overlaps_aabb":
                aabb = case.get("aabb")
                if not is_capsule(case.get("capsule")) or not isinstance(aabb, dict) or not is_point2(aabb.get("min")) or not is_point2(aabb.get("max")):
                    ok = False
                if not isinstance(case.get("expected_overlap"), bool):
                    ok = False
            if kind == "capsule_overlaps_polygon":
                if not is_capsule(case.get("capsule")) or not is_polygon(case.get("polygon")):
                    ok = False
                if not isinstance(case.get("expected_overlap"), bool):
                    ok = False
            if kind == "capsule_overlaps_oriented_box":
                if not is_capsule(case.get("capsule")) or not is_oriented_box(case.get("oriented_box")):
                    ok = False
                if not isinstance(case.get("expected_overlap"), bool):
                    ok = False
            if kind == "polygon_overlaps_aabb":
                aabb = case.get("aabb")
                if not is_polygon(case.get("polygon")) or not isinstance(aabb, dict) or not is_point2(aabb.get("min")) or not is_point2(aabb.get("max")):
                    ok = False
                if not isinstance(case.get("expected_overlap"), bool):
                    ok = False
            if kind == "polygon_overlaps_polygon":
                if not is_polygon(case.get("polygon")) or not is_polygon(case.get("other_polygon")):
                    ok = False
                if not isinstance(case.get("expected_overlap"), bool):
                    ok = False
            if kind == "oriented_box_overlaps_aabb":
                aabb = case.get("aabb")
                if not is_oriented_box(case.get("oriented_box")) or not isinstance(aabb, dict) or not is_point2(aabb.get("min")) or not is_point2(aabb.get("max")):
                    ok = False
                if not isinstance(case.get("expected_overlap"), bool):
                    ok = False
            if kind == "oriented_box_overlaps_oriented_box":
                if not is_oriented_box(case.get("oriented_box")) or not is_oriented_box(case.get("other_oriented_box")):
                    ok = False
                if not isinstance(case.get("expected_overlap"), bool):
                    ok = False
            if kind == "point_in_aabb3":
                aabb = case.get("aabb3")
                if not isinstance(aabb, dict) or not is_point3(aabb.get("min")) or not is_point3(aabb.get("max")):
                    ok = False
                if not is_point3(case.get("point3")) or not isinstance(case.get("expected_contains"), bool):
                    ok = False
            if kind == "closest_point_segment3":
                if not is_segment3(case.get("segment3")) or not is_point3(case.get("point3")):
                    ok = False
                if not is_point3(case.get("expected_closest_point3")):
                    ok = False
            if kind == "distance_squared_point_segment3":
                if not is_segment3(case.get("segment3")) or not is_point3(case.get("point3")):
                    ok = False
                if not isinstance(case.get("expected_distance_sq"), int):
                    ok = False
            if kind == "closest_point_sphere":
                if not is_sphere(case.get("sphere")) or not is_point3(case.get("point3")):
                    ok = False
                if not is_point3(case.get("expected_closest_point3")):
                    ok = False
            if kind == "distance_squared_point_sphere":
                if not is_sphere(case.get("sphere")) or not is_point3(case.get("point3")):
                    ok = False
                if not isinstance(case.get("expected_distance_sq"), int):
                    ok = False
            if kind == "closest_point_capsule3":
                if not is_capsule3(case.get("capsule3")) or not is_point3(case.get("point3")):
                    ok = False
                if not is_point3(case.get("expected_closest_point3")):
                    ok = False
            if kind == "distance_squared_point_capsule3":
                if not is_capsule3(case.get("capsule3")) or not is_point3(case.get("point3")):
                    ok = False
                if not isinstance(case.get("expected_distance_sq"), int):
                    ok = False
            if kind == "closest_point_triangle3":
                if not is_triangle3(case.get("triangle3")) or not is_point3(case.get("point3")):
                    ok = False
                if not is_point3(case.get("expected_closest_point3")):
                    ok = False
            if kind == "distance_squared_point_triangle3":
                if not is_triangle3(case.get("triangle3")) or not is_point3(case.get("point3")):
                    ok = False
                if not isinstance(case.get("expected_distance_sq"), int):
                    ok = False
            if kind == "closest_points_segment3_triangle3":
                if not is_segment3(case.get("segment3")) or not is_triangle3(case.get("triangle3")):
                    ok = False
                if not is_point3(case.get("expected_segment_point3")) or not is_point3(case.get("expected_triangle_point3")):
                    ok = False
            if kind == "distance_squared_segment3_triangle3":
                if not is_segment3(case.get("segment3")) or not is_triangle3(case.get("triangle3")):
                    ok = False
                if not isinstance(case.get("expected_distance_sq"), int):
                    ok = False
            if kind == "closest_points_sphere_triangle3":
                if not is_sphere(case.get("sphere")) or not is_triangle3(case.get("triangle3")):
                    ok = False
                if not is_point3(case.get("expected_sphere_point3")) or not is_point3(case.get("expected_triangle_point3")):
                    ok = False
            if kind == "distance_squared_sphere_triangle3":
                if not is_sphere(case.get("sphere")) or not is_triangle3(case.get("triangle3")):
                    ok = False
                if not isinstance(case.get("expected_distance_sq"), int):
                    ok = False
            if kind == "closest_points_capsule3_triangle3":
                if not is_capsule3(case.get("capsule3")) or not is_triangle3(case.get("triangle3")):
                    ok = False
                if not is_point3(case.get("expected_capsule_point3")) or not is_point3(case.get("expected_triangle_point3")):
                    ok = False
            if kind == "distance_squared_capsule3_triangle3":
                if not is_capsule3(case.get("capsule3")) or not is_triangle3(case.get("triangle3")):
                    ok = False
                if not isinstance(case.get("expected_distance_sq"), int):
                    ok = False
            if kind == "closest_points_triangle3_triangle3":
                if not is_triangle3(case.get("triangle3")) or not is_triangle3(case.get("other_triangle3")):
                    ok = False
                if not is_point3(case.get("expected_triangle_point3")) or not is_point3(case.get("expected_other_triangle_point3")):
                    ok = False
            if kind == "distance_squared_triangle3_triangle3":
                if not is_triangle3(case.get("triangle3")) or not is_triangle3(case.get("other_triangle3")):
                    ok = False
                if not isinstance(case.get("expected_distance_sq"), int):
                    ok = False
            if kind == "closest_points_triangle3_aabb3":
                aabb = case.get("aabb3")
                if not is_triangle3(case.get("triangle3")) or not isinstance(aabb, dict) or not is_point3(aabb.get("min")) or not is_point3(aabb.get("max")):
                    ok = False
                if not is_point3(case.get("expected_triangle_point3")) or not is_point3(case.get("expected_aabb_point3")):
                    ok = False
            if kind == "distance_squared_triangle3_aabb3":
                aabb = case.get("aabb3")
                if not is_triangle3(case.get("triangle3")) or not isinstance(aabb, dict) or not is_point3(aabb.get("min")) or not is_point3(aabb.get("max")):
                    ok = False
                if not isinstance(case.get("expected_distance_sq"), int):
                    ok = False
            if kind == "raycast_aabb3":
                aabb = case.get("aabb3")
                if not isinstance(aabb, dict) or not is_point3(aabb.get("min")) or not is_point3(aabb.get("max")):
                    ok = False
                ray = case.get("ray3")
                if not isinstance(ray, dict) or not is_point3(ray.get("origin")) or not is_point3(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "raycast_sphere":
                ray = case.get("ray3")
                if not is_sphere(case.get("sphere")):
                    ok = False
                if not isinstance(ray, dict) or not is_point3(ray.get("origin")) or not is_point3(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "raycast_capsule3":
                ray = case.get("ray3")
                if not is_capsule3(case.get("capsule3")):
                    ok = False
                if not isinstance(ray, dict) or not is_point3(ray.get("origin")) or not is_point3(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "raycast_triangle3":
                ray = case.get("ray3")
                if not is_triangle3(case.get("triangle3")):
                    ok = False
                if not isinstance(ray, dict) or not is_point3(ray.get("origin")) or not is_point3(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "sweep_segment3_aabb3":
                aabb = case.get("aabb3")
                ray = case.get("ray3")
                if not is_segment3(case.get("segment3")) or not isinstance(aabb, dict) or not is_point3(aabb.get("min")) or not is_point3(aabb.get("max")):
                    ok = False
                if not isinstance(ray, dict) or not is_point3(ray.get("origin")) or not is_point3(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "sweep_segment3_triangle3":
                ray = case.get("ray3")
                if not is_segment3(case.get("segment3")) or not is_triangle3(case.get("triangle3")):
                    ok = False
                if not isinstance(ray, dict) or not is_point3(ray.get("origin")) or not is_point3(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "sweep_triangle3_triangle3":
                ray = case.get("ray3")
                if not is_triangle3(case.get("triangle3")) or not is_triangle3(case.get("other_triangle3")):
                    ok = False
                if not isinstance(ray, dict) or not is_point3(ray.get("origin")) or not is_point3(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "sweep_sphere_aabb3":
                aabb = case.get("aabb3")
                ray = case.get("ray3")
                if not is_sphere(case.get("sphere")) or not isinstance(aabb, dict) or not is_point3(aabb.get("min")) or not is_point3(aabb.get("max")):
                    ok = False
                if not isinstance(ray, dict) or not is_point3(ray.get("origin")) or not is_point3(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "sweep_sphere_capsule3":
                ray = case.get("ray3")
                if not is_sphere(case.get("sphere")) or not is_capsule3(case.get("capsule3")):
                    ok = False
                if not isinstance(ray, dict) or not is_point3(ray.get("origin")) or not is_point3(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "sweep_sphere_segment3":
                ray = case.get("ray3")
                if not is_sphere(case.get("sphere")) or not is_segment3(case.get("segment3")):
                    ok = False
                if not isinstance(ray, dict) or not is_point3(ray.get("origin")) or not is_point3(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "sweep_sphere_sphere":
                ray = case.get("ray3")
                if not is_sphere(case.get("sphere")) or not is_sphere(case.get("other_sphere")):
                    ok = False
                if not isinstance(ray, dict) or not is_point3(ray.get("origin")) or not is_point3(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "sweep_sphere_triangle3":
                ray = case.get("ray3")
                if not is_sphere(case.get("sphere")) or not is_triangle3(case.get("triangle3")):
                    ok = False
                if not isinstance(ray, dict) or not is_point3(ray.get("origin")) or not is_point3(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "sweep_capsule3_sphere":
                ray = case.get("ray3")
                if not is_capsule3(case.get("capsule3")) or not is_sphere(case.get("sphere")):
                    ok = False
                if not isinstance(ray, dict) or not is_point3(ray.get("origin")) or not is_point3(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "sweep_capsule3_triangle3":
                ray = case.get("ray3")
                if not is_capsule3(case.get("capsule3")) or not is_triangle3(case.get("triangle3")):
                    ok = False
                if not isinstance(ray, dict) or not is_point3(ray.get("origin")) or not is_point3(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "sweep_capsule3_aabb3":
                aabb = case.get("aabb3")
                ray = case.get("ray3")
                if not is_capsule3(case.get("capsule3")) or not isinstance(aabb, dict) or not is_point3(aabb.get("min")) or not is_point3(aabb.get("max")):
                    ok = False
                if not isinstance(ray, dict) or not is_point3(ray.get("origin")) or not is_point3(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "sweep_capsule3_segment3":
                ray = case.get("ray3")
                if not is_capsule3(case.get("capsule3")) or not is_segment3(case.get("segment3")):
                    ok = False
                if not isinstance(ray, dict) or not is_point3(ray.get("origin")) or not is_point3(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "sweep_capsule3_capsule3":
                ray = case.get("ray3")
                if not is_capsule3(case.get("capsule3")) or not is_capsule3(case.get("other_capsule3")):
                    ok = False
                if not isinstance(ray, dict) or not is_point3(ray.get("origin")) or not is_point3(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
                    ok = False
            if kind == "segment3_overlaps_aabb3":
                aabb = case.get("aabb3")
                if not is_segment3(case.get("segment3")) or not isinstance(aabb, dict) or not is_point3(aabb.get("min")) or not is_point3(aabb.get("max")):
                    ok = False
                if not isinstance(case.get("expected_overlap"), bool):
                    ok = False
            if kind == "segment3_overlaps_triangle3":
                if not is_segment3(case.get("segment3")) or not is_triangle3(case.get("triangle3")):
                    ok = False
                if not isinstance(case.get("expected_overlap"), bool):
                    ok = False
            if kind == "sphere_overlaps_segment3":
                if not is_sphere(case.get("sphere")) or not is_segment3(case.get("segment3")):
                    ok = False
                if not isinstance(case.get("expected_overlap"), bool):
                    ok = False
            if kind == "sphere_overlaps_triangle3":
                if not is_sphere(case.get("sphere")) or not is_triangle3(case.get("triangle3")):
                    ok = False
                if not isinstance(case.get("expected_overlap"), bool):
                    ok = False
            if kind == "sphere_overlaps_sphere":
                if not is_sphere(case.get("sphere")) or not is_sphere(case.get("other_sphere")):
                    ok = False
                if not isinstance(case.get("expected_overlap"), bool):
                    ok = False
            if kind == "capsule3_overlaps_triangle3":
                if not is_capsule3(case.get("capsule3")) or not is_triangle3(case.get("triangle3")):
                    ok = False
                if not isinstance(case.get("expected_overlap"), bool):
                    ok = False
            if kind == "capsule3_overlaps_aabb3":
                aabb = case.get("aabb3")
                if not is_capsule3(case.get("capsule3")) or not isinstance(aabb, dict) or not is_point3(aabb.get("min")) or not is_point3(aabb.get("max")):
                    ok = False
                if not isinstance(case.get("expected_overlap"), bool):
                    ok = False
            if kind == "capsule3_overlaps_capsule3":
                if not is_capsule3(case.get("capsule3")) or not is_capsule3(case.get("other_capsule3")):
                    ok = False
                if not isinstance(case.get("expected_overlap"), bool):
                    ok = False
            if kind == "triangle3_overlaps_triangle3":
                if not is_triangle3(case.get("triangle3")) or not is_triangle3(case.get("other_triangle3")):
                    ok = False
                if not isinstance(case.get("expected_overlap"), bool):
                    ok = False
            if kind == "triangle3_overlaps_aabb3":
                aabb = case.get("aabb3")
                if not is_triangle3(case.get("triangle3")) or not isinstance(aabb, dict) or not is_point3(aabb.get("min")) or not is_point3(aabb.get("max")):
                    ok = False
                if not isinstance(case.get("expected_overlap"), bool):
                    ok = False
    if expect_valid and not ok:
        fail("query-cases.valid.json did not satisfy the validator")
    if not expect_valid and ok:
        fail("query-cases.invalid.json unexpectedly passed validation")

def validate_grid_config(payload, expect_valid: bool) -> None:
    ok = True
    if not isinstance(payload, dict):
        ok = False
    else:
        for key in ("width", "height", "cell_size", "max_per_cell"):
            value = payload.get(key)
            if not isinstance(value, int) or value < 1:
                ok = False
        if "depth" in payload:
            value = payload.get("depth")
            if not isinstance(value, int) or value < 1:
                ok = False
    if expect_valid and not ok:
        fail("grid-config.valid.json did not satisfy the validator")
    if not expect_valid and ok:
        fail("grid-config.invalid.json unexpectedly passed validation")

def main() -> None:
    validate_query_cases(load("query-cases.valid.json"), expect_valid=True)
    validate_query_cases(load("query-cases.invalid.json"), expect_valid=False)
    validate_grid_config(load("grid-config.valid.json"), expect_valid=True)
    validate_grid_config(load("grid-config.invalid.json"), expect_valid=False)
    print("[contract-validation] all bumpbox fixtures validated")

if __name__ == "__main__":
    main()
