# U4a + U4b review, rust-reviewer (read only), round 1

U4a: /home/user/Shamble-Frontlines/.claude/worktrees/agent-a9297644e7c0354f3 (base dffef0e), crates/proto + spikes/netcode-proof/check.mjs.
U4b: /home/user/Shamble-Frontlines/.gate/u4b-wt (base 4b5812a), crates/server schedule.rs.
Plan: u4-build-plan.md §5 U4a/U4b, §4.1, §4.3, §4.4-4.6, §4.10, §4.11, §4.14; advisor-u4.md; u4s-carries.md (U4a 1-5, 34; U4b 6, 35).
(Notes are rewritten whole with Write: the Edit tool is not available in this run.)

## Plan
1. Read plan §4.1 (pub items list), §4.3-4.6, §4.10, §4.11, §4.14, §5 U4a/U4b; advisor rulings; carries 1-6, 34, 35.
2. U4a: git status/diff stat; read every src file (bits, control, error, hash, input, limits, msg, quant, snapshot, wrap, lib), clippy.toml, Cargo.toml.
   Check: unsafe, panics from wire input (indexing, slicing, unwrap, arithmetic overflow, shifts), allocation bounds, float use (quant only), HashMap, integer math, wasm32 (usize 32-bit), bites per instance.
3. U4a tests: wire.rs, bites.rs, catalogue.rs, common/mod.rs; check.mjs/check.test.mjs (only as far as Rust-relevant).
4. U4a public items: list every pub item vs §4.1; needed now or "for later".
5. S1 (skeptic, U4s): GROUPS tables and per-(group, kind) field selection; weigh.
6. U4b: git diff; read schedule.rs, lib.rs, clippy.toml, tests/schedule.rs. Integer time math, overflow, catch-up policy, pub items (Dropped, LoopCounters, next_due_ns, tick_hz, catchup, counters), clippy tokio allow-invalid.
7. Findings -> verdicts.

## Progress
- Steps 1-5 done (U4a read whole: src 11 files, tests wire/bites/common, catalogue diff skimmed, check.mjs diff). No
  unsafe (workspace forbid inherited), no static/thread_local/Atomic/OnceLock in proto src, no f32/f64 in proto src, no
  HashMap; panics only in tests; decode order size -> type -> type size -> length -> direction -> channel -> body;
  every index guarded (msg.rs:369-392, bits.rs:69-128, control.rs:557-566). Release overflow-checks on for npt-proto
  (workspace Cargo.toml:111-112): arithmetic checked: all widths <= 32 bits in i64/u64, shifts < 64; none reachable.
  wasm32: KIND_BITS (snapshot.rs:157) is 3 on both (usize::BITS - lz); no usize in any output. Bites: Quant and Wrap
  values, per instance (quant.rs:57-61, wrap.rs:16-21). Decode allocation bounded: Input with_capacity(count<=8)
  (input.rs:200-204), peers <=16 (control.rs:389-392), Snapshot/FullState push-as-decoded (>=21 bits per item:
  snapshot.rs:521-534, :616-623); size refused before any allocation (msg.rs:363). Carries 1-5, 34: met per the
  builder's record (u4a-build.md:122-126); nothing here contradicts them.
- F-A1 Medium (S1 weighed): one layout per kind for own state and remote soldiers; U4d's worktree layout
  (wf_f54246ff-e0d-1 seam.rs:208-221) adds HEALTH, WEAPON, WEAPON_STATE (0 ready,1 cooling,2 reloading,3 empty) to
  SOLDIER, so with U4a's put_delta (snapshot.rs:223-276) every client that sees a soldier gets its velocity and weapon
  state (catalogue :136 owner-only, :137 remote row has neither). GROUPS (snapshot.rs:429-454, :566-588) is read by
  one unit test only (snapshot.rs:638-652); encode/decode never consult it. Fix needs an audience per layout entry
  from npt-sim (seam change, condition 3: after U4d's merge) + a per-(section, kind) field selection in proto. Gate:
  before U4e's brief (it changes the wire format U4e builds on); not U4a's lane now.
- F-A2 Low: GROUPS not enforced: an Effect (owner-only) Add in Snapshot.entities or FullState.entities encodes and
  decodes; cheap check now (kind per section), folds into F-A1.
- F-A3 Medium: encode's Snapshot limit is the receive limit 1,150, not the send cap 1,000: content_limit
  (msg.rs:219-231) used by encode_into (msg.rs:343); contradicts EncodeError::TooLarge's doc (error.rs:48-49: "above
  its type's limit (Limits)") and Limits::SNAPSHOT/TYPES (limits.rs:23-24, :61-66); generator common/mod.rs:245-249
  builds up to 1,150. Proof: encode a 1,001 B snapshot, expect TooLarge (Ok today). Before merge (cheap; a pub fn's
  contract U4e's encoder budget builds on): tighten (preferred) or document + carry into U4e's brief.
- F-A4 Low: pub items without docs: Limits::RESYNC..KICK, SESSION_STATUS (limits.rs:29-38, :41); Direction::
  ClientToRoom/RoomToClient (msg.rs:31-32); JoinAccepted slot/team/squad/mode_hash/content_hash (msg.rs:79-85);
  JoinRejected variants (msg.rs:97-101); Message variants (msg.rs:157-171); ProfileId P0-P4, StepResult variants,
  SessionStatus::profile, TokenKind variants, RoomControl variant fields (control.rs:32-46, :70, :282-344).
- F-A5 Low: client_hash allocates a Vec when its input is unsorted (hash.rs:63, :92) and "any order" holds only for
  unique ids (stable sort keeps input order for equal keys).
- F-A6 Low: never-0 guard (hash.rs:96) untestable by a8's 100,000 random views (p ~ 5e-15): a plant removing it passes.
- F-A7 Low: client_hash runs in Wasm (client) and native (room); a8's vectors are native only. Parity by construction
  (integers, FNV); add a8's vectors to U4d2's Wasm-in-Node run.
- F-A8 Low nits: Snapshot::GROUPS vs Limits::GROUPS one name two meanings; input.rs:205/:285 `let _ = i;` leftover;
  Wrap counter names are free strings (wrap.rs:33-40): a misspelt name plants nothing; decode accepts
  Direction::BothWays as a receiving end (msg.rs:33-34), which no end is.
- Pub items (A19) U4a: needed now: Message, Direction, DecodeError, EncodeError (encode/decode signatures); EndReason,
  ProfileId, StepResult, TokenKind (fields of listed messages/controls); Limits::TYPES + consts (Limits listed;
  catalogue item 3 test); Wrap after8/16/32 + widen16 (§4.4 one compare per width, §4.7 rebuild); Quant::encode/
  decode/with_bite; EntityDelta between/apply (the mapping). Test-driven: Message::name, SessionControl::VARIANTS/
  variant, RoomControl::NAMES/name (catalogue item 6 audit, approved). Revised after re-reading §5 U4a ("the Snapshot
  container ... and its field-group layout table"): Snapshot::GROUPS, FullState::GROUPS and Class are that plan item,
  so not "for later", but inert today (only snapshot.rs:638-652 reads them): F-A1/F-A2 make them operative.
- Step 6 (U4b) read whole: schedule.rs (263 lines), lib.rs diff, clippy.toml, tests/schedule.rs (389 lines), Cargo.toml.
  Checked: no unsafe, no float, no HashMap, no alloc, no clock; pure; u128 math: last_due (schedule.rs:139-141)
  closed form proven (floor(k e9/hz) <= e iff k e9 <= (e+1)hz - 1); (u64::MAX+1)*1000 fits u128; result <= 1.8e13;
  offset_ns saturates; every subtraction guarded (now<base :156, last<next :160, resume >= offset(next+run) by
  monotonicity). Worked 30 Hz 1 s: last 30, owed 30, run 7, drop 23, 1,033,333,333 - 266,666,666 = 766,666,667: matches
  the test. 200 ms at 30 Hz: last 6, owed 6. Bites per instance (field :31-32, with_bite :257-262), no static state.
  npt-server is native only (§4.3: the Worker reads LOOP_CATCHUP through npt-client-wasm), so u128 on wasm32 is moot.
  Carry 6 met (tests:210-217, rates() :8-13); carry 35 met (only NS_PER_S :17; nothing from TICK_HZ).
- Pub items (A19) U4b: Schedule, Due, due_ticks (§4.1); Dropped, LoopCounters (field/return types of those: needed);
  next_due_ns (U6's timer; tests' "no tick owed" check: needed); counters (monitoring, ADR 0004 part 5; tests: needed);
  tick_hz, catchup (carry 6's read-back test and the test helpers: test-driven, needed now). None for later. Nit: each
  item has two paths (npt_server::schedule::X and npt_server::X).
- F-B1 Low: clippy.toml:7, :12 tokio entries with allow-invalid. Clippy resolves config paths through every crate in
  the session (clippy_utils/src/paths.rs find_crates: tcx.crates(()), which includes crates loaded transitively), and
  rustc loads an --extern crate only when code names it; so the entries go live when npt-server's code first names a
  crate whose metadata loads tokio (npt-net), which can be before any direct tokio dependency; the record's "until U6
  names tokio as a direct dependency" (u4b-build.md) is imprecise. allow-invalid also hides a misspelt path forever.
  Follow-up (U6's brief): drop allow-invalid once tokio resolves, plant tokio::time::Instant::now() to show it live;
  the std half can be planted now (std::time::Instant::now() in schedule.rs -> clippy disallowed_methods).
- F-B2 Low: Schedule::new panics (schedule.rs:78-85) on rate 0/>1000 or cap 0: fine for npt-sim's constants; a carry
  for U4r/U6/npt-lab: build it in a const (`const S: Schedule = Schedule::new(TICK_HZ, LOOP_CATCHUP)`, it is a const
  fn, so a bad constant fails the build), never from a parsed scenario value (a panic from content data).
- F-B3 Low nits: counters use plain += (schedule.rs:177, :191-193) while base_ns saturates; npt-server has no release
  overflow-checks; unreachable (2^64 wakes; dropped_ns <= now). At now near u64::MAX next_due_ns saturates to
  u64::MAX <= now ("no tick left owed" fails at that edge; harmless). Literal caps in tests (:290, :296, :302, :329)
  are refusal/lookup tests, not 60 Hz runs: carry 6's wording holds.

## Verdicts
- U4a: REVISE (one before-merge item: F-A3; F-A1 gates U4e's brief, after U4d's merge; the rest follow-ups).
- U4b: APPROVE WITH FINDINGS (F-B1..F-B3, all Low follow-ups).
DONE.
