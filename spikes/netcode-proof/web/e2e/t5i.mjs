#!/usr/bin/env node
// t5i.mjs: Block T5 (i), early contact (u7-brief.md rev 1.1:129-132; advisor-u7-brief.md C9), with U6b's merged
// release binary and no cargo, before the Wasm client exists. Each load gets its own fresh room start in its own folder
// (its own settings, token file, log and hash file; C9: no load sends a Join, so no slot is ever joined):
//   A-wt          the contact page: the hash file, lib.js's connect() over WebTransport, the control stream;
//   B-ws          the contact page: wss straight, the room's own certificate admitted by this context's
//                 ignoreHTTPSErrors only (never a flag on the laptop: plan.md:1222-1223);
//   C-wt-wrong    the contact page with each hash byte XOR 0xff: a control, which must fail (no fallback);
//   D-real-page   index.html with main.js, the module Worker, lib.js and hooks.js, on a link with a synthetic token:
//                 the Worker connects over WebTransport, then ends "the client did not load" (no Wasm yet).
// The room's 2 s Join deadline closes each session. Container readings, recorded and never judged.
// Use: node t5i.mjs --server <npt-server> --server-sha256 <64 hex> --chromium <chrome> --playwright <index.mjs>
//      [--room NN] [--commit <40 hex>]
// Writes <checkout>/.gate/u7/t5i/<UTC stamp>/<load>/ (settings.json, room.jsonl, room.stdout.txt, room.stderr.txt,
// hash.json, contact.json; the token file is deleted once its room has exited) and summary.json. Prints no token.
import { spawn } from "node:child_process";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, statSync, writeFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { serve } from "./serve.mjs";

const HERE = dirname(fileURLToPath(import.meta.url));
const WEB = resolve(HERE, "..");
const ROOT = resolve(WEB, "..", "..", "..");
const OUT = resolve(process.env.NPT_WEB_OUT || join(ROOT, ".gate", "netcode-proof-test", "web-check"));
const sleep = (ms) => new Promise((ok) => setTimeout(() => ok(null), ms));
const sha256 = (f) => createHash("sha256").update(readFileSync(f)).digest("hex");
/** Text that may name a page URL, cut at any fragment (U7R-NC-4: as t5ii.mjs's). */
const clean = (text) => String(text).replace(/#[^\s"']*/g, "#<cut>").slice(0, 240);

function parseArgs(argv) {
  const a = { room: "27", commit: "8efb9b2d971c3bb48ef41337249438c725f34708" };
  for (let i = 0; i < argv.length; i += 2) {
    const k = argv[i]?.replace(/^--/, "");
    if (!["server", "server-sha256", "chromium", "playwright", "room", "commit"].includes(k) || argv[i + 1] === undefined) {
      throw new Error(`unknown or empty argument ${argv[i]}`);
    }
    a[k] = argv[i + 1];
  }
  for (const k of ["server", "server-sha256", "chromium", "playwright"]) if (!a[k]) throw new Error(`--${k} is needed`);
  if (!/^(0[0-9]|[12][0-9]|3[01])$/.test(a.room) || !/^[0-9a-f]{40}$/.test(a.commit)) throw new Error("bad --room or --commit");
  return a;
}

function settings(dir, nn, transport, commit) {
  return {
    run: `T5i-${basename(dir)}`, set: "T5i", role: "contact", marks: [], date: new Date().toISOString().slice(0, 10),
    commit, layer: "L2", profile: "P0", seed: 7, scene: "SS", clients: 1, cmd_timing: "B", tick_hz: 30,
    machine_role: "C", machine_name: "C", machine_plan: "container", machine_location: "here", machine_address: "",
    tick_vcpu: null, io_vcpus: [], io_threads: 2, log: join(dir, "room.jsonl"), token_file: join(dir, "tokens"),
    duration_ms: 0, warmup_ticks: 0, first_cpu_model: "", transport,
    wt_bind: `127.0.0.1:${30001 + Number(nn)}`, ws_bind: `127.0.0.1:${31001 + Number(nn)}`,
    ws_cert_chain: "", ws_cert_key: "", hash_path: join(dir, "hash.json"), ws_split_mark_s: null,
  };
}

/** Starts the room from its settings; `open` resolves true at its OPEN line, false at an exit or 15 s. */
function startRoom(server, dir, s) {
  const file = join(dir, "settings.json");
  writeFileSync(file, `${JSON.stringify(s, null, 1)}\n`);
  const child = spawn(server, ["--settings", file], { stdio: ["pipe", "pipe", "pipe"] });
  const out = [];
  const err = [];
  child.stderr.on("data", (d) => err.push(d));
  const exited = new Promise((ok) => child.on("exit", (code, signal) => ok({ code, signal })));
  const open = new Promise((ok) => {
    child.stdout.on("data", (d) => {
      out.push(d);
      if (Buffer.concat(out).toString().includes("OPEN")) ok(true);
    });
    exited.then(() => ok(false));
    setTimeout(() => ok(false), 15000);
  });
  return { child, out, err, exited, open };
}

/** Ends the room by closing its stdin (main.rs:799-829), at most 20 s, then a kill; keeps its output; drops the tokens. */
async function stopRoom(room, dir) {
  room.child.stdin.end();
  let exit = await Promise.race([room.exited, sleep(20000)]);
  const killed = exit === null;
  if (killed) {
    room.child.kill("SIGKILL");
    exit = await room.exited;
  }
  writeFileSync(join(dir, "room.stdout.txt"), Buffer.concat(room.out));
  writeFileSync(join(dir, "room.stderr.txt"), Buffer.concat(room.err));
  rmSync(join(dir, "tokens"), { force: true });
  return { ...exit, killed };
}

/** The room log's events (they hold no token: SEC-U6AD-3), its record count, its footer. */
function roomLog(file) {
  if (!existsSync(file)) return { present: false };
  const recs = readFileSync(file, "utf8").split("\n").filter(Boolean).map((l) => JSON.parse(l));
  return {
    present: true,
    records: recs.length,
    header_transport: recs[0]?.params?.transport ?? null,
    events: recs.filter((r) => r.k === "event").map((r) => ({ ev: r.ev, ph: r.ph, f: r.f ?? {} })),
    footer: recs.find((r) => r.k === "footer") ?? null,
  };
}

const LOADS = [
  { name: "A-wt", transport: "wt", path: "/contact.html", query: "mode=wt", ignoreHTTPSErrors: false },
  { name: "B-ws", transport: "ws", path: "/contact.html", query: "mode=ws", ignoreHTTPSErrors: true },
  { name: "C-wt-wrong", transport: "wt", path: "/contact.html", query: "mode=wt-wrong", ignoreHTTPSErrors: false },
  { name: "D-real-page", transport: "wt", path: "/", query: "smoke=1&suite=smoke&room=NN&transport=wt&attempt=0", ignoreHTTPSErrors: false },
];

async function runLoad(load, a, browser, site, base, current) {
  const dir = join(base, load.name);
  mkdirSync(dir, { recursive: true });
  current.hash = join(dir, "hash.json");
  const room = startRoom(a.server, dir, settings(dir, a.room, load.transport, a.commit));
  const opened = await room.open;
  const query = load.path === "/" ? load.query.replace("NN", a.room) : `${load.query}&room=${a.room}`;
  const result = { load: load.name, page: `${load.path}?${query}`, ignoreHTTPSErrors: load.ignoreHTTPSErrors, room_open: opened };
  if (opened) {
    const ctx = await browser.newContext({ ignoreHTTPSErrors: load.ignoreHTTPSErrors });
    const page = await ctx.newPage();
    const consoleLines = [];
    page.on("console", (m) => consoleLines.push({ type: m.type(), text: m.text().slice(0, 200) }));
    page.on("pageerror", (e) => consoleLines.push({ type: "pageerror", text: String(e.message).slice(0, 200) }));
    const servedBefore = site.served.length;
    const t0 = Date.now();
    const fragment = load.path === "/" ? `#t=${["nptj", "_", "T5iContactNoJoinSynth", "A"].join("")}` : "";
    await page.goto(`http://127.0.0.1:${site.port}${load.path}?${query}${fragment}`);
    try {
      if (load.path === "/") {
        await page.waitForFunction(() => (document.querySelector("#status")?.textContent ?? "").startsWith("Ended"), null, { timeout: 20000 });
        result.page_state = await page.evaluate(() => ({
          status: document.querySelector("#status")?.textContent ?? "",
          version: document.querySelector("#version")?.textContent ?? "",
          records: Array.isArray(window.nptRecords) ? window.nptRecords.slice() : null,
          nptClockResNs: window.nptClockResNs ?? null,
          nptDropped: window.nptDropped ?? null,
        }));
      } else {
        await page.waitForFunction(() => window.contactDone === true, null, { timeout: 20000 });
        result.contact = await page.evaluate(() => window.contact);
      }
    } catch (e) {
      result.wait_error = clean(String(e.message).split("\n")[0]).slice(0, 160);
    }
    result.page_ms = Date.now() - t0;
    await sleep(300);
    result.console = consoleLines;
    result.served = site.served.slice(servedBefore);
    await ctx.close();
  }
  result.room_exit = await stopRoom(room, dir);
  result.room_log = roomLog(join(dir, "room.jsonl"));
  result.hash_file = existsSync(current.hash) ? sha256(current.hash) : null;
  writeFileSync(join(dir, "contact.json"), `${JSON.stringify(result, null, 1)}\n`);
  return result;
}

async function main() {
  const a = parseArgs(process.argv.slice(2));
  const serverSha = sha256(a.server);
  if (serverSha !== a["server-sha256"]) throw new Error("the npt-server binary's SHA-256 is not the one named");
  for (const n of ["lib.js", "hooks.js", "main.js", "worker.js"]) if (!existsSync(join(OUT, n))) throw new Error(`no ${n}: run tsc first`);
  const { chromium } = await import(pathToFileURL(a.playwright).href);
  const pwVersion = JSON.parse(readFileSync(join(dirname(a.playwright), "package.json"), "utf8")).version;
  const base = join(ROOT, ".gate", "u7", "t5i", new Date().toISOString().replace(/[:.]/g, "-"));
  mkdirSync(base, { recursive: true });
  const current = { hash: null };
  const site = await serve({
    files: {
      "/": join(WEB, "index.html"), "/main.js": join(OUT, "main.js"), "/worker.js": join(OUT, "worker.js"),
      "/lib.js": join(OUT, "lib.js"), "/hooks.js": join(OUT, "hooks.js"),
      "/contact.html": join(HERE, "contact.html"), "/contact.js": join(HERE, "contact.js"),
    },
    hash: (nn) => (nn === a.room ? current.hash : null),
  });
  const browser = await chromium.launch({ executablePath: a.chromium, headless: true });
  const summary = {
    when: new Date().toISOString(), commit: a.commit, server: { path: a.server, sha256: serverSha },
    browser: { path: a.chromium, version: browser.version(), headless: true }, playwright_core: pwVersion,
    command: ["node", "web/e2e/t5i.mjs", ...process.argv.slice(2)].join(" "), room: a.room,
    ports: { wt_udp: 30001 + Number(a.room), ws_tcp: 31001 + Number(a.room) }, page_server: "http://127.0.0.1 (port chosen at start)",
    loads: [],
  };
  try {
    for (const load of LOADS) {
      const r = await runLoad(load, a, browser, site, base, current);
      const steps = r.contact?.steps?.map((s) => `${s.what}@${s.ms}`) ?? [];
      summary.loads.push({ load: r.load, room_open: r.room_open, room_exit: r.room_exit, steps,
        status: r.page_state?.status ?? null, records: r.page_state?.records ?? null, wait_error: r.wait_error ?? null,
        violations: r.contact?.violations ?? null });
      console.log(`${r.load}: room ${r.room_open ? "open" : "NOT open"}, exit ${r.room_exit.code}; ${steps.join(", ") || r.page_state?.status || r.wait_error}`);
    }
  } finally {
    await browser.close();
    await site.close();
  }
  summary.files = readdirSync(base, { recursive: true }).sort().map((f) => join(base, f))
    .filter((f) => statSync(f).isFile()).map((f) => ({ file: f.slice(ROOT.length + 1), sha256: sha256(f) }));
  writeFileSync(join(base, "summary.json"), `${JSON.stringify(summary, null, 1)}\n`);
  console.log(`t5i: ${summary.loads.length} loads; records under ${base.slice(ROOT.length + 1)}`);
}

main().catch((e) => {
  console.error(`t5i: ${clean(String(e?.message ?? e).split("\n")[0])}`);
  process.exit(1);
});
