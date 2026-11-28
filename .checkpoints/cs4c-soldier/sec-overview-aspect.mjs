// security-auditor scratch: does scene.ts's `overview` doubling loop ever end when the aspect is 0 (window width 0)?
// Same math as packages/client/src/render/scene.ts `overview`, with the `while (!fits(far))` loop capped for the probe.
import * as THREE from "three";

const VIEW_DIRECTION = new THREE.Vector3(0, 14, 16).normalize();
const VIEW_MARGIN = 0.92;
function probe(aspect) {
  const [minX, y, minZ] = [-50, 0, -50];
  const [maxX, maxZ] = [50, 50];
  const target = new THREE.Vector3((minX + maxX) / 2, y, (minZ + maxZ) / 2);
  const corners = [
    new THREE.Vector3(minX, y, minZ),
    new THREE.Vector3(maxX, y, minZ),
    new THREE.Vector3(maxX, y, maxZ),
    new THREE.Vector3(minX, y, maxZ),
  ];
  const camera = new THREE.PerspectiveCamera(50, aspect, 0.1, 1e6);
  const projected = new THREE.Vector3();
  const fits = (distance) => {
    camera.position.copy(target).addScaledVector(VIEW_DIRECTION, distance);
    camera.lookAt(target);
    camera.updateMatrixWorld(true);
    return corners.every((corner) => {
      projected.copy(corner).project(camera);
      return Math.abs(projected.x) <= VIEW_MARGIN && Math.abs(projected.y) <= VIEW_MARGIN && projected.z < 1;
    });
  };
  let far = 1;
  let doublings = 0;
  while (!fits(far) && doublings < 5000) {
    far *= 2;
    doublings += 1;
  }
  return { aspect, doublings, far };
}
for (const aspect of [16 / 9, 1e-3, 0, Number.NaN]) console.log(JSON.stringify(probe(aspect)));
