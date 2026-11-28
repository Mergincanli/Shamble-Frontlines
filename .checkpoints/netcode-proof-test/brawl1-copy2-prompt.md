You are BRAWL-1's builder for the netcode proof test (PR #40, the spike branch claude/netcode-proof-test-plan-hcky5g): one builder, alone, in your own worktree. Your instructions are the brief /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/brawl1-brief.md (sha256 921c3243958a09dd5f914fb7eb247f07333558d0905b3d849d2f13222d6d2a50, 261 lines). Read it whole before anything else, check its sha256, then start at its section 1.

- **Your worktree** was made for you by the main session, because isolated launches gave worktrees based on main: /home/user/Shamble-Frontlines/.claude/worktrees/brawl1, detached at `<base>`. Work only there: run git and your commands as `cd /home/user/Shamble-Frontlines/.claude/worktrees/brawl1 && ...`, and edit files by their absolute paths in it. Never edit the main folder's spikes/ (/home/user/Shamble-Frontlines/spikes/).
- `<base>` (the brief's section 1 item 1) is 352a245c3018c50843f62dcb90e91973ea56d9da: the worktree's HEAD must equal it, with an empty `git status --short` (its spikes/netcode-proof tree is abad063555b0105e2afd47dcff96acd7bd74f63e). If it does not, stop at once, change nothing, and report what you found.
- Your notes: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/brawl1-build.md (in the main folder). If it exists, you are a later copy: read it whole and the diff in the worktree before you change anything, and continue after its last complete block.
- The runner: u4r-run.sh with this worktree's path as its first argument (it accepts worktrees under .claude/worktrees/), logs under /home/user/Shamble-Frontlines/.gate/u4r/brawl1/.
- T4 item 1, word for word: "Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes; never `cargo fmt` writing files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole with Write."
- Git runs only in the worktree, as `cd <worktree> && git ...`. Never `git -C`, `git -c`, `git checkout` or `git restore`; the guard refuses them. Commit nothing and push nothing; the main session merges.
- Cuts: the session's spend limit may cut you mid-run. Save as you go; a fresh copy continues from your notes and the diff.
- Your report is the brief's section 6 list, in its order, written at the end of your notes under "## Report" and then sent as your final message. If you stop early by the brief's stop rules or at your context limit, write the "## State" block first, then report that you stopped and why.

**Resume: copy 2, the fix pass of review round 1 (the main session, 2026-11-15 about 06:45 UTC).** You are a fresh copy. Copy 1 built BRAWL-1 and reported at about 04:10 UTC; its report is the end of your notes. Review round 1 (four seats) then reviewed the build's snapshot. The advisor ruled on copy 1's points (advisor-brawl1-points-full.md: no diff change) and on the round (advisor-brawl1-r1-full.md, sha256 86bec63bfffbd676efbc09e09c35a952e63abc7865e89b20b5aebec11bee75c5). Nothing is committed. Where this part and the text above differ, this part wins for you; where the advisor's ruling gives exact words, they win over both.
- **Your worktree already exists:** /home/user/Shamble-Frontlines/.claude/worktrees/brawl1 (detached HEAD 352a245c3018c50843f62dcb90e91973ea56d9da). Its uncommitted work is the reviewed snapshot, the main folder's .checkpoints/netcode-proof-test/wip/brawl1.patch (10 files, sha256 ee2973aa8ad8c453b5720099c27b37cb9980820da74f33ad9c78ddd7ad591dc7); the main session checked at 06:40 UTC that `git diff HEAD | sha256sum` in the worktree gives that value. Your session's folder is the main folder, not the worktree: run each worktree command in a subshell, `( cd /home/user/Shamble-Frontlines/.claude/worktrees/brawl1 && ... )`, so your shell stays in the main folder, and give the Read, Edit and Write tools absolute paths inside the worktree. Your notes stay where copy 1 kept them, in the main folder: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/brawl1-build.md (388 lines). Never edit any other file outside the worktree.
- **The runner changed** (the ruling's step 2, FP-0, made by the main session and committed at 7402dac): /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4r-run.sh is now sha256 2f754d125b10b6bde06a980f46cb4e078a582ca277ec8828bc48e8aacb064e50, in place of the 6efb9157... above. Its only change, one line, is in u4r-run.sh.te-b1-7.diff beside it: each log's header now also reads `diff <sha256>; untracked <sha256>`. Check the runner's sha256 before its first use and read that diff. Your first runner log, before any edit, must show `diff ee2973aa8ad8c453b5720099c27b37cb9980820da74f33ad9c78ddd7ad591dc7`: record that it does (FP-0's evidence).
- **The start checks, in place of copy 1's:** HEAD is 352a245c...; the brief's sha256 is 921c3243958a09dd5f914fb7eb247f07333558d0905b3d849d2f13222d6d2a50; the runner's is 2f754d12...; `git diff HEAD | sha256sum` in the worktree is ee2973aa... (the ruling's step 1: if it is not, stop and report); and `df -h /home/user`. Write them under a new heading "## Fix pass, round 1 (copy 2)" at the end of your notes. If any does not hold, stop and report.
- **Before you change anything** (Mustafa's rule: a fresh copy reads its notes and the diff in its worktree before it changes anything): read your notes whole, then the diff (`git status --short`, `git diff`). Read again the brief's section 4 (its rules hold throughout: the ruling's step 1) and the code each step names.
- **The advisor's rulings bind you**, in the main folder's .checkpoints/netcode-proof-test/: advisor-brawl1-plan-full.md (conditions 1-18), advisor-brawl1-points-full.md, and new, **advisor-brawl1-r1-full.md** (sha256 86bec63bfffbd676efbc09e09c35a952e63abc7865e89b20b5aebec11bee75c5, 249 lines). Its section "The fix pass, in order" is copied below word for word: it is your work. Its sections "The pins", "The size" and "Each finding" give the reasons; round 1's findings themselves (Where, Wrong, Evidence, Fix) are in brawl1-review-r1-full.md (sha256 fc979c632a7681885c7c298af1d99b1e389533b21d19c7a41ad3d0e041973305). Where a finding's own Fix and the ruling differ, the ruling wins. Keep a block "Rulings (advisor-brawl1-r1)" in your notes with each item's state as you go.
- **Your work, in this order:** the ruling's steps 3 to 11 (FP-1 to FP-9), in its order. Step 2 (FP-0) is the main session's and is done (above). Step 1's rules hold throughout. FP-8's end checks come last, after every edit. One plant at a time (FP-7): take the file's sha256 before the plant, plant by Edit, restore by Edit, check the sha256 equal, and never stop with a plant in place.
- **Disk:** 6,566 MiB was free at 06:40 UTC (the shared target 6,442 MiB). Keep section 4 rule 6's floors: df first; 4 GiB before any release or wasm32 build, 2 GiB before any other. A floor stop is reported and never cleaned around. Delete nothing.
- **Stops** (each is reported whole and never retried): a check.mjs test-step failure (TE2-5); a SHA-256 mismatch in det-u4.mjs or the precheck regeneration (with the first differing tick); any change to a figure the full sim-only test printed in round 1 (FP-8 (4)); a non-cargo sum over 240 s; a plant among P2 to P4 that does not bite as ruled (that item stops, the rest goes on); P1 still not biting after the door-entry bar (that item stops); numstat past 1,400 changed lines against 352a245, SERIES.md included (stop: the main session tells Mustafa in three lines). Never re-run a failed suite into a pass.
- **Cuts:** the session's spend limit or a restart may cut you mid-run. Save each finished item to your notes at once. When most of your context is used, stop at a clean point (no plant in place, no half edit, no run going), write a "## State" block with the next steps, and report.
- **Your report** must be on disk before round 2 starts: write it at the end of your notes under "## Fix pass, round 1: report", then send it as your final message. In it: each step's state with file:line; every new test named with its file, and the counts as deltas from 535/613; each plant P1 to P4 with its three sha256 values, its log, each command's exit code and the failing assert's message (P1: per seed and pattern, in_building, first, and the largest distance with its slot, tick, x and y; and whether the door-entry bar was added); FP-8's results (check.mjs's summary lines, det-u4.mjs's PASS line with both pins and the non-cargo sum, the regeneration line, the short copy's times and dead figures, the full test's figures against round 1's, the two manifests' sha256, the size by numstat); FP-9's items 1 to 6; and anything left undone, with why.
- Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes; never `cargo fmt` writing files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole with Write.

**The advisor's fix pass, word for word** (advisor-brawl1-r1-full.md, lines 11-192):

## The fix pass, in order

### Step 1: 1. THE COPY AND ITS RULES (the main session sends this with the steps below)

- **Findings:** TE-B1-1, TE-B1-2, RR-B1-1
- **Words:** A fresh builder copy (copy 2): general-purpose, opus, effort xhigh. It works in the same worktree (/home/user/Shamble-Frontlines/.claude/worktrees/brawl1), with no isolation. Its prompt is copy 1's launch prompt plus a resume part. Before any edit it checks that `git diff HEAD | sha256sum` gives ee2973aa8ad8c453b5720099c27b37cb9980820da74f33ad9c78ddd7ad591dc7. If it does not, the copy stops and tells the main session. It reads brawl1-build.md and the diff first, and adds a new section 'Fix pass, round 1' to brawl1-build.md as it goes. The rules of brawl1-brief.md section 4 hold throughout:
- Every change is made with the Edit tool. rustfmt's diffs are applied by Edit; never `cargo fmt` writing files.
- Every run goes through u4r-run.sh with -x, df first, and 4 GiB free before any release or wasm32 build. A floor stop is reported and never cleaned around.
- Each plant is restored by Edit, and the file's sha256 is then checked equal to its pre-plant value.
- Counts are deltas from 535/613, and each new test is named with its file.
- A check.mjs test-step failure is recorded whole (TE2-5) and never retried.
- Any SHA-256 mismatch in det-u4.mjs or the precheck regeneration: stop. No re-pin and no retry; the log comes to me whole, with the first differing tick.
- Any change to a figure the full sim-only test printed in round 1 (FP-8 (4)): stop, and it comes to me whole.
- SIZE-CHECK: past 1,400 changed lines by numstat against 352a245 (SERIES.md included), stop. It goes to Mustafa in three lines.
- No commit.
The Lows ride this push because the Mediums need it anyway; a Low never triggers a push on its own. At the end the main session cuts a new snapshot by script, and round 2 reviews its diff against ee2973aa.
- **Evidence:** From FP-0 on, each runner log's header carries the diff hash at that moment (a log run before any edit shows ee2973aa...). The notes section in brawl1-build.md. Logs under .gate/u4r/brawl1/ named fp-*.

### Step 2: 2. FP-0 (the main session, before copy 2 starts; this ruling approves the runner edit, outside the change set, as FP-0 in U4r's round 1)

- **Findings:** TE-B1-7
- **Words:** In /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4r-run.sh, inside the header block (:64-67), after the `echo "pwd $(pwd); free ...; HEAD ..."` line (:66), add:
`echo "diff $(git diff HEAD -- . | sha256sum | cut -d' ' -f1); untracked $(git status --porcelain --untracked-files=all -- . | sha256sum | cut -d' ' -f1)"`
In main-run.sh, add the same line after :41. Its working folder is the main folder's spikes/netcode-proof, set at :26.
Rules: Edit tool only. Save each change as a .diff beside its runner (u4r-run.sh.te-b1-7.diff, main-run.sh.te-b1-7.diff), in INF2-4's form. `bash -n` on each runner must print nothing. No other runner line changes.
Why: today every log records only 'HEAD 352a245' while the change is uncommitted. Run from spikes/netcode-proof, `git diff HEAD -- .` equals the whole `git diff HEAD`, because every change lies under that folder. So the line gives the snapshot's own sha256. On a clean committed tree, both hashes are the sha256 of empty input.
- **Evidence:** The two .diff files. Empty `bash -n` output. Copy 2's first log shows the diff line. At M3, the counted run's header should show e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 for both diff and untracked.

### Step 3: 3. FP-1: the sim-only test (bots/src/lib.rs, the #[cfg(test)] module only)

- **Findings:** TE-B1-1, TE-B1-2, TE-B1-6, NA-B1-3, RR-B1-3, EA-B1-1, ADV-PT-L1
- **Words:** (a) TE-B1-1's record check: after `let now = soldiers(&world);` (bots lib.rs:519), add `assert_eq!(now.len(), n, "tick {t}: one Soldier record per slot");`. The World writes one record per soldier slot, dead or alive (npt-sim world.rs:1816-1818).

(b) The dead-bit tie: one fix for TE-B1-6, NA-B1-3, RR-B1-3 and the part of EA-B1-1 that is done now.
- In `soldiers()` (:474-488), after `let dead = r.grid[5] & (1 << 3) != 0;`, add `assert_eq!(dead, r.grid[6] == 0, "slot {}: flags (field 5) bit 3 is set exactly when health (field 6) is 0", r.index);`. npt-sim sets flags = FLAG_DEAD and health 0 together at a death (world.rs:1630-1634) and clears both at a revive (:1176-1188).
- Record (:459-468) gains `dead: u64`: the soldier-ticks seen dead, counted in sim_run's loop over `now` (bind the dead flag at :520). printed() prints it.
- The short copy (:636-640) asserts `r.dead > 0` for each pattern, with the message "the dead-bit tie is exercised". If a pattern at seed 1 shows none, that assert moves to the full test (any seed of that pattern), and the notes say why.
- soldiers()'s doc adds: 'This pins the literals npt-client copies (client lib.rs:247-248, SOLDIER_FLAGS and SOLDIER_DEAD) against npt-sim's real deaths.'
TE-B1-6's separate kill test is not needed: this tie runs on every record of every tick, with the brawl's real deaths.

(c) TE-B1-2's npt-sim half: a new #[test] in the same module, `brawl1_a_soldier_teleported_into_building_0_reads_area_1`.
- It builds `World::new(&Scene::stress().with_stand_ins(StandIns::OFF), 1, FIRST)`.
- It takes one step with `TickInput { tick: FIRST, teleports: vec![Placement { slot: 0, position: [2_200, 0, 0] }], ..TickInput::default() }` and `HitHistory::for_ms(2_000)`.
- It asserts that `soldiers(&world)` gives slot 0 area 1 and every other slot area 0.
The point is building 0's centre (greybox.rs:92). Its footprint is half-open, x 1,800 to 2,600 (:103-104, :176-183). A teleport moves only a live soldier (world.rs:1147-1151), and all soldiers start alive (:745). Import Placement.

(d) ADV-PT-L1: printed() (:610-631) prints the chances and the per-1000 rate only for Pattern::Skips. EveryTick prints 'skipped {n}' alone, and Bursts prints 'skipped {n} in its bursts'. The numbers come from the Record, with no literal. The longest streak stays for all three patterns.
- **Evidence:** fmt and clippy clean in both builds (check.mjs). The new test passes. The short copy passes in both builds; record each build's time (at most 60 s, condition 13) and its dead figure, which must be above 0. The full test's prints: FP-8 (4).

### Step 4: 4. FP-2: the steering unit test

- **Findings:** TE-B1-3
- **Words:** In brawl1_the_leash_steers_past_its_outer_radius_until_inside_its_inner_one (bots lib.rs:346-432). I re-computed the command sequence: slot 5 at seed 1 has walk phase 7.

(a) Replace the blocked loop (:365-371) with:
`let blocked = toward_centre(1_450, 200); for _ in 0..3 { let (got, walk) = give(at(1_450, 200)); assert_ne!(walk, blocked, "the twin's walk differs, so the check can fail"); assert_eq!(got, blocked, "blocked: the position does not move"); }`
toward_centre(1,450, 200) = [-125, -17], which is in no MOVES entry. Today's asserts sit on commands 4 to 6, whose walk is [-127, 0]: the steer itself, so they cannot fail.

(b) Right after the None case (:389-390), insert:
`let (got, walk) = give(at(1_350, 0)); assert_ne!(walk, steer, "the twin's walk differs, so the check can fail"); assert_eq!(got, steer, "None and dead keep the leash's state");`
This is command 12, whose walk is [-89, -89].

(c) Right after the case 'between the radii, not past the outer since' (:393-394), insert:
`let (got, walk) = give(at(1_400, 0)); assert_ne!(walk, steer, "the twin's walk differs, so the check can fail"); assert_eq!(got, walk, "on the outer radius: not past it"); let (got, walk) = give(at(1_401, 0)); assert_ne!(walk, steer, "the twin's walk differs, so the check can fail"); assert_eq!(got, steer, "1 cm past the outer radius");`
These are commands 15 and 16, whose walks are [-89, -89] and [-49, -117]. The diagonal case after them still steers to [-89, 89].

(d) At the top of the tests module: `const _: () = assert!(LEASH_IN_CM >= 1_300 && LEASH_OUT_CM >= LEASH_IN_CM + 100 && LEASH_OUT_CM <= 1_500, "the advisor's plan review, condition 1's bounds");`.
- **Evidence:** The unit test passes. Its count does not change: it is the same test.

### Step 5: 5. FP-3: the lab figure's positive control (logs.rs; TE-B1-2 and RR-B1-1 name the same gap and are fixed once)

- **Findings:** TE-B1-2, RR-B1-1, EA-B1-4
- **Words:** In logs.rs's #[cfg(test)] module (:775 on), add `brawl1_the_building_figure_counts_a_soldier_in_a_building_in_measured_ticks_only`:
1. `let scene = Scene::stress().with_stand_ins(StandIns::OFF); let first = 63_736u32; let mut server = crate::scenario::Server::build(&scene, 1, first, 48, 0, 0, &crate::scenario::Bites::default());`
2. `server.room.script(&[], &[Placement { slot: 3, position: [2_200, 0, 0] }], &[]); server.room.tick();` This puts soldier 3 in building 0 (area 1). With no command it stays there.
3. Take a folder from `crate::scenario::tests::scratch("brawl1-figure")` and make it with `std::fs::create_dir_all`; Logs::open's File::create_new needs the folder to exist (logs.rs:77-88). Then `let s = crate::scenario::tests::settings(&dir, "--profile P0 --seed 1"); let mut logs = Logs::open(Some(&dir), &s.meta, first, false)`.
4. `logs.room_tick(&server.room, server.room.last_tick(), Phase::Warmup, &[], &[], &[])`. Assert that `logs.counts.in_building_measured == 0` and `logs.counts.in_building_slots.is_empty()`.
5. `server.room.tick();` then `logs.room_tick(&server.room, server.room.last_tick(), Phase::Measured, &[], &[], &[])`. Assert that `in_building_measured == 1` and `in_building_slots == BTreeSet::from([3])`. Two room ticks are needed: npt-log's writer refuses a tick record at or before the last one (log lib.rs:467-474). So TE-B1-2's 'the same tick in both phases' would fail on the writer, not on the figure.
6. `let counts = logs.finish(Ended::Clean, &[])`. Read room.jsonl with `crate::scenario::tests::read(&dir, "room")`. Both tick records hold `in_building_slot_area` = [3, 1], the warm-up one too (condition 10: written in any phase).
7. `summary_line(&s, &Summary { counts: counts.clone(), ..Summary::default() })` ends with "in a building: 1 slot-ticks (slots 3)". With `settings(&dir, "--profile P0 --seed 1 --logs 0")`, it ends with "in a building: not counted (--logs 0)".
The test holds the Scratch guard until its last assert (T-5's rule). The figure's code (:584-599) does not change in this pass (RR-B1-7 (1) is deferred).

EA-B1-4: replace logs.rs:6-7's 'Every tick and second record carries all its counters, 0 included; every log ends with its footer.' with 'Every tick and second record carries all its counters, 0 included (BRAWL-1's `in_building_slot_area`, a list, is written only on ticks with one: the advisor's plan review, condition 10); every log ends with its footer.'
- **Evidence:** The new test passes in both builds and is named in check.mjs's counts. Plant P2 (FP-7) makes it fail at its first figure assert.

### Step 6: 6. FP-4: the client read item's tests and docs

- **Findings:** TE-B1-4, NA-B1-1, RR-B1-5, EA-B1-3, RR-B1-3
- **Words:** (a) TE-B1-4 (a): in brawl1_own_soldier_is_the_newest_complete_filed_tick_s_own_record (client/tests/endpoint.rs:397-442), after the tick 105 assert (:435) and before tick 106, add:
`let out = feed(&mut e, vec![datagram(own_only(105, 1, vec![soldier(0, -701)]))]); assert_eq!(out.hashes, [(105, 0)], "a second, different group 0 turns tick 105 into the sentinel (carry 61 (b))"); assert_eq!(e.own_soldier(), first, "a complete tick turned sentinel is not read: tick 101 again");`
The group handler passes a different group to sentinel(), which sets only `filed` (client lib.rs:626-637, :725-731). Tick 101 is then the newest complete tick filed Hash still in the ring. The test's doc line names this case.

(b) One wrap case, for TE-B1-4 (b), NA-B1-1, RR-B1-5 and EA-B1-3 (b): a new #[test] `brawl1_own_soldier_compares_ticks_by_the_endpoint_s_wrap_rule`, in NA-B1-1's form.
- A fresh endpoint; JoinAccepted for slot 0 at tick u32::MAX - 1.
- A complete own_only(u32::MAX, 1, vec![soldier(0, 11)]). Assert own_soldier = Some(([11, -11, 150], false)).
- Then a complete own_only(0, 1, vec![soldier(0, 22)]). Assert out.hashes.len() == 1, and own_soldier = Some(([22, -22, 150], false)) with the message 'tick 0 comes after u32::MAX by the endpoint's wrap rule'.
With ACK_RING_TICKS at 32, tick u32::MAX sits in ring slot 31 and tick 0 in slot 0. So this one case kills both mutants: a plain maximum and a 'last matching ring entry' read both return tick u32::MAX's record. TE-B1-4 (b)'s 127/128 case is then not needed.

(c) EA-B1-3 (a): in own_soldier's doc (client lib.rs:347-348), replace 'A newer tick that is incomplete, refused or the sentinel changes nothing.' with 'A newer tick that is incomplete, refused or the sentinel changes nothing while the complete tick stays in the ring (ACK_RING_TICKS ticks); a newer tick that takes its ring slot drops it, and then the newest complete tick still held is read, else `None`.'

(d) RR-B1-3's doc line: at the end of the doc above SOLDIER_FLAGS (client lib.rs:244-246), add: 'npt-bots' sim-only test pins the same two literals against npt-sim's real deaths (its `soldiers()`: the bit set exactly when health is 0), so a change there fails that test; both copies change together until npt-sim exposes them (BRAWL-1 round 1, EA-B1-1).'
- **Evidence:** Both client tests pass, and the wrap test is named in the counts. Plants P3 and P4 (FP-7) make each test fail at its new assert.

### Step 7: 7. FP-5: doc words in npt-bots' library (no code)

- **Findings:** NA-B1-2, RR-B1-2
- **Words:** (a) NA-B1-2 (a), bots lib.rs:57-59: replace '..., 4 m beyond it.' with '..., 4 m beyond it as geometry. A soldier overshoots the outer radius, because the leash reads a decoded position some steps old and a skipped tick re-runs the last command: at BRAWL-1's review the sim-only test's skip pattern reached 1,720 cm and B1's bursts 1,832 cm, past the door entry (it prints the largest distance per pattern; BRAWL-1 round 1, NA-B1-2).'

(b) NA-B1-2 (b), :245-246: replace 'so a re-run command or a step cut short is corrected at the next command' with 'so a re-run command or a step cut short is corrected once the decoded position shows it (lead - 1 steps later, plus the ticks between the newest and the newest complete tick)'.

(c) RR-B1-2, by its documented alternative, with no code change:
- toward_centre's doc (:125-127) adds: 'Its inputs are decoded grid positions (16 bits on the wire: |x|, |y| at most 32,768, npt-proto quant.rs:197-215), so x² + y² stays below 2^31 and the i64 arithmetic cannot overflow; it is not for other inputs.'
- leash's doc adds: '(x and y in toward_centre's domain)'.
The u64 rewrite is not taken. It would edit library code on the recording path for an input that cannot arrive.
- **Evidence:** fmt clean. The full test's figures are unchanged (FP-8 (4)) and the pins are reproduced (FP-8 (2), (3)). Together these show the library behaves as before.

### Step 8: 8. FP-6: small items in scenario.rs and hash.rs

- **Findings:** EA-B1-2, RR-B1-7, RR-B1-4, ADV-PT-L2
- **Words:** (a) EA-B1-2: at scenario.rs:397, `pub(crate) fn new(room: Room<Sim>, quant: Quant, rtt: RttSource) -> Server {` becomes `fn new(room: Room<Sim>, quant: Quant, rtt: RttSource) -> Server {`. Its one caller is Server::build (:390); I grepped crates/ and check.mjs.

(b) RR-B1-7 (2): rewrap Server::build's doc (:368-372) so that no line holds a lone fragment: '... behind one decoder per session on `bites`' `Quant`; `bites` plants its World bite (armed at `arm_tick`), its room bite and its pump bite. `run_leg` passes ...'. The words do not change.

(c) RR-B1-4. The code order is kept, because a line that breaks both bounds is refused either way.
- Reword :1433-1434 to: '// advisor-u4r-r2, RR4-L2, after the run-length 64-bit checks (which keep their messages): the run's last tick, counted from its first (the warm-up, measured and after phases' ticks), fits 32 bits, on every profile. The burst block's own 64-bit check comes below it, then its 32-bit cap.'
- Reword :2576-2577 to: '// u32::MAX is odd, so never a whole-second length; the largest below it (4,294,967,280 ticks) accepted.'
- After the after-max accepted case (:2584-2587), add two cases. First: `assert!(parsed("B1", "--bursts 2 --burst-first-s 0 --burst-every-s 143165566").is_ok(), "the last burst at 4,294,967,280 ticks");`. Second: `parsed("B1", "--bursts 2 --burst-first-s 0 --burst-every-s 143165567")` is refused with exactly '--bursts 2, --burst-first-s 0 and --burst-every-s 143165567: the last burst's tick, 4294967310 ticks from the run's first, is above 4294967295 (32 bits)'.
The arithmetic: (1 x 143,165,566 + 0) x 30 + 300 = 4,294,967,280, and burst_starts has no in-run check. fp3's doc (:2508) then stays true.

(d) ADV-PT-L2, at hash.rs:1560-1562:
- '360000' becomes '359993' in the replace.
- `let at = 360_000 / 12;` becomes `let at = 359_993 / 12;` (29,999).
- The comment reads: 'At the rule's boundary below RR4-L1's 36,000-tick cap (advisor-u4r-r2): 359,993 bytes hold at most 29,999 ticks; not a multiple of 12, so the rule's rounding down is checked (ADV-PT-L2).'
- **Evidence:** fp3 and r10_carry_82 pass in both builds. Clippy is clean: narrowing Server::new raises no dead-code or private-link warning.

### Step 9: 9. FP-7: the plants

- **Findings:** TE-B1-1, TE-B1-2, RR-B1-1, TE-B1-4, NA-B1-1, RR-B1-5, EA-B1-3
- **Words:** Each plant: the file's sha256 before, the plant by Edit, the sha256 planted, the runs, the restore by Edit, and the sha256 again, equal to before. All runs go through u4r-run.sh -x -k, so every command runs even after one fails.

P1, TE-B1-1 (the leash off): make `return walk;` the first line of `fn leash`'s body (bots lib.rs:249). Run in one call, in final-regen-tests.log's forms (:27, :48, :69):
(1) `cargo test --locked -p npt-bots --lib -- tests::brawl1_sim_short_no_soldier_enters_a_building --exact --nocapture`
(2) the same with `--all-features`
(3) `cargo test --locked --release -p npt-bots --lib -- tests::brawl1_sim_full_no_soldier_enters_a_building_seeds_1_to_3 --ignored --exact --nocapture`
The bite rule: the short copy fails in both builds, and the full test fails with Skips listed for at least one seed. Record EveryTick's figures as they come. If it never bites, the record says so: it is the closed-circle baseline.
If the bite rule does not hold on area: add the door-entry bar below to both barred patterns of both tests, and run the same call under the plant again. The rule must then hold. If it still does not, that item stops and its record comes to me whole; the rest of the pass goes on.
The bar:
- In the tests module: `/// The nearest door entry, cm from the centre: a soldier's centre enters building 0 at x 1,800 (centre x 2,200 minus ROOM 400) with |y| < 60 (npt-sim greybox.rs:64-70, :92-99). const DOOR_ENTRY_CM: i64 = 1_800;`
- In the short copy, after the area assert: `assert!(r.far.0 < DOOR_ENTRY_CM.pow(2), "{pattern:?}: largest distance {} cm, at or past the door entry", r.far.0.isqrt());`
- In the full test, a barred pattern with `r.far.0 >= DOOR_ENTRY_CM.pow(2)` is pushed to `barred` too.
Round 1's largest barred distance is 1,720 cm, so the bar holds with the leash on. This bounded fallback replaces TE-B1-1's open 'lengthen or harden until it does'.

P2, the figure (TE-B1-2 = RR-B1-1): in room_tick's building block (logs.rs:586-588), make the filter `.filter(|_| false)`. Run `cargo test --locked -p npt-lab --bin npt-lab -- logs::tests::<FP-3's test> --exact --nocapture`. It must fail at its first figure assert.

P3, the Hash filter (TE-B1-4 (a)): remove `&& matches!(t.filed, Filed::Hash(_))` from own_soldier (client lib.rs:356).
P4, the wrap rule (TE-B1-4 (b) = NA-B1-1 = RR-B1-5 = EA-B1-3 (b)): replace the `.reduce(...)` (:357-363) with `.max_by_key(|t| t.tick)`.
For each of P3 and P4, run `cargo test --locked -p npt-client --test endpoint -- brawl1 --nocapture`. P3 must fail at 'a complete tick turned sentinel is not read'; P4 at the wrap test's last assert.
- **Evidence:** For each plant, in brawl1-build.md's new section: the file, its three sha256 values, the planted line, the log path (.gate/u4r/brawl1/fp-plant-<n>.log), each command's exit code, and the failing assert's message. For P1 also, per seed and pattern: in_building, first, and the largest distance with its slot, tick, x and y; and whether the door-entry bar was needed. A plant among P2 to P4 that does not bite as ruled stops that item, and its record comes to me whole.

### Step 10: 10. FP-8: the end checks (after every edit above)

- **Findings:** TE-B1-1, TE-B1-2, RR-B1-1
- **Words:** Before its first edit, the copy records the sha256 of crates/lab/det-recordings.json and crates/lab/precheck-recordings.json. On the final code, through u4r-run.sh -x, df first, with each log carrying FP-0's diff line:
(1) check.mjs once: all 11 steps PASS. Counts as deltas from 535/613, each new test named with its file. Expected: +3 in each build (the bots area test, the logs.rs figure test, the client wrap test).
(2) `uptime`, then `node ops/runs/det-u4.mjs`, in one call, with no other lane building or running (condition 8). It must PASS and reproduce SS-P2-s1 1ec25de7c9ce8ea869ed9a8b343746ba131593e1de5c65bc9c1c626ce01cc76d and ES-P3-s2 6abf7643982b5970a630c97daa4d0815ceb2022958b18e18116397dcad9c893e. The non-cargo sum must stay under 240 s; over it, stop, and it comes to me whole.
(3) The precheck regeneration as final-regen-tests.log:18 ran it. It must print 'regenerated SS-P0-s4: 16950444 bytes, sha256 1f00d2d5f70a2c7b6836ede611b5f1c281319a5312060ead91e016da3c0b1b42, as the manifest's'.
(4) The short copy in both builds, with each build's time (at most 60 s), and the full test by name in release. Every figure round 1 printed must equal final-regen-tests.log:41-42, :62-63 and :83-91: in a building, first, the largest distance with its slot, tick, x and y, skipped, the chances and rate for Skips, the longest streak, and stuck. Only the dead figure and the reworded skip text are new. Any other difference means behaviour moved: stop, and it comes to me whole.
(5) Both manifests keep the sha256 recorded before the first edit.
(6) The size, by numstat against 352a245, SERIES.md included.
A det-u4.mjs or regeneration mismatch: stop. No re-pin and no retry; the log comes to me whole, with the first differing tick.
- **Evidence:** final-fp-check.log, final-fp-det.log and final-fp-regen-tests.log under .gate/u4r/brawl1/, each with its diff line. The figures go in the notes and the report.

### Step 11: 11. FP-9: report words, no code (in brawl1-build.md's new section and the copy's report, on disk before round 2)

- **Findings:** TE-B1-5, TE-B1-8, ADV-BP-3, ADV-BP-4, RR-B1-6, EA-B1-5, ADV-PT-M1, NA-B1-2
- **Words:** 1. The plant records P1 to P4, and whether the door-entry bar was added, and why.
2. Pattern (iii): withdraw 'a fixed radius stops it' and the 14 m vehicle. sim_run runs with stand-ins off (bots lib.rs:496). No solid lies within 1.6 m of (1734, -592), (1671, -751) or (1586, -917): building 0's corner (1770, -430) is 166 cm from the first point, the crates lie within 950 cm of the centre, and the wall stand-in is at y 1,180-1,220. The reading for U4g is this ruling's U4g words.
3. TE-B1-8: 'No test drives Behaviour::step with an own soldier past the outer radius. The glue at bots lib.rs:223 is shown only by the moved pins (own_soldier returns Some and changes commands) and by the two 10-minute evidence runs (condition 14).'
4. My corrections, in my words, beside ADV-BP-1 and ADV-BP-2: ADV-BP-3 and ADV-BP-4 as the points ruling words them.
5. RR4-L1's constant: MAX_ENTRY_TICK_COUNT stands (points ruling, Point 1). RR-B1-6 and EA-B1-5 are answered there, and ADV-PT-L3 carries the point to the check.mjs follow-up.
6. The follow-ups with their homes, as listed under 'Each finding': RR-B1-7 (1); EA-B1-1's pub constants, with NA-B1-3's npt-client half; ADV-PT-L3; TE-B1-8's optional glue test.
- **Evidence:** The section is on disk before the main session cuts round 2's snapshot.
