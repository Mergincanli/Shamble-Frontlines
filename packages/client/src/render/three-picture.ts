import * as THREE from "three/webgpu";
import type { Picture, PictureInfo, RenderCounters } from "../picture.ts";
import { readInventory } from "./inventory.ts";
import { buildEmptyScene, buildMapFloor, disposeFloor, frameCamera } from "./scene.ts";

// Written into the code by the build (vite.config.mjs, `define`): the literal `false` in a production build, which
// cuts the development inventory out of it (the production scan checks that).
declare const __SHAMBLE_DEVTOOLS__: boolean;

/** The parts of three's two backends this file reads; its types do not name them. */
interface Backend {
  isWebGPUBackend?: boolean;
  device?: { adapterInfo?: { vendor?: string; architecture?: string; description?: string } };
  gl?: WebGL2RenderingContext | null;
}

/** The pixel ratio is followed up to this; a quality tier may lower it later. */
const MAX_PIXEL_RATIO = 2;

/** The graphics adapter as the browser names it; empty when the browser gives nothing (every part may be hidden). */
function adapterText(backend: Backend): string {
  const info = backend.device?.adapterInfo;
  if (info !== undefined) return [info.vendor, info.architecture, info.description].filter((part) => part).join(", ");
  const gl = backend.gl ?? null;
  const names = gl?.getExtension("WEBGL_debug_renderer_info") ?? null;
  return gl === null || names === null ? "" : String(gl.getParameter(names.UNMASKED_RENDERER_WEBGL));
}

// three reports a lost device, an uncaptured WebGPU error and a shader that does not compile through its own
// console function only. Sent on to the browser's console as before, and each error also to the page's error
// event, so the page's error count sees it (ADR 0003: a device error is an error).
THREE.setConsoleFunction((type: "log" | "warn" | "error", message: string, ...rest: unknown[]) => {
  console[type](message, ...rest);
  if (type === "error") reportError(new Error(message));
});

/**
 * The picture on three.js's WebGPU renderer, which falls back to WebGL 2 by itself (ADR 0003 parts 1 to 4). It
 * waits for `renderer.init()` before the first frame and reports the backend that really runs.
 */
export function createThreePicture(): Picture {
  const { scene, camera, floor: emptyFloor } = buildEmptyScene();
  let floor = emptyFloor;
  // The map's bounds as numbers, for the overview at each aspect: no reference to the document is kept.
  let framed: { min: [number, number, number]; max: [number, number, number] } | null = null;
  const frame = (): void => {
    // A floor that cannot be framed at this aspect leaves the camera as it is (scene.ts, `frameCamera`).
    if (framed !== null) frameCamera(camera, framed);
  };
  let renderer: THREE.WebGPURenderer | null = null;
  return {
    async start(canvas, options): Promise<PictureInfo> {
      renderer = new THREE.WebGPURenderer({ canvas, antialias: true, forceWebGL: options.forceWebGL });
      await renderer.init();
      const backend = renderer.backend as Backend;
      return {
        backend: backend.isWebGPUBackend === true ? "WebGPU" : "WebGL 2 fallback",
        adapter: adapterText(backend),
        timestampQuery: renderer.hasFeature("timestamp-query"),
      };
    },
    loop(onFrame): void {
      void renderer?.setAnimationLoop(onFrame);
    },
    setContent(content): void {
      const next = buildMapFloor(content.map);
      scene.remove(floor);
      disposeFloor(floor);
      scene.add(next);
      floor = next;
      const { min, max } = content.map.bounds;
      framed = { min: [min[0], min[1], min[2]], max: [max[0], max[1], max[2]] };
      frame();
    },
    inventory() {
      return __SHAMBLE_DEVTOOLS__ ? readInventory(floor) : null;
    },
    draw(): void {
      // The scene has nothing a block moves yet; the soldier arrives later in change-set 4c.
      renderer?.render(scene, camera);
    },
    counters(): RenderCounters {
      const info = renderer?.info;
      return {
        drawCalls: info?.render.drawCalls ?? 0,
        triangles: info?.render.triangles ?? 0,
        lines: info?.render.lines ?? 0,
        geometries: info?.memory.geometries ?? 0,
        textures: info?.memory.textures ?? 0,
        programs: info?.memory.programs ?? 0,
      };
    },
    resize(width, height, devicePixelRatio): void {
      renderer?.setPixelRatio(Math.min(devicePixelRatio, MAX_PIXEL_RATIO));
      renderer?.setSize(width, height, false);
      // A window of no width is held at one pixel, as one of no height is: the aspect stays a positive number.
      camera.aspect = Math.max(1, width) / Math.max(1, height);
      camera.updateProjectionMatrix();
      frame();
    },
    stop(): void {
      void renderer?.setAnimationLoop(null);
      void renderer?.dispose();
      renderer = null;
    },
  };
}
