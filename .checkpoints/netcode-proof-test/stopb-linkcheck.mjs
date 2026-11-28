// Stop B's A1, the static link check (advisor-u8-f1-head.md:57-79): every link R1's browser legs build, made by
// config.mjs's pure pageUrl() exactly as smoke.mjs:15, cert14d.mjs:19, fallback.mjs:19 and mds36.mjs:21 call it, fed to
// U7's own parseLink() as the web step compiles it (.gate/netcode-proof-test/web-check/lib.js; run
// `node check.mjs --only web` first so it is current). Scratch: it writes nothing, prints one line per link and a
// count, and exits 1 on any mismatch. Run: node /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/stopb-linkcheck.mjs
import { pathToFileURL } from "node:url";

const SPIKE = "/home/user/Shamble-Frontlines/spikes/netcode-proof";
const cfg = await import(pathToFileURL(`${SPIKE}/ops/browser/config.mjs`).href);
const lib = await import(pathToFileURL("/home/user/Shamble-Frontlines/.gate/netcode-proof-test/web-check/lib.js").href);

// A stand-in slot token per link in U6a's form ("nptj_" then 22 base64url characters), distinct per link.
let serial = 0;
const token = () => `nptj_${String(serial++).padStart(22, "A").replace(/[^A-Za-z0-9_-]/g, "A")}`;

const links = [];
// smoke.mjs:15: suite smoke, room smoke, each of its slots' transport, attempt 0 (the default).
for (const slot of cfg.SMOKE_SLOTS) {
  const transport = slot.endsWith("-ws") ? "ws" : "wt";
  links.push({ leg: `smoke ${slot}`, suite: "smoke", nn: cfg.ROOMS.smoke.nn, transport, attempt: 0 });
}
// cert14d.mjs:19 and fallback.mjs:19: one room per browser, transport wt, attempt k for each of the room's slots.
for (const [suite, prefix] of [["cert-14d", "cert"], ["fallback", "fallback"]]) {
  for (const browser of cfg.ALL_BROWSERS) {
    const room = cfg.ROOMS[`${prefix}-${browser}`];
    for (let k = 0; k < room.slots; k++) links.push({ leg: `${suite} ${browser} attempt ${k}`, suite, nn: room.nn, transport: "wt", attempt: k });
  }
}
// mds36.mjs:21: suite mds, room mds, transport wt, attempt 0 (the default), one link per slot.
for (const slot of cfg.MDS_SLOTS) links.push({ leg: `mds ${slot}`, suite: "mds", nn: cfg.ROOMS.mds.nn, transport: "wt", attempt: 0 });

let bad = 0;
for (const l of links) {
  const t = token();
  const url = new URL(cfg.pageUrl({ suite: l.suite, nn: l.nn, transport: l.transport, attempt: l.attempt, token: t }));
  const p = lib.parseLink(url.search, url.hash);
  const want = { suite: l.suite, room: l.nn, transport: l.transport, attempt: l.attempt, token: t, wrongHash: false, bite: null };
  const fields = Object.keys(want);
  const ok = url.origin === cfg.PAGE_ORIGIN && url.pathname === "/" && p.ok && fields.every((f) => p.value[f] === want[f]) && Object.keys(p.value).length === fields.length;
  if (!ok) bad++;
  console.log(`${ok ? "ok" : "MISMATCH"} ${l.leg}: room ${l.nn}, ${l.transport}, attempt ${l.attempt}${p.ok ? "" : ` (${p.why})`}`);
}
console.log(`links: ${links.length}, mismatches: ${bad}`);
process.exit(bad === 0 ? 0 : 1);
