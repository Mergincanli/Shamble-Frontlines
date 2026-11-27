// Test 4 (u8-brief.md rev 1.1 section 3; C2, C3; SEC-R1D-2, -3, -6): the browser package's config, lock and scripts.
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { execFileSync, spawnSync } from "node:child_process";
import { readFileSync, readdirSync, statSync } from "node:fs";
import { join, relative } from "node:path";
import { test } from "node:test";
import * as config from "../browser/config.mjs";
import { OPS, SPIKE, leaks, text } from "./helpers.mjs";

const FIXTURE = {
  chrome: { browser: "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe", driver: "C:\\Users\\m\\npt-operator\\drivers\\chromedriver.exe", driver_sha256: "a".repeat(64) },
  edge: { browser: "C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe", driver: "C:\\Users\\m\\npt-operator\\drivers\\msedgedriver.exe", driver_sha256: "b".repeat(64) },
  firefox: { browser: "C:\\Program Files\\Mozilla Firefox\\firefox.exe", driver: "C:\\Users\\m\\npt-operator\\drivers\\geckodriver.exe", driver_sha256: "c".repeat(64) },
};
const m = config.loadMachine(FIXTURE);

test("SEC-SB-1, SEC-SB-4: every driver's argv bound to loopback; remote() names a started driver for all three", () => {
  const p = 9515;
  assert.deepEqual(config.driverArgs("chrome", p), ["--port=9515"]);
  assert.deepEqual(config.driverArgs("edge", p), ["--port=9515"]);
  assert.deepEqual(config.driverArgs("firefox", p), ["--host=127.0.0.1", "--port=9515", "--websocket-port=0"]);
  for (const b of config.BROWSERS) {
    const argv = config.driverArgs(b, p).join(" ");
    assert.doesNotMatch(argv, /0\.0\.0\.0|--allowed-ips|--allowed-origins|--allow-hosts/, `${b}: never every interface`);
    assert.ok(b !== "firefox" || argv.includes("--host=127.0.0.1"), "geckodriver binds 127.0.0.1 itself");
    const o = config.remoteOptions(m, b, p);
    assert.deepEqual([o.hostname, o.port, o.path], ["127.0.0.1", p, "/"], `${b}: WebdriverIO starts no driver`);
    assert.ok(!Object.keys(o.capabilities).some((k) => k.startsWith("wdio:")), `${b}: no driver option for WebdriverIO`);
    assert.doesNotMatch(JSON.stringify(o), /0\.0\.0\.0|"\*"/, `${b}: no any-address, no wildcard`);
  }
  assert.throws(() => config.driverArgs("safari", p), /no WebDriver browser/);
  assert.doesNotMatch(text("browser/config.mjs") + text("browser/lib.mjs"), /0\.0\.0\.0|allowedIps/);
  assert.match(text("browser/lib.mjs"), /spawn\(m\[browser\]\.driver, driverArgs\(browser, port\),/, "every driver started by lib.mjs");
});

test("SEC-R1D-3: every driver and browser binary set from the machine record; cache and logs under .gate", () => {
  const want = { chrome: "goog:chromeOptions", edge: "ms:edgeOptions", firefox: "moz:firefoxOptions" };
  for (const [b, browserKey] of Object.entries(want)) {
    const caps = config.capabilities(m, b);
    assert.equal(caps[browserKey].binary, FIXTURE[b].browser, `${b} browser binary`);
    const o = config.remoteOptions(m, b, 9515);
    for (const d of [o.outputDir, o.cacheDir]) assert.match(d, /[\\/]\.gate[\\/]/);
  }
  assert.match(text("browser/lib.mjs"), /verifyDriver\(m, browser\);\n {2}const port = await freePort\(\);/, "the driver's SHA-256 checked before its start");
  assert.throws(() => config.loadMachine({ ...FIXTURE, edge: { ...FIXTURE.edge, driver: "msedgedriver.exe" } }), /absolute path/);
  assert.throws(() => config.loadMachine({ ...FIXTURE, chrome: { ...FIXTURE.chrome, driver_sha256: "" } }), /64 hex/);
  assert.throws(() => config.loadMachine({ chrome: FIXTURE.chrome, edge: FIXTURE.edge }), /no firefox/);
  assert.throws(() => config.verifyDriver(m, "chrome"), /no driver/);
});

test("the URL contract: the token rides in the fragment; bad rooms, suites and tokens refused", () => {
  const u = new URL(config.pageUrl({ suite: "fallback", nn: "15", transport: "wt", attempt: 3, token: "nptj_" + "A".repeat(22) }));
  assert.equal(u.origin, "https://rs.nt.shamblefrontlines.com:31000");
  assert.equal(u.searchParams.get("suite"), "fallback");
  assert.equal(u.searchParams.get("room"), "15");
  assert.ok(u.hash.startsWith("#t=nptj_"), "the token in the fragment, never the query");
  assert.ok(!u.search.includes("nptj_"));
  assert.throws(() => config.pageUrl({ suite: "smoke", nn: "05", token: "x".repeat(20) }), /NN 10-19/);
  assert.throws(() => config.pageUrl({ suite: "other", nn: "10", token: "x".repeat(20) }), /suite/);
  assert.throws(() => config.pageUrl({ suite: "smoke", nn: "10", token: "a b" }), /token/);
});

test("the slot table equals R1.md's and r1.sh's (C3), a slot per attempt", () => {
  const r1 = text("runs/r1.sh");
  const plan = text("runs/R1.md");
  for (const [room, { nn, slots }] of Object.entries(config.ROOMS)) {
    assert.match(r1, new RegExp(`${room}\\) printf '${nn} ${slots}\\\\n'`));
    assert.match(plan, new RegExp(`\\| ${room} \\| ${nn} \\| ${slots} \\|`));
  }
  assert.equal(config.SMOKE_SLOTS.length, config.ROOMS.smoke.slots);
  assert.equal(config.MDS_SLOTS.length, config.ROOMS.mds.slots);
  assert.equal(config.ATTEMPTS.laptop, config.ROOMS["cert-chrome"].slots);
  assert.equal(config.ATTEMPTS.laptop, config.ROOMS["fallback-edge"].slots);
});

test("the package: webdriverio 10.2.0 exact, the lock as cleared (DEP-HOLD), every entry from the registry with sha512", () => {
  const sha = (f) => createHash("sha256").update(readFileSync(join(OPS, "browser", f))).digest("hex");
  assert.equal(sha("package.json"), "8370495dee2ed77e6a937c6f40c72a5485015428d57f90505f4e440504293c90");
  assert.equal(sha("package-lock.json"), "a0efce8e511a8865d50008172b3f88e9cc072efbe92a85a035340562b7c49524");
  const pkg = JSON.parse(text("browser/package.json"));
  assert.deepEqual(pkg.dependencies, { webdriverio: "10.2.0" });
  assert.ok(!pkg.scripts && !pkg.overrides && !pkg.devDependencies);
  const lock = JSON.parse(text("browser/package-lock.json"));
  assert.equal(lock.lockfileVersion, 3);
  const entries = Object.entries(lock.packages).filter(([k]) => k !== "");
  assert.equal(entries.length, 167, "security-u8-deps.md C0: 167 entries");
  assert.equal(lock.packages["node_modules/webdriverio"].version, "10.2.0");
  for (const [k, e] of entries) {
    assert.ok(e.resolved.startsWith("https://registry.npmjs.org/"), `${k}: registry`);
    assert.match(e.integrity, /^sha512-/, `${k}: sha512`);
  }
  assert.deepEqual(entries.filter(([, e]) => e.hasInstallScript).map(([k]) => k).sort(), ["node_modules/edgedriver", "node_modules/geckodriver"]);
  assert.ok(!entries.some(([k]) => /@wdio\/cli|@wdio\/local-runner/.test(k)), "no testrunner");
});

/** Every text file under ops/ but node_modules and the tests themselves. */
function opsFiles(dir = OPS, out = []) {
  for (const n of readdirSync(dir)) {
    const p = join(dir, n);
    if (n === "node_modules" || p === join(OPS, "tests")) continue;
    if (statSync(p).isDirectory()) opsFiles(p, out);
    else out.push(p);
  }
  return out;
}

test("C2: no .npmrc under spikes/; every npm install is `npm ci --ignore-scripts` with npm_config_ignore_scripts=true", () => {
  const tracked = execFileSync("git", ["ls-files", "--", "."], { cwd: SPIKE, encoding: "utf8" }).split("\n");
  assert.deepEqual(tracked.filter((f) => /(^|\/)\.npmrc$/.test(f)), []);
  const onDisk = (dir) => readdirSync(dir).flatMap((n) => {
    const p = join(dir, n);
    if (n === "node_modules" || n === "target" || n === ".gate") return [];
    if (n === ".npmrc") return [p];
    return statSync(p).isDirectory() ? onDisk(p) : [];
  });
  assert.deepEqual(onDisk(SPIKE), []);
  let installs = 0;
  for (const f of opsFiles()) {
    if (f.endsWith("package-lock.json")) continue;
    const lines = readFileSync(f, "utf8").split("\n");
    for (const raw of lines) {
      // The prohibition itself names each form as a lone code span; a real command never is one.
      const l = raw.replace(/`(npx|npm exec|npm create|npm install|npm update)`/g, "");
      assert.doesNotMatch(l, /\bnpx\b|npm exec|npm create|npm install|npm i\b|npm update/, `${relative(OPS, f)}: ${raw}`);
      if (/\bnpm ci\b/.test(l)) {
        installs++;
        assert.match(l, /npm ci --ignore-scripts/, `${relative(OPS, f)}: ${l}`);
        assert.match(l, /npm_config_ignore_scripts=true|\$env:npm_config_ignore_scripts = "true"/, `${relative(OPS, f)}: ${l}`);
      }
    }
  }
  assert.ok(installs >= 3, "R1.md's two forms, rules.md's, rs.yaml's");
});

test("the scripts: plain Node on standalone webdriverio, no testrunner; each refuses a bad browser; no address", () => {
  for (const s of ["lib.mjs", "smoke.mjs", "cert14d.mjs", "fallback.mjs", "mds36.mjs", "config.mjs"]) {
    const body = text(`browser/${s}`);
    execFileSync(process.execPath, ["--check", join(OPS, "browser", s)]);
    const code = body.split("\n").filter((l) => !/^\s*\/\/.*\bnever\b/.test(l)).join("\n");
    assert.doesNotMatch(code, /@wdio\/cli|@wdio\/local-runner|\bdescribe\(|\bit\(/, s);
    assert.deepEqual(leaks(body), [], s);
  }
  assert.match(text("browser/lib.mjs"), /^import \{ remote \} from "webdriverio";$/m);
  // A refused browser exits 2 with its usage line before any SSH or WebDriver call.
  for (const s of ["smoke.mjs", "cert14d.mjs", "fallback.mjs"]) {
    const r = spawnSync(process.execPath, [join(OPS, "browser", s), "safari"], { encoding: "utf8" });
    assert.equal(r.status, 2, `${s}: ${r.stderr}`);
    assert.match(r.stderr, /^usage: node /, s);
  }
  const r = spawnSync(process.execPath, [join(OPS, "browser", "mds36.mjs"), "extra"], { encoding: "utf8" });
  assert.equal(r.status, 2);
});

// Item 9 (Q-U7-1 ruled (a)): save()'s header and footer, built by config.mjs's pure legHeader and legLog.
const LOG_RS = readFileSync(join(SPIKE, "crates", "log", "src", "lib.rs"), "utf8");
const rustFields = (name) => [...LOG_RS.match(new RegExp(`\\npub struct ${name} \\{\\n([\\s\\S]*?)\\n\\}`))[1]
  .matchAll(/^ {4}pub ([a-z_0-9]+):/gm)].map((x) => x[1]).sort();
const PAGE_LINES = [
  '{"k":"event","ev":"mds_read","ph":"measured","ns":1200,"f":{"size_bytes":1232}}',
  '{"k":"second","sec":0,"ph":"measured","v":{"frames_count":60}}',
  '{"k":"event","ev":"mds_read","ph":"measured","ns":10000001200,"f":{"size_bytes":1232}}',
];
const RUN = { run: "R1", date: config.logDate(new Date(Date.UTC(2026, 11, 3, 11, 40))), commit: "1".repeat(40), wasm_sha256: "e".repeat(64) };
const PAGE_STATE = { version: "npt-proof 0.1.0", nptClockResNs: 100000, nptDropped: 0, records: PAGE_LINES, timed_out: false };
const FAKE_OS = { cpus: () => [{ model: "Test CPU" }], arch: () => "x64", availableParallelism: () => 8, totalmem: () => 16 * 1073741824, release: () => "10.0.26100" };
const LEG = { suite: "mds", browser: "chrome", transport: "wt", nn: "19", attempt: 0, slot: 0 };

test("item 9: a leg's .jsonl is one npt-log log: Header's fields exactly, the page's lines unchanged, the footer's count", () => {
  assert.equal(config.LOG_SCHEMA, Number(LOG_RS.match(/^pub const SCHEMA: u32 = (\d+);$/m)[1]), "npt-log's SCHEMA");
  const machine = config.osMachine(FAKE_OS);
  const h = config.legHeader(RUN, LEG, PAGE_STATE, machine);
  assert.deepEqual(Object.keys(h).sort(), rustFields("Header"));
  assert.deepEqual(Object.keys(h.machine).sort(), rustFields("Machine"));
  assert.deepEqual(Object.keys(h.build).sort(), rustFields("Build"));
  assert.deepEqual(
    [h.schema, h.run, h.set, h.role, h.marks, h.bite, h.layer, h.profile, h.seed, h.tick_hz, h.clients, h.scene, h.process, h.slot, h.date],
    [1, "R1-mds-chrome-wt-a00", "R1-mds", "measured", ["PM-0001-36"], null, "real", "real-path", 0, 30, 1, "SS", "browser", 0, "2026-12-03"],
  );
  assert.deepEqual(h.machine, { role: "ML", name: "ML*", plan: "", arch: "x64", cpu: "Test CPU", vcpus: 8, mem_mb: 16384,
    location: "home", kernel: "10.0.26100", address: "", threads_per_core: 0, smt: [], clock_res_ns: 100000 });
  assert.deepEqual(h.build, { commit: RUN.commit, binary_sha256: RUN.wasm_sha256, features: [], profile: "release" });
  assert.deepEqual(h.params, { suite: "mds", browser: "chrome", path: "eu-path", transport: "wt", room: "19", attempt: 0,
    version: "npt-proof 0.1.0", records_dropped_count: 0 });
  const log = config.legLog(h, PAGE_LINES, false);
  const lines = log.split("\n");
  assert.equal(lines.pop(), "", "every line ends in a newline");
  assert.deepEqual(JSON.parse(lines[0]), { k: "header", ...h });
  assert.deepEqual(lines.slice(1, -1), PAGE_LINES, "every page line unchanged");
  const footer = JSON.parse(lines.at(-1));
  assert.deepEqual(footer, { k: "footer", records: lines.length - 1, ended: "clean" }, "the records before it, the header included");
  assert.equal(JSON.parse(config.legLog(h, PAGE_LINES, true).trim().split("\n").at(-1)).ended, "stopped", "the time limit");
  assert.doesNotMatch(log, /nptj_|#t=|https?:\/\/|\b(?:\d{1,3}\.){3}\d{1,3}\b/, "no token, URL or address in the file");
  // The roles: control on a wt-attempts leg with hash=wrong (with its crate), bite on a bites-dist leg.
  const c = config.legHeader(RUN, { ...LEG, suite: "wt-attempts", hash: "wrong", crate: "wtransport" }, PAGE_STATE, machine);
  assert.deepEqual([c.role, c.marks, c.params.crate], ["control", ["PM-0001-28"], "wtransport"]);
  const b = config.legHeader(RUN, { ...LEG, suite: "smoke", bite: "PM-0001-14/unsplit-group" }, PAGE_STATE, machine);
  assert.deepEqual([b.role, b.bite, b.build.features, "crate" in b.params], ["bite", "PM-0001-14/unsplit-group", ["bites"], false]);
  assert.throws(() => config.legHeader(RUN, { ...LEG, suite: "other" }, PAGE_STATE, machine), /no marks/);
  // What Node's os does not give stays empty, 0 or []; a value the page does not give is never invented.
  const bare = config.osMachine({ ...FAKE_OS, availableParallelism: undefined, cpus: () => [] });
  assert.deepEqual([bare.cpu, bare.vcpus, bare.plan, bare.threads_per_core, bare.smt], ["", 0, "", 0, []]);
  const none = config.legHeader(RUN, LEG, { version: "v" }, machine);
  assert.deepEqual([none.machine.clock_res_ns, none.params.records_dropped_count], [0, null]);
  // Y from rs.yaml's commit line, filled at Stop B; the zeros refused.
  const rs = text("cloud-init/rs.yaml").replace(/ {6}[0-9a-f]{40}\n/, `      ${"0".repeat(40)}\n`);
  assert.throws(() => config.codeCommit(rs), /not filled/);
  assert.equal(config.codeCommit(rs.replace(/ {6}0{40}\n/, `      ${"ab".repeat(20)}\n`)), "ab".repeat(20));
  // read() returns the clock step and the dropped count; save() writes legLog with the time limit's end.
  assert.equal(config.PAGE.clockResNs, "nptClockResNs");
  assert.equal(config.PAGE.dropped, "nptDropped");
  const lib = text("browser/lib.mjs");
  assert.match(lib, /nptClockResNs: window\[p\.clockResNs\] \?\? null, nptDropped: window\[p\.dropped\] \?\? null/);
  assert.match(lib, /writeFileSync\(`\$\{base\}\.jsonl`, legLog\(header, state\.records, state\.timed_out === true\)\)/);
  for (const s of ["cert14d.mjs", "fallback.mjs"]) assert.doesNotMatch(text(`browser/${s}`), /save\("/, `${s}: a summary is no leg log`);
});

test("C3: fallback re-arms the block per browser and marks a WebTransport arrival INVALID; cert-14d writes attempt.ok", () => {
  const f = text("browser/fallback.mjs");
  assert.match(f, /console\.log\(quic\("off"\)\);/);
  assert.match(f, /reached_webtransport/);
  assert.match(f, /INVALID: an attempt reached WebTransport while QUIC was meant blocked/);
  assert.match(f, /\} finally \{\n  console\.log\(quic\("on"\)\);/);
  assert.match(text("browser/cert14d.mjs"), /events\(st\.records, "attempt"\)/);
  assert.match(text("browser/mds36.mjs"), /console\.log\(quic\("on"\)\);/);
  assert.match(text("browser/lib.mjs"), /`sudo -n \/usr\/local\/sbin\/npt-ctl quic \$\{state\}`/);
});
