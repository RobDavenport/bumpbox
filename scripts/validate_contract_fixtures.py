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
            if kind not in {"point_in_aabb", "raycast_aabb"}:
                ok = False
                continue
            aabb = case.get("aabb")
            if not isinstance(aabb, dict) or not is_point2(aabb.get("min")) or not is_point2(aabb.get("max")):
                ok = False
            if kind == "point_in_aabb":
                if not is_point2(case.get("point")) or not isinstance(case.get("expected_contains"), bool):
                    ok = False
            if kind == "raycast_aabb":
                ray = case.get("ray")
                if not isinstance(ray, dict) or not is_point2(ray.get("origin")) or not is_point2(ray.get("dir")) or not isinstance(case.get("expected_hit"), bool):
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
