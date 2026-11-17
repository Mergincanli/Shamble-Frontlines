// Change-set 0: copies Mustafa's messages into docs/DECISIONS.md, byte for byte, and builds the report's appendix
// from the same files. His words are never retyped: they come from the files that the extract scripts wrote from the
// session's transcript. Run from the repository root:
//   node .checkpoints/cs0-model-rules/decisions-insert.mjs            (writes both files, then checks them)
//   node .checkpoints/cs0-model-rules/decisions-insert.mjs --check    (checks only)
import { readFileSync, writeFileSync, existsSync } from 'node:fs';
import { createHash } from 'node:crypto';
const N = '.checkpoints/build-plan-slice-1/', C = '.checkpoints/cs0-model-rules/';
const DEC = 'docs/DECISIONS.md';
const APP = 'docs/reports/2026-11-17-build-plan-slice-1-appendix.md';
const sha = (s) => createHash('sha256').update(s).digest('hex');
const words = (path) => { // his words: everything below the first line that is exactly '---', without the final newline
  const t = readFileSync(path, 'utf8'); const i = t.indexOf('\n---\n');
  if (i < 0) throw new Error(`no --- line in ${path}`);
  return t.slice(i + 5).replace(/\n$/, '');
};
const quote = (s) => s.split('\n').map((l) => (l === '' ? '>' : `> ${l}`)).join('\n');
const unquote = (s) => s.split('\n').map((l) => (l === '>' ? '' : l.replace(/^> /, ''))).join('\n');
const wrap = (text, first, rest) => { // greedy wrap at 120 columns
  const out = []; let line = first; let empty = true;
  for (const w of text.split(' ')) {
    if (!empty && line.length + 1 + w.length > 120) { out.push(line); line = rest + w; }
    else { line += (empty ? '' : ' ') + w; empty = false; }
  }
  out.push(line); return out.join('\n');
};
const SESSION = 'the build chat, session_019YNMUV9LFhASseJ9EP12kL, on his personal account';
const COPIED = (a) => `Copied by script and checked byte for byte against \`${APP}\`, ${a}:`;
const msgs = [ // newest first
  { key: 'answers-2133', file: C + 'mustafa-2026-11-17-2133.md', app: 'A11',
    head: `- **Taken by Mustafa, 2026-11-17, 20:39:23 UTC: SIZE-0 (a), change-set 0 of the build plan goes on as one pull request, with a new stop line; LOG-HISTORY (a), the branch's history stays; and which copy of the Art lead's correction the appendix holds** (${SESSION}; his label: "My words of 2026-11-17, 21:33 (Amsterdam time)"; sent while the session's turn was running: the transcript holds it as an attachment record of 2026-11-17T20:39:23.909Z and as the queued text taken from the queue at 20:39:36.464Z, from which the script copied it). It answers two questions the build chat asked at about 20:25 UTC. SIZE-0: change-set 0 had passed twice its planned size (about 150 lines of docs planned, about 900 written), and his rule of 2026-11-14 stops a change-set there. LOG-HISTORY: five lines of a digest of the Advisor chat's log stand in the branch's history (the entry below says how they got there). ${COPIED('A11')}` },
  { key: 'notes-2058', file: C + 'mustafa-2026-11-17-2058.md', app: 'A9',
    head: `- **Taken by Mustafa, 2026-11-17, 20:03:57 UTC: four small things for the build chat: the Art lead's correction of 2026-11-15, 18:40, is the fourth Art text, and change-set 0 takes only its rule and the socket names; the Advisor chat's log is not an input; time stamps come from the clock** (${SESSION}; his label: "My words of 2026-11-17, 20:58 (Amsterdam time)"; sent while the session's turn was running; extracted by script from the session's transcript, the queued user message stamped 2026-11-17T20:03:57.522Z). It re-sends his message of 19:59:06 UTC (his label 20:20; \`${APP}\`, A8) with item 3 new. What the record shows for its item 2: one read-only agent had been started on that log at 19:16:35 UTC and had returned a digest into the chat; it wrote no file. Five lines of the digest stood in the session's checkpoint card from commit 8ea40cd (19:44:56 UTC) until commit 9b31dad (20:01 UTC) removed them; nothing of the digest is used, and the log's second copy was not opened. Of the correction's two attached copies the appendix holds the Art lead's text alone (his words of 21:33, the entry above). ${COPIED('A9')}` },
  { key: 'go', file: N + 'mustafa-2026-11-17-go.md', app: 'A3',
    head: `- **Taken by Mustafa, 2026-11-17, 19:10:44 UTC: his go on the build plan (foundation steps 2.4 and 2.5 and Slice 1), his answers to its twelve decisions, and four changes** (${SESSION}; his label: "My words of 2026-11-17, 20:10 (Amsterdam time)"; extracted by script from the session's transcript, message 79197e10-3194-466e-b811-6b7ed65a3650, from its first words to its end; the five attachment references before them are left out). It answers SPLIT-25, FIRST-SOLDIER, GATE-FOLLOW, MACHINES, BRAVE-ROUTE, PREVIEW-START, STAND-IN-LOOK, COMPARE, MODEL-PUBLIC, RULES-WHO, SCOPE-RELOAD and START-RULES as the plan asks them (\`docs/reports/2026-11-17-build-plan-slice-1.md\`, "Decisions needed"). Its answer 3 (GATE-FOLLOW) sets his words of 2026-11-12 on gate and CI work aside for the edits the build chat asks of him. Its answer 4 (MACHINES) names the desktop of his words of 2026-11-03 and puts the naming of the actual machines before change-set 6. Its answer 8 is his word for the change to PM-0004-34's wording. Its CHANGE 1 changes his words of 2026-11-14 on who records that day's decision; its CHANGE 2 holds a decision he asks to be recorded word for word (two builder chats) and changes his words of 2026-11-12 on models for the second builder chat only. The kit lines that still say otherwise (CLAUDE.md, "Models and effort") are not edited by this change-set. ${COPIED('A3')}` },
  { key: 'addition', file: N + 'mustafa-2026-11-14-addition.md', app: 'A2',
    head: `- **Taken by Mustafa, 2026-11-14, 14:12:13 UTC: his addition to his decision of that day (Blender is the route for the models), and items 11 to 13 of the build plan's task** (${SESSION}; his label: "My words of 2026-11-14, 15:03"; sent while the session's turn was running; extracted by script from the session's transcript, the queued user message stamped 2026-11-14T14:12:13.167Z). It replaces the Blender sentences of his decision in the entry below, as it says, and leaves his words of 2026-11-03 on the route (LOOK-TEST-INPUTS, "Route: Blender") unchanged. ${COPIED('A2')}` },
  { key: 'task', file: N + 'mustafa-2026-11-14-task.md', app: 'A1',
    head: `- **Taken by Mustafa, 2026-11-14, 13:58:05 UTC: the game is built now: foundation steps 2.4 and 2.5 and Slice 1 may be planned, and built after his go on the plan; with the build plan's task** (${SESSION}; his label: "My words of 2026-11-14, for a fresh chat on my personal account"; extracted by script from the session's transcript, message 55dc4ac7-b4f4-4f02-a83f-98adb1ba391d). His decision stands inside it, between quote marks ("The game is built now, ..."). Its sentences on Blender are replaced by his addition of the same day (the entry above); who records it is changed by his words of 2026-11-17 (CHANGE 1, in the entry of 19:10:44 UTC above): change-set 0 of the build plan records it, here. It changes, for these steps, his words of 2026-11-03 (SPIKE-ORDER (b)), condition 3 of SLICE-ORDER, his words of 2026-11-10 ("Steps 2.3 to 2.6 keep their order and wait for their results") and his words of 2026-11-13 ("Steps 2.4 to 2.6 still wait for their results"); dated pointers of 2026-11-17 mark those lines (in this file: SPIKE-ORDER (b), SLICE-ORDER's condition 3 and the entries of 2026-11-10 and 2026-11-13, below; SPIKE_PLAN; ADR 0002's migration step 1). The kit lines that still say otherwise (CLAUDE.md, \`.claude/agents/advisor.md\`) are not edited: his words say they are the rule there too. ${COPIED('A1')}` },
];
for (const m of msgs) m.head = wrap(m.head, '', '  ');
const TOP = '## Decisions, newest first\n';
const BEFORE52 = '- **Taken by Mustafa, 2026-11-14, 09:02:29 UTC: his go for #52**';
const COND3_END = `     still wait for their results."_\n- **Taken by Mustafa 2026-10-31 (in the v2 day session's chat`;
const POINTER = wrap(`_Pointer (2026-11-17, change-set 0 of the build plan): for foundation steps 2.4 and 2.5 and Slice 1, changed by his decision of 2026-11-14: the entry "Taken by Mustafa, 2026-11-14, 13:58:05 UTC: the game is built now", above. His words there: "Slice 3, online, still waits for the netcode proof test's result."_`, '     ', '     ') + '\n';
const once = (t, s) => { const n = t.split(s).length - 1; if (n !== 1) throw new Error(`anchor found ${n} times: ${s.slice(0, 60)}`); };
if (!process.argv.includes('--check')) {
  let t = readFileSync(DEC, 'utf8');
  if (t.includes('2026-11-17, 19:10:44 UTC: his go on the build plan')) throw new Error('already inserted');
  for (const a of [TOP, BEFORE52, COND3_END]) once(t, a);
  const entry = (m) => `${m.head}\n${quote(words(m.file))}\n`;
  t = t.replace(TOP, TOP + entry(msgs[0]) + entry(msgs[1]) + entry(msgs[2]));
  t = t.replace(BEFORE52, entry(msgs[3]) + entry(msgs[4]) + BEFORE52);
  t = t.replace(COND3_END, COND3_END.replace('\n- **', `\n${POINTER}- **`));
  writeFileSync(DEC, t);
  const fence = (s) => '````text\n' + s + '\n````\n';
  let a = `# Appendix to the build plan's report: his words and the inputs, byte for byte\n\n` +
    `_Made by \`decisions-insert.mjs\` (change-set 0) from the files that were copied by script from the build chat's\n` +
    `transcript (session_019YNMUV9LFhASseJ9EP12kL) and from his attachments. Nothing inside a fence was retyped. Under\n` +
    `each block stand its sha256 and its size, counted without the file's last line end. Everything in A4 to A7 and A10 is the Art lead's proposal unless a\n` +
    `line of Mustafa's says otherwise (his words of 2026-11-17, A3, CHANGE 3)._\n\n`;
  const sec = (id, title, body) => { a += `## ${id}. ${title}\n${fence(body)}sha256 of the block: \`${sha(body)}\` (${Buffer.byteLength(body)} bytes)\n\n`; };
  const brief = (f) => readFileSync(N + 'inputs/' + f, 'utf8').replace(/\n$/, '');
  sec('A1', 'His message of 2026-11-14, 13:58:05 UTC: the build plan\'s task, with his decision of that day', words(msgs[4].file));
  sec('A2', 'His addition of 2026-11-14, 14:12:13 UTC (his label: 15:03)', words(msgs[3].file));
  sec('A3', 'His message of 2026-11-17, 19:10:44 UTC (his label: 20:10): his go, his answers, four changes', words(msgs[2].file));
  sec('A4', 'The Art lead\'s brief as he attached it on 2026-11-17: ART_BRIEF_2026-11-14_1635_conscript_stand_in.md', brief('ART_BRIEF_2026-11-14_1635_conscript_stand_in.md'));
  sec('A5', 'The Art lead\'s brief as he attached it on 2026-11-17: ART_BRIEF_ADDITION_2026-11-15_poses_yard.md. Its section A1 is replaced by A10', brief('ART_BRIEF_ADDITION_2026-11-15_poses_yard.md'));
  sec('A6', 'The Art lead\'s brief as he attached it on 2026-11-17: ART_BRIEF_2026-11-15_1735_smg_stage1.md. Its section "How the Conscript holds it" is replaced, and its socket names and magazine corrected, by A10', brief('ART_BRIEF_2026-11-15_1735_smg_stage1.md'));
  sec('A7', 'The three Art sheets of 2026-11-14 (images sent into the chat), as this session typed their text. Its soldier sizes are replaced by the brief in A4 (his words of 2026-11-17, A3, CHANGE 3); kept because the plan\'s Part 4 cites them', readFileSync(N + 'art-sheets-2026-11-14.md', 'utf8').replace(/\n$/, ''));
  sec('A8', 'His message of 2026-11-17, 19:59:06 UTC (his label: 20:20): three small things. Re-sent with one more item as A9', words(C + 'mustafa-2026-11-17-2020.md'));
  sec('A9', 'His message of 2026-11-17, 20:03:57 UTC (his label: 20:58): four small things', words(msgs[1].file));
  sec('A10', 'The Art lead\'s correction of 2026-11-15, 18:40, "The gun poses, restated from the sockets": the Art lead\'s text alone, the copy of 3,244 bytes (his words of 2026-11-17, A11)', brief('ART_BRIEF_CORRECTION_2026-11-15_1840_gun_poses.md'));
  sec('A11', 'His message of 2026-11-17, 20:39:23 UTC (his label: 21:33): his answers to SIZE-0 and LOG-HISTORY, the correction file, the SMG line drawing', words(msgs[0].file));
  writeFileSync(APP, a);
}
// checks: every quoted block in DECISIONS.md, unquoted, equals his words; the appendix holds the same bytes
const t = readFileSync(DEC, 'utf8'); const ap = existsSync(APP) ? readFileSync(APP, 'utf8') : '';
let ok = true;
for (const m of msgs) {
  const w = words(m.file); const i = t.indexOf(m.head);
  const got = i < 0 ? null : unquote(t.slice(i + m.head.length + 1, i + m.head.length + 1 + quote(w).length));
  const inApp = ap.includes('````text\n' + w + '\n````');
  console.log(m.key, Buffer.byteLength(w), 'bytes', sha(w).slice(0, 16), 'DECISIONS:', got === w ? 'equal' : 'DIFFERENT', 'appendix:', inApp ? 'equal' : 'MISSING');
  if (got !== w || !inApp) ok = false;
}
const long = t.split('\n').filter((l) => !l.startsWith('>') && l.length > 120 && /2026-11-1[47], (13|14|19|20):\d\d:\d\d UTC|session_019YNMUV9LFhASseJ9EP12kL/.test(l)).length;
const a10 = ap.includes('(3243 bytes)') || ap.includes('(3244 bytes)');
console.log('entry header lines over 120:', long, '| A10 is the copy of 3,244 bytes:', a10 ? 'yes' : 'NO');
process.exit(ok && !long && a10 ? 0 : 1);
