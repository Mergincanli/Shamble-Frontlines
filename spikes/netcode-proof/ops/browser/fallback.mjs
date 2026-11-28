// fallback.mjs <chrome|edge|firefox|brave>: -29's QUIC-blocked fallback as rough readings (C3; Q-U8-3; u8-brief.md
// rev 1.1 Block 9). Room `fallback-<browser>` must be up. It re-arms `npt-ctl quic off` for this browser (15 minutes at
// most), then runs each attempt in its own slot and a fresh session, closed before the next; the page tries
// WebTransport (1.5 s timeout), then WebSocket, and writes one `event.fallback.playing_ns` per attempt. An attempt that
// reaches WebTransport while the block is meant on makes this browser's leg INVALID, never a pass. At its end it turns
// the block off (`quic on`). Brave: it arms the block and writes the link; mds36.mjs turns the block off first.
import { ALL_BROWSERS, ATTEMPTS, PAGE, ROOMS, pageUrl } from "./config.mjs";
import {
  ROUGH, braveLinks, close, events, machine, openAndWait, quic, roomTokens, runInfo, save, saveSummary, session,
} from "./lib.mjs";

const browser = process.argv[2];
if (!ALL_BROWSERS.includes(browser) || process.argv.length !== 3) {
  console.error("usage: node fallback.mjs chrome|edge|firefox|brave");
  process.exit(2);
}
const room = `fallback-${browser}`;
const tokens = roomTokens(room);
const url = (k) => pageUrl({ suite: "fallback", nn: ROOMS[room].nn, transport: "wt", attempt: k, token: tokens[k] });
console.log(quic("off"));

if (browser === "brave") {
  const f = braveLinks(room, [url(0)]);
  console.log(`QUIC is blocked for 15 minutes at most. Brave's fallback link (slot 0) is in ${f}: open it once on the PC;`);
  console.log(`expect "${PAGE.playing("ws")}" within 3 s. "${PAGE.playing("wt")}" makes Brave's leg INVALID.`);
  process.exit(0);
}

const m = machine();
const run = await runInfo();
const attempts = [];
try {
  for (let k = 0; k < ATTEMPTS.laptop; k++) {
    const s = await session(m, browser);
    try {
      const st = await openAndWait(s, url(k), (x) => events(x.records, "fallback").length > 0 || x.status.includes(PAGE.playing("wt")), 15000);
      const ev = events(st.records, "fallback");
      const reachedWt = st.status.includes(PAGE.playing("wt"));
      attempts.push({ attempt: k, slot: k, reached_webtransport: reachedWt, events: ev.length,
        playing_ns: ev.length === 1 ? ev[0].playing_ns : null, elapsed_ms: st.elapsed_ms });
      const leg = { suite: "fallback", browser, transport: "wt", nn: ROOMS[room].nn, attempt: k, slot: k };
      save(run, leg, st, attempts.at(-1));
    } finally {
      await close(s);
    }
  }
} finally {
  console.log(quic("on"));
}
const invalid = attempts.some((a) => a.reached_webtransport);
const played = attempts.filter((a) => a.events === 1).length;
const max = Math.max(...attempts.map((a) => a.playing_ns ?? Infinity));
saveSummary("fallback", browser, { browser, attempts: attempts.length, played, max_playing_ns: max, invalid });
console.log(invalid
  ? `${browser} fallback: INVALID: an attempt reached WebTransport while QUIC was meant blocked`
  : `${browser} fallback: ${played} of ${attempts.length} attempts playing over WebSocket; max ${max} ns (${ROUGH})`);
process.exit(!invalid && played === attempts.length ? 0 : 1);
