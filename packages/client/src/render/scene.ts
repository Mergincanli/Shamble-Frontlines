import * as THREE from "three/webgpu";
import type { Bounds, Ground, MapDocument, Region, Shape } from "@shamble/types";

// The scene: lights, a camera, and one floor group that `setContent` swaps. The empty floor (a lit 30 m ground with a
// 1 m grid) is drawn until the map's arrives, and stays drawn when the content is refused. The map's floor is the
// greybox of LOOP_V2 section 9's default, "plain blocks, with height read through brightness (higher is lighter)", in
// neutral greys (his GREY-COLOURS (a)). Every colour and light here is a plain placeholder that decides nothing: the
// look is LOOK_V2's. Node materials only (ADR 0003 part 1). Nothing here runs per frame.

/**
 * The empty ground's size in metres, and so the number of 1 m grid squares along each side. With the camera below, the
 * whole ground lies in front of the camera: a grid line that runs from in front of the camera to behind it is not
 * drawn at all by a software renderer (seen in headless Chromium, 2026-11-19).
 */
const GROUND_M = 30;

/** The greys of the map's blocks, from the lowest top to the tallest: one channel value, the same in red, green and blue. */
const BLOCK_LOW = 0x70;
const BLOCK_HIGH = 0xd8;
/** The map's ground, under its patches, and the two kinds of patch: each darker than every block. */
const MAP_GROUND = 0x3c3c3c;
const PATCH_GREYS: Readonly<Record<Ground, number>> = { street: 0x404040, grass: 0x505050 };
/** The grid's colour: the empty scene's. */
const GRID = 0x2c323a;

/** What each object of a floor is, for the development inventory (render/inventory.ts). */
export type FloorKind = "ground" | "box" | "ramp" | "region" | "grid";

/** A floor's objects, each named by its map id and marked with its kind; `setContent` swaps the whole group. */
export type Floor = THREE.Group;

/** The scene, with the empty floor in it, and the camera that frames it. */
export function buildEmptyScene(): { scene: THREE.Scene; camera: THREE.PerspectiveCamera; floor: Floor } {
  const scene = new THREE.Scene();
  scene.background = new THREE.Color(0x20242a);

  const floor: Floor = new THREE.Group();
  const ground = new THREE.Mesh(
    new THREE.PlaneGeometry(GROUND_M, GROUND_M),
    // Pushed back a little in depth, so the grid's lines on it are never hidden by it.
    new THREE.MeshStandardNodeMaterial({
      color: 0x6b7480,
      roughness: 1,
      polygonOffset: true,
      polygonOffsetFactor: 1,
      polygonOffsetUnits: 1,
    }),
  );
  ground.rotation.x = -Math.PI / 2;
  mark(ground, "ground", "ground");
  floor.add(ground);

  // One line per metre in both directions.
  const half = GROUND_M / 2;
  const points: number[] = [];
  for (let i = -half; i <= half; i += 1) {
    points.push(i, 0, -half, i, 0, half, -half, 0, i, half, 0, i);
  }
  const grid = new THREE.BufferGeometry();
  grid.setAttribute("position", new THREE.Float32BufferAttribute(points, 3));
  floor.add(mark(new THREE.LineSegments(grid, new THREE.LineBasicNodeMaterial({ color: GRID })), "grid", "grid"));
  scene.add(floor);

  const sun = new THREE.DirectionalLight(0xffffff, 2.5);
  sun.position.set(15, 30, 10);
  scene.add(sun, new THREE.HemisphereLight(0xdfe8f0, 0x40444a, 1));

  const camera = new THREE.PerspectiveCamera(50, 1, 0.1, 500);
  camera.position.set(0, 14, 16);
  camera.lookAt(0, 0, 0);
  return { scene, camera, floor };
}

function mark<T extends THREE.Object3D>(object: T, id: string, kind: FloorKind): T {
  object.name = id;
  object.userData = { kind };
  return object;
}

/**
 * A solid's distinct corners, from the map's own numbers: a box's eight; a ramp's six, the four corners of its floor
 * and the two top corners on the side it rises toward, as the simulation's ray cast takes it (crates/sim/src/
 * collision.rs, `enter_ramp`). Written into the geometry with no transform, so each drawn corner is the map's number
 * rounded to f32, as the simulation's own corners are.
 */
export function solidPoints(shape: Shape): [number, number, number][] {
  const [x0, y0, z0] = shape.min;
  const [x1, y1, z1] = shape.max;
  const floor: [number, number, number][] = [
    [x0, y0, z0],
    [x1, y0, z0],
    [x1, y0, z1],
    [x0, y0, z1],
  ];
  if (shape.type === "box") return [...floor, [x0, y1, z0], [x1, y1, z0], [x1, y1, z1], [x0, y1, z1]];
  switch (shape.rises_toward) {
    case "+x":
      return [...floor, [x1, y1, z0], [x1, y1, z1]];
    case "-x":
      return [...floor, [x0, y1, z0], [x0, y1, z1]];
    case "+z":
      return [...floor, [x0, y1, z1], [x1, y1, z1]];
    case "-z":
      return [...floor, [x0, y1, z0], [x1, y1, z0]];
  }
}

/**
 * A solid's faces as lists of indices into `solidPoints`: a box's six quads; a ramp's floor, its high wall, its slope
 * (quads) and its two sides (triangles). Which way each face turns is settled by `solidGeometry`, outward.
 */
function solidFaces(shape: Shape): number[][] {
  // Floor corners: 0 (x0, z0), 1 (x1, z0), 2 (x1, z1), 3 (x0, z1); a box's tops 4 to 7 above them.
  if (shape.type === "box") {
    return [
      [0, 1, 2, 3],
      [4, 5, 6, 7],
      [0, 1, 5, 4],
      [1, 2, 6, 5],
      [2, 3, 7, 6],
      [3, 0, 4, 7],
    ];
  }
  // A ramp's two top corners, 4 and 5, stand over two floor corners: the high wall joins them, the slope joins the
  // top to the other two floor corners, and a triangle closes each side.
  const [a, b, c, d] =
    shape.rises_toward === "+x" ? [1, 2, 0, 3] : shape.rises_toward === "-x" ? [0, 3, 1, 2] : shape.rises_toward === "+z" ? [3, 2, 0, 1] : [0, 1, 3, 2];
  // a and b: the floor corners under tops 4 and 5; c and d: the floor corners on the low side, c beside a, d beside b.
  return [[0, 1, 2, 3], [a, b, 5, 4], [c, d, 5, 4], [c, a, 4], [d, b, 5]];
}

/**
 * Triangles from faces, each turned so that its normal (by the right-hand rule) points away from the solid's centre,
 * and given that normal: flat faces, so a side reads darker than a top under the light.
 */
function solidGeometry(points: readonly (readonly [number, number, number])[], faces: readonly number[][]): THREE.BufferGeometry {
  const centre = [0, 1, 2].map((axis) => points.reduce((sum, point) => sum + point[axis]!, 0) / points.length);
  const positions: number[] = [];
  const normals: number[] = [];
  for (const face of faces) {
    for (let k = 1; k + 1 < face.length; k += 1) {
      let [p, q, r] = [points[face[0]!]!, points[face[k]!]!, points[face[k + 1]!]!];
      let n = cross(sub(q, p), sub(r, p));
      if (dot(n, sub(p, centre)) < 0) {
        [q, r] = [r, q];
        n = [-n[0], -n[1], -n[2]];
      }
      const length = Math.hypot(n[0], n[1], n[2]);
      positions.push(...p, ...q, ...r);
      for (let v = 0; v < 3; v += 1) normals.push(n[0] / length, n[1] / length, n[2] / length);
    }
  }
  const geometry = new THREE.BufferGeometry();
  geometry.setAttribute("position", new THREE.Float32BufferAttribute(positions, 3));
  geometry.setAttribute("normal", new THREE.Float32BufferAttribute(normals, 3));
  return geometry;
}

function sub(a: readonly number[], b: readonly number[]): [number, number, number] {
  return [a[0]! - b[0]!, a[1]! - b[1]!, a[2]! - b[2]!];
}
function cross(a: readonly number[], b: readonly number[]): [number, number, number] {
  return [a[1]! * b[2]! - a[2]! * b[1]!, a[2]! * b[0]! - a[0]! * b[2]!, a[0]! * b[1]! - a[1]! * b[0]!];
}
function dot(a: readonly number[], b: readonly number[]): number {
  return a[0]! * b[0]! + a[1]! * b[1]! + a[2]! * b[2]!;
}

/** A flat quad on the floor, facing up, from its lowest to its highest corner in x and z. */
function floorQuad(x0: number, z0: number, x1: number, z1: number, y: number): THREE.BufferGeometry {
  const positions = [x0, y, z0, x0, y, z1, x1, y, z1, x0, y, z0, x1, y, z1, x1, y, z0];
  const geometry = new THREE.BufferGeometry();
  geometry.setAttribute("position", new THREE.Float32BufferAttribute(positions, 3));
  geometry.setAttribute("normal", new THREE.Float32BufferAttribute([0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0], 3));
  return geometry;
}

/** A block's grey by its top: the lowest top darkest, the tallest lightest, neutral (red, green and blue equal). */
export function greyOf(top: number, lowest: number, tallest: number): number {
  const share = tallest > lowest ? (top - lowest) / (tallest - lowest) : 1;
  const level = Math.round(BLOCK_LOW + share * (BLOCK_HIGH - BLOCK_LOW));
  return level * 0x010101;
}

/** A patch of ground's grey by its kind (his STREET-GREY, default (b): two neutral greys, both darker than every block). */
export function groundGrey(kind: Ground): number {
  return PATCH_GREYS[kind];
}

/**
 * The map's floor: its ground and a grid of one line per metre over its bounds, each patch of ground, and each shape,
 * by the map's own corners. Spawns are not drawn. Materials are made here, one per grey, and belong to this floor
 * alone, so freeing it frees nothing another floor uses.
 */
export function buildMapFloor(map: MapDocument): Floor {
  const floor: Floor = new THREE.Group();
  const [minX, floorY, minZ] = map.bounds.min;
  const [maxX, , maxZ] = map.bounds.max;
  const materials = new Map<number, THREE.MeshStandardNodeMaterial>();
  const material = (grey: number, push: number): THREE.MeshStandardNodeMaterial => {
    const key = grey * 4 + push;
    let made = materials.get(key);
    if (made === undefined) {
      // A floor's own pieces are pushed back in depth, the ground most, so the grid on them is never hidden.
      made = new THREE.MeshStandardNodeMaterial({ color: grey, roughness: 1, polygonOffset: push > 0, polygonOffsetFactor: push, polygonOffsetUnits: push });
      materials.set(key, made);
    }
    return made;
  };
  floor.add(mark(new THREE.Mesh(floorQuad(minX, minZ, maxX, maxZ, floorY), material(MAP_GROUND, 2)), "ground", "ground"));
  for (const region of map.regions) floor.add(regionMesh(region, floorY, material(groundGrey(region.kind), 1)));
  const tops = map.shapes.map((shape) => shape.max[1]);
  const lowest = Math.min(...tops);
  const tallest = Math.max(...tops);
  for (const shape of map.shapes) {
    const mesh = new THREE.Mesh(solidGeometry(solidPoints(shape), solidFaces(shape)), material(greyOf(shape.max[1], lowest, tallest), 0));
    floor.add(mark(mesh, shape.id, shape.type));
  }
  floor.add(mark(new THREE.LineSegments(mapGrid(map.bounds), new THREE.LineBasicNodeMaterial({ color: GRID })), "grid", "grid"));
  return floor;
}

function regionMesh(region: Region, y: number, material: THREE.MeshStandardNodeMaterial): THREE.Mesh {
  const [x0, z0] = region.min;
  const [x1, z1] = region.max;
  return mark(new THREE.Mesh(floorQuad(x0, z0, x1, z1, y), material), region.id, "region");
}

/** One line per whole metre over the bounds, both ways, on the floor. */
function mapGrid(bounds: Bounds): THREE.BufferGeometry {
  const [minX, y, minZ] = bounds.min;
  const [maxX, , maxZ] = bounds.max;
  const points: number[] = [];
  for (let x = Math.ceil(minX); x <= maxX; x += 1) points.push(x, y, minZ, x, y, maxZ);
  for (let z = Math.ceil(minZ); z <= maxZ; z += 1) points.push(minX, y, z, maxX, y, z);
  const grid = new THREE.BufferGeometry();
  grid.setAttribute("position", new THREE.Float32BufferAttribute(points, 3));
  return grid;
}

/** Frees a floor's geometries and materials, each once. */
export function disposeFloor(floor: Floor): void {
  const materials = new Set<THREE.Material>();
  for (const object of floor.children) {
    if (object instanceof THREE.Mesh || object instanceof THREE.LineSegments) {
      object.geometry.dispose();
      materials.add(object.material as THREE.Material);
    }
  }
  for (const material of materials) material.dispose();
}

/** The camera's way of looking: today's, from the side and above at the same angle, at the floor's middle. */
const VIEW_DIRECTION = new THREE.Vector3(0, 14, 16).normalize();
/** How far into the picture's edges the floor's corners may come, as a share of the half-width (and half-height). */
const VIEW_MARGIN = 0.92;

/**
 * Where the camera stands to show the whole map: on today's line of sight to the floor's middle, as near as lets the
 * floor's four corners lie inside the picture at this aspect. Every grid line then lies in front of the camera.
 */
export function overview(bounds: Bounds, aspect: number, fovDeg = 50): { position: [number, number, number]; target: [number, number, number] } {
  const [minX, y, minZ] = bounds.min;
  const [maxX, , maxZ] = bounds.max;
  const target = new THREE.Vector3((minX + maxX) / 2, y, (minZ + maxZ) / 2);
  const corners = [
    new THREE.Vector3(minX, y, minZ),
    new THREE.Vector3(maxX, y, minZ),
    new THREE.Vector3(maxX, y, maxZ),
    new THREE.Vector3(minX, y, maxZ),
  ];
  const camera = new THREE.PerspectiveCamera(fovDeg, aspect, 0.1, 1e6);
  const projected = new THREE.Vector3();
  const fits = (distance: number): boolean => {
    camera.position.copy(target).addScaledVector(VIEW_DIRECTION, distance);
    camera.lookAt(target);
    camera.updateMatrixWorld(true);
    return corners.every((corner) => {
      projected.copy(corner).project(camera);
      return Math.abs(projected.x) <= VIEW_MARGIN && Math.abs(projected.y) <= VIEW_MARGIN && projected.z < 1;
    });
  };
  // The nearest distance that fits, by halving: a fixed number of steps, so the same bounds give the same camera.
  let near = 0;
  let far = 1;
  while (!fits(far)) far *= 2;
  for (let step = 0; step < 40; step += 1) {
    const middle = (near + far) / 2;
    if (fits(middle)) far = middle;
    else near = middle;
  }
  const position = target.clone().addScaledVector(VIEW_DIRECTION, far);
  return { position: [position.x, position.y, position.z], target: [target.x, target.y, target.z] };
}
