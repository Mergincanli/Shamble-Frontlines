// docs-keeper (review-tracks docs pass): adds five entries to docs/DECISIONS.md, newest first. His words come byte for
// byte from the saved files at commit 6a630c25 (git show, as bytes); the chat's words come by line ranges from its notes.
// Each header is one physical line (track.mjs's wordsCheck reads the header up to the quote). Run from the worktree root:
//   node .checkpoints/review-tracks/docs-keeper-decisions.mjs
// Refuses to run twice (each new first line must be absent) and checks each anchor once.
import { execFileSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';
import { createHash } from 'node:crypto';

const COMMIT = '6a630c2529338d7bea8aa2ceb060a87f91d09e29';
const DIR = '.checkpoints/review-tracks';
const DOC = 'docs/DECISIONS.md';
const CHAT = 'the chat he calls the Guard fix chat, session_019YNMUUc6oiiNVygR9H56Q7, a Claude Code chat on the branch `claude/confident-gates-tbnq38`';
const saved = (f) => execFileSync('git', ['show', `${COMMIT}:${DIR}/${f}`]); // a Buffer: exact bytes
const lines = (f, a, b) => readFileSync(`${DIR}/${f}`, 'utf8').split('\n').slice(a - 1, b).map((l) => l.trim()).filter((l) => l !== '').join(' ');
const ctx = (a, b) => lines('answers-context.md', a, b);
const offered = (a, b) => lines('chat-2026-11-26-option-d-offered.md', a, b);
const CTX_NOTE = 'its lines from the chat\'s notes file `.checkpoints/review-tracks/answers-context.md` (written from its transcript for this docs pass), joined into one line, list marks kept';

function entry(head, uuid, file, answers, records) {
  const buf = saved(file);
  let text = buf.toString('utf8');
  if (Buffer.byteLength(text) !== buf.length) throw new Error(`${file}: not plain UTF-8`);
  if (text.includes('\r')) throw new Error(`${file}: carriage return`);
  const sha = createHash('sha256').update(buf).digest('hex');
  const body = text.endsWith('\n') ? text.slice(0, -1) : text;
  const quote = body.split('\n').map((l) => (l === '' ? '>' : `> ${l}`));
  const header = `- **Taken by Mustafa, ${head}** (${CHAT}; message ${uuid}, whole; copied by script from the chat's saved file \`${DIR}/${file}\` (readable at commit ${COMMIT}), which the chat saved by script from its transcript). What it answers, in the chat's words (not his): ${answers}${records ? ` Records, not his words: ${records}` : ''} Copied by script (${buf.length} bytes, sha256 ${sha}):`;
  if (header.includes('\n')) throw new Error('header must be one line');
  return { first: header, text: [header, ...quote].join('\n'), sha, bytes: buf.length };
}

const E1 = entry(
  '2026-11-26, 20:43:30 UTC: PLAN-GO (a), his go on the review-tracks plan in its full version; WAKE-END (a): a pull request whose building chat has ended waits for one line from him to a new chat; and MERGE-PROMPT (A), every merge without a prompt, which his MERGE-PROMPT (d) of 20:58:55 UTC (above) replaced, so MERGE-PROMPT (A) is not in force; PLAN-GO (a) and WAKE-END (a) stand',
  '2d747465-b4f4-49e9-9c26-e0bd2148196c', 'mustafa-2026-11-26-plan-go-answers.md',
  `the chat's three questions on the plan (${CTX_NOTE}): "${ctx(5, 24)}"`,
  'the plan he answered is `.checkpoints/review-tracks/plan.md` (679 lines then, the chat\'s card, step 11; updated for his answers afterwards, step 14); the chat asked him once to confirm MERGE-PROMPT (A) against (c), since the options were listed (c) first (card, step 12); his question of 20:49:39 UTC and his proposal of 20:50:45 UTC (above) followed.');

const E2 = entry(
  '2026-11-26, 20:49:39 UTC: his question, before he decides, on what it means if the guard ever fails to load (in plain terms); no decision',
  '17f11dda-16e0-434d-a9df-b390d13e596c', 'mustafa-2026-11-26-guard-fail-question.md',
  `the chat's request to confirm MERGE-PROMPT (A) (${CTX_NOTE}): "${ctx(29, 35)}"`,
  'the chat answered in the chat (its card, step 13).');

const E3 = entry(
  '2026-11-26, 20:50:45 UTC: his proposal, the basis of option (d): the four conditions he lists (exactly the approved version; gate, smoke and toolchain green; not a subagent merging; the PASS comment real) without the check of which files may merge without him',
  '376dac3c-777d-470b-879e-0781daaa2b7c', 'mustafa-2026-11-26-variant-d-proposal.md',
  `the chat's second request to confirm MERGE-PROMPT (A) (${CTX_NOTE}): "${ctx(39, 43)}"`,
  'the chat offered it to him as option (d), which he chose at 20:58:55 UTC (above).');

const E4 = entry(
  '2026-11-26, 20:58:55 UTC: MERGE-PROMPT (d), replacing his MERGE-PROMPT (A) of 20:43:30 UTC (below): the option the chat offered as his version: the four checks (the approved version; gate, smoke and toolchain green; not a subagent merging; a real, unedited PASS comment of his Advisor), no file check, and the PASS covers everything (where his rules need his go, his Brave check or his yes to his Advisor comes first, then it posts PASS); a guard that fails means the prompt',
  '442b9871-8e0d-40f4-941c-52d9214d20a9', 'mustafa-2026-11-26-merge-prompt-d.md',
  `the chat's offered option (its lines from the chat's notes file \`.checkpoints/review-tracks/chat-2026-11-26-option-d-offered.md\` at the same commit, joined into one line, list marks kept): "${offered(3, 14)}" and its question (${CTX_NOTE}): "${ctx(47, 56)}" He typed the one letter, choosing that offered text; the offered text is not his words.`,
  'as built on the branch (`.claude/hooks/routine-guard.mjs` and `CLAUDE.md`, Standing permission, way (1); read by docs-keeper at commit 6a630c25): in a day session, and never for a subagent, the guard skips the merge\'s permission prompt only when the expected head is the pull request\'s current full head, gate, smoke and toolchain are green on it, and a PASS comment of his Advisor is found (by `Mergincanli`, user type User, its first line exactly the pull request and its full head, never edited, posted after the gate run on that head); anything else, or a failed read, gives the normal prompt; no file list is read; his Advisor writes PASS only when nothing in advisor.md\'s "His go keeps" applies, or after his go for that head. The advisor agent and security-auditor each took a quick look at (d) and approved it (the chat\'s card, steps 15 and 16).');

const E5 = entry(
  '2026-11-27, 13:55:39 UTC: ADDITIONS-OK (a), all nine additions taken; SIZE-LINE (a), go on past the stop line, no split; USAGE: save and push after every kit edit, with the card exact about which edits are in and which are not; he switches the chat to Accept edits when told the edits start; FEWER-ASKS: an item for the next change-set\'s plan (the guard change-set), with the security-auditor, no work on it now; his rules unchanged; and his question on item 8',
  '5ab2d0b4-596a-45ba-a9ca-8c157a8028ad', 'mustafa-2026-11-27-additions-size-usage.md',
  `the chat's two questions (${CTX_NOTE}): "${ctx(60, 68)}"`,
  'the chat answered his question on item 8 in the chat (its card, step 29); the kit edits (the chat\'s card, "Applies", rows 1 to 11) followed.');

let doc = readFileSync(DOC, 'utf8');
for (const e of [E1, E2, E3, E4, E5]) if (doc.includes(e.first.slice(0, 60))) throw new Error('already added: ' + e.first.slice(0, 60));
function insertBefore(anchor, block) {
  const at = doc.indexOf(`\n${anchor}`);
  if (at < 0 || doc.indexOf(`\n${anchor}`, at + 1) >= 0) throw new Error('anchor not found once: ' + anchor);
  doc = doc.slice(0, at + 1) + block + '\n' + doc.slice(at + 1);
}
insertBefore('- **Taken by Mustafa, 2026-11-27, 12:37:33 UTC', E5.text);
insertBefore('- **Taken by Mustafa, 2026-11-26, 18:16:35 UTC', [E4, E3, E2, E1].map((e) => e.text).join('\n'));
writeFileSync(DOC, doc);
for (const e of [E5, E4, E3, E2, E1]) console.log(`${e.bytes} bytes, sha256 ${e.sha}`);
