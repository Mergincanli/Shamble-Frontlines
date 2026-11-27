// Step 9, cut B (Builder V6, 2026-11-27): the proof that putting the frozen content into the first recording's header
// changed no state (netcode-architect's route (i)). Main 2483e9a's recording files, copied out with `git show` before
// any tool wrote, against the branch's: the 1,800 step records (122,400 bytes at the file's end) equal; the hashes
// file's lines 2 to 1,801 equal; the vector file equal byte for byte; by design the header (its content hash and map
// id), the hashes file's line 1 (it names the recording's own sha256) and the manifest's lines change.
//   node .checkpoints/cs4b-content/recordings-proof.mjs <main's copies folder> <the branch's recordings folder> <out file>
// All three are full paths. It prints its lines, writes them to <out file>, and exits 1 on any difference but the
// designed ones. Node's built-in modules only.
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
const args = process.argv.slice(2);
if (args.length !== 3 || args.some((p) => !p.startsWith('/') || p.split('/').includes('..'))) { console.error('three full paths are wanted'); process.exit(2); }
const [mainDir, branchDir, out] = args;
const read = (dir, name) => readFileSync(path.join(dir, name));
const sha = (bytes) => createHash('sha256').update(bytes).digest('hex');
const STEPS = 1800, STEP_BYTES = 68;
const lines = [];
let bad = 0;
const say = (ok, text) => { if (!ok) bad += 1; lines.push(`${ok ? 'same' : 'DIFFERS'}: ${text}`); };
const note = (text) => lines.push(`by design: ${text}`);
// The recording: the steps at the end equal; the header differs in its content hash and its map id only.
const [mainRec, rec] = [read(mainDir, 'first-loop.rec'), read(branchDir, 'first-loop.rec')];
const tail = STEPS * STEP_BYTES;
say(Buffer.compare(mainRec.subarray(mainRec.length - tail), rec.subarray(rec.length - tail)) === 0, `the ${STEPS} step records (${tail} bytes at the file's end)`);
const [mainHead, head] = [mainRec.subarray(0, mainRec.length - tail), rec.subarray(0, rec.length - tail)];
// Header: magic 8, version 2, seed 32, start tick 4, first sequence 2, tick rate 2 (50 bytes), content hash 32, the
// map id's length byte and its bytes, the step count 4.
say(Buffer.compare(mainHead.subarray(0, 50), head.subarray(0, 50)) === 0, 'the header up to the content hash (magic, version, seed, start tick, first sequence, tick rate)');
say(Buffer.compare(mainHead.subarray(mainHead.length - 4), head.subarray(head.length - 4)) === 0, `the step count (${head.readUInt32LE(head.length - 4)})`);
const idOf = (h) => h.subarray(83, 83 + h[82]).toString('utf8');
note(`the content hash: ${mainHead.subarray(50, 82).toString('hex')} (none) -> ${head.subarray(50, 82).toString('hex')}`);
note(`the map id: ${JSON.stringify(idOf(mainHead))} -> ${JSON.stringify(idOf(head))} (the file ${rec.length - mainRec.length} bytes longer)`);
const frozen = read(branchDir, 'first-loop.content.json');
say(head.subarray(50, 82).toString('hex') === sha(frozen), `the header's content hash is the frozen content's sha256 (first-loop.content.json, ${frozen.length} bytes)`);
// The hashes file: line 1 names the recording's sha256; lines 2 to 1,801 are the replay's.
const [mainHashes, hashes] = [read(mainDir, 'first-loop.hashes').toString('utf8').split('\n'), read(branchDir, 'first-loop.hashes').toString('utf8').split('\n')];
say(mainHashes.length === hashes.length && mainHashes.length === STEPS + 2, `the hashes file's ${STEPS + 1} lines and its last line end`);
let firstOff = -1;
for (let i = 1; i <= STEPS; i++) if (mainHashes[i] !== hashes[i]) { firstOff = i + 1; break; }
say(firstOff === -1, firstOff === -1 ? `the hashes file's lines 2 to ${STEPS + 1}, every step's shared and whole hash` : `the hashes file from line ${firstOff} on`);
note(`the hashes file's line 1: "${mainHashes[0]}" -> "${hashes[0]}"`);
say(hashes[0] === `shamble-hashes 1 ${sha(rec)} ${STEPS}`, "line 1 names the branch's recording by its sha256");
// The vector file, whole.
say(Buffer.compare(read(mainDir, 'first-loop.vector.txt'), read(branchDir, 'first-loop.vector.txt')) === 0, 'first-loop.vector.txt, byte for byte');
// The manifest: what changed, line by line.
const [mainManifest, manifest] = [read(mainDir, 'manifest.txt').toString('utf8').trimEnd().split('\n'), read(branchDir, 'manifest.txt').toString('utf8').trimEnd().split('\n')];
for (const line of manifest) if (!mainManifest.includes(line)) note(`a manifest line new or changed: ${line}`);
for (const line of mainManifest) if (!manifest.includes(line)) note(`a manifest line of main's gone or changed: ${line}`);
lines.push(bad === 0 ? 'every difference is one by design' : `${bad} difference(s) not by design`);
writeFileSync(out, lines.join('\n') + '\n');
console.log(lines.join('\n'));
process.exit(bad === 0 ? 0 : 1);
