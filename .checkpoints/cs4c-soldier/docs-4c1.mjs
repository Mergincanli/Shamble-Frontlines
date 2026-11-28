// docs-keeper's script for 4c-1's docs pass (2026-11-28): STATUS's replaced lines and #79's row move to the archive
// byte for byte (by line ranges, never retyped); his two marks are quoted from his saved message by script; the
// archive gains 4c-1's follow-ups, risks and health row; ROADMAP gains its entry and folds its oldest by line range.
// Run once from the worktree's root: node .checkpoints/cs4c-soldier/docs-4c1.mjs
import { readFileSync, writeFileSync } from 'node:fs';

const STATUS = 'docs/STATUS.md';
const ARCHIVE = 'docs/STATUS_ARCHIVE.md';
const ROADMAP = 'docs/ROADMAP.md';
const ANSWERS = '.checkpoints/cs4-walk/mustafa-2026-11-28-answers.md';
const DECISIONS = 'docs/DECISIONS.md';
const fail = (m) => { throw new Error(m); };

const s = readFileSync(STATUS, 'utf8').split('\n');
const a = readFileSync(ARCHIVE, 'utf8').split('\n');
const r = readFileSync(ROADMAP, 'utf8').split('\n');
const L = (arr, n) => arr[n - 1];
const expect = (arr, n, start, file) => { if (!L(arr, n).startsWith(start)) fail(`${file}:${n} is not as read: ${L(arr, n).slice(0, 80)}`); };

// STATUS as read (lines 1-indexed).
expect(s, 5, "_Last updated: 2026-11-27 (change-set 4b-2's docs pass", STATUS);
expect(s, 13, '- **Last merged** (read with git, 2026-11-27): #78', STATUS);
expect(s, 34, '| none yet | change-set 4b-2, "The world"', STATUS);
expect(s, 39, "2. v2: the netcode proof test (#40); step 2.3's chat stays stopped", STATUS);
expect(s, 40, "   (a)): 4a and 4b-1 merged; **4b-2 reviewed**", STATUS);
expect(s, 47, '- 4b-2, built and reviewed (its report): his go.', STATUS);
expect(s, 97, '- **From change-sets 4b-1 and 4b-2**', STATUS);
expect(s, 103, "  4B-HEAPLESS; the advisor agent's 4B-FIRST-DEFAULT", STATUS);
expect(s, 104, '', STATUS);
expect(s, 105, '## Open risks', STATUS);
const RISK_OLD = "4b-2's (four, in its report and the archive).";
if (!L(s, 116).includes(RISK_OLD)) fail(`${STATUS}:116 lacks the risks' 4b-2 entry`);
if (s.length !== 121 || L(s, 121) !== '') fail(`${STATUS} is not 120 lines`);

// His item 12 b and c, from his saved message, checked against the register.
const ans = readFileSync(ANSWERS, 'utf8').split('\n');
const dec = readFileSync(DECISIONS, 'utf8').split('\n');
const take = (line, mark) => { if (!line.startsWith(mark)) fail(`not his item: ${line.slice(0, 40)}`); return line.slice(mark.length); };
const B = take(L(ans, 25), '   b. ');
const C = take(L(ans, 26), '   c. ');
if (take(L(dec, 220), '>    b. ') !== B || take(L(dec, 221), '>    c. ') !== C) fail('the register does not hold his item 12 b and c as saved');
if (!B.startsWith('PM-0004-60: ') || !C.startsWith("PM-0004-20's 48-soldier run: ")) fail('not the two marks');

// The new STATUS lines (docs-keeper's words, but the two quotes).
const NEW5 = "_Last updated: 2026-11-28 (change-set 4c-1's docs pass, by lane 1's docs-keeper, on code 0645432f: 4c-1, the page's scene, built and reviewed in two rounds, its row and follow-ups; his two open marks of 2026-11-28, quoted by script; HEADROOM-FEET open; #79 (4b-2) merged, its row, the old top line, \"Last merged\" and lane 1's replaced lines moved to the archive by script under OLD-LINES)._";
const NEW13 = '- **Last merged** (read with git, 2026-11-28): #79, 4b-2 (dae6d107, 2026-11-27 23:12:03 UTC); #78, ci-time (b8f02b0); #75, 4b-1 (b9aad01); #77 and #76, main-wall (47bb79e, 2671595); #74, ci-preview-handover (3b949c1a); #73, guard-wrappers (4a62873e); #70, 4a (2483e9a).';
const NEW34 = "| none yet | change-set 4c-1, \"The page's scene\" (the first of 4c's two pull requests, his SIZE-4C (b)), by lane 1 (the chat he calls Builder V6): step 0 (his words of 2026-11-27 and 2026-11-28 in the register by script; LOOP_V2's §14 with its dated pointers, EDITOR_V2's pointer) and step S (the page reads the test scene's map field by field and draws it in greys by height under an overview camera; the readout's content and scene rows) | `claude/ecstatic-wozniak-5cbdzs` (stands in for `cs/4c-soldier`); his Advisor opens the pull request from a branch of its own at the scene's final head (his item 6 c) | risk tier 1, needs-mustafa (his Brave check; LOOP_V2 and the register are decision docs); built; reviewed in two rounds (six reviewers, then graphics-engineer and test-engineer on what changed, both APPROVE); planted defects 45 of 45 caught, 3 of 3 controls held (on 2b2f44c2); next: the verifier, the report, the advisor agent on the final head | his Brave check on Sunday 29 November on that head, then his go; 4c-2 (the soldier, steps 1 to 7) is built on top meanwhile (his item 6 b) |";
const NEW39 = L(s, 39);
const NEW40 = "   (a)): 4a, 4b-1 and 4b-2 merged; **4c-1, the page's scene, reviewed**; then 4c-2 (the soldier, on the same branch), 4d, the link simulator, change-set 7 (ROADMAP, \"Next / open\").";
const NEW47 = "- 4c-1, the page's scene: his Brave check on Sunday 29 November (art-director's six steps, about 6 minutes, come with its final head), then his go for that head. Open, needed before 4c-2's movement step: HEADROOM-FEET (question 13 of 4c's question file): may the game differ from his sandbox where the top of a block at or below the soldier's feet would stop him like a wall? Answered on 2026-11-28 (`docs/DECISIONS.md`, 11:43:45 UTC): MOVE-RULES (its row 9, the slopes, recorded there as open), NO-SOURCE, RESUME-FLASH, TUNE-HOME. Still open from 4a's report: SPRINT-DELAY-4; `docs/UNITS.md` §0's two lines (his unit walk).";
const NEW97 = [
  "- **From change-sets 4b-1 and 4b-2** (the archive's blocks of those names): 4B-AIM-REACH (the loader refuses a playable map",
  "  beyond the aim's reach; the aim widens at the latest in Slice 3's change-set that sets the command codec, PM-0001-51; no",
  "  numbered change-set holds it yet) and 13 more of 4b-1's, 4B-SIGHT-MARK among them. 4b-2's: for 4c, 4B-QUERY-RECORDING,",
  "  4B-RAY-WASM, 4B-RAY-COUNTS, 4B-WORLD-ROOM, 4B-LIMIT-IMPORT; the CI Builder's 4B-CHROMIUM-TIME; hosting's 4B-CONTENT-CACHE,",
  "  4B-LICENCES; 4B-HEAPLESS; the advisor agent's 4B-FIRST-DEFAULT, 4B-UNIT-COLUMN; 4B-WALLCLOCK, 4B-SERVE-LINKS. Met by 4c-1, closed when it merges: 4B-TYPES-READER (for the keys), 4B-MEMBER-KINDS, 4B-BOOT-CALLS.",
  "- **From change-set 4c-1** (the archive's block of that name; each owned by the reviewer who found it): GE-F2 (graphics-engineer: the depth range, with its round 2 note on a window a few pixels wide; for the first large map); GE-F3 with P-2 (graphics-engineer, perf-auditor: draw calls on the largest map, merge the floor by material; with the look's change-set); GE-F4 (graphics-engineer: the WebGPU pipelines built when the map arrives; no change-set named); AD-4 (art-director: the readout over a short window; at 4c-2's step 4, where the overview leaves); TE2-1 to TE2-5 (test-engineer: five gaps in the scene's tests; no change-set named).",
  `- **His two open marks of 2026-11-28** (his DEPART-4C terms, items 12 b and c, quoted by script from his saved message, as in \`docs/DECISIONS.md:220-221\`): "${B}" and "${C}" Neither is named yet: no plan names the change-set that brings the nightly 10 million or the first build that can show 48 soldiers (the build plan's rows, \`docs/reports/2026-11-17-build-plan-slice-1.md:654\` and \`:626\`, name none; read 2026-11-28).`,
];
const NEW116 = L(s, 116).replace(RISK_OLD, "4b-2's (four, in its report and the archive); 4c-1's (two, in the archive: nothing seen on WebGPU; the browser run's time).");

const moved = [s[4], s[12], s[33], s[38], s[39], s[46], ...s.slice(96, 103), s[115]];
const ns = [
  ...s.slice(0, 4), NEW5, ...s.slice(5, 12), NEW13, ...s.slice(13, 33), NEW34, ...s.slice(34, 38), NEW39, NEW40,
  ...s.slice(40, 46), NEW47, ...s.slice(47, 96), ...NEW97, ...s.slice(103, 115), NEW116, ...s.slice(116),
];
if (ns.length - 1 > 120) fail(`STATUS would be ${ns.length - 1} lines`);

// The archive: the In flight block (before the blank line ahead of "## Targets"), the follow-ups, the risk, the health row.
const at = (arr, text) => { const i = arr.indexOf(text); if (i < 0 || arr.indexOf(text, i + 1) >= 0) fail(`not once: ${text.slice(0, 60)}`); return i; };
const targets = at(a, '## Targets');
if (a[targets - 1] !== '' || !a[targets - 2].startsWith('  navigation; the editor and map bugs')) fail('the In flight section does not end as read');
const IN_FLIGHT = [
  '',
  "_Moved from STATUS (2026-11-28, change-set 4c-1's docs pass, by script, byte for byte): under his OLD-LINES (a) of 2026-11-18, the lines about merged pull requests: the row of 4b-2 (merged as #79, merge commit dae6d107, 2026-11-27 23:12:03 UTC by the merge commit's date, read with git), the top line and \"Last merged\" as they stood; and lane 1's own lines this pass replaced in STATUS, so their words stay on record: \"Next\" item 2, the \"Waiting on Mustafa\" bullet on 4b-2, the follow-ups bullet \"From change-sets 4b-1 and 4b-2\", and the \"Open risks\" line that 4c-1's risks joined._",
  '',
  '| PR | Work | Branch | State | Waiting on |',
  '|---|---|---|---|---|',
  s[33],
  '',
  s[4],
  '',
  s[12],
  '',
  s[38], s[39],
  '',
  s[46],
  '',
  ...s.slice(96, 103),
  '',
  s[115],
];
const fuEnd = at(a, '## Decisions pending: the rest');
if (a[fuEnd - 1] !== '' || !a[fuEnd - 2].startsWith('    `packages/client/serve.test.mjs` still skips')) fail('the Follow-ups section does not end as read');
const FOLLOW = [
  "  - Met by 4c-1 (engine-architect's review of step S, `.checkpoints/cs4c-soldier/review-s-engine-architect.md` lines 29, 31, 41 and 50 to 51), closed when 4c-1 merges: 4B-TYPES-READER, met for the keys (`readMap` gives `Outcome<MapDocument>` from typed literals, `packages/client/src/app/content.ts:163`), 4B-MEMBER-KINDS (each member's kind compared, `packages/types/src/content-schema.test.mjs:152`) and 4B-BOOT-CALLS (the plants E1 and E3 caught, `.checkpoints/cs4c-soldier/plants-4c.txt` lines 25 and 28) [verified 2026-11-28 at 0645432f].",
  "- **From change-set 4c-1, \"The page's scene\"** (lane 1, the chat he calls Builder V6; branch `claude/ecstatic-wozniak-5cbdzs`",
  "  standing in for `cs/4c-soldier`; his Advisor opens the pull request from a branch of its own at the scene's final head,",
  "  his item 6 c; risk tier 1, needs-mustafa; step 0 and step S; reviewed in two rounds, six reviewers, then",
  "  graphics-engineer and test-engineer on what changed, both APPROVE; code head 0645432f; the planted defects' whole run",
  "  on 2b2f44c2 caught 45 of 45 and held 3 controls of 3, `.checkpoints/cs4c-soldier/plants-4c.txt`. Written by 4c-1's",
  "  docs pass, 2026-11-28, from `.checkpoints/cs4c-soldier/review-s-round.md`: the fix pass's \"Deferred\" and round 2. The",
  "  ids are the reviewers'; each item's owner is the reviewer who found it. The notes files leave the tree at the pull",
  "  request's close and stay readable at the branch's commits):",
  "  - GE-F2 (graphics-engineer, Low; for the first large map): `near` stays 0.1 m while `far` follows the camera's",
  "    distance, so on the largest map the loader allows a 24-bit depth step is 0.3 to 1.4 m and the grid shows through",
  "    blocks under about half a metre (the test scene's step: 8 mm). Its round 2 note, folded in here: a window a few",
  "    pixels wide still frames at a great distance with near 0.1 (depth fighting on a floor about a pixel wide).",
  "  - GE-F3 with P-2 (graphics-engineer and perf-auditor, Low; with the look's change-set, before maps grow): one object",
  "    per shape and per patch gives about 579 draw calls on the loader's largest map (512 shapes, 64 regions; inferred",
  "    from the test scene's 31 objects and 32 draws, not measured), 2.9 times the world's share of 202 in ADR 0003's",
  "    draft; merge the floor by material (which breaks the inventory's rule of one buffer per object).",
  "  - GE-F4 (graphics-engineer, Low; no change-set named): the first frame after `setContent` builds two new WebGPU",
  "    pipelines (one per new depth offset), a one-time hitch at the match's start; `compileAsync` before the swap would",
  "    avoid it. Not measured (no WebGPU adapter in the container).",
  "  - AD-4 (art-director, Medium, deferred; at 4c-2's step 4): the readout panel, see-through near-black, ends at 213 px",
  "    in every window size tried, so in a window under about 600 px tall it lies over the houses and the yard's back row",
  "    and darkens them. The overview camera is the scene's alone and leaves at step 4, where the follow camera replaces",
  "    it; his Brave steps ask for a maximised window meanwhile.",
  "  - TE2-1 to TE2-5 (test-engineer, Low, round 2, measured on a scratch copy; no change-set named; scene changes after",
  "    his Brave check go into 4c-2, his item 6 d): TE2-1, `frameCamera` may drop the turn toward the map",
  "    (`packages/client/src/render/scene.ts:318`) or the projection update (`:320`) with all 11 scene tests passing;",
  "    TE2-2, the search's limit is tested from one side only (at 10 in place of 19 every test passes, yet a 9:16 window",
  "    on the largest map gets no camera); TE2-3, the readout's \"refused\" wiring (`packages/client/src/app/boot.ts:166`)",
  "    is reached by no test; TE2-4, the grid test checks its lines' count and ends, not where they stand; TE2-5, the",
  "    overview's aim height is never checked on the raised map (`scene.ts:274`) [the lines verified 2026-11-28 at",
  "    0645432f].",
];
const riskAt = at(a, '     recorded here [assumed: open].');
if (!a[riskAt + 1].startsWith('- **Change-set 4a of the v2 build plan')) fail('the risks do not run as read');
const RISK = [
  "- **Change-set 4c-1 of the v2 build plan, \"The page's scene\"** (2026-11-28, no pull request number yet; the evidence:",
  "  `.checkpoints/cs4c-soldier/review-s-round.md` and the card `.checkpoints/cs4c-soldier.md`, folded into its report",
  "  before the pull request is ready; the follow-ups: the block \"From change-set 4c-1\" in \"Follow-ups\"):",
  "  1. Nothing was seen on WebGPU: the container's Chromium has no WebGPU adapter, so the WebGPU render counts",
  "     (`packages/e2e/tests/skeleton/render-counts.json` holds null for them), the pipelines built when the map arrives",
  "     (GE-F4) and a canvas 0 wide on WebGPU are unmeasured; every picture is the WebGL 2 fallback on a software",
  "     renderer. His Brave check is final.",
  "  2. The container's whole browser run took 217 s on bc7720f3 (the card), against CI's 5-minute Chromium limit",
  "     (4B-CHROMIUM-TIME); by his CHROME-TIME (`docs/DECISIONS.md`, 2026-11-28) the CI Builder raises it after Tuesday",
  "     1 December, 20:00, and until then no test is cut or thinned to fit.",
];
const health = at(a, '| CI on main, latest (the rows below are older) | `gate`, `smoke` and `toolchain` success on 2483e9a, PR #70\'s merge commit (push; gate run 45818982040 with its gate and smoke jobs, completed 12:23 UTC; toolchain run 45818982052, 12:28 UTC) | 2026-11-26 | GitHub\'s REST API, the commit\'s workflow runs and the gate run\'s jobs (read by ci-preview-handover\'s docs-keeper) |');
const HEALTH = "| Change-set 4c-1 (no pull request yet): the page drawing the test scene | render counts on the WebGL 2 fallback (the container's Chromium, a software renderer): the empty floor 3 draw calls, 3 triangles, 62 lines; the test scene 32 draw calls, 267 triangles, 162 lines, 32 geometries, 3 textures, 6 programs (`packages/e2e/tests/skeleton/render-counts.json` [verified 2026-11-28 at 0645432f]); WebGPU's not measured (no adapter); the whole page's allocation +4.1 KB a frame with the test scene (base 41.7 KB, head 45.4 to 46.3 KB, 3 runs each), the hand-off unchanged at about 93 B a frame; the players' WebAssembly unchanged at 257,421 B; the development bundle +7,344 B raw, +2,649 B gzip | 2026-11-28 | perf-auditor's review of step S (`.checkpoints/cs4c-soldier/review-s-round.md` lines 34 to 38); not re-measured by docs-keeper |";

const na = [
  ...a.slice(0, targets - 1), ...IN_FLIGHT, ...a.slice(targets - 1, fuEnd - 1), ...FOLLOW, ...a.slice(fuEnd - 1, riskAt + 1),
  ...RISK, ...a.slice(riskAt + 1, health), HEALTH, ...a.slice(health),
];
for (const line of moved) if (!na.includes(line)) fail(`a moved line is not in the archive: ${line.slice(0, 60)}`);

// ROADMAP: the oldest "Recent work" entry (the preview hosting check) folds into "Build history" by line range, after
// change-set 3a's entry; 4c-1's entry goes on top; the pointer under 4b-2's entry; the dated pointer in "Next / open".
const recent = at(r, '## Recent work');
const oldStart = r.findIndex((l) => l.startsWith('**The preview hosting check, ADR 0006\'s spike 1 [2026-11-15 to 11-25, MERGED]**'));
if (oldStart < 0 || r[oldStart - 1] !== '') fail('the oldest entry is not where read');
let oldEnd = r.length - 1; while (r[oldEnd] === '') oldEnd -= 1;
if (!r[oldEnd].endsWith('with the expected head d7937de._')) fail('the oldest entry does not end the file as read');
const fold = r.slice(oldStart, oldEnd + 1);
const histAt = r.findIndex((l) => l.startsWith('- **Kit: KEPT-LOGS (2026-11-15):**'));
if (r[histAt - 1] !== '' || !r[histAt - 2].startsWith('UTC).')) fail('Build history does not run as read');
const nextAt = r.findIndex((l) => l.startsWith('- **Superseded by v2**'));
if (!r[nextAt - 1].startsWith('  then 4c, 4d and the link simulator (`docs/STATUS.md`, "Next").')) fail('Next / open does not run as read');
const p4b2 = r.findIndex((l) => l === 'Report `docs/reports/2026-11-27-cs4b2-world.md`.');
if (p4b2 < recent || r[p4b2 + 1] !== '') fail('the 4b-2 entry does not end as read');
if (r[recent + 1] !== '*(Newest first, a few lines each. Older items fold into "Build history" above.)*' || r[recent + 2] !== '') fail('Recent work does not start as read');
const ENTRY = [
  "**Change-set 4c-1 of the build plan, The page's scene [2026-11-28, reviewed]** — cs4c-soldier, the first of 4c's two",
  "pull requests (his SIZE-4C (b)), by lane 1 (the chat he calls Builder V6). Step 0: his words of 2026-11-27 and",
  "2026-11-28 in the register by script; LOOP_V2's §14 with its dated pointers and EDITOR_V2's pointer (his DESIGN-CLASH",
  "(a)). Step S, 4b's old step 12 (his STEP12-GO (a)): the page reads the test scene's map field by field and draws it as",
  "plain grey blocks, a higher top never darker, the street and grass in greys of their own (his STREET-GREY (b)), under",
  "an overview camera that frames the whole map and refuses a window of no width; the readout's content and scene rows.",
  "Two review rounds; planted defects 45 of 45 caught, 3 controls held. Next: his Brave check (Sunday 29 November), and",
  "4c-2, the soldier, on the same branch (STATUS).",
  '',
];
const POINTER_4B2 = '_Pointer (2026-11-28, 4c-1\'s docs pass): merged as PR #79, merge commit dae6d107 (its own time 2026-11-27 23:12:03 UTC, read with `git log`)._';
const NEXT_POINTER = [
  "- **Pointer, 2026-11-28 (change-set 4c-1's docs pass; not his words):** change-set 4b-2 is merged (PR #79); 4c is two",
  "  pull requests by his SIZE-4C (b) (`docs/DECISIONS.md`, 2026-11-28, 11:43:45 UTC): 4c-1, the page's scene, is",
  "  reviewed and waits for his Brave check; 4c-2, the soldier (steps 1 to 7), follows on the same branch; then 4d and",
  "  the link simulator (`docs/STATUS.md`, \"Next\").",
];
const nr = [
  ...r.slice(0, histAt - 1), '', ...fold, ...r.slice(histAt - 1, nextAt), ...NEXT_POINTER, ...r.slice(nextAt, recent + 3),
  ...ENTRY, ...r.slice(recent + 3, p4b2 + 1), POINTER_4B2, ...r.slice(p4b2 + 1, oldStart - 1), '',
];
for (const line of fold) if (nr.filter((l) => l === line).length !== 1) fail(`a folded line is not there once: ${line.slice(0, 60)}`);

writeFileSync(STATUS, ns.join('\n'));
writeFileSync(ARCHIVE, na.join('\n'));
writeFileSync(ROADMAP, nr.join('\n'));
console.log(`STATUS ${ns.length - 1} lines; archive +${na.length - a.length} lines; ROADMAP ${r.length} -> ${nr.length} lines; ${moved.length} STATUS lines moved, ${fold.length} ROADMAP lines folded; quotes: ${B.length} and ${C.length} characters`);
