// Verifier: the archive's five follow-up items begin with the pieces of #56's STATUS row ("Open after it, ...") at the base.
import { readFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
const flat = (s) => s.replace(/\s+/g, ' ').trim();
const st = execFileSync('git', ['show', 'd7dad97:docs/STATUS.md']).toString('utf8');
const row = st.split('\n').find((l) => l.startsWith('| #56 |'));
const tail = row.slice(row.indexOf('Open after it, for "Follow-ups" once merged: ') + 'Open after it, for "Follow-ups" once merged: '.length, row.indexOf(' | `claude/wonderful'));
const pieces = tail.split('; ').reduce((a, p) => { // re-join pieces split inside a parenthesis
  if (a.length && (a[a.length - 1].split('(').length > a[a.length - 1].split(')').length)) a[a.length - 1] += '; ' + p; else a.push(p); return a; }, []);
console.log('pieces in the row:', pieces.length);
const ar = readFileSync('docs/STATUS_ARCHIVE.md', 'utf8');
const i = ar.indexOf('- **From change-set 0 (PR #56) and Part 15 (PR #57)**'); const block = ar.slice(i, ar.indexOf('\n## ', i));
const items = block.split('\n  - ').slice(1).map(flat);
pieces.forEach((p, k) => console.log(k + 1, 'item starts with the row piece:', items[k] && items[k].startsWith(flat(p)), '|', flat(p).slice(0, 60)));
// the archived row's first cell equals #56's STATUS row's work cell
const arRow = ar.split('\n').find((l) => l.startsWith('| **change-set 0 of the build plan**'));
const work = row.split(' | ')[1]; const arWork = arRow.split(' | ')[0].slice(2);
console.log('archived work cell == STATUS work cell (bold title aside):', arWork.replace('**change-set 0 of the build plan**', 'change-set 0 of the build plan') === work);
const done = row.split(' | ')[3]; const arDone = arRow.split(' | ')[2];
console.log('archived "done" cell starts with STATUS state cell minus "ready for review; ":', arDone.startsWith('done: ' + done.replace(/^ready for review; /, '')));
