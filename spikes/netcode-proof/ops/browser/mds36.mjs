// mds36.mjs: -36's home EU path as rough readings (u8-brief.md rev 1.1 Block 9; plan.md:739, :818): Chrome, Edge and
// Firefox at once, each in its own slot of room `mds` (NN 19, `r1.sh browsers mds`), 10 minutes; the page reads
// maxDatagramSize at start and every 10 s and writes one `event.mds_read.size_bytes` per read; at least 61 reads each.
// It turns the QUIC block off first (the fallback legs ran before it). Records go under .gate/ as npt-log lines.
import { MDS_MINUTES, MDS_MIN_READS, MDS_SLOTS, ROOMS, pageUrl } from "./config.mjs";
import { ROUGH, close, events, machine, openAndWait, quic, read, roomTokens, runInfo, save, session } from "./lib.mjs";

if (process.argv.length !== 2) {
  console.error("usage: node mds36.mjs");
  process.exit(2);
}
console.log(quic("on"));
const tokens = roomTokens("mds");
const m = machine();
const run = await runInfo();
const sessions = [];
try {
  for (const [slot, browser] of MDS_SLOTS.entries()) {
    const s = await session(m, browser);
    sessions.push(s);
    const url = pageUrl({ suite: "mds", nn: ROOMS.mds.nn, transport: "wt", token: tokens[slot] });
    await openAndWait(s, url, (x) => events(x.records, "mds_read").length > 0, 15000);
  }
  await new Promise((r) => setTimeout(r, MDS_MINUTES * 60 * 1000 + 15000));
  let all = true;
  for (const [slot, s] of sessions.entries()) {
    const st = await read(s);
    const reads = events(st.records, "mds_read").map((f) => f.size_bytes);
    const enough = reads.length >= MDS_MIN_READS;
    all &&= enough;
    const leg = { suite: "mds", browser: s.browser, transport: "wt", nn: ROOMS.mds.nn, attempt: 0, slot };
    save(run, leg, st, { browser: s.browser, slot, path: "eu-path", reads: reads.length,
      min_bytes: Math.min(...reads), max_bytes: Math.max(...reads), at_least_61: enough });
    console.log(`${s.browser}: ${reads.length} maxDatagramSize reads, min ${Math.min(...reads)} B (${ROUGH})`);
  }
  process.exitCode = all ? 0 : 1;
} finally {
  for (const s of sessions) await close(s);
}
