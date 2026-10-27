#!/usr/bin/env node
// Guard hook launcher (kit-v3-1). Registered in .claude/settings.json for PreToolUse, PermissionRequest and
// ConfigChange with "onFailure": "block". It stays small and rarely changes: it loads the rules in routine-guard.mjs
// and prints their decision as JSON. If the rules can't load or throw, a routine (SHAMBLE_ROLE set) is refused and a day
// session goes on (plan R3c), so a broken rules file never locks a day session out of fixing it. Always exits 0: a
// decision travels as JSON, and any other exit code would count as a hook failure.
import { readFileSync } from 'node:fs';

const role = process.env.SHAMBLE_ROLE || '';
let input = null;
try { input = JSON.parse(readFileSync(0, 'utf8')); } catch { input = null; }
const event = input?.hook_event_name || '';

function print(reason, interrupt) {
  const msg = String(reason).slice(0, 2000);
  let out;
  if (event === 'PermissionRequest') {
    out = { hookSpecificOutput: { hookEventName: 'PermissionRequest', decision: { behavior: 'deny', message: msg, interrupt: !!interrupt } } };
  } else if (event === 'ConfigChange') {
    out = { decision: 'block', reason: msg };
  } else {
    out = { hookSpecificOutput: { hookEventName: 'PreToolUse', permissionDecision: 'deny', permissionDecisionReason: msg } };
  }
  process.stdout.write(JSON.stringify(out));
}

try {
  if (!input) throw new Error('unreadable hook input');
  const { decide, realIo } = await import('./routine-guard.mjs');
  const d = await decide(input, realIo());
  if (d) print(d.deny, d.interrupt);
} catch (e) {
  if (role) print(`guard error (${String(e?.message ?? e).slice(0, 300)}): refused, because a routine fails closed`, true);
}
process.exit(0);
