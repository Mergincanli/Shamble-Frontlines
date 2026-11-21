# security-auditor: U4a / U4b round 2 (on what changed only)

Started 2026-11-10. Read-only. Focus U4a: SEC-U4a-7 (send cap), SEC-U4a-6 (canonical ack), SEC-U4a-3 (Debug redaction,
every type holding a token or capability), and no new decode path that panics or allocates before its size check.
U4b: not reviewed (out of brief).

## Plan
1. diff -ru u4a-prefix/proto vs worktree/proto; read fixer's "Fix pass, round 1" notes.
2. SEC-7: send_limit / receive_limit split in msg.rs; Quant::encode framing + size check; nothing appended on error;
   channel_max uses receive; FullState; test a3_a.
3. SEC-6: input.rs decoder (strict) + encoder NotCanonical; generator; wire.rs a6 last case.
4. SEC-3: hand-written Debug for Join, SessionControl, Message; grep every type with token/bind/cap/secret fields
   (msg.rs, control.rs, others); derived Debug on wrappers (Message, RoomControl, errors, Quant?) that embed them.
5. New decode paths: any new index/slice/arith/alloc in changed decode code.
6. Findings, verdicts.

## Progress
- 1 done: diff read (8 files, 629 lines); sha256 of all 8 files match the fixer's report.
- 2 SEC-U4a-7 CLOSED: msg.rs:235-238 send_limit = row limit (Snapshot row = Limits::SNAPSHOT 1,000, limits.rs:64-67);
  Quant::encode :308-322 checks after encode_into, truncates on any error; :312 no underflow (encode_into appended
  frame + type byte at least); decode :406 and channel_max :275/:279 use receive_limit (1,150 for Snapshot, identical
  to old content_limit); encode_into private, only callers :311 and the in-module test :559. Length prefix written
  (as u16/u32, :380-381) before the size check, but refused + truncated after: no defect (no control type can exceed
  65,535 anyway).
- 3 SEC-U4a-6 CLOSED: input.rs:205-207 refuses ack_groups 0 with tick != 0 as UnknownField when strict (strict() is
  true outside a bites build, quant.rs:116-122), before count / with_capacity (no new alloc); encoder :134-136
  NotCanonical ungated. Matches Snapshot's flag rule.
- 4 SEC-U4a-3 CLOSED: Join (msg.rs:81-90) and SessionControl (control.rs:78-109) hand-written Debug with
  Redacted(len) for token, bind, cap in every variant; Message's derived Debug delegates. Grep: no other [u8;16]
  secret in proto (RoomControl holds slot/kind only; mint replies are U6's npt-server); no serde, no Display on them;
  errors data-free. Other crates (room, server, client) hold no token/cap today. Test wire.rs debug_output_... covers
  4 variants + Join, direct and through Message.
- 5 no new decode path: decode-side changes are input.rs:205 (comparisons only) and receive_limit (const, old
  content_limit renamed). hash.rs finish() is not a decode path, no panic.
- 5b considered, no finding: wire.rs a2's as_sent (:228-231) only asks that encode refuse a decoded 1,001-1,150 B
  Snapshot, not that it be canonical. Random buffers reach 1,151 B (random_buffer :207-226), but a valid snapshot that
  large from random or 1-3-bit-flipped bytes is improbable, and decode has no length-dependent rule beyond the limit
  check (msg.rs:398-424), so the canonicality proof loses nothing real. Only clients decode Snapshot (direction check),
  so this has no anti-cheat reach. The server's receive bounds are unchanged: msg.rs:546-548 (166 / 512 / 514),
  untouched by the diff. Test-engineer's call, if any.
- 5c N2 (header carries command sequences, not tick offsets): record text only; no server-side input changes, no
  security bearing.
- Fixer's notes (worktree u4a-build.md "Fix pass, round 1") read: claims match the diff; before/after failures
  recorded for fixes 1-3 (fix 3 failed showing token bytes 167 in Debug, as expected); plants PF4 and PF2 bite.

## Findings (round 2)
- None new. The fix adds no defect in my scope.

## Carries (not U4a findings; for the owners' units)
- U4e: encode's TooLarge is now reachable at 1,000 B for Snapshot. The room's builder must budget against the same
  number (Limits::SNAPSHOT / the row's limit, one home), handle TooLarge without a panic or expect, and keep the
  event and own-state sections bounded, so that other players' actions (a scripted crowd firing near a victim) can't
  push that victim's snapshot over the cap every tick. That would starve the victim into Kick::Overflow (plan risk 5
  measures the budget, not this abuse).
- U4e / U6: any new type holding a token or capability (room roster, ControlSocket's mint replies, U6) needs the same
  length-only Debug. control.rs's Redacted is pub(crate), so other crates can't reuse it: a pub secret newtype in
  proto, or each crate's own, plus a test like wire.rs debug_output_shows_a_secrets_length_never_its_bytes.
- Round-1 SEC-U4a-1 (ruling, binds U4d), -2, -4, -5: not claimed closed by this pass. Unchanged, they stay open
  follow-ups (each keeps needs-mustafa until addressed or explicitly deferred).

## Durable items (for my memory; this round is read-only)
- Threat-model rule held in U4a: a secret's (token, bind, capability) Debug prints its length only (Redacted "[16 B]");
  every later type holding one follows it.
- Snapshot limits: encode refuses above 1,000 B (send cap), the client decodes up to 1,150 B; every other type has
  one limit for both ends.

## Verdict
U4a: APPROVE WITH FINDINGS. SEC-U4a-3, -6 and -7 are closed, and the fix adds no new decode path that panics or
allocates before its size check. "With findings" covers round 1's still-open SEC-U4a-1, -2, -4 and -5 (follow-ups,
not reopened). U4b: not reviewed (out of brief). DONE 2026-11-10.
