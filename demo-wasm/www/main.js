const wasmStatus = document.getElementById("wasm-status");
const wasmDetail = document.getElementById("wasm-detail");
const speedRange = document.getElementById("speed-range");
const speedValue = document.getElementById("speed-value");
const playButton = document.getElementById("play-button");
const pauseButton = document.getElementById("pause-button");
const resetButton = document.getElementById("reset-button");

const scene2Canvas = document.getElementById("scene2d");
const scene3Canvas = document.getElementById("scene3d");
const scene2Ctx = scene2Canvas.getContext("2d");
const scene3Ctx = scene3Canvas.getContext("2d");

const scene2Summary = document.getElementById("scene2-summary");
const scene3Summary = document.getElementById("scene3-summary");
const scene2Flags = document.getElementById("scene2-flags");
const scene3Flags = document.getElementById("scene3-flags");
const scene2Candidates = document.getElementById("scene2-candidates");
const scene3Candidates = document.getElementById("scene3-candidates");

const colors = {
  bg: "#fff8ef",
  panel: "#fffdf8",
  grid: "rgba(100, 116, 139, 0.14)",
  staticStroke: "#475569",
  staticFill: "rgba(148, 163, 184, 0.12)",
  dynamic: "#d97706",
  dynamicFill: "rgba(245, 158, 11, 0.18)",
  sensor: "#0f766e",
  sensorFill: "rgba(45, 212, 191, 0.12)",
  hit: "#dc2626",
  ray: "#0f172a",
  rayRound: "#d97706",
  rayCapsule: "#0f766e",
  rayPolygon: "#1d4ed8",
  rayObb: "#7c3aed",
  rayTriangle: "#0ea5a4",
  sweepTriangle: "#ea580c",
  sweepCapsuleTriangle: "#be123c",
  sweepSegmentTriangle: "#2563eb",
  sweepTriangleTriangle: "#7c2d12",
  sphereTriangleDistance: "#dc2626",
  sphereTriangleDistanceSkew: "#991b1b",
  sphereTriangleDistanceVertex: "#b91c1c",
  capsuleTriangleDistance: "#1d4ed8",
  capsuleTriangleDistanceSkew: "#1e3a8a",
  triangleSegment: "#0891b2",
  triangleDistance: "#7c2d12",
  trianglePeer: "#7c3aed",
  trianglePeerDistance: "#9333ea",
  triangleAabbDistance: "#c026d3",
  triangleAabbDistanceVertex: "#a21caf",
  triangleStroke: "#0f766e",
  triangleFill: "rgba(15, 118, 110, 0.16)",
  text: "#0f172a",
};

let wasmReady = false;
let isPlaying = true;
let playbackSpeed = 1;
let demo2d = null;
let demo3d = null;
let state2d = null;
let state3d = null;
let accumulator = 0;
let lastFrame = performance.now();

const setStatus = (text, detail = "", error = false) => {
  wasmStatus.textContent = text;
  wasmDetail.textContent = detail;
  wasmStatus.classList.toggle("error", error);
};

const toCanvas2D = (point) => {
  const pad = 28;
  const scaleX = (scene2Canvas.width - pad * 2) / 64;
  const scaleY = (scene2Canvas.height - pad * 2) / 64;
  return [pad + point[0] * scaleX, scene2Canvas.height - pad - point[1] * scaleY];
};

const isoProject = (point) => {
  const scale = 8.4;
  const centerX = scene3Canvas.width * 0.51;
  const centerY = scene3Canvas.height * 0.7;
  const [x, y, z] = point;
  return [
    centerX + (x - z) * scale,
    centerY - y * scale + (x + z) * scale * 0.36,
  ];
};

const drawGrid = (ctx, width, height, step = 40) => {
  ctx.strokeStyle = colors.grid;
  ctx.lineWidth = 1;
  for (let x = step; x < width; x += step) {
    ctx.beginPath();
    ctx.moveTo(x, 0);
    ctx.lineTo(x, height);
    ctx.stroke();
  }
  for (let y = step; y < height; y += step) {
    ctx.beginPath();
    ctx.moveTo(0, y);
    ctx.lineTo(width, y);
    ctx.stroke();
  }
};

const drawCapsule2D = (ctx, capsule, stroke, fill) => {
  const [sx, sy] = toCanvas2D(capsule.start);
  const [ex, ey] = toCanvas2D(capsule.end);
  const scale = (scene2Canvas.width - 56) / 64;
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

const drawAabb2D = (ctx, aabb, stroke, fill, dashed = false) => {
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

const drawPolygon2D = (ctx, points, stroke, fill) => {
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

const drawRay2D = (ctx, ray, stroke, hitFill) => {
  const [ox, oy] = toCanvas2D(ray.origin);
  const [dx, dy] = ray.dir;
  const end = [ray.origin[0] + dx * 64, ray.origin[1] + dy * 64];
  const [ex, ey] = toCanvas2D(end);

  ctx.strokeStyle = stroke;
  ctx.lineWidth = 2;
  ctx.beginPath();
  ctx.moveTo(ox, oy);
  ctx.lineTo(ex, ey);
  ctx.stroke();

  if (ray.hit_point) {
    const [hx, hy] = toCanvas2D(ray.hit_point);
    ctx.fillStyle = hitFill;
    ctx.beginPath();
    ctx.arc(hx, hy, 6, 0, Math.PI * 2);
    ctx.fill();

    if (ray.hit_normal) {
      const [nx, ny] = ray.hit_normal;
      const [tx, ty] = toCanvas2D([
        ray.hit_point[0] + nx * 4,
        ray.hit_point[1] + ny * 4,
      ]);
      ctx.strokeStyle = hitFill;
      ctx.lineWidth = 2;
      ctx.beginPath();
      ctx.moveTo(hx, hy);
      ctx.lineTo(tx, ty);
      ctx.stroke();
    }
  }
};

const drawScene2D = () => {
  if (!state2d) return;
  scene2Ctx.clearRect(0, 0, scene2Canvas.width, scene2Canvas.height);
  scene2Ctx.fillStyle = colors.bg;
  scene2Ctx.fillRect(0, 0, scene2Canvas.width, scene2Canvas.height);
  drawGrid(scene2Ctx, scene2Canvas.width, scene2Canvas.height);

  drawPolygon2D(scene2Ctx, state2d.polygon, colors.staticStroke, colors.staticFill);
  drawCapsule2D(scene2Ctx, state2d.capsule, colors.staticStroke, colors.staticFill);
  drawAabb2D(scene2Ctx, state2d.aabb, colors.staticStroke, colors.staticFill);
  drawPolygon2D(scene2Ctx, state2d.oriented_box, colors.staticStroke, "rgba(100, 116, 139, 0.08)");
  drawAabb2D(scene2Ctx, state2d.sensor, colors.sensor, colors.sensorFill, true);
  drawRay2D(scene2Ctx, state2d.aabb_ray, colors.ray, colors.hit);
  drawRay2D(scene2Ctx, state2d.circle_ray, colors.rayRound, colors.rayRound);
  drawRay2D(scene2Ctx, state2d.capsule_ray, colors.rayCapsule, colors.rayCapsule);
  drawRay2D(scene2Ctx, state2d.polygon_ray, colors.rayPolygon, colors.rayPolygon);
  drawRay2D(scene2Ctx, state2d.oriented_box_ray, colors.rayObb, colors.rayObb);

  const [probeX, probeY] = toCanvas2D(state2d.probe);
  scene2Ctx.fillStyle = state2d.flags.probe_in_polygon || state2d.flags.probe_in_capsule || state2d.flags.probe_in_oriented_box
    ? colors.hit
    : colors.dynamic;
  scene2Ctx.beginPath();
  scene2Ctx.arc(probeX, probeY, 5, 0, Math.PI * 2);
  scene2Ctx.fill();

  const [circleX, circleY] = toCanvas2D(state2d.circle_center);
  const scale = (scene2Canvas.width - 56) / 64;
  scene2Ctx.fillStyle = colors.dynamicFill;
  scene2Ctx.strokeStyle = state2d.flags.circle_hits_aabb ? colors.hit : colors.dynamic;
  scene2Ctx.lineWidth = 3;
  scene2Ctx.beginPath();
  scene2Ctx.arc(circleX, circleY, state2d.circle_radius * scale, 0, Math.PI * 2);
  scene2Ctx.fill();
  scene2Ctx.stroke();

  const rayHitCount2D = [
    state2d.flags.ray_hits_aabb,
    state2d.flags.ray_hits_circle,
    state2d.flags.ray_hits_capsule,
    state2d.flags.ray_hits_polygon,
    state2d.flags.ray_hits_oriented_box,
  ].filter(Boolean).length;
  scene2Summary.textContent = `${rayHitCount2D}/5 raycasts live`;
};

const aabb3Corners = (aabb) => {
  const [x0, y0, z0] = aabb.min;
  const [x1, y1, z1] = aabb.max;
  return [
    [x0, y0, z0], [x1, y0, z0], [x1, y1, z0], [x0, y1, z0],
    [x0, y0, z1], [x1, y0, z1], [x1, y1, z1], [x0, y1, z1],
  ];
};

const drawWireAabb3 = (ctx, aabb, stroke, fill, dashed = false) => {
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
  for (const face of faces) {
    ctx.beginPath();
    face.forEach((index, i) => {
      const [x, y] = pts[index];
      if (i === 0) ctx.moveTo(x, y);
      else ctx.lineTo(x, y);
    });
    ctx.closePath();
    ctx.fillStyle = fill;
    ctx.fill();
  }
  ctx.strokeStyle = stroke;
  ctx.lineWidth = 2;
  for (const [a, b] of edges) {
    ctx.beginPath();
    ctx.moveTo(pts[a][0], pts[a][1]);
    ctx.lineTo(pts[b][0], pts[b][1]);
    ctx.stroke();
  }
  ctx.restore();
};

const drawCapsule3 = (ctx, capsule, stroke, fill) => {
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

  ctx.strokeStyle = stroke;
  ctx.lineWidth = 2;
  ctx.fillStyle = fill;
  for (const point of [start, end]) {
    ctx.beginPath();
    ctx.arc(point[0], point[1], radius, 0, Math.PI * 2);
    ctx.fill();
    ctx.stroke();
  }
};

const drawSegment3 = (ctx, segment, stroke) => {
  const start = isoProject(segment.start);
  const end = isoProject(segment.end);

  ctx.save();
  ctx.strokeStyle = stroke;
  ctx.lineWidth = 4;
  ctx.lineCap = "round";
  ctx.beginPath();
  ctx.moveTo(start[0], start[1]);
  ctx.lineTo(end[0], end[1]);
  ctx.stroke();

  ctx.fillStyle = stroke;
  for (const point of [start, end]) {
    ctx.beginPath();
    ctx.arc(point[0], point[1], 4, 0, Math.PI * 2);
    ctx.fill();
  }
  ctx.restore();
};

const drawSphere3 = (ctx, center, radius, stroke, fill) => {
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

const drawTriangle3 = (ctx, triangle, stroke, fill) => {
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

const drawRay3 = (ctx, ray, stroke, hitFill) => {
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

  if (ray.hit_point) {
    const hit = isoProject(ray.hit_point);
    ctx.fillStyle = hitFill;
    ctx.beginPath();
    ctx.arc(hit[0], hit[1], 6, 0, Math.PI * 2);
    ctx.fill();

    if (ray.hit_normal) {
      const normalTip = isoProject([
        ray.hit_point[0] + ray.hit_normal[0] * 4,
        ray.hit_point[1] + ray.hit_normal[1] * 4,
        ray.hit_point[2] + ray.hit_normal[2] * 4,
      ]);
      ctx.strokeStyle = hitFill;
      ctx.lineWidth = 2;
      ctx.beginPath();
      ctx.moveTo(hit[0], hit[1]);
      ctx.lineTo(normalTip[0], normalTip[1]);
      ctx.stroke();
    }
  }
};

const drawScene3D = () => {
  if (!state3d) return;
  scene3Ctx.clearRect(0, 0, scene3Canvas.width, scene3Canvas.height);
  scene3Ctx.fillStyle = "#f8fafc";
  scene3Ctx.fillRect(0, 0, scene3Canvas.width, scene3Canvas.height);
  drawGrid(scene3Ctx, scene3Canvas.width, scene3Canvas.height, 52);

  drawWireAabb3(scene3Ctx, state3d.aabb, colors.staticStroke, "rgba(148, 163, 184, 0.09)");
  drawWireAabb3(scene3Ctx, state3d.sensor, colors.sensor, colors.sensorFill, true);
  drawTriangle3(
    scene3Ctx,
    state3d.triangle,
    state3d.flags.triangle_hits_aabb ? colors.hit : colors.triangleStroke,
    colors.triangleFill,
  );
  drawTriangle3(
    scene3Ctx,
    state3d.triangle_peer,
    state3d.flags.triangle_hits_triangle ? colors.trianglePeer : colors.staticStroke,
    "rgba(124, 58, 237, 0.10)",
  );
  drawTriangle3(
    scene3Ctx,
    state3d.triangle_distance_peer,
    colors.trianglePeerDistance,
    "rgba(147, 51, 234, 0.08)",
  );
  drawWireAabb3(
    scene3Ctx,
    state3d.triangle_distance_aabb,
    colors.triangleAabbDistance,
    "rgba(192, 38, 211, 0.08)",
  );
  drawWireAabb3(
    scene3Ctx,
    state3d.triangle_vertex_distance_aabb,
    colors.triangleAabbDistanceVertex,
    "rgba(162, 28, 175, 0.08)",
  );
  drawSphere3(
    scene3Ctx,
    state3d.triangle_distance_sphere_center,
    state3d.triangle_distance_sphere_radius,
    colors.sphereTriangleDistance,
    "rgba(220, 38, 38, 0.08)",
  );
  drawSphere3(
    scene3Ctx,
    state3d.triangle_skew_distance_sphere_center,
    state3d.triangle_skew_distance_sphere_radius,
    colors.sphereTriangleDistanceSkew,
    "rgba(153, 27, 27, 0.08)",
  );
  drawSphere3(
    scene3Ctx,
    state3d.triangle_vertex_distance_sphere_center,
    state3d.triangle_vertex_distance_sphere_radius,
    colors.sphereTriangleDistanceVertex,
    "rgba(185, 28, 28, 0.08)",
  );
  drawCapsule3(
    scene3Ctx,
    state3d.triangle_distance_capsule,
    colors.capsuleTriangleDistance,
    "rgba(29, 78, 216, 0.08)",
  );
  drawCapsule3(
    scene3Ctx,
    state3d.triangle_skew_distance_capsule,
    colors.capsuleTriangleDistanceSkew,
    "rgba(30, 58, 138, 0.08)",
  );
  drawTriangle3(
    scene3Ctx,
    state3d.triangle_sweep_peer,
    colors.sweepTriangleTriangle,
    "rgba(124, 45, 18, 0.08)",
  );
  drawTriangle3(
    scene3Ctx,
    state3d.triangle_sweep_triangle,
    colors.sweepTriangleTriangle,
    "rgba(124, 45, 18, 0.12)",
  );
  drawSegment3(
    scene3Ctx,
    state3d.triangle_segment,
    state3d.flags.segment_hits_triangle ? colors.triangleSegment : colors.staticStroke,
  );
  drawSegment3(scene3Ctx, state3d.triangle_distance_segment, colors.triangleDistance);
  drawCapsule3(scene3Ctx, state3d.capsule, colors.staticStroke, colors.staticFill);
  drawRay3(scene3Ctx, state3d.aabb_ray, colors.ray, colors.hit);
  drawRay3(scene3Ctx, state3d.sphere_ray, colors.rayRound, colors.rayRound);
  drawRay3(scene3Ctx, state3d.capsule_ray, colors.rayCapsule, colors.rayCapsule);
  drawRay3(scene3Ctx, state3d.triangle_ray, colors.rayTriangle, colors.rayTriangle);
  drawRay3(scene3Ctx, state3d.triangle_sweep, colors.sweepTriangle, colors.sweepTriangle);
  drawCapsule3(
    scene3Ctx,
    state3d.triangle_capsule_sweep_capsule,
    colors.sweepCapsuleTriangle,
    "rgba(190, 18, 60, 0.10)",
  );
  drawRay3(
    scene3Ctx,
    state3d.triangle_capsule_sweep,
    colors.sweepCapsuleTriangle,
    colors.sweepCapsuleTriangle,
  );
  drawSegment3(
    scene3Ctx,
    state3d.triangle_segment_sweep_segment,
    colors.sweepSegmentTriangle,
  );
  drawRay3(
    scene3Ctx,
    state3d.triangle_segment_sweep,
    colors.sweepSegmentTriangle,
    colors.sweepSegmentTriangle,
  );
  drawRay3(
    scene3Ctx,
    state3d.triangle_triangle_sweep,
    colors.sweepTriangleTriangle,
    colors.sweepTriangleTriangle,
  );

  const probe = isoProject(state3d.probe);
  const closest = isoProject(state3d.triangle_closest_point);
  const overlapClosest = isoProject(state3d.triangle_segment_closest_point);
  const distanceSegmentPoint = isoProject(state3d.triangle_distance_segment_point);
  const distanceTrianglePoint = isoProject(state3d.triangle_distance_triangle_point);
  const trianglePeerDistancePointA = isoProject(state3d.triangle_peer_distance_point_a);
  const trianglePeerDistancePointB = isoProject(state3d.triangle_peer_distance_point_b);
  const triangleAabbDistancePointTriangle = isoProject(state3d.triangle_aabb_distance_point_triangle);
  const triangleAabbDistancePointAabb = isoProject(state3d.triangle_aabb_distance_point_aabb);
  const triangleVertexAabbDistancePointTriangle = isoProject(state3d.triangle_vertex_aabb_distance_point_triangle);
  const triangleVertexAabbDistancePointAabb = isoProject(state3d.triangle_vertex_aabb_distance_point_aabb);
  const triangleSphereDistancePointSphere = isoProject(state3d.triangle_sphere_distance_point_sphere);
  const triangleSphereDistancePointTriangle = isoProject(state3d.triangle_sphere_distance_point_triangle);
  const triangleCapsuleDistancePointCapsule = isoProject(state3d.triangle_capsule_distance_point_capsule);
  const triangleCapsuleDistancePointTriangle = isoProject(state3d.triangle_capsule_distance_point_triangle);
  const triangleSkewSphereDistancePointSphere = isoProject(state3d.triangle_skew_sphere_distance_point_sphere);
  const triangleSkewSphereDistancePointTriangle = isoProject(state3d.triangle_skew_sphere_distance_point_triangle);
  const triangleSkewCapsuleDistancePointCapsule = isoProject(state3d.triangle_skew_capsule_distance_point_capsule);
  const triangleSkewCapsuleDistancePointTriangle = isoProject(state3d.triangle_skew_capsule_distance_point_triangle);
  const triangleVertexSphereDistancePointSphere = isoProject(state3d.triangle_vertex_sphere_distance_point_sphere);
  const triangleVertexSphereDistancePointTriangle = isoProject(state3d.triangle_vertex_sphere_distance_point_triangle);
  scene3Ctx.save();
  scene3Ctx.setLineDash([6, 5]);
  scene3Ctx.strokeStyle = colors.triangleStroke;
  scene3Ctx.lineWidth = 1.5;
  scene3Ctx.beginPath();
  scene3Ctx.moveTo(probe[0], probe[1]);
  scene3Ctx.lineTo(closest[0], closest[1]);
  scene3Ctx.stroke();
  scene3Ctx.restore();

  scene3Ctx.save();
  scene3Ctx.setLineDash([3, 5]);
  scene3Ctx.strokeStyle = colors.trianglePeerDistance;
  scene3Ctx.lineWidth = 2;
  scene3Ctx.beginPath();
  scene3Ctx.moveTo(trianglePeerDistancePointA[0], trianglePeerDistancePointA[1]);
  scene3Ctx.lineTo(trianglePeerDistancePointB[0], trianglePeerDistancePointB[1]);
  scene3Ctx.stroke();
  scene3Ctx.restore();

  scene3Ctx.save();
  scene3Ctx.setLineDash([6, 3]);
  scene3Ctx.strokeStyle = colors.triangleAabbDistance;
  scene3Ctx.lineWidth = 2;
  scene3Ctx.beginPath();
  scene3Ctx.moveTo(triangleAabbDistancePointTriangle[0], triangleAabbDistancePointTriangle[1]);
  scene3Ctx.lineTo(triangleAabbDistancePointAabb[0], triangleAabbDistancePointAabb[1]);
  scene3Ctx.stroke();
  scene3Ctx.restore();

  scene3Ctx.save();
  scene3Ctx.setLineDash([2, 6]);
  scene3Ctx.strokeStyle = colors.triangleAabbDistanceVertex;
  scene3Ctx.lineWidth = 2;
  scene3Ctx.beginPath();
  scene3Ctx.moveTo(triangleVertexAabbDistancePointTriangle[0], triangleVertexAabbDistancePointTriangle[1]);
  scene3Ctx.lineTo(triangleVertexAabbDistancePointAabb[0], triangleVertexAabbDistancePointAabb[1]);
  scene3Ctx.stroke();
  scene3Ctx.restore();

  scene3Ctx.save();
  scene3Ctx.setLineDash([4, 4]);
  scene3Ctx.strokeStyle = colors.triangleDistance;
  scene3Ctx.lineWidth = 2;
  scene3Ctx.beginPath();
  scene3Ctx.moveTo(distanceSegmentPoint[0], distanceSegmentPoint[1]);
  scene3Ctx.lineTo(distanceTrianglePoint[0], distanceTrianglePoint[1]);
  scene3Ctx.stroke();
  scene3Ctx.restore();

  scene3Ctx.save();
  scene3Ctx.setLineDash([5, 4]);
  scene3Ctx.strokeStyle = colors.sphereTriangleDistance;
  scene3Ctx.lineWidth = 2;
  scene3Ctx.beginPath();
  scene3Ctx.moveTo(triangleSphereDistancePointSphere[0], triangleSphereDistancePointSphere[1]);
  scene3Ctx.lineTo(triangleSphereDistancePointTriangle[0], triangleSphereDistancePointTriangle[1]);
  scene3Ctx.stroke();
  scene3Ctx.restore();

  scene3Ctx.save();
  scene3Ctx.setLineDash([2, 5]);
  scene3Ctx.strokeStyle = colors.capsuleTriangleDistance;
  scene3Ctx.lineWidth = 2;
  scene3Ctx.beginPath();
  scene3Ctx.moveTo(triangleCapsuleDistancePointCapsule[0], triangleCapsuleDistancePointCapsule[1]);
  scene3Ctx.lineTo(triangleCapsuleDistancePointTriangle[0], triangleCapsuleDistancePointTriangle[1]);
  scene3Ctx.stroke();
  scene3Ctx.restore();

  scene3Ctx.save();
  scene3Ctx.setLineDash([8, 4]);
  scene3Ctx.strokeStyle = colors.sphereTriangleDistanceSkew;
  scene3Ctx.lineWidth = 2;
  scene3Ctx.beginPath();
  scene3Ctx.moveTo(triangleSkewSphereDistancePointSphere[0], triangleSkewSphereDistancePointSphere[1]);
  scene3Ctx.lineTo(triangleSkewSphereDistancePointTriangle[0], triangleSkewSphereDistancePointTriangle[1]);
  scene3Ctx.stroke();
  scene3Ctx.restore();

  scene3Ctx.save();
  scene3Ctx.setLineDash([7, 3]);
  scene3Ctx.strokeStyle = colors.sphereTriangleDistanceVertex;
  scene3Ctx.lineWidth = 2;
  scene3Ctx.beginPath();
  scene3Ctx.moveTo(triangleVertexSphereDistancePointSphere[0], triangleVertexSphereDistancePointSphere[1]);
  scene3Ctx.lineTo(triangleVertexSphereDistancePointTriangle[0], triangleVertexSphereDistancePointTriangle[1]);
  scene3Ctx.stroke();
  scene3Ctx.restore();

  scene3Ctx.save();
  scene3Ctx.setLineDash([1, 4]);
  scene3Ctx.strokeStyle = colors.capsuleTriangleDistanceSkew;
  scene3Ctx.lineWidth = 2;
  scene3Ctx.beginPath();
  scene3Ctx.moveTo(triangleSkewCapsuleDistancePointCapsule[0], triangleSkewCapsuleDistancePointCapsule[1]);
  scene3Ctx.lineTo(triangleSkewCapsuleDistancePointTriangle[0], triangleSkewCapsuleDistancePointTriangle[1]);
  scene3Ctx.stroke();
  scene3Ctx.restore();

  scene3Ctx.fillStyle = state3d.flags.probe_in_capsule || state3d.flags.probe_on_triangle
    ? colors.hit
    : colors.dynamic;
  scene3Ctx.beginPath();
  scene3Ctx.arc(probe[0], probe[1], 5, 0, Math.PI * 2);
  scene3Ctx.fill();

  scene3Ctx.fillStyle = colors.triangleStroke;
  scene3Ctx.beginPath();
  scene3Ctx.arc(closest[0], closest[1], 4, 0, Math.PI * 2);
  scene3Ctx.fill();

  scene3Ctx.fillStyle = colors.triangleSegment;
  scene3Ctx.beginPath();
  scene3Ctx.arc(overlapClosest[0], overlapClosest[1], 4, 0, Math.PI * 2);
  scene3Ctx.fill();

  scene3Ctx.fillStyle = colors.triangleDistance;
  for (const point of [distanceSegmentPoint, distanceTrianglePoint]) {
    scene3Ctx.beginPath();
    scene3Ctx.arc(point[0], point[1], 4, 0, Math.PI * 2);
    scene3Ctx.fill();
  }

  scene3Ctx.fillStyle = colors.trianglePeerDistance;
  for (const point of [trianglePeerDistancePointA, trianglePeerDistancePointB]) {
    scene3Ctx.beginPath();
    scene3Ctx.arc(point[0], point[1], 4, 0, Math.PI * 2);
    scene3Ctx.fill();
  }

  scene3Ctx.fillStyle = colors.triangleAabbDistance;
  for (const point of [triangleAabbDistancePointTriangle, triangleAabbDistancePointAabb]) {
    scene3Ctx.beginPath();
    scene3Ctx.arc(point[0], point[1], 4, 0, Math.PI * 2);
    scene3Ctx.fill();
  }

  scene3Ctx.fillStyle = colors.triangleAabbDistanceVertex;
  for (const point of [triangleVertexAabbDistancePointTriangle, triangleVertexAabbDistancePointAabb]) {
    scene3Ctx.beginPath();
    scene3Ctx.arc(point[0], point[1], 4, 0, Math.PI * 2);
    scene3Ctx.fill();
  }

  scene3Ctx.fillStyle = colors.sphereTriangleDistance;
  for (const point of [triangleSphereDistancePointSphere, triangleSphereDistancePointTriangle]) {
    scene3Ctx.beginPath();
    scene3Ctx.arc(point[0], point[1], 4, 0, Math.PI * 2);
    scene3Ctx.fill();
  }

  scene3Ctx.fillStyle = colors.capsuleTriangleDistance;
  for (const point of [triangleCapsuleDistancePointCapsule, triangleCapsuleDistancePointTriangle]) {
    scene3Ctx.beginPath();
    scene3Ctx.arc(point[0], point[1], 4, 0, Math.PI * 2);
    scene3Ctx.fill();
  }

  scene3Ctx.fillStyle = colors.sphereTriangleDistanceSkew;
  for (const point of [triangleSkewSphereDistancePointSphere, triangleSkewSphereDistancePointTriangle]) {
    scene3Ctx.beginPath();
    scene3Ctx.arc(point[0], point[1], 4, 0, Math.PI * 2);
    scene3Ctx.fill();
  }

  scene3Ctx.fillStyle = colors.sphereTriangleDistanceVertex;
  for (const point of [triangleVertexSphereDistancePointSphere, triangleVertexSphereDistancePointTriangle]) {
    scene3Ctx.beginPath();
    scene3Ctx.arc(point[0], point[1], 4, 0, Math.PI * 2);
    scene3Ctx.fill();
  }

  scene3Ctx.fillStyle = colors.capsuleTriangleDistanceSkew;
  for (const point of [triangleSkewCapsuleDistancePointCapsule, triangleSkewCapsuleDistancePointTriangle]) {
    scene3Ctx.beginPath();
    scene3Ctx.arc(point[0], point[1], 4, 0, Math.PI * 2);
    scene3Ctx.fill();
  }

  drawSphere3(
    scene3Ctx,
    state3d.triangle_sweep.origin,
    state3d.triangle_sweep_radius,
    colors.sweepTriangle,
    "rgba(234, 88, 12, 0.08)",
  );

  drawSphere3(
    scene3Ctx,
    state3d.sphere_center,
    state3d.sphere_radius,
    state3d.flags.sphere_hits_aabb || state3d.flags.sphere_hits_capsule || state3d.flags.sphere_hits_triangle
      ? colors.hit
      : colors.dynamic,
    colors.dynamicFill,
  );

  const rayHitCount3D = [
    state3d.flags.ray_hits_aabb,
    state3d.flags.ray_hits_sphere,
    state3d.flags.ray_hits_capsule,
    state3d.flags.ray_hits_triangle,
  ].filter(Boolean).length;
  const sweepHitCount3D = [
    state3d.flags.sweep_hits_triangle,
    state3d.flags.capsule_sweep_hits_triangle,
    state3d.flags.segment_sweep_hits_triangle,
    state3d.flags.triangle_sweep_hits_triangle,
  ].filter(Boolean).length;
  scene3Summary.textContent = `${rayHitCount3D}/4 raycasts live | ${sweepHitCount3D}/4 triangle sweeps live`;
};

const renderFlags = (root, entries) => {
  root.innerHTML = "";
  Object.entries(entries).forEach(([name, value]) => {
    const item = document.createElement("li");
    item.className = value ? "flag-on" : "flag-off";
    item.textContent = `${name}: ${value ? "true" : "false"}`;
    root.append(item);
  });
};

const renderCandidates = (root, ids) => {
  root.innerHTML = "";
  if (!ids.length) {
    const empty = document.createElement("span");
    empty.className = "chip ghost";
    empty.textContent = "none";
    root.append(empty);
    return;
  }
  ids.forEach((id) => {
    const chip = document.createElement("span");
    chip.className = "chip";
    chip.textContent = `#${id}`;
    root.append(chip);
  });
};

const refreshState = () => {
  state2d = JSON.parse(demo2d.render_state());
  state3d = JSON.parse(demo3d.render_state());
  drawScene2D();
  drawScene3D();
  renderFlags(scene2Flags, state2d.flags);
  renderFlags(scene3Flags, state3d.flags);
  renderCandidates(scene2Candidates, state2d.candidate_ids);
  renderCandidates(scene3Candidates, state3d.candidate_ids);
  setStatus(
    "WASM core: ready",
    `2D tick=${state2d.tick} | 3D tick=${state3d.tick}`,
    false,
  );
};

const boot = async () => {
  try {
    const wasmModule = await import("./pkg/bumpbox_demo_wasm.js");
    await wasmModule.default();
    demo2d = new wasmModule.Demo2D();
    demo3d = new wasmModule.Demo3D();
    wasmReady = true;
    refreshState();
  } catch (error) {
    console.warn(error);
    wasmReady = false;
    setStatus("WASM core: unavailable (run wasm-pack build)", "demo-wasm/www expects ./pkg output", true);
  }
};

speedRange.addEventListener("input", () => {
  playbackSpeed = Number(speedRange.value);
  speedValue.textContent = `${playbackSpeed}x`;
});

playButton.addEventListener("click", () => {
  isPlaying = true;
});

pauseButton.addEventListener("click", () => {
  isPlaying = false;
});

resetButton.addEventListener("click", () => {
  if (!wasmReady) return;
  demo2d.reset();
  demo3d.reset();
  refreshState();
});

const frame = (ts) => {
  const dt = ts - lastFrame;
  lastFrame = ts;

  if (wasmReady && isPlaying) {
    accumulator += dt;
    const stepMs = 1000 / 60;
    while (accumulator >= stepMs) {
      for (let i = 0; i < playbackSpeed; i += 1) {
        demo2d.tick();
        demo3d.tick();
      }
      accumulator -= stepMs;
    }
    refreshState();
  }

  requestAnimationFrame(frame);
};

void boot();
requestAnimationFrame(frame);
