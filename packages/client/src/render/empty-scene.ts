import * as THREE from "three/webgpu";

/**
 * The ground's size in metres, and so the number of 1 m grid squares along each side. With the camera below, the
 * whole ground lies in front of the camera: a grid line that runs from in front of the camera to behind it is not
 * drawn at all by a software renderer (seen in headless Chromium, 2026-11-19).
 */
const GROUND_M = 30;

/**
 * The empty scene: a lit ground with a grid of 1 m squares, seen from above at an angle. Every colour and light
 * here is a plain placeholder that decides nothing: the look is LOOK_V2's, built in change-set 6. Node materials
 * only (ADR 0003 part 1).
 */
export function buildEmptyScene(): { scene: THREE.Scene; camera: THREE.PerspectiveCamera } {
  const scene = new THREE.Scene();
  scene.background = new THREE.Color(0x20242a);

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
  scene.add(ground);

  // One line per metre in both directions.
  const half = GROUND_M / 2;
  const points: number[] = [];
  for (let i = -half; i <= half; i += 1) {
    points.push(i, 0, -half, i, 0, half, -half, 0, i, half, 0, i);
  }
  const grid = new THREE.BufferGeometry();
  grid.setAttribute("position", new THREE.Float32BufferAttribute(points, 3));
  scene.add(new THREE.LineSegments(grid, new THREE.LineBasicNodeMaterial({ color: 0x2c323a })));

  const sun = new THREE.DirectionalLight(0xffffff, 2.5);
  sun.position.set(15, 30, 10);
  scene.add(sun, new THREE.HemisphereLight(0xdfe8f0, 0x40444a, 1));

  const camera = new THREE.PerspectiveCamera(50, 1, 0.1, 500);
  camera.position.set(0, 14, 16);
  camera.lookAt(0, 0, 0);
  return { scene, camera };
}
