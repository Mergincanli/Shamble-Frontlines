// Routine guard rules (kit-v3-1, plan R1-R6; CLAUDE.md "Routines"). guard.mjs loads this module and prints its
// decision; routine-guard.test.mjs calls decide() directly with a fake io.
// Who is bound by what:
//   every session (day sessions have no SHAMBLE_ROLE): git and gh option forms (a long option must be spelled in full,
//     no dangerous option or bundled short flag, no option before the git subcommand) and the merge check
//     (expectedHeadSha, same repository, gate and smoke green on that SHA);
//   routines (SHAMBLE_ROLE = night-shift or pr-advisor, set only by the routine-only cloud environments): the role rules
//     below as well.
// decide(input, io) returns null (no decision: the normal permission flow goes on) or { deny, interrupt }. It never
// allows anything: this guard only takes away. guard.mjs turns an exception into a refusal for routines and into "no
// decision" for day sessions (R3c), so a guard bug never locks a day session out of fixing it.
// io: { env, projectDir, home, git(args, cwd) -> { code, out }, gh(args) -> { code, out }, exists(p), realpath(p),
//       readFile(p), gitLongOptions(sub) -> string[] }. realIo() builds the real one. Node 22, no dependencies.
import { spawnSync } from 'node:child_process';
import { existsSync, readFileSync, realpathSync } from 'node:fs';
import { basename, dirname, isAbsolute, join, relative, resolve } from 'node:path';
import { homedir } from 'node:os';

export const REPO = 'Mergincanli/Shamble-Frontlines';
export const ROLES = new Set(['night-shift', 'pr-advisor']);
const GATE_FILES = new Set(['tools/gate.mjs', 'tools/smoke.mjs', 'tools/battery.js', 'tools/gate-globals.json',
  'tools/package.json', 'tools/package-lock.json']);
const NIGHT_NPM = 'npm ci --prefix tools --ignore-scripts --no-audit --no-fund';
// Upstream sources routines may fetch from open-publish hosts (plan C8): the repositories and crates brief §4 names.
// Adding one is a daytime kit change.
const GITHUB_REPOS = new Set([REPO.toLowerCase(), 'mrdoob/three.js', 'dimforge/rapier', 'rustwasm/wasm-bindgen',
  'wasm-bindgen/wasm-bindgen', 'bevyengine/bevy', 'ralith/hecs', 'w3c/webtransport', 'gpuweb/gpuweb']);
const CRATES = new Set(['bevy_ecs', 'hecs', 'rapier3d', 'rapier2d', 'parry3d', 'wasm-bindgen', 'js-sys', 'web-sys',
  'serde', 'tokio', 'quinn', 'wtransport', 'h3', 'tokio-tungstenite', 'glam']);

// ---------- paths ----------
// A path routines may not write: the rules, CI, the gate's own files, agent instructions anywhere, git internals.
export function protectedRel(rel) {
  const p = rel.split('/').filter((s) => s && s !== '.');
  const base = p[p.length - 1] || '';
  if (base === 'CLAUDE.md' || base === '.npmrc' || base === '.mcp.json') return true;
  if (p[0] === '.github' || p[0] === '.git') return true;
  if (p[0] === '.claude') {
    if (p[1] === 'agent-memory-local') return false;
    if (p[1] === 'worktrees') return p.length > 3 ? protectedRel(p.slice(3).join('/')) : false;
    return true;
  }
  return GATE_FILES.has(p.join('/'));
}
// The real location of a path that may not exist yet: the nearest existing ancestor through realpath, plus the rest.
function realOf(abs, io) {
  let head = abs, tail = [];
  while (!io.exists(head) && dirname(head) !== head) { tail.unshift(basename(head)); head = dirname(head); }
  let real = head;
  try { real = io.realpath(head); } catch { /* keep head */ }
  return join(real, ...tail);
}
// The path relative to the project, or null when it is outside (after following symlinks).
function projectRel(p, cwd, io) {
  const abs = realOf(isAbsolute(p) ? p : resolve(cwd, p), io);
  const root = realOf(io.projectDir, io);
  const rel = relative(root, abs);
  return rel === '' ? '.' : (rel.startsWith('..') || isAbsolute(rel)) ? null : rel.split('\\').join('/');
}
function insideDir(p, dir, cwd, io) {
  if (!dir) return false;
  const rel = relative(realOf(dir, io), realOf(isAbsolute(p) ? p : resolve(cwd, p), io));
  return rel === '' || (!rel.startsWith('..') && !isAbsolute(rel));
}

// ---------- shell ----------
// Split a Bash command line into simple commands: { argv, redirects }. Quotes and backslashes are honoured; &&, ||,
// ;, |, |&, & and newlines separate commands; $(...), `...`, <(...) and >(...) are parsed as commands of their own
// (subs: true marks a line that had them). A here-document's body is skipped.
export function parseShell(src) {
  const cmds = [];
  let subs = false, unterminated = false;
  const walk = (s) => {
    let argv = [], redirects = [], reads = [], word = null, i = 0, pendingRedirect = false, wordExpand = false, cmdExpand = false;
    const heredocs = [];
    const endWord = () => {
      if (word === null) { wordExpand = false; return; }
      if (wordExpand) cmdExpand = true; // an unquoted glob/~ in this word: the shell would expand it before the program runs (J1/J2/J3)
      wordExpand = false;
      if (pendingRedirect === 'read') { reads.push(word); pendingRedirect = false; } // a < reads this file (F5: still credential-checked)
      else if (pendingRedirect) { redirects.push(word); pendingRedirect = false; } else argv.push(word);
      word = null;
    };
    const endCmd = () => { endWord(); if (argv.length || redirects.length || reads.length) cmds.push({ argv, redirects, reads, expand: cmdExpand }); argv = []; redirects = []; reads = []; cmdExpand = false; };
    const sub = (open, close) => { // returns index after the matching close
      let depth = 1, j = i + open.length, q = null;
      for (; j < s.length; j++) {
        const c = s[j];
        if (q) { if (c === '\\' && q === '"') j++; else if (c === q) q = null; continue; }
        if (c === "'" || c === '"') { q = c; continue; }
        if (close === ')' && c === '(') { depth++; continue; }
        if (c === close) { if (--depth === 0) break; }
      }
      subs = true;
      walk(s.slice(i + open.length, j));
      return j + 1;
    };
    while (i < s.length) {
      const c = s[i];
      if (c === '\n') {
        endCmd();
        i++;
        while (heredocs.length) { // skip each here-document body up to its delimiter line
          const { delim, strip } = heredocs.shift();
          const lines = s.slice(i).split('\n');
          let k = 0;
          for (; k < lines.length; k++) if ((strip ? lines[k].replace(/^\t+/, '') : lines[k]) === delim) break; // only <<- strips tabs (F2)
          if (k === lines.length) unterminated = true; // the delimiter never came (F2): the body swallows the rest
          i += lines.slice(0, Math.min(k + 1, lines.length)).reduce((n, l) => n + l.length + 1, 0);
        }
        continue;
      }
      if (c === ' ' || c === '\t') { endWord(); i++; continue; }
      if (c === '\\') { word = (word ?? '') + (s[i + 1] ?? ''); i += 2; continue; }
      if (c === "'") { const j = s.indexOf("'", i + 1); word = (word ?? '') + s.slice(i + 1, j < 0 ? s.length : j); i = j < 0 ? s.length : j + 1; continue; }
      if (c === '"') {
        let j = i + 1, w = '';
        for (; j < s.length && s[j] !== '"'; j++) {
          if (s[j] === '\\' && j + 1 < s.length) { w += s[++j]; continue; }
          if (s.startsWith('$(', j)) { const save = i; i = j; j = sub('$(', ')') - 1; i = save; w += '$()'; continue; }
          if (s[j] === '`') { const save = i; i = j; j = sub('`', '`') - 1; i = save; w += '``'; continue; }
          w += s[j];
        }
        word = (word ?? '') + w; i = j + 1; continue;
      }
      if (s.startsWith('$(', i) && !s.startsWith('$((', i)) { i = sub('$(', ')'); word = (word ?? '') + '$()'; continue; }
      if (c === '`') { i = sub('`', '`'); word = (word ?? '') + '``'; continue; }
      if ((c === '<' || c === '>') && s[i + 1] === '(') { i = sub(c + '(', ')'); word = (word ?? '') + '<()'; continue; }
      if (s.startsWith('<<', i) && !s.startsWith('<<<', i)) {
        endWord();
        const dash = s[i + 2] === '-';
        i += dash ? 3 : 2;
        while (s[i] === ' ') i++;
        let d = '';
        while (i < s.length && !/[\s;&|<>]/.test(s[i])) { if (s[i] !== "'" && s[i] !== '"' && s[i] !== '\\') d += s[i]; i++; }
        heredocs.push({ delim: d, strip: dash });
        continue;
      }
      if (c === '&' && s[i + 1] === '&') { endCmd(); i += 2; continue; }
      if (c === '|' && s[i + 1] === '|') { endCmd(); i += 2; continue; }
      if (c === '|') { endCmd(); i += s[i + 1] === '&' ? 2 : 1; continue; }
      if (c === ';' || c === '(' || c === ')') { endCmd(); i++; continue; }
      if (c === '&' && s[i + 1] === '>') { endWord(); pendingRedirect = true; i += s[i + 2] === '>' ? 3 : 2; continue; }
      if (c === '&') { endCmd(); i++; continue; }
      if (c === '>' || (c === '<' && s[i + 1] !== '<')) {
        // a digit right before (2>, 1>>) belongs to the operator, not to a word
        if (word !== null && /^\d+$/.test(word)) word = null; else endWord();
        if (c === '>') {
          i += s[i + 1] === '>' || s[i + 1] === '|' ? 2 : 1;
          if (s[i] === '&') { i++; while (/\d|-/.test(s[i] ?? '')) i++; continue; } // >&2 duplicates a descriptor
          pendingRedirect = true;
        } else { i++; pendingRedirect = 'read'; }
        continue;
      }
      if (c === '*' || c === '?' || (c === '~' && word === null)) wordExpand = true; // unquoted; the shell expands it
      word = (word ?? '') + c; i++;
    }
    endCmd();
  };
  walk(src);
  return { cmds, subs, unterminated };
}
// Leading VAR=value words, wrappers that run the rest (timeout, nice, env, xvfb-run, ...): stripped before matching.
// Shell keywords in front of a command ({ ... }, if/then/do ..., !) are stripped too.
const WRAPPERS = new Set(['time', 'nohup', 'command', 'builtin', 'noglob', 'exec', '!', '{', '}', 'if', 'then', 'else',
  'elif', 'fi', 'do', 'done', 'while', 'until']);
export function unwrap(argv) {
  let a = argv.slice();
  for (let guard = 0; guard < 20 && a.length; guard++) {
    if (/^[A-Za-z_][A-Za-z0-9_]*=/.test(a[0])) { a = a.slice(1); continue; }
    const prog = basename(a[0]);
    if (WRAPPERS.has(a[0]) || WRAPPERS.has(prog)) { a = a.slice(1); continue; }
    if (prog === 'env') { a = a.slice(1); while (a.length && (a[0].startsWith('-') || /^[A-Za-z_][A-Za-z0-9_]*=/.test(a[0]))) a = a.slice(1); continue; }
    if (prog === 'timeout') { // timeout [-s SIG | -k DUR | --opt[=v]]... DURATION command
      a = a.slice(1);
      while (a.length && a[0].startsWith('-')) a = a.slice(/^(-s|-k|--signal|--kill-after)$/.test(a[0]) ? 2 : 1);
      a = a.slice(1);
      continue;
    }
    if (prog === 'nice') { a = a.slice(1); if (a[0] === '-n') a = a.slice(2); else if (/^-n?\d+$|^--adjustment/.test(a[0] ?? '')) a = a.slice(1); continue; }
    if (prog === 'stdbuf') { a = a.slice(1); while (a.length && a[0].startsWith('-')) a = a.slice(1); continue; }
    if (prog === 'xvfb-run') { a = a.slice(1); while (a.length && a[0].startsWith('-')) a = a.slice(/^-(n|s|e|f|p|w)$|^--(server-num|server-args|error-file|auth-file|xauth-protocol|wait)$/.test(a[0]) ? 2 : 1); continue; }
    if (prog === 'xargs' && a.length > 1 && !a[1].startsWith('-')) { a = a.slice(1); continue; }
    break;
  }
  return a;
}

// ---------- git ----------
const GIT_LONG_DANGER = {
  push: ['--delete', '--force', '--force-with-lease', '--force-if-includes', '--mirror', '--prune', '--receive-pack',
    '--exec', '--all'],
  branch: ['--delete', '--force', '--move', '--copy'],
  commit: ['--amend'],
  reset: ['--hard', '--soft', '--mixed', '--keep', '--merge'],
  switch: ['--force', '--discard-changes', '--force-create'],
  fetch: ['--upload-pack'],
  pull: ['--upload-pack'],
  clone: ['--upload-pack', '--config', '--template'],
  'ls-remote': ['--upload-pack', '--exec'],
  grep: ['--open-files-in-pager'],
  archive: ['--remote', '--exec'],
};
const GIT_SHORT_DANGER = { push: 'fd', branch: 'dDfmMcC', switch: 'fC', 'ls-remote': 'u', grep: 'O', clone: 'cu' };
// Short options whose value is the next word (so a value starting with '-' is not read as flags).
const GIT_SHORT_VALUE = { commit: 'mFCct', grep: 'efABCm', log: 'n', branch: '', 'ls-remote': '', push: 'o' };
// Every session: refused outright, with what to do instead.
const GIT_NEVER = {
  checkout: 'use git switch, or git show <ref>:<path>',
  restore: 'use git show <ref>:<path>, or git reset -q HEAD -- <path> to unstage',
  clean: 'remove only the files you made, by name',
  rebase: 'history is never rewritten here',
  'filter-branch': 'history is never rewritten here',
  'filter-repo': 'history is never rewritten here',
  replace: 'history is never rewritten here',
  'update-ref': 'refs move only through commit, switch and push',
};
// The routines' git: anything else is refused for them.
const GIT_ROUTINE = new Set(['status', 'diff', 'log', 'show', 'fetch', 'branch', 'switch', 'add', 'commit', 'push',
  'worktree', 'merge-base', 'rev-parse', 'ls-tree', 'cat-file', 'for-each-ref', 'remote', 'ls-remote', 'rm', 'mv',
  'reset', 'config', 'ls-files', 'blame', 'shortlog', 'describe', 'grep', 'version', 'help', 'symbolic-ref']);
const GIT_CONFIG_READ = new Set(['user.name', 'user.email']);
const GIT_READ = new Set(['status', 'diff', 'log', 'show', 'fetch', 'branch', 'switch', 'merge-base', 'rev-parse',
  'ls-tree', 'cat-file', 'for-each-ref', 'remote', 'ls-remote', 'ls-files', 'blame', 'shortlog', 'describe', 'grep',
  'version', 'help', 'symbolic-ref']);

function gitArgsCheck(sub, args, io) {
  const danger = GIT_LONG_DANGER[sub] || [];
  // git's completion list leaves some options out (git grep --open-files-in-pager), so the dangerous ones count too
  const known = [...new Set([...io.gitLongOptions(sub).map((o) => o.replace(/=$/, '')), ...danger])];
  const shortDanger = GIT_SHORT_DANGER[sub] || '';
  const shortValue = GIT_SHORT_VALUE[sub] || '';
  const rest = [];
  for (let i = 0; i < args.length; i++) {
    const t = args[i];
    if (t === '--') { rest.push(...args.slice(i)); break; }
    if (t.startsWith('--') && t.length > 2) {
      const name = t.split('=')[0];
      const exact = known.includes(name) || (name.startsWith('--no-') && known.includes('--' + name.slice(5)));
      if (!exact && known.some((k) => k.startsWith(name))) return `git ${sub} ${name}: spell long options in full (git accepts abbreviations, the rules don't)`;
      if (danger.includes(name)) return `git ${sub} ${name} is not allowed here`;
      continue;
    }
    if (t.startsWith('-') && t.length > 1) {
      for (let k = 1; k < t.length; k++) {
        if (shortDanger.includes(t[k])) return `git ${sub} -${t[k]} is not allowed here (in ${t})`;
        if (shortValue.includes(t[k])) { if (k === t.length - 1) i++; break; }
      }
      continue;
    }
    rest.push(t);
  }
  return rest; // the words that aren't options (and everything after --)
}

// ---------- the decision ----------
const deny = (reason, interrupt = false) => ({ deny: reason, interrupt });

export async function decide(input, io) {
  const event = input.hook_event_name;
  const rawRole = io.env.SHAMBLE_ROLE || '';
  const role = ROLES.has(rawRole) ? rawRole : rawRole ? 'unknown' : '';
  if (event === 'ConfigChange') {
    return role && ['project_settings', 'local_settings', 'skills', 'user_settings'].includes(input.source)
      ? deny(`routine (${rawRole}): settings and skills don't change mid-run`) : null;
  }
  if (event === 'PermissionRequest') {
    return role ? deny(`routine (${rawRole}): this would have waited for a human. Refused; record it under ` +
      '"Decisions needed" and stop.', true) : null;
  }
  if (event !== 'PreToolUse') return null;
  if (role === 'unknown') return deny(`SHAMBLE_ROLE="${rawRole}" is not night-shift or pr-advisor: refused`, true);
  const tool = input.tool_name || '';
  const ti = input.tool_input || {};
  const cwd = input.cwd || io.projectDir;
  if (tool === 'Bash') return bash(String(ti.command ?? ''), cwd, role, io, input);
  if (tool.startsWith('mcp__')) return mcp(tool, ti, role, io);
  if (!role) return null;
  // ---- routines only from here ----
  if (['AskUserQuestion', 'EnterPlanMode', 'ExitPlanMode'].includes(tool)) {
    return deny(`routine (${role}): nobody is watching. Write the question under "Decisions needed" instead.`);
  }
  if (['Edit', 'Write', 'NotebookEdit', 'MultiEdit'].includes(tool)) {
    return writePath(ti.file_path ?? ti.notebook_path ?? '', cwd, role, io, input);
  }
  if (tool === 'Read') {
    const raw = String(ti.file_path ?? '');
    const p = realOf(isAbsolute(raw) ? raw : resolve(cwd, raw), io); // follow symlinks before matching (F5)
    if (/^\/proc(\/|$)/.test(p) || /(^|\/)\.git\/config$/.test(p) || /(^|\/)\.(gitconfig|git-credentials|netrc|npmrc)$/.test(p) ||
      p.includes('/.config/gh/') || raw.includes('$')) return deny(`routine (${role}): ${raw} may hold credentials`);
    return null;
  }
  if (tool === 'WebFetch') return webFetch(String(ti.url ?? ''), role, io);
  return null;
}

function writePath(p, cwd, role, io, input) {
  if (!p) return deny(`routine (${role}): a write needs a path`);
  const rel = projectRel(p, cwd, io); // follows symlinks, so a link into the project resolves here (F4)
  if (rel !== null) {
    if (protectedRel(rel)) return deny(`routine (${role}): ${rel} is a rules, CI or gate file; those change only in a daytime kit change-set`);
    if (role === 'pr-advisor' && !/^(\.checkpoints|\.gate|\.claude\/agent-memory-local)(\/|$)/.test(rel)) {
      return deny('routine (pr-advisor): the PR Advisor writes only its notes (.checkpoints/, .gate/)');
    }
    return null;
  }
  if (insideDir(p, input.scratchpad_dir, cwd, io)) return null;
  return deny(`routine (${role}): ${p} is outside the project`);
}

function webFetch(url, role, io) {
  let u;
  try { u = new URL(url); } catch { return deny(`routine (${role}): not a URL: ${url}`); }
  const host = u.hostname.toLowerCase();
  let allowed = [];
  try {
    const s = JSON.parse(io.readFile(join(io.projectDir, '.claude/settings.json')));
    allowed = (s.permissions?.allow ?? []).map((r) => /^WebFetch\(domain:(.+)\)$/.exec(r)?.[1]).filter(Boolean);
  } catch { return deny(`routine (${role}): can't read the allow list`); }
  const ok = allowed.some((d) => d.startsWith('*.') ? host.endsWith(d.slice(1)) : host === d);
  if (!ok) return deny(`routine (${role}): ${host} is not on the documentation allow list; note the source as not read`);
  const seg = u.pathname.split('/').filter(Boolean);
  if (host === 'github.com' || host === 'raw.githubusercontent.com') {
    if (!GITHUB_REPOS.has(`${seg[0] ?? ''}/${seg[1] ?? ''}`.toLowerCase())) return deny(`routine (${role}): github.com/${seg.slice(0, 2).join('/')} is not on the named list (a daytime kit change adds it)`);
  }
  if (host === 'docs.rs' || host === 'crates.io') {
    const crate = host === 'docs.rs' ? seg[0] : seg[0] === 'crates' ? seg[1] : seg[0] === 'api' ? seg[3] : undefined;
    if (!crate || !CRATES.has(crate.toLowerCase())) return deny(`routine (${role}): crate "${crate ?? ''}" is not on the named list (a daytime kit change adds it)`);
  }
  return null;
}

// ---------- Bash ----------
// Routines run only the programs named here (deny-by-default: advisor verdict-3; security review F1/F3/F6). git, gh,
// node, python, python3 and npm have their own handlers below; a program that is on no list is refused. Day sessions
// (no role) are unchanged: only git and gh are checked.
const NET = new Set(['curl', 'wget', 'nc', 'ncat', 'netcat', 'socat', 'ssh', 'scp', 'sftp', 'rsync', 'telnet', 'ftp']);
const INSTALLERS = new Set(['npx', 'yarn', 'pnpm', 'pip', 'pip3', 'pipx', 'apt', 'apt-get', 'brew', 'gem', 'cargo',
  'go', 'bun', 'deno']);
const SHELLS = new Set(['sh', 'bash', 'zsh', 'dash', 'ksh', 'fish', 'pwsh']);
const DENIED_SCRIPTS = new Set(['tools/preview.js', 'tools/pageshot.js']);
const CODE_RUNNERS = new Set(['eval', 'source', '.', 'trap', 'alias', 'command', 'builtin', 'exec', 'env', 'printenv',
  'export', 'set', 'declare', 'hash', 'enable', 'shopt', 'xargs', 'time']);
// Only these may be set on a routine's command line (prefix, or via env); others can change what an allowed program
// runs (PATH, NODE_OPTIONS, BASH_ENV, LD_*, GIT_*, PYTHON*, npm_config_*). Verdict-3 condition 4.
const ROLE_ENV_OK = new Set(['TZ', 'LC_ALL', 'LANG', 'LANGUAGE', 'NO_COLOR']);
// Read-only and inspection programs routines may run. The set is kept small and obviously safe: tools with a rich
// write/exec option surface (sort, xxd, tree, fd, jq, awk, sed) are left off on purpose, so there is no bundled-option
// corner to get wrong (security review, second pass). `date` and `find` are the only ones with a write/exec option,
// each a small closed set refused by readToolBadOpt. Every path operand is credential- and scope-checked.
const ROLE_READ = new Set(['date', 'ls', 'cat', 'head', 'tail', 'wc', 'grep', 'egrep', 'fgrep', 'pwd', 'echo',
  'printf', 'test', '[', 'true', 'false', 'basename', 'dirname', 'realpath', 'readlink', 'cut', 'tr', 'comm', 'diff',
  'cmp', 'nl', 'rev', 'uname', 'hostname', 'which', 'sha256sum', 'find']);
const GREPS = new Set(['grep', 'egrep', 'fgrep']); // the first bare operand is a pattern, not a file (unless -e/-f given)
// File tools routines may run; every path they write is checked (verdict-3 condition 3). Recursive cp/mv/rm is refused.
const ROLE_WRITE = new Set(['mkdir', 'rm', 'rmdir', 'cp', 'mv', 'touch', 'tee']);
// A directory whose removal or overwrite would reach a protected file (its ancestor or the project root).
function coversProtected(rel) {
  if (rel === '.' || rel === '') return true;
  return ['.github', '.git', '.claude', 'tools'].some((d) => d === rel || d.startsWith(rel + '/'));
}
// A GNU long option that could be an abbreviation of `full` (any non-empty prefix, I1/I3). Used only where
// over-detection is the safe direction (recognising a grep pattern-supplying option), so even `--r` counts.
function abbrevOf(tok, full) {
  const m = /^--([a-z][a-z-]*)(=|$)/.exec(tok);
  return !!m && full.startsWith(m[1]);
}

// For a routine, strip the leading wrappers of a command (like unwrap) but validate the environment it sets: only
// ROLE_ENV_OK variables may be set (prefix or env), and command/exec/time/env-with-options are refused (verdict-3
// conditions 4, and 1 for the re-exec wrappers). Returns { deny } or { argv: the real command }.
function unwrapRole(argv, role) {
  let a = argv.slice();
  const okList = [...ROLE_ENV_OK].join(', ');
  for (let g = 0; g < 30 && a.length; g++) {
    const t = a[0], b = basename(t);
    if (/^[A-Za-z_][A-Za-z0-9_]*=/.test(t)) {
      const name = t.slice(0, t.indexOf('='));
      if (!ROLE_ENV_OK.has(name)) return { deny: `routine (${role}): only ${okList} may be set on a command; ${name}= can change what an allowed program runs` };
      a = a.slice(1); continue;
    }
    if (b === 'env') {
      a = a.slice(1);
      while (a.length) {
        if (a[0].startsWith('-')) return { deny: `routine (${role}): env options can change what runs; set ${okList} as a plain prefix instead` };
        if (/^[A-Za-z_][A-Za-z0-9_]*=/.test(a[0])) {
          const name = a[0].slice(0, a[0].indexOf('='));
          if (!ROLE_ENV_OK.has(name)) return { deny: `routine (${role}): env ${name}=... can change what runs; only ${okList} are allowed` };
          a = a.slice(1); continue;
        }
        break;
      }
      if (!a.length) return { deny: `routine (${role}): bare env prints the environment; it is not allowed` };
      continue;
    }
    if (b === 'command' || b === 'builtin' || b === 'exec' || b === 'time' || b === 'nohup') return { deny: `routine (${role}): ${b} can hide or re-exec a command; run the program directly` };
    if (WRAPPERS.has(t) || WRAPPERS.has(b)) { a = a.slice(1); continue; }
    if (b === 'timeout') { a = a.slice(1); while (a.length && a[0].startsWith('-')) a = a.slice(/^(-s|-k|--signal|--kill-after)$/.test(a[0]) ? 2 : 1); a = a.slice(1); continue; }
    if (b === 'nice') { a = a.slice(1); if (a[0] === '-n') a = a.slice(2); else if (/^-n?\d+$|^--adjustment/.test(a[0] ?? '')) a = a.slice(1); continue; }
    if (b === 'stdbuf') { a = a.slice(1); while (a.length && a[0].startsWith('-')) a = a.slice(1); continue; }
    if (b === 'xvfb-run') { a = a.slice(1); while (a.length && a[0].startsWith('-')) a = a.slice(/^-(n|s|e|f|p|w)$|^--(server-num|server-args|error-file|auth-file|xauth-protocol|wait)$/.test(a[0]) ? 2 : 1); continue; }
    break;
  }
  return { argv: a };
}
// A file a routine reads (an operand, an option's file value, or a < redirect): refuse a variable in a path, a glob
// that could reach outside the project, a credential path, and any read that resolves outside the project, the
// scratchpad or the pre-installed browser directory (verdict-3 condition 5; F5; security review second pass G2). The
// path is normalised (resolved against the command's folder and realpath'd) before every match, so `..` and symlinks
// can't dodge it.
function readOperandDeny(p, vcwd, role, io, scratch, opts = {}) {
  const { credOnly = false } = opts;
  if (p === '' || p === '-') return null;
  const looksPath = p.includes('/') || /^~/.test(p);
  const exRaw = p.replace(/^~(?=\/|$)/, io.home);
  const abs = realOf(isAbsolute(exRaw) ? exRaw : resolve(vcwd, exRaw), io);
  // credential paths are refused wherever they resolve, even a bare filename or a grep pattern (H2, I3): .git
  // internals, ssh, gh and the dot-files. This check runs even for the grep pattern (credOnly).
  const cred = /^\/proc(\/|$)/.test(abs) || /(^|\/)\.git(\/|$)/.test(abs) || /(^|\/)\.(gitconfig|git-credentials|netrc|npmrc)$/.test(abs) || abs.includes('/.config/gh/') || /(^|\/)\.ssh(\/|$)/.test(abs);
  if (cred) return deny(`routine (${role}): ${p} may hold credentials`);
  if (credOnly) return null; // a grep pattern: credential-checked above, but not a path to scope-check (unquoted glob patterns are caught at the command level)
  if (/[$`]/.test(p)) return deny(`routine (${role}): no variable or substitution in a file path (${p})`);
  if (/[[{]/.test(p)) return deny(`routine (${role}): a brace or bracket expansion in a file path isn't allowed; name the file (${p})`); // the shell would expand it past the guard (J3); * and ? are caught at the command level
  if (!looksPath) return null; // a bare word that isn't a path (a flag value)
  const inProject = projectRel(p, vcwd, io) !== null;
  const inScratch = scratch && insideDir(p, scratch, vcwd, io);
  const inBrowsers = abs === '/opt/pw-browsers' || abs.startsWith('/opt/pw-browsers/') || abs === '/dev/null';
  if (!inProject && !inScratch && !inBrowsers) return deny(`routine (${role}): reading outside the project (${p}); only the project, the scratchpad and /opt/pw-browsers`);
  return null;
}
// A path a routine writes (redirect, or a file tool's target): the scratchpad, or inside the project and not protected
// (verdict-3 condition 3, F4). Protected and project-scope decide before the scratchpad; a symlink is followed first.
function writeTarget(what, p, vcwd, role, io, input) {
  if (/[$`]/.test(p)) return deny(`routine (${role}): no variable or substitution in a write path (${p})`);
  if (/[*?[{]/.test(p)) return deny(`routine (${role}): a glob or brace expansion in a write path isn't allowed; name the file (${p})`); // the shell would expand it past the guard (I1/J3)
  if (p === '/dev/null' || /^\/dev\/(stdout|stderr|fd\/\d+)$/.test(p)) return null;
  const rel = projectRel(p, vcwd, io);
  if (rel !== null) {
    if (protectedRel(rel)) return deny(`routine (${role}): ${what} on ${rel} (a rules, CI or gate file) is not allowed`);
    if (coversProtected(rel)) return deny(`routine (${role}): ${what} on ${rel || '.'} would reach a rules, CI or gate file (G3)`);
    if (role === 'pr-advisor' && !/^(\.checkpoints|\.gate)(\/|$)/.test(rel)) return deny(`routine (pr-advisor): ${what} on ${rel}; the PR Advisor writes only its notes (.checkpoints/, .gate/)`);
    return null;
  }
  if (insideDir(p, input.scratchpad_dir, vcwd, io)) return null;
  return deny(`routine (${role}): ${what} must write inside the project or the scratchpad (${p})`);
}
// The non-option operands of a command (skipping the value of a value-taking option named by valRe).
function operands(args, valRe) {
  const out = [];
  for (let i = 0; i < args.length; i++) {
    const a = args[i];
    if (a === '--') { for (const x of args.slice(i + 1)) out.push(x); break; }
    if (a.startsWith('-') && a !== '-') { if (valRe && valRe.test(a)) i++; continue; }
    out.push(a);
  }
  return out;
}
// cp/mv: which operands are written (dest, and for mv the sources too) and which are only read (cp sources).
function cpMvTargets(args) {
  const bare = [];
  let tdir = null;
  for (let i = 0; i < args.length; i++) {
    const a = args[i];
    if (a === '--') { for (const x of args.slice(i + 1)) bare.push(x); break; }
    if (a === '-t' || a === '--target-directory') { tdir = args[++i]; continue; }
    const m = /^(?:--target-directory=|-t)(.+)$/.exec(a);
    if (m) { tdir = m[1]; continue; }
    if (a.startsWith('-') && a !== '-') { if (/^(-S|--suffix)$/.test(a)) i++; continue; }
    bare.push(a);
  }
  if (tdir !== null) return { dests: [tdir], sources: bare };
  if (!bare.length) return { dests: [], sources: [] };
  return { dests: [bare[bare.length - 1]], sources: bare.slice(0, -1) };
}
// The only two read tools with a write/exec option, each a small closed set (verdict-3 condition 1). Everything else
// with a rich option surface was left off ROLE_READ, so there is nothing else to enumerate.
function readToolBadOpt(prog, args) {
  const has = (re) => args.some((a) => re.test(a));
  if (prog === 'date' && has(/^(-s|--set(=|$))/)) return '-s sets the clock';
  if (prog === 'find' && has(/^-(exec|execdir|ok|okdir|delete|fprint|fprintf|fprint0|fls)$|^--(exec|delete)(=|$)/)) return '-exec/-delete/-fprint run or write';
  return null;
}
function npmCheck(argv, vcwd, role, io) {
  if (role === 'night-shift' && argv.join(' ') === NIGHT_NPM) {
    if (projectRel(vcwd, vcwd, io) !== '.') return deny('routine (night-shift): run the toolkit install from the project folder, not after a cd');
    for (const d of [io.projectDir, join(io.projectDir, 'tools'), io.home]) {
      if (io.exists(join(d, '.npmrc'))) return deny(`routine (night-shift): ${join(d, '.npmrc')} exists; npm config can change what installs`);
    }
    const same = io.git(['diff', '--quiet', 'origin/main', '--', 'tools/package.json', 'tools/package-lock.json'], io.projectDir);
    return same.code === 0 ? null : deny("routine (night-shift): the toolkit's manifest differs from main's; install only main's lockfile");
  }
  return deny(`routine (${role}): npm only as "${NIGHT_NPM}" (night shift; the PR Advisor installs nothing)`);
}

async function bash(command, cwd, role, io, input) {
  const { cmds, subs, unterminated } = parseShell(command);
  if (role && subs) return deny(`routine (${role}): no command substitution ($(...), backticks); run the commands one by one`);
  if (role && unterminated) return deny(`routine (${role}): an unterminated here-document; the command can't be read safely`);
  let vcwd = cwd;
  for (const c of cmds) {
    if (role && c.expand) return deny(`routine (${role}): an unquoted glob (* ?) or ~ would be expanded by the shell past the guard; quote it, or name the files`);
    let argv;
    if (role) { const u = unwrapRole(c.argv, role); if (u.deny) return deny(u.deny); argv = u.argv; }
    else argv = unwrap(c.argv);
    if (!argv.length) continue;
    const rawProg = argv[0];
    const prog = basename(rawProg);
    if (prog === 'cd') { // track the folder, so later paths in the line resolve where they will run
      const target = argv[1] && argv[1] !== '-' ? resolve(vcwd, argv[1].replace(/^~(?=\/|$)/, io.home)) : io.home;
      if (role && projectRel(target, vcwd, io) === null && !insideDir(target, input.scratchpad_dir, vcwd, io)) {
        return deny(`routine (${role}): cd outside the project`);
      }
      vcwd = target;
      continue;
    }
    if (role) {
      for (const r of c.redirects) { const d = writeTarget('a redirect', r, vcwd, role, io, input); if (d) return d; }
      for (const r of c.reads) { const d = readOperandDeny(r, vcwd, role, io, input.scratchpad_dir); if (d) return d; }
      if (rawProg !== prog) return deny(`routine (${role}): run ${prog} by name, not by a path (${rawProg})`); // condition 2
      if ((prog === 'git' || prog === 'gh') && argv.some((t) => t.includes('$') || t.includes('`'))) {
        return deny(`routine (${role}): write values out in ${prog} commands (no variables or substitutions)`);
      }
    }
    let r = null;
    if (prog === 'git') r = await git(argv, vcwd, role, io);
    else if (prog === 'gh') r = await gh(argv, role, io);
    else if (role) r = await routineProgram(prog, argv, vcwd, role, io, input);
    if (r) return r;
  }
  return null;
}

// Deny-by-default: a routine runs only the programs on the lists (verdict-3). Everything else is refused, the common
// cases with a message naming what to use instead.
async function routineProgram(prog, argv, vcwd, role, io, input) {
  const args = argv.slice(1);
  if (prog === 'sudo' || prog === 'su' || prog === 'doas') return deny(`routine (${role}): no ${prog}`);
  if (prog === 'kill' || prog === 'pkill' || prog === 'killall') return deny(`routine (${role}): stop the background saver with the TaskStop tool, not ${prog}`);
  if (prog === 'sed' || prog === 'awk' || prog === 'gawk') return deny(`routine (${role}): ${prog} can write files and run code; read with Read/head/tail/grep, edit with Edit`);
  if (CODE_RUNNERS.has(prog)) return deny(`routine (${role}): ${prog} can run or hide other commands; run the program directly`);
  if (SHELLS.has(prog)) return deny(`routine (${role}): ${prog} runs arbitrary commands; run the program directly`);
  if (NET.has(prog)) return deny(`routine (${role}): ${prog} reaches the network; use WebFetch for the documented docs hosts`);
  if (prog === 'npm') return npmCheck(argv, vcwd, role, io);
  if (prog === 'node' || prog === 'nodejs' || prog === 'python' || prog === 'python3') return interpreter(prog, argv, vcwd, role, io);
  if (INSTALLERS.has(prog)) return deny(`routine (${role}): ${prog} installs or runs downloaded code`);
  if (ROLE_READ.has(prog)) {
    const bad = readToolBadOpt(prog, args);
    if (bad) return deny(`routine (${role}): ${prog} ${bad}; not allowed here`);
    // credential- and scope-check every path the tool reads: operands and the file value of an option (-fFILE,
    // --file=FILE). After `--` everything is an operand (H1). For grep the first bare operand is the pattern, not a
    // file, unless -e/-f supplied the pattern.
    // grep has an inline pattern operand only if no pattern-supplying option is present (abbreviations included, I3).
    const patternOpt = (a) => /^(-e|-f)(=|$)/.test(a) || /^-[A-Za-z]*[ef]/.test(a) || abbrevOf(a, 'regexp') || abbrevOf(a, 'file');
    const skipPattern = GREPS.has(prog) && !args.some(patternOpt);
    let seenPattern = false, endOpts = false;
    for (const tok of args) {
      if (!endOpts && tok === '--') { endOpts = true; continue; }
      let cands, isPattern = false;
      if (!endOpts && tok.startsWith('-') && tok !== '-') {
        cands = [];
        const eq = /^--[a-z][a-z-]*=(.*)$/.exec(tok);
        if (eq) cands.push(eq[1]);
        else if (/^-[A-Za-z].+$/.test(tok) && !tok.startsWith('--')) cands.push(tok.slice(2)); // -fFILE (best effort)
        // a bare flag's separate value is a later token, checked as an operand
      } else {
        if (skipPattern && !seenPattern) { seenPattern = true; isPattern = true; } // the grep pattern: credential-checked, not scope-checked
        cands = [tok];
      }
      for (const v of cands) { const d = readOperandDeny(v, vcwd, role, io, input.scratchpad_dir, { credOnly: isPattern }); if (d) return d; }
    }
    return null;
  }
  if (ROLE_WRITE.has(prog)) {
    // mv moves tracked files and whole directories with no flag; routines move files with git mv / Edit, so bash mv is
    // off (H4). rm is allowed only on untracked paths (tracked files go through git rm, so a protected file can't be
    // removed by name or by a directory that holds it).
    if (prog === 'mv') return deny(`routine (${role}): mv isn't allowed; use git mv, or Write the new file and git rm the old`);
    // cp/rm take only short flags here: a long option could be an abbreviation of --recursive/--archive and isn't needed (I1).
    const longOpt = (prog === 'cp' || prog === 'rm') && args.find((a) => a !== '--' && /^--[a-z]/.test(a));
    if (longOpt) return deny(`routine (${role}): ${prog} ${longOpt}: long options aren't allowed here; use short flags, or git rm`);
    if ((prog === 'cp' || prog === 'rm') && args.some((a) => (prog === 'cp' ? /^-[^-]*[rRa]/ : /^-[^-]*[rR]/).test(a))) { // cp -a implies recursive, bundled too (J4)
      return deny(`routine (${role}): recursive ${prog} isn't allowed; name the files, or use git rm for the checkpoint`);
    }
    if (prog === 'cp') {
      const { dests, sources } = cpMvTargets(args);
      for (const s of sources) { const d = readOperandDeny(s, vcwd, role, io, input.scratchpad_dir); if (d) return d; }
      for (const t of dests) { const d = writeTarget('cp', t, vcwd, role, io, input); if (d) return d; }
      return null;
    }
    if (prog === 'rm') {
      for (const t of operands(args)) {
        if (io.git(['ls-files', '--error-unmatch', '--', t], vcwd).code === 0) return deny(`routine (${role}): rm ${t} is tracked; remove it with git rm, or edit it with Edit`);
        const d = writeTarget('rm', t, vcwd, role, io, input); if (d) return d;
      }
      return null;
    }
    const valRe = prog === 'mkdir' ? /^(-m|--mode)$/ : prog === 'touch' ? /^(-d|--date|-r|--reference|-t)$/ : null;
    for (const t of operands(args, valRe)) { const d = writeTarget(prog, t, vcwd, role, io, input); if (d) return d; }
    return null;
  }
  return deny(`routine (${role}): ${prog} is not one of the routines' programs (git, gh, node, python, npm, and read-only tools); a daytime kit change adds one if a run needs it`);
}

// Routines run only a tracked script whose working copy equals main's (R2); the PR Advisor runs none.
function interpreter(prog, argv, vcwd, role, io) {
  if (role === 'pr-advisor') return deny("routine (pr-advisor): the PR Advisor runs no program; CI's gate and smoke results stand in");
  if (!['node', 'python', 'python3'].includes(prog)) return deny(`routine (${role}): ${prog} is not one of the routines' interpreters`);
  const flags = prog === 'node' ? ['--test', '--check'] : ['-I', '-B', '-u'];
  let i = 1;
  for (; i < argv.length && argv[i].startsWith('-'); i++) {
    if (!flags.includes(argv[i])) return deny(`routine (${role}): ${prog} ${argv[i]}: routines run only main's tracked scripts`);
  }
  const script = argv[i];
  if (!script || script === '-') return deny(`routine (${role}): ${prog} needs a tracked script file`);
  const rel = projectRel(script, vcwd, io);
  if (rel === null) return deny(`routine (${role}): ${script} is outside the project`);
  if (DENIED_SCRIPTS.has(rel) || rel.startsWith('.gate/')) return deny(`routine (${role}): ${rel} runs code from the tree; not in routines in phase 0`);
  const root = io.projectDir;
  if (io.git(['ls-files', '--error-unmatch', '--', rel], root).code !== 0) return deny(`routine (${role}): ${rel} is not a tracked file`);
  if (io.git(['diff', '--quiet', 'origin/main', '--', rel], root).code !== 0) return deny(`routine (${role}): ${rel} differs from main's copy`);
  // checkpoint.mjs pushes the branch in a child process the guard never sees: check what it would publish first.
  if (rel === '.claude/scripts/checkpoint.mjs' && /^(save|watch)$/.test(argv[i + 1] ?? '')) {
    const hit = protectedInBranch(vcwd, io);
    return hit ? deny(hit) : null;
  }
  return null;
}

async function git(argv, vcwd, role, io) {
  let i = 1;
  if (argv[1] === '--version' || argv[1] === '--help' || argv.length === 1) return null;
  if (argv[1]?.startsWith('-')) return deny('git: no option before the subcommand (run git from the folder, without -C, -c or --git-dir)');
  const sub = argv[i];
  const args = argv.slice(i + 1);
  if (GIT_NEVER[sub]) return deny(`git ${sub} is not allowed here: ${GIT_NEVER[sub]}`);
  if (sub === 'reflog' && /^(expire|delete)$/.test(args[0] ?? '')) return deny('git reflog expire/delete is not allowed here');
  if (sub === 'stash' && /^(drop|clear)$/.test(args[0] ?? '')) return deny('git stash drop/clear is not allowed here');
  if (sub === 'submodule' && args.includes('foreach')) return deny('git submodule foreach runs commands; not here');
  if (sub === 'worktree' && args[0] === 'remove' && args.slice(1).some((a) => a === '--force' || (/^--f/.test(a) && '--force'.startsWith(a.split('=')[0])) || /^-[a-z]*f/.test(a))) return deny('git worktree remove --force deletes untracked work');
  if (sub === 'config' && args.some((a) => /extraheader|credential|token|password/i.test(a) || a === '--list' || a === '-l' || /^--li/.test(a))) return deny('git config: this reads credentials');
  if (sub === 'config' && role) { // plan M4: routines only read a few named keys
    const key = args[0] === '--get' && args.length === 2 ? args[1] : args.length === 1 ? args[0] : '';
    if (!GIT_CONFIG_READ.has(key.toLowerCase())) return deny(`routine (${role}): git config only reads ${[...GIT_CONFIG_READ].join(' or ')} (git config --get <key>)`);
  }
  if (sub === 'merge' && role) return deny(`routine (${role}): no git merge in routines (a day session merges main in)`);
  if (role && !GIT_ROUTINE.has(sub)) return deny(`routine (${role}): git ${sub} is not on the routines' list`);
  if (role === 'pr-advisor' && !GIT_READ.has(sub)) return deny(`routine (pr-advisor): git ${sub}: the PR Advisor only reads`);
  const rest = gitArgsCheck(sub, args, io);
  if (typeof rest === 'string') return deny(rest);
  if (sub === 'reset') { // only unstaging: [HEAD] [-- <path>...]; anything that could name another commit is refused
    const k = rest.indexOf('--');
    const before = k < 0 ? rest : rest.slice(0, k);
    if (!(before.length === 0 || (before.length === 1 && before[0] === 'HEAD'))) {
      return deny('git reset only unstages here: git reset -q HEAD -- <path>');
    }
  }
  if (sub === 'push') {
    if (!rest.length) return deny('git push: name the remote and the branch (git push origin <branch>)');
    const refs = rest.slice(1).filter((r) => r !== '--');
    if (!refs.length) return deny('git push: name the branch (git push origin <branch>)');
    for (const r of refs) {
      if (r.startsWith('+') || r.startsWith(':') || /(^|:)(refs\/heads\/)?(main|master|HEAD)$/.test(r) || r === 'HEAD') return deny(`git push ${r}: no force, delete, HEAD or main pushes`);
    }
    if (role === 'night-shift') {
      if (rest[0] !== 'origin') return deny('routine (night-shift): push only to origin');
      const cur = io.git(['branch', '--show-current'], vcwd).out.trim();
      if (refs.length !== 1 || refs[0] !== cur) return deny(`routine (night-shift): push only the current branch (${cur || 'none'})`);
      const r = protectedInBranch(vcwd, io);
      if (r) return deny(r);
    }
    if (role === 'pr-advisor') return deny('routine (pr-advisor): the PR Advisor never pushes');
  }
  if (sub === 'fetch' && role) {
    const remote = rest[0];
    if (remote && remote !== 'origin') return deny(`routine (${role}): fetch only from origin`);
    if (rest.slice(1).some((r) => /(^|:|\/)pull\//.test(r))) return deny(`routine (${role}): no pull request refs; read PRs through the GitHub tools`);
  }
  if (sub === 'ls-remote' && role && rest.some((r) => /::|^file:|^\.|^\//.test(r))) return deny(`routine (${role}): ls-remote only on origin`);
  if (sub === 'switch' && role === 'pr-advisor' && !(args[0] === '--detach' && args[1] === 'origin/main' && args.length === 2)) return deny('routine (pr-advisor): the only switch is git switch --detach origin/main');
  if (sub === 'commit' && role === 'night-shift') {
    const staged = io.git(['diff', '--cached', '--name-only'], vcwd).out.split('\n');
    const all = args.some((a) => a === '-a' || a === '--all' || /^-[A-Za-z]*a/.test(a)) ? io.git(['diff', '--name-only'], vcwd).out.split('\n') : [];
    const named = rest.includes('--') ? rest.slice(rest.indexOf('--') + 1) : [];
    const top = gitTop(vcwd, io);
    const hit = [...staged, ...all, ...named].map((f) => f.trim()).filter(Boolean).find((f) => protectedRel(worktreeRel(f, top, vcwd, io)));
    if (hit) return deny(`routine (night-shift): the commit would change ${hit} (a rules, CI or gate file)`);
  }
  if ((sub === 'rm' || sub === 'mv' || sub === 'add') && role) {
    for (const a of rest.filter((x) => x !== '--')) {
      const rel = projectRel(a, vcwd, io);
      if (rel !== null && protectedRel(rel) && sub !== 'add') return deny(`routine (${role}): git ${sub} ${rel} (a rules, CI or gate file)`);
    }
  }
  return null;
}
// A path git printed relative to its top level, as a project-relative path (worktrees under .claude/worktrees/ map
// back to their files).
const gitTop = (vcwd, io) => io.git(['rev-parse', '--show-toplevel'], vcwd).out.trim() || io.projectDir;
function worktreeRel(f, top, vcwd, io) {
  return projectRel(join(top, f), vcwd, io) ?? f;
}
function protectedInBranch(vcwd, io) {
  const d = io.git(['diff', '--name-only', 'origin/main...HEAD'], vcwd);
  if (d.code !== 0) return "routine (night-shift): can't compare the branch with origin/main; fetch it first";
  const top = gitTop(vcwd, io);
  const hit = d.out.split('\n').map((f) => f.trim()).filter(Boolean).find((f) => protectedRel(worktreeRel(f, top, vcwd, io)));
  return hit ? `routine (night-shift): the branch changes ${hit} (a rules, CI or gate file); it can't be pushed at night` : null;
}

async function gh(argv, role, io) {
  const sub = argv[1] ?? '', sub2 = argv[2] ?? '';
  if (['auth', 'secret', 'variable', 'ssh-key', 'gpg-key', 'extension', 'codespace'].includes(sub)) return deny(`gh ${sub} is not allowed here`);
  if (sub === 'repo' && /^(edit|delete|rename|archive|unarchive|fork|set-default|sync|deploy-key)$/.test(sub2)) return deny(`gh repo ${sub2} is not allowed here`);
  if ((sub === 'alias' || sub === 'config') && sub2 === 'set') return deny(`gh ${sub} set is not allowed here`);
  if (sub === 'pr' && sub2 === 'merge') return deny('gh pr merge is not allowed: merge with the GitHub MCP merge tool and the approved SHA as expectedHeadSha');
  if (sub === 'api') {
    let method = 'GET', write = false, endpoint = null;
    for (let i = 2; i < argv.length; i++) {
      const t = argv[i];
      if (t === '-X' || t === '--method') { method = argv[++i] ?? ''; continue; }
      if (/^-X./.test(t)) { method = t.slice(2); continue; }
      if (t.startsWith('--method=')) { method = t.slice(9); continue; }
      if (/^(-f|-F|--field|--raw-field|--input)$/.test(t)) { write = true; i++; continue; }
      if (/^-[fF]./.test(t) || /^--(field|raw-field|input)=/.test(t)) { write = true; continue; }
      if (/^(-H|--header|--hostname|-q|--jq|-t|--template|--cache)$/.test(t)) { i++; continue; }
      if (t.startsWith('-')) continue;
      endpoint ??= t;
    }
    if (write || method.toUpperCase() !== 'GET') return deny('gh api is read-only here; GitHub writes go through the GitHub MCP tools');
    if (role && endpoint && !new RegExp(`^/?(repos/${REPO.replace('/', '\\/')}(/|$)|rate_limit$)`, 'i').test(endpoint)) return deny(`routine (${role}): gh api only on ${REPO}`);
    return null;
  }
  if (role) { // deny-by-default (F6): only the named read-only forms; gh api (read) returned above
    if ((sub === 'pr' || sub === 'issue') && /^(view|list|status|diff|checks)$/.test(sub2)) return null;
    return deny(`routine (${role}): gh ${sub}${sub2 ? ' ' + sub2 : ''}: routines use only gh api (read), gh pr/issue view|list|status|diff|checks, and the GitHub MCP tools`);
  }
  return null;
}

// ---------- MCP ----------
const REMOTE_READ = /^(list_|get_|read_documentation$|check_repo_access$)/;
const GITHUB_READ = /^(get_|list_|search_)|_read$/;

async function mcp(tool, ti, role, io) {
  const m = /^mcp__(.+?)__(.+)$/.exec(tool);
  const server = m?.[1] ?? '', name = m?.[2] ?? '';
  if (server === 'github') {
    if (name === 'enable_pr_auto_merge') return deny('auto-merge skips the expected-head check; merge with the MCP merge tool instead');
    // merge fails closed for every session (K1): if the check can't complete, refuse rather than let it through
    if (name === 'merge_pull_request') { try { return await mergeCheck(ti, role, io); } catch (e) { return deny(`merge: the check could not complete (${String(e?.message ?? e).slice(0, 80)}); refused`); } }
    if (!role || GITHUB_READ.test(name)) return null;
    if (`${ti.owner}/${ti.repo}`.toLowerCase() !== REPO.toLowerCase()) return deny(`routine (${role}): GitHub writes only on ${REPO}`);
    return githubWrite(name, ti, role, io);
  }
  if (!role) return null;
  if (server === 'claude-code-remote') return REMOTE_READ.test(name) ? null : deny(`routine (${role}): routines don't change routines or sessions`);
  return deny(`routine (${role}): the ${server} connector is not used in routines`);
}

function ghJson(io, path) {
  const r = io.gh(['api', path]);
  if (r.code !== 0) throw new Error(`gh api ${path} failed`);
  return JSON.parse(r.out);
}
async function githubWrite(name, ti, role, io) {
  const n = Number(ti.issue_number ?? ti.pullNumber ?? ti.pull_number);
  const isPr = (num) => io.gh(['api', `repos/${REPO}/pulls/${num}`]).code === 0;
  const ownPr = (num) => {
    const cur = io.git(['branch', '--show-current'], io.projectDir).out.trim();
    const pr = ghJson(io, `repos/${REPO}/pulls/${num}`);
    return cur && pr.head?.ref === cur && pr.head?.repo?.full_name?.toLowerCase() === REPO.toLowerCase();
  };
  const commentIssue = (id) => Number(String(ghJson(io, `repos/${REPO}/issues/comments/${id}`).issue_url ?? '').split('/').pop());
  if (role === 'night-shift') {
    if (name === 'add_issue_comment') return n === 16 || ownPr(n) ? null : deny('routine (night-shift): comments only on #16 and its own PRs');
    if (name === 'update_issue_comment') { const k = commentIssue(ti.comment_id); return k === 16 || ownPr(k) ? null : deny('routine (night-shift): edits only its comments on #16 and its own PRs'); }
    if (name === 'issue_write') return ti.method === 'update' && n === 16 && ti.title === undefined ? null : deny('routine (night-shift): issue_write only updates #16');
    if (name === 'create_pull_request') {
      const cur = io.git(['branch', '--show-current'], io.projectDir).out.trim();
      return ti.head === cur && (ti.base ?? 'main') === 'main' ? null : deny('routine (night-shift): a PR only from the current branch into main');
    }
    if (name === 'update_pull_request') {
      if (ti.base !== undefined || ti.state !== undefined) return deny('routine (night-shift): no base or state changes');
      return ownPr(n) ? null : deny('routine (night-shift): only its own PRs');
    }
    return deny(`routine (night-shift): ${name} is not on the night shift's GitHub list`);
  }
  // pr-advisor
  if (name === 'add_issue_comment') return n !== 16 && isPr(n) ? null : deny('routine (pr-advisor): comments only on pull requests, never on #16');
  if (name === 'update_issue_comment') { const k = commentIssue(ti.comment_id); return k !== 16 && isPr(k) ? null : deny('routine (pr-advisor): edits only its review comments on pull requests'); }
  if (name === 'issue_write') {
    const keys = Object.keys(ti).filter((k) => !['owner', 'repo', 'method', 'issue_number', 'labels'].includes(k));
    return ti.method === 'update' && !keys.length && n !== 16 && isPr(n) ? null : deny('routine (pr-advisor): issue_write only sets labels on pull requests');
  }
  return deny(`routine (pr-advisor): ${name} is not on the PR Advisor's GitHub list`);
}

async function mergeCheck(ti, role, io) {
  const sha = String(ti.expectedHeadSha ?? '');
  if (!/^[0-9a-f]{40}$/.test(sha)) return deny('merge: pass the approved head as expectedHeadSha (the full 40-character SHA)');
  const repo = `${ti.owner}/${ti.repo}`;
  if (role && repo.toLowerCase() !== REPO.toLowerCase()) return deny(`routine (${role}): merges only on ${REPO}`);
  if (role === 'night-shift') return deny('routine (night-shift): the night shift never merges');
  const pr = ghJson(io, `repos/${repo}/pulls/${ti.pullNumber}`);
  if (pr.head?.sha !== sha) return deny(`merge: the PR's head is ${pr.head?.sha}, not ${sha}`);
  if (pr.head?.repo?.full_name?.toLowerCase() !== pr.base?.repo?.full_name?.toLowerCase()) return deny('merge: the head is in another repository (a fork); not merged here');
  const runs = ghJson(io, `repos/${repo}/commits/${sha}/check-runs?per_page=100`).check_runs ?? [];
  for (const nm of ['gate', 'smoke']) {
    const rs = runs.filter((r) => r.name === nm);
    if (!rs.length) return deny(`merge: no ${nm} check on ${sha.slice(0, 7)}`);
    if (rs.some((r) => r.status !== 'completed')) return deny(`merge: ${nm} is still running on ${sha.slice(0, 7)}`);
    if (rs.some((r) => !['success', 'skipped', 'neutral'].includes(r.conclusion)) || !rs.some((r) => r.conclusion === 'success')) return deny(`merge: ${nm} is not green on ${sha.slice(0, 7)}`);
  }
  if (role === 'pr-advisor') {
    const r = io.gh(['api', '--paginate', `repos/${repo}/pulls/${ti.pullNumber}/files?per_page=100`, '--jq', '.[].filename']);
    if (r.code !== 0) throw new Error('gh api pulls/files failed');
    const hit = r.out.split('\n').map((f) => f.trim()).filter(Boolean).find(protectedRel);
    if (hit) return deny(`routine (pr-advisor): the PR changes ${hit}; rules, CI and gate changes merge from a day session on Mustafa's go`);
  }
  return null;
}

// ---------- the real io ----------
export function realIo() {
  const cache = new Map();
  const run = (cmd, args, cwd) => {
    const r = spawnSync(cmd, args, { cwd, encoding: 'utf8', timeout: 20000, maxBuffer: 32 * 1024 * 1024 });
    return { code: r.status ?? 1, out: r.stdout ?? '' };
  };
  const projectDir = process.env.CLAUDE_PROJECT_DIR || process.cwd();
  return {
    env: process.env,
    projectDir,
    home: homedir(),
    git: (args, cwd) => run('git', args, cwd || projectDir),
    gh: (args) => run('gh', args, projectDir),
    exists: (p) => existsSync(p),
    realpath: (p) => realpathSync(p),
    readFile: (p) => readFileSync(p, 'utf8'),
    gitLongOptions: (sub) => {
      if (!cache.has(sub)) {
        const r = /^[a-z][a-z-]*$/.test(sub) ? run('git', [sub, '--git-completion-helper'], projectDir) : { out: '' };
        cache.set(sub, r.out.split(/\s+/).filter((o) => o.startsWith('--')));
      }
      return cache.get(sub);
    },
  };
}
