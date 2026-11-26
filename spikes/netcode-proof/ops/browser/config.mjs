// config.mjs: the one home for the browser scripts' URL contract with U7's page, the slot table and every WebDriver
// capability (u8-brief.md rev 1.1 Block 9; SEC-R1D-2, SEC-R1D-3; C3). Pure: it imports no WebDriver code, so the
// container test (ops/tests/browser.test.mjs) reads it directly.
// - SEC-SB-1, SEC-SB-4 (supersede SEC-R1D-2's terms): lib.mjs starts all three drivers itself, each bound to loopback:
//   chromedriver and msedgedriver with `--port=<p>` only (any `--allowed-ips` makes them bind every interface),
//   geckodriver with `--host=127.0.0.1 --port=<p>` and its WebSocket port left to the system (its npm wrapper's
//   default host is every interface). remote() names that driver (127.0.0.1:<p>), so WebdriverIO starts none.
// - SEC-R1D-3: every driver and every browser is named by path from ~/npt-operator/browsers.json (written at Stop B:
//   the vendors' drivers, their versions and SHA-256s, the installed stable browsers), and each driver's SHA-256 is
//   checked before use, so nothing downloads. Fresh temporary profiles (WebDriver's default); cache and logs under
//   .gate/.
// - The URL contract and the page's records are U8's proposal until U7's brief fixes them (u8-brief.md section 10).
import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { homedir } from "node:os";
import { basename, dirname, isAbsolute, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const HERE = dirname(fileURLToPath(import.meta.url));

/** The nearest `.gate` folder above this file (the laptop clone's), else the repository root's `.gate`. */
export function gateDir(from = HERE) {
  for (let d = from; d !== dirname(d); d = dirname(d)) {
    if (basename(d) === ".gate") return d;
  }
  return resolve(from, "..", "..", "..", "..", ".gate");
}
export const GATE = gateDir();
export const CACHE = join(GATE, "npt-browser", "cache");
export const LOGS = join(GATE, "npt-browser", "logs");
export const RECORDS = join(GATE, "npt-browser", "records");
export const LINKS = join(GATE, "npt-run-links");
export const SSH = { config: join(GATE, "npt-run-ssh", "config"), host: "npt-rs" };
export const R1SH = "bash /opt/npt/clone/spikes/netcode-proof/ops/runs/r1.sh";
export const MACHINE_FILE = join(homedir(), "npt-operator", "browsers.json");

export const BROWSERS = ["chrome", "edge", "firefox"];
export const ALL_BROWSERS = [...BROWSERS, "brave"];
export const TRANSPORTS = { wt: "WebTransport", ws: "WebSocket" };

// R1.md section 4's slot table: one room per mark and browser, one slot per attempt (C3; condition 2).
export const ROOMS = {
  smoke: { nn: "10", slots: 8 },
  "cert-chrome": { nn: "11", slots: 20 },
  "cert-edge": { nn: "12", slots: 20 },
  "cert-firefox": { nn: "13", slots: 20 },
  "cert-brave": { nn: "14", slots: 1 },
  "fallback-chrome": { nn: "15", slots: 20 },
  "fallback-edge": { nn: "16", slots: 20 },
  "fallback-firefox": { nn: "17", slots: 20 },
  "fallback-brave": { nn: "18", slots: 1 },
  mds: { nn: "19", slots: 3 },
};
// The smoke room's slots: each browser on each transport, once.
export const SMOKE_SLOTS = ["brave-wt", "brave-ws", "chrome-wt", "chrome-ws", "edge-wt", "edge-ws", "firefox-wt", "firefox-ws"];
export const MDS_SLOTS = ["chrome", "edge", "firefox"];
export const ATTEMPTS = { laptop: 20, brave: 1 };

// The page (U7's): the trusted name's page host, a query naming the suite, room, transport and attempt, and the slot's
// token in the fragment (the brief's choice 7: no replaceState; the token dies with the run).
export const PAGE_ORIGIN = "https://rs.nt.shamblefrontlines.com:31000";
export const SUITES = ["smoke", "cert-14d", "fallback", "mds"];
export function pageUrl({ suite, nn, transport = "wt", attempt = 0, token }) {
  if (!SUITES.includes(suite)) throw new Error(`unknown suite ${suite}`);
  if (!/^(1[0-9])$/.test(nn)) throw new Error("a browser room is NN 10-19");
  if (!(transport in TRANSPORTS)) throw new Error(`unknown transport ${transport}`);
  if (!Number.isInteger(attempt) || attempt < 0 || attempt > 19) throw new Error("attempt 0-19");
  if (typeof token !== "string" || !/^[A-Za-z0-9_-]{16,128}$/.test(token)) throw new Error("a malformed slot token");
  const q = new URLSearchParams({ smoke: "1", suite, room: nn, transport, attempt: String(attempt) });
  return `${PAGE_ORIGIN}/?${q}#t=${token}`;
}
// What the scripts read from the page (U7's): the status line and the version, by textContent, the records as
// npt-log JSON lines held in page memory, the Worker clock's step and the records dropped past the page's cap.
export const PAGE = {
  status: "#status",
  version: "#version",
  playing: (transport) => `${TRANSPORTS[transport]} · Playing`,
  records: "nptRecords",
  clockResNs: "nptClockResNs",
  dropped: "nptDropped",
  playingWithinMs: 3000,
};

// ---- the browser logs' header and footer (Q-U7-1 ruled (a): advisor-u7-brief.md section 5, U8's fix pass item 9) ----
// save() writes each leg's .jsonl (npt-judge's extension) as one npt-log log that read_log accepts (log/src/lib.rs:641):
// line 1 the header, exactly Header's fields (:136-169, deny_unknown_fields, with Machine's 13 and Build's 4); then the
// page's lines unchanged; then the footer, its count the records before it, the header included.
export const LOG_SCHEMA = 1;
// Each suite's marks, from R1.md's block table (wt-attempts: -28's, C4 of advisor-u7-brief.md; R1 runs none).
export const SUITE_MARKS = {
  smoke: ["PM-0001-14", "PM-0001-56"],
  "cert-14d": ["PM-0006-25"],
  fallback: ["PM-0001-29"],
  mds: ["PM-0001-36"],
  "wt-attempts": ["PM-0001-28"],
};
export const ROOM_SCENE = "SS"; // the browser rooms' scene (r1.sh room_settings)
export const RUN_PATH = "eu-path"; // the path every R1 browser leg takes

/** The run's code commit Y, as rs.yaml's /etc/npt/commit names it (filled at Stop B; zeros refused). */
export function codeCommit(rsYaml) {
  const m = rsYaml.match(/- path: \/etc\/npt\/commit\n(?: {4}.*\n)*? {6}([0-9a-f]{40})\n/);
  if (!m || /^0+$/.test(m[1])) throw new Error("rs.yaml's /etc/npt/commit is not filled (Stop B fills Y)");
  return m[1];
}

/** The run's date in the header's form (YYYY-MM-DD, UTC). */
export function logDate(d) {
  return d.toISOString().slice(0, 10);
}

/** This laptop's machine fields from Node's `os` where it gives them; else empty, 0 or [] (never invented). */
export function osMachine(os) {
  const cpus = os.cpus();
  return {
    plan: "",
    arch: os.arch(),
    cpu: cpus[0]?.model ?? "",
    vcpus: typeof os.availableParallelism === "function" ? os.availableParallelism() : cpus.length,
    mem_mb: Math.floor(os.totalmem() / 1048576),
    kernel: os.release(),
    threads_per_core: 0,
    smt: [],
  };
}

const count = (v) => (Number.isSafeInteger(v) && v >= 0 ? v : null);

/**
 * One leg's npt-log header (pure). `run`: { run, date, commit, wasm_sha256 }; `leg`: { suite, browser, transport, nn,
 * attempt, slot, bite?, hash?, crate? }; `page`: what read() returned; `machine`: osMachine(os).
 */
export function legHeader(run, leg, page, machine) {
  if (!(leg.suite in SUITE_MARKS)) throw new Error(`no marks for suite ${leg.suite}`);
  const control = leg.suite === "wt-attempts" && leg.hash === "wrong";
  const params = {
    suite: leg.suite, browser: leg.browser, path: RUN_PATH, transport: leg.transport, room: leg.nn,
    attempt: leg.attempt, version: page.version ?? "", records_dropped_count: count(page.nptDropped),
  };
  if (leg.suite === "wt-attempts") params.crate = leg.crate ?? "";
  return {
    schema: LOG_SCHEMA,
    run: `${run.run}-${leg.suite}-${leg.browser}-${leg.transport}-a${String(leg.attempt).padStart(2, "0")}`,
    set: `${run.run}-${leg.suite}`,
    role: leg.bite ? "bite" : control ? "control" : "measured",
    marks: SUITE_MARKS[leg.suite],
    machine: {
      role: "ML", name: "ML*", plan: machine.plan, arch: machine.arch, cpu: machine.cpu, vcpus: machine.vcpus,
      mem_mb: machine.mem_mb, location: "home", kernel: machine.kernel, address: "",
      threads_per_core: machine.threads_per_core, smt: machine.smt, clock_res_ns: count(page.nptClockResNs) ?? 0,
    },
    build: { commit: run.commit, binary_sha256: run.wasm_sha256, features: leg.bite ? ["bites"] : [], profile: "release" },
    bite: leg.bite ?? null,
    layer: "real",
    profile: "real-path",
    seed: 0,
    tick_hz: 30,
    clients: 1,
    scene: ROOM_SCENE,
    process: "browser",
    slot: leg.slot,
    date: run.date,
    params,
  };
}

/** The leg's .jsonl text: the header, the page's lines unchanged, the footer (`stopped` on the time limit). */
export function legLog(header, lines, stopped) {
  const footer = { k: "footer", records: 1 + lines.length, ended: stopped ? "stopped" : "clean" };
  return [JSON.stringify({ k: "header", ...header }), ...lines, JSON.stringify(footer)].map((l) => `${l}\n`).join("");
}
export const MDS_MINUTES = 10;
export const MDS_MIN_READS = 61;

/** Reads and checks the machine record: every path absolute, every driver hash 64 hex digits. */
export function loadMachine(record) {
  const m = typeof record === "string" ? JSON.parse(readFileSync(record, "utf8")) : record;
  for (const b of BROWSERS) {
    const e = m?.[b];
    if (!e) throw new Error(`browsers.json: no ${b} entry`);
    for (const k of ["browser", "driver"]) {
      if (typeof e[k] !== "string" || !(isAbsolute(e[k]) || /^[A-Za-z]:[\\/]/.test(e[k]))) {
        throw new Error(`browsers.json: ${b}.${k} must be an absolute path`);
      }
    }
    if (!/^[0-9a-f]{64}$/.test(e.driver_sha256 || "")) throw new Error(`browsers.json: ${b}.driver_sha256 must be 64 hex digits`);
  }
  return m;
}

/** SEC-R1D-3: the driver on disk is the one recorded at Stop B. */
export function verifyDriver(machine, browser) {
  const e = machine[browser];
  if (!existsSync(e.driver)) throw new Error(`${browser}: no driver at its recorded path`);
  const got = createHash("sha256").update(readFileSync(e.driver)).digest("hex");
  if (got !== e.driver_sha256) throw new Error(`${browser}: the driver's SHA-256 is not the one recorded at Stop B`);
}

/** The W3C capabilities for one browser: its binary from the machine record (SEC-R1D-3); no driver option. */
export function capabilities(machine, browser) {
  const e = machine[browser];
  switch (browser) {
    case "chrome":
      return { browserName: "chrome", "goog:chromeOptions": { binary: e.browser, args: ["--no-first-run", "--no-default-browser-check"] } };
    case "edge":
      return { browserName: "MicrosoftEdge", "ms:edgeOptions": { binary: e.browser, args: ["--no-first-run", "--no-default-browser-check"] } };
    case "firefox":
      return { browserName: "firefox", "moz:firefoxOptions": { binary: e.browser } };
    default:
      throw new Error(`no WebDriver browser ${browser}`);
  }
}

/** Each driver's own command line, loopback only (SEC-SB-1, SEC-SB-4); lib.mjs starts it from the recorded path. */
export function driverArgs(browser, port) {
  if (!Number.isInteger(port) || port < 1024 || port > 65535) throw new Error("bad driver port");
  if (browser === "firefox") return ["--host=127.0.0.1", `--port=${port}`, "--websocket-port=0"];
  if (browser === "chrome" || browser === "edge") return [`--port=${port}`];
  throw new Error(`no WebDriver browser ${browser}`);
}

/** remote()'s options: the driver lib.mjs started on 127.0.0.1:<port>, so WebdriverIO starts none; logs under .gate/. */
export function remoteOptions(machine, browser, port) {
  return { capabilities: capabilities(machine, browser), logLevel: "warn", outputDir: LOGS, cacheDir: CACHE,
    hostname: "127.0.0.1", port, path: "/" };
}
