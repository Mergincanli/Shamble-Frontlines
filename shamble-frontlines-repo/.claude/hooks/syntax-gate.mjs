#!/usr/bin/env node
// PostToolUse hook (Edit|Write): instant syntax gate for the file Claude just changed.
//   .js / .mjs / .cjs -> `node --check` in place (module type resolved by Node as usual)
//   .html             -> every inline <script> without src (classic -> .js, type="module" -> .mjs) -> `node --check`
// Exit 2 + stderr: Claude sees the error immediately and must fix it before moving on. Anything else: exit 0, silent.
// Cross-platform (Windows / Linux / macOS), no dependencies. The full four-check battery is tools/gate.mjs, not this.
import { readFileSync, writeFileSync, mkdtempSync, rmSync, existsSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, extname, basename } from 'node:path';
import { spawnSync } from 'node:child_process';

let file = null;
try { file = JSON.parse(readFileSync(0, 'utf8'))?.tool_input?.file_path ?? null; } catch { process.exit(0); }
if (!file || !existsSync(file)) process.exit(0);
const ext = extname(file).toLowerCase();
const nodeCheck = (path) => {
  const r = spawnSync(process.execPath, ['--check', path], { encoding: 'utf8' });
  return r.status === 0 ? null : (r.stderr || r.stdout || 'unknown syntax error').trim();
};
const failures = [];
if (ext === '.js' || ext === '.mjs' || ext === '.cjs') {
  const err = nodeCheck(file);
  if (err) failures.push(err);
} else if (ext === '.html' || ext === '.htm') {
  const html = readFileSync(file, 'utf8');
  const dir = mkdtempSync(join(tmpdir(), 'syntax-gate-'));
  try {
    let i = 0;
    for (const m of html.matchAll(/<script\b([^>]*)>([\s\S]*?)<\/script>/gi)) {
      const attrs = m[1], code = m[2];
      if (/\bsrc\s*=/.test(attrs) || !code.trim()) continue;
      const type = (attrs.match(/\btype\s*=\s*["']?([^"'\s>]+)/i) || [])[1];
      if (type && !/^(module|text\/javascript|application\/javascript)$/i.test(type)) continue;
      const line = html.slice(0, m.index).split('\n').length;
      const tmp = join(dir, `inline-${++i}.${type && /module/i.test(type) ? 'mjs' : 'js'}`);
      writeFileSync(tmp, code);
      const err = nodeCheck(tmp);
      if (err) failures.push(`inline <script> #${i} (starts at ${basename(file)}:${line})\n${err.split(tmp).join(`inline#${i}`)}`);
    }
  } finally { rmSync(dir, { recursive: true, force: true }); }
}
if (failures.length) {
  process.stderr.write(`SYNTAX GATE FAILED for ${file}\n${failures.join('\n\n')}\nFix this before doing anything else.\n`);
  process.exit(2);
}
process.exit(0);
