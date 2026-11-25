// Writes Mustafa's words of 2026-11-25 to Builder V4 into docs/DECISIONS.md and his Hornet sentence into
// docs/UNITS.md, from the chat's saved message files (.checkpoints/cs4-walk/mustafa-2026-11-25-*.md, each copied by
// script from the chat's transcript): no word of his is typed here. From the worktree's root:
//   node .checkpoints/cs4a-recording/decisions-4a.mjs write <commit at which the saved files are readable>
//   node .checkpoints/cs4a-recording/decisions-4a.mjs check
// `write` refuses a file it has already written to. `check` reads both docs back and compares every quoted line with
// the saved message, byte for byte, and each saved message with the byte count and sha256 its own header gives.
// The text in bold, the brackets after it and the lines marked as records are this chat's, written for the file.
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';

const NOTES = '.checkpoints/cs4-walk/';
const DECISIONS = 'docs/DECISIONS.md';
const UNITS = 'docs/UNITS.md';
const CHAT = 'the builder chat he calls Builder V4, lane 1 of the build, session_019YNMUVZR6ociHFWjByxkrq, a Claude Code chat';
const ENTRIES = [
  ['mustafa-2026-11-25-stop-4a-answer.md',
    'STOP-4A (a): finish change-set 4a as it stands, the new stop line is 3,400, and the two green tests stay with the code they prove; WATCH-70 (a): the chat watches pull request #70, where a comment or review is information and never his word; 4a\'s report says in a few lines where the difference to its designs went and what it means for the sizes of 4b to 4d, whose stop lines stay as they are'],
  ['mustafa-2026-11-25-go-walk.md',
    'his go on the plan of change-set 4 (revision 4, pushed at 25c42c5): start 4a, the recording; WALK-CUT (a), four change-sets, each its own pull request; STEP-60 (a), he plays the sandbox at 30 steps and exports again, and no value is carried from the 60-step Export; a question for 4a\'s report on what the first soldier on screen does not need; every report says which size is the designs\' own count and which holds the 1.8; the Hornet\'s entry goes in as the plan says and the two lines of docs/UNITS.md stay as they are, listed as open for his unit walk; the other questions wait for the part that needs them'],
  ['mustafa-2026-11-25-export-block.md',
    'his block for his Testlines Export: save it whole by script with its byte count and sha256 before reading it; say its format and version, its build commit, what its warnings hold and how many values changed from the start; a line in the warnings means he tuned at a step rate other than the game\'s 30, to be said, and asked before values that depend on it are carried; the values are starting values, not decisions, and go one way, from the Export into the game; the Hornet\'s numbers are a sketch and no part of Slice 1'],
  ['mustafa-2026-11-25-order-3c-answer.md',
    'ORDER-3C (c), with four conditions: change-set 3c is split at its seam, the recording becomes the first step of change-set 4, Walk, and the link simulator follows Walk as its own small change-set, merged before change-set 7 starts; then change-set 4\'s plan, sized at 1.8 times with a stop line, showing the earliest playable build and whether Walk is cut in two; main 311c0484 taken in; the wider interim rule that git and gh always start their own command; the Hornet\'s entry for docs/UNITS.md section 0'],
  ['mustafa-2026-11-25-takeover.md',
    'his takeover block for the chat he calls Builder V4: lane 1 of the Slice 1 build from the merge of change-set 3b on, in twelve points (among them: the first job is the question ORDER-3C; then the plan of the stop he chooses, sized at the ratio change-sets 3a and 3b ran at, with a stop line; his Testlines values, point 8, a to g; no CI, kit or gate file is written by this chat; a merge only on his go for the head the advisor agent approved; this block recorded word for word with the first pull request)'],
];
const NOT_RECORDED = 'Not recorded here, by his own words in it: his board add-on of the same day ("Copy nothing from this block into the repository").';

const sha = (text) => createHash('sha256').update(text).digest('hex');
function saved(file) {
  const all = readFileSync(NOTES + file, 'utf8');
  const cut = all.indexOf('\n---\n');
  const note = all.slice(0, cut);
  const text = all.slice(cut + 5).replace(/\n$/, '');
  const m = note.match(/the user message of (\d{4}-\d{2}-\d{2})T(\d{2}:\d{2}:\d{2})\.\d+Z \(uuid ([0-9a-f-]{36})\), whole \((\d+) bytes, sha256 ([0-9a-f]{64})\)\. What it answers: (.*)_\s*$/s);
  if (!m) throw new Error(`${file}: its header does not read as a saved message's`);
  const [, date, time, uuid, bytes, digest, answers] = m;
  if (Buffer.byteLength(text) !== Number(bytes) || sha(text) !== digest) throw new Error(`${file}: the message is not the ${bytes} bytes with sha256 ${digest} its header gives`);
  return { date, time, uuid, bytes, digest, answers: answers.trim().replace(/\.$/, ''), text };
}
function wrap(paragraph, indent = '  ', width = 118) {
  const lines = [];
  let line = '';
  for (const word of paragraph.split(' ')) {
    const next = line === '' ? word : `${line} ${word}`;
    if (next.length > width - (lines.length === 0 ? 0 : indent.length) && line !== '') { lines.push(line); line = word; } else line = next;
  }
  lines.push(line);
  return lines.map((l, i) => (i === 0 ? l : `${indent}${l}`)).join('\n');
}
const quoted = (text) => text.split('\n').map((l) => (l === '' ? '>' : `> ${l}`)).join('\n');
function entry(file, summary, commit) {
  const s = saved(file);
  const label = s.text.split('\n')[0];
  const extra = file.includes('export-block') ? ' The Export itself came in his next message (2026-11-25, 17:51:49 UTC) and is saved whole by script as `.checkpoints/cs4-walk/testlines-export.json` (15,567 bytes, sha256 9ad1d9f914acc27c56cbfbe7c4e6c7e654ba24fa0e9ae75cb4e3b93103f06498); by his STEP-60 (a) no value of it is carried, and it is not recorded here.' : '';
  const last = file.includes('takeover') ? ` ${NOT_RECORDED}` : '';
  const head = `- **Taken by Mustafa, ${s.date}, ${s.time} UTC: ${summary}** (${CHAT}; his label: "${label}"; message ${s.uuid}, whole; copied by script from the chat's saved file \`${NOTES}${file}\` (readable at commit ${commit} of the chat's branch \`claude/cool-pasteur-oiqk1r\`), which the chat copied by script from its transcript). What it answers, in the chat's words (not his): ${s.answers}.${extra}${last} Copied by script (${s.bytes} bytes, sha256 ${s.digest}):`;
  return { ...s, block: `${wrap(head)}\n${quoted(s.text)}\n` };
}
const HORNET_MARK = 'The Hornet is a tier-one unit of the Empire';
// The two lines of section 0 the Hornet's entry meets: quoted in it, and checked to stand in the file as they are.
const KEPT = ['the counter triangle and tier one stay shared; the differences grow with tech (tier-two specialists and vehicles are unique)', 'Empire: a hover skimmer as its first vehicle'];
function hornet() {
  const s = saved('mustafa-2026-11-25-order-3c-answer.md');
  const m = s.text.match(/recorded by script from this message: "(The Hornet[^"]*)"/);
  if (!m) throw new Error('his Hornet sentence is not in his answer');
  // His later words on the two lines, from his go; and the two lines themselves, which must stand in the file as quoted.
  const stay = saved('mustafa-2026-11-25-go-walk.md').text.match(/The two lines of docs\/UNITS\.md stay as they are[^.]*/);
  if (!stay) throw new Error('his words on the two lines are not in his go');
  const flat = readFileSync(UNITS, 'utf8').replace(/\n {2,4}/g, ' ');
  for (const line of KEPT) if (!flat.includes(line)) throw new Error(`${UNITS} does not hold the line "${line}"`);
  const lines = [
    `- \`[DECIDED]\` Empire: the Hornet. In his words: "${m[1]}" The unit is decided; its numbers are a sketch to tune, not decided. It is not part of Slice 1 and nothing of it is built there. (2026-11-25, 16:59:52 UTC; whole in \`docs/DECISIONS.md\`, his answer ORDER-3C, point 6)`,
    `  - Open, for his unit walk (listed, never settled here: his words of 2026-11-25, "${stay[0]}"): this entry meets two lines of this section. Under Factions, decided: "${KEPT[0]}", while the Hornet is a tier-one unit of the Empire alone. And the idea just above: "${KEPT[1]}", while the Hornet is a vehicle at tier one. Both lines stay as they are.`,
  ];
  // The second line is a bullet under the first: `wrap` drops leading spaces, so its indent is put back here.
  return { sentence: m[1], block: `${wrap(lines[0])}\n  ${wrap(lines[1].trimStart(), '    ', 116)}\n` };
}

const [mode, commit] = process.argv.slice(2);
if (mode === 'write') {
  if (!/^[0-9a-f]{40}$/.test(commit ?? '')) throw new Error('write needs the full commit id at which the saved files are readable');
  let text = readFileSync(DECISIONS, 'utf8');
  for (const [file, summary] of ENTRIES) {
    const e = entry(file, summary, commit);
    if (text.includes(e.uuid)) throw new Error(`${file} is already in ${DECISIONS}`);
    // Newest first: before the first entry of the list that is older than this one.
    const lines = text.split('\n');
    const from = lines.findIndex((l) => l === '## Decisions, newest first') + 1;
    let at = lines.findIndex((l, i) => i >= from && l.startsWith('## '));
    for (let i = from; i < at; i += 1) {
      const m = lines[i].match(/^- \*\*Taken by [^,]+, (\d{4}-\d{2}-\d{2})(?:, (\d{2}:\d{2}:\d{2}) UTC)?/);
      if (m && `${m[1]} ${m[2] ?? '00:00:00'}` < `${e.date} ${e.time}`) { at = i; break; }
    }
    lines.splice(at, 0, ...e.block.replace(/\n$/, '').split('\n'));
    text = lines.join('\n');
    console.log(`${file}: ${e.date} ${e.time} UTC, before line ${at + 1}`);
  }
  writeFileSync(DECISIONS, text);
  const units = readFileSync(UNITS, 'utf8');
  if (units.includes(HORNET_MARK)) throw new Error(`the Hornet is already in ${UNITS}`);
  const anchor = units.match(/- `\[IDEA\]` Empire: a hover skimmer as its first vehicle[^\n]*\n(?: {2}[^\n]*\n)*/);
  if (!anchor) throw new Error('the hover skimmer line of section 0 was not found');
  writeFileSync(UNITS, units.replace(anchor[0], anchor[0] + hornet().block));
  console.log(`${UNITS}: the Hornet's entry after the hover skimmer line of section 0`);
} else if (mode === 'check') {
  const text = readFileSync(DECISIONS, 'utf8');
  for (const [file] of ENTRIES) {
    const s = saved(file);
    // The entry's bracket is wrapped, so it is found by the message's id, and its end by the digest that closes it.
    const at = text.indexOf(s.uuid);
    const block = quoted(s.text) + '\n';
    const end = `${s.digest}):\n`;
    const start = text.indexOf(end, at);
    if (at < 0 || start < 0 || text.slice(start + end.length).slice(0, block.length) !== block) throw new Error(`${file}: its words in ${DECISIONS} are not the saved message's, byte for byte`);
    console.log(`${file}: ${s.bytes} bytes, sha256 ${s.digest.slice(0, 12)}..., in ${DECISIONS} word for word`);
  }
  const units = readFileSync(UNITS, 'utf8');
  const h = hornet();
  const flat = units.replace(/\n {2,4}/g, ' ');
  if (!flat.includes(`"${h.sentence}"`)) throw new Error(`his Hornet sentence in ${UNITS} is not the message's`);
  for (const kept of KEPT) if (flat.split(kept).length !== 3) throw new Error(`${UNITS}: the line "${kept}" is not there once as itself and once as quoted`);
  console.log(`${UNITS}: his Hornet sentence word for word; the two lines it meets are unchanged`);
} else throw new Error('usage: decisions-4a.mjs write <commit> | check');
