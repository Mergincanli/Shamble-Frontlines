// What the browser tests read from the development page: the same numbers its readout shows.
import type { Page } from "@playwright/test";
import { ADDRESS } from "../../playwright.config.ts";

export interface Report {
  header: { crossOriginIsolated: boolean };
  picture: { backend: string };
  counters: Record<string, number>;
  tick: number;
  blocks: number;
  posts: number;
  messages: { sent: number; received: number; refused: number };
  postUsed: number;
  presses: { made: number; echoed: number; lost: number | null };
  releases: { made: number; echoed: number };
  keyToTickP50: number;
  keyToTickP95: number;
  keyToTickCount: number;
  frames: number;
  errors: number;
  pauses: number;
  lastPause: { ticksWhileHidden: number } | null;
  worker: {
    ticks: number;
    wakes: number;
    drops: number;
    pauses: number;
    stalePosts: number;
    undetached: number;
    maxTicksInOneWake: number;
    crossOriginIsolated: boolean;
  } | null;
}
type Hooked = { __shambleReport?(): Report };

export const report = (page: Page): Promise<Report> =>
  page.evaluate(() => (window as unknown as Required<Hooked>).__shambleReport());

/** Waits until the page has blocks and the Worker's own report. */
export async function running(page: Page, blocks = 90): Promise<void> {
  await page.waitForFunction(
    (wanted) => {
      const data = (window as unknown as Hooked).__shambleReport?.();
      return data !== undefined && data.worker !== null && data.blocks >= wanted;
    },
    blocks,
    { timeout: 60_000 },
  );
}

/**
 * Waits until the page has drawn its frame number `frames`. The Worker's numbers reach the page from its frame loop
 * (a request at most four times a second, the answer between two frames), so on a slow page a read right after a
 * change can still be the old report.
 */
export async function drawn(page: Page, frames: number): Promise<void> {
  await page.waitForFunction((wanted) => ((window as unknown as Hooked).__shambleReport?.().frames ?? 0) >= wanted, frames);
}

/**
 * Opens the page with every WebAssembly entry point on the page replaced by one that counts and throws, and waits
 * until the match runs. Returns the list that fills with every request to another address than the small server's.
 */
export async function open(page: Page, query: string): Promise<string[]> {
  const outside: string[] = [];
  page.on("request", (request) => {
    if (!request.url().startsWith(`${ADDRESS}/`)) outside.push(request.url());
  });
  // Init scripts run in the page and not in its Worker, so the Worker still loads the Wasm.
  await page.addInitScript(() => {
    const count = window as unknown as { __wasmCalls: number };
    count.__wasmCalls = 0;
    const wasm = WebAssembly as unknown as Record<string, unknown>;
    // Every function and constructor the name holds: the first five run code or check it, the rest make its parts.
    const names = ["instantiate", "instantiateStreaming", "compile", "compileStreaming", "validate", "Module", "Instance", "Memory"];
    for (const name of [...names, "Table", "Global", "Tag", "Exception", "Suspending", "promising"]) {
      wasm[name] = function () {
        count.__wasmCalls += 1;
        throw new Error(`the page used WebAssembly.${name}`);
      };
    }
  });
  await page.goto(`/${query}`);
  await running(page);
  return outside;
}

/** How often the page itself used WebAssembly since `open`: 0, always (PM-0002-2). */
export const wasmCalls = (page: Page): Promise<number> =>
  page.evaluate(() => (window as unknown as { __wasmCalls: number }).__wasmCalls);

/** Every browser spec needs the built development page. */
export function needsBuild(): string {
  const dist = process.env["SHAMBLE_DIST"];
  if (dist === undefined) throw new Error("set SHAMBLE_DIST to the built page's folder (playwright.config.ts says how)");
  return dist;
}
