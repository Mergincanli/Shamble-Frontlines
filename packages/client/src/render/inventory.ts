import * as THREE from "three/webgpu";
import type { SceneObject } from "../picture.ts";
import type { Floor } from "./scene.ts";

// The development inventory: the drawn floor read back object by object, from each object's own geometry buffer,
// its name and its material's colour, never from the map it was built from (a list written beside the meshes would
// test the list). Its one caller is three-picture.ts's `inventory`, which a production build cuts away; its sentence
// below is one of the markers that the production scan looks for (packages/client/scripts/build-checks.mjs).

/** Said when the floor holds an object the inventory does not know: the floor and this reader have drifted apart. */
export const INVENTORY_UNKNOWN = "the scene inventory met an object it does not know";

/** The kinds an inventory reports: each one `SceneObject`'s own (step S's review, EA-4), so no other crosses the interface. */
const KINDS = ["ground", "box", "ramp", "region"] as const satisfies readonly SceneObject["kind"][];
const isKind = (kind: unknown): kind is SceneObject["kind"] => typeof kind === "string" && (KINDS as readonly string[]).includes(kind);

/** The floor's drawn objects, but for its grid, in the floor's order. */
export function readInventory(floor: Floor): SceneObject[] {
  floor.updateMatrixWorld(true);
  const objects: SceneObject[] = [];
  const point = new THREE.Vector3();
  for (const object of floor.children) {
    const kind: unknown = object.userData["kind"];
    if (kind === "grid" && object instanceof THREE.LineSegments) continue;
    if (!(object instanceof THREE.Mesh) || !isKind(kind) || !(object.material instanceof THREE.MeshStandardNodeMaterial)) throw new Error(INVENTORY_UNKNOWN);
    const position = object.geometry.getAttribute("position");
    const seen = new Map<string, [number, number, number]>();
    for (let i = 0; i < position.count; i += 1) {
      point.fromBufferAttribute(position, i).applyMatrix4(object.matrixWorld);
      seen.set(`${point.x},${point.y},${point.z}`, [point.x, point.y, point.z]);
    }
    const points = [...seen.values()].sort((a, b) => a[0] - b[0] || a[1] - b[1] || a[2] - b[2]);
    objects.push({ id: object.name, kind, points, grey: object.material.color.getHex() });
  }
  return objects;
}
