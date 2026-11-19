// What the browser tests read from the development page: the same numbers its readout shows.
import type { Page } from "@playwright/test";

export interface Report {
  header: { crossOriginIsolated: boolean };
  picture: { backend: string };
  counters: Record<string, number>;
  tick: number;
  blocks: number;
  posts: number;
  frames: number;
  errors: number;
  pauses: number;
  lastPause: { ticksWhileHidden: number } | null;
  worker: { ticks: number; undetached: number; maxTicksInOneWake: number; crossOriginIsolated: boolean } | null;
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

/** Every browser spec needs the built development page. */
export function needsBuild(): string {
  const dist = process.env["SHAMBLE_DIST"];
  if (dist === undefined) throw new Error("set SHAMBLE_DIST to the built page's folder (playwright.config.ts says how)");
  return dist;
}
