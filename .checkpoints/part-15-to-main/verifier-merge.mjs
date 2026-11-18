// Verifier: is the saved go-message byte for byte inside merge commit d7dad97's message? Prints offsets only.
import { readFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
const t = readFileSync('.checkpoints/part-15-to-main/mustafa-2026-11-17-2357-merge-56.md', 'utf8'); const w = t.slice(t.indexOf('\n---\n') + 5).replace(/\n$/, '');
const body = execFileSync('git', ['log', '-1', '--format=%B', 'd7dad97']).toString('utf8');
const raw = execFileSync('git', ['cat-file', 'commit', 'd7dad97']).toString('utf8');
const i = body.indexOf(w);
console.log('in %B:', i >= 0, 'at', i, 'of', Buffer.byteLength(body), 'bytes; count', body.split(w).length - 1);
const lines = body.split('\n'); console.log('first line of %B:', JSON.stringify(lines[0])); console.log('%B before the words (bytes):', i, JSON.stringify(body.slice(0, i).slice(0, 200)));
console.log('%B after the words:', JSON.stringify(body.slice(i + w.length)));
console.log('in raw commit object:', raw.includes(w));
