// verifier delta: CLAUDE.md's Never bullet vs Mustafa's words (whitespace folded), and the WT-NIGHT (a) texts.
// Usage: node vd-texts.mjs <a3 worktree>. Prints counts and short excerpts only.
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
const A3 = process.argv[2];
const n = (s) => s.replace(/\s+/g, ' ');
const rd = (p) => n(readFileSync(join(A3, p), 'utf8'));
const c = rd('CLAUDE.md'), m = rd('.checkpoints/kit-v3-3/mustafa-words.md');
const rule = m.slice(m.indexOf('never switch a session') + 6, m.indexOf('act on a day session.') + 21);
console.log('rule (his words without "never"), length', rule.length, 'starts', JSON.stringify(rule.slice(0, 30)));
console.log('CLAUDE.md "- " + rule count:', c.split('- ' + rule).length - 1);
const ex = "One exception: the night shift's handoff switch (NIGHT_SHIFT.md step 4), which the guard refuses unless the branch's rules, CI and gate files equal main's.";
console.log('exception clause count:', c.split(ex).length - 1);
const i = c.indexOf(rule), j = c.indexOf(ex);
console.log('rule before exception:', i >= 0 && j > i, '| text between:', JSON.stringify(c.slice(i + rule.length, j).slice(0, 50) + '...' + c.slice(i + rule.length, j).slice(-60)));
console.log('bullet ends after the clause:', JSON.stringify(c.slice(j + ex.length, j + ex.length + 3)));
const w = m.match(/WT-NIGHT: \(a\)\. (.+?) PC-SETUP/);
console.log('his WT-NIGHT (a) words:', w ? w[1] : '(not found)');
const ns = rd('.claude/NIGHT_SHIFT.md'), ck = rd('.claude/CHECKPOINTS.md');
const s4 = ns.slice(ns.indexOf('4. Handoff:'), ns.indexOf('5. ', ns.indexOf('4. Handoff:')));
console.log('NIGHT_SHIFT step 4 switch text:', JSON.stringify(s4.slice(s4.indexOf('Switch to that branch'), s4.indexOf('and resume from') + 15)));
console.log('NIGHT_SHIFT "Check out that branch" count:', ns.split('Check out that branch').length - 1);
const s8 = ck.slice(ck.indexOf('2. Get onto the branch'), ck.indexOf('3. Continue from'));
console.log('CHECKPOINTS s8 step 2:', JSON.stringify(s8));
