// rust-reviewer r1 scratch: the test-only module's two replay exports on hostile bytes, in wasm32 (a 32-bit usize),
// in the dev profile (debug assertions, overflow checks) and in release. A panic is a trap here: it shows as a thrown
// RuntimeError, and the run stops at the first one. Every accepted input's hashes text is checked against Node's own
// SHA-256 of the bytes and against its step count.
// Usage: node rr_fuzz_wasm.mjs <module folder with dev/ and release/> <rounds>
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import path from "node:path";

const [folder, roundsText] = process.argv.slice(2);
const rounds = Number(roundsText);

/** A recording of `steps` steps with every field of the format v1 that this build replays. */
function recording(steps, mapId = "") {
  const id = Buffer.from(mapId, "utf8");
  const header = Buffer.alloc(87 + id.length);
  let at = 0;
  header.write("SHMBLREC", at, "latin1"); at += 8;
  header.writeUInt16LE(1, at); at += 2;
  header.write("TESTSEEDfuzz", at, "latin1"); at += 32;
  header.writeUInt32LE(0, at); at += 4;
  header.writeUInt16LE(0, at); at += 2;
  header.writeUInt16LE(30, at); at += 2;
  at += 32;
  header.writeUInt8(id.length, at); at += 1;
  id.copy(header, at); at += id.length;
  header.writeUInt32LE(steps, at); at += 4;
  const body = Buffer.alloc(steps * 68);
  for (let k = 0; k < steps; k++) {
    const o = k * 68;
    body.writeDoubleLE(1000 + k * 33.25, o);
    body.writeUInt32LE(k + 1, o + 8);
    body.writeUInt32LE((k * 37) & 0xffff, o + 12);
    body.writeUInt32LE((k * 0x01010101) >>> 0, o + 16);
    body.writeUInt32LE(0x0403_0201, o + 20);
  }
  return Buffer.concat([header, body]);
}

let state = 0x9e3779b97f4a7c15n;
const next = () => {
  state ^= (state << 13n) & 0xffffffffffffffffn;
  state ^= state >> 7n;
  state ^= (state << 17n) & 0xffffffffffffffffn;
  return Number(state & 0xffffffffn);
};

/** Counts that a 32-bit `usize` product would wrap: count * 68 mod 2^32 small, plus the caps' edges. */
const nastyCounts = [0, 1, 5, 6, 7, 107_999, 108_000, 108_001, 63_161_284, 63_161_285, 0x0f0f_0f10, 0x7fff_ffff, 0x8000_0000, 0xffff_ffff];

function check(glue, bytes, label) {
  const text = glue.replay_hashes(bytes);
  const tick = next() % 12;
  const state = glue.replay_state_at(bytes, tick);
  if (text.startsWith("error: ")) {
    if (!state.startsWith("error: ")) throw new Error(`${label}: hashes refused, state taken`);
    return false;
  }
  const lines = text.split("\n");
  const [mark, version, digest, count] = lines[0].split(" ");
  const want = createHash("sha256").update(bytes).digest("hex");
  if (mark !== "shamble-hashes" || version !== "1" || digest !== want) throw new Error(`${label}: first line ${lines[0]} (sha256 ${want})`);
  const steps = bytes.readUInt32LE(bytes.length - Number(count) * 68 - 4);
  if (Number(count) !== steps || lines.length !== steps + 2) throw new Error(`${label}: ${count} lines for ${steps} steps`);
  return true;
}

for (const profile of ["dev", "release"]) {
  const glue = await import(path.join(folder, profile, "devtools.js"));
  glue.initSync({ module: readFileSync(path.join(folder, profile, "devtools_bg.wasm")) });
  const good = recording(6);
  let taken = 0, refused = 0;
  if (!check(glue, good, `${profile} good`)) throw new Error("the good one is refused");
  // The counts that would wrap a 32-bit product, at the right length and at the length the wrap would give.
  for (const count of nastyCounts) {
    for (const extra of [0, 68, (count * 68) % 2 ** 32, ((count * 68) % 2 ** 32) - 68 * 6]) {
      if (extra < 0 || extra > 1 << 20) continue;
      const bytes = Buffer.concat([recording(0).subarray(0, 83), Buffer.alloc(4), Buffer.alloc(extra)]);
      bytes.writeUInt32LE(count >>> 0, 83);
      check(glue, bytes, `${profile} count ${count} extra ${extra}`) ? taken++ : refused++;
    }
  }
  // Map ids of every length byte, with and without the bytes they announce.
  for (let length = 0; length < 256; length++) {
    const bytes = recording(2, "m".repeat(Math.min(length, 64)));
    bytes.writeUInt8(length, 82);
    check(glue, bytes, `${profile} map id ${length}`) ? taken++ : refused++;
  }
  for (let round = 0; round < rounds; round++) {
    let bytes = Buffer.from(good);
    switch (round % 5) {
      case 0: case 1:
        for (let k = 0; k <= next() % 3; k++) bytes[next() % bytes.length] = next() & 0xff;
        break;
      case 2: {
        const length = next() % (bytes.length + 80);
        const out = Buffer.alloc(length, next() & 0xff);
        bytes.copy(out, 0, 0, Math.min(length, bytes.length));
        bytes = out;
        break;
      }
      case 3:
        for (let k = next() % 20; k < bytes.length; k++) bytes[k] = next() & 0xff;
        break;
      default: {
        // A host time of any bits, a held word of any bits, any count word.
        const step = next() % 6;
        bytes.writeUInt32LE(next() >>> 0, 87 + step * 68 + (next() % 2) * 4);
        bytes.writeUInt32LE(next() >>> 0, 87 + step * 68 + 4 + 8 * (next() % 7));
        if (next() % 4 === 0) bytes.writeUInt32LE(next() >>> 0, 83);
      }
    }
    check(glue, bytes, `${profile} round ${round}`) ? taken++ : refused++;
  }
  console.log(`${profile}: ${taken} taken, ${refused} refused, no trap; panic_buffer export present: ${typeof glue.panic_buffer === "function"}`);
}
