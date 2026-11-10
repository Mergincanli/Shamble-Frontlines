# U4a/U4b review round 2 (rust-reviewer), on what changed only

Started 2026-11-10 (resume after usage limit; no earlier round-2 notes found).
Read only: no edit, build or run in any copy.

## Plan
1. Read round-1 notes (rust + netcode + security + test) for the finding ids the fix claims to close.
2. diff -ru u4a-prefix proto vs worktree proto; read each changed hunk.
3. Judge each claimed closure: N1/SEC-U4a-7/F-A3/T-A1 (send cap), N6/SEC-U4a-6/T-A6 (ack encoding),
   SEC-U4a-3 (Debug redaction), F-A6/T-A7 (finish guard), nit, N2 record.
4. Look for defects added by the fix (panics, arithmetic, append-on-error, Debug leaks, clippy/fmt-risk, docs).
5. diff -ru u4b-prefix vs u4b-wt server; judge T-B1, T-B2, T-B3.
6. Verdicts.

## Progress
- Steps 1-2 done: round-1 notes read (rust F-A3/F-A6/F-A8; netcode N1/N2/N6; security SEC-3/6/7; test T-A1/A6/A7).
  diff -ru prefix vs worktree proto: 8 files, = the fixer's list; sha256 (first 16) of all 8 = the record; snapshot,
  bits, lib, quant, wrap = round-1 baseline. No other crate in the U4a copy names npt_proto (grep).
- F-A3/N1/SEC-7/T-A1 CLOSED: msg.rs:235-259 send_limit/receive_limit/after_length; check moved to Quant::encode
  msg.rs:308-322 (out.len()-start-framing >= 0 after Ok; truncate on any Err); decode msg.rs:406 and channel_max
  :275/:279 on receive_limit; docs error.rs (TooLarge both), limits.rs SNAPSHOT/_RECEIVE, msg.rs module text :17-19.
  Length prefix now written before the check (msg.rs:378-383, `as u16`/`as u32` may truncate for an oversized
  message) but any such message is refused and truncated away (control send_limit <= 510; FullState 65,536): no leak.
  Other types unchanged in behaviour (content_limit already = row limit for them). Generator common/mod.rs:247 trims
  via encoded().is_none(), so it now yields <= 1,000 B snapshots (a1 :46 unwrap safe). Test msg.rs a3 pair reads right.
- N6/SEC-6/T-A6 CLOSED: input.rs:134-136 (NotCanonical), :205-207 (UnknownField under strict, same pattern as
  :238/:277, snapshot.rs:491); generator same RNG draw order (tuple left to right = old field order); wire a6 tail.
- SEC-3 CLOSED: control.rs Redacted + SessionControl Debug; msg.rs Join Debug; no other [u8;16] secret in proto
  (grep: RoomControl `cap: u16` is a number, JoinAccepted hashes public). Test's bytes 0xa7/0xb3/0xc9 (167/179/201)
  appear in no field name or other value: test meaningful.
- F-A6/T-A7 CLOSED for the guard's logic (hash.rs finish + unit test); the call site client_hash -> finish(h.0) is held
  by reading only (a plant that bypasses finish passes both tests): residual, Low, not reopened.
- F-A8 (let _ = i) CLOSED. N2: text only, recorded for the main session (catalogue/plan edits outstanding: N8/T-A9).
- Candidate R2-L1 (Low, new in the fix, test only): wire.rs:228-232 as_sent maps any encode error to None, so for a
  decoded 1,001-1,150 B Snapshot a2 accepts NotCanonical/OutOfRange as well as TooLarge; reach: random half only
  (wire.rs:210 max 1,151 B), improbable. Cheap fix: assert encode == Err(TooLarge) in that band.
- Lints (workspace Cargo.toml:73-99): forbid unsafe_code, deny disallowed_types/methods, iter_over_hash_type; no
  pedantic/nursery, so Redacted's pub(crate) and the and_then form raise nothing (fixer: clippy -D warnings clean host
  + wasm32, both builds; the test-engineer rebuilds).
- Step 5 U4b: diff = tests/schedule.rs only (sha256 dfb7da2c -> 05e2068b); schedule.rs f9076ff8, lib.rs 2b62afca,
  clippy.toml 5a363a03 = round-1 baseline.
  - T-B3 CLOSED: tests/schedule.rs:221-226: wake 0 then 1e9 ns: last_due(1e9,30)=30, next=1, owed 30, run 7, drop 23
    = LOOP_CATCHUP / TICK_HZ - LOOP_CATCHUP (ticks.rs:12, :47). The cap acted on, not the stored one; 60 Hz runs keep a
    literal-7 mutant visible. :215-218 (LOOP_CATCHUP == loop_catchup_at(TICK_HZ)) is true by ticks.rs:47: harmless.
  - T-B1 CLOSED: :300-311: at(3,30)=1e8, at(3,60)=5e7 exact (premise asserted). last_due(1e8-1,30)=(3e9-1)/1e9=2 ->
    (2, at(3)); last_due(1e8,30)=(3e9+29)/1e9=3 -> (3, at(4)); 60 Hz alike (59 for 29). Mutant without "- 1": early
    wake gives (3, at(4)): fails (main session's plant failed it by name). No underflow; no literal caps (carry 6).
  - T-B2 CLOSED (comment only, :294-295; wording accurate).
  - No defect added. F-B1..F-B3 (round 1, Low) untouched: stay follow-ups.

## Verdicts (round 2)
- U4a: APPROVE WITH FINDINGS. The before-merge item F-A3 (= N1, SEC-7, T-A1) is closed, as are N6/SEC-6/T-A6,
  SEC-3, F-A6/T-A7 (guard logic; call site by reading), F-A8's `let _ = i`. New: R2-L1 Low (test only). Still open
  and not reopened: F-A1/F-A2 (S1, gates U4e's brief after U4d), F-A4, F-A5, F-A7, F-A8's other nits; N2/N8 text
  edits are the main session's.
- U4b: APPROVE WITH FINDINGS (no new finding; F-B1..F-B3 stay follow-ups).
DONE.
