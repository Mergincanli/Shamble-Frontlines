# Advisor: U4 wave 1 rulings (PR #40, branch claude/netcode-proof-test-plan-hcky5g, draft)

Seat: day session, advisor agent run as a general agent reading main's `.claude/agents/advisor.md` (this session
started before main's advisor.md last changed). Read-only except this file. Effort: high (netcode work: the caller
asked for rulings, not an ADR).

## Plan
1. Read the sources: resume card (netcode-proof-test.md top), history.md "Decisions taken" of 2026-11-10, advisor-u4.md
   (verdict, conditions, Rulings, U4s, F1/F2), u4-build-plan.md rev 2 (check sha256), plan.md (the sections the rulings
   touch: §4.1, §4.2, §4.4, §4.14, the U8 row, line 1436), u4s-carries.md, the message catalogue.
2. Read wave 1's records and reviews: u4a-build.md, u4b-build.md, u4c-build.md (context only), u4d-build.md,
   u4ab-review-{netcode,security,rust,test}.md, u4d-review-{netcode,rust,arch,test}.md, skeptic S1 (git show), the U4e
   brief draft rev 2 (git show).
3. Spot-check the code where a ruling turns on it (U4d's worktree: the vehicle parts, the press counters, the stress
   scene, D-8/D-9, load_movers; U4a's worktree: SOLDIER, GROUPS, the snapshot header, the public items).
4. Rule, one block per code, writing each ruling below as soon as it is made:
   W1-U4D (High, each Medium, the press-counter wording, the two tuning choices, carry 13), W1-S1, W1-N2, W1-PUB,
   W1-U4E (Q1-Q6, Q8, Q10, Q11), W1-PLAN, W1-TARGET.
5. Return the report: each code with its ruling in one or two lines, the text changes, Decisions needed, For Mustafa.

## Sources read (filled in as I go)
- main's advisor.md (git show origin/main:.claude/agents/advisor.md); the resume card (netcode-proof-test.md:1-120);
  history.md:280-380 (Mustafa's 2026-11-10 words: RESUME-CARD, WIP-CLEAN, TCP-FREEZE, SQUASH-40, PROMOTE,
  SIZE-CHECK, the second lane; his "A" = SQUASH-CITES (a) and NS-START (a); WS-BAR and its refinement; WS-SCOPE).
- advisor-u4.md whole (verdict, conditions 1-13, Rulings, U4s ruling, F1/F2).
- u4-build-plan.md :177-500 and :496-895 (sha256 0d69332c3c60c83c9328504259c279609b546e15cc5510f528346471aabc3b84,
  1,033 lines: the approved rev 2 as recorded after F1); u4s-carries.md (sha256 505e54d6…, the F2 value: unchanged).
- plan.md :568-600 (§4.7), :1236-1248 (§12.1, the U8 row :1247), :1425-1438 (§16; :1436 the never-squash line);
  advisor-u3q.md :120-145 (S13, S14: "A miss goes to the advisor before the WebSocket part of -18 counts").
- Reviews: u4d-review-{netcode,rust,arch,test}.md; u4ab-review-{netcode,security,rust,test}.md. Builder notes:
  u4a-build.md (whole, incl. the fix pass's progress to step 4), u4d-build.md (whole: it has no "For the advisor"
  section, so I ruled U4d's public items from the code's surface, crates/sim/src/lib.rs in its worktree).
- Skeptic S1 at 6652be4 (:20-31); U4e brief draft rev 2 at 0dccbf0, §8 (its 11 codes).
- Code read (worktrees): U4d seam.rs :160-470 (layouts, HitVolumes, InputReports), world.rs :1-60, :235-300, :470-505,
  :570-600, :1025-1050, :1430-1445, :1515-1545, :1635-1650, scene.rs :95-140, lib.rs; U4a lib.rs, snapshot.rs :1-40,
  :380-460, :632-652, quant.rs :25-60, wrap.rs :14-42, tests/bites.rs :40-125, client/src/lib.rs.
- Catalogue :125-150; ADR 0001 :200-212, :222-245, :300-330, :1468-1484, :1607 (PM-0001-58's row), :1611
  (PM-0001-21's row); marks.json PM-0001-2, -7, -58 (read-only parse). u4b-build.md :20-45; U4b's schedule.rs and
  lib.rs public items (.gate/u4b-wt). main's docs: the four cites (git grep origin/main: ADR 0001:432, :1275;
  ws-reconnect.md:22-29, :74).
- Disk: 12 GB free; .gate/shared-target 4.7 GB, the main folder's cache 6.5 GB. flock 2.39.3 present.

## Rulings (one block per code, appended as made)

### W1-S1: the soldier's layout (S1; SEC-U4a-1, with SEC-U4a-2; U4d's engine-architect M2)
**Ruling:** I take the reviewers' fix, add the codec's enforcement of the GROUPS tables (SEC-U4a-2), and **lift the
freeze for exactly this change: it goes into U4d's fix pass now** (the npt-sim half). U4a's half (two GROUPS lines,
the enforcement and a test) is the main session's, in U4d's merge commit, on the tree that already holds U4a. U4a may
merge first without it.

**Why:**
- The leak holds. All four U4a reviewers agree, and so do the test-engineer's probe (velocity: 48 bits in every
  remote soldier record today) and U4d's worktree (seam.rs:208-221 adds WEAPON_STATE: 0 ready, 1 cooling down,
  2 reloading, 3 empty, to SOLDIER). Against them stand catalogue :136-137 (the weapon state is owner only; the remote
  row has neither) and ADR 0001:310-313 (remote soldiers: position, yaw, pitch, health, flag bits, weapon id).
- The freeze (my U4s ruling, condition 3) protected U4a while it built its tables against the same types. U4a's
  build is done, and its code reads `layout()` and `ALL` at run time (snapshot.rs:4-7, :86, :131; common/mod.rs:82,
  :98), so the freeze's reason no longer covers this change. The one U4a test it breaks (snapshot.rs:638-652, every
  kind needs a GROUPS home) is fixed in the same merge commit.
- U4d's fix pass already rewrites the same paths: the soldier's record writer (FLAG_FIRED, the magazine) and the
  records-to-volumes path (H1, A-M1). One pass and one round 2 cover all of it. A main-session commit after U4d's
  merge would need its own review, and its own pin and test churn.
- The fields that leave SOLDIER are exactly what U4g's Predictor needs to reset the own soldier, so
  engine-architect M2's "FIELDS is full" goes away with it.
- Nothing that sends one snapshot to several clients (U4e) exists before U4d merges, so SEC-U4a-1 closes before any
  client could read another's weapon state. That is the security-auditor's own bar ("Nothing sending to >1 client
  (U4e) merges before it").

**The change, exactly:**
- **npt-sim (U4d's fix pass):**
  - Add one new `EntityKind` variant, appended last to `ALL` (the builder names it; I suggest `OwnState`). The seven
    kinds keep their 3-bit codes: KIND_BITS stays 3 with 8 kinds.
  - Its layout holds every per-soldier value that decides that soldier's next ticks and is not in SOLDIER:
    velocity x, y, z (`grid::VELOCITY`; `raw` before the snap and `grid` of it, as carry 9 says), the ammunition, the
    fire timer (`wait`), the reload left, and the move-speed and fire-interval attributes (PM-0001-60's two rows).
    Each is a `Field::integer` whose range holds every value the World can reach, asserted. There is no
    active-reload field: the stand-in has none.
  - The World writes one such record per soldier: index = the slot, generation = the soldier's.
  - SOLDIER becomes position x, y, z, yaw, pitch, flags, health and weapon id (9 fields), the catalogue's remote row
    (:137). "Reloading" stays a flag bit (`FLAG_RELOADING`; ADR 0001:310-311 lists it among the remote flags).
  - `EntityRecord::FIELDS` stays 12.
  - Tests: s6's and carry 12's pins are updated; D-3 compares the new record too; carry 9's per-field off-grid
    counts cover the velocity fields in the new record.
- **npt-proto (the main session, in U4d's merge commit, after U4a has merged):**
  - The new kind joins the "own state" rows of `Snapshot::GROUPS` and `FullState::GROUPS` (2 lines).
  - The encoder and the decoder refuse a record whose kind its section's GROUPS rows do not list (SEC-U4a-2), with
    the existing error kinds.
  - The GROUPS test becomes three checks:
    - every kind has a home in both tables;
    - every owner-only kind (Effect and the new one) sits only in Owner rows;
    - a remote soldier's Add carries exactly the catalogue's remote row: position 16/16/12, yaw 16, pitch 10,
      flags 8, health, weapon id. This is the security-auditor's proof form, and it fails on today's tree by 48 bits.
  - tests/bites.rs's -57 test also sends the owner-only record, so "velocity untouched" still has a velocity field
    to check.
- **The review:**
  - U4d's round 2 reviews the combined diff (U4d's fix pass plus the proto half) in a scratch worktree, before the
    main-folder merge.
  - The security-auditor is added for this part only, so SEC-U4a-1 and SEC-U4a-2 close in writing.
- **u4-build-plan.md, Lanes' freeze bullet (:824-831):** add after its last sentence: "**Lifted for one change (the
  advisor's W1-S1 ruling, 2026-11-10):** U4d's fix pass appends one owner-only kind to `EntityKind` (last in `ALL`),
  holding velocity, the ammunition, the fire timer, the reload left and the move-speed and fire-interval attributes,
  and takes velocity and the weapon state out of SOLDIER's layout. `EntityRecord::FIELDS`, `Command`, `Rewind`,
  `CoverVariant` and the five grid fields stay frozen. U4a's half (the kind in both GROUPS tables' own-state rows, the
  codec's refusal of a kind its section does not list, and its test) is the main session's, in U4d's merge commit,
  after U4a has merged."
- **§4.1's npt-sim row** (:184) gains "the owner-only own-state kind (W1-S1)".
- **The catalogue, at U4d's merge** (with N8's edits): the own-state section carries the owner's soldier record, its
  owner-only record and its effect rows.
- **U4e's brief (rev 3)** states the kind and that the room sends it only in its owner's own-state section.

W1-S1 written 2026-11-10 (the first ruling made, since W1-U4D builds on it).

### W1-U4D: U4d's round 1 (netcode REVISE with one High; test REVISE; rust and arch APPROVE WITH FINDINGS)
**Ruling:** one fix pass now covers the High, every Medium but two, and W1-S1. Round 2 then reviews what changed
(netcode, rust, arch, test, plus security for W1-S1's part). The two Mediums not fixed now:
- the press counters, which I rule on below (the build stands, and the plan's words change);
- the Predictor's reset, carried to U4g with its words.

The review budget holds: this is round 1's fix pass, and round 2 is on what changed.

**Fix in U4d's fix pass now:**
- **H1 (netcode, High): the vehicle parts are written as world transforms that change every tick.** Probe K
  measured 123.5 of 124 part records changed a tick. This is F112's case of "every part's transform changed:
  4 × 20 × 8.5 = 680 B, which with the rest breaks 1,000 B", against ADR 0001:313-314 and catalogue :138 ("parts
  travel with their parent").
  - The fix: a VehiclePart record holds the part's transform relative to its hull, in the hull's frame (the same
    PART fields: offset x, y, z and yaw; health; parent). A rigid part's record then changes only when its health
    changes, and the hull record carries the world transform.
  - Every world box is composed hull ∘ part from the records' grid integers by one function (A-M1 below): the
    authority's movers and volumes, `load_movers`, and later the client.
  - The builder asserts that every relative offset fits its field's range (POSITION_Z has no negative values).
  - Proof:
    - probe K's count re-run: part records changed a tick ≤ the parts whose health changed that tick;
    - a unit test: a hull that moves and turns for N ticks leaves its parts' records unchanged;
    - D-1, D-2 and D-3 still pass.
- **N-M1 (netcode): SS's magazine runs dry inside an S30.**
  - The numbers: 10,000 rounds is 685.7 s of fire. Probe F found 1 soldier dry and 676.31 shots/s in minute 13.
    PM-0001-2's notes say its bite run must pass 10 minutes (an S30), so a clean S30 would drift toward the bite's
    signature.
  - The fix: SS's magazine covers an S30 plus 5 minutes at one soldier's rate: 35 × 60 × 700 / 48 = 30,625 rounds
    (for example 32,000; a u16 holds it). The arithmetic goes in a const assert or a D-6 line.
  - ES gets the same check at its own rate.
- **N-M3 (netcode) = T-M4 (test): FLAG_FIRED.** It is cleared per tick in `step` (world.rs:1821-1824) and per command
  in `predict` (:756), so a catch-up tick mispredicts the flags (probe B). It also changes on every shot, against
  ADR 0001:310-313 ("action starts are tick-stamped cosmetic events"; the flags "change rarely").
  - The fix: drop FLAG_FIRED from the flag bits; a fire is the Shot event.
  - D-3 gains a catch-up tick: two commands in one tick, the first firing. P11's form (predict clears per command,
    step per tick) must fail it.
- **N-M4 (netcode): the vehicles and the stand-in projectiles are not "in the fight".** All 15 vehicles loop 40-82 m
  out (world.rs:587-591), and the projectiles spread ±60 m (:1438-1442), against §4.13 and F113: "the 4 very large
  units in the fight, the other 11 moving elsewhere on the map; 300 live projectiles in the fight".
  - The fix: the 4 very large units loop inside the fight, within the soldiers' cluster plus their own reach, so
    every client's interest set can hold them; the other 11 loop elsewhere; the stand-in projectiles start inside the
    fight's extent. The builder states the numbers.
  - D-6 is re-measured.
  - A carry to U4r (below) reports whether the load really is in view.
- **N-M5 (netcode): the effect rows change every tick.** EFFECT_LEFT counts down (world.rs:1468-1474), so all 384
  records change every tick (probe K).
  - The fix: each row records the tick it ends (or its start tick and duration), so its record changes only when the
    row starts or ends.
  - The per-tick expiry work stays: it is S1's stand-in load.
- **R-M1 (rust) = T-M5 (test): `load_movers` grows its pools to any decoded index.** Its doc (world.rs:770-773) says
  it skips such a record, but the code resizes (:1760-1797). Probe C: cover went 7 → 408 against MAX_COVER 160.
  - The fix: `load_movers` skips a record whose index the world has no entity for, as its doc says, and never grows
    a pool from decoded data.
  - A test on probe C's case.
- **A-M1 (engine-architect): the client cannot build `HitVolumes` from decoded records.** `build_volumes` and its
  geometry constants are private (world.rs:63-66, :1211-1249), so HitVolumes' doc (seam.rs:330-333: "a client can
  build the same volumes from what it decoded") has no path behind it. U4h's AimCast would then copy the sim's
  geometry into npt-client, against U4-HIT.
  - The fix: one public npt-sim function builds `HitVolumes` from records into existing storage (the builder names
    it). The authority's phase 8 calls it on its own records, `load_movers` on decoded ones, and U4h's AimCast later
    on the client's interpolated ones. The geometry constants stay private behind it.
  - A test: in every tick of an SS run, the authority's volumes equal the function applied to its own records.
  - This is H1's one home for hull ∘ part, so it goes in now, not "for later": the plan already makes `ray_cast` take
    the client's movers (§4.2:233-234).
- **T-M1 (test): ES misses two PM-0001-2 bands.** Probe G: live projectiles average 39.91 (band 30.4-33.6) and
  spawns 25.06 a second (band 20.064-22.176). ES also has no source of the "10 reliable gameplay events a second"
  that ADR 0001:1478 and PM-0001-2's ES checks ask for (netcode's ES Low, folded in here).
  - The fix: ES meets every one of PM-0001-2's ES bands on its synthetic recording: 72 shots a second (68.4-75.6),
    10 gameplay events a second from hits and kills (9.5-10.5), 32 live projectiles (30.4-33.6) and 21.12 spawns a
    second (20.064-22.176).
  - Thrown projectiles count toward both projectile bands, and the stand-in spawner fills the rest. Every tick has
    48 soldiers and 15 vehicles.
  - A D-6-like test asserts the exact thresholds.
- **T-M2 (test): the 1 MB cap is 2^20 B.** PM-0001-7 judges MB = 10^6 B (marks.json notes; the F1 ruling uses 10^6).
  - The fix: HitHistory's cap is 1,000,000 B: 1,000,000 / 4,536 = 220 ticks, 7,333 ms at 30 Hz (was 231).
  - history.rs's constant and doc change, and carry 14's edge test moves with them.
  - u4-build-plan.md §4.9 (:336): "at most 1 MB" → "at most 1 MB (10^6 B, as PM-0001-7 judges it)".
- **T-M3 (test): D-9 cannot see a wrong blend.** "By its fraction" holds only to the 6 cm box, so E6's plant (a
  divisor of 255) passes (world.rs:1266; tests/world.rs:811-815, :861-887).
  - The fix: a unit test in world.rs's own test module pins the blended boxes, against an independent reference
    computed in the test, for fractions 0, 1, 127, 128 and 255. E6's plant must fail it.
  - The hit-or-miss check stays as a second test.

**Ruled, with text changes:**
- **N-M2 (netcode) = R-M2 (rust) = A-C (arch): the press counters and my U4 condition 1's wording.** The build keeps
  per slot the press counters of the last command it ran (`claimed`). It finds a press edge as a change from them,
  keeps them outside the hash and the dump, and defines a refused press's twin as "the counter one lower from that
  command on" (world.rs:24-30, :1035-1040, :1645-1647). Condition 1 said "what the server accepted, never what the
  client claimed", and §4.10 says "the forged command with only the refused part removed, everything else equal".
  - The build is right; my words were wrong for counters. A counter carries presses cumulatively. A press removed at
    one command but kept at the next only moves one tick later (the twin would then fire where the forged run does
    not). So "everything else equal" cannot hold for a press, and "accepted" counters would turn a refused press
    into a queued one.
  - `claimed` is input bookkeeping that is a function of the command stream alone: it is the presses of the last
    command run. A refused press's edge is consumed there and changes nothing hashed, which is condition 1's
    purpose.
  - Two things follow:
    1. The twin must know exactly which presses were refused. So `InputReports` gains this tick's refusals as
       records `{ slot, seq, kind }` (kind: cooldown, ammunition, reload, Rewind outside the ring, pool full), cleared
       each step like the clamp reports, beside the running counters, outside the hash and the dump (carry 38
       holds). D-10 asserts the refused command's record. This goes in U4d's fix pass now.
    2. The predicting World must restore `claimed` after a correction (rust M2): carried to U4g (A-M2 below).
  - Text, u4-build-plan.md §4.2 (:240-242): replace "Any per-soldier input state the sim keeps holds what the server
    accepted, never what the client claimed, so a refused part changes nothing on the ticks after it either (the
    press counters are the case to watch: D-10 runs past the next accepted press)." with: "The sim's one piece of
    per-slot input bookkeeping is the press counters of the last command it ran for that slot: a press edge is a
    counter that differs from them, and a refused press's edge is consumed there and changes nothing hashed. They
    stay outside `state_hash` and `StateDump` (a twin's counters differ by design); they are a function of the
    command stream alone, and the predicting World restores them from the client's own command at the snapshot's
    last-run sequence (U4g). `reports()` also lists each refusal per command (`{ slot, seq, kind }`, beside the clamp
    reports), so a twin removes exactly the refused parts. D-10 runs past the next accepted press. (The advisor's
    W1-U4D ruling: it replaces condition 1's 'what the server accepted, never what the client claimed' for the press
    counters.)"
  - Text, §4.10 (:356-358), appended to its bullet: "For a refused press the refused part is the press itself: the
    twin's counter for that button is one lower from the refused command on, every other field equal, and the twin
    removes exactly the presses the forged run's reports list (the advisor's W1-U4D ruling)."
  - Text, advisor-u4.md: the main session adds one line under my U4 verdict, as it did for NF-D4: "Condition 1's
    'Any per-soldier input state the sim keeps holds what the server accepted, not what the client claimed' is
    amended by the W1-U4D ruling (advisor-u4-wave1.md) for the press counters."
  - New carry (U4f): "-21's press twins: the twin removes exactly the presses the forged run's `reports()` list as
    refused (slot, seq, kind) and lowers that button's counter by one from the refused command on, every other field
    equal; a test shows a forged press stream and its twin give equal state hashes on every tick (D-10's rule, at the
    room's level)."
  - A reading, for the readings PR (§10's list and §11), beside Q16's clamp: "PM-0001-21's 'a run with only their
    valid parts' (ADR 0001:1611), for a refused press, is the same stream with that press removed: the button's
    counter one lower from the refused command on (W1-U4D)". It lowers no bar: the hashes must still be equal on
    every tick.
- **A-M2 (engine-architect): no way to reset the predicting World's own soldier.** Carried to U4g. W1-S1 supplies
  the data now.
  - New carry (U4g): "The predicting World's reset: an npt-sim entry that sets one slot's soldier from the decoded
    own-state records (its SOLDIER record and its owner-only record: W1-S1) and the press counters of the client's
    own command at the snapshot's last-run sequence, exactly as the authority held them after that command. A test:
    reset from the authority's records at tick T, then predict N commands, equals the authority bit for bit (D-3's
    rule from a reset, not from a clone). U4g edits crates/sim for it; no lane holds it then (the F2 ruling,
    point 6)."
- **TUNE-1: SS's health of 1,000, and no reload presses in SS and ES.**
  - Approved as stand-in tuning. It keeps D-6's shot mean inside 665-735: 542.15 shots/s with reloads and lower
    health, 683.62 after. The stress scene is a load; PM-0001-2 sets no death rate; §4.13 asks for the rate "with
    deaths counted".
  - Conditions:
    - each run's header records the weapon (interval, damage, magazine, reload) and the health;
    - U4d's report states the measured kill rate (probe F: 329 kills in 13 min, about 0.42 a second) as a property
      of the load: respawns, and so generation changes, are rare in SS;
    - N-M1's magazine makes "no reloads" hold for a whole S30.
- **TUNE-2: D-8's "at the reach" only inside the grid's z range.**
  - Approved as a stated bound, not a defect. An aim point that came over the wire is inside the grid's range (the
    aim travels on the position grid), so the reach point leaves that range only when the fire origin itself is near
    or above the grid's top. Probe A: idempotence breaks for feet above about 40.57 m. Rust L4 and test L3 agree.
  - Conditions:
    - U4d states the measured bound in World's doc and in D-8, and asserts that no scene's spawn points or greybox
      surfaces come near it;
    - a scripted teleport above it is a scenario error, which U4r's runner refuses (carry);
    - the readings PR's Q16 text (the clamp twin's points at the reach) adds "for fire origins inside the grid's
      range, as in every scene".
  - D-8's text in §5 U4d (:612-613) gains: "for every fire origin below the bound World's doc states (no scene places
    one above it); above it a reach point saturates in z and is not claimed to lie at the reach".
- **C13: carry 13 is met only in part.** A counting allocator cannot compile under `unsafe_code = "forbid"` (a
  `GlobalAlloc` impl is `unsafe impl`).
  - Accepted at U4 with the storage-identity test: each vector's pointer and capacity are unchanged across a full
    ring's pushes. HitVolumes owns no other heap storage, so that test shows `clone_from` allocates nothing.
  - The counting half moves to U5 with PM-0001-34's counting allocator, as catalogue item 3's allocation half already
    did (Q8). New carry (U5): "Carry 13's second half: with PM-0001-34's counting allocator, a test that a push into a
    full `HitHistory` allocates nothing (u4s-carries.md entry 13; W1-U4D)." U5's plan states where its allocator's
    `unsafe` lives under ADR 0002:264-265.
  - U4d's report lists carry 13 as "met in part, the rest carried to U5 (W1-U4D)".

**The Lows:**
- Two Lows ride this fix pass, because it changes their lines anyway:
  - rust N4: `record()` debug-asserts each integer within its Field's range. The pass adds integer fields, and SS's
    health of 1,000 sits near HEALTH's 1,023.
  - rust L1 = test L5: `Recording::decode` caps its pre-allocation by the bytes present (replay.rs:320).
- Three more ride at the builder's choice:
  - rust L2 = test L4: `Replayer` returns `Step::Failed` on a non-canonical recording;
  - test L1 = rust L8: D-4's finite check also reads the pre-snap values;
  - rust N5 / netcode's clamp Low: the clamp runs only inside the shot phase, as §4.2 says.
- One nit rides: `Recording` keeps one public path (W1-PUB).
- Every Low not fixed is a named follow-up in U4d's report, with its reviewer's id.

**New carries, besides those above:**
- **U4r:** "The contract test's report gives, for each SS suite, the share of snapshots whose client set holds all 4
  very large units, and the mean number of projectiles per client set (reported, not judged: W1-U4D N-M4). The
  runner refuses a scripted spawn or teleport above World's stated clamp bound (W1-U4D TUNE-2)."
- **plan text:**
  - §4.13's SS bullet (:415-420) gains, after "DM_Tri's extent": "the 4 very large units loop inside the fight and
    the other 11 elsewhere, and the stand-in projectiles start inside it; parts are recorded relative to their hull;
    effect rows record their end tick; a hit takes 1 of 1,000 health and SS and ES script no reloads, with a magazine
    that covers an S30 plus 5 minutes at one soldier's rate (the advisor's W1-U4D ruling)".
  - §4.15 (:478), "a seeded generator in npt-sim's test support" → "a seeded generator in npt-sim's library
    (`Recording::synthetic`), since U4d2's step regenerates the recordings from the manifest" (engine-architect F).

W1-U4D written 2026-11-10.

### W1-N2: U4a's snapshot header carries command sequences, not tick offsets
**Ruling: APPROVE the plan text edit (my condition 12), §4.14's -58/lt-tick-offset row included.** U4a's header
(snapshot.rs:9-16, :392-395) carries the full 32-bit tick, the baseline as its 16-bit age, the newest command received
and the last command run as 16-bit command sequences (each with a known flag), and the group index and count.

**Why:**
- ADR 0001:228-230 has each snapshot "acknowledge two commands": the newest received, which ends its repeats, and the
  last run, where the replay starts. A sequence names a command exactly. The tick a command ran at need not be its
  stamped tick: the input buffer delays it, and OQ-0001-6 (b) repeats the last command on a miss.
- The scout's own estimate already used sequences (net F112: "newest-received seq 2, last-run seq 2"). §4.4's
  "16-bit offsets (baseline, newest command received, last command run)" was a drafting slip.
- ADR 0001 part 6's "other ticks are 16-bit offsets" still holds for the ticks the header does carry: the baseline's
  age, and the event age in the body.
- Nothing gets looser. -58 keeps its six bites and its six wrapping counters.

**Text changes (u4-build-plan.md):**
- §4.4 (:273-274): replace the bullet with: "The snapshot header: the full 32-bit server tick; the baseline as its
  16-bit age in ticks (0: coded against the per-type defaults); the newest command received and the last command run
  as 16-bit command sequences, each with a known flag (ADR 0001:228-230 acknowledges two commands; a sequence names
  the command, while the tick it ran at can differ from its stamp; net F112 proposed them; the advisor's W1-N2
  ruling); the group index **and the group count** (so the client knows when a tick is complete: §4.6)."
- §4.14, the -58/lt-tick-offset row (:454): replace with "| -58/lt-tick-offset | the client's rebuild of the
  header's 16-bit 'last command run' sequence, where its replay starts (a plain `<` in place of its wrap-aware
  compare): U4e builds the rebuild, U4g's Predictor starts its replay there (the advisor's W1-N2 ruling; U4E-Q2) |
  0 untraced corrections | command sequences crossing the wrap (as -58/lt-cmd-seq: a few steps below it) | U4e: the
  rebuilt sequence names the wrong command; U4g: the replay starts at the wrong command |".
- §5 U4r (:737): delete "-58 for 10 s at P0 and P3, every counter a few steps below its wrap; its six bites (§4.14)."
- §5 U4g, Marks moved (:791-795): add "-58 (10 s at P0 and P3, every counter a few steps below its wrap, 3 seeds per
  part; its six bites, §4.14), judged after U4g merges: its untraced-corrections check and lt-tick-offset's failing
  check read the Predictor (the advisor's W1-N2 ruling and U4E-Q2)."
- §10's readings preamble (:927-928) and §11 (:1028-1033): the readings list gains "PM-0001-58's 'the snapshot's
  tick offsets' (ADR 0001:1607) read as the snapshot header's two command acknowledgements (16-bit sequences); the
  remaining tick offsets, the baseline's age (at most 31: §4.5's ring) and the event age (8 bits), never cross a
  wrap, so they get no bite (W1-N2)".

**Elsewhere:**
- marks.json, `PM-0001-58/lt-tick-offset`: its "what" becomes "the snapshot header's command acknowledgements
  (16-bit sequences) compared with a plain <", and the id stays. The wrap check's name ("... tick offsets ...") says
  "the header's command acknowledgements" in place of "tick offsets". The client logs that counter's wrap as its own
  `event.counter_wrap` (U4e), so the check's count of 6 stays honest. U4j's brief (u4j-brief.md) carries both lines;
  U4j owns marks/ in wave 1.
- The catalogue :135: in N8's edits at U4a's merge (the header's fields as built).
- New carry (U4e): "An event older than 255 ticks cannot be coded: the event section's age is 8 bits (snapshot.rs's
  wire text). The room's EventQueue never queues one past that age: the overflow resync (§4.7) ends it first, or the
  room drops it from the resend and counts it. A unit test."

W1-N2 written 2026-11-10.

### W1-PUB: the public items beyond §4.1's lists (A19)
**Ruling: APPROVE all three lists, and remove nothing.** Two nits are below, and §4.1's table is to be brought up to
date. Every item is one of three things:
- a type in the signature or a field of an item §4.1 or plan.md §4.7 names;
- the field-group layout table that §5 U4a itself lists;
- an item a planned test needs (the catalogue audit, carry 6's read-back).

None is a "for later" item.

- **U4a (npt-proto; its lib.rs:32-45, checked against the rust-reviewer's A19 list):**
  - `Message`, `Direction`, `DecodeError` and `EncodeError`: in `encode`'s and `decode`'s signatures.
  - The message structs, `EndReason`, `ProfileId`, `StepResult` and `TokenKind`: fields of the listed messages and
    controls.
  - `Limits`' constants and `Limits::TYPES`: `Limits` is listed, and catalogue items 2 and 3 test against `TYPES`.
  - `Quant::encode`, `Quant::decode` and the bites-only `Quant::with_bite` and `BITES`: the per-instance bites
    (§4.11).
  - `Wrap::after8`, `after16`, `after32` and `widen16`: §4.4's one compare per width, and the rebuild. `plain_lt` is
    bites-only.
  - `EntityDelta`'s `between` and `apply`: the `EntityRecord` mapping (§4.4).
  - `Class`, `Snapshot::GROUPS` and `FullState::GROUPS`: §5 U4a's "field-group layout table". W1-S1 makes the codec
    enforce them, so they become operative.
  - `Message::name`, `SessionControl::VARIANTS` and `variant`, `RoomControl::NAMES` and `name`: catalogue item 6's
    audit (approved, Q7).
  - Nit (rust F-A8, a follow-up): `Limits::GROUPS` and `Snapshot::GROUPS` share a name with two meanings. Rename the
    limit at the next edit of limits.rs.
- **U4b (npt-server's schedule.rs):**
  - `Dropped` and `LoopCounters`: types inside `Due` and `counters`.
  - `next_due_ns`: U6's timer, and the tests' "no tick owed" check.
  - `counters`: ADR 0004 part 5's monitoring.
  - `tick_hz` and `catchup`: carry 6's read-back.
  - Bites builds only: `Schedule::with_bite` and `BITES` (PM-0004-67's three, schedule.rs:251; lib.rs:13).
  - These match u4b-build.md:31-42 and schedule.rs:22-119 and :251-257.
  - Nit (a follow-up): each item has two paths (`npt_server::schedule::X` and `npt_server::X`: lib.rs:10, :13-14).
    Keep one at U4e's next edit of server/src/lib.rs.
- **U4d (npt-sim).** u4d-build.md holds no "For the advisor" section, so I ruled on the surface its lib.rs exports
  (worktree crates/sim/src/lib.rs:25-55, and each module's `pub` items):
  - Plain-data types of named items (plan.md §4.7: "with its plain-data input and output types"):
    - `Placement`, `AttributeChange` and `Attribute`: `TickInput`'s fields;
    - `ClampReport`: `InputReports`' field;
    - `VolumeBox`: `HitVolumes`' field;
    - `FieldDiff`: `StateDump::first_difference`'s result (RQ5's field diff);
    - `Ray` and `RayHit`: `ray_cast`'s argument and result;
    - `replay::Step`: `Replayer::step`'s result.
  - Named in plan.md §4.7: `Greybox`, `Areas`, `Portals`, `StandIns` and `trajectory_at`. Engine-architect F is
    right that the builder's gap list overstated this.
  - The constructors and accessors the room, lab and Wasm wrapper need:
    - `World::new`, `scene`, `greybox`, `next_tick` and `dump`;
    - `Scene::by_id`, `id`, `name`, `slots`, `team`, `stand_ins` and `with_stand_ins` (the header's scene id; the
      stand-ins' off switch on L1: §4.13);
    - `StandIns::OFF` and `s1`;
    - `HitVolumes::MAX_SOLDIERS`, `MAX_COVER` and `MAX_BYTES`;
    - `Recording::GENERATOR`, `encode`, `decode` and `synthetic`;
    - `Replayer`'s methods;
    - carry 17's button and press constants.
  - Bites-only: `World::BITES`, `with_bite` and `bite_order_digest`.
  - `Recording::synthetic` belongs in the library: U4d2 regenerates the recordings from the manifest (§4.15's text
    changes: W1-U4D).
  - `Greybox::dm_tri` stays public: the static graph without a World, for the room's AreaFlood. U4e's report says
    which route the room takes.
  - Nit (rides U4d's fix pass): `Recording` has two paths (lib.rs:17's `pub mod replay` and :28's re-export). Keep
    one.
  - `SimRng` stays crate-private, narrower than plan.md §4.7: no other crate draws from the sim's streams (§4.5; the
    spread stream is authority-only). Record it in U4d's report, and plan.md §4.7's npt-sim line at its next edit:
    "`SimRng` (crate-private: no other crate draws from the sim's streams; the advisor's W1-PUB ruling)".
- **§4.1's table (main session; text):**
  - npt-proto's rows gain one row: "`Message`, `Direction`, `DecodeError`, `EncodeError`; the message and control
    types' field types (`EndReason`, `ProfileId`, `StepResult`, `TokenKind`); `Limits`' constants and `TYPES`; `Quant`'s
    `encode` and `decode`; `Wrap`'s compares and `widen16`; `EntityDelta`; `Class` and the two `GROUPS` tables; the
    catalogue audit's name and variant helpers; bites builds only: `BITES`, `Quant::with_bite`, `Wrap::plain_lt` |
    the signatures and fields of §4.7's items; §5 U4a's layout table; catalogue items 2, 3 and 6 (the advisor's
    W1-PUB ruling)".
  - npt-server's row gains "`Dropped`, `LoopCounters`, `next_due_ns`, `counters`, `tick_hz`, `catchup`; bites builds
    only: `Schedule::with_bite`, `BITES`".
  - npt-sim's row gains:
    - "the plain-data types `Placement`, `AttributeChange`, `Attribute`, `ClampReport`, `VolumeBox`, `FieldDiff`, `Ray`,
      `RayHit`, `replay::Step`";
    - the owner-only own-state kind (W1-S1);
    - the records-to-volumes function (W1-U4D A-M1);
    - `InputReports`' refusal records (W1-U4D N-M2);
    - `Scene`'s and `World`'s accessors as listed above.
  - npt-room's row gains "bites builds only: `BITES`" (U4E-Q3).

W1-PUB written 2026-11-10.

### W1-U4E: the U4e brief's questions (draft rev 2 at 0dccbf0, §8)
The brief can be finished after wave 1 merges on these rulings. Its rev 3 also carries W1-S1 (the owner-only kind,
sent only in its owner's own-state section), W1-N2 (the header, and U4e's event-age carry) and W1-U4D's carries
that name U4e.
- **U4E-Q1, confirmed: (a).** U4e plants and unit-shows PM-0001-58/lt-cmd-seq and lt-press: post-wrap commands
  refused, and an edge lost.
  - Why: §4.14:438 plants each bite where its code lives, and U4e builds the room's sequence-forward and press-edge
    compares (:691). The Bites line (:711-712) left them out by a slip.
  - Text: §5 U4e's Bites line gains "lt-cmd-seq and lt-press (the room's compares)".
- **U4E-Q2: (c), widened.** U4e builds the client's rebuild of the header's 16-bit last-run sequence (W1-N2) and
  unit-shows the bite as "the rebuilt sequence names the wrong command". U4g's Predictor starts its replay there and
  shows "the replay starts at the wrong command".
  - The widening: all of -58 is judged after U4g, not only this bite. Its check "0 corrections larger than 1 mm not
    traced" reads the Predictor's count, which does not exist before U4g. Judged at U4r, -58 would show a gap, or
    worse, a vacuous 0.
  - Text: W1-N2's §5 U4r and §5 U4g edits.
- **U4E-Q3: (a).** npt-room gains a bites-only `BITES` list, as npt-client has. npt-server already has one: U4b added
  PM-0004-67's three ids (schedule.rs:251, re-exported at lib.rs:13), and U4e adds its pump's ids to that list rather
  than making a second. U4e's report names them as beyond §4.1 under this ruling, and §4.1's rows gain them (W1-PUB).
  - Why: U4r's `--bite` dispatch and check.mjs's bite-home grep need one findable home for each id, and a bites-only
    item never reaches a measured build.
- **U4E-Q4: (a).** PM-0001-17/tick16 and PM-0001-58/tick16 stay in `npt_proto::BITES`, which is where U4a put them
  (quant.rs:34-42). The client's endpoint takes its `Quant` from `npt_proto::Quant::with_bite`, as it takes its grid
  from `grid_with_bite`, so each literal lives once.
  - Why not (c): it would add ids marks.json does not spell, for nothing a run needs.
  - Carry 24's text changes: "... `npt_client::BITES` grows with U4e's other client-site ids (-58/lt-tick-offset
    and -58/lt-event-seq, the client's rebuilds); -17/tick16 and -58/tick16 stay in `npt_proto::BITES`, the endpoint taking its `Quant` from
    `npt_proto::Quant::with_bite` (U4E-Q4); -45's ids and -62/cues-from-replay join npt-client in U4g, whose code
    holds their sites (U4E-Q10); no id appears in two crates' lists."
  - U4e then reports carry 24 as met under these words.
- **U4E-Q5: (a), plus one condition.** With a claimed view time whose tick the ring does not hold, `rewind_at` gives
  the command no Rewind and the room counts it. A refused Rewind runs as none in the sim, so this is one rule. U4h
  then replaces it with the tolerance and the cap.
  - The condition: that counter counts as a replacement for -30's setup condition. A pre-check leg with any is void
    (my condition 3).
  - Text: U4r's brief and carry 40 name the counter.
- **U4E-Q6: (a), plus two conditions.** U4e builds the cap as data, with a provisional value from net F121's
  arithmetic, recorded as provisional (1,024 is F121's example, above the about 400 a B1 burst adds). U4f measures
  the rate and records §4.7's value before -22's runs.
  - Condition 1: U4r's contract test logs each client's relevant gameplay-event rate and its queue's high-water mark
    per burst.
  - Condition 2: an overflow resync in any judged -17 or -18 run is reported, and those runs come to me before they
    count. F121 warns that a cap set too low resyncs every client on every burst and changes -18's heal path.
- **U4E-Q7: answered, confirmed.** The plan gives the series' writer (§4.12:397-398; U4r :719-720; `hs` is U4j's,
  :627). No ruling needed.
- **U4E-Q8: (a).** U4e adds the HashMap and HashSet type bans to crates/client/clippy.toml. It is §4.11's text and
  U4e's lane, and the client hash depends on iteration order.
  - Text: §5 U4e (:692) "room/clippy.toml gains the hash-map type bans" → "room's and client's clippy.toml gain the
    hash-map type bans (§4.11; U4E-Q8)".
- **U4E-Q9: answered, confirmed.** The F2 ruling covers it: the rate is passed as an argument, with a pin at 30 and
  60 Hz. No ruling needed.
- **U4E-Q10: (a).** U4e plants PM-0001-62/server-echo in npt-room, its event routing, with its unit proof ("one event
  sent"). -45's ids and -62/cues-from-replay join npt-client in U4g, which builds `Interp` and `CueLog`.
  - Why: an id with no site is a bite that cannot bite (risk 3).
  - Text: carry 24 as under Q4; §5 U4e's Bites line gains "server-echo (-62, the room's routing)".
- **U4E-Q11: (a).** U4e builds the server-side checks (size, trailing bytes, wrong channel, the bucket) and their
  unit tests, as its §5 says. U4f adds their -21 counters and the judged runs on U4e's pump.
  - Text: §5 U4f's Builds (:775) "the server-side checks (size, trailing bytes, wrong channel, the bucket)" → "the
    server-side checks' -21 counters and judged runs on U4e's pump (U4e builds and unit-tests the checks: U4E-Q11)".

W1-U4E written 2026-11-10.

### W1-PLAN: the plan text Mustafa's words of 2026-11-10 require
**Ruling: both edits confirmed, in the words below.**
- One interaction between TCP-FREEZE and his WS-BAR rule is his to settle. It is NETEM-BAR, under Decisions needed.
  It blocks nothing before R1.
- I see no other conflict.

**SQUASH-CITES (a), plan.md:1436-1438.** Replace the bullet "**PR #40, this test's own PR, merges with a merge
commit, never squashed,** ... (the advisor's finding 4 on #45)." with: "**PR #40, this test's own PR, merges by
squash** (Mustafa's SQUASH-40, 2026-11-10: its history carries about 28 MB, most of it checkpoints), with no
`.checkpoints/` in the final tree. Before the merge, the closing commit re-points every citation of the commits
bf9d248, 975e72b, 95a04a1 and 1e020a7 in ADR 0001 and docs/design/ws-reconnect.md (the advisor's finding 4 on #45)
to files in the final tree, and the advisor checks each re-pointed citation against the file and lines it names
(Mustafa's SQUASH-CITES (a), 2026-11-10). A citation of a `.checkpoints/` file is re-pointed to a copy of the cited
words in a file the final tree keeps (the closing report's appendix), since no `.checkpoints/` file survives."
- Why the last sentence: main's ws-reconnect.md cites the checkpoint at 95a04a1 (:26, :74), and the squash deletes
  every `.checkpoints/` file. ADR 0001's own cites (:432 "commit bf9d248"; :1275 "printed at commit 1e020a7 and
  unchanged at bf9d248") can point at results/u3-ws-fallback/bar-fix1-bf9d248.txt and tables-1e020a7.txt, which 975e72b
  committed under spikes/netcode-proof/ and the final tree keeps.
- His decision replaces my "never squashed" (finding 4 on #45). My reason, evidence that stays reachable, is kept by
  the re-pointing, so there is no conflict.
- The re-pointing edits an ADR's citations only: no word or decision of his changes. PR #40 is needs-mustafa anyway.

**TCP-FREEZE, plan.md:1247 (the U8 row).** After "...since this container has no tc", insert: "; if that check
disagrees with the model (S14: the kernel's stall or burst-recovery p50 or p99 more than 10 % from the model's, at zero
jitter), netem wins (Mustafa's TCP-FREEZE, 2026-11-10): the WebSocket rows are then judged on VM runs with netem in
place of DelayStream's model, the model is not repaired, and the disagreement no longer comes to the advisor for a
remedy (this replaces S14's 'A miss goes to the advisor before the WebSocket part of -18 counts', advisor-u3q.md:143-144);
DelayStream's loaded self-test below then no longer gates -18's and -31's WebSocket parts, which no longer run through
it; R1's report records the comparison and the switch".
- §15's R-25 row (:1417): after "against RS's at R1 (S14)", add "; if they disagree, netem wins and the model is not
  repaired (TCP-FREEZE)".
- The self-test clause follows from his words: rows judged on netem runs do not go through DelayStream. The netem
  runs replace DelayStream's runs one for one. Their setup, and any extra RS hours, go into R1's and R3's run plans
  for his OK, as every run's cost already does.

**The interaction (NETEM-BAR).** WS-BAR (2026-11-07) sets the WebSocket bar's numbers from one practice run "but never
looser than the model predicts for a correct build with fix 1", and his refinement says "beyond 10 %, investigate
first and never loosen".
- The case: netem shows the real kernel more than 10 % slower than the model.
- Read literally, the two rules together judge the WebSocket rows on netem runs, against a bar that may not loosen. A
  correct build could then fail -18's or -31's WebSocket part because of the kernel, not the build. "Netem wins"
  could also be read as letting netem set the numbers, which would loosen the bar past the model, against WS-BAR.
- Only he can say which reading he meant.
- My recommendation: keep the bar (never loosen), report any such failure with S14's numbers as the kernel's and not
  the build's, and bring it to him then.
- Nothing before R1 depends on it.

**"Tell me if any later unit plans work on the model" (TCP-FREEZE's last sentence), for the main session to relay.**
No later unit plans fidelity work on the TCP model. Two units plan work on what runs it:
- U6: WsServer's send path with fix 1. On L2 its gate reads DelayStream's inputs; U3c's writer properties are re-run;
  and there is the reader stamp (plan.md:1245).
- U8/R1: DelayStream's loaded self-test and its fallback order (the reader wake, then the pumps off tokio:
  plan.md:1247).

Both concern the wrapper's timing, not the model's TCP rules, so I read them as outside the freeze. If netem wins at
R1, the second no longer gates any row (the edit above).

W1-PLAN written 2026-11-10.

### W1-TARGET: the shared Cargo target (a note; one rule for wave 2's briefs)
- **What wave 1 showed.** Cargo keys a workspace member's artifacts without its folder and keeps dep-info paths
  package-relative (u4d-build.md, step 7). So a copy that builds after another copy's touch makes the other copy's
  stale sources look fresh, and the lanes ran each other's crates.
  - This happened at least five times: U4a 22:13, 22:14 and 22:55; U4d 06:35; and the test-engineers' runs A in
    u4ab-review-test.md and u4d-review-test.md.
  - Every time, the count check or a compile error caught it, and the run was not reported.
  - The 2 s touch loop narrows the window but does not close it: another lane can still build between a touch and
    this lane's fingerprint check.
- **The gap wave 2 opens.** Fix passes and re-runs often leave the test count unchanged (U4d's round 2 will). There
  the count check cannot tell this copy's code from another copy's.
- **What wave 2's briefs add** (agent briefs and notes, no kit edit, within Mustafa's one-folder rule):
  1. **A lock.** Every cargo, rustc or rustdoc call in every lane runs under `flock` on one file,
     `.gate/shared-target/.lane.lock`: shared (`flock -s`) for ordinary builds, exclusive (`flock -x`) for a counted
     run (the touch, check.mjs, the `--list` and clippy confirmations) and for each bite run. No other lane can then
     build inside a counted run. flock 2.39.3 is present.
  2. **Provenance.** A counted run's log must show "Compiling <crate> (<this copy's path>)" for every crate the lane
     changed, in each cargo invocation that builds it. A missing line means "not counted", whatever the count says.
  3. **Disk.** 12 GB is free, with the shared target at 4.7 GB. U4d2's wasm32 release build adds a target triple.
     Each lane checks for at least 4 GB free before a wasm32 or release build (the 2 GB floor stays for the rest).
     Between waves, with no lane running, the main session may empty the shared folder by `cargo clean --target-dir
     .gate/shared-target`, an approved method (LEFTOVER-DELETE). §7.1 item 7 ("No copy cleans the shared folder")
     binds the copies, not the main session.
  4. **After the merges.** The main session removes `.gate/u4a-prefix` and `.gate/u4b-prefix` (the resume card
     already says so). Wave 1's worktrees go by the standing procedure (history.md, 2026-11-08: leftovers on a local
     branch, then a plain `git worktree remove`).
- Nothing here needs Mustafa: it keeps his one-folder rule as written.

W1-TARGET written 2026-11-10.

## What I checked, and what I did not
- **Checked, by reading:**
  - the reviews against the code at the lines they cite: U4d's seam.rs layouts and SOLDIER, world.rs's `claimed`,
    FLAG_FIRED, the vehicle loops, the stand-in spread and the record writer; U4a's header, GROUPS and its test, the
    bites list, the aim's coding;
  - the ADR and catalogue rows behind each ruling;
  - PM-0001-2's, -7's and -58's marks;
  - the plan lines each text change replaces;
  - the sizes: U4d is about 5,600 new lines plus 325 changed, U4a about 5,000 plus 833;
  - the arithmetic: 35 × 60 × 700 / 48 = 30,625; 1,000,000 / 4,536 = 220.46, so 220 ticks, 7.33 s at 30 Hz.
- **Not checked:**
  - I built, ran and planted nothing (read-only). The probe results (B, C, F, G, K, A) and every count are the
    reviewers'.
  - I did not read U4a's fix pass beyond its notes (it is in flight), U4c's reviews, or u4j-brief.md's text, which
    W1-N2 adds two lines to.
  - Whether the moved vehicles keep D-6's bands, and whether ES can reach 10 gameplay events a second with its
    weapon, are the builder's to measure. A band that cannot be met comes back to me before U4d merges.

## Decisions needed (Mustafa)
- **NETEM-BAR: when netem wins, may the WebSocket bar loosen?**
  - The question: if U8's netem check on RS shows the real Linux kernel more than 10 % slower than the TCP model,
    TCP-FREEZE has the WebSocket rows judged on netem runs. Does WS-BAR's "never looser than the model predicts...
    beyond 10 %, investigate first and never loosen" still cap the bar's numbers?
  - Why it matters: if it does, a correct build can fail -18's or -31's WebSocket part because of the kernel, not
    the code.
  - Options:
    - (a) the bar stays capped and never loosens; such a failure is reported with S14's numbers as the kernel's, and
      comes back to you;
    - (b) netem's practice run sets the numbers, even past the model (this changes WS-BAR);
    - (c) decide only if it happens, at R1.
  - Recommendation: (a). It is both rules read literally, it keeps "never loosen", and S14's comparison is the
    investigation WS-BAR asks for. Nothing before R1 depends on it.

## For Mustafa
1. Nothing to do now. Optionally, answer NETEM-BAR in a session's chat (it does not block wave 1 or U4).
2. Later, as planned: your go on the readings PR before R5. It now also carries three readings:
   - PM-0001-58's "tick offsets" (W1-N2);
   - PM-0001-21's press twin (W1-U4D);
   - the clamp reading's grid-range clause (TUNE-2).

**Errata (the advisor, 2026-11-11; advisor-u4d-close.md, B1.4 and B3):** W1-S1's "(9 fields)" (:79) reads "(8
fields)": the list it heads names 8, as do the catalogue's remote row and SOLDIER as built. W1-U4D's two quotes of
condition 1 (:200 and :224) read "what the server accepted, not what the client claimed" (advisor-u4.md:86). W1-PLAN's
cite "advisor-u3q.md:143-144" (:525) reads ":142-143". W1-PUB's "§4.7's items" (:422) reads "plan §4.7's items". No
ruling changes.
