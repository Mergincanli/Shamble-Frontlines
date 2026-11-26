// lib.mjs: the browser scripts' shared code (u8-brief.md rev 1.1 Block 9). Standalone WebdriverIO `remote()`, never
// the @wdio/cli testrunner (security-r1-deps.md:241-244). Tokens and page URLs are never printed: a message names the
// room and the slot only. Every write goes under .gate/.
import { execFileSync, spawn } from "node:child_process";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, openSync, readFileSync, writeFileSync } from "node:fs";
import { createServer, connect } from "node:net";
import os from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { remote } from "webdriverio";
import {
  CACHE, LINKS, LOGS, MACHINE_FILE, PAGE, PAGE_ORIGIN, R1SH, RECORDS, ROOMS, SSH, codeCommit, driverArgs, legHeader,
  legLog, loadMachine, logDate, osMachine, remoteOptions, verifyDriver,
} from "./config.mjs";

const HERE = dirname(fileURLToPath(import.meta.url));

export const ROUGH = "rough, not judged; R8 judges the seven marks (-1, -14, -28, -29, -36, PM-0006-16, PM-0006-25) once he names the laptop";

for (const d of [CACHE, LOGS, RECORDS, LINKS]) mkdirSync(d, { recursive: true });

export function machine() {
  return loadMachine(MACHINE_FILE);
}

function ssh(remoteCommand, input) {
  return execFileSync("ssh", ["-F", SSH.config, SSH.host, remoteCommand], {
    input, encoding: "utf8", stdio: ["pipe", "pipe", "pipe"], maxBuffer: 1 << 20,
  });
}

/** A browser room's slot tokens, by slot (U6a's token file; one per line, `<slot> <token>` or the token alone). */
export function roomTokens(room) {
  const { nn, slots } = ROOMS[room];
  const lines = ssh(`${R1SH} tokens ${nn}`).split(/\r?\n/).filter(Boolean);
  const tokens = lines.map((l, i) => {
    const m = l.match(/^(?:(\d{1,2})\s+)?([A-Za-z0-9_-]{16,128})$/);
    if (!m) throw new Error(`room ${room}: token file line ${i} is malformed (not printed)`);
    return m[2];
  });
  if (tokens.length < slots) throw new Error(`room ${room}: ${tokens.length} tokens for ${slots} slots`);
  return tokens;
}

/** -29's QUIC block (Q-U8-3): `npt-ctl quic off|on` on RS, nothing else. */
export function quic(state) {
  if (state !== "off" && state !== "on") throw new Error("quic off|on");
  return ssh(`sudo -n /usr/local/sbin/npt-ctl quic ${state}`).trim();
}

function freePort() {
  return new Promise((ok, fail) => {
    const s = createServer();
    s.once("error", fail);
    s.listen(0, "127.0.0.1", () => {
      const { port } = s.address();
      s.close(() => ok(port));
    });
  });
}

function waitPort(port, ms) {
  const until = Date.now() + ms;
  return new Promise((ok, fail) => {
    const once = () => {
      const c = connect(port, "127.0.0.1", () => { c.destroy(); ok(); });
      c.once("error", () => {
        c.destroy();
        if (Date.now() > until) fail(new Error(`no driver on 127.0.0.1:${port}`));
        else setTimeout(once, 200);
      });
    };
    once();
  });
}

/**
 * A fresh WebDriver session (a fresh temporary profile). Every driver is started here from its recorded path after its
 * SHA-256 check, bound to loopback (config.mjs driverArgs: SEC-SB-1, SEC-SB-4), and remote() connects to it on
 * 127.0.0.1, so WebdriverIO starts no driver of its own.
 */
export async function session(m, browser) {
  verifyDriver(m, browser);
  const port = await freePort();
  const log = openSync(join(LOGS, `${browser}-driver-${port}.log`), "w");
  const driver = spawn(m[browser].driver, driverArgs(browser, port), { stdio: ["ignore", log, log], windowsHide: true });
  await waitPort(port, 15000);
  return { browser, b: await remote(remoteOptions(m, browser, port)), driver };
}

export async function close(s) {
  try {
    await s.b.deleteSession();
  } finally {
    if (s.driver) s.driver.kill();
  }
}

/** Opens a page and waits until `done(state)` holds, polling the status, version and records; `timed_out` when not. */
export async function openAndWait(s, url, done, ms) {
  const t0 = Date.now();
  await s.b.url(url);
  let state;
  for (;;) {
    state = await read(s);
    state.elapsed_ms = Date.now() - t0;
    state.timed_out = !done(state);
    if (!state.timed_out || state.elapsed_ms > ms) return state;
    await new Promise((r) => setTimeout(r, 200));
  }
}

/** The page's status line, version, npt-log records, clock step and dropped count (U7's page; config.mjs's PAGE). */
export async function read(s) {
  return s.b.execute((p) => {
    const text = (sel) => (document.querySelector(sel) || {}).textContent || "";
    const recs = Array.isArray(window[p.records]) ? window[p.records].slice() : [];
    return { status: text(p.status), version: text(p.version), records: recs,
      nptClockResNs: window[p.clockResNs] ?? null, nptDropped: window[p.dropped] ?? null };
  }, { status: PAGE.status, version: PAGE.version, records: PAGE.records, clockResNs: PAGE.clockResNs, dropped: PAGE.dropped });
}

/**
 * The run's header values, once per run: Y from rs.yaml's commit line, today's date, and the SHA-256 of the page host's
 * `/npt_client_wasm_bg.wasm`, fetched once per commit (normal TLS verification) and kept under .gate/.
 */
export async function runInfo() {
  const commit = codeCommit(readFileSync(join(HERE, "..", "cloud-init", "rs.yaml"), "utf8"));
  const kept = join(RECORDS, `wasm-${commit}.sha256`);
  let sha = existsSync(kept) ? readFileSync(kept, "utf8").trim() : "";
  if (!/^[0-9a-f]{64}$/.test(sha)) {
    const res = await fetch(`${PAGE_ORIGIN}/npt_client_wasm_bg.wasm`);
    if (!res.ok) throw new Error(`the page host answered ${res.status} for the Wasm`);
    sha = createHash("sha256").update(Buffer.from(await res.arrayBuffer())).digest("hex");
    writeFileSync(kept, `${sha}\n`);
  }
  return { run: "R1", date: logDate(new Date()), commit, wasm_sha256: sha };
}

/** The records' events of one name (npt-log: {"k":"event","ev":<name>,"f":{...}}). */
export function events(records, name) {
  const out = [];
  for (const line of records) {
    try {
      const r = JSON.parse(line);
      if (r.k === "event" && r.ev === name) out.push(r.f || {});
    } catch {
      // not a record line: ignored, counted by the caller's checks
    }
  }
  return out;
}

/**
 * Writes a leg's .jsonl log (config.mjs's legHeader and legLog: the header, the page's lines unchanged, the footer,
 * `stopped` when the leg ended on its time limit) and its summary under .gate/, named by suite, browser and time.
 */
export function save(run, leg, state, summary) {
  const base = join(RECORDS, `${leg.suite}-${leg.browser}-${leg.transport}-a${leg.attempt}-${stampNow()}`);
  const header = legHeader(run, leg, state, osMachine(os));
  writeFileSync(`${base}.jsonl`, legLog(header, state.records, state.timed_out === true));
  writeFileSync(`${base}.summary.json`, `${JSON.stringify({ ...summary, label: ROUGH }, null, 1)}\n`);
  return base;
}

/** A browser's whole-suite summary under .gate/ (no log: it is no leg). */
export function saveSummary(suite, browser, summary) {
  const base = join(RECORDS, `${suite}-${browser}-${stampNow()}`);
  writeFileSync(`${base}.summary.json`, `${JSON.stringify({ ...summary, label: ROUGH }, null, 1)}\n`);
  return base;
}

function stampNow() {
  return new Date().toISOString().replace(/[:.]/g, "-");
}

/** Brave runs on his PC: its links go to a text file for the USB stick (plan.md:1111-1112), deleted after the run. */
export function braveLinks(name, urls) {
  const f = join(LINKS, `brave-${name}.txt`);
  writeFileSync(f, `${urls.join("\n")}\n`);
  return f;
}
