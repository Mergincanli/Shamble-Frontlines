// hooks.js of the bites dist only (u7-brief.md rev 1.1 Block T4; C13): `node build.mjs --bites` copies this file over
// tsc's measured hooks.js into <checkout>/.gate/netcode-proof-test/dist-bites/, never under web/. The page imports it;
// it arms at most one bite, named by the link's `bite` query (marks.json's ids, :6541 and :7493), and the page refuses
// a `bite` it does not list. With no bite it gives the measured timings.
export const BITE_IDS = Object.freeze(["PM-0001-36/read-once", "PM-0001-29/no-wt-timeout"]);
const bite = new URLSearchParams(globalThis.location?.search ?? "").get("bite");
/** PM-0001-29/no-wt-timeout: no WebTransport timeout, so the attempt hangs (no fallback). */
export const WT_TIMEOUT_MS = bite === "PM-0001-29/no-wt-timeout" ? null : 1_500;
/** PM-0001-36/read-once: maxDatagramSize read once, at session ready only. */
export const MDS_READ_PERIOD_MS = bite === "PM-0001-36/read-once" ? null : 10_000;
