#!/usr/bin/env node
// Guard hook launcher (kit-v3-1). Registered in .claude/settings.json for PreToolUse, PermissionRequest and
// ConfigChange with "onFailure": "block". It stays small and rarely changes: it loads the rules in routine-guard.mjs
// and prints their decision as JSON. If the rules can't load or throw, a routine (SHAMBLE_ROLE set) is refused and a day
// session goes on (plan R3c), so a broken rules file never locks a day session out of fixing it. Whenever this file
// runs to its end, it exits 0: a decision travels as JSON, and any other exit code would count as a hook failure.
// settings.json does not run this file directly: each handler runs a one-line bootstrap (`node -e`, GUARD-FILE,
// Mustafa 2026-11-03) that imports it. A routine keeps the hook settings it loaded (this guard blocks the ConfigChange
// reload); a day session reloads them. Either way a branch switch changes the files on disk under a live hook, so this
// file itself can be missing or unable to load, or end without its own exit (an empty file, an await that never
// settles). The bootstrap then applies the R3c rule one level up: a day session
// exits 0 with no output (no decision: the call, and any settings reload, go on), a routine exits 1, a hook failure
// that "onFailure": "block" turns into a block. Only the bootstrap ever exits non-zero.
// The invariant this file keeps: nothing outside a try can throw (the stdin read has its own), the work runs inside the
// one top-level try, and the file ends with its single, unconditional process.exit(0). Removing that exit would block
// every routine call (the bootstrap's beforeExit check).
// An ask (kit-v3-3: a day session's kit write or main-folder move) is printed as an ask only on PreToolUse with no
// SHAMBLE_ROLE; anywhere else it is printed as a deny with the same reason (C5: an ask never reaches a routine).
import { readFileSync } from 'node:fs';

const role = process.env.SHAMBLE_ROLE || '';
let input = null;
try { input = JSON.parse(readFileSync(0, 'utf8')); } catch { input = null; }
const event = input?.hook_event_name || '';

function print(reason, interrupt, asks = false) {
  const msg = String(reason).slice(0, 2000);
  let out;
  if (event === 'PermissionRequest') {
    out = { hookSpecificOutput: { hookEventName: 'PermissionRequest', decision: { behavior: 'deny', message: msg, interrupt: !!interrupt } } };
  } else if (event === 'ConfigChange') {
    out = { decision: 'block', reason: msg };
  } else {
    const permissionDecision = asks ? 'ask' : 'deny';
    out = { hookSpecificOutput: { hookEventName: 'PreToolUse', permissionDecision, permissionDecisionReason: msg } };
  }
  process.stdout.write(JSON.stringify(out));
}

try {
  if (!input) throw new Error('unreadable hook input');
  const { decide, realIo } = await import('./routine-guard.mjs');
  const d = await decide(input, realIo());
  if (d && d.deny === undefined && d.ask !== undefined) print(d.ask, false, event === 'PreToolUse' && !role);
  else if (d) print(d.deny, d.interrupt);
} catch (e) {
  if (role) print(`guard error (${String(e?.message ?? e).slice(0, 300)}): refused, because a routine fails closed`, true);
}
process.exit(0);
