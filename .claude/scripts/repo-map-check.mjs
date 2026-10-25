#!/usr/bin/env node
// Checks docs/REPO_MAP.md against the repository: node .claude/scripts/repo-map-check.mjs (from the repository root).
// A map entry is the first `backticked` path of a table row ("| `path` | …"); a folder entry ends with "/".
// 1. Every entry exists.  2. Every tracked or new (not ignored) path is listed down to these depths:
//    top level · docs/*, tools/*, .claude/*, .github/* · .claude/{agents,skills,rules,scripts,hooks}/* ·
//    .github/workflows/*
//    (docs/adr/, docs/reference/, docs/reports/ and docs/research/ are listed as folders only; .checkpoints/ is
//    transient and never required). Exit 0 when the map is complete, 1 when not, 2 on a usage error. Node 22, no deps.
import { execFileSync } from 'node:child_process';
import { existsSync, readFileSync } from 'node:fs';

const MAP = 'docs/REPO_MAP.md';
if (!existsSync(MAP)) {
  process.stderr.write(`repo-map-check: ${MAP} not found (run from the repository root)\n`);
  process.exit(2);
}
const listed = new Set();
for (const line of readFileSync(MAP, 'utf8').split('\n')) {
  const m = /^\|\s*`([^`\s]+)`/.exec(line);
  if (m) listed.add(m[1]);
}
const files = execFileSync('git', ['ls-files', '--cached', '--others', '--exclude-standard'], { encoding: 'utf8' })
  .split('\n').filter((p) => p && existsSync(p) && !p.startsWith('.checkpoints/'));
const required = new Set();
const entry = (parts, n) => parts.slice(0, n).join('/') + (parts.length > n ? '/' : '');
for (const p of files) {
  const s = p.split('/');
  required.add(entry(s, 1));
  if (['docs', 'tools', '.claude', '.github'].includes(s[0]) && s.length >= 2) required.add(entry(s, 2));
  const deep = ['agents', 'skills', 'rules', 'scripts', 'hooks'];
  if (s[0] === '.claude' && deep.includes(s[1]) && s.length >= 3) required.add(entry(s, 3));
  if (s[0] === '.github' && s[1] === 'workflows' && s.length >= 3) required.add(entry(s, 3));
}
const missing = [...required].filter((p) => !listed.has(p)).sort();
const stale = [...listed].filter((p) => !existsSync(p.replace(/\/$/, ''))).sort();
for (const p of missing) process.stdout.write(`missing from the map: ${p}\n`);
for (const p of stale) process.stdout.write(`in the map but not in the repository: ${p}\n`);
if (missing.length || stale.length) process.exit(1);
process.stdout.write(`repo map OK: ${listed.size} entries cover ${required.size} required paths\n`);
