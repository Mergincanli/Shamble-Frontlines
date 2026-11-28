// room.mjs: one npt-server start for T5's browser loads (u7-brief.md rev 1.1 Block T5; advisor-u7-brief.md C9): its
// settings in a fresh folder (its own token file, log and hash file: `create_new` refuses a used one), OPEN awaited,
// its slot tokens read by slot (never printed or recorded), its end by closing stdin, its log's events read.
import { spawn } from "node:child_process";
import { existsSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { basename, join } from "node:path";

export const sleep = (ms) => new Promise((ok) => setTimeout(() => ok(null), ms));

/** A room's settings (crates/server/src/main.rs:67-128): one scene, P0 (the pass-through), the room's own WT cert. */
export function settings(dir, { nn, transport, commit, clients = 1, wtPort = null, wsCert = null }) {
  return {
    run: `T5ii-${basename(dir)}`, set: "T5ii", role: "contact", marks: [], date: new Date().toISOString().slice(0, 10),
    commit, layer: "L2", profile: "P0", seed: 7, scene: "SS", clients, cmd_timing: "B", tick_hz: 30,
    machine_role: "C", machine_name: "C", machine_plan: "container", machine_location: "here", machine_address: "",
    tick_vcpu: null, io_vcpus: [], io_threads: 2, log: join(dir, "room.jsonl"), token_file: join(dir, "tokens"),
    duration_ms: 0, warmup_ticks: 0, first_cpu_model: "", transport,
    wt_bind: `127.0.0.1:${wtPort ?? 30001 + Number(nn)}`, ws_bind: `127.0.0.1:${31001 + Number(nn)}`,
    ws_cert_chain: wsCert?.chain ?? "", ws_cert_key: wsCert?.key ?? "", hash_path: join(dir, `hash-${nn}.json`),
    ws_split_mark_s: null,
  };
}

/** Every child process the drivers started and that has not exited (rooms here; t5ii.mjs adds its npt-host). */
export const CHILDREN = new Set();

/** Keeps `child` in CHILDREN until it exits. */
export function track(child) {
  CHILDREN.add(child);
  child.on("exit", () => CHILDREN.delete(child));
  return child;
}

/** A failure's last act (U7R-NC-4): SIGKILL every child still running, and drop the load's tokens file in `dir`. */
export function reap(dir) {
  for (const c of CHILDREN) c.kill("SIGKILL");
  CHILDREN.clear();
  if (dir) rmSync(join(dir, "tokens"), { force: true });
}

/** Starts the room; `open` resolves true at its OPEN line, false at an exit or 15 s. */
export function startRoom(server, dir, s) {
  const file = join(dir, "settings.json");
  writeFileSync(file, `${JSON.stringify(s, null, 1)}\n`);
  const child = track(spawn(server, ["--settings", file], { stdio: ["pipe", "pipe", "pipe"] }));
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

/** The room's slot tokens by slot (U6a's lines `{slot} nptj_<22 base64url>`); a malformed line throws, unprinted. */
export function roomTokens(dir) {
  const lines = readFileSync(join(dir, "tokens"), "utf8").split("\n").filter(Boolean);
  return lines.map((l, i) => {
    const m = /^(\d{1,2}) (nptj_[A-Za-z0-9_-]{22})$/.exec(l);
    if (!m || Number(m[1]) !== i) throw new Error(`token file line ${i} is malformed (not printed)`);
    return m[2];
  });
}

/** Ends the room by closing its stdin (main.rs:799-829), at most 20 s, then a kill; keeps its output; drops the tokens. */
export async function stopRoom(room, dir) {
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

/** The room log's events (no token in them: SEC-U6AD-3), its record count and its footer. */
export function roomLog(file) {
  if (!existsSync(file)) return { present: false };
  const recs = readFileSync(file, "utf8").split("\n").filter(Boolean).map((l) => JSON.parse(l));
  return {
    present: true,
    records: recs.length,
    events: recs.filter((r) => r.k === "event").map((r) => ({ ev: r.ev, ph: r.ph, f: r.f ?? {} })),
    footer: recs.find((r) => r.k === "footer") ?? null,
  };
}
