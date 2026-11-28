// smoke.mjs <chrome|edge|firefox|brave>: R1's four-browser smoke, each browser on each transport, one slot each
// (u8-brief.md rev 1.1 Block 9; plan.md:1214-1220; -14 rough, -56's item 2). Room `smoke` (NN 10) must be up
// (`r1.sh browsers smoke`). Brave runs on his PC: `brave` writes its two links for the USB stick. Run from this folder:
// `node smoke.mjs chrome` (never `node .gate/...`).
import { ALL_BROWSERS, PAGE, ROOMS, SMOKE_SLOTS, TRANSPORTS, pageUrl } from "./config.mjs";
import { ROUGH, braveLinks, close, machine, openAndWait, roomTokens, runInfo, save, session } from "./lib.mjs";

const browser = process.argv[2];
if (!ALL_BROWSERS.includes(browser) || process.argv.length !== 3) {
  console.error("usage: node smoke.mjs chrome|edge|firefox|brave");
  process.exit(2);
}
const tokens = roomTokens("smoke");
const slot = (t) => SMOKE_SLOTS.indexOf(`${browser}-${t}`);
const url = (t) => pageUrl({ suite: "smoke", nn: ROOMS.smoke.nn, transport: t, token: tokens[slot(t)] });

if (browser === "brave") {
  const f = braveLinks("smoke", Object.keys(TRANSPORTS).map(url));
  console.log(`Brave's two smoke links (slots ${slot("wt")} and ${slot("ws")}) are in ${f}: copy it to the USB stick.`);
  console.log(`On the PC, open each; expect "${PAGE.playing("wt")}", then "${PAGE.playing("ws")}", within 3 s, and Brave's version.`);
  process.exit(0);
}

const m = machine();
const run = await runInfo();
const results = [];
for (const t of Object.keys(TRANSPORTS)) {
  const s = await session(m, browser);
  try {
    const want = PAGE.playing(t);
    const st = await openAndWait(s, url(t), (x) => x.status.includes(want), 10000);
    const ok = st.status.includes(want);
    results.push({ transport: t, slot: slot(t), playing: ok, within_3s: ok && st.elapsed_ms <= PAGE.playingWithinMs,
      elapsed_ms: st.elapsed_ms, status: st.status.slice(0, 80), version: st.version.slice(0, 80) });
    save(run, { suite: "smoke", browser, transport: t, nn: ROOMS.smoke.nn, attempt: 0, slot: slot(t) }, st, results.at(-1));
    console.log(`${browser} ${TRANSPORTS[t]}: ${ok ? "Playing" : "NOT playing"} after ${st.elapsed_ms} ms (slot ${slot(t)}; ${ROUGH})`);
  } finally {
    await close(s);
  }
}
process.exit(results.every((r) => r.playing) ? 0 : 1);
