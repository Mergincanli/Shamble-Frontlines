import * as THREE from "three/webgpu";
import type { Bounds, MapDocument, Shape } from "@shamble/types";
import { expect, test } from "vitest";
import { INVENTORY_UNKNOWN, readInventory } from "./inventory.ts";
import { VIEW_MARGIN, buildEmptyScene, buildMapFloor, disposeFloor, frameCamera, greyOf, groundGrey, overview, solidPoints } from "./scene.ts";

// The scene's floors in plain Node: three's objects are made and read without a renderer.

const box = (id: string, min: [number, number, number], max: [number, number, number]): Shape => ({ id, min, max, type: "box" });
const ramp = (id: string, min: [number, number, number], max: [number, number, number], rises_toward: "+x" | "-x" | "+z" | "-z"): Shape => ({
  id,
  min,
  max,
  type: "ramp",
  rises_toward,
});

/** A made map: a box, a ramp toward each of the four sides, two patches of ground, and the test scene's bounds. */
const MAP: MapDocument = {
  id: "made",
  bounds: { min: [-32, 0, -24], max: [80, 20, 24] },
  spawns: [{ id: "one", at: [0, 0, 0], facing_deg: 90 }],
  shapes: [
    box("wall", [0.1, 0, 0.3], [2.7, 2.55, 0.5]),
    ramp("ramp_px", [10, 0, 0], [14, 1, 2], "+x"),
    ramp("ramp_mx", [20, 0, 0], [24, 1.5, 2], "-x"),
    ramp("ramp_pz", [30, 0, 0], [32, 0.55, 4], "+z"),
    ramp("ramp_mz", [40, 0, 0], [42, 4.7, 4], "-z"),
  ],
  regions: [
    { id: "street", kind: "street", min: [-32, -2.15], max: [80, 2.15] },
    { id: "grass", kind: "grass", min: [-32, 2.15], max: [80, 24] },
  ],
};

/**
 * The made map raised 2 m: its bounds' bottom, and so its ground, patches and solids' floors, off the plane y = 0
 * (step S's review: G-3, a ground drawn at 0 and not at the bounds' bottom; GE-F5, faces on a plane through the origin
 * add nothing to the enclosed volume).
 */
const up = (point: readonly number[]): [number, number, number] => [point[0]!, point[1]! + 2, point[2]!];
const RAISED: MapDocument = {
  ...MAP,
  id: "raised",
  bounds: { min: up(MAP.bounds.min), max: up(MAP.bounds.max) },
  spawns: MAP.spawns.map((spawn) => ({ ...spawn, at: up(spawn.at) })),
  shapes: MAP.shapes.map((shape) => ({ ...shape, min: up(shape.min), max: up(shape.max) })),
};
/** The largest floor the loader allows: the aim's reach each way, its height to the aim's top (map.rs:316-330). */
const LARGEST: Bounds = { min: [-327.67, 0, -327.67], max: [327.67, 655.35, 327.67] };

const f = Math.fround;
const sorted = (points: readonly (readonly number[])[]): number[][] =>
  points.map((point) => [...point]).sort((a, b) => a[0]! - b[0]! || a[1]! - b[1]! || a[2]! - b[2]!);

/** The floor's mesh of one shape or patch, by its id. */
function meshOf(floor: THREE.Group, id: string): THREE.Mesh {
  const found = floor.children.find((object) => object.name === id);
  if (!(found instanceof THREE.Mesh)) throw new Error(`no mesh ${id}`);
  return found;
}

test("each solid is drawn at its own corners, rounded to f32 as the simulation's are: 8 for a box, 6 for a ramp toward each side", () => {
  const floor = buildMapFloor(MAP);
  const drawn = new Map(readInventory(floor).map((object) => [object.id, object]));
  const expected: Record<string, number[][]> = {
    wall: [
      [0.1, 0, 0.3], [0.1, 0, 0.5], [0.1, 2.55, 0.3], [0.1, 2.55, 0.5],
      [2.7, 0, 0.3], [2.7, 0, 0.5], [2.7, 2.55, 0.3], [2.7, 2.55, 0.5],
    ],
    // Each ramp: its four floor corners, and its two top corners on the side it rises toward.
    ramp_px: [[10, 0, 0], [10, 0, 2], [14, 0, 0], [14, 0, 2], [14, 1, 0], [14, 1, 2]],
    ramp_mx: [[20, 0, 0], [20, 0, 2], [20, 1.5, 0], [20, 1.5, 2], [24, 0, 0], [24, 0, 2]],
    ramp_pz: [[30, 0, 0], [30, 0, 4], [30, 0.55, 4], [32, 0, 0], [32, 0, 4], [32, 0.55, 4]],
    ramp_mz: [[40, 0, 0], [40, 0, 4], [40, 4.7, 0], [42, 0, 0], [42, 0, 4], [42, 4.7, 0]],
  };
  for (const [id, points] of Object.entries(expected)) {
    expect(drawn.get(id)?.points, id).toEqual(sorted(points.map((point) => point.map(f))));
    expect(sorted(solidPoints(MAP.shapes.find((shape) => shape.id === id)!).map((point) => point.map(f))), id).toEqual(sorted(points.map((point) => point.map(f))));
  }
  expect(drawn.get("wall")?.kind).toBe("box");
  expect(drawn.get("ramp_mz")?.kind).toBe("ramp");
});

test("every triangle of every solid faces outward, and its normal is the face's: a box and a ramp toward each side, on the made map and on it raised", () => {
  for (const map of [MAP, RAISED]) solidsFaceOutward(map);
});

function solidsFaceOutward(map: MapDocument): void {
  const floor = buildMapFloor(map);
  for (const shape of map.shapes) {
    const geometry = meshOf(floor, shape.id).geometry;
    const position = geometry.getAttribute("position");
    const normal = geometry.getAttribute("normal");
    const points = solidPoints(shape);
    const centre = new THREE.Vector3(...[0, 1, 2].map((axis) => points.reduce((sum, point) => sum + point[axis]!, 0) / points.length));
    expect(position.count, shape.id).toBe(shape.type === "box" ? 36 : 24);
    const [a, b, c, n, ab, ac] = [0, 0, 0, 0, 0, 0].map(() => new THREE.Vector3());
    // The triangles close the solid exactly: the volume they enclose (each outward triangle's signed share) is the
    // solid's own, a box's whole and a ramp's half of its box. Faces that cut through the solid, or leave a hole, do not.
    // Measured from a point off every face's plane, so that every face counts (GE-F5: from the origin, a face on a plane
    // through it adds nothing).
    const from = new THREE.Vector3(-1000.5, -1000.25, -1000.125);
    let volume = 0;
    for (let i = 0; i < position.count; i += 3) {
      a!.fromBufferAttribute(position, i).sub(from);
      b!.fromBufferAttribute(position, i + 1).sub(from);
      c!.fromBufferAttribute(position, i + 2).sub(from);
      volume += a!.dot(new THREE.Vector3().crossVectors(b!, c!)) / 6;
    }
    const [dx, dy, dz] = [0, 1, 2].map((axis) => Math.fround(shape.max[axis]!) - Math.fround(shape.min[axis]!));
    expect(volume, `${map.id} ${shape.id} volume`).toBeCloseTo((dx! * dy! * dz!) / (shape.type === "box" ? 1 : 2), 3);
    // Closed: every edge is shared by exactly two triangles, once each way (TE-2: a face drawn twice, or one missing,
    // leaves edges unpaired, which the volume alone can miss).
    const edges = new Map<string, number>();
    const at = (i: number): string => `${position.getX(i)},${position.getY(i)},${position.getZ(i)}`;
    for (let i = 0; i < position.count; i += 3) {
      for (let e = 0; e < 3; e += 1) {
        const edge = `${at(i + e)}>${at(i + ((e + 1) % 3))}`;
        edges.set(edge, (edges.get(edge) ?? 0) + 1);
      }
    }
    for (const [edge, times] of edges) {
      const [start, end] = edge.split(">");
      expect(times, `${map.id} ${shape.id} edge ${edge} drawn ${times} times`).toBe(1);
      expect(edges.get(`${end}>${start}`), `${map.id} ${shape.id} edge ${edge} has no partner`).toBe(1);
    }
    for (let i = 0; i < position.count; i += 3) {
      a!.fromBufferAttribute(position, i);
      b!.fromBufferAttribute(position, i + 1);
      c!.fromBufferAttribute(position, i + 2);
      const face = ab!.subVectors(b!, a!).cross(ac!.subVectors(c!, a!));
      // Outward by the right-hand rule: away from the solid's centre.
      expect(face.dot(new THREE.Vector3().subVectors(a!, centre)), `${shape.id} triangle ${i / 3}`).toBeGreaterThan(0);
      // Flat: each corner carries the face's own normal.
      for (let v = 0; v < 3; v += 1) {
        n!.fromBufferAttribute(normal, i + v);
        expect(n!.dot(face.clone().normalize()), `${shape.id} normal ${i / 3}`).toBeCloseTo(1, 5);
      }
    }
  }
}

test("the ground and each patch lie flat at the bounds' bottom, at their own corners, facing up: on the made map and on it raised (step S's review, TE-1, G-3)", () => {
  for (const map of [MAP, RAISED]) {
    const floor = buildMapFloor(map);
    const drawn = readInventory(floor);
    const y = map.bounds.min[1];
    const flat = (id: string, x0: number, z0: number, x1: number, z1: number): void => {
      const corners = [[x0, y, z0], [x0, y, z1], [x1, y, z0], [x1, y, z1]];
      expect(drawn.find((object) => object.id === id)?.points, `${map.id} ${id} corners`).toEqual(sorted(corners.map((point) => point.map(f))));
      const position = meshOf(floor, id).geometry.getAttribute("position");
      const normal = meshOf(floor, id).geometry.getAttribute("normal");
      expect(position.count, `${map.id} ${id}`).toBe(6);
      const [a, b, c, n] = [0, 0, 0, 0].map(() => new THREE.Vector3());
      for (let i = 0; i < position.count; i += 3) {
        a!.fromBufferAttribute(position, i);
        b!.fromBufferAttribute(position, i + 1);
        c!.fromBufferAttribute(position, i + 2);
        // Up by the right-hand rule, so it is seen from above, and its normal up.
        const face = new THREE.Vector3().subVectors(b!, a!).cross(new THREE.Vector3().subVectors(c!, a!)).normalize();
        expect(face.y, `${map.id} ${id} triangle ${i / 3} faces up`).toBeCloseTo(1, 9);
        for (let v = 0; v < 3; v += 1) expect(n!.fromBufferAttribute(normal, i + v).y, `${map.id} ${id} normal`).toBe(1);
      }
    };
    const [minX, , minZ] = map.bounds.min;
    const [maxX, , maxZ] = map.bounds.max;
    flat("ground", minX, minZ, maxX, maxZ);
    for (const region of map.regions) flat(region.id, region.min[0], region.min[1], region.max[0], region.max[1]);
  }
});

test("each shape and patch is drawn once, by its id; the floor holds nothing else but its ground and its grid; no spawn is drawn", () => {
  const floor = buildMapFloor(MAP);
  const names = floor.children.map((object) => `${String(object.userData["kind"])}:${object.name}`);
  expect(names.sort()).toEqual(
    ["ground:ground", "grid:grid", "region:street", "region:grass", "box:wall", "ramp:ramp_px", "ramp:ramp_mx", "ramp:ramp_pz", "ramp:ramp_mz"].sort(),
  );
  expect(readInventory(floor).map((object) => object.id)).not.toContain("one");
});

test("the greys: neutral, a higher top never darker, the tallest the lightest, every patch of ground darker than every block", () => {
  const floor = buildMapFloor(MAP);
  const drawn = readInventory(floor);
  const neutral = (grey: number): boolean => (grey >> 16) === ((grey >> 8) & 0xff) && ((grey >> 8) & 0xff) === (grey & 0xff);
  for (const object of drawn) expect(neutral(object.grey), object.id).toBe(true);
  const blocks = drawn.filter((object) => object.kind === "box" || object.kind === "ramp");
  const top = (id: string): number => MAP.shapes.find((shape) => shape.id === id)!.max[1];
  for (const one of blocks) {
    for (const other of blocks) if (top(one.id) > top(other.id)) expect(one.grey & 0xff, `${one.id} over ${other.id}`).toBeGreaterThanOrEqual(other.grey & 0xff);
  }
  // Never darker, and lighter wherever the tops differ by more than a grey level's share of the height (about 4 cm
  // over the test scene's 0.55 to 4.7 m): 1 cm apart may share a grey, 1 m apart may not.
  expect(greyOf(0.56, 0.55, 4.7)).toBe(greyOf(0.55, 0.55, 4.7));
  expect(greyOf(1.55, 0.55, 4.7)).toBeGreaterThan(greyOf(0.55, 0.55, 4.7));
  // Over the whole range in 1 cm steps: never darker, and every grey level from the lowest's to the tallest's taken
  // (TE-5: a version with a few greys passes a check at one pair of heights).
  const levels = new Set<number>();
  let last = 0;
  for (let cm = 55; cm <= 470; cm += 1) {
    const level = greyOf(cm / 100, 0.55, 4.7) & 0xff;
    expect(level, `${cm} cm`).toBeGreaterThanOrEqual(last);
    last = level;
    levels.add(level);
  }
  expect(levels.size).toBe(0xd8 - 0x70 + 1);
  const lightest = Math.max(...blocks.map((object) => object.grey & 0xff));
  expect(drawn.find((object) => object.id === "ramp_mz")!.grey & 0xff).toBe(lightest);
  const darkestBlock = Math.min(...blocks.map((object) => object.grey & 0xff));
  for (const object of drawn.filter((object) => object.kind === "region" || object.kind === "ground")) {
    expect(object.grey & 0xff, object.id).toBeLessThan(darkestBlock);
  }
  // His STREET-GREY (b), 2026-11-28: the two kinds of ground take two greys.
  expect(groundGrey("street")).not.toBe(groundGrey("grass"));
  // The colour read back from each material is the colour written: what the inventory reports is what was authored.
  expect(drawn.find((object) => object.id === "wall")!.grey).toBe(greyOf(2.55, 0.55, 4.7));
  expect(greyOf(4.7, 0.55, 4.7)).toBe(0xd8d8d8);
  expect(greyOf(0.55, 0.55, 4.7)).toBe(0x707070);
});

test("the grid: one line per whole metre over the bounds, both ways (113 and 49 over the test scene's bounds), each on the floor from edge to edge", () => {
  for (const map of [MAP, RAISED]) {
    const grid = buildMapFloor(map).children.find((object) => object.userData["kind"] === "grid");
    if (!(grid instanceof THREE.LineSegments)) throw new Error("no grid");
    const position = grid.geometry.getAttribute("position");
    expect(position.count / 2).toBe(113 + 49);
    const [minX, y, minZ] = map.bounds.min.map(f);
    const [maxX, , maxZ] = map.bounds.max.map(f);
    // Every line lies on the floor, the bounds' bottom (G-3), and runs from one edge of the bounds to the other
    // (TE-4): a line along z ends at the bounds' two z edges, a line along x at its two x edges.
    for (let i = 0; i < position.count; i += 2) {
      expect([position.getY(i), position.getY(i + 1)], `${map.id} line ${i / 2}`).toEqual([y, y]);
      const alongZ = position.getX(i) === position.getX(i + 1);
      const ends = alongZ ? [position.getZ(i), position.getZ(i + 1)] : [position.getX(i), position.getX(i + 1)];
      expect(ends, `${map.id} line ${i / 2}`).toEqual(alongZ ? [minZ, maxZ] : [minX, maxX]);
      if (!alongZ) expect(position.getZ(i), `${map.id} line ${i / 2}`).toBe(position.getZ(i + 1));
    }
  }
});

test("the swap frees each geometry and material of the old floor once, and two floors share none", () => {
  const counted = (floor: THREE.Group): { geometries: Set<THREE.BufferGeometry>; materials: Set<THREE.Material>; disposed: Map<object, number> } => {
    const geometries = new Set<THREE.BufferGeometry>();
    const materials = new Set<THREE.Material>();
    const disposed = new Map<object, number>();
    for (const object of floor.children) {
      if (object instanceof THREE.Mesh || object instanceof THREE.LineSegments) {
        geometries.add(object.geometry);
        materials.add(object.material as THREE.Material);
      }
    }
    for (const thing of [...geometries, ...materials]) thing.addEventListener("dispose", () => disposed.set(thing, (disposed.get(thing) ?? 0) + 1));
    return { geometries, materials, disposed };
  };
  const first = counted(buildMapFloor(MAP));
  const second = counted(buildMapFloor(MAP));
  for (const geometry of first.geometries) expect(second.geometries.has(geometry)).toBe(false);
  for (const material of first.materials) expect(second.materials.has(material)).toBe(false);
  const { floor: empty } = buildEmptyScene();
  const emptyCount = counted(empty);
  disposeFloor(empty);
  expect([...emptyCount.disposed.values()]).toEqual(new Array(emptyCount.geometries.size + emptyCount.materials.size).fill(1));
  const floor = buildMapFloor(MAP);
  const map = counted(floor);
  disposeFloor(floor);
  expect(map.disposed.size).toBe(map.geometries.size + map.materials.size);
  expect([...map.disposed.values()].every((times) => times === 1)).toBe(true);
});

test("the overview shows the whole floor at 16:9, 4:3 and 1:1, with every corner in front of the camera, as near as that lets it", () => {
  for (const aspect of [16 / 9, 4 / 3, 1]) {
    const view = overview(MAP.bounds, aspect);
    if (view === null) throw new Error(`no overview at ${aspect}`);
    const [minX, y, minZ] = MAP.bounds.min;
    const [maxX, , maxZ] = MAP.bounds.max;
    const corners = [[minX, minZ], [maxX, minZ], [maxX, maxZ], [minX, maxZ]].map(([x, z]) => new THREE.Vector3(x, y, z));
    /** The farthest any corner reaches toward the picture's edge, with the camera at `share` of the overview's distance. */
    const reach = (share: number): number => {
      const camera = new THREE.PerspectiveCamera(50, aspect, 0.1, 1e6);
      const target = new THREE.Vector3(...view.target);
      camera.position.copy(target).addScaledVector(new THREE.Vector3(...view.position).sub(target), share);
      camera.lookAt(target);
      camera.updateMatrixWorld(true);
      return Math.max(
        ...corners.map((corner) => {
          const p = corner.clone().project(camera);
          expect(p.z, `${aspect} in front`).toBeLessThan(1);
          return Math.max(Math.abs(p.x), Math.abs(p.y));
        }),
      );
    };
    // Every corner inside the picture's margin; and as near as fits (TE-3): a little nearer, some corner is past it.
    expect(reach(1), `${aspect}`).toBeLessThanOrEqual(VIEW_MARGIN + 1e-9);
    expect(reach(0.99), `${aspect}`).toBeGreaterThan(VIEW_MARGIN);
    // Seen from today's side and angle: from above, from the near side (+z), straight at the floor's middle.
    expect(view.target).toEqual([24, 0, 0]);
    expect(view.position[0]).toBeCloseTo(24, 9);
    expect(view.position[1] / (view.position[2] - view.target[2])).toBeCloseTo(14 / 16, 9);
  }
});

// A search with no end blocks the runner itself: Vitest's timeout cannot stop a synchronous loop, so the plant that
// takes the limit away is run under an outer timeout (`.checkpoints/cs4c-soldier/plants-4c.py`).
test("the overview frames nothing it cannot: at an aspect of 0 or NaN it gives none, and on the largest floor the loader allows its search always ends (step S's review, GE-F1)", () => {
  expect(overview(MAP.bounds, 0)).toBeNull();
  expect(overview(MAP.bounds, Number.NaN)).toBeNull();
  expect(overview(LARGEST, 16 / 9)).not.toBeNull();
  // A window a thousand times taller than it is wide: no view, and the search ended.
  expect(overview(LARGEST, 0.001)).toBeNull();
});

test("the camera takes the overview, with its far plane well beyond the floor's middle, or stays as it was when the floor cannot be framed (step S's review, TE-7)", () => {
  const camera = new THREE.PerspectiveCamera(50, 16 / 9, 0.1, 500);
  expect(frameCamera(camera, MAP.bounds)).toBe(true);
  const view = overview(MAP.bounds, 16 / 9)!;
  expect(camera.position.toArray()).toEqual(view.position);
  const distance = new THREE.Vector3(...view.position).distanceTo(new THREE.Vector3(...view.target));
  expect(camera.far).toBe(Math.max(500, 4 * distance));
  // The largest floor: the far plane follows the distance, past 500 m.
  const far = new THREE.PerspectiveCamera(50, 16 / 9, 0.1, 500);
  expect(frameCamera(far, LARGEST)).toBe(true);
  expect(far.far).toBeGreaterThan(500);
  // A window of no width: the camera is left exactly as it was.
  const before = [camera.position.toArray(), camera.quaternion.toArray(), camera.far];
  camera.aspect = 0;
  expect(frameCamera(camera, MAP.bounds)).toBe(false);
  expect([camera.position.toArray(), camera.quaternion.toArray(), camera.far]).toEqual(before);
});

test("the inventory reads what is drawn, not the map: a corner moved after the build is reported moved, an unknown object stops it", () => {
  const floor = buildMapFloor(MAP);
  const position = meshOf(floor, "wall").geometry.getAttribute("position");
  position.setX(0, 99);
  const wall = readInventory(floor).find((object) => object.id === "wall");
  expect(wall?.points.some((point) => point[0] === 99)).toBe(true);
  floor.add(new THREE.Mesh(new THREE.BufferGeometry(), new THREE.MeshStandardNodeMaterial()));
  expect(() => readInventory(floor)).toThrow(INVENTORY_UNKNOWN);
  // An object with a kind the inventory knows but not a floor's own material stops it too (step S's review, EA-4).
  const other = buildMapFloor(MAP);
  const stray = new THREE.Mesh(new THREE.BufferGeometry(), new THREE.MeshBasicNodeMaterial());
  stray.name = "stray";
  stray.userData = { kind: "box" };
  other.add(stray);
  expect(() => readInventory(other)).toThrow(INVENTORY_UNKNOWN);
  // The empty floor reads as its ground alone.
  expect(readInventory(buildEmptyScene().floor).map((object) => [object.id, object.kind, object.grey])).toEqual([["ground", "ground", 0x6b7480]]);
});
