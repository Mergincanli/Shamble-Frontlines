# U4r review, rust-reviewer seat (batch 2), round 1

Snapshot: wip/u4r.patch sha256 572c4f69435da2ad6f62d54d4bcbc47b04d6329b5493e0faf78fcecfdc4a09c2 (verified by sha256sum at start).
Packet: u4r-packet.md cbd82485..., tests file d12555c8... (verified).
Read copy: /home/user/Shamble-Frontlines/.gate/u4r-snap1 (2fbec77 + patch).

## Plan
1. Read the panel section 5 + "What every seat is told" (done).
2. Read the packet and tests file; read the patch whole.
3. Per crate: lab/src (scenario, logs, mirror, precheck, replay, contract, hash, lib, main), bots (lib, tests, clippy.toml), server (pump, schedule, tests/schedule), log/src/lib.rs.
   Checks: panics on input, overflow, determinism (HashMap order, cfg(bites) gating), memory on large runs (no-log path, judge reading logs whole).
4. npt-server edits: site in pump, clippy allow on site alone, field rule.
5. npt-log edits: dup-key and control-char refusals, cost, nothing else changed.
6. Own earlier findings: carry 82 (RR-L5), 83 (d) (RR2-L3), 85 (RR2-L1, RR2-L2, RR2-L4 (a)), 3.3 carries 6 and 14 (rust 1, rust 3).
7. scratch helper scenario::tests::scratch.
8. Runs only if a finding needs one (u4r-run.sh -x; logs under .gate/u4r-review-rust/).
   Note: u4r-run.sh accepts only a worktree under .claude/worktrees/ and a log under .gate/u4r/, so neither the read
   copy nor a .gate/ worktree nor .gate/u4r-review-rust/ logs can go through it; free disk 4.7 GB at start. So: no
   cargo run; findings by reading, labelled so.

## Done so far
- server pump/schedule/tests diff read; bots lib + clippy.toml read; log diff read; scenario.rs read whole; logs.rs
  read whole; mirror.rs read whole; hash.rs diff read.

## Findings (a line each as found)

- F1 (Medium, candidate) scenario.rs:588-594 `record()` builds argv by format! + split_whitespace; the runner entry's
  `profile` string (hash.rs:276-282, any JSON string) is not checked, so whitespace in it injects scenario flags
  (`--lead 3`, `--every-step 1`, `--behaviour`, `--bytes <dir>` file writes, `--role twin --bite <id>` in a bites
  build). parse's refusal of --lead/--every-step with a recording (:1377) keys on `record.is_some()`, which record()
  never sets (it sets record_mem after parse), so ruling 1 point 9 (a)'s refusal is bypassed on the regeneration path.
  Fix: validate profile in manifest() (P0..P4|B1), build argv as a Vec (no split), and refuse --lead/--every-step when
  record_mem too. By reading; not run.
- F2 (Low) hash.rs manifest(): for Kind::Runner, `ticks` does not drive generation; warmup_s/measured_s do, and are
  unbounded and not checked against ticks before generating (parse `* hz` unchecked at scenario.rs:1322-1324, wraps in
  release). Carry 82's "bound before generating" does not hold for runner entries. Also `bytes` itself is unbounded, so
  `ticks <= bytes/12` bounds nothing for a synthetic entry whose bytes is large (my own RR-L5 fix's limit).
- F3 (Low) logs.rs no-log path: `filed()` inserts a Vec<Res> per tick into `res` (logs.rs:285-288, :325) and only
  write_client_tick removes it, which flush never reaches with no room_tick (phases empty): about n*16 B + node per
  tick kept for the whole run (about 7 MB for 9,000 ticks at 48 clients). Bounded; Low.
- F4 (Low) scenario.rs parse: `num(..)? * hz` (:1322-1324), burst_starts (:1263) and run's sums (:821, :1038)
  unchecked: operator text; debug panics, release wraps. Fix: checked_mul/checked_add with a usage error.
