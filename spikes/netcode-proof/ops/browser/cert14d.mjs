// cert14d.mjs <chrome|edge|firefox|brave>: PM-0006-25's suite `cert-14d` as rough readings (C3; u8-brief.md rev 1.1
// Block 9). Room `cert-<browser>` must be up (`r1.sh browsers cert-<browser>`). Each attempt joins its own slot in a
// fresh session, closed before the next: the page fetches /npt/cert-hash-NN.json (NN from its `room` query) and
// opens WebTransport with `serverCertificateHashes`, then writes one `event.attempt.ok` (1 when the session is ready
// and its echo worked).
// 20 attempts on each laptop browser; Brave's one attempt is a link for the USB stick.
import { ALL_BROWSERS, ATTEMPTS, ROOMS, pageUrl } from "./config.mjs";
import {
  ROUGH, braveLinks, close, events, machine, openAndWait, roomTokens, runInfo, save, saveSummary, session,
} from "./lib.mjs";

const browser = process.argv[2];
if (!ALL_BROWSERS.includes(browser) || process.argv.length !== 3) {
  console.error("usage: node cert14d.mjs chrome|edge|firefox|brave");
  process.exit(2);
}
const room = `cert-${browser}`;
const tokens = roomTokens(room);
const url = (k) => pageUrl({ suite: "cert-14d", nn: ROOMS[room].nn, transport: "wt", attempt: k, token: tokens[k] });

if (browser === "brave") {
  const f = braveLinks(room, [url(0)]);
  console.log(`Brave's cert-14d link (slot 0) is in ${f}: open it once on the PC and read the page's result.`);
  process.exit(0);
}

const m = machine();
const run = await runInfo();
const attempts = [];
for (let k = 0; k < ATTEMPTS.laptop; k++) {
  const s = await session(m, browser);
  try {
    const st = await openAndWait(s, url(k), (x) => events(x.records, "attempt").length > 0, 15000);
    const ev = events(st.records, "attempt");
    const ok = ev.length === 1 && ev[0].ok === 1;
    attempts.push({ attempt: k, slot: k, ok, events: ev.length, elapsed_ms: st.elapsed_ms });
    const leg = { suite: "cert-14d", browser, transport: "wt", nn: ROOMS[room].nn, attempt: k, slot: k };
    save(run, leg, st, attempts.at(-1));
  } finally {
    await close(s);
  }
}
const okCount = attempts.filter((a) => a.ok).length;
saveSummary("cert-14d", browser, { browser, attempts: attempts.length, ok: okCount });
console.log(`${browser} cert-14d: ${okCount} of ${attempts.length} attempts connected with the pinned hash (${ROUGH})`);
process.exit(okCount === attempts.length ? 0 : 1);
