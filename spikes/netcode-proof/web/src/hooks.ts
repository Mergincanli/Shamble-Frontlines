// hooks.ts: the measured build's two timings (u7-brief.md rev 1.1 Block T4). The bites dist swaps this file for
// web/bites/hooks.js, which arms one page bite from the link's `bite` query; this one holds no bite.

/** WebTransport's wait before the page falls back to WebSocket (plan.md:496-497). */
export const WT_TIMEOUT_MS: number | null = 1_500;
/** The period of `maxDatagramSize` reads after the one at session ready (plan.md:497). */
export const MDS_READ_PERIOD_MS: number | null = 10_000;
