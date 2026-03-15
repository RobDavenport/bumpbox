const wasmStatus = document.getElementById("wasm-status");
const wasmDetail = document.getElementById("wasm-detail");
const dimensionRow = document.getElementById("dimension-row");
const modeRow = document.getElementById("mode-row");
const raycastTargetGroup = document.getElementById("raycast-target-group");
const rayTargetRow = document.getElementById("ray-target-row");
const viewPresetGroup = document.getElementById("view-preset-group");
const viewPresetRow = document.getElementById("view-preset-row");
const layerRow = document.getElementById("layer-row");
const resetButton = document.getElementById("reset-button");

const canvas = document.getElementById("scene-canvas");
const ctx = canvas.getContext("2d");

const modeTitle = document.getElementById("mode-title");
const modeSummary = document.getElementById("mode-summary");
const modeDescription = document.getElementById("mode-description");
const dragHint = document.getElementById("drag-hint");
const handleList = document.getElementById("handle-list");
const scenarioList = document.getElementById("scenario-list");
const scenarioPrev = document.getElementById("scenario-prev");
const scenarioReplay = document.getElementById("scenario-replay");
const scenarioNext = document.getElementById("scenario-next");
const scenarioPlay = document.getElementById("scenario-play");
const scenarioStatus = document.getElementById("scenario-status");
const scenarioCaption = document.getElementById("scenario-caption");
const selectionLabel = document.getElementById("selection-label");
const selectionCoords = document.getElementById("selection-coords");
const depthBlock = document.getElementById("depth-block");
const depthLabel = document.getElementById("depth-label");
const depthRange = document.getElementById("depth-range");
const depthValue = document.getElementById("depth-value");
const liveFlags = document.getElementById("live-flags");
const detailTitle = document.getElementById("detail-title");
const detailBody = document.getElementById("detail-body");
const insightList = document.getElementById("insight-list");
const candidateBlock = document.getElementById("candidate-block");
const candidateIds = document.getElementById("candidate-ids");

const colors = {
  bg2d: "#fff8ef",
  bg3d: "#f8fafc",
  grid: "rgba(100, 116, 139, 0.14)",
  staticStroke: "#475569",
  staticFill: "rgba(148, 163, 184, 0.12)",
  dynamic: "#d97706",
  dynamicFill: "rgba(245, 158, 11, 0.18)",
  sensor: "#0f766e",
  sensorFill: "rgba(45, 212, 191, 0.12)",
  hit: "#dc2626",
  handle: "#2563eb",
  handleFill: "rgba(37, 99, 235, 0.16)",
  polygon: "#1d4ed8",
  polygonFill: "rgba(29, 78, 216, 0.08)",
  obb: "#7c3aed",
  obbFill: "rgba(124, 58, 237, 0.08)",
  triangle: "#0f766e",
  triangleFill: "rgba(15, 118, 110, 0.12)",
  ray: "#0f172a",
  rayRound: "#d97706",
  rayCapsule: "#0f766e",
  rayPolygon: "#1d4ed8",
  rayObb: "#7c3aed",
  rayTriangle: "#0ea5a4",
  text: "#0f172a",
};

const HANDLE_IDS = {
  "2d": {
    probe: 0,
    circle: 1,
    aabb: 2,
    capsule: 3,
    oriented_box: 4,
    polygon: 5,
    sensor: 6,
    aabb_ray: 7,
    circle_ray: 8,
    capsule_ray: 9,
    polygon_ray: 10,
    oriented_box_ray: 11,
  },
  "3d": {
    probe: 0,
    sphere: 1,
    aabb: 2,
    capsule: 3,
    triangle: 4,
    sensor: 5,
    aabb_ray: 6,
    sphere_ray: 7,
    capsule_ray: 8,
    triangle_ray: 9,
  },
};

const HANDLE_LABELS = {
  probe: "Probe",
  circle: "Circle",
  sphere: "Sphere",
  aabb: "AABB",
  capsule: "Capsule",
  oriented_box: "OBB",
  polygon: "Polygon",
  triangle: "Triangle",
  sensor: "Sensor",
  aabb_ray: "AABB Ray",
  circle_ray: "Circle Ray",
  sphere_ray: "Sphere Ray",
  capsule_ray: "Capsule Ray",
  polygon_ray: "Polygon Ray",
  oriented_box_ray: "OBB Ray",
  triangle_ray: "Triangle Ray",
};

const CANDIDATE_LABELS = {
  "2d": { 11: "AABB", 22: "Capsule", 33: "OBB", 44: "Polygon" },
  "3d": { 101: "AABB", 202: "Capsule", 303: "Triangle" },
};

const RAY_TARGETS = {
  "2d": {
    aabb: { label: "AABB", rayKey: "aabb_ray", flag: "ray_hits_aabb", color: colors.ray },
    circle: { label: "Circle", rayKey: "circle_ray", flag: "ray_hits_circle", color: colors.rayRound },
    capsule: { label: "Capsule", rayKey: "capsule_ray", flag: "ray_hits_capsule", color: colors.rayCapsule },
    polygon: { label: "Polygon", rayKey: "polygon_ray", flag: "ray_hits_polygon", color: colors.rayPolygon },
    oriented_box: { label: "OBB", rayKey: "oriented_box_ray", flag: "ray_hits_oriented_box", color: colors.rayObb },
  },
  "3d": {
    aabb: { label: "AABB", rayKey: "aabb_ray", flag: "ray_hits_aabb", color: colors.ray },
    sphere: { label: "Sphere", rayKey: "sphere_ray", flag: "ray_hits_sphere", color: colors.rayRound },
    capsule: { label: "Capsule", rayKey: "capsule_ray", flag: "ray_hits_capsule", color: colors.rayCapsule },
    triangle: { label: "Triangle", rayKey: "triangle_ray", flag: "ray_hits_triangle", color: colors.rayTriangle },
  },
};

const LESSONS = {
  "2d": {
    overlap: {
      title: "2D Overlap Lab",
      description:
        "Drag the circle or any target shape. The circle overlap tests recompute immediately against AABB, capsule, polygon, and oriented box geometry.",
      handles: () => ["circle", "aabb", "capsule", "polygon", "oriented_box"],
      flags: [
        ["Circle vs AABB", "circle_hits_aabb", (state) => state.flags.circle_hits_aabb],
        ["Circle vs Capsule", "circle_hits_capsule", (state) => state.flags.circle_hits_capsule],
        ["Circle vs Polygon", "circle_hits_polygon", (state) => state.flags.circle_hits_polygon],
        ["Circle vs OBB", "circle_hits_oriented_box", (state) => state.flags.circle_hits_oriented_box],
      ],
      summary: (state) =>
        `${[
          state.flags.circle_hits_aabb,
          state.flags.circle_hits_capsule,
          state.flags.circle_hits_polygon,
          state.flags.circle_hits_oriented_box,
        ].filter(Boolean).length}/4 overlaps active`,
      insights: (state) => {
        const hits = [];
        if (state.flags.circle_hits_aabb) hits.push("The circle is penetrating the AABB footprint.");
        if (state.flags.circle_hits_capsule) hits.push("The circle is within capsule radius of the segment lane.");
        if (state.flags.circle_hits_polygon) hits.push("The polygon boundary or interior is intersecting the circle.");
        if (state.flags.circle_hits_oriented_box) hits.push("The rotated box is close enough to produce a positive overlap.");
        return hits.length ? hits : ["No overlap is active. Pull a target into the circle or drag the circle into one of the static shapes."];
      },
    },
    containment: {
      title: "2D Containment Lab",
      description:
        "Drag the probe point through the shapes, or move the shapes around the probe. This isolates the boolean point-inside tests without ray or broadphase noise.",
      handles: () => ["probe", "aabb", "capsule", "polygon", "oriented_box"],
      flags: [
        ["Probe in AABB", "probe_in_aabb", (state) => state.flags.probe_in_aabb],
        ["Probe in Capsule", "probe_in_capsule", (state) => state.flags.probe_in_capsule],
        ["Probe in Polygon", "probe_in_polygon", (state) => state.flags.probe_in_polygon],
        ["Probe in OBB", "probe_in_oriented_box", (state) => state.flags.probe_in_oriented_box],
      ],
      summary: (state) =>
        `${[
          state.flags.probe_in_aabb,
          state.flags.probe_in_capsule,
          state.flags.probe_in_polygon,
          state.flags.probe_in_oriented_box,
        ].filter(Boolean).length}/4 containments active`,
      insights: (state) => {
        const hits = [];
        if (state.flags.probe_in_aabb) hits.push("The probe is inside the AABB extents.");
        if (state.flags.probe_in_capsule) hits.push("The probe is within the capsule radius from its segment.");
        if (state.flags.probe_in_polygon) hits.push("The probe lies on or inside the convex polygon.");
        if (state.flags.probe_in_oriented_box) hits.push("The probe is inside the rotated local box bounds.");
        return hits.length ? hits : ["The probe is outside every tested shape. Drag it across an edge to watch each predicate flip."];
      },
    },
  },
};

const SCENARIOS = {
  "2d": {
    overlap: [
      {
        label: "AABB Hit",
        description: "Moves the circle into the box footprint so the simplest overlap predicate flips on immediately.",
        moves: [{ handle: "circle", point: [30, 21] }],
      },
      {
        label: "Capsule Hit",
        description: "Slides the circle onto the capsule lane to show round-vs-round distance overlap.",
        moves: [{ handle: "circle", point: [18, 41] }],
      },
      {
        label: "Clear",
        description: "Pulls the circle back into empty space so every overlap flag returns false.",
        moves: [{ handle: "circle", point: [10, 10] }],
      },
    ],
    containment: [
      {
        label: "Inside Polygon",
        description: "Places the probe in the polygon interior so you can isolate the polygon containment result.",
        moves: [{ handle: "probe", point: [47, 43] }],
      },
      {
        label: "Inside Capsule",
        description: "Drops the probe onto the capsule lane to show the round containment test without other hits.",
        moves: [{ handle: "probe", point: [18, 41] }],
      },
      {
        label: "Outside All",
        description: "Resets the probe into empty space so every containment predicate is false.",
        moves: [{ handle: "probe", point: [8, 8] }],
      },
    ],
    raycast: [
      {
        label: "Circle Hit",
        rayTarget: "circle",
        description: "Repositions the circle ray so the first hit lands on the round target with a clear contact marker.",
        moves: [{ handle: "circle_ray", point: [4, 16] }],
      },
      {
        label: "Polygon Hit",
        rayTarget: "polygon",
        description: "Aims the polygon ray through the convex shape so you can read the first impact point cleanly.",
        moves: [{ handle: "polygon_ray", point: [4, 44] }],
      },
      {
        label: "OBB Miss",
        rayTarget: "oriented_box",
        description: "Offsets the OBB ray below the box to show the miss case without changing the target geometry.",
        moves: [{ handle: "oriented_box_ray", point: [4, 6] }],
      },
    ],
    broadphase: [
      {
        label: "AABB Only",
        description: "Slides the sensor onto the AABB cells so the broadphase returns a single deterministic candidate.",
        moves: [{ handle: "sensor", point: [30, 21] }],
      },
      {
        label: "Polygon Zone",
        description: "Moves the sensor into the polygon area to show a different broadphase candidate region.",
        moves: [{ handle: "sensor", point: [49, 44] }],
      },
      {
        label: "Empty Space",
        description: "Places the sensor away from all occupied cells so the candidate list is empty.",
        moves: [{ handle: "sensor", point: [8, 8] }],
      },
    ],
  },
  "3d": {
    overlap: [
      {
        label: "Triangle Hit",
        description: "Moves the sphere onto the triangle region so face, edge, or vertex contact becomes visible.",
        moves: [{ handle: "sphere", point: [24, 22, 20] }],
      },
      {
        label: "Capsule Hit",
        description: "Aligns the sphere with the capsule lane to show 3D round overlap at a readable depth.",
        moves: [{ handle: "sphere", point: [20, 34, 18] }],
      },
      {
        label: "Clear",
        description: "Pulls the sphere out into empty space so the 3D overlap readout clears completely.",
        moves: [{ handle: "sphere", point: [8, 8, 8] }],
      },
    ],
    containment: [
      {
        label: "On Triangle",
        description: "Places the probe directly on the triangle so the point-on-triangle predicate turns true.",
        moves: [{ handle: "probe", point: [24, 22, 20] }],
      },
      {
        label: "Inside Capsule",
        description: "Moves the probe into the capsule volume to isolate the capsule containment check.",
        moves: [{ handle: "probe", point: [20, 34, 18] }],
      },
      {
        label: "Outside All",
        description: "Resets the probe away from every 3D target so the containment readout is fully false.",
        moves: [{ handle: "probe", point: [8, 8, 8] }],
      },
    ],
    raycast: [
      {
        label: "Sphere Hit",
        rayTarget: "sphere",
        description: "Aims the sphere ray through the round target so the 3D hit point and normal are easy to read.",
        moves: [{ handle: "sphere_ray", point: [4, 16, 16] }],
      },
      {
        label: "Triangle Hit",
        rayTarget: "triangle",
        description: "Places the triangle ray on a clean intercept path so the triangle hit marker is unambiguous.",
        moves: [{ handle: "triangle_ray", point: [4, 22, 20] }],
      },
      {
        label: "AABB Miss",
        rayTarget: "aabb",
        description: "Offsets the AABB ray into empty space to show the miss case in 3D.",
        moves: [{ handle: "aabb_ray", point: [4, 4, 4] }],
      },
    ],
    broadphase: [
      {
        label: "AABB Only",
        description: "Slides the 3D sensor into the AABB cells so the broadphase returns just that object.",
        moves: [{ handle: "sensor", point: [26, 17, 17] }],
      },
      {
        label: "Triangle Zone",
        description: "Moves the sensor through the triangle's occupied cells to surface that broadphase candidate.",
        moves: [{ handle: "sensor", point: [24, 22, 20] }],
      },
      {
        label: "Empty Space",
        description: "Resets the 3D sensor away from occupied cells so the candidate set is empty.",
        moves: [{ handle: "sensor", point: [53, 53, 53] }],
      },
    ],
  },
};

const sandboxes = { "2d": null, "3d": null };
const states = { "2d": null, "3d": null };
const VIEW_PRESETS = {
  default: { label: "Default", scale: 8.4, skew: 0.36, centerX: 0.51, centerY: 0.72 },
  wide: { label: "Wide", scale: 7.2, skew: 0.28, centerX: 0.53, centerY: 0.74 },
  top: { label: "Top Bias", scale: 8.8, skew: 0.18, centerX: 0.52, centerY: 0.77 },
};

let wasmReady = false;
let activeDimension = "2d";
let activeModes = { "2d": "overlap", "3d": "overlap" };
let activeRayTargets = { "2d": "aabb", "3d": "aabb" };
let draggingHandle = null;
let activeHandleByDimension = { "2d": "circle", "3d": "sphere" };
let activeDepthHandle = "sphere";
let activeScenarioByKey = {};
let activeViewPreset = "default";
let layerVisibility = { handles: true, callouts: true, axes: true };
let scenarioAnimationFrame = 0;
let scenarioAnimationToken = 0;
let scenarioTourTimer = 0;
let scenarioTourPlaying = false;
const SCENARIO_TOUR_STEP_MS = 2600;

const scenarioKey = () => `${activeDimension}:${activeMode()}`;
const projection = () => VIEW_PRESETS[activeViewPreset];
const recommendedViewPreset = (dimension = activeDimension, mode = activeModes[dimension]) =>
  dimension === "3d" ? LESSONS["3d"][mode]?.viewPreset ?? "default" : "default";
const applyRecommendedViewPreset = (dimension = activeDimension) => {
  if (dimension !== "3d") return;
  activeViewPreset = recommendedViewPreset(dimension);
};
const activeScenarios = () => SCENARIOS[activeDimension][activeMode()] ?? [];
const activeScenarioIndex = () => activeScenarioByKey[scenarioKey()];
const activeScenario = () => {
  const index = activeScenarioIndex();
  return Number.isInteger(index) ? activeScenarios()[index] ?? null : null;
};
const clearScenarioTourTimer = () => {
  if (!scenarioTourTimer) return;
  clearTimeout(scenarioTourTimer);
  scenarioTourTimer = 0;
};
const stopScenarioTour = () => {
  clearScenarioTourTimer();
  scenarioTourPlaying = false;
};

Object.assign(LESSONS["2d"], {
  raycast: {
    title: "2D Raycast Lab",
    description:
      "Choose a target, then drag the ray origin or the target itself. The horizontal ray shows the first hit point and normal returned by bumpbox.",
    handles: (_, rayTarget) => [
      RAY_TARGETS["2d"][rayTarget].rayKey,
      "aabb",
      "circle",
      "capsule",
      "polygon",
      "oriented_box",
    ],
    flags: [
      ["AABB ray hit", "ray_hits_aabb", (state) => state.flags.ray_hits_aabb],
      ["Circle ray hit", "ray_hits_circle", (state) => state.flags.ray_hits_circle],
      ["Capsule ray hit", "ray_hits_capsule", (state) => state.flags.ray_hits_capsule],
      ["Polygon ray hit", "ray_hits_polygon", (state) => state.flags.ray_hits_polygon],
      ["OBB ray hit", "ray_hits_oriented_box", (state) => state.flags.ray_hits_oriented_box],
    ],
    summary: (state, rayTarget) =>
      `${RAY_TARGETS["2d"][rayTarget].label}: ${
        state.flags[RAY_TARGETS["2d"][rayTarget].flag] ? "hit" : "miss"
      }`,
    insights: (state, rayTarget) => {
      const spec = RAY_TARGETS["2d"][rayTarget];
      const ray = state[spec.rayKey];
      if (!state.flags[spec.flag]) {
        return [
          `${spec.label} is not intersected. Move the ray origin up or down until the horizontal line crosses the target.`,
        ];
      }
      return [
        `${spec.label} is the active first hit.`,
        `Contact point ${formatPoint(ray.hit_point)} with normal ${formatPoint(ray.hit_normal)}.`,
      ];
    },
  },
  broadphase: {
    title: "2D Broadphase Lab",
    description:
      "Drag the dashed sensor AABB across the scene. The candidate IDs update in deterministic sorted order from the uniform grid before any narrow-phase overlap test runs.",
    handles: () => ["sensor", "aabb", "capsule", "polygon", "oriented_box"],
    flags: [
      ["AABB candidate", null, (state) => state.candidate_ids.includes(11)],
      ["Capsule candidate", null, (state) => state.candidate_ids.includes(22)],
      ["OBB candidate", null, (state) => state.candidate_ids.includes(33)],
      ["Polygon candidate", null, (state) => state.candidate_ids.includes(44)],
    ],
    summary: (state) => `${state.candidate_ids.length} grid candidates`,
    insights: (state) =>
      state.candidate_ids.length
        ? [
            `Candidates are emitted as ${state.candidate_ids.map((id) => `#${id}`).join(", ")}.`,
            "This list is broadphase only. It tells you which objects are worth testing more precisely next.",
          ]
        : [
            "The sensor is in empty space. Slide it over one of the shapes to pull that object into the candidate set.",
          ],
  },
});

LESSONS["3d"] = {
  overlap: {
    title: "3D Overlap Lab",
    viewPreset: "wide",
    description:
      "Drag the sphere or any 3D target. The sphere overlap tests recompute against the AABB, capsule, and triangle in the projected isometric view.",
    handles: () => ["sphere", "aabb", "capsule", "triangle"],
    flags: [
      ["Sphere vs AABB", "sphere_hits_aabb", (state) => state.flags.sphere_hits_aabb],
      ["Sphere vs Capsule", "sphere_hits_capsule", (state) => state.flags.sphere_hits_capsule],
      ["Sphere vs Triangle", "sphere_hits_triangle", (state) => state.flags.sphere_hits_triangle],
    ],
    summary: (state) =>
      `${[
        state.flags.sphere_hits_aabb,
        state.flags.sphere_hits_capsule,
        state.flags.sphere_hits_triangle,
      ].filter(Boolean).length}/3 overlaps active`,
    insights: (state) => {
      const hits = [];
      if (state.flags.sphere_hits_aabb) hits.push("The sphere volume intersects the AABB.");
      if (state.flags.sphere_hits_capsule) hits.push("The sphere is close enough to hit the capsule sweep volume.");
      if (state.flags.sphere_hits_triangle) hits.push("The sphere reaches the triangle face, edge, or vertex.");
      return hits.length
        ? hits
        : [
            "No 3D overlap is active. Drag the sphere toward the box, capsule, or triangle, then use the depth slider to align the z plane if needed.",
          ];
    },
  },
  containment: {
    title: "3D Containment Lab",
    viewPreset: "top",
    description:
      "Drag the probe point through the 3D scene. This isolates point-in-AABB, point-in-capsule, and point-on-triangle semantics, with a guide line to the triangle closest point.",
    handles: () => ["probe", "aabb", "capsule", "triangle"],
    flags: [
      ["Probe in AABB", "probe_in_aabb", (state) => state.flags.probe_in_aabb],
      ["Probe in Capsule", "probe_in_capsule", (state) => state.flags.probe_in_capsule],
      ["Probe on Triangle", "probe_on_triangle", (state) => state.flags.probe_on_triangle],
    ],
    summary: (state) =>
      `${[
        state.flags.probe_in_aabb,
        state.flags.probe_in_capsule,
        state.flags.probe_on_triangle,
      ].filter(Boolean).length}/3 containments active`,
    insights: (state) =>
      state.flags.probe_on_triangle
        ? ["The probe is on the triangle surface exactly.", `Closest point ${formatPoint(state.triangle_closest_point)}.`]
        : [
            "The dashed guide shows the nearest point on the triangle from the current probe position.",
            `Closest point ${formatPoint(state.triangle_closest_point)}.`,
          ],
  },
  raycast: {
    title: "3D Raycast Lab",
    viewPreset: "default",
    description:
      "Choose a 3D target, drag its ray origin in the projected view, and use the depth slider when you need to move the origin along z. The first hit marker and normal update live.",
    handles: (_, rayTarget) => [RAY_TARGETS["3d"][rayTarget].rayKey, "aabb", "sphere", "capsule", "triangle"],
    flags: [
      ["AABB ray hit", "ray_hits_aabb", (state) => state.flags.ray_hits_aabb],
      ["Sphere ray hit", "ray_hits_sphere", (state) => state.flags.ray_hits_sphere],
      ["Capsule ray hit", "ray_hits_capsule", (state) => state.flags.ray_hits_capsule],
      ["Triangle ray hit", "ray_hits_triangle", (state) => state.flags.ray_hits_triangle],
    ],
    summary: (state, rayTarget) =>
      `${RAY_TARGETS["3d"][rayTarget].label}: ${
        state.flags[RAY_TARGETS["3d"][rayTarget].flag] ? "hit" : "miss"
      }`,
    insights: (state, rayTarget) => {
      const spec = RAY_TARGETS["3d"][rayTarget];
      const ray = state[spec.rayKey];
      if (!state.flags[spec.flag]) {
        return [
          `${spec.label} is not intersected. Drag the ray in screen space, then tune z with the depth slider until the ray crosses the target.`,
        ];
      }
      return [
        `${spec.label} is the active first hit.`,
        `Contact point ${formatPoint(ray.hit_point)} with normal ${formatPoint(ray.hit_normal)}.`,
      ];
    },
  },
  broadphase: {
    title: "3D Broadphase Lab",
    viewPreset: "wide",
    description:
      "Drag the 3D sensor box through space. The candidate IDs come from the deterministic UniformGrid3 broadphase before any narrow-phase test is applied.",
    handles: () => ["sensor", "aabb", "capsule", "triangle"],
    flags: [
      ["AABB candidate", null, (state) => state.candidate_ids.includes(101)],
      ["Capsule candidate", null, (state) => state.candidate_ids.includes(202)],
      ["Triangle candidate", null, (state) => state.candidate_ids.includes(303)],
    ],
    summary: (state) => `${state.candidate_ids.length} grid candidates`,
    insights: (state) =>
      state.candidate_ids.length
        ? [
            `Candidates are emitted as ${state.candidate_ids.map((id) => `#${id}`).join(", ")}.`,
            "This is the coarse 3D query set that a runtime would feed into narrower tests next.",
          ]
        : [
            "The sensor is not touching any occupied cells. Slide it across x/y or change z to pull shapes into the candidate set.",
          ],
  },
};

const setStatus = (text, detail = "", error = false) => {
  wasmStatus.textContent = text;
  wasmDetail.textContent = detail;
  wasmStatus.classList.toggle("error", error);
};

const activeState = () => states[activeDimension];
const activeSandbox = () => sandboxes[activeDimension];
const activeMode = () => activeModes[activeDimension];
const activeRayTarget = () => activeRayTargets[activeDimension];

const toCanvas2D = (point) => {
  const pad = 32;
  const scaleX = (canvas.width - pad * 2) / 64;
  const scaleY = (canvas.height - pad * 2) / 64;
  return [pad + point[0] * scaleX, canvas.height - pad - point[1] * scaleY];
};

const fromCanvas2D = (x, y) => {
  const pad = 32;
  const scaleX = (canvas.width - pad * 2) / 64;
  const scaleY = (canvas.height - pad * 2) / 64;
  return [
    Math.max(0, Math.min(64, (x - pad) / scaleX)),
    Math.max(0, Math.min(64, (canvas.height - pad - y) / scaleY)),
  ];
};

const isoProject = (point) => {
  const { scale, skew, centerX, centerY } = projection();
  const originX = canvas.width * centerX;
  const originY = canvas.height * centerY;
  const [x, y, z] = point;
  return [originX + (x - z) * scale, originY - y * scale + (x + z) * scale * skew];
};

const fromCanvas3D = (x, y, z) => {
  const { scale, skew, centerX, centerY } = projection();
  const originX = canvas.width * centerX;
  const originY = canvas.height * centerY;
  const xMinusZ = (x - originX) / scale;
  const worldX = xMinusZ + z;
  const worldY = (originY - y) / scale + skew * (worldX + z);
  return [Math.max(0, Math.min(64, worldX)), Math.max(0, Math.min(64, worldY)), z];
};

const eventToCanvas = (event) => {
  const rect = canvas.getBoundingClientRect();
  return [
    ((event.clientX - rect.left) / rect.width) * canvas.width,
    ((event.clientY - rect.top) / rect.height) * canvas.height,
  ];
};

const drawGrid = (step = 40) => {
  ctx.strokeStyle = colors.grid;
  ctx.lineWidth = 1;
  for (let x = step; x < canvas.width; x += step) {
    ctx.beginPath();
    ctx.moveTo(x, 0);
    ctx.lineTo(x, canvas.height);
    ctx.stroke();
  }
  for (let y = step; y < canvas.height; y += step) {
    ctx.beginPath();
    ctx.moveTo(0, y);
    ctx.lineTo(canvas.width, y);
    ctx.stroke();
  }
};

const drawCapsule2D = (capsule, stroke, fill) => {
  const [sx, sy] = toCanvas2D(capsule.start);
  const [ex, ey] = toCanvas2D(capsule.end);
  const scale = (canvas.width - 64) / 64;
  const radius = capsule.radius * scale;
  const angle = Math.atan2(ey - sy, ex - sx);

  ctx.save();
  ctx.translate(sx, sy);
  ctx.rotate(angle);
  const len = Math.hypot(ex - sx, ey - sy);
  ctx.beginPath();
  ctx.moveTo(0, -radius);
  ctx.lineTo(len, -radius);
  ctx.arc(len, 0, radius, -Math.PI / 2, Math.PI / 2);
  ctx.lineTo(0, radius);
  ctx.arc(0, 0, radius, Math.PI / 2, -Math.PI / 2);
  ctx.closePath();
  ctx.fillStyle = fill;
  ctx.strokeStyle = stroke;
  ctx.lineWidth = 2;
  ctx.fill();
  ctx.stroke();
  ctx.restore();
};

const drawAabb2D = (aabb, stroke, fill, dashed = false) => {
  const [x0, y1] = toCanvas2D(aabb.min);
  const [x1, y0] = toCanvas2D(aabb.max);
  ctx.save();
  ctx.setLineDash(dashed ? [8, 6] : []);
  ctx.fillStyle = fill;
  ctx.strokeStyle = stroke;
  ctx.lineWidth = 2;
  ctx.fillRect(x0, y0, x1 - x0, y1 - y0);
  ctx.strokeRect(x0, y0, x1 - x0, y1 - y0);
  ctx.restore();
};

const drawPolygon2D = (points, stroke, fill) => {
  ctx.beginPath();
  points.forEach((point, index) => {
    const [x, y] = toCanvas2D(point);
    if (index === 0) ctx.moveTo(x, y);
    else ctx.lineTo(x, y);
  });
  ctx.closePath();
  ctx.fillStyle = fill;
  ctx.strokeStyle = stroke;
  ctx.lineWidth = 2;
  ctx.fill();
  ctx.stroke();
};

const drawCircle2D = (center, radius, stroke, fill) => {
  const [x, y] = toCanvas2D(center);
  const scale = (canvas.width - 64) / 64;
  ctx.beginPath();
  ctx.arc(x, y, radius * scale, 0, Math.PI * 2);
  ctx.fillStyle = fill;
  ctx.strokeStyle = stroke;
  ctx.lineWidth = 3;
  ctx.fill();
  ctx.stroke();
};

const drawProbe2D = (point, stroke) => {
  const [x, y] = toCanvas2D(point);
  ctx.fillStyle = stroke;
  ctx.beginPath();
  ctx.arc(x, y, 6, 0, Math.PI * 2);
  ctx.fill();
};

const drawRay2D = (ray, stroke) => {
  const [ox, oy] = toCanvas2D(ray.origin);
  const end = [ray.origin[0] + ray.dir[0] * 64, ray.origin[1] + ray.dir[1] * 64];
  const [ex, ey] = toCanvas2D(end);
  ctx.strokeStyle = stroke;
  ctx.lineWidth = 2.5;
  ctx.beginPath();
  ctx.moveTo(ox, oy);
  ctx.lineTo(ex, ey);
  ctx.stroke();

  if (!ray.hit_point) return;
  const [hx, hy] = toCanvas2D(ray.hit_point);
  ctx.fillStyle = stroke;
  ctx.beginPath();
  ctx.arc(hx, hy, 6, 0, Math.PI * 2);
  ctx.fill();

  if (!ray.hit_normal) return;
  const [nx, ny] = ray.hit_normal;
  const [tx, ty] = toCanvas2D([
    ray.hit_point[0] + nx * 4,
    ray.hit_point[1] + ny * 4,
  ]);
  ctx.beginPath();
  ctx.moveTo(hx, hy);
  ctx.lineTo(tx, ty);
  ctx.stroke();
};

const aabb3Corners = (aabb) => {
  const [x0, y0, z0] = aabb.min;
  const [x1, y1, z1] = aabb.max;
  return [
    [x0, y0, z0], [x1, y0, z0], [x1, y1, z0], [x0, y1, z0],
    [x0, y0, z1], [x1, y0, z1], [x1, y1, z1], [x0, y1, z1],
  ];
};

const drawWireAabb3 = (aabb, stroke, fill, dashed = false) => {
  const pts = aabb3Corners(aabb).map(isoProject);
  const faces = [
    [0, 1, 2, 3],
    [4, 5, 6, 7],
  ];
  const edges = [
    [0, 1], [1, 2], [2, 3], [3, 0],
    [4, 5], [5, 6], [6, 7], [7, 4],
    [0, 4], [1, 5], [2, 6], [3, 7],
  ];

  ctx.save();
  ctx.setLineDash(dashed ? [7, 5] : []);
  faces.forEach((face) => {
    ctx.beginPath();
    face.forEach((index, i) => {
      const [x, y] = pts[index];
      if (i === 0) ctx.moveTo(x, y);
      else ctx.lineTo(x, y);
    });
    ctx.closePath();
    ctx.fillStyle = fill;
    ctx.fill();
  });
  ctx.strokeStyle = stroke;
  ctx.lineWidth = 2;
  edges.forEach(([a, b]) => {
    ctx.beginPath();
    ctx.moveTo(pts[a][0], pts[a][1]);
    ctx.lineTo(pts[b][0], pts[b][1]);
    ctx.stroke();
  });
  ctx.restore();
};

const drawCapsule3 = (capsule, stroke, fill) => {
  const start = isoProject(capsule.start);
  const end = isoProject(capsule.end);
  const radius = capsule.radius * 6;
  ctx.strokeStyle = stroke;
  ctx.lineWidth = radius * 1.4;
  ctx.lineCap = "round";
  ctx.beginPath();
  ctx.moveTo(start[0], start[1]);
  ctx.lineTo(end[0], end[1]);
  ctx.stroke();

  ctx.lineWidth = 2;
  ctx.fillStyle = fill;
  [start, end].forEach((point) => {
    ctx.beginPath();
    ctx.arc(point[0], point[1], radius, 0, Math.PI * 2);
    ctx.fill();
    ctx.stroke();
  });
};

const drawSphere3 = (center, radius, stroke, fill) => {
  const projected = isoProject(center);
  const scale = 5 + center[2] * 0.08;
  ctx.beginPath();
  ctx.arc(projected[0], projected[1], radius * scale, 0, Math.PI * 2);
  ctx.fillStyle = fill;
  ctx.strokeStyle = stroke;
  ctx.lineWidth = 3;
  ctx.fill();
  ctx.stroke();
};

const drawTriangle3 = (triangle, stroke, fill) => {
  const points = triangle.map(isoProject);
  ctx.beginPath();
  points.forEach(([x, y], index) => {
    if (index === 0) ctx.moveTo(x, y);
    else ctx.lineTo(x, y);
  });
  ctx.closePath();
  ctx.fillStyle = fill;
  ctx.strokeStyle = stroke;
  ctx.lineWidth = 2;
  ctx.fill();
  ctx.stroke();
};

const drawRay3 = (ray, stroke) => {
  const origin = isoProject(ray.origin);
  const end = isoProject([
    ray.origin[0] + ray.dir[0] * 64,
    ray.origin[1] + ray.dir[1] * 64,
    ray.origin[2] + ray.dir[2] * 64,
  ]);
  ctx.strokeStyle = stroke;
  ctx.lineWidth = 2;
  ctx.beginPath();
  ctx.moveTo(origin[0], origin[1]);
  ctx.lineTo(end[0], end[1]);
  ctx.stroke();

  if (!ray.hit_point) return;
  const hit = isoProject(ray.hit_point);
  ctx.fillStyle = stroke;
  ctx.beginPath();
  ctx.arc(hit[0], hit[1], 6, 0, Math.PI * 2);
  ctx.fill();

  if (!ray.hit_normal) return;
  const normalTip = isoProject([
    ray.hit_point[0] + ray.hit_normal[0] * 4,
    ray.hit_point[1] + ray.hit_normal[1] * 4,
    ray.hit_point[2] + ray.hit_normal[2] * 4,
  ]);
  ctx.beginPath();
  ctx.moveTo(hit[0], hit[1]);
  ctx.lineTo(normalTip[0], normalTip[1]);
  ctx.stroke();
};

const drawProbe3 = (point, stroke) => {
  const [x, y] = isoProject(point);
  ctx.fillStyle = stroke;
  ctx.beginPath();
  ctx.arc(x, y, 6, 0, Math.PI * 2);
  ctx.fill();
};

const drawHandle = (dimension, key, position, active = false) => {
  if (!layerVisibility.handles) return;
  const [x, y] = dimension === "2d" ? toCanvas2D(position) : isoProject(position);
  ctx.fillStyle = active ? colors.handle : colors.handleFill;
  ctx.strokeStyle = colors.handle;
  ctx.lineWidth = 2;
  ctx.beginPath();
  ctx.arc(x, y, 9, 0, Math.PI * 2);
  ctx.fill();
  ctx.stroke();
  ctx.fillStyle = colors.text;
  ctx.font = "600 12px IBM Plex Mono";
  ctx.textBaseline = "middle";
  ctx.fillText(HANDLE_LABELS[key], x + 13, y - 14);
};

const drawCallout = (x, y, text, accent = colors.text) => {
  if (!layerVisibility.callouts) return;
  ctx.save();
  ctx.font = "600 12px IBM Plex Mono";
  const width = ctx.measureText(text).width + 16;
  const height = 24;
  const boxX = x + 12;
  const boxY = y - height - 10;
  ctx.fillStyle = "rgba(255, 255, 255, 0.94)";
  ctx.strokeStyle = accent;
  ctx.lineWidth = 1.5;
  ctx.beginPath();
  ctx.roundRect(boxX, boxY, width, height, 10);
  ctx.fill();
  ctx.stroke();
  ctx.fillStyle = accent;
  ctx.textBaseline = "middle";
  ctx.fillText(text, boxX + 8, boxY + height / 2);
  ctx.restore();
};

const drawAxes3D = () => {
  if (!layerVisibility.axes) return;
  const origin = [56, canvas.height - 68];
  const axes = [
    { label: "X", dx: 44, dy: 12, color: "#d97706" },
    { label: "Y", dx: 0, dy: -48, color: "#0f766e" },
    { label: "Z", dx: -40, dy: 12, color: "#2563eb" },
  ];

  ctx.save();
  ctx.lineWidth = 2;
  ctx.font = "600 12px IBM Plex Mono";
  ctx.textBaseline = "middle";
  axes.forEach((axis) => {
    ctx.strokeStyle = axis.color;
    ctx.fillStyle = axis.color;
    ctx.beginPath();
    ctx.moveTo(origin[0], origin[1]);
    ctx.lineTo(origin[0] + axis.dx, origin[1] + axis.dy);
    ctx.stroke();
    ctx.fillText(axis.label, origin[0] + axis.dx + 8, origin[1] + axis.dy);
  });
  ctx.restore();
};

const renderBaseScene = () => {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  ctx.fillStyle = activeDimension === "2d" ? colors.bg2d : colors.bg3d;
  ctx.fillRect(0, 0, canvas.width, canvas.height);
  drawGrid(activeDimension === "2d" ? 40 : 52);
};

const render2DScene = (state, mode, rayTarget) => {
  if (mode === "overlap") {
    drawAabb2D(state.aabb, state.flags.circle_hits_aabb ? colors.hit : colors.staticStroke, state.flags.circle_hits_aabb ? "rgba(220, 38, 38, 0.10)" : colors.staticFill);
    drawCapsule2D(state.capsule, state.flags.circle_hits_capsule ? colors.hit : colors.staticStroke, state.flags.circle_hits_capsule ? "rgba(220, 38, 38, 0.10)" : colors.staticFill);
    drawPolygon2D(state.polygon, state.flags.circle_hits_polygon ? colors.hit : colors.polygon, state.flags.circle_hits_polygon ? "rgba(220, 38, 38, 0.08)" : colors.polygonFill);
    drawPolygon2D(state.oriented_box, state.flags.circle_hits_oriented_box ? colors.hit : colors.obb, state.flags.circle_hits_oriented_box ? "rgba(220, 38, 38, 0.08)" : colors.obbFill);
    drawCircle2D(state.circle_center, state.circle_radius, colors.dynamic, colors.dynamicFill);
    return;
  }

  if (mode === "containment") {
    drawAabb2D(state.aabb, state.flags.probe_in_aabb ? colors.hit : colors.staticStroke, state.flags.probe_in_aabb ? "rgba(220, 38, 38, 0.10)" : colors.staticFill);
    drawCapsule2D(state.capsule, state.flags.probe_in_capsule ? colors.hit : colors.staticStroke, state.flags.probe_in_capsule ? "rgba(220, 38, 38, 0.10)" : colors.staticFill);
    drawPolygon2D(state.polygon, state.flags.probe_in_polygon ? colors.hit : colors.polygon, state.flags.probe_in_polygon ? "rgba(220, 38, 38, 0.08)" : colors.polygonFill);
    drawPolygon2D(state.oriented_box, state.flags.probe_in_oriented_box ? colors.hit : colors.obb, state.flags.probe_in_oriented_box ? "rgba(220, 38, 38, 0.08)" : colors.obbFill);
    drawProbe2D(state.probe, state.flags.probe_in_aabb || state.flags.probe_in_capsule || state.flags.probe_in_polygon || state.flags.probe_in_oriented_box ? colors.hit : colors.dynamic);
    return;
  }

  if (mode === "raycast") {
    drawAabb2D(state.aabb, colors.staticStroke, colors.staticFill);
    drawCapsule2D(state.capsule, colors.staticStroke, colors.staticFill);
    drawPolygon2D(state.polygon, colors.polygon, colors.polygonFill);
    drawPolygon2D(state.oriented_box, colors.obb, colors.obbFill);
    drawCircle2D(state.circle_center, state.circle_radius, colors.dynamic, colors.dynamicFill);
    const ray = state[RAY_TARGETS["2d"][rayTarget].rayKey];
    drawRay2D(ray, RAY_TARGETS["2d"][rayTarget].color);
    if (ray.hit_point) {
      const [x, y] = toCanvas2D(ray.hit_point);
      drawCallout(x, y, `${RAY_TARGETS["2d"][rayTarget].label} hit`, RAY_TARGETS["2d"][rayTarget].color);
    }
    return;
  }

  const hasCandidate = (id) => state.candidate_ids.includes(id);
  drawAabb2D(state.aabb, hasCandidate(11) ? colors.hit : colors.staticStroke, hasCandidate(11) ? "rgba(220, 38, 38, 0.10)" : colors.staticFill);
  drawCapsule2D(state.capsule, hasCandidate(22) ? colors.hit : colors.staticStroke, hasCandidate(22) ? "rgba(220, 38, 38, 0.10)" : colors.staticFill);
  drawPolygon2D(state.oriented_box, hasCandidate(33) ? colors.hit : colors.obb, hasCandidate(33) ? "rgba(220, 38, 38, 0.08)" : colors.obbFill);
  drawPolygon2D(state.polygon, hasCandidate(44) ? colors.hit : colors.polygon, hasCandidate(44) ? "rgba(220, 38, 38, 0.08)" : colors.polygonFill);
  drawAabb2D(state.sensor, colors.sensor, colors.sensorFill, true);
  if (hasCandidate(11)) {
    const [x, y] = toCanvas2D([
      (state.aabb.min[0] + state.aabb.max[0]) / 2,
      state.aabb.max[1],
    ]);
    drawCallout(x, y, "candidate #11", colors.hit);
  }
  if (hasCandidate(22)) {
    const [x, y] = toCanvas2D(state.handles.capsule);
    drawCallout(x, y, "candidate #22", colors.hit);
  }
  if (hasCandidate(33)) {
    const [x, y] = toCanvas2D(state.handles.oriented_box);
    drawCallout(x, y, "candidate #33", colors.hit);
  }
  if (hasCandidate(44)) {
    const [x, y] = toCanvas2D(state.handles.polygon);
    drawCallout(x, y, "candidate #44", colors.hit);
  }
};

const render3DScene = (state, mode, rayTarget) => {
  if (mode === "overlap") {
    drawWireAabb3(state.aabb, state.flags.sphere_hits_aabb ? colors.hit : colors.staticStroke, state.flags.sphere_hits_aabb ? "rgba(220, 38, 38, 0.10)" : "rgba(148, 163, 184, 0.09)");
    drawCapsule3(state.capsule, state.flags.sphere_hits_capsule ? colors.hit : colors.staticStroke, state.flags.sphere_hits_capsule ? "rgba(220, 38, 38, 0.10)" : colors.staticFill);
    drawTriangle3(state.triangle, state.flags.sphere_hits_triangle ? colors.hit : colors.triangle, state.flags.sphere_hits_triangle ? "rgba(220, 38, 38, 0.08)" : colors.triangleFill);
    drawSphere3(state.sphere_center, state.sphere_radius, colors.dynamic, colors.dynamicFill);
    return;
  }

  if (mode === "containment") {
    drawWireAabb3(state.aabb, state.flags.probe_in_aabb ? colors.hit : colors.staticStroke, state.flags.probe_in_aabb ? "rgba(220, 38, 38, 0.10)" : "rgba(148, 163, 184, 0.09)");
    drawCapsule3(state.capsule, state.flags.probe_in_capsule ? colors.hit : colors.staticStroke, state.flags.probe_in_capsule ? "rgba(220, 38, 38, 0.10)" : colors.staticFill);
    drawTriangle3(state.triangle, state.flags.probe_on_triangle ? colors.hit : colors.triangle, state.flags.probe_on_triangle ? "rgba(220, 38, 38, 0.08)" : colors.triangleFill);
    const probe = isoProject(state.probe);
    const closest = isoProject(state.triangle_closest_point);
    ctx.save();
    ctx.setLineDash([6, 5]);
    ctx.strokeStyle = colors.triangle;
    ctx.lineWidth = 1.5;
    ctx.beginPath();
    ctx.moveTo(probe[0], probe[1]);
    ctx.lineTo(closest[0], closest[1]);
    ctx.stroke();
    ctx.restore();
    drawProbe3(state.probe, state.flags.probe_in_aabb || state.flags.probe_in_capsule || state.flags.probe_on_triangle ? colors.hit : colors.dynamic);
    drawProbe3(state.triangle_closest_point, colors.triangle);
    {
      const [x, y] = isoProject(state.triangle_closest_point);
      drawCallout(x, y, "closest point", colors.triangle);
    }
    return;
  }

  if (mode === "raycast") {
    drawWireAabb3(state.aabb, colors.staticStroke, "rgba(148, 163, 184, 0.09)");
    drawCapsule3(state.capsule, colors.staticStroke, colors.staticFill);
    drawTriangle3(state.triangle, colors.triangle, colors.triangleFill);
    drawSphere3(state.sphere_center, state.sphere_radius, colors.dynamic, colors.dynamicFill);
    const ray = state[RAY_TARGETS["3d"][rayTarget].rayKey];
    drawRay3(ray, RAY_TARGETS["3d"][rayTarget].color);
    if (ray.hit_point) {
      const [x, y] = isoProject(ray.hit_point);
      drawCallout(x, y, `${RAY_TARGETS["3d"][rayTarget].label} hit`, RAY_TARGETS["3d"][rayTarget].color);
    }
    return;
  }

  const hasCandidate = (id) => state.candidate_ids.includes(id);
  drawWireAabb3(state.aabb, hasCandidate(101) ? colors.hit : colors.staticStroke, hasCandidate(101) ? "rgba(220, 38, 38, 0.10)" : "rgba(148, 163, 184, 0.09)");
  drawCapsule3(state.capsule, hasCandidate(202) ? colors.hit : colors.staticStroke, hasCandidate(202) ? "rgba(220, 38, 38, 0.10)" : colors.staticFill);
  drawTriangle3(state.triangle, hasCandidate(303) ? colors.hit : colors.triangle, hasCandidate(303) ? "rgba(220, 38, 38, 0.08)" : colors.triangleFill);
  drawWireAabb3(state.sensor, colors.sensor, colors.sensorFill, true);
  if (hasCandidate(101)) {
    const [x, y] = isoProject(state.handles.aabb);
    drawCallout(x, y, "candidate #101", colors.hit);
  }
  if (hasCandidate(202)) {
    const [x, y] = isoProject(state.handles.capsule);
    drawCallout(x, y, "candidate #202", colors.hit);
  }
  if (hasCandidate(303)) {
    const [x, y] = isoProject(state.handles.triangle);
    drawCallout(x, y, "candidate #303", colors.hit);
  }
};

const activeHandleKeys = () => LESSONS[activeDimension][activeMode()].handles(activeState(), activeRayTarget());

const ensureActiveDepthHandle = () => {
  if (activeDimension !== "3d") return;
  const keys = activeHandleKeys();
  if (!keys.includes(activeDepthHandle)) {
    activeDepthHandle = keys[0];
  }
};

const renderScene = () => {
  const state = activeState();
  if (!state) return;
  renderBaseScene();

  if (activeDimension === "2d") {
    render2DScene(state, activeMode(), activeRayTarget());
  } else {
    render3DScene(state, activeMode(), activeRayTarget());
    drawAxes3D();
  }

  activeHandleKeys().forEach((key) => {
    drawHandle(activeDimension, key, state.handles[key], draggingHandle === key || (activeDimension === "3d" && activeDepthHandle === key));
  });
};

const renderFlags = () => {
  liveFlags.innerHTML = "";
  const flagEntries = LESSONS[activeDimension][activeMode()].flags;
  flagEntries.forEach(([label, flagKey, evaluator], index) => {
    const value = evaluator(activeState(), activeRayTarget());
    const item = document.createElement("li");
    item.className = value ? "flag-on" : "flag-off";
    item.textContent = `${label}: ${value ? "true" : "false"}`;
    item.tabIndex = 0;
    item.addEventListener("mouseenter", () => {
      setDetail(label, flagKey ? FLAG_DETAILS[flagKey] : "This readout tracks whether the matching lesson condition is currently in the broadphase candidate set.");
    });
    item.addEventListener("focus", () => {
      setDetail(label, flagKey ? FLAG_DETAILS[flagKey] : "This readout tracks whether the matching lesson condition is currently in the broadphase candidate set.");
    });
    if (index === 0) {
      setDetail(label, flagKey ? FLAG_DETAILS[flagKey] : "This readout tracks whether the matching lesson condition is currently in the broadphase candidate set.");
    }
    liveFlags.append(item);
  });
};

const renderHandleList = () => {
  handleList.innerHTML = "";
  activeHandleKeys().forEach((key) => {
    const chip = document.createElement("span");
    const isSelected =
      activeDimension === "3d" ? activeDepthHandle === key : activeHandleByDimension["2d"] === key;
    chip.className = `chip${isSelected ? " is-selected" : ""}`;
    chip.dataset.handle = key;
    chip.textContent =
      activeDimension === "3d"
        ? `${HANDLE_LABELS[key]} z=${activeState().handles[key][2].toFixed(1)}`
        : HANDLE_LABELS[key];
    handleList.append(chip);
  });
};

const renderScenarios = () => {
  scenarioList.innerHTML = "";
  const scenarios = activeScenarios();
  scenarios.forEach((scenario, index) => {
    const chip = document.createElement("button");
    chip.type = "button";
    chip.className = `chip button-chip${activeScenarioByKey[scenarioKey()] === index ? " is-active" : ""}`;
    chip.dataset.scenarioIndex = `${index}`;
    chip.textContent = scenario.label;
    scenarioList.append(chip);
  });
};

const renderScenarioState = () => {
  const scenarios = activeScenarios();
  const selectedIndex = activeScenarioIndex();
  const selectedScenario = activeScenario();
  const hasSelection = selectedScenario != null;

  if (!scenarios.length) {
    scenarioStatus.textContent = "No scenarios";
    scenarioCaption.textContent = "This lesson does not define a guided preset sequence.";
    scenarioPrev.disabled = true;
    scenarioReplay.disabled = true;
    scenarioNext.disabled = true;
    scenarioPlay.disabled = true;
    scenarioPlay.textContent = "Play Tour";
    return;
  }

  if (!hasSelection) {
    scenarioStatus.textContent = `${scenarios.length} presets available`;
    scenarioCaption.textContent = "Pick a scenario to animate a guided setup for the current lesson.";
    scenarioPrev.disabled = true;
    scenarioReplay.disabled = true;
    scenarioNext.disabled = false;
    scenarioPlay.disabled = false;
    scenarioPlay.textContent = scenarioTourPlaying ? "Pause Tour" : "Play Tour";
    return;
  }

  scenarioStatus.textContent = `Scenario ${selectedIndex + 1} / ${scenarios.length}${scenarioTourPlaying ? " - Tour running" : ""}`;
  scenarioCaption.textContent = selectedScenario.description;
  scenarioPrev.disabled = selectedIndex === 0;
  scenarioReplay.disabled = false;
  scenarioNext.disabled = selectedIndex === scenarios.length - 1;
  scenarioPlay.disabled = false;
  scenarioPlay.textContent = scenarioTourPlaying ? "Pause Tour" : "Play Tour";
};

const renderInsights = () => {
  insightList.innerHTML = "";
  LESSONS[activeDimension][activeMode()]
    .insights(activeState(), activeRayTarget())
    .forEach((text) => {
      const item = document.createElement("div");
      item.className = "note-card";
      item.textContent = text;
      insightList.append(item);
    });
};

const renderCandidates = () => {
  candidateIds.innerHTML = "";
  const labels = CANDIDATE_LABELS[activeDimension];
  if (!activeState().candidate_ids.length) {
    const chip = document.createElement("span");
    chip.className = "chip";
    chip.textContent = "none";
    candidateIds.append(chip);
    return;
  }

  activeState().candidate_ids.forEach((id) => {
    const chip = document.createElement("span");
    chip.className = "chip";
    chip.textContent = `${labels[id] ?? "Shape"} #${id}`;
    chip.tabIndex = 0;
    chip.addEventListener("mouseenter", () => {
      setDetail(`${labels[id] ?? "Shape"} #${id}`, CANDIDATE_DETAILS[activeDimension][id] ?? "This object was returned by the deterministic broadphase query.");
    });
    chip.addEventListener("focus", () => {
      setDetail(`${labels[id] ?? "Shape"} #${id}`, CANDIDATE_DETAILS[activeDimension][id] ?? "This object was returned by the deterministic broadphase query.");
    });
    candidateIds.append(chip);
  });
};

const renderSelectionState = () => {
  const selectedKey =
    activeDimension === "3d" ? activeDepthHandle : activeHandleByDimension["2d"];
  const point = activeState().handles[selectedKey];
  selectionLabel.textContent = `Selected handle: ${HANDLE_LABELS[selectedKey]}`;
  selectionCoords.textContent =
    activeDimension === "3d"
      ? `x=${point[0].toFixed(1)} y=${point[1].toFixed(1)} z=${point[2].toFixed(1)}`
      : `x=${point[0].toFixed(1)} y=${point[1].toFixed(1)}`;
};

const renderDepthState = () => {
  const show = activeDimension === "3d";
  depthBlock.classList.toggle("is-hidden", !show);
  if (!show) return;

  ensureActiveDepthHandle();
  const z = activeState().handles[activeDepthHandle][2];
  depthLabel.textContent = `Selected handle: ${HANDLE_LABELS[activeDepthHandle]}`;
  depthRange.value = z.toFixed(1);
  depthValue.textContent = `z=${z.toFixed(1)}`;
};

const renderToolbarState = () => {
  dimensionRow.querySelectorAll("[data-dimension]").forEach((button) => {
    button.classList.toggle("active", button.dataset.dimension === activeDimension);
  });
  modeRow.querySelectorAll("[data-mode]").forEach((button) => {
    button.classList.toggle("active", button.dataset.mode === activeMode());
  });

  rayTargetRow.innerHTML = "";
  Object.entries(RAY_TARGETS[activeDimension]).forEach(([key, spec]) => {
    const button = document.createElement("button");
    button.type = "button";
    button.className = `target-pill${activeRayTarget() === key ? " active" : ""}`;
    button.dataset.rayTarget = key;
    button.textContent = spec.label;
    rayTargetRow.append(button);
  });

  raycastTargetGroup.classList.toggle("is-hidden", activeMode() !== "raycast");
  viewPresetGroup.classList.toggle("is-hidden", activeDimension !== "3d");
  const recommendedView = recommendedViewPreset();
  viewPresetRow.querySelectorAll("[data-view]").forEach((button) => {
    button.classList.toggle("active", button.dataset.view === activeViewPreset);
    button.classList.toggle(
      "is-recommended",
      activeDimension === "3d" && button.dataset.view === recommendedView,
    );
    button.title =
      activeDimension === "3d" && button.dataset.view === recommendedView
        ? "Recommended for this lesson"
        : "";
  });
  layerRow.querySelectorAll("[data-layer]").forEach((button) => {
    const layer = button.dataset.layer;
    button.classList.toggle("active", layerVisibility[layer]);
    button.classList.toggle("is-hidden", layer === "axes" && activeDimension !== "3d");
  });
};

const renderUi = () => {
  if (!activeState()) return;
  ensureActiveDepthHandle();
  const lesson = LESSONS[activeDimension][activeMode()];
  modeTitle.textContent = lesson.title;
  modeDescription.textContent = lesson.description;
  modeSummary.textContent = lesson.summary(activeState(), activeRayTarget());
  dragHint.textContent =
    activeDimension === "3d"
      ? `Drag projected handles on the canvas. Current view: ${
          VIEW_PRESETS[activeViewPreset].label
        }. Recommended for this lesson: ${
          VIEW_PRESETS[recommendedViewPreset()].label
        }. Use the depth slider or mouse wheel to move the selected 3D handle along z.`
      : activeMode() === "raycast"
        ? "Drag the selected ray origin or any target shape. The hit marker and normal update live."
        : "Drag any labeled handle directly on the canvas. Every query recomputes after each move.";

  candidateBlock.classList.toggle("is-hidden", activeMode() !== "broadphase");
  renderToolbarState();
  renderHandleList();
  renderScenarios();
  renderScenarioState();
  renderSelectionState();
  renderDepthState();
  renderFlags();
  renderInsights();
  if (activeMode() === "broadphase") renderCandidates();
  renderScene();
  setStatus("WASM core: ready", `${activeDimension.toUpperCase()} interactive lab active`, false);
};

const syncState = (dimension) => {
  states[dimension] = JSON.parse(sandboxes[dimension].render_state());
};

const refreshActiveState = () => {
  syncState(activeDimension);
  renderUi();
};

const cancelScenarioAnimation = () => {
  if (scenarioAnimationFrame) {
    cancelAnimationFrame(scenarioAnimationFrame);
    scenarioAnimationFrame = 0;
  }
  scenarioAnimationToken += 1;
};

const clearActiveScenarioSelection = () => {
  delete activeScenarioByKey[scenarioKey()];
  stopScenarioTour();
  cancelScenarioAnimation();
};

const animateScenarioMoves = (dimension, moves, durationMs = 420) => {
  if (!moves.length) {
    syncState(dimension);
    if (activeDimension === dimension) renderUi();
    return;
  }

  cancelScenarioAnimation();
  const token = scenarioAnimationToken;
  const sandbox = sandboxes[dimension];
  const initialState = JSON.parse(sandbox.render_state());
  const origins = Object.fromEntries(
    moves.map(({ handle }) => [handle, [...initialState.handles[handle]]]),
  );
  const ease = (t) => 1 - (1 - t) * (1 - t);
  let startTime = null;

  const step = (now) => {
    if (token !== scenarioAnimationToken) return;
    if (startTime == null) startTime = now;
    const progress = Math.min(1, (now - startTime) / durationMs);
    const eased = ease(progress);

    moves.forEach(({ handle, point }) => {
      const origin = origins[handle];
      const nextPoint = origin.map((value, index) => value + (point[index] - value) * eased);
      if (dimension === "2d") {
        sandbox.move_handle(HANDLE_IDS["2d"][handle], nextPoint[0], nextPoint[1]);
      } else {
        sandbox.move_handle(HANDLE_IDS["3d"][handle], nextPoint[0], nextPoint[1], nextPoint[2]);
      }
    });

    syncState(dimension);
    if (activeDimension === dimension) renderUi();

    if (progress < 1) {
      scenarioAnimationFrame = requestAnimationFrame(step);
      return;
    }

    scenarioAnimationFrame = 0;
  };

  scenarioAnimationFrame = requestAnimationFrame(step);
};

const formatPoint = (point) => {
  if (!point) return "none";
  return `(${point.map((value) => value.toFixed(1)).join(", ")})`;
};

const FLAG_DETAILS = {
  circle_hits_aabb: "The circle's radius reaches the AABB after clamping the circle center to the box bounds.",
  circle_hits_capsule: "The circle overlaps the capsule when its center comes within the combined round radius of the capsule segment.",
  circle_hits_polygon: "The circle overlaps the convex polygon when it reaches an edge or sits inside the polygon interior.",
  circle_hits_oriented_box: "The circle overlaps the oriented box using the same boundary-inclusive polygon test applied to the rotated box corners.",
  probe_in_aabb: "The probe lies within the box min/max extents on every axis.",
  probe_in_capsule: "The probe is inside the round radius around the capsule segment.",
  probe_in_polygon: "The probe is on or inside the convex polygon boundary.",
  probe_in_oriented_box: "The probe is inside the oriented box after projecting into the box's local basis.",
  ray_hits_aabb: "The ray's first non-negative slab intersection lands on the box within the configured maximum travel.",
  ray_hits_circle: "The ray reaches the circle before the maximum travel distance.",
  ray_hits_capsule: "The ray hits the capsule's side or either endcap before the travel limit.",
  ray_hits_polygon: "The ray enters the convex polygon across one of its clipping planes.",
  ray_hits_oriented_box: "The ray hits the rotated box, which is tested through the oriented box corner polygon.",
  sphere_hits_aabb: "The sphere reaches the AABB when the closest point on the box is within the sphere radius.",
  sphere_hits_capsule: "The sphere overlaps the capsule when the center-to-segment distance is within the combined round radius.",
  sphere_hits_triangle: "The sphere reaches the triangle face, edge, or vertex when the closest triangle point is within the radius.",
  probe_on_triangle: "The probe lies exactly on the triangle according to the deterministic closest-point distance query.",
  ray_hits_sphere: "The 3D ray reaches the sphere before the maximum travel distance.",
  ray_hits_triangle: "The 3D ray intersects the triangle plane and lands inside the triangle bounds.",
};

const CANDIDATE_DETAILS = {
  "2d": {
    11: "The sensor overlaps the AABB's occupied grid cells, so the AABB enters the broadphase candidate set.",
    22: "The sensor overlaps the capsule bounds inserted into the 2D uniform grid.",
    33: "The sensor overlaps the oriented box bounds inserted into the 2D uniform grid.",
    44: "The sensor overlaps the polygon bounds inserted into the 2D uniform grid.",
  },
  "3d": {
    101: "The 3D sensor overlaps the AABB's occupied cells in UniformGrid3.",
    202: "The 3D sensor overlaps the capsule bounds inserted into UniformGrid3.",
    303: "The 3D sensor overlaps the triangle bounds inserted into UniformGrid3.",
  },
};

const setDetail = (title, body) => {
  detailTitle.textContent = title;
  detailBody.textContent = body;
};

const projectHandle = (dimension, point) => (dimension === "2d" ? toCanvas2D(point) : isoProject(point));

const pickHandle = (event) => {
  const state = activeState();
  if (!state || !layerVisibility.handles) return null;
  const [canvasX, canvasY] = eventToCanvas(event);
  let best = null;
  let bestDistance = 18;

  activeHandleKeys().forEach((key) => {
    const [hx, hy] = projectHandle(activeDimension, state.handles[key]);
    const distance = Math.hypot(canvasX - hx, canvasY - hy);
    if (distance < bestDistance) {
      best = key;
      bestDistance = distance;
    }
  });

  return best;
};

const updateCursor = (event) => {
  canvas.style.cursor = pickHandle(event) ? "grab" : "default";
};

const moveHandleFromEvent = (handleKey, event) => {
  const sandbox = activeSandbox();
  clearActiveScenarioSelection();
  if (activeDimension === "2d") {
    activeHandleByDimension["2d"] = handleKey;
    const [canvasX, canvasY] = eventToCanvas(event);
    const [x, y] = fromCanvas2D(canvasX, canvasY);
    sandbox.move_handle(HANDLE_IDS["2d"][handleKey], x, y);
  } else {
    const [canvasX, canvasY] = eventToCanvas(event);
    const z = activeState().handles[handleKey][2];
    const [x, y] = fromCanvas3D(canvasX, canvasY, z);
    sandbox.move_handle(HANDLE_IDS["3d"][handleKey], x, y, z);
    activeDepthHandle = handleKey;
  }
  refreshActiveState();
};

const applyScenario = (scenario) => {
  if (!wasmReady) return;
  activeSandbox().reset();
  syncState(activeDimension);
  applyRecommendedViewPreset();
  if (scenario.rayTarget) {
    activeRayTargets[activeDimension] = scenario.rayTarget;
  }
  scenario.moves.forEach(({ handle, point }) => {
    if (activeDimension === "2d") {
      activeHandleByDimension["2d"] = handle;
    } else {
      activeDepthHandle = handle;
    }
  });
  animateScenarioMoves(activeDimension, scenario.moves);
};

const scheduleScenarioTourAdvance = () => {
  clearScenarioTourTimer();
  if (!scenarioTourPlaying) return;
  scenarioTourTimer = setTimeout(() => {
    if (!scenarioTourPlaying) return;
    const scenarios = activeScenarios();
    const current = activeScenarioIndex();
    const nextIndex = Number.isInteger(current) ? current + 1 : 0;
    if (nextIndex >= scenarios.length) {
      stopScenarioTour();
      renderUi();
      return;
    }
    runScenarioAtIndex(nextIndex);
  }, SCENARIO_TOUR_STEP_MS);
};

const runScenarioAtIndex = (index) => {
  const scenarios = activeScenarios();
  const scenario = scenarios[index];
  if (!scenario) return;
  activeScenarioByKey[scenarioKey()] = index;
  applyScenario(scenario);
  if (scenarioTourPlaying) scheduleScenarioTourAdvance();
};

canvas.addEventListener("pointerdown", (event) => {
  if (!wasmReady) return;
  const handle = pickHandle(event);
  if (!handle) return;
  draggingHandle = handle;
  if (activeDimension === "3d") activeDepthHandle = handle;
  else activeHandleByDimension["2d"] = handle;
  canvas.setPointerCapture(event.pointerId);
  moveHandleFromEvent(handle, event);
});

canvas.addEventListener("pointermove", (event) => {
  if (!wasmReady) return;
  if (!draggingHandle) {
    updateCursor(event);
    return;
  }

  moveHandleFromEvent(draggingHandle, event);
});

const stopDragging = (event) => {
  if (!draggingHandle) return;
  draggingHandle = null;
  if (event?.pointerId != null && canvas.hasPointerCapture(event.pointerId)) {
    canvas.releasePointerCapture(event.pointerId);
  }
  canvas.style.cursor = "default";
  renderScene();
};

canvas.addEventListener("pointerup", stopDragging);
canvas.addEventListener("pointercancel", stopDragging);
canvas.addEventListener("pointerleave", (event) => {
  if (!draggingHandle) updateCursor(event);
});

dimensionRow.addEventListener("click", (event) => {
  const button = event.target.closest("[data-dimension]");
  if (!button) return;
  stopScenarioTour();
  cancelScenarioAnimation();
  activeDimension = button.dataset.dimension;
  applyRecommendedViewPreset();
  renderUi();
});

modeRow.addEventListener("click", (event) => {
  const button = event.target.closest("[data-mode]");
  if (!button) return;
  stopScenarioTour();
  cancelScenarioAnimation();
  activeModes[activeDimension] = button.dataset.mode;
  applyRecommendedViewPreset();
  renderUi();
});

rayTargetRow.addEventListener("click", (event) => {
  const button = event.target.closest("[data-ray-target]");
  if (!button) return;
  stopScenarioTour();
  activeRayTargets[activeDimension] = button.dataset.rayTarget;
  renderUi();
});

viewPresetRow.addEventListener("click", (event) => {
  const button = event.target.closest("[data-view]");
  if (!button) return;
  stopScenarioTour();
  activeViewPreset = button.dataset.view;
  renderUi();
});

layerRow.addEventListener("click", (event) => {
  const button = event.target.closest("[data-layer]");
  if (!button) return;
  const layer = button.dataset.layer;
  if (layer === "axes" && activeDimension !== "3d") return;
  layerVisibility[layer] = !layerVisibility[layer];
  if (layer === "handles") {
    draggingHandle = null;
    canvas.style.cursor = "default";
  }
  renderUi();
});

handleList.addEventListener("click", (event) => {
  const chip = event.target.closest("[data-handle]");
  if (!chip) return;
  if (activeDimension === "3d") {
    activeDepthHandle = chip.dataset.handle;
  } else {
    activeHandleByDimension["2d"] = chip.dataset.handle;
  }
  renderUi();
});

scenarioList.addEventListener("click", (event) => {
  const button = event.target.closest("[data-scenario-index]");
  if (!button) return;
  stopScenarioTour();
  runScenarioAtIndex(Number(button.dataset.scenarioIndex));
});

scenarioPrev.addEventListener("click", () => {
  const current = activeScenarioIndex();
  if (!Number.isInteger(current) || current <= 0) return;
  stopScenarioTour();
  runScenarioAtIndex(current - 1);
});

scenarioReplay.addEventListener("click", () => {
  const current = activeScenarioIndex();
  if (!Number.isInteger(current)) return;
  stopScenarioTour();
  runScenarioAtIndex(current);
});

scenarioNext.addEventListener("click", () => {
  const current = activeScenarioIndex();
  stopScenarioTour();
  if (!Number.isInteger(current)) {
    runScenarioAtIndex(0);
    return;
  }
  runScenarioAtIndex(current + 1);
});

scenarioPlay.addEventListener("click", () => {
  if (scenarioTourPlaying) {
    stopScenarioTour();
    renderUi();
    return;
  }

  const scenarios = activeScenarios();
  if (!scenarios.length) return;
  scenarioTourPlaying = true;
  const current = activeScenarioIndex();
  runScenarioAtIndex(Number.isInteger(current) ? current : 0);
});

depthRange.addEventListener("input", () => {
  if (!wasmReady || activeDimension !== "3d") return;
  ensureActiveDepthHandle();
  clearActiveScenarioSelection();
  const [x, y] = activeState().handles[activeDepthHandle];
  sandboxes["3d"].move_handle(HANDLE_IDS["3d"][activeDepthHandle], x, y, Number(depthRange.value));
  refreshActiveState();
});

const nudgeSelectedDepth = (direction, multiplier = 1) => {
  if (!wasmReady || activeDimension !== "3d") return;
  ensureActiveDepthHandle();
  clearActiveScenarioSelection();
  const [x, y, z] = activeState().handles[activeDepthHandle];
  const nextZ = Math.max(2, Math.min(62, z + direction * 0.5 * multiplier));
  sandboxes["3d"].move_handle(HANDLE_IDS["3d"][activeDepthHandle], x, y, nextZ);
  refreshActiveState();
};

canvas.addEventListener(
  "wheel",
  (event) => {
    if (!wasmReady || activeDimension !== "3d") return;
    ensureActiveDepthHandle();
    event.preventDefault();
    nudgeSelectedDepth(-Math.sign(event.deltaY), event.shiftKey ? 4 : 1);
  },
  { passive: false },
);

window.addEventListener("keydown", (event) => {
  if (!wasmReady || activeDimension !== "3d") return;
  if (event.key !== "[" && event.key !== "]" && event.key !== "ArrowDown" && event.key !== "ArrowUp") {
    return;
  }
  event.preventDefault();
  const direction = event.key === "]" || event.key === "ArrowUp" ? 1 : -1;
  nudgeSelectedDepth(direction, event.shiftKey ? 4 : 1);
});

resetButton.addEventListener("click", () => {
  if (!wasmReady) return;
  clearActiveScenarioSelection();
  activeSandbox().reset();
  refreshActiveState();
});

const boot = async () => {
  try {
    const wasmModule = await import("./pkg/bumpbox_demo_wasm.js");
    await wasmModule.default();
    sandboxes["2d"] = new wasmModule.Sandbox2D();
    sandboxes["3d"] = new wasmModule.Sandbox3D();
    syncState("2d");
    syncState("3d");
    wasmReady = true;
    renderUi();
  } catch (error) {
    console.warn(error);
    wasmReady = false;
    setStatus(
      "WASM core: unavailable (run wasm-pack build)",
      "demo-wasm/www expects ./pkg output",
      true,
    );
  }
};

void boot();
