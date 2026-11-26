#!/usr/bin/env node
// t5ii.mjs: Block T5 (ii), the whole client in a browser (u7-brief.md rev 1.1 section 3's browser line, pass marks 1-9;
// advisor-u7-brief.md C8, C9): the measured and bites dists from build.mjs, `<base>`'s release npt-server on 127.0.0.1,
// the dists from serve.mjs on http://127.0.0.1 (the CSP word for word with 127.0.0.1 added), Chromium headless driven by
// tools/'s playwright-core. Each load gets its own fresh room start, or its own slot (C9); nothing joins twice. Then
// C8's leg through U6c's npt-host. Container readings, recorded and never judged.
// Use: node t5ii.mjs --server <npt-server> --server-sha256 <hex> --host <npt-host> --host-sha256 <hex>
//      --chromium <chrome> --playwright <index.mjs> --dist <measured dist> --bites-dist <bites dist>
//      --commit <40 hex> --dist-log <build log> --bites-log <build log> [--room NN]
// Writes <checkout>/.gate/u7/t5ii/<UTC stamp>/<load>/ (settings.json, room.jsonl, room.stdout.txt, room.stderr.txt,
// hash-NN.json, result.json) and summary.json. No token, fragment or bind value is written or printed.
import { spawn, spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { createSocket } from "node:dgram";
import { mkdirSync, readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { connect as tcpConnect } from "node:net";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { reap, roomLog, roomTokens, settings, sleep, startRoom, stopRoom, track } from "./room.mjs";
import { serve } from "./serve.mjs";

const HERE = dirname(fileURLToPath(import.meta.url));
const ROOT = resolve(HERE, "..", "..", "..", "..");
const NAME = "rs.nt.shamblefrontlines.com";
const DIST = ["index.html", "main.js", "worker.js", "lib.js", "hooks.js", "npt_client_wasm.js", "npt_client_wasm_bg.wasm"];
const sha256 = (f) => createHash("sha256").update(readFileSync(f)).digest("hex");
const playing = (t) => (s) => s.status === `${t === "wt" ? "WebTransport" : "WebSocket"} · Playing`;
const events = (s, ev) => (s.records ?? []).map((l) => JSON.parse(l)).filter((r) => r.k === "event" && r.ev === ev);
const attempted = (s) => events(s, "attempt").length > 0;
const always = () => true;
/** Text that may name a page URL (Playwright's errors do), cut at any fragment so no token is ever kept or printed. */
const clean = (text) => String(text).replace(/#[^\s"']*/g, "#<cut>").slice(0, 240);

/** The current load's folder, read by `fail` to drop its tokens file (U7R-NC-4). */
const current = { dir: null };

function parseArgs(argv) {
  const a = { room: "27" };
  const keys = ["server", "server-sha256", "host", "host-sha256", "chromium", "playwright", "dist", "bites-dist", "commit",
    "dist-log", "bites-log", "room"];
  for (let i = 0; i < argv.length; i += 2) {
    const k = argv[i]?.replace(/^--/, "");
    if (!keys.includes(k) || argv[i + 1] === undefined) throw new Error(`unknown or empty argument ${argv[i]}`);
    a[k] = argv[i + 1];
  }
  for (const k of keys.slice(0, 11)) if (!a[k]) throw new Error(`--${k} is needed`);
  // U7R-TE-10: the tree's commit is given, never a default (the caller reads it with git, as its own command).
  if (!/^[0-9a-f]{40}$/.test(a.commit)) throw new Error("--commit is 40 lower-case hex");
  if (!/^(0[0-9]|[12][0-8])$/.test(a.room)) throw new Error("--room is 00-28 (29-31 stay free for the silent sockets)");
  return a;
}

/** The page's link for a load (U8's contract, the query only: the fragment is added at goto and never kept). */
const link = (nn, suite, transport, attempt, extra = "") =>
  `smoke=1&suite=${suite}&room=${nn}&transport=${transport}&attempt=${attempt}${extra}`;

/** What the page shows: U8's reads (lib.mjs:115-122) and the counters table, by textContent. */
async function read(page) {
  return page.evaluate(() => ({
    status: document.querySelector("#status")?.textContent ?? "",
    version: document.querySelector("#version")?.textContent ?? "",
    records: Array.isArray(window.nptRecords) ? window.nptRecords.slice() : null,
    nptClockResNs: window.nptClockResNs ?? null,
    nptDropped: window.nptDropped ?? null,
    counters: Object.fromEntries([...document.querySelectorAll("#counters tbody tr")]
      .map((tr) => [tr.children[0]?.textContent ?? "", Number(tr.children[1]?.textContent ?? "")])),
  }));
}

/**
 * One page load in a fresh context: `until(state)` polled every 200 ms (U8's openAndWait) for at most `ms`, then
 * `extraMs` more; the context's console (page and Worker), page errors and responses (path, status, type) kept.
 */
async function pageRun(browser, origin, query, fragment, { path = "/", ignore = false, until, ms = 15000, extraMs = 0 }) {
  const ctx = await browser.newContext({ ignoreHTTPSErrors: ignore });
  const consoleLines = [];
  const responses = [];
  const failed = [];
  ctx.on("console", (m) => consoleLines.push({ type: m.type(), page: m.page() !== null, text: clean(m.text()) }));
  ctx.on("response", async (r) => {
    const u = new URL(r.url());
    const addr = await r.serverAddr().catch(() => null);
    responses.push({ path: u.pathname, status: r.status(), type: r.headers()["content-type"] ?? "",
      ip: addr?.ipAddress ?? null, port: addr?.port ?? null });
  });
  ctx.on("requestfailed", (r) => failed.push({ path: new URL(r.url()).pathname, error: clean(r.failure()?.errorText ?? "") }));
  const page = await ctx.newPage();
  page.on("pageerror", (e) => consoleLines.push({ type: "pageerror", page: true, text: clean(e.message) }));
  const t0 = Date.now();
  let state = {};
  let at = null;
  let error = null;
  try {
    await page.goto(`${origin}${path}${query === null ? "" : "?"}${query ?? ""}${fragment}`);
    for (;;) {
      state = await read(page);
      const el = Date.now() - t0;
      if (at === null && until(state)) at = el;
      if ((at !== null && el >= at + extraMs) || el > ms + extraMs) break;
      await sleep(200);
    }
  } catch (e) {
    error = clean(String(e.message).split("\n")[0]);
  }
  await ctx.close();
  return { query, ignoreHTTPSErrors: ignore, until_ms: at, error, ...state, console: consoleLines, responses, failed };
}

/** Every record is npt-log's `event` or `second` form exactly (C1), and none holds a token or URL. */
function recordProblems(records) {
  const name = /^[a-z][a-z0-9_]*$/;
  const p = [];
  for (const line of records ?? []) {
    let r;
    try {
      r = JSON.parse(line);
    } catch {
      p.push("a line that is not JSON");
      continue;
    }
    const keys = Object.keys(r).join(",");
    const ok = (r.k === "event" && keys === "k,ev,ph,ns,f" && name.test(r.ev) && Number.isSafeInteger(r.ns))
      || (r.k === "second" && keys === "k,sec,ph,v" && Number.isSafeInteger(r.sec));
    if (!ok || r.ph !== "measured") p.push(`a record not in npt-log's form: ${keys}`);
    if (/nptj_|https?:|wss?:/.test(line)) p.push("a record holding a token or a URL");
  }
  return p;
}

const cspLines = (c) => c.filter((l) => /Content Security Policy|Refused to (connect|load|create|execute)/.test(l.text));

/** A UDP socket bound and silent on `port` (marks 5 and 8: WebTransport's port that never answers). */
function silentUdp(port) {
  return new Promise((ok, fail) => {
    const s = createSocket("udp4");
    s.once("error", fail);
    s.on("message", () => {});
    s.bind(port, "127.0.0.1", () => ok(s));
  });
}

/** One load: a fresh room in its own folder, its pages (each its own slot), the room stopped, result.json written. */
async function runLoad(spec, a, browser, site, base, current) {
  const dir = join(base, spec.name);
  mkdirSync(dir, { recursive: true });
  current.dir = dir;
  const pages = spec.pages;
  const quiet = spec.silent ? await silentUdp(30001 + Number(a.room)) : null;
  const s = settings(dir, { nn: a.room, transport: spec.transport, commit: a.commit, clients: pages.length,
    wtPort: spec.silent ? 30001 + 30 : null, wsCert: spec.wsCert ?? null });
  const room = startRoom(a.server, dir, s);
  const result = { load: spec.name, room_open: await room.open, pages: [] };
  try {
    if (result.room_open) {
      const tokens = roomTokens(dir);
      for (const [slot, p] of pages.entries()) {
        const r = await pageRun(browser, spec.origin ?? site.origin, p.query, `#t=${tokens[slot]}`, p);
        result.pages.push({ slot, ...r });
      }
    }
  } finally {
    result.room_exit = await stopRoom(room, dir);
    quiet?.close();
  }
  result.room_log = roomLog(join(dir, "room.jsonl"));
  writeFileSync(join(dir, "result.json"), `${JSON.stringify(result, null, 1)}\n`);
  return result;
}

/** C8's leg: U6c's npt-host serving the measured dist under the trusted name, mapped to 127.0.0.1 in Chromium. */
async function realHost(a, chromium, base) {
  const dir = join(base, "C8-real-host");
  mkdirSync(dir, { recursive: true });
  const [chain, key] = [join(dir, "test-chain.pem"), join(dir, "test-key.pem")];
  const made = spawnSync("openssl", ["req", "-x509", "-newkey", "ec", "-pkeyopt", "ec_paramgen_curve:prime256v1", "-nodes",
    "-keyout", key, "-out", chain, "-days", "2", "-subj", `/CN=${NAME}`, "-addext", `subjectAltName=DNS:${NAME}`],
  { encoding: "utf8" });
  if (made.status !== 0) throw new Error("openssl could not make the test certificate");
  writeFileSync(join(dir, "host-keys.txt"), "test only\n");
  const hostSettings = { bind: "127.0.0.1:31000", cert_chain: chain, cert_key: key, dist: resolve(a.dist),
    hash_template: join(dir, "hash-NN.json"), host_keys: join(dir, "host-keys.txt") };
  writeFileSync(join(dir, "host.json"), `${JSON.stringify(hostSettings, null, 1)}\n`);
  const host = track(spawn(a.host, ["serve", join(dir, "host.json")], { stdio: ["ignore", "pipe", "pipe"] }));
  const hostErr = [];
  host.stderr.on("data", (d) => hostErr.push(d));
  const hostExit = new Promise((ok) => host.on("exit", (code, signal) => ok({ code, signal })));
  let browser = null;
  let result;
  let up = false;
  try {
    up = await waitPort(31000, 10000);
    // The trusted name is mapped to loopback and this browser goes direct, never through the container's outbound
    // proxy (Chromium reads the proxy variables; only loopback bypasses them), so no request leaves this machine.
    browser = await chromium.launch({ executablePath: a.chromium, headless: true,
      args: [`--host-resolver-rules=MAP ${NAME} 127.0.0.1`, "--no-proxy-server"] });
    result = await runLoad({ name: "C8-real-host", transport: "both", wsCert: { chain, key }, origin: `https://${NAME}:31000`,
      pages: [
        { query: link(a.room, "smoke", "wt", 0), ignore: true, until: playing("wt"), ms: 10000, extraMs: 3000 },
        { query: link(a.room, "smoke", "ws", 0), ignore: true, until: playing("ws"), ms: 10000, extraMs: 3000 },
      ] }, a, browser, null, base, current);
  } finally {
    await browser?.close();
    host.kill("SIGTERM");
    await Promise.race([hostExit, sleep(5000)]);
    if (host.exitCode === null && host.signalCode === null) host.kill("SIGKILL");
  }
  result.host = { listening: up, stderr: Buffer.concat(hostErr).toString().slice(0, 400) };
  writeFileSync(join(dir, "result.json"), `${JSON.stringify(result, null, 1)}\n`);
  return result;
}

function waitPort(port, ms) {
  const until = Date.now() + ms;
  return new Promise((ok) => {
    const once = () => {
      const c = tcpConnect(port, "127.0.0.1", () => {
        c.destroy();
        ok(true);
      });
      c.once("error", () => {
        c.destroy();
        if (Date.now() > until) ok(false);
        else setTimeout(once, 100);
      });
    };
    once();
  });
}

/** A page's page_csp_violations as its counters table showed it, or "not read" when the row was absent. */
const cspCount = (s) => (Number.isFinite(s.counters?.page_csp_violations) ? s.counters.page_csp_violations : "not read");

/** The marks, from the loads (readings only). */
function marks(r) {
  const first = (n) => r[n]?.pages ?? [];
  const p = (n, i = 0) => first(n)[i] ?? {};
  const reads = (s) => events(s, "mds_read").map((e) => e.f.size_bytes);
  const fb = events(p("5-fallback"), "fallback");
  const all = Object.values(r).flatMap((x) => x.pages ?? []);
  return {
    m1_smoke_wt_playing_ms: p("1-smoke-wt").until_ms,
    m1_within_3_s: p("1-smoke-wt").until_ms !== null && p("1-smoke-wt").until_ms <= 3000,
    m2_smoke_ws_playing_ms: p("2-smoke-ws").until_ms,
    m3_cert_14d_ok: first("3-cert-14d").map((s) => events(s, "attempt").map((e) => e.f.ok)),
    m4_wrong_hash_ok: events(p("4-wt-attempts-wrong-hash"), "attempt").map((e) => e.f.ok),
    m5_fallback: { status: p("5-fallback").status, playing_ns: fb.map((e) => e.f.playing_ns) },
    m6_mds_reads: reads(p("6-mds")),
    m7_record_problems: all.flatMap((s) => recordProblems(s.records)),
    m7_csp_lines: all.flatMap((s) => cspLines(s.console ?? [])),
    // U7R-NC-7: the page's and the Worker's securitypolicyviolation events, per page by load; a page whose table
    // never held the row reads "not read", never 0.
    m7_csp_violations: Object.fromEntries(Object.entries(r).map(([k, x]) => [k, (x.pages ?? []).map(cspCount)])),
    m8_read_once_reads: reads(p("8a-bite-read-once")).length,
    m8_no_wt_timeout_fallbacks: events(p("8b-bite-no-wt-timeout"), "fallback").length,
    m8_no_wt_timeout_playing_ns: events(p("8b-bite-no-wt-timeout"), "fallback").map((e) => e.f.playing_ns),
    m9_room_log: "no per-slot refusal or forged count in the binary's log (tick_loop.rs:1159-1185): not readable",
    commands: Object.fromEntries(Object.entries(r).map(([k, x]) => [k, (x.pages ?? []).map((s) => ({
      commands: s.counters?.commands, last_run_command: s.counters?.last_run_command,
      unencodable_commands: s.counters?.unencodable_commands, refused: s.counters?.refused }))])),
  };
}

async function main() {
  const a = parseArgs(process.argv.slice(2));
  for (const [bin, want] of [[a.server, a["server-sha256"]], [a.host, a["host-sha256"]]]) {
    if (sha256(bin) !== want) throw new Error(`${bin}: its SHA-256 is not the one named`);
  }
  const { chromium } = await import(pathToFileURL(a.playwright).href);
  const pwVersion = JSON.parse(readFileSync(join(dirname(a.playwright), "package.json"), "utf8")).version;
  const base = join(ROOT, ".gate", "u7", "t5ii", new Date().toISOString().replace(/[:.]/g, "-"));
  mkdirSync(base, { recursive: true });
  const files = (dist) => Object.fromEntries(DIST.map((n) => [n === "index.html" ? "/" : `/${n}`, join(dist, n)]));
  const hash = (nn) => (nn === a.room && current.dir ? join(current.dir, `hash-${nn}.json`) : null);
  const probeFiles = { "/probe.html": join(HERE, "probe.html"), "/probe.js": join(HERE, "probe.js"),
    "/probe-worker.js": join(HERE, "probe-worker.js") };
  const measured = await serve({ files: { ...files(a.dist), ...probeFiles }, hash });
  const bites = await serve({ files: files(a["bites-dist"]), hash });
  measured.origin = `http://127.0.0.1:${measured.port}`;
  bites.origin = `http://127.0.0.1:${bites.port}`;
  // Every load is on 127.0.0.1, which Chromium never sends through a proxy; `--no-proxy-server` makes sure no other
  // request can reach the container's outbound proxy either (T5ii-C).
  const browser = await chromium.launch({ executablePath: a.chromium, headless: true, args: ["--no-proxy-server"] });
  const n = a.room;
  const loads = [
    { name: "1-smoke-wt", transport: "wt", pages: [{ query: link(n, "smoke", "wt", 0), until: playing("wt"), ms: 10000, extraMs: 6000 }] },
    { name: "2-smoke-ws", transport: "ws", pages: [{ query: link(n, "smoke", "ws", 0), ignore: true, until: playing("ws"), ms: 10000, extraMs: 4000 }] },
    // Loads 3-5 wait 2.5 s past their mark, so the counters table (drawn each second from the Worker's last post) holds
    // page_csp_violations when it is read (U7R-NC-7).
    { name: "3-cert-14d", transport: "wt", pages: [0, 1, 2].map((k) => ({ query: link(n, "cert-14d", "wt", k), until: attempted, ms: 15000, extraMs: 2500 })) },
    { name: "4-wt-attempts-wrong-hash", transport: "wt", pages: [{ query: link(n, "wt-attempts", "wt", 0, "&hash=wrong"), until: attempted, ms: 15000, extraMs: 2500 }] },
    { name: "5-fallback", transport: "both", silent: true, pages: [{ query: link(n, "fallback", "wt", 0), ignore: true, until: playing("ws"), ms: 10000, extraMs: 2500 }] },
    { name: "6-mds", transport: "wt", pages: [{ query: link(n, "mds", "wt", 0), until: always, extraMs: 12000 }] },
    { name: "8a-bite-read-once", transport: "wt", bites: true, pages: [{ query: link(n, "mds", "wt", 0, "&bite=PM-0001-36/read-once"), until: always, extraMs: 12000 }] },
    { name: "8b-bite-no-wt-timeout", transport: "both", bites: true, silent: true, pages: [{ query: link(n, "fallback", "wt", 0, "&bite=PM-0001-29/no-wt-timeout"), ignore: true, until: always, extraMs: 10000 }] },
  ];
  const summary = {
    when: new Date().toISOString(), commit: a.commit, room: n,
    server: { path: a.server, sha256: a["server-sha256"] }, host: { path: a.host, sha256: a["host-sha256"] },
    dist: Object.fromEntries(DIST.map((f) => [f, sha256(join(a.dist, f))])),
    bites_dist: Object.fromEntries(DIST.map((f) => [f, sha256(join(a["bites-dist"], f))])),
    // U7R-TE-10: each dist's build log, by hash (its runner header names the tree it was built from).
    dist_log: { path: a["dist-log"], sha256: sha256(a["dist-log"]) },
    bites_log: { path: a["bites-log"], sha256: sha256(a["bites-log"]) },
    browser: { path: a.chromium, version: browser.version(), headless: true }, playwright_core: pwVersion,
    command: ["node", "web/e2e/t5ii.mjs", ...process.argv.slice(2)].join(" "),
  };
  const results = {};
  try {
    const probe = await pageRun(browser, measured.origin, null, "", { path: "/probe.html", until: always, extraMs: 2000 });
    // The positive control (U7R-NC-7): one refused connect on the page and one in its Worker, each counted by the dist's
    // own countCsp on the targets the page uses (document, the Worker's scope): both 1, or the count cannot be trusted.
    const counts = /page csp (\d+); worker csp (\d+)/.exec(probe.status ?? "");
    summary.console_probe = { worker_marker: probe.console.some((l) => l.text.includes("npt-probe: worker console")),
      worker_csp: cspLines(probe.console).length > 0, status: probe.status ?? null, error: probe.error,
      page_csp: counts ? Number(counts[1]) : "not read", worker_csp_count: counts ? Number(counts[2]) : "not read",
      responses: probe.responses.map((x) => [x.path, x.status]), failed: probe.failed, lines: probe.console.slice(0, 6) };
    for (const spec of loads) {
      results[spec.name] = await runLoad(spec, a, browser, spec.bites ? bites : measured, base, current);
      const s = results[spec.name].pages.map((x) => `${x.status} @${x.until_ms}`).join("; ");
      console.log(`${spec.name}: room ${results[spec.name].room_open ? "open" : "NOT open"}; ${s}`);
    }
  } finally {
    await browser.close();
    await measured.close();
    await bites.close();
  }
  results["C8-real-host"] = await realHost(a, chromium, base);
  console.log(`C8-real-host: ${results["C8-real-host"].pages.map((x) => `${x.status} @${x.until_ms}`).join("; ")}`);
  summary.marks = marks(results);
  const c8 = results["C8-real-host"];
  const seen = c8.pages.flatMap((x) => x.responses);
  const types = { "/": "text/html; charset=utf-8", "/npt_client_wasm_bg.wasm": "application/wasm" };
  const want = (path) => types[path] ?? "text/javascript; charset=utf-8";
  summary.c8 = {
    host_listening: c8.host.listening,
    statuses: c8.pages.map((x) => [x.status, x.until_ms, x.error]),
    dist_types: DIST.map((f) => (f === "index.html" ? "/" : `/${f}`)).map((path) => {
      const r = seen.find((x) => x.path === path);
      return { path, status: r?.status ?? null, type: r?.type ?? null, as_listed: r?.status === 200 && r.type === want(path) };
    }),
    responses: seen,
    // The outbound proxy listens on loopback too, so the address alone proves nothing: npt-host's port must match.
    every_response_from_npt_host: seen.length > 0 && seen.every((x) => x.ip === "127.0.0.1" && x.port === 31000),
    failed: c8.pages.flatMap((x) => x.failed),
    csp_lines: c8.pages.flatMap((x) => cspLines(x.console)),
    csp_violations: c8.pages.map(cspCount),
  };
  summary.files = readdirSync(base, { recursive: true }).sort().map((f) => join(base, f))
    .filter((f) => statSync(f).isFile() && !f.endsWith(".pem")).map((f) => ({ file: f.slice(ROOT.length + 1), sha256: sha256(f) }));
  writeFileSync(join(base, "summary.json"), `${JSON.stringify(summary, null, 1)}\n`);
  console.log(`t5ii: marks ${JSON.stringify(summary.marks).slice(0, 600)}`);
  console.log(`t5ii: records under ${base.slice(ROOT.length + 1)}`);
}

/**
 * Any failure, caught or not: every child still running SIGKILLed and the load's tokens file dropped (U7R-NC-4), then
 * its first line printed, cut at any '#' (a page URL's fragment holds the join token).
 */
const fail = (e) => {
  reap(current.dir);
  console.error(`t5ii: ${clean(String(e?.message ?? e).split("\n")[0])}`);
  process.exit(1);
};
process.on("unhandledRejection", fail);
process.on("uncaughtException", fail);
main().catch(fail);
