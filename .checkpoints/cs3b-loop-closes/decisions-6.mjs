// (decisions-6: decisions-5 with his two messages of 2026-11-25 on top: his decision RELEASE-COUNT and his browser check.)
// The docs pass of change-set 3b: writes Mustafa's chat messages since his entry of 2026-11-24, 21:48:49 UTC into
// docs/DECISIONS.md, newest first. His words are never retyped: they come from the files chat-words.mjs saved from
// the chat's transcript, each with its byte count and sha256, and every quote of his inside a lead is checked
// against those saved words. Made from decisions-4.mjs as it stands at 6491b839 (the same helpers and entry form), with one more entry kind: a
// line of the chat's own that holds no word of his (`line`).
// The file is always rebuilt from BEFORE (main's copy), so a second run writes the same bytes. From the repository root:
//   node .checkpoints/cs3b-loop-closes/decisions-5.mjs [--check] [--out <file>]
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';

const DEC = 'docs/DECISIONS.md';
const at = process.argv.indexOf('--out');
const OUT = at === -1 ? DEC : process.argv[at + 1];
const NOTES = '.checkpoints/cs3b-loop-closes/';
const BEFORE = '28254a73ab226379373e380eeee64336e58ef916';
const V3 =
  'the builder chat he calls Builder V3, session_01WLuqnvpAh9XTitx8ehKJcQ, a Claude Code chat on the account that also holds his claude.ai project "Shamble Frontlines ALT"';
const sha = (s) => createHash('sha256').update(s).digest('hex');
const quote = (s) => s.split('\n').map((l) => (l === '' ? '>' : `> ${l}`)).join('\n');
const wrap = (text, rest = '  ') => {
  const out = [];
  let line = '';
  let empty = true;
  for (const w of text.split(' ')) {
    if (!empty && line.length + 1 + w.length > 120) {
      out.push(line);
      line = rest + w;
    } else {
      line += (empty ? '' : ' ') + w;
      empty = false;
    }
  }
  out.push(line);
  return out.join('\n');
};
function saved(file) {
  const note = readFileSync(NOTES + file, 'utf8');
  const cut = note.indexOf('\n---\n');
  if (cut < 0) throw new Error(`no --- in ${file}`);
  const head = note.slice(0, cut);
  const words = note.slice(cut + 5).replace(/\n$/, '');
  const count = head.match(/\((\d+) bytes, sha256 ([0-9a-f]{64})\)/);
  if (!count || Number(count[1]) !== Buffer.byteLength(words) || count[2] !== sha(words)) throw new Error(`${file}: the words differ from its own count or sha256`);
  const id = head.match(/\(uuid ([0-9a-f-]{36})\)/);
  const answers = head.match(/What it answers: ([\s\S]*)_\s*$/);
  if (!id || !answers) throw new Error(`${file}: no uuid or no "What it answers" in its header`);
  return { words, bytes: Buffer.byteLength(words), sha256: sha(words), id: id[1], answers: answers[1].trim() };
}
const said = (m, s) => {
  if (!m.words.includes(s)) throw new Error(`not in his saved words: ${s}`);
  return `"${s}"`;
};

// Newest first. Each lead is the header's text after "Taken by Mustafa, <when>: "; `records` holds what the
// repository's records show, marked as not his words.
const ENTRIES = [
  {
    when: '2026-11-25, 12:14:23 UTC',
    file: 'mustafa-2026-11-25-firefox-check.md',
    title:
      'his browser check of change-set 3b in Firefox: clean; the hidden-tab remark he made of Brave did not come again, in Firefox or in Brave',
    lead: (m) =>
      `In his words: ${said(m, 'Did some extra thorough spamming and testing, came in pretty clean.')} Of the hidden-tab step in Firefox: ${said(m, 'that it did work properly, unlike the brave one.')} And of Brave, tried again: ${said(m, "But when I tried remaking the brave one, it didn't happen so I think it was either a 1 time flaw from my side and not an issue")} The Copy report he pasted follows whole.`,
    records:
      'the report is read line by line in `docs/reports/2026-11-25-cs3b-browser-check.md`: Firefox 161 on Windows, backend WebGPU, commit 8f835d114, 4,786 ticks, messages refused 0, olderPosts 0, wrongKeys 0, presses 215 made and 215 echoed with 0 lost and 0 doubled, releases 0 doubled, errors 0. The chat had asked him RESUME-FLASH (leave as built, or make the boxes go dark the moment the tab hides); he named no letter, and the chat takes these words as: no change now. What the first picture after a pause shows stays a follow-up, 3B-RESUME-FLASH, for change-set 4',
  },
  {
    when: '2026-11-25, 11:38:37 UTC',
    file: 'mustafa-2026-11-25-brave-check.md',
    title:
      'RELEASE-COUNT: (a), accept as built, with two conditions; the order of change-set 3b\'s last steps; his browser check in Brave, with one remark on the hidden-tab step',
    lead: (m) =>
      `His decision: ${said(m, 'RELEASE-COUNT (how releases are counted for the presses mark PM-0002-12): (a), accept as built.')} And: ${said(m, 'No release counter is added in change-set 3c.')} His first condition: ${said(m, "the report keeps saying that presses were counted by the row's own method and releases by working them out, and it calls no part of that mark passed. The mark's numbers stay as the row has them.")} His second: ${said(m, "the two drafts are made to agree at their next revision (ADR 0002 part 5's sentence about press and release edge counters, and the method cell of the row PM-0002-12), with the reason written there. That revision comes to me before it is written. Put it on the follow-up list under a name.")} On the last steps: ${said(m, "the last commit, your advisor agent's approval of exactly that commit, GitHub's checks green on it. Then give me its full commit id in one line and push nothing after it.")} His remark on the hidden-tab step in Brave: ${said(m, 'it was actually still going until the moment I tabbed back.')}`,
    records:
      'he sent the five numbered points first at 11:07:13 UTC, without the check (the same text; his message 13b9e16f-570b-400f-b1d2-544d7ee0865e, 1,431 bytes, sha256 3e7f2bf0238516aa3fe385829569a9186872f7f743fa6ed3c367bebcb6a4f05e); the follow-up he asks for is named 3B-RELEASE-DRAFTS (STATUS and the archive); no draft is edited by change-set 3b. His Brave report is read line by line in `docs/reports/2026-11-25-cs3b-browser-check.md`: every number he was asked to check is 0. The chat\'s reading of his remark, from the code: the match stands still while the tab is hidden, the box shows the server\'s last echo, and the first tick after the return lets the key go, so the picture from before the pause can show for about a thirtieth of a second',
  },
  {
    when: '2026-11-24, 23:24:50 UTC',
    file: 'mustafa-2026-11-24T232450-after-the-compact.md',
    title:
      'the chat is compacted; the build plan\'s third sentence on command timing (its risk 8): (a), changed with the other two; build change-set 3b by the plan under the advisor agent\'s eight conditions, without waiting for him',
    lead: (m) =>
      `His answer on the third sentence: ${said(m, 'The third sentence in the build plan (its risk 8): (a). Change it with the other two in 3b\'s docs pass, and show me all three old and new sentences in your report.')} On the build: ${said(m, 'If the card, the repo and the summary disagree, or something is missing, say what and stop. Otherwise build change-set 3b by the plan under your advisor agent\'s eight conditions, without waiting for me.')} On the size and the browser tests: ${said(m, 'If the size passes 4,900 lines, the ladder applies as you described: tell me in three lines before a part moves. Time the browser tests before the first push, as you said.')} He dated the message 25 November; the chat's clock read 2026-11-24, 23:24:50 UTC.`,
    records:
      'the card, the repository and the summary agreed (the branch at 8e3c0a75, main 28254a73, the session record: model `claude-fable-5-2`, permission mode `auto`, context about 11 %); the three old and new sentences stand in change-set 3b\'s report; the size passed 4,900 lines and the ladder applied: he was told in three lines on 2026-11-24, about 23:56 UTC, before a part moved, and the link simulator and the first recording are change-set 3c\'s; the browser tests were timed on a clean copy before the push whose run counts as evidence (16 tests in 3.4 of 5 minutes, there and on GitHub)',
  },
  {
    when: '2026-11-24, 22:59:49 UTC',
    file: 'mustafa-2026-11-24T225949-where-the-decisions-are.md',
    title:
      'where his decisions of 21 and 22 November stand: on the netcode test\'s branch, not on main; the chat may read them there with git show, read only',
    lead: (m) =>
      `In his words: ${said(m, 'The decisions you could not find are recorded on the netcode test\'s branch, not on main: branch claude/netcode-proof-test-plan-hcky5g, file .checkpoints/netcode-proof-test/history.md, lines 7561 to 7607 (NPT-TRIM of 21 November and my answers to the re-plan of 22 November), and .checkpoints/netcode-proof-test/moved-block.md, the part "FOR CHANGE-SET 3 ITSELF".')} How to read them: ${said(m, 'Read them with git show, read only. That is not the spikes/ folder.')} And: ${said(m, 'My message with the three items is my word either way.')}`,
    records:
      'a pointer, not a copy: the chat read those lines on 2026-11-24, 23:00 UTC, at that branch\'s head e8e276f0bf8b201f6a4db3b59315ac6de4c84a04 (fetched by name; nothing is ever pushed there). They are not copied onto main here: his decisions of 21 and 22 November stay recorded where he recorded them, and reach main with that branch. What they say for change-set 3 is the same three items as his message of 22:44:50 UTC (the entry below)',
  },
  {
    when: '2026-11-24, 22:44:50 UTC',
    file: 'mustafa-2026-11-24T224450-merged-65-next-3b.md',
    title:
      '#65 (change-set 3a) is merged, by his main Advisor on his tap, with the expected head 1624f08; the next CI edit is an Opus 5.5 chat\'s; next is change-set 3b, with three items first: the decoder test at its row\'s count, command timing picked by Slice 3 and no longer by the proof test, and the link simulator built by the plan',
    lead: (m) =>
      `The merge, in his words: ${said(m, 'My main Advisor marked it ready and merged it on my tap of 22:40:00 UTC, with your commit 1624f084924976f4bde8556509c957e1f4634341 as the expected head.')} And: ${said(m, 'That tap was my go. I typed no go line in your chat. Record it by script from this message.')} On CI: ${said(m, 'Which model writes the next CI edit: (a). An Opus 5.5 chat writes the small CI pull request, as my rule of 12 November says. You write no CI file.')} The decoder test's bar: ${said(m, 'Its bar is its row in ADR 0001, exactly as written: 10 million random and mutated inputs per message type, 0 panics and 0 allocations above the size limits, and its planted bug (an unchecked index in one decoder) is found within the run.')} and ${said(m, 'Until then your report says the mark has started, never that it passed.')} On command timing: ${said(m, 'The proof test no longer picks it. Slice 3 picks it, and change-set 7 still builds both ways. My go: with 3b\'s docs pass, the build plan\'s two places that say the proof test chooses say this instead.')} On the link simulator: ${said(m, 'If the ladder moves the link simulator out of 3b, tell me before anyone builds it a second time.')}`,
    records:
      'main is 28254a73ab226379373e380eeee64336e58ef916 (2026-11-24, 22:40:39 UTC; its parents 1ec6f6c3 and 1624f084); gate, smoke and toolchain were green on it (read 22:45 UTC). The advisor agent read the three items against the plan on that main and approved building 3b under eight conditions; the chat answered in six lines. In change-set 3b the decoder test runs at the row\'s count for the five message types and the report says the mark has started; the build plan\'s sentences on command timing were changed with 3b\'s docs pass (three, by his answer of 23:24:50 UTC above); the size ladder moved the link simulator to change-set 3c, and he was told before anyone built it',
  },
];

const before = execFileSync('git', ['show', `${BEFORE}:${DEC}`], { encoding: 'utf8', maxBuffer: 64 * 1024 * 1024 });
const MARK = '## Decisions, newest first\n';
if (before.split(MARK).length !== 2) throw new Error('the heading "Decisions, newest first" must stand once');
const blocks = ENTRIES.map((entry) => {
  if (entry.line) return `${wrap(`- ${entry.line}`)}\n`;
  const m = saved(entry.file);
  if (entry.pointer) return `${wrap(`- ${entry.pointer(m)}`)}\n`;
  const header = `- **Taken by Mustafa, ${entry.when}: ${entry.title}** (${V3}; his message ${m.id}, whole; extracted by script from the chat's transcript). ${entry.lead(m)} What it answers, in the chat's words (not his): ${m.answers} Records, not his words: ${entry.records}. Copied by script (${m.bytes} bytes, sha256 ${m.sha256}):`;
  return `${wrap(header)}\n${quote(m.words)}\n`;
});
const after = before.replace(MARK, MARK + blocks.join(''));
if (process.argv.includes('--check')) {
  const now = readFileSync(DEC, 'utf8');
  console.log(now === after ? `${DEC} is what this script writes (${ENTRIES.length} entries on top of ${BEFORE.slice(0, 7)}'s)` : `${DEC} differs from what this script writes`);
  process.exit(now === after ? 0 : 1);
}
writeFileSync(OUT, after);
console.log(`${OUT}: ${ENTRIES.length} entries added on top of ${BEFORE.slice(0, 7)}'s, ${after.split('\n').length - before.split('\n').length} lines`);
