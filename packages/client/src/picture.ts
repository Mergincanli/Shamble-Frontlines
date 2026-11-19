// The interface file between the two tracks (build plan, Part 15.2): the app shell on one side, the picture on the
// other. The app gives the picture the newest blocks and the time each frame; the picture draws and reports what it
// runs on. Only src/render/ implements it and only src/render/ imports three; the app, the overlay and the harness
// reach the picture through this file alone. engine-architect reviews every change to it.

/** The two names a backend can have (ADR 0003 part 4). */
export type BackendName = "WebGPU" | "WebGL 2 fallback";

/** What the picture runs on, for the readout, the report and every screenshot. */
export interface PictureInfo {
  readonly backend: BackendName;
  /** The graphics adapter as the browser names it; empty when the browser gives nothing. */
  readonly adapter: string;
  /** Whether the backend can time the GPU (`timestamp-query`). */
  readonly timestampQuery: boolean;
}

/**
 * What the app hands over each frame. Events join when the first change-set has some. The blocks are valid during
 * this `draw` call only: afterwards the app may send them back to the Worker, and a view kept on one then reads an
 * empty buffer. Keep no view past the call.
 */
export interface PictureFrame {
  /** The newest block, and the one before it, to draw between two ticks; `null` until they have arrived. */
  readonly newest: ArrayBuffer | null;
  readonly previous: ArrayBuffer | null;
  /** The frame's time, `performance.now()`. */
  readonly timeMs: number;
}

/** The renderer's own counts for the last frame it drew. */
export interface RenderCounters {
  readonly drawCalls: number;
  readonly triangles: number;
  readonly lines: number;
  readonly geometries: number;
  readonly textures: number;
  /** Compiled shader stages (three counts a pipeline's vertex and fragment stage as two); the counts gate names it. */
  readonly programs: number;
}

export interface Picture {
  /**
   * Sets the renderer up on the canvas; resolves once it can draw (`await renderer.init()`). From then on the
   * picture reports its renderer's errors (a lost device among them) as error events on the window.
   */
  start(canvas: HTMLCanvasElement, options: { readonly forceWebGL: boolean }): Promise<PictureInfo>;
  /**
   * Runs `frame` once per display frame, on the renderer's own loop, with the frame's time (`performance.now()`).
   * The app does its per-frame work there (the input post rides this clock) and calls `draw`. It runs exactly once
   * per display frame from `start` to `stop` and is never withheld, during a warm-up or a quality change either.
   * The renderer resets its counters at the start of each of these frames, so `counters()` read between two frames
   * gives the last drawn frame's numbers.
   */
  loop(frame: (timeMs: number) => void): void;
  draw(frame: PictureFrame): void;
  counters(): RenderCounters;
  /** The canvas's size in CSS pixels and the display's own pixel ratio; how far it follows that ratio is the picture's call. */
  resize(width: number, height: number, devicePixelRatio: number): void;
  /** Stops drawing and frees the GPU's resources. */
  stop(): void;
}

/**
 * How the app gets its picture: `src/render/index.ts` exports one `createPicture` of this type, and that is the
 * only thing outside `render/` may import from it. Everything else in that folder is its owner's to rearrange.
 */
export type CreatePicture = () => Picture;
