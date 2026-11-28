import * as THREE from "three/webgpu";
import type { MapDocument, Shape } from "@shamble/types";
import { expect, test } from "vitest";
import { INVENTORY_UNKNOWN, readInventory } from "./inventory.ts";
import { buildEmptyScene, buildMapFloor, disposeFloor, greyOf, groundGrey, overview, solidPoints } from "./scene.ts";

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

test("every triangle of every solid faces outward, and its normal is the face's: a box and a ramp toward each side", () => {
  const floor = buildMapFloor(MAP);
  for (const shape of MAP.shapes) {
    const geometry = meshOf(floor, shape.id).geometry;
    const position = geometry.getAttribute("position");
    const normal = geometry.getAttribute("normal");
    const points = solidPoints(shape);
    const centre = new THREE.Vector3(...[0, 1, 2].map((axis) => points.reduce((sum, point) => sum + point[axis]!, 0) / points.length));
    expect(position.count, shape.id).toBe(shape.type === "box" ? 36 : 24);
    const [a, b, c, n, ab, ac] = [0, 0, 0, 0, 0, 0].map(() => new THREE.Vector3());
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
  const lightest = Math.max(...blocks.map((object) => object.grey & 0xff));
  expect(drawn.find((object) => object.id === "ramp_mz")!.grey & 0xff).toBe(lightest);
  const darkestBlock = Math.min(...blocks.map((object) => object.grey & 0xff));
  for (const object of drawn.filter((object) => object.kind === "region" || object.kind === "ground")) {
    expect(object.grey & 0xff, object.id).toBeLessThan(darkestBlock);
  }
  // STREET-GREY, the default (b) until he says otherwise: the two kinds of ground take two greys.
  expect(groundGrey("street")).not.toBe(groundGrey("grass"));
  // The colour read back from each material is the colour written: what the inventory reports is what was authored.
  expect(drawn.find((object) => object.id === "wall")!.grey).toBe(greyOf(2.55, 0.55, 4.7));
  expect(greyOf(4.7, 0.55, 4.7)).toBe(0xd8d8d8);
  expect(greyOf(0.55, 0.55, 4.7)).toBe(0x707070);
});

test("the grid: one line per whole metre over the bounds, both ways (113 and 49 over the test scene's bounds)", () => {
  const grid = buildMapFloor(MAP).children.find((object) => object.userData["kind"] === "grid");
  if (!(grid instanceof THREE.LineSegments)) throw new Error("no grid");
  const position = grid.geometry.getAttribute("position");
  expect(position.count / 2).toBe(113 + 49);
  // Every line lies on the floor and spans the bounds.
  for (let i = 0; i < position.count; i += 1) expect(position.getY(i)).toBe(0);
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

test("the overview shows the whole floor at 16:9, 4:3 and 1:1, with every corner in front of the camera", () => {
  for (const aspect of [16 / 9, 4 / 3, 1]) {
    const view = overview(MAP.bounds, aspect);
    const camera = new THREE.PerspectiveCamera(50, aspect, 0.1, 1e6);
    camera.position.set(...view.position);
    camera.lookAt(...view.target);
    camera.updateMatrixWorld(true);
    const [minX, y, minZ] = MAP.bounds.min;
    const [maxX, , maxZ] = MAP.bounds.max;
    for (const [x, z] of [[minX, minZ], [maxX, minZ], [maxX, maxZ], [minX, maxZ]] as const) {
      const p = new THREE.Vector3(x, y, z).project(camera);
      expect(Math.abs(p.x), `${aspect} ${x} ${z}`).toBeLessThanOrEqual(1);
      expect(Math.abs(p.y), `${aspect} ${x} ${z}`).toBeLessThanOrEqual(1);
      expect(p.z, `${aspect} ${x} ${z}`).toBeLessThan(1);
    }
    // Seen from today's side and angle: from above, from the near side (+z), straight at the floor's middle.
    expect(view.target).toEqual([24, 0, 0]);
    expect(view.position[0]).toBeCloseTo(24, 9);
    expect(view.position[1] / (view.position[2] - view.target[2])).toBeCloseTo(14 / 16, 9);
  }
});

test("the inventory reads what is drawn, not the map: a corner moved after the build is reported moved, an unknown object stops it", () => {
  const floor = buildMapFloor(MAP);
  const position = meshOf(floor, "wall").geometry.getAttribute("position");
  position.setX(0, 99);
  const wall = readInventory(floor).find((object) => object.id === "wall");
  expect(wall?.points.some((point) => point[0] === 99)).toBe(true);
  floor.add(new THREE.Mesh(new THREE.BufferGeometry(), new THREE.MeshStandardNodeMaterial()));
  expect(() => readInventory(floor)).toThrow(INVENTORY_UNKNOWN);
  // The empty floor reads as its ground alone.
  expect(readInventory(buildEmptyScene().floor).map((object) => [object.id, object.kind, object.grey])).toEqual([["ground", "ground", 0x6b7480]]);
});
