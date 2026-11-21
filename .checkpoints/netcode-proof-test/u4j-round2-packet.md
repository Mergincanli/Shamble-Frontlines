# Review packet: U4j round 2 (the judge and marks change: the fix pass on round 1's findings)

_Cut by `review_packet.py` from `u4j-round2-packet.json`. It holds sources only: code diffs, the approved plan's lines, the advisor's rulings, the last round's findings and Mustafa's words, each copied by line range. It holds no conclusion of the builder or the main session (no builder notes, brief, checkpoint card or summary). You stay free to read any file and run any test._

## Heads

- Base: 411eeeed50d8c71e7e1f74366f0e1d5c27388444 on the spike branch (U4b's merge), where U4j's build started.
- Round 1 reviewed the pre-fix copy `.gate/u4j-prefix` (U4j's first build).
- Round 2 reviews the fixed copy `.claude/worktrees/agent-a3b8dfeb01ed52b50` (uncommitted there; saved on the branch as `.checkpoints/netcode-proof-test/wip/u4j.patch` at 4ad80a9, sha256 98ee1f638db724c5…, 64 files on the base).
- The diffs below compare the two copies: what changed since round 1.

## What changed (the diffs)

### npt-judge's sources (crates/judge/src)
_`git diff --no-index .gate/u4j-prefix/spikes/netcode-proof/crates/judge/src .claude/worktrees/agent-a3b8dfeb01ed52b50/spikes/netcode-proof/crates/judge/src`:  4 files changed, 777 insertions(+), 146 deletions(-)._

````diff
diff --git a/.gate/u4j-prefix/spikes/netcode-proof/crates/judge/src/eval.rs b/.claude/worktrees/agent-a3b8dfeb01ed52b50/spikes/netcode-proof/crates/judge/src/eval.rs
index afef95a..ea4b91d 100644
--- a/.gate/u4j-prefix/spikes/netcode-proof/crates/judge/src/eval.rs
+++ b/.claude/worktrees/agent-a3b8dfeb01ed52b50/spikes/netcode-proof/crates/judge/src/eval.rs
@@ -437,12 +437,11 @@ fn in_force(
     check: &crate::marks::Check,
     n: BoundNumber,
     run: &Run<'_>,
-    set: &[&Run<'_>],
     ctx: &Ctx<'_>,
 ) -> Result<(crate::marks::Check, String), String> {
     let tolerance = practice_tolerance(ctx.mark, check)
         .ok_or_else(|| format!("{} holds no ws-practice rule", ctx.mark.id))?;
-    let p = practice::find(run, set, ctx.mark, tolerance)?;
+    let p = practice::find(run, ctx.all, ctx.mark, tolerance)?;
     let mut moved = check.clone();
     match (n, &mut moved.measure) {
         (BoundNumber::Y, Measure::Series { cond: Some(c), .. }) => c.value = p.moved(n),
@@ -459,7 +458,7 @@ fn eval_check(
     ctx: &Ctx<'_>,
 ) -> CheckOut {
     if let (Some(n), [run]) = (check.ws_bound, runs) {
-        return match in_force(check, n, run, set, ctx) {
+        return match in_force(check, n, run, ctx) {
             Ok((moved, how)) => {
                 let mut o = eval_check(&moved, runs, set, ctx);
                 o.name = check.name.clone();
@@ -996,7 +995,7 @@ fn rule(
             }
         }
         Rule::WsPractice { tolerance, .. } => {
-            if let Err(e) = practice::find(run, set, ctx.mark, *tolerance) {
+            if let Err(e) = practice::find(run, ctx.all, ctx.mark, *tolerance) {
                 why.push(format!("ws-practice: {e}"));
             }
         }
@@ -1027,11 +1026,12 @@ fn validity<'r>(
     v
 }
 
-/// What every judgement may consult: the whole marks file (for preconditions) and the mark being judged (whose
-/// practice run the ws-practice rule looks for).
+/// What every judgement may consult: the whole marks file (for preconditions), the mark being judged (whose practice
+/// run the ws-practice rule looks for), and every run the judge reads (where that practice run may be, in any set).
 pub(crate) struct Ctx<'m> {
     marks: &'m Marks,
     mark: &'m Mark,
+    all: &'m [Run<'m>],
 }
 
 /// The parts of a mark whose `select` a run matches, its role aside.
@@ -1726,7 +1726,17 @@ pub fn judge(marks: &Marks, logs: &[Log]) -> Vec<Verdict> {
     marks
         .marks
         .iter()
-        .map(|m| judge_mark(m, &all, &Ctx { marks, mark: m }))
+        .map(|m| {
+            judge_mark(
+                m,
+                &all,
+                &Ctx {
+                    marks,
+                    mark: m,
+                    all: &all,
+                },
+            )
+        })
         .collect()
 }
 
diff --git a/.gate/u4j-prefix/spikes/netcode-proof/crates/judge/src/figures.rs b/.claude/worktrees/agent-a3b8dfeb01ed52b50/spikes/netcode-proof/crates/judge/src/figures.rs
index 0e98c15..94e81c0 100644
--- a/.gate/u4j-prefix/spikes/netcode-proof/crates/judge/src/figures.rs
+++ b/.claude/worktrees/agent-a3b8dfeb01ed52b50/spikes/netcode-proof/crates/judge/src/figures.rs
@@ -190,11 +190,17 @@ fn state_hashes(run: &Run<'_>) -> Result<BTreeMap<u64, u64>, Lack> {
 
 /// PM-0001-21 and -30 ("identical server state hashes on every tick"): the tick numbers whose state hash (`h`) differs
 /// between the run and the run it names in `params.twin`, a tick hashed in one only counting; every phase. The twin
-/// (`twin_of`) has the same profile, seed and bite, any role (-30's twins are each other's measured runs). It fails,
-/// never passes, when neither run hashed a tick.
+/// (`twin_of`) has the same profile, seed and bite, and role `twin`, else a gap, as for `fate_log_twin_equal` (the
+/// advisor's FX-4; -30's form comes with U6). It fails, never passes, when neither run hashed a tick.
 fn twin_hash_mismatch_count(run: &Run<'_>, set: &[&Run<'_>]) -> Result<Ratio, Lack> {
     let own = state_hashes(run)?;
     let twin = twin_of(run, set, &["profile", "seed", "bite"])?;
+    if twin.role != "twin" {
+        return Err(Lack::Gap(format!(
+            "run {}: {} has role {}, not twin",
+            run.id, twin.id, twin.role
+        )));
+    }
     let other = state_hashes(twin)?;
     let ticks: BTreeSet<u64> = own.keys().chain(other.keys()).copied().collect();
     if ticks.is_empty() {
@@ -208,15 +214,38 @@ fn twin_hash_mismatch_count(run: &Run<'_>, set: &[&Run<'_>]) -> Result<Ratio, La
 }
 
 /// One `hs` name's hashes in a run, merged over its logs and every phase: per tick one entry per client slot, each
-/// with whether its log was written on aarch64 (the Arm rule). `None` when no log has the name. Gaps: lists of
-/// different lengths, a malformed entry, or two logs giving one (slot, tick) a hash.
+/// with whether its log was written on LG on aarch64 (`machine.role` LG, `machine.arch` aarch64: the Arm rule's
+/// clients). `None` when no log has the name. Gaps: the name in a log of another process than the one that writes it
+/// (`source`), lists of different lengths, a malformed entry, or two logs giving one (slot, tick) a hash.
 type SlotHashes = BTreeMap<u64, Vec<Option<(u64, bool)>>>;
 
+/// Which logs write each `hs` name (the advisor's FX-5 (a)): `server_hash` only the room's, `decoded_hash` only the
+/// clients' (process `client` on L1, `bots` on L2), `replay_hash` only a `replay` log on x86_64.
+fn source(name: &str) -> (&'static str, fn(&npt_log::Header) -> bool) {
+    match name {
+        "server_hash" => ("process room", |h| h.process == "room"),
+        "decoded_hash" => ("process client (L1) or bots (L2)", |h| {
+            h.process == "client" || h.process == "bots"
+        }),
+        _ => ("process replay on x86_64", |h| {
+            h.process == "replay" && h.machine.arch == "x86_64"
+        }),
+    }
+}
+
 fn slot_hashes(run: &Run<'_>, name: &str) -> Result<Option<(usize, SlotHashes)>, Lack> {
     let mut out: SlotHashes = BTreeMap::new();
     let mut slots: Option<usize> = None;
+    let (writer, writes) = source(name);
     for log in &run.logs {
-        let arm = log.header.machine.arch == "aarch64";
+        let h = &log.header;
+        if !writes(h) && log.ticks.iter().any(|t| t.hs.contains_key(name)) {
+            return Err(Lack::Gap(format!(
+                "run {}: hs.{name} in the {} log on {} ({}): only {writer} writes it",
+                run.id, h.process, h.machine.name, h.machine.arch
+            )));
+        }
+        let arm = h.machine.role == "LG" && h.machine.arch == "aarch64";
         for t in log.ticks.iter().filter(|t| t.hs.contains_key(name)) {
             let Some(list) = t.hashes(name) else {
                 return Err(Lack::Gap(format!(
@@ -249,25 +278,41 @@ fn slot_hashes(run: &Run<'_>, name: &str) -> Result<Option<(usize, SlotHashes)>,
     Ok(slots.map(|n| (n, out)))
 }
 
-/// The decoded-hash comparison of one run (u4-build-plan.md §4.6, the advisor's Q11 ruling).
+/// The decoded-hash comparison of one run (u4-build-plan.md §4.6, the advisor's Q11 ruling, FX-5 and FX-10).
 struct Decoded {
     /// Client hashes looked at: a (client, tick) pair with the server's hash, or a client hash at a (client, tick) the
     /// server never logged.
     compared: i128,
-    /// Of those, the ones that count: a different hash, the sentinel 0, or no server hash there; on aarch64 only when
-    /// the x86-64 re-decode misses the server's hash too (the Arm rule).
+    /// Of those, the ones that count: a different hash, the sentinel 0, or no server hash there (on every machine);
+    /// under the Arm rule (an LG client on aarch64, the room on x86_64) a different hash only when the x86-64
+    /// re-decode misses the server's hash too.
     mismatched: i128,
-    /// aarch64 mismatches the x86-64 re-decode does not reproduce: a cross-architecture finding, deciding nothing.
+    /// Mismatches under the Arm rule that the x86-64 re-decode does not reproduce: a cross-architecture finding,
+    /// deciding nothing.
     cross_arch: i128,
+    /// The first coverage gap (README §1's `null` rule: FX-10 (b) to (d)) or incomplete re-decode (FX-5 (d)). It
+    /// decides the run only after the counted mismatches and the empty comparison (FX-10 (f)), so it never hides one.
+    gap: Option<String>,
+    /// An Arm-rule mismatch in a run with no replay log: NOT-RUN, after everything above.
+    waiting: Option<String>,
 }
 
 /// Compares `hs.decoded_hash` (each client's hash of the state it holds, under the tick it rebuilt; the sentinel 0 for
 /// a tick with a group it refused; `null` for none) with `hs.server_hash` (the server's hash of what it meant that
-/// client to hold), over every phase. A server-only (client, tick) is loss, never compared. An aarch64 client's
-/// mismatch is checked against `hs.replay_hash` (its logged bytes decoded again on x86-64, process `replay`): it
-/// counts when the replay's entry differs from the server's too (both absent is agreement). Gaps: either name missing
-/// from every log, the lists' slot counts disagreeing, two logs at one (slot, tick), or a server hash of 0 (the hash
-/// is never 0, so 0 is the client's sentinel alone). NOT-RUN while an aarch64 mismatch has no replay log in the run.
+/// client to hold), over every phase. A server-only (client, tick) is loss, never compared; a client hash at a
+/// (slot, tick) the server never logged counts on every machine. The Arm rule (ADR 0001, -17's key; the advisor's
+/// FX-5): on an LG client's log on aarch64, in a run whose room log is on x86_64, a mismatch against a server hash is
+/// checked against `hs.replay_hash` (its logged bytes decoded again by a `replay` log on x86_64): it counts when the
+/// replay's entry differs from the server's too; a replay log with no entry there is a gap.
+///
+/// Structural gaps, returned at once: either name missing from every log, a name in a log of another process
+/// (`source`), a list that does not hold the run's `clients` entries (the room log's header; every `hs` list and every
+/// client log's `tick.s.snap_incomplete` list), a `snap_incomplete` entry other than 0 or 1, two logs at one
+/// (slot, tick), or a server hash of 0 (the hash is never 0, so 0 is the client's sentinel alone). The coverage rule
+/// (README §1's `null` rule, the advisor's FX-10) is kept for after the count: a (slot, tick) is resolved by a client
+/// hash (compared) or by `snap_incomplete` 1 (lost, not compared), never both; per slot, every tick from its first
+/// resolved tick to its last at which the room logged that slot's hash is resolved, and that span covers the room's
+/// measured ticks for the slot. Ticks outside a slot's span are neither compared nor counted.
 fn decoded(run: &Run<'_>) -> Result<Decoded, Lack> {
     let missing = |name: &str| {
         Lack::Gap(format!(
@@ -280,12 +325,69 @@ fn decoded(run: &Run<'_>) -> Result<Decoded, Lack> {
     let (n_client, client) =
         slot_hashes(run, "decoded_hash")?.ok_or_else(|| missing("decoded_hash"))?;
     let replay = slot_hashes(run, "replay_hash")?;
-    let n_replay = replay.as_ref().map_or(n_client, |(n, _)| *n);
-    if n_server != n_client || n_replay != n_client {
-        return Err(Lack::Gap(format!(
-            "run {}: the hash lists' slot counts differ (server {n_server}, decoded {n_client}, replay {n_replay})",
-            run.id
-        )));
+    let rooms: Vec<&npt_log::Header> = run
+        .logs
+        .iter()
+        .map(|l| &l.header)
+        .filter(|h| h.process == "room")
+        .collect();
+    let counts: BTreeSet<u32> = rooms.iter().map(|h| h.clients).collect();
+    let clients = match counts.iter().collect::<Vec<_>>().as_slice() {
+        [c] => usize::try_from(**c).unwrap_or(usize::MAX),
+        _ => {
+            return Err(Lack::Gap(format!(
+                "run {}: its room logs disagree on clients",
+                run.id
+            )));
+        }
+    };
+    // Every list holds the run's clients entries (FX-10 (a)).
+    let lengths = [
+        ("server_hash", Some(n_server)),
+        ("decoded_hash", Some(n_client)),
+        ("replay_hash", replay.as_ref().map(|(n, _)| *n)),
+    ];
+    for (name, n) in lengths {
+        if let Some(n) = n.filter(|n| *n != clients) {
+            return Err(Lack::Gap(format!(
+                "run {}: hs.{name} lists hold {n} entries, not the run's {clients} clients (the room log's header)",
+                run.id
+            )));
+        }
+    }
+    let (_, client_writes) = source("decoded_hash");
+    let mut incomplete: BTreeSet<(usize, u64)> = BTreeSet::new();
+    for log in run.logs.iter().filter(|l| client_writes(&l.header)) {
+        for t in &log.ticks {
+            let Some(list) = t.s.get("snap_incomplete") else {
+                continue;
+            };
+            if list.len() != clients {
+                return Err(Lack::Gap(format!(
+                    "run {}: tick {}'s tick.s.snap_incomplete in the {} log on {} holds {} entries, not the run's \
+                     {clients} clients",
+                    run.id,
+                    t.t,
+                    log.header.process,
+                    log.header.machine.name,
+                    list.len()
+                )));
+            }
+            for (slot, v) in list.iter().enumerate() {
+                match v {
+                    0 => {}
+                    1 => {
+                        incomplete.insert((slot, t.t));
+                    }
+                    _ => {
+                        return Err(Lack::Gap(format!(
+                            "run {}: tick {}'s tick.s.snap_incomplete holds {v} for slot {slot}, not 0 or 1",
+                            run.id, t.t
+                        )));
+                    }
+                }
+            }
+        }
     }
     for (t, list) in &server {
         if let Some(slot) = list.iter().position(|e| matches!(e, Some((0, _)))) {
@@ -300,50 +402,161 @@ fn decoded(run: &Run<'_>) -> Result<Decoded, Lack> {
             .and_then(|l| l.get(slot).copied().flatten())
             .map(|(h, _)| h)
     };
+    // The Arm rule needs the run's room on x86_64 (the server's hash is the x86-64 reference).
+    let room_x86 = rooms.iter().all(|h| h.machine.arch == "x86_64");
     let mut d = Decoded {
         compared: 0,
         mismatched: 0,
         cross_arch: 0,
+        gap: None,
+        waiting: None,
     };
     for (t, list) in &client {
         for (slot, entry) in list.iter().enumerate() {
-            let Some((h, arm)) = *entry else { continue };
+            let Some((h, lg_arm)) = *entry else { continue };
             d.compared += 1;
             let theirs = at(&server, *t, slot);
             if theirs == Some(h) {
                 continue;
             }
-            if !arm {
+            // An orphan (no server hash there) counts on every machine: the replay has nothing to reproduce.
+            if theirs.is_none() || !(lg_arm && room_x86) {
                 d.mismatched += 1;
                 continue;
             }
             let Some((_, rep)) = &replay else {
-                return Err(Lack::Waiting(format!(
-                    "run {}: slot {slot}'s aarch64 hash at tick {t} differs, and the run has no x86-64 re-decode \
-                     (hs.replay_hash, process replay) yet",
-                    run.id
-                )));
+                d.waiting.get_or_insert_with(|| {
+                    format!(
+                        "run {}: slot {slot}'s LG aarch64 hash at tick {t} differs, and the run has no x86-64 \
+                         re-decode (hs.replay_hash, process replay) yet",
+                        run.id
+                    )
+                });
+                continue;
             };
-            if at(rep, *t, slot) == theirs {
-                d.cross_arch += 1;
-            } else {
-                d.mismatched += 1;
+            match at(rep, *t, slot) {
+                None => {
+                    d.gap.get_or_insert_with(|| {
+                        format!(
+                            "run {}: slot {slot}'s LG aarch64 hash at tick {t} differs from the server's, and the \
+                             x86-64 re-decode has no entry there (an incomplete re-decode)",
+                            run.id
+                        )
+                    });
+                }
+                Some(r) if Some(r) == theirs => d.cross_arch += 1,
+                Some(_) => d.mismatched += 1,
             }
         }
     }
+    if let Some(g) = coverage(run, &server, &client, &incomplete) {
+        d.gap.get_or_insert(g);
+    }
     Ok(d)
 }
 
-/// PM-0001-17, -19 and -58: the (client, tick) mismatches `decoded` counts; it fails, never passes, on 0 compared.
+/// README §1's `null` rule on one run (the advisor's FX-10 (b) to (d)): the first (slot, tick) the client side leaves
+/// unresolved, or resolves twice. `incomplete` holds the (slot, tick) pairs a client log marks `snap_incomplete` 1.
+fn coverage(
+    run: &Run<'_>,
+    server: &SlotHashes,
+    client: &SlotHashes,
+    incomplete: &BTreeSet<(usize, u64)>,
+) -> Option<String> {
+    let mut resolved: BTreeMap<usize, BTreeSet<u64>> = BTreeMap::new();
+    for (t, list) in client {
+        for (slot, e) in list.iter().enumerate() {
+            if e.is_some() {
+                if incomplete.contains(&(slot, *t)) {
+                    return Some(format!(
+                        "run {}: slot {slot}, tick {t}: a client hash and tick.s.snap_incomplete 1 at once",
+                        run.id
+                    ));
+                }
+                resolved.entry(slot).or_default().insert(*t);
+            }
+        }
+    }
+    for (slot, t) in incomplete {
+        resolved.entry(*slot).or_default().insert(*t);
+    }
+    let span = |slot: usize| {
+        let ticks = resolved.get(&slot)?;
+        Some((*ticks.first()?, *ticks.last()?))
+    };
+    // (c): inside a slot's span, every tick the room logged a hash for it is resolved.
+    for (t, list) in server {
+        for (slot, e) in list.iter().enumerate() {
+            let inside = span(slot).is_some_and(|(a, b)| a <= *t && *t <= b);
+            let done = resolved.get(&slot).is_some_and(|r| r.contains(t));
+            if e.is_some() && inside && !done {
+                return Some(format!(
+                    "run {}: slot {slot}, tick {t}: the room logged a hash and no client log resolves it",
+                    run.id
+                ));
+            }
+        }
+    }
+    // (d): every slot the room logged at a measured tick has a span covering the room's first and last measured hash.
+    let mut measured: BTreeMap<usize, (u64, u64)> = BTreeMap::new();
+    for log in run.logs.iter().filter(|l| l.header.process == "room") {
+        for t in log.ticks.iter().filter(|t| t.ph == Phase::Measured) {
+            let Some(list) = t.hashes("server_hash") else {
+                continue;
+            };
+            for (slot, e) in list.iter().enumerate() {
+                if e.is_some() {
+                    let r = measured.entry(slot).or_insert((t.t, t.t));
+                    r.0 = r.0.min(t.t);
+                    r.1 = r.1.max(t.t);
+                }
+            }
+        }
+    }
+    for (slot, (a, b)) in measured {
+        match span(slot) {
+            None => {
+                return Some(format!(
+                    "run {}: slot {slot}: the room logged its hash at measured ticks {a} to {b}, and no client log \
+                     resolves any tick of it",
+                    run.id
+                ));
+            }
+            Some((c, e)) if c > a || e < b => {
+                return Some(format!(
+                    "run {}: slot {slot}: the client side resolves ticks {c} to {e} only, not the room's measured \
+                     ticks {a} to {b}",
+                    run.id
+                ));
+            }
+            Some(_) => {}
+        }
+    }
+    None
+}
+
+/// PM-0001-17, -19 and -58: the (client, tick) mismatches `decoded` counts, in the advisor's FX-10 (f) order: the
+/// structural gaps first (`decoded` returns them), then any counted mismatch gives its count (a FAIL), then 0 compared
+/// is `empty` (fails, never passes), then a coverage gap or an incomplete re-decode, then an Arm-rule mismatch waiting
+/// for its re-decode (NOT-RUN), then 0.
 fn decoded_hash_mismatch_count(run: &Run<'_>, _set: &[&Run<'_>]) -> Result<Ratio, Lack> {
     let d = decoded(run)?;
+    if d.mismatched > 0 {
+        return Ok(Ratio::int(d.mismatched));
+    }
     if d.compared == 0 {
         return Err(Lack::Empty(format!(
             "run {}: no client logged a hash (0 compared)",
             run.id
         )));
     }
-    Ok(Ratio::int(d.mismatched))
+    if let Some(g) = d.gap {
+        return Err(Lack::Gap(g));
+    }
+    if let Some(w) = d.waiting {
+        return Err(Lack::Waiting(w));
+    }
+    Ok(Ratio::int(0))
 }
 
 /// PM-0001-17, reported: the client hashes `decoded` looked at.
@@ -351,12 +564,16 @@ fn decoded_hash_compared_count(run: &Run<'_>, _set: &[&Run<'_>]) -> Result<Ratio
     Ok(Ratio::int(decoded(run)?.compared))
 }
 
-/// The Arm rule's finding, reported: aarch64 client hashes that differ from the server's while their x86-64 re-decode
-/// agrees with it. The hash kind only: the shot view test's and the throw landing's kinds join with the figures that
-/// read them (U4h's view_hit_registered_share, false_hit_share and throw_desync_count), so a run without decoded
-/// hashes is a gap here, never a 0.
+/// The Arm rule's finding, reported: LG aarch64 client hashes that differ from the server's while their x86-64
+/// re-decode agrees with it. The hash kind only: the shot view test's and the throw landing's kinds join with the
+/// figures that read them (U4h's view_hit_registered_share, false_hit_share and throw_desync_count), so a run without
+/// decoded hashes is a gap here, never a 0; NOT-RUN while an Arm-rule mismatch waits for its re-decode.
 fn cross_arch_mismatch_count(run: &Run<'_>, _set: &[&Run<'_>]) -> Result<Ratio, Lack> {
-    Ok(Ratio::int(decoded(run)?.cross_arch))
+    let d = decoded(run)?;
+    match d.waiting {
+        Some(w) => Err(Lack::Waiting(w)),
+        None => Ok(Ratio::int(d.cross_arch)),
+    }
 }
 
 /// PM-0001-1 and PM-0006-11: 1 when every log's header names its machine in full (role, name, plan, arch, cpu,
diff --git a/.gate/u4j-prefix/spikes/netcode-proof/crates/judge/src/marks.rs b/.claude/worktrees/agent-a3b8dfeb01ed52b50/spikes/netcode-proof/crates/judge/src/marks.rs
index d2ba270..913a497 100644
--- a/.gate/u4j-prefix/spikes/netcode-proof/crates/judge/src/marks.rs
+++ b/.claude/worktrees/agent-a3b8dfeb01ed52b50/spikes/netcode-proof/crates/judge/src/marks.rs
@@ -41,6 +41,10 @@ struct RawMark {
     stand_in: bool,
     #[serde(default)]
     arm_rule: bool,
+    /// PM-0001-18's WebSocket bound line, verbatim (README §2.5): only on a mark with `ws_bound` checks, which must
+    /// hold it.
+    #[serde(default)]
+    ws_bound_line: Option<String>,
     #[serde(default)]
     readings: Vec<String>,
     #[serde(default)]
@@ -389,6 +393,10 @@ pub struct Mark {
     pub bites: Vec<Bite>,
     pub stand_in: bool,
     pub arm_rule: bool,
+    /// The WebSocket bound's machine-readable line, verbatim (README §2.5; PM-0001-18): checked at load against the
+    /// mark's `ws_bound` checks and its `ws-practice` tolerance; a practice record's `bound` must equal it byte for
+    /// byte.
+    pub ws_bound_line: Option<String>,
     pub readings: Vec<String>,
     pub notes: String,
 }
@@ -604,17 +612,27 @@ fn compile_check(c: &RawCheck, part_names: &BTreeSet<&str>) -> Result<Check, Str
             if c.parts.is_empty() {
                 return Err("a ws_bound check names its parts (each with ws-practice)".to_owned());
             }
-            match (n, stat, cond) {
-                (BoundNumber::Y, Stat::Count, Some(_)) => {}
-                (BoundNumber::Y, _, _) => {
-                    return Err("ws_bound y is a count with a where (over Y, at most k)".to_owned());
-                }
-                (_, Stat::Count | Stat::Share, _) | (_, _, Some(_)) => {
-                    return Err(
-                        "ws_bound x or z is a limit on a statistic, with no where".to_owned()
-                    );
-                }
-                _ => {}
+            // Each letter's shape is pinned (SEC-F11 on U4j): the judge moves exactly these limits by f.
+            let (shape, want) = match n {
+                BoundNumber::X => (
+                    matches!(stat, Stat::Pct(q) if *q == Ratio::new(1, 2))
+                        && cond.is_none()
+                        && op == Op::Le,
+                    "stat p50, op <= and no where (the median at most X)",
+                ),
+                BoundNumber::Y => (
+                    matches!(stat, Stat::Count)
+                        && matches!(cond, Some(w) if w.op == Op::Gt)
+                        && op == Op::Le,
+                    "stat count, a where with op > and op <= (at most k over Y)",
+                ),
+                BoundNumber::Z => (
+                    matches!(stat, Stat::Max) && cond.is_none() && op == Op::Le,
+                    "stat max, op <= and no where (none over Z)",
+                ),
+            };
+            if !shape {
+                return Err(format!("ws_bound {letter} is {want}; its shape is another"));
             }
             Some(n)
         }
@@ -770,6 +788,40 @@ fn compile_mark(m: &RawMark) -> Result<Mark, Vec<String>> {
             }
         }
     }
+    // The bound's line (the advisor's FX-6): held by a mark with ws_bound checks, and only by one; its X, k, Y and Z are
+    // the checks' and its tolerance every ws-practice rule's, so M_m, the band, the guard, the model and the commit
+    // come from reviewed marks.json, never from a log.
+    let bound_checks = m.checks.iter().any(|c| c.ws_bound.is_some());
+    match (&m.ws_bound_line, bound_checks) {
+        (None, true) => errors.push(
+            "ws_bound_line: a mark with ws_bound checks holds the bound's line (README §2.5)"
+                .to_owned(),
+        ),
+        (Some(_), false) => errors
+            .push("ws_bound_line: only a mark with ws_bound checks holds a bound line".to_owned()),
+        (Some(line), true) => match crate::practice::parse_bound(line) {
+            Err(e) => errors.push(format!("ws_bound_line: {e}")),
+            Ok(bound) => {
+                if let Err(e) = crate::practice::checks_agree(&bound, &checks) {
+                    errors.push(format!("ws_bound_line: {e}"));
+                }
+                for r in validity
+                    .iter()
+                    .chain(parts.iter().flat_map(|p| &p.validity))
+                {
+                    if let Rule::WsPractice { tolerance, .. } = r
+                        && *tolerance != bound.tolerance
+                    {
+                        errors.push(format!(
+                            "ws_bound_line: its tolerance {} is not the ws-practice rule's {tolerance}",
+                            bound.tolerance
+                        ));
+                    }
+                }
+            }
+        },
+        (None, false) => {}
+    }
     // The shape is judged on the file's lists, so a check that failed to compile is reported once, not twice.
     match kind {
         Kind::Pass => {
@@ -847,6 +899,7 @@ fn compile_mark(m: &RawMark) -> Result<Mark, Vec<String>> {
             bites: m.bites.clone(),
             stand_in: m.stand_in,
             arm_rule: m.arm_rule,
+            ws_bound_line: m.ws_bound_line.clone(),
             readings: m.readings.clone(),
             notes: m.notes.clone(),
         })
@@ -1018,19 +1071,88 @@ mod tests {
         assert!(parse(&unknown_field).is_err());
     }
 
+    /// A WebSocket bound line (README §2.5) that agrees with the X, Y and Z checks below and the tolerance 1.10.
+    const LINE: &str = r#"npt-ws-bound v1 {"model":"linux-6.8-tcp v3","commit":"bf9d248000000000000000000000000000000000","runs_count":2000,"client_bursts_count":480,"x_ns":670000000,"y_ns":2500000000,"k_count":5,"z_ns":6900000000,"m_m_ns":654000000,"band_low":"0.965","band_high":"1.035","tolerance":"1.10","guard_share":"0.0000"}"#;
+    const X: &str = r#"{"name":"x","series":"event.burst_heal.after_link_ns","stat":"p50","op":"<=","value":670000000,"parts":["ws"],"ws_bound":"x"}"#;
+    const Y: &str = r#"{"name":"y","series":"event.burst_heal.after_link_ns","stat":"count","where":{"op":">","value":2500000000},"op":"<=","value":5,"parts":["ws"],"ws_bound":"y"}"#;
+    const Z: &str = r#"{"name":"z","series":"event.burst_heal.after_link_ns","stat":"max","op":"<=","value":6900000000,"parts":["ws"],"ws_bound":"z"}"#;
+
+    /// A mark with a WebSocket part judged by `rule`, these checks and, if given, this bound line.
+    fn ws_mark(rule: &str, line: Option<&str>, checks: &str) -> String {
+        let line = line.map_or(String::new(), |l| {
+            format!(
+                r#","ws_bound_line":{}"#,
+                serde_json::to_string(l).expect("a string")
+            )
+        });
+        file(&mark(&format!(
+            r#""verdict":"pass","parts":[{{"name":"ws","select":{{"layer":"L2"}},"min":1,"validity":["{rule}"]}},
+               {{"name":"wt","select":{{"layer":"L1"}},"min":1}}],"checks":[{checks}]{line}"#
+        )))
+    }
+
+    /// The bound's line held verbatim in `ws_bound_line` (the advisor's FX-6 on U4j): a mark with `ws_bound` checks
+    /// holds it and no other mark does, and the mark is refused unless the line's X, k, Y and Z equal its checks' and
+    /// its tolerance the `ws-practice` rule's.
+    #[test]
+    fn ws_bound_line_agrees_with_its_checks() {
+        let all = format!("{X},{Y},{Z}");
+        let good = parse(&ws_mark("ws-practice:1.10", Some(LINE), &all)).expect("compiles");
+        assert_eq!(good.marks[0].ws_bound_line.as_deref(), Some(LINE));
+        let other = |from: &str, to: &str| LINE.replacen(from, to, 1);
+        let plain =
+            r#"{"name":"p","series":"tick.v.w","stat":"max","op":"<=","value":1,"parts":["wt"]}"#;
+        for (text, why) in [
+            (
+                ws_mark(
+                    "ws-practice:1.10",
+                    Some(&other(r#""x_ns":670000000"#, r#""x_ns":680000000"#)),
+                    &all,
+                ),
+                "its X is 680000000, check \"x\" holds 670000000",
+            ),
+            (
+                ws_mark(
+                    "ws-practice:1.10",
+                    Some(&other(r#""tolerance":"1.10""#, r#""tolerance":"1.05""#)),
+                    &all,
+                ),
+                "its tolerance 1.05 is not the ws-practice rule's 1.1",
+            ),
+            (
+                ws_mark("ws-practice:1.10", None, &all),
+                "a mark with ws_bound checks holds the bound's line",
+            ),
+            (
+                ws_mark("ws-practice:1.10", Some(LINE), plain),
+                "only a mark with ws_bound checks holds a bound line",
+            ),
+            (
+                ws_mark(
+                    "ws-practice:1.10",
+                    Some(
+                        &other(r#""band_low":"0.965""#, r#""band_low":"2.0""#).replacen(
+                            r#""band_high":"1.035""#,
+                            r#""band_high":"2.0""#,
+                            1,
+                        ),
+                    ),
+                    &all,
+                ),
+                "a band that is not 0 < low <= 1 <= high",
+            ),
+        ] {
+            let errors = parse(&text).expect_err(why);
+            assert!(errors.iter().any(|e| e.contains(why)), "{why}: {errors:?}");
+        }
+    }
+
     /// PM-0001-18's WebSocket checks (README §2.3 `ws_bound`, §4 `ws-practice`): only on parts the rule judges, one
-    /// per bound number, each in the shape its number needs; the tolerance between 1 and 1.10.
+    /// per bound number, each in the shape its number needs (x a median at most X, y a count over Y at most k, z a
+    /// maximum at most Z: SEC-F11); the tolerance between 1 and 1.10.
     #[test]
     fn ws_bound_checks_keep_their_shape() {
-        let ws = |rule: &str, checks: &str| {
-            file(&mark(&format!(
-                r#""verdict":"pass","parts":[{{"name":"ws","select":{{"layer":"L2"}},"min":1,"validity":["{rule}"]}},
-                   {{"name":"wt","select":{{"layer":"L1"}},"min":1}}],"checks":[{checks}]"#
-            )))
-        };
-        const X: &str = r#"{"name":"x","series":"event.burst_heal.after_link_ns","stat":"p50","op":"<=","value":670000000,"parts":["ws"],"ws_bound":"x"}"#;
-        const Y: &str = r#"{"name":"y","series":"event.burst_heal.after_link_ns","stat":"count","where":{"op":">","value":2500000000},"op":"<=","value":5,"parts":["ws"],"ws_bound":"y"}"#;
-        const Z: &str = r#"{"name":"z","series":"event.burst_heal.after_link_ns","stat":"max","op":"<=","value":6900000000,"parts":["ws"],"ws_bound":"z"}"#;
+        let ws = |rule: &str, checks: &str| ws_mark(rule, Some(LINE), checks);
         let good = parse(&ws("ws-practice:1.10", &format!("{X},{Y},{Z}"))).expect("compiles");
         let m = &good.marks[0];
         assert_eq!(
@@ -1111,6 +1233,28 @@ mod tests {
         for (text, why) in bad {
             assert!(parse(&text).is_err(), "{why} must be refused");
         }
+        // Each letter's shape (SEC-F11): another op or statistic is refused, by its own reason.
+        for (check, why) in [
+            (X.replace(r#""op":"<=""#, r#""op":">=""#), "x with op >="),
+            (
+                X.replace(r#""stat":"p50""#, r#""stat":"max""#),
+                "x with stat max",
+            ),
+            (
+                Z.replace(r#""stat":"max""#, r#""stat":"p50""#),
+                "z with stat p50",
+            ),
+            (
+                Y.replace(r#""where":{"op":">""#, r#""where":{"op":">=""#),
+                "y with a where op >=",
+            ),
+        ] {
+            let errors = parse(&ws("ws-practice:1.10", &check)).expect_err(why);
+            assert!(
+                errors.iter().any(|e| e.contains("its shape is")),
+                "{why}: {errors:?}"
+            );
+        }
         let report = file(&mark(&format!(
             r#""verdict":"reported",{PART},"report":[{{"name":"r","series":"tick.v.w","stat":"max","ws_bound":"x"}}]"#
         )));
diff --git a/.gate/u4j-prefix/spikes/netcode-proof/crates/judge/src/practice.rs b/.claude/worktrees/agent-a3b8dfeb01ed52b50/spikes/netcode-proof/crates/judge/src/practice.rs
index 6b1bb4f..f90baee 100644
--- a/.gate/u4j-prefix/spikes/netcode-proof/crates/judge/src/practice.rs
+++ b/.claude/worktrees/agent-a3b8dfeb01ed52b50/spikes/netcode-proof/crates/judge/src/practice.rs
@@ -4,7 +4,7 @@
 //! the numbers moved by f = max(1, s) only while s is within the tolerance, never set by hand.
 
 use crate::eval::{Run, always_rules, nearest_rank};
-use crate::marks::{BoundNumber, Mark, Measure, Threshold};
+use crate::marks::{BoundNumber, Check, Mark, Measure, Threshold};
 use crate::ratio::Ratio;
 use npt_log::{EventRecord, Phase};
 use serde::Deserialize;
@@ -35,6 +35,10 @@ struct RawBound {
 /// The run-structured bound, as its machine-readable line gives it (README §2.5).
 #[derive(Debug, Clone, PartialEq, Eq)]
 pub(crate) struct Bound {
+    /// The TCP model the bound simulated: the practice run's and the run's `params.ws_tcp_model` (FX-13).
+    pub model: String,
+    /// Client-bursts per simulated run (480 for 48 clients x 10 bursts): a practice run holds exactly this many heals.
+    pub client_bursts_count: u64,
     pub x_ns: i64,
     pub y_ns: i64,
     pub k_count: i64,
@@ -44,6 +48,9 @@ pub(crate) struct Bound {
     pub band_low: Ratio,
     pub band_high: Ratio,
     pub tolerance: Ratio,
+    /// The band and the tolerance as the line writes them (`"0.965"`, `"1.10"`), for the verdict's detail.
+    pub band_text: (String, String),
+    pub tolerance_text: String,
 }
 
 /// Reads the bound's machine-readable line: [`BOUND_TAG`], then one JSON object with exactly README §2.5's fields.
@@ -53,9 +60,18 @@ pub(crate) fn parse_bound(line: &str) -> Result<Bound, String> {
         .ok_or_else(|| format!("the bound line does not start with {BOUND_TAG:?}"))?;
     let raw: RawBound =
         serde_json::from_str(json).map_err(|e| format!("the bound line's object: {e}"))?;
-    let decimal =
-        |name: &str, s: &str| Ratio::parse(s).map_err(|e| format!("the bound line's {name}: {e}"));
+    // At most 6 digits after the point (the advisor's FX-8), so no comparison of these fractions can overflow.
+    let decimal = |name: &str, s: &str| {
+        if s.split_once('.').is_some_and(|(_, f)| f.len() > 6) {
+            return Err(format!(
+                "the bound line's {name}: {s:?} has more than 6 digits after the point"
+            ));
+        }
+        Ratio::parse(s).map_err(|e| format!("the bound line's {name}: {e}"))
+    };
     let b = Bound {
+        model: raw.model.clone(),
+        client_bursts_count: raw.client_bursts_count,
         x_ns: raw.x_ns,
         y_ns: raw.y_ns,
         k_count: raw.k_count,
@@ -64,6 +80,8 @@ pub(crate) fn parse_bound(line: &str) -> Result<Bound, String> {
         band_low: decimal("band_low", &raw.band_low)?,
         band_high: decimal("band_high", &raw.band_high)?,
         tolerance: decimal("tolerance", &raw.tolerance)?,
+        band_text: (raw.band_low.clone(), raw.band_high.clone()),
+        tolerance_text: raw.tolerance.clone(),
     };
     let guard = decimal("guard_share", &raw.guard_share)?;
     let hex = |s: &str| {
@@ -85,8 +103,11 @@ pub(crate) fn parse_bound(line: &str) -> Result<Bound, String> {
         ),
         (b.k_count < 0, "a k below 0"),
         (
-            b.band_low <= Ratio::int(0) || b.band_low > b.band_high,
-            "a band that is not 0 < low <= high",
+            // M_m is the median of the run medians, so a correct build's band holds 1 (the advisor's FX-1).
+            b.band_low <= Ratio::int(0)
+                || b.band_low > Ratio::int(1)
+                || b.band_high < Ratio::int(1),
+            "a band that is not 0 < low <= 1 <= high",
         ),
         (
             b.tolerance < Ratio::int(1) || b.tolerance > Ratio::new(11, 10),
@@ -103,6 +124,40 @@ pub(crate) fn parse_bound(line: &str) -> Result<Bound, String> {
     }
 }
 
+/// The bound's X, k, Y and Z against a mark's `ws_bound` checks (README §2.3): each check holds the bound's number.
+/// Checked when marks.json loads (marks.rs, the advisor's FX-6 (b)) and again by `find`.
+pub(crate) fn checks_agree(bound: &Bound, checks: &[Check]) -> Result<(), String> {
+    for c in checks {
+        let (Some(n), Threshold::Value(value)) = (c.ws_bound, &c.threshold) else {
+            continue;
+        };
+        let (got, want, letter) = match n {
+            BoundNumber::X => (*value, bound.x_ns, "X"),
+            BoundNumber::Z => (*value, bound.z_ns, "Z"),
+            BoundNumber::Y => {
+                // marks.rs gives every ws_bound y check a where: its value is Y, the check's value k.
+                let Measure::Series { cond: Some(w), .. } = &c.measure else {
+                    continue;
+                };
+                if *value != Ratio::int(i128::from(bound.k_count)) {
+                    return Err(format!(
+                        "its k is {}, check {:?} allows {value} over Y",
+                        bound.k_count, c.name
+                    ));
+                }
+                (w.value, bound.y_ns, "Y")
+            }
+        };
+        if got != Ratio::int(i128::from(want)) {
+            return Err(format!(
+                "its {letter} is {want}, check {:?} holds {got}",
+                c.name
+            ));
+        }
+    }
+    Ok(())
+}
+
 /// `v` rounded up to the next whole millisecond (the advisor's R6: each moved number "rounded up to the next
 /// millisecond").
 pub(crate) fn ceil_ms(v: Ratio) -> Ratio {
@@ -116,13 +171,17 @@ pub(crate) fn ceil_ms(v: Ratio) -> Ratio {
     Ratio::int(q * unit)
 }
 
-/// What the ws-practice rule found for a run: its practice run, the bound, and the factor applied.
+/// What the ws-practice rule found for a run: its practice run, the bound, the factor applied, and the record's class
+/// and rulings (printed whole beside the numbers in force: the advisor's FX-11).
 pub(crate) struct Practice {
     pub run: String,
     pub bound: Bound,
     pub m_p_ns: i64,
     pub s: Ratio,
     pub f: Ratio,
+    pub class: String,
+    pub chance_ruling: Option<String>,
+    pub second_ruling: Option<String>,
 }
 
 impl Practice {
@@ -140,18 +199,37 @@ impl Practice {
         ceil_ms(Ratio::int(i128::from(self.bound_of(n))) * self.f)
     }
 
-    /// How the number in force was reached, for the verdict's detail.
+    /// How the number in force was reached, for the verdict's detail: the practice outcome whole, so a run judged
+    /// under a ruling never reads as IN TOLERANCE (the advisor's FX-11).
     pub fn describe(&self, n: BoundNumber) -> String {
-        format!(
-            "{n:?} {} x f {} = {} rounded up to the ms; f = max(1, s), s = M_p {} / M_m {} = {}; practice run {}",
+        let why_f = if self.class == "FAR BETTER" {
+            "FAR BETTER: at the bound"
+        } else {
+            "max(1, s)"
+        };
+        let mut text = format!(
+            "{n:?} {} x f {} = {}, rounded up to the ms; practice run {}: {}, s = M_p {} / M_m {} = {}, band {} to {}, \
+             tolerance {}, f = {} ({why_f})",
             self.bound_of(n),
             self.f,
             self.moved(n),
+            self.run,
+            self.class,
             self.m_p_ns,
             self.bound.m_m_ns,
             self.s,
-            self.run
-        )
+            self.bound.band_text.0,
+            self.bound.band_text.1,
+            self.bound.tolerance_text,
+            self.f,
+        );
+        if let Some(r) = &self.chance_ruling {
+            text.push_str(&format!("; chance ruling: {r:?}"));
+        }
+        if let Some(r) = &self.second_ruling {
+            text.push_str(&format!("; second ruling: {r:?}"));
+        }
+        text
     }
 }
 
@@ -202,6 +280,25 @@ fn spacing(run: &Run<'_>) -> Result<i64, String> {
         .ok_or_else(|| format!("run {} has no integer params.burst_spacing_ns", run.id))
 }
 
+/// RS's `machine.kernel` in a run (its logs with `machine.role` RS, all agreeing), `None` with no RS log or an empty
+/// kernel.
+fn rs_kernel<'r>(run: &'r Run<'_>) -> Result<Option<&'r str>, String> {
+    let kernels: BTreeSet<&str> = run
+        .logs
+        .iter()
+        .filter(|l| l.header.machine.role == "RS")
+        .map(|l| l.header.machine.kernel.as_str())
+        .collect();
+    match kernels.iter().copied().collect::<Vec<_>>().as_slice() {
+        [] | [""] => Ok(None),
+        [k] => Ok(Some(k)),
+        _ => Err(format!(
+            "run {}: its RS logs disagree on machine.kernel",
+            run.id
+        )),
+    }
+}
+
 /// The binaries a run's logs came from.
 fn builds<'r>(run: &'r Run<'_>) -> BTreeSet<&'r str> {
     run.logs
@@ -245,46 +342,43 @@ fn int(e: &EventRecord, name: &str) -> Result<i64, String> {
         .ok_or_else(|| format!("the ws_practice record has no integer {name}"))
 }
 
-/// The practice run whose numbers are in force for `run`, checked whole (README §4, `ws-practice`). A bite run takes
-/// the measured build's practice run of its set (a bites build has none of its own).
-pub(crate) fn find(
-    run: &Run<'_>,
-    set: &[&Run<'_>],
+/// The practice run in force for a counted run, with its one record: of the practice runs built from the counted
+/// run's binaries and started before it, over every run the judge reads (in any set, listing the mark or not), the
+/// latest; with more than one, only if its record has a `second_ruling`; and it must list the mark (README §4,
+/// `ws-practice`; the advisor's R7, R9, FX-2 and FX-3).
+fn practice_of<'r, 'a>(
+    counted: &Run<'_>,
+    all: &'r [Run<'a>],
     mark: &Mark,
-    tolerance: Ratio,
-) -> Result<Practice, String> {
-    let when = started(run)?;
-    let gap = spacing(run)?;
-    let mine = builds(run);
-    let mut candidates: Vec<(&Run<'_>, String)> = Vec::new();
-    for p in set
+) -> Result<(&'r Run<'a>, &'r EventRecord), String> {
+    let when = started(counted)?;
+    let mine = builds(counted);
+    let mut candidates: Vec<(&'r Run<'a>, String)> = Vec::new();
+    for p in all
         .iter()
-        .copied()
-        .filter(|p| p.role == "practice" && p.serves(&mark.id))
+        .filter(|p| p.role == "practice" && builds(p) == mine)
     {
-        if run.role == "bite" || builds(p) == mine {
-            candidates.push((p, started(p)?));
-        }
-    }
-    if run.role == "bite" {
-        let kinds: BTreeSet<BTreeSet<&str>> = candidates.iter().map(|(p, _)| builds(p)).collect();
-        if kinds.len() > 1 {
-            return Err(format!(
-                "set {} holds practice runs of {} builds, so bite run {} cannot tell whose numbers are in force",
-                run.set,
-                kinds.len(),
-                run.id
-            ));
+        // Only a practice run started before the run is a candidate: one started after never changes its verdict
+        // (the advisor's R9 and FX-2).
+        let p_when = started(p)?;
+        if p_when < when {
+            candidates.push((p, p_when));
         }
     }
     candidates.sort_by(|a, b| a.1.cmp(&b.1));
-    let Some((p, p_when)) = candidates.last() else {
+    let Some(&(p, _)) = candidates.last() else {
         return Err(format!(
-            "set {} holds no practice run of run {}'s build (role practice, listing {})",
-            run.set, run.id, mark.id
+            "run {}: no practice run of its build started before it (role practice, listing {})",
+            counted.id, mark.id
         ));
     };
-    let records: Vec<&EventRecord> = p
+    if !p.serves(&mark.id) {
+        return Err(format!(
+            "practice run {}, the latest of run {}'s build, does not list {}",
+            p.id, counted.id, mark.id
+        ));
+    }
+    let records: Vec<&'r EventRecord> = p
         .logs
         .iter()
         .flat_map(|l| &l.events)
@@ -304,6 +398,82 @@ pub(crate) fn find(
             candidates.len()
         ));
     }
+    Ok((p, *record))
+}
+
+/// The measured runs whose practice run a bite run takes (a bites build has none of its own): those of its set that
+/// serve the mark with its layer, profile and `params.transport`, all of one build (the advisor's FX-3 (c)).
+fn measured_of<'r, 'a>(
+    bite: &Run<'_>,
+    all: &'r [Run<'a>],
+    mark: &Mark,
+) -> Result<Vec<&'r Run<'a>>, String> {
+    let (layer, profile) = (&bite.logs[0].header.layer, &bite.logs[0].header.profile);
+    let transport = param(bite, "transport")?;
+    let mut measured = Vec::new();
+    for m in all
+        .iter()
+        .filter(|m| m.set == bite.set && m.role == "measured" && m.serves(&mark.id))
+    {
+        let h = &m.logs[0].header;
+        if &h.layer == layer && &h.profile == profile && param(m, "transport")? == transport {
+            measured.push(m);
+        }
+    }
+    let kinds = measured
+        .iter()
+        .map(|m| builds(m))
+        .collect::<BTreeSet<_>>()
+        .len();
+    match kinds {
+        0 => Err(format!(
+            "set {} holds no measured run serving {} with bite run {}'s layer, profile and transport",
+            bite.set, mark.id, bite.id
+        )),
+        1 => Ok(measured),
+        n => Err(format!(
+            "set {}'s measured runs of bite run {}'s layer, profile and transport come from {n} builds, so it \
+             cannot tell whose numbers are in force",
+            bite.set, bite.id
+        )),
+    }
+}
+
+/// The practice run whose numbers are in force for `run`, checked whole (README §4, `ws-practice`), over every run
+/// the judge reads. A bite run takes the practice run of its set's measured runs (`measured_of`), so it is judged at
+/// its counted run's numbers.
+pub(crate) fn find(
+    run: &Run<'_>,
+    all: &[Run<'_>],
+    mark: &Mark,
+    tolerance: Ratio,
+) -> Result<Practice, String> {
+    let gap = spacing(run)?;
+    let counted = if run.role == "bite" {
+        measured_of(run, all, mark)?
+    } else {
+        vec![run]
+    };
+    let mut chosen: Option<(&Run<'_>, &EventRecord)> = None;
+    for c in &counted {
+        let (p, record) = practice_of(c, all, mark)?;
+        if let Some((q, _)) = chosen
+            && q.id != p.id
+        {
+            return Err(format!(
+                "set {}'s measured runs have practice runs {} and {} in force, so bite run {} cannot tell whose \
+                 numbers are in force",
+                run.set, q.id, p.id, run.id
+            ));
+        }
+        chosen = Some((p, record));
+    }
+    let Some((p, record)) = chosen else {
+        return Err(format!(
+            "run {}: no counted run to take numbers from",
+            run.id
+        ));
+    };
     let broken = always_rules(p);
     if !broken.is_empty() {
         return Err(format!(
@@ -322,56 +492,75 @@ pub(crate) fn find(
     if spacing(p)? != gap {
         return Err(format!("practice run {} ran another burst spacing", p.id));
     }
-    if *p_when >= when {
+    // The record carries the mark's bound line byte for byte (the advisor's FX-6): M_m, the band, the guard, the
+    // model and the commit come from reviewed marks.json, never from a log.
+    let mark_line = mark
+        .ws_bound_line
+        .as_deref()
+        .ok_or_else(|| format!("{} holds no ws_bound_line", mark.id))?;
+    let line = text(record, "bound").ok_or("the ws_practice record has no bound line")?;
+    if line != mark_line {
         return Err(format!(
-            "practice run {} did not start before run {}",
-            p.id, run.id
+            "practice run {}'s record: its bound line is not {}'s ws_bound_line, byte for byte",
+            p.id, mark.id
         ));
     }
-    let line = text(record, "bound").ok_or("the ws_practice record has no bound line")?;
-    let bound = parse_bound(line)?;
+    let bound = parse_bound(mark_line)?;
     if bound.tolerance != tolerance {
         return Err(format!(
             "the bound's tolerance {} is not marks.json's {tolerance}",
             bound.tolerance
         ));
     }
-    let numbers = mark
-        .checks
-        .iter()
-        .filter_map(|c| c.ws_bound.map(|n| (n, c)));
-    for (n, c) in numbers {
-        let value = match &c.threshold {
-            Threshold::Value(v) => *v,
-            Threshold::Against { .. } => continue,
-        };
-        let (got, want) = match n {
-            BoundNumber::X => (value, bound.x_ns),
-            BoundNumber::Z => (value, bound.z_ns),
-            BoundNumber::Y => {
-                // marks.rs gives every ws_bound y check a where: its value is Y, the check's value k.
-                let Measure::Series { cond: Some(w), .. } = &c.measure else {
-                    continue;
-                };
-                if value != Ratio::int(i128::from(bound.k_count)) {
-                    return Err(format!(
-                        "check {:?} allows {value} over Y, the bound's k is {}",
-                        c.name, bound.k_count
-                    ));
-                }
-                (w.value, bound.y_ns)
+    checks_agree(&bound, &mark.checks).map_err(|e| format!("the bound line: {e}"))?;
+    // The practice run ran on the run's setup (the advisor's R7 and FX-13): the same TCP model, the bound's; the
+    // same CONFIG_HZ; the same RS kernel; each present in both runs.
+    let present = |v: Option<&Value>| v.filter(|v| !v.is_null()).cloned();
+    let setups = [
+        (
+            "params.ws_tcp_model",
+            present(param(run, "ws_tcp_model")?),
+            present(param(p, "ws_tcp_model")?),
+        ),
+        (
+            "params.config_hz",
+            present(param(run, "config_hz")?),
+            present(param(p, "config_hz")?),
+        ),
+        (
+            "RS's machine.kernel",
+            rs_kernel(run)?.map(Value::from),
+            rs_kernel(p)?.map(Value::from),
+        ),
+    ];
+    for (what, mine, theirs) in setups {
+        match (mine, theirs) {
+            (None, _) => return Err(format!("run {} has no {what}", run.id)),
+            (_, None) => return Err(format!("practice run {} has no {what}", p.id)),
+            (Some(a), Some(b)) if a != b => {
+                return Err(format!(
+                    "practice run {} ran another {what} than run {}",
+                    p.id, run.id
+                ));
             }
-        };
-        if got != Ratio::int(i128::from(want)) {
-            return Err(format!(
-                "check {:?} holds {got} as the bound's {n:?}, the bound line says {want}",
-                c.name
-            ));
+            _ => {}
         }
     }
+    if param(run, "ws_tcp_model")?.and_then(Value::as_str) != Some(bound.model.as_str()) {
+        return Err(format!(
+            "run {}'s params.ws_tcp_model is not the bound's model {:?}",
+            run.id, bound.model
+        ));
+    }
     let heal = heals(p)?;
-    if heal.is_empty() {
-        return Err(format!("practice run {} has no heal records", p.id));
+    // Exactly the bound's client-bursts (the advisor's R7, "Valid only with 480 heal records"; FX-7).
+    if heal.len() as u64 != bound.client_bursts_count {
+        return Err(format!(
+            "practice run {} has {} heal records, not the bound's {}",
+            p.id,
+            heal.len(),
+            bound.client_bursts_count
+        ));
     }
     let m_p_ns = heal[nearest_rank(heal.len(), Ratio::new(1, 2)) - 1];
     if int(record, "m_p_ns")? != m_p_ns {
@@ -403,13 +592,28 @@ pub(crate) fn find(
             }
         ));
     }
-    let f = if s > Ratio::int(1) { s } else { Ratio::int(1) };
+    // FAR BETTER runs at the bound (the advisor's R8 (v)); IN TOLERANCE and BEYOND at max(1, s). The numbers never
+    // move past the tolerance (Mustafa's WS-BAR cap), whatever the record says.
+    let f = if class == "FAR BETTER" || s <= Ratio::int(1) {
+        Ratio::int(1)
+    } else {
+        s
+    };
+    if f > tolerance {
+        return Err(format!(
+            "practice run {}: f = {f} is above the tolerance {tolerance}: the numbers never move past it",
+            p.id
+        ));
+    }
     let practice = Practice {
         run: p.id.to_owned(),
         bound,
         m_p_ns,
         s,
         f,
+        class: class.to_owned(),
+        chance_ruling: text(record, "chance_ruling").map(str::to_owned),
+        second_ruling: text(record, "second_ruling").map(str::to_owned),
     };
     for (n, field) in [
         (BoundNumber::X, "x_moved_ns"),
@@ -511,6 +715,62 @@ mod tests {
         }
     }
 
+    /// M_m is the median of a correct build's run medians, so the band of their 1st to 99th percentiles holds 1: a
+    /// line whose band leaves 1 out is refused (the advisor's FX-1 on U4j).
+    #[test]
+    fn the_bound_line_refuses_a_band_that_leaves_out_1() {
+        let band = |low: &str, high: &str| {
+            LINE.replacen(
+                r#""band_low":"0.965""#,
+                &format!(r#""band_low":"{low}""#),
+                1,
+            )
+            .replacen(
+                r#""band_high":"1.035""#,
+                &format!(r#""band_high":"{high}""#),
+                1,
+            )
+        };
+        for (low, high) in [("2.0", "2.0"), ("1.01", "1.2"), ("0.5", "0.9")] {
+            assert!(
+                parse_bound(&band(low, high)).is_err(),
+                "band {low} to {high} must be refused"
+            );
+        }
+        assert!(
+            parse_bound(&band("1", "1")).is_ok(),
+            "a band of 1 to 1 holds 1"
+        );
+    }
+
+    /// The four fractions have at most 6 digits after the point, so no comparison of them overflows: the
+    /// test-engineer's probe B (two 29-digit fractions) is an error, never a panic (the advisor's FX-8).
+    #[test]
+    fn the_bound_line_refuses_more_than_6_decimals() {
+        let probe_b = LINE
+            .replacen(
+                r#""band_low":"0.965""#,
+                r#""band_low":"0.12345678901234567890123456789""#,
+                1,
+            )
+            .replacen(
+                r#""band_high":"1.035""#,
+                r#""band_high":"0.99999999999999999999999999999""#,
+                1,
+            );
+        assert!(parse_bound(&probe_b).is_err(), "probe B");
+        for (from, to) in [
+            (r#""band_low":"0.965""#, r#""band_low":"0.9650001""#),
+            (r#""band_high":"1.035""#, r#""band_high":"1.0350001""#),
+            (r#""tolerance":"1.10""#, r#""tolerance":"1.1000000""#),
+            (r#""guard_share":"0.0000""#, r#""guard_share":"0.0000000""#),
+        ] {
+            assert!(parse_bound(&LINE.replacen(from, to, 1)).is_err(), "{to}");
+        }
+        let six = LINE.replacen(r#""band_low":"0.965""#, r#""band_low":"0.965000""#, 1);
+        assert!(parse_bound(&six).is_ok(), "6 digits after the point");
+    }
+
     #[test]
     fn moved_numbers_round_up_to_the_millisecond() {
         assert_eq!(ceil_ms(Ratio::int(704_000_000)), Ratio::int(704_000_000));
````

### npt-judge's tests (crates/judge/tests)
_`git diff --no-index .gate/u4j-prefix/spikes/netcode-proof/crates/judge/tests .claude/worktrees/agent-a3b8dfeb01ed52b50/spikes/netcode-proof/crates/judge/tests`:  2 files changed, 885 insertions(+), 48 deletions(-)._

````diff
diff --git a/.gate/u4j-prefix/spikes/netcode-proof/crates/judge/tests/engine.rs b/.claude/worktrees/agent-a3b8dfeb01ed52b50/spikes/netcode-proof/crates/judge/tests/engine.rs
index df1a9a7..4cc2c19 100644
--- a/.gate/u4j-prefix/spikes/netcode-proof/crates/judge/tests/engine.rs
+++ b/.claude/worktrees/agent-a3b8dfeb01ed52b50/spikes/netcode-proof/crates/judge/tests/engine.rs
@@ -36,6 +36,8 @@ struct LogSpec {
     features: Option<Vec<&'static str>>,
     arch: &'static str,
     binary: &'static str,
+    /// The header's `clients` (every `hs` list holds this many entries: README §1).
+    clients: u32,
 }
 
 impl LogSpec {
@@ -59,6 +61,7 @@ impl LogSpec {
             features: None,
             arch: "x86_64",
             binary: "0000000000000000000000000000000000000000000000000000000000000000",
+            clients: 48,
         }
     }
 
@@ -107,7 +110,7 @@ impl LogSpec {
             profile: self.profile.to_owned(),
             seed: self.seed,
             tick_hz: 30,
-            clients: 48,
+            clients: self.clients,
             scene: "SS".to_owned(),
             process: self.process.to_owned(),
             slot: 0,
@@ -1313,6 +1316,7 @@ fn hash_log(
     let mut s = LogSpec::room(run, "set-h", "PM-0001-17");
     s.layer = "L1";
     s.process = process;
+    s.clients = 2;
     s.ticks = per_tick.iter().map(|_| (Phase::Measured, vec![])).collect();
     edit(&mut s);
     let mut log = s.build();
@@ -1346,6 +1350,11 @@ fn u4j_the_decoded_hash_gaps_and_its_every_phase() {
     let equal = client(&SERVER);
     let (k, why) = verdict_of(&[hash_mark()], &[room.clone(), equal.clone()]);
     assert_eq!(k, VerdictKind::NotRun, "the bite has no run: {why:?}");
+    let incomplete = |list: Vec<i64>| {
+        let mut log = equal.clone();
+        log.ticks[0].s.insert("snap_incomplete".to_owned(), list);
+        log
+    };
     let cases: Vec<(Vec<Log>, &str)> = vec![
         (vec![equal.clone()], "no log gives hs.server_hash"),
         (vec![room.clone()], "no log gives hs.decoded_hash"),
@@ -1354,16 +1363,40 @@ fn u4j_the_decoded_hash_gaps_and_its_every_phase() {
                 room.clone(),
                 client(&[&[Some(11), Some(21), None], &[], &[]]),
             ],
-            "the hash lists' slot counts differ (server 2, decoded 3, replay 3)",
+            "hs.decoded_hash lists hold 3 entries, not the run's 2 clients (the room log's header)",
         ),
         (
             vec![
                 room.clone(),
                 equal.clone(),
-                hash_log("h1", "lab", "decoded_hash", &[&[Some(11), None]], no),
+                hash_log("h1", "client", "decoded_hash", &[&[Some(11), None]], no),
             ],
             "two logs give slot 0 at tick 1 an hs.decoded_hash",
         ),
+        (
+            vec![
+                room.clone(),
+                hash_log("h1", "lab", "decoded_hash", &SERVER, no),
+            ],
+            "hs.decoded_hash in the lab log on RS1 (x86_64): only process client (L1) or bots (L2) writes it",
+        ),
+        (
+            vec![
+                room.clone(),
+                equal.clone(),
+                hash_log("h1", "client", "server_hash", &SERVER, no),
+            ],
+            "hs.server_hash in the client log on RS1 (x86_64): only process room writes it",
+        ),
+        // FX-10 (a) and its malformed entries: a snap_incomplete list of the run's clients entries, each 0 or 1.
+        (
+            vec![room.clone(), incomplete(vec![0, 0, 0])],
+            "tick 1's tick.s.snap_incomplete in the client log on RS1 holds 3 entries, not the run's 2 clients",
+        ),
+        (
+            vec![room.clone(), incomplete(vec![2, 0])],
+            "tick 1's tick.s.snap_incomplete holds 2 for slot 0, not 0 or 1",
+        ),
         (
             vec![
                 hash_log("h1", "room", "server_hash", &[&[Some(0), Some(21)]], no),
@@ -1400,7 +1433,10 @@ fn u4j_the_decoded_hash_gaps_and_its_every_phase() {
 #[test]
 fn u4j_the_arm_rule_counts_an_aarch64_mismatch_only_when_the_x86_64_re_decode_misses_too() {
     let no = &|_: &mut LogSpec| {};
-    let arm = &|s: &mut LogSpec| s.arch = "aarch64";
+    let arm = &|s: &mut LogSpec| {
+        s.arch = "aarch64";
+        s.machine_role = "LG";
+    };
     let room = hash_log("h1", "room", "server_hash", &SERVER, no);
     let wrong: [&[Option<u64>]; 3] = [
         &[Some(11), Some(21)],
@@ -1445,6 +1481,13 @@ fn u4j_the_arm_rule_counts_an_aarch64_mismatch_only_when_the_x86_64_re_decode_mi
     let x86 = hash_log("h1", "client", "decoded_hash", &wrong, no);
     let (k, _) = verdict_of(&[hash_mark()], &[room, x86, replay(&SERVER)]);
     assert_eq!(k, VerdictKind::Fail);
+    // The Arm rule needs the room on x86_64 (the advisor's FX-5 (b)): with an aarch64 room, the LG mismatch counts.
+    let arm_room = hash_log("h1", "room", "server_hash", &SERVER, &|s| {
+        s.arch = "aarch64"
+    });
+    let lg = hash_log("h1", "bots", "decoded_hash", &wrong, arm);
+    let (k, why) = verdict_of(&[hash_mark()], &[arm_room, lg, replay(&SERVER)]);
+    assert_eq!(k, VerdictKind::Fail, "{why:?}");
 }
 
 #[test]
@@ -1588,6 +1631,11 @@ fn u4j_the_twin_hash_compares_every_tick_and_its_twin_whole() {
     bites.features = Some(vec!["bites"]);
     let mut cut = hashed("f1-twin", "twin", None, &h);
     cut.footer = false;
+    let mut profile = hashed("f1-twin", "twin", None, &h);
+    profile.profile = "P3";
+    let mut other_bite = hashed("f1-twin", "twin", None, &h);
+    other_bite.bite = Some("PM-0001-21/other");
+    let measured = hashed("f1-twin", "measured", Some("f1"), &h);
     let own = || with_hashes(hashed("f1", "measured", Some("f1-twin"), &h), &h);
     let mut bad_h = own();
     bad_h.ticks[1].h = Some("not-a-hash".to_owned());
@@ -1609,6 +1657,18 @@ fn u4j_the_twin_hash_compares_every_tick_and_its_twin_whole() {
             vec![with_hashes(cut, &h), own()],
             "its twin f1-twin is invalid: complete:",
         ),
+        (
+            vec![with_hashes(profile, &h), own()],
+            "its twin f1-twin has another profile",
+        ),
+        (
+            vec![with_hashes(other_bite, &h), own()],
+            "its twin f1-twin has another bite",
+        ),
+        (
+            vec![with_hashes(measured, &h), own()],
+            "run f1: f1-twin has role measured, not twin",
+        ),
         (
             vec![with_hashes(hashed("f1", "measured", Some("f1"), &h), &h)],
             "params.twin names the run itself",
@@ -1658,7 +1718,7 @@ fn u4j_machine_named_needs_every_field_and_a_vm_its_address() {
         "not a VM: no address"
     );
     type Edit<'a> = &'a dyn Fn(&mut npt_log::Machine);
-    let missing: [(&str, Edit<'_>); 8] = [
+    let missing: [(&str, Edit<'_>); 14] = [
         ("a VM with no address", &|m| m.address.clear()),
         ("no kernel", &|m| m.kernel.clear()),
         ("no CPU model", &|m| m.cpu.clear()),
@@ -1667,6 +1727,13 @@ fn u4j_machine_named_needs_every_field_and_a_vm_its_address() {
         ("no SMT map", &|m| m.smt.clear()),
         ("an empty sibling list", &|m| m.smt.push(vec![])),
         ("no clock resolution", &|m| m.clock_res_ns = 0),
+        // The advisor's FX-14 (TE-F13): every other condition, each alone.
+        ("no role", &|m| m.role.clear()),
+        ("no name", &|m| m.name.clear()),
+        ("no plan", &|m| m.plan.clear()),
+        ("no arch", &|m| m.arch.clear()),
+        ("no memory", &|m| m.mem_mb = 0),
+        ("no threads per core", &|m| m.threads_per_core = 0),
     ];
     for (why, edit) in missing {
         assert_eq!(run("RS", edit), VerdictKind::Fail, "{why}");
@@ -1684,7 +1751,9 @@ fn real_marks() -> npt_judge::Marks {
     parse_marks(&text).expect("marks.json compiles")
 }
 
-const WS_BOUND: &str = r#"npt-ws-bound v1 {"model":"linux-6.8-tcp v3","commit":"bf9d248000000000000000000000000000000000","runs_count":2000,"client_bursts_count":480,"x_ns":670000000,"y_ns":2500000000,"k_count":5,"z_ns":6900000000,"m_m_ns":654000000,"band_low":"0.965","band_high":"1.035","tolerance":"1.10","guard_share":"0.0000"}"#;
+/// marks.json's PM-0001-18 `ws_bound_line`, the provisional line (the advisor's FX-6 (d)): a record carries it byte for
+/// byte.
+const WS_BOUND: &str = r#"npt-ws-bound v1 {"model":"provisional: the single-burst bound of bf9d248 (fixes.rs ws_bar_fix1), not the run-structured bound","commit":"bf9d24807106c4524d1aa0849409d219259c3943","runs_count":2000,"client_bursts_count":480,"x_ns":670000000,"y_ns":2500000000,"k_count":5,"z_ns":6900000000,"m_m_ns":654000000,"band_low":"0.965","band_high":"1.035","tolerance":"1.10","guard_share":"0.0000"}"#;
 
 /// One WebSocket B1 run of 480 client-bursts: `low` at 600 ms, the 240th (the median) at `median`, then `high` at
 /// 700 ms, and `slow` at `slow_ns`.
@@ -1696,6 +1765,24 @@ fn heals(median: i64, slow: usize, slow_ns: i64) -> Vec<i64> {
     v
 }
 
+/// The bound's model, as marks.json's provisional `ws_bound_line` names it (FX-6 (d)).
+const WS_MODEL: &str = "provisional: the single-burst bound of bf9d248 (fixes.rs ws_bar_fix1), not the run-structured bound";
+
+/// A run's setup the ws-practice rule compares (the advisor's FX-13): RS's `machine.kernel` (`None`: no RS log in the
+/// run), `params.config_hz` and `params.ws_tcp_model` (`None`: left out).
+#[derive(Clone)]
+struct Setup {
+    kernel: Option<&'static str>,
+    config_hz: Option<i64>,
+    model: Option<&'static str>,
+}
+
+const SETUP: Setup = Setup {
+    kernel: Some("6.8.0-48-generic"),
+    config_hz: Some(250),
+    model: Some(WS_MODEL),
+};
+
 /// Everything a ws-practice scenario changes.
 #[derive(Clone)]
 struct Ws {
@@ -1710,6 +1797,9 @@ struct Ws {
     records: usize,
     counted_started: Option<&'static str>,
     second_practice: bool,
+    /// The counted run's setup (the bite run's too) and the practice run's.
+    counted_setup: Setup,
+    practice_setup: Setup,
 }
 
 /// The base: M_p 680 ms against M_m 654 ms, so s = f = 680/654; the numbers in force are 697 ms, 2.6 s and 7.175 s
@@ -1743,6 +1833,8 @@ fn ws_base() -> Ws {
         records: 1,
         counted_started: Some("2026-11-20T21:00:00Z"),
         second_practice: false,
+        counted_setup: SETUP,
+        practice_setup: SETUP,
     }
 }
 
@@ -1779,58 +1871,115 @@ fn ws_log(
     s
 }
 
+/// One WebSocket B1 run: LG's bots log (`edit` applied to it) with the setup's params, and RS's room log carrying the
+/// setup's kernel when it names one.
+fn ws_run(
+    run: &'static str,
+    role: &'static str,
+    binary: &'static str,
+    started: Option<&'static str>,
+    heals: &[i64],
+    setup: &Setup,
+    edit: &dyn Fn(&mut LogSpec),
+) -> Vec<Log> {
+    let mut bots = ws_log(run, role, binary, started, heals);
+    if let Some(m) = setup.model {
+        bots.params.push(("ws_tcp_model", json!(m)));
+    }
+    if let Some(hz) = setup.config_hz {
+        bots.params.push(("config_hz", json!(hz)));
+    }
+    edit(&mut bots);
+    let bite = bots.bite;
+    let mut logs = vec![bots.build()];
+    if let Some(k) = setup.kernel {
+        let mut rs = LogSpec::room(run, "set-ws", "PM-0001-18");
+        rs.role = role;
+        rs.profile = "B1";
+        rs.binary = binary;
+        rs.bite = bite;
+        rs.params = vec![("transport", json!("ws"))];
+        let mut log = rs.build();
+        log.header.machine.kernel = k.to_owned();
+        logs.push(log);
+    }
+    logs
+}
+
+/// Applies `edit` to every log of run `id`.
+fn edit_run(logs: &mut [Log], id: &str, edit: &dyn Fn(&mut Log)) {
+    for l in logs.iter_mut().filter(|l| l.header.run == id) {
+        edit(l);
+    }
+}
+
 fn ws_logs(w: &Ws) -> Vec<Log> {
     let binary = |c: &str| -> &'static str { if c == "a" { "aa" } else { "bb" } };
-    let mut practice = ws_log(
+    let mut practice = ws_run(
         "ws-practice",
         "practice",
         binary(w.practice_binary),
         Some(w.practice_started),
         &w.practice,
+        &w.practice_setup,
+        &|s| {
+            s.footer = w.practice_footer;
+            s.params[1] = ("burst_spacing_ns", json!(w.practice_spacing));
+            for _ in 0..w.records {
+                let mut e = EventRecord::new("ws_practice", Phase::After);
+                for (k, v) in &w.record {
+                    e = e.with(k, v.clone());
+                }
+                s.events.push(e);
+            }
+        },
     );
-    practice.footer = w.practice_footer;
-    practice.params[1] = ("burst_spacing_ns", json!(w.practice_spacing));
-    for _ in 0..w.records {
-        let mut e = EventRecord::new("ws_practice", Phase::After);
-        for (k, v) in &w.record {
-            e = e.with(k, v.clone());
-        }
-        practice.events.push(e);
-    }
-    let mut practice = practice.build();
-    practice.header.scene = w.practice_scene.to_owned();
-    let mut logs = vec![
-        ws_log("ws-1", "measured", "aa", w.counted_started, &w.counted).build(),
-        practice,
-    ];
+    edit_run(&mut practice, "ws-practice", &|l| {
+        l.header.scene = w.practice_scene.to_owned();
+    });
+    let mut logs = ws_run(
+        "ws-1",
+        "measured",
+        "aa",
+        w.counted_started,
+        &w.counted,
+        &w.counted_setup,
+        &|_| {},
+    );
+    logs.extend(practice.clone());
     if w.second_practice {
-        let mut p = logs[1].clone();
-        p.header.run = "ws-practice-0".to_owned();
-        p.header
-            .params
-            .insert("started_utc".to_owned(), json!("2026-11-20T17:00:00Z"));
-        logs.push(p);
+        logs.extend(another_practice(
+            &practice,
+            "ws-practice-0",
+            "2026-11-20T17:00:00Z",
+            &[],
+        ));
     }
-    let mut bite = ws_log(
+    logs.extend(ws_run(
         "ws-bite",
         "bite",
         "cc",
         Some("2026-11-20T22:00:00Z"),
         &heals(776_000_000, 0, 0),
-    );
-    bite.bite = Some("PM-0001-18/ws-budget-x2");
-    logs.push(bite.build());
+        &w.counted_setup,
+        &|s| s.bite = Some("PM-0001-18/ws-budget-x2"),
+    ));
     logs
 }
 
 fn ws_part(w: &Ws) -> (npt_judge::eval::PartOut, npt_judge::Verdict) {
+    ws_judge(&ws_logs(w))
+}
+
+/// -18 judged on these logs with the real marks.json: the WebSocket part and the mark's verdict.
+fn ws_judge(logs: &[Log]) -> (npt_judge::eval::PartOut, npt_judge::Verdict) {
     let marks = real_marks();
     let i = marks
         .marks
         .iter()
         .position(|m| m.id == "PM-0001-18")
         .expect("-18");
-    let v = judge(&marks, &ws_logs(w)).remove(i);
+    let v = judge(&marks, logs).remove(i);
     let part = v
         .parts
         .iter()
@@ -1862,6 +2011,15 @@ fn u4j_ws_practice_puts_the_practice_runs_numbers_in_force() {
         "{}",
         median.needs
     );
+    // The practice outcome printed whole (the advisor's FX-11, R8 (v): "the ruling is in the run's report").
+    for text in [
+        "practice run ws-practice: IN TOLERANCE",
+        "band 0.965 to 1.035",
+        "tolerance 1.10",
+        "f = 1.039755 (max(1, s))",
+    ] {
+        assert!(median.needs.contains(text), "{text}: {}", median.needs);
+    }
     let over = checks
         .iter()
         .find(|c| c.name.starts_with("WebSocket: at most"))
@@ -1913,6 +2071,24 @@ fn u4j_ws_practice_puts_the_practice_runs_numbers_in_force() {
         json!("advisor-r3.md:40: the cause found and fixed"),
     ));
     assert_eq!(ws_part(&w).0.verdict, VerdictKind::Pass);
+    // BEYOND under a chance ruling never reads as IN TOLERANCE: the class and the ruling's text are printed.
+    let mut w = ws_base();
+    w.record[0].1 = json!("BEYOND");
+    w.record
+        .push(("chance_ruling", json!("advisor-r3.md:20: a lone count")));
+    let (part, _) = ws_part(&w);
+    assert_eq!(part.verdict, VerdictKind::Pass, "{:?}", part.why);
+    let median = part.runs[0]
+        .checks
+        .iter()
+        .find(|c| c.name.starts_with("WebSocket: half"))
+        .expect("check c");
+    for text in [
+        "practice run ws-practice: BEYOND",
+        "chance ruling: \"advisor-r3.md:20: a lone count\"",
+    ] {
+        assert!(median.needs.contains(text), "{text}: {}", median.needs);
+    }
 }
 
 #[test]
@@ -1933,11 +2109,11 @@ fn u4j_ws_practice_refuses_every_number_not_set_by_its_rules() {
     let cases: Vec<(Edit, &str)> = vec![
         (
             Box::new(|w| w.practice_binary = "b"),
-            "holds no practice run of run ws-1's build",
+            "run ws-1: no practice run of its build started before it",
         ),
         (
             Box::new(|w| w.practice_started = "2026-11-20T22:00:00Z"),
-            "did not start before run ws-1",
+            "run ws-1: no practice run of its build started before it",
         ),
         (
             Box::new(|w| w.counted_started = None),
@@ -1984,26 +2160,86 @@ fn u4j_ws_practice_refuses_every_number_not_set_by_its_rules() {
             Box::new(field("k_count", json!(6))),
             "the record's k_count is not the bound's",
         ),
+        // A record's bound line other than marks.json's, in any field (FX-6: these numbers' agreement with the
+        // checks and the rule is checked at load, marks.rs ws_bound_line_agrees_with_its_checks).
         (
             Box::new(bound(r#""tolerance":"1.10""#, r#""tolerance":"1.05""#)),
-            "the bound's tolerance 1.05 is not marks.json's 1.1",
+            "its bound line is not PM-0001-18's ws_bound_line, byte for byte",
         ),
         (
             Box::new(bound(r#""x_ns":670000000"#, r#""x_ns":680000000"#)),
-            "holds 670000000 as the bound's X, the bound line says 680000000",
+            "its bound line is not PM-0001-18's ws_bound_line, byte for byte",
         ),
         (
             Box::new(bound(r#""k_count":5"#, r#""k_count":4"#)),
-            "allows 5 over Y, the bound's k is 4",
+            "its bound line is not PM-0001-18's ws_bound_line, byte for byte",
         ),
         (
             Box::new(bound("npt-ws-bound v1 ", "npt-ws-bound ")),
-            "the bound line does not start with",
+            "its bound line is not PM-0001-18's ws_bound_line, byte for byte",
         ),
         (
             Box::new(|w| w.practice = heals(680_000_000, 6, 2_610_000_000)),
             "but practice run ws-practice has 6 heals over 2600000000 ns",
         ),
+        // FX-7 (the advisor's R7: "Valid only with 480 heal records"; probe G): one heal short or over, M_p unchanged.
+        (
+            Box::new(|w| {
+                let i = w
+                    .practice
+                    .iter()
+                    .position(|h| *h == 700_000_000)
+                    .expect("a 700 ms heal");
+                w.practice.remove(i);
+            }),
+            "practice run ws-practice has 479 heal records, not the bound's 480",
+        ),
+        (
+            Box::new(|w| w.practice.insert(0, 600_000_000)),
+            "practice run ws-practice has 481 heal records, not the bound's 480",
+        ),
+        // FX-13 (the advisor's R7: "a header naming the model, RS's kernel and CONFIG_HZ"): the practice run ran on
+        // the counted run's setup, its model the bound's.
+        (
+            Box::new(|w| w.practice_setup.kernel = Some("6.8.0-49-generic")),
+            "practice run ws-practice ran another RS's machine.kernel than run ws-1",
+        ),
+        (
+            Box::new(|w| w.practice_setup.config_hz = Some(1000)),
+            "practice run ws-practice ran another params.config_hz than run ws-1",
+        ),
+        (
+            Box::new(|w| w.practice_setup.model = Some("linux-6.8-tcp v3")),
+            "practice run ws-practice ran another params.ws_tcp_model than run ws-1",
+        ),
+        (
+            Box::new(|w| {
+                w.practice_setup.model = Some("linux-6.8-tcp v3");
+                w.counted_setup.model = Some("linux-6.8-tcp v3");
+            }),
+            "run ws-1's params.ws_tcp_model is not the bound's model",
+        ),
+        (
+            Box::new(|w| {
+                w.practice_setup.kernel = None;
+                w.counted_setup.kernel = None;
+            }),
+            "run ws-1 has no RS's machine.kernel",
+        ),
+        (
+            Box::new(|w| {
+                w.practice_setup.config_hz = None;
+                w.counted_setup.config_hz = None;
+            }),
+            "run ws-1 has no params.config_hz",
+        ),
+        (
+            Box::new(|w| {
+                w.practice_setup.model = None;
+                w.counted_setup.model = None;
+            }),
+            "run ws-1 has no params.ws_tcp_model",
+        ),
         (
             Box::new(|w| {
                 // M_p 740 ms: s = 740/654 = 1.1315, beyond the tolerance (the 240th of 480 heals, 750 ms above it).
@@ -2036,3 +2272,402 @@ fn u4j_ws_practice_refuses_every_number_not_set_by_its_rules() {
     w.record.push(("chance_ruling", json!("advisor-r3.md:20")));
     assert_eq!(ws_part(&w).0.verdict, VerdictKind::Pass);
 }
+
+/// The record's field `name` set to `value`.
+fn set_field(w: &mut Ws, name: &str, value: Value) {
+    for f in &mut w.record {
+        if f.0 == name {
+            f.1 = value.clone();
+        }
+    }
+}
+
+/// The test-engineer's probe A (the advisor's FX-1, the High): a bound line whose band is 2.0 to 2.0 and a FAR BETTER
+/// record with a chance ruling at s = 1.9 would move X, Y and Z by 90 % (X' 1.273 s), past Mustafa's 10 %. The
+/// numbers never move past the tolerance: the run is INVALID, and its 1.2 s median never passes.
+#[test]
+fn u4j_ws_far_better_past_the_cap_is_invalid() {
+    let mut w = ws_base();
+    let mut counted = vec![600_000_000; 239];
+    counted.push(1_200_000_000);
+    counted.extend(std::iter::repeat_n(1_250_000_000, 240));
+    w.counted = counted;
+    let mut practice = vec![600_000_000; 239];
+    practice.push(1_242_600_000);
+    practice.extend(std::iter::repeat_n(1_300_000_000, 240));
+    w.practice = practice;
+    let line = WS_BOUND
+        .replacen(r#""band_low":"0.965""#, r#""band_low":"2.0""#, 1)
+        .replacen(r#""band_high":"1.035""#, r#""band_high":"2.0""#, 1);
+    for (name, value) in [
+        ("class", json!("FAR BETTER")),
+        ("m_p_ns", json!(1_242_600_000)),
+        ("bound", json!(line)),
+        ("x_moved_ns", json!(1_273_000_000)),
+        ("y_moved_ns", json!(4_750_000_000_i64)),
+        ("z_moved_ns", json!(13_110_000_000_i64)),
+    ] {
+        set_field(&mut w, name, value);
+    }
+    w.record.push(("chance_ruling", json!("x")));
+    let (part, v) = ws_part(&w);
+    assert_eq!(part.verdict, VerdictKind::Invalid, "{:?}", part.why);
+    assert!(says(&part.why, "ws-practice: "), "{:?}", part.why);
+    assert_eq!(v.verdict, VerdictKind::Invalid, "{:?}", v.why);
+}
+
+/// 480 practice heals with the median (the 240th) at `median`: 239 at 600 ms below it, then `rest` above it and
+/// `slow` at `slow_ns`.
+fn practice_heals(median: i64, rest: i64, slow: usize, slow_ns: i64) -> Vec<i64> {
+    let mut v = vec![600_000_000; 239];
+    v.push(median);
+    v.extend(std::iter::repeat_n(rest, 240 - slow));
+    v.extend(std::iter::repeat_n(slow_ns, slow));
+    v
+}
+
+/// The advisor's boundaries on the outcome record (advisor-ws-docs-r2.md:378: "s = 1.00, 1.10 and 1.1001"; FX-9 and
+/// the test-engineer's T1a, T1b, T2, T3): Mustafa's 10 % cap, band_low and the Z' recheck, each pinned.
+#[test]
+fn u4j_ws_the_tolerance_and_band_boundaries() {
+    let needs = |part: &npt_judge::eval::PartOut, prefix: &str| {
+        part.runs[0]
+            .checks
+            .iter()
+            .find(|c| c.name.starts_with(prefix))
+            .map(|c| c.needs.clone())
+            .unwrap_or_default()
+    };
+    let record = |w: &mut Ws, m_p: i64, moved: [i64; 3]| {
+        set_field(w, "m_p_ns", json!(m_p));
+        set_field(w, "x_moved_ns", json!(moved[0]));
+        set_field(w, "y_moved_ns", json!(moved[1]));
+        set_field(w, "z_moved_ns", json!(moved[2]));
+    };
+    // s = 1.00: the bound itself, f = 1.
+    let mut w = ws_base();
+    w.practice = practice_heals(654_000_000, 700_000_000, 5, 2_590_000_000);
+    w.counted = heals(660_000_000, 5, 2_550_000_000);
+    record(
+        &mut w,
+        654_000_000,
+        [670_000_000, 2_500_000_000, 6_900_000_000],
+    );
+    let (part, _) = ws_part(&w);
+    assert_eq!(part.verdict, VerdictKind::Pass, "s = 1.00: {:?}", part.why);
+    let c = needs(&part, "WebSocket: half");
+    assert!(
+        c.starts_with("<= 670000000 (X 670000000 x f 1 = 670000000"),
+        "s = 1.00: {c}"
+    );
+    // s = 1.10 (T1a): at the cap, 0.737 s, 2.75 s and 7.59 s.
+    let mut w = ws_base();
+    w.practice = practice_heals(719_400_000, 750_000_000, 5, 2_590_000_000);
+    record(
+        &mut w,
+        719_400_000,
+        [737_000_000, 2_750_000_000, 7_590_000_000],
+    );
+    let (part, _) = ws_part(&w);
+    assert_eq!(part.verdict, VerdictKind::Pass, "s = 1.10: {:?}", part.why);
+    for (prefix, start) in [
+        (
+            "WebSocket: half",
+            "<= 737000000 (X 670000000 x f 1.1 = 737000000",
+        ),
+        (
+            "WebSocket: at most",
+            "<= 5 (Y 2500000000 x f 1.1 = 2750000000",
+        ),
+        (
+            "WebSocket: no client-burst",
+            "<= 7590000000 (Z 6900000000 x f 1.1 = 7590000000",
+        ),
+    ] {
+        let c = needs(&part, prefix);
+        assert!(c.starts_with(start), "s = 1.10: {c}");
+    }
+    // The three refusals, each judged and collected, so a plant that loosens all three shows each by name.
+    // s = 1.1001 (T1b): just past the cap, its record's moved numbers the judge's at that f: INVALID.
+    let mut t1b = ws_base();
+    t1b.practice = practice_heals(719_465_400, 750_000_000, 5, 2_590_000_000);
+    record(
+        &mut t1b,
+        719_465_400,
+        [738_000_000, 2_751_000_000, 7_591_000_000],
+    );
+    // IN TOLERANCE at s = 0.95, below band_low 0.965 (T2): INVALID, though the counted run passes at the bound.
+    let mut t2 = ws_base();
+    t2.practice = practice_heals(621_300_000, 650_000_000, 0, 0);
+    t2.counted = heals(660_000_000, 5, 2_550_000_000);
+    record(
+        &mut t2,
+        621_300_000,
+        [670_000_000, 2_500_000_000, 6_900_000_000],
+    );
+    // One practice heal over Z' = 7.175 s only (T3; 1 over Y', within k): IN TOLERANCE does not hold.
+    let mut t3 = ws_base();
+    t3.practice = heals(680_000_000, 1, 7_200_000_000);
+    let mut wrong = Vec::new();
+    for (case, w, expected) in [
+        ("T1b", &t1b, "is \"IN TOLERANCE\" at s = 1.1001"),
+        ("T2", &t2, "is \"IN TOLERANCE\" at s = 0.95"),
+        (
+            "T3",
+            &t3,
+            "has 1 heals over 2600000000 ns and its slowest is 7200000000 ns",
+        ),
+    ] {
+        let (part, _) = ws_part(w);
+        if part.verdict != VerdictKind::Invalid || !says(&part.why, expected) {
+            wrong.push(format!("{case}: {:?} {:?}", part.verdict, part.why));
+        }
+    }
+    assert!(wrong.is_empty(), "{wrong:#?}");
+}
+
+/// A copy of the scenario's practice run (every log of run ws-practice) under another id, started at `started`, its
+/// record given `extra`.
+fn another_practice(logs: &[Log], id: &str, started: &str, extra: &[(&str, Value)]) -> Vec<Log> {
+    let mut out: Vec<Log> = logs
+        .iter()
+        .filter(|l| l.header.run == "ws-practice")
+        .cloned()
+        .collect();
+    for p in &mut out {
+        p.header.run = id.to_owned();
+        if p.header.params.contains_key("started_utc") {
+            p.header
+                .params
+                .insert("started_utc".to_owned(), json!(started));
+        }
+        for e in p.events.iter_mut().filter(|e| e.ev == "ws_practice") {
+            for (k, v) in extra {
+                e.f.insert((*k).to_owned(), v.clone());
+            }
+        }
+    }
+    out
+}
+
+/// The advisor's R9 ("a counted WebSocket run that fails stands as FAIL") and FX-2: a practice run started after the
+/// counted run (the test-engineer's probes E1 and E2) is no candidate, so it never turns the FAIL into INVALID.
+#[test]
+fn u4j_ws_a_practice_run_after_the_counted_run_changes_nothing() {
+    let mut w = ws_base();
+    w.counted = heals(700_000_000, 6, 2_550_000_000);
+    let logs = ws_logs(&w);
+    let (part, _) = ws_judge(&logs);
+    assert_eq!(part.verdict, VerdictKind::Fail, "E0: {:?}", part.why);
+    assert!(
+        says(
+            &part.why,
+            "WebSocket: half the client-bursts heal within X x f"
+        ),
+        "E0: {:?}",
+        part.why
+    );
+    for (case, extra) in [
+        (
+            "E1",
+            vec![(
+                "second_ruling",
+                json!("advisor-r3.md:40: the cause found and fixed"),
+            )],
+        ),
+        ("E2", vec![]),
+    ] {
+        let mut later = logs.clone();
+        later.extend(another_practice(
+            &logs,
+            "ws-practice-2",
+            "2026-11-20T23:00:00Z",
+            &extra,
+        ));
+        let (part, _) = ws_judge(&later);
+        assert_eq!(part.verdict, VerdictKind::Fail, "{case}: {:?}", part.why);
+        assert!(
+            says(
+                &part.why,
+                "WebSocket: half the client-bursts heal within X x f"
+            ),
+            "{case}: {:?}",
+            part.why
+        );
+    }
+}
+
+/// The advisor's R7 ("one practice run per build") over every run the judge reads (FX-3 (a), (b); the test-engineer's
+/// probe F): a second practice run of the build in another set counts, listing the mark or not, so with no
+/// second_ruling the run is INVALID; the latest must list the mark; a practice run in another set alone is in force.
+#[test]
+fn u4j_ws_one_practice_run_per_build_over_every_set() {
+    let logs = ws_logs(&ws_base());
+    let elsewhere = |listed: bool| {
+        let mut p = another_practice(
+            &logs,
+            "ws-practice-0",
+            "2026-11-20T17:00:00Z",
+            &[("class", json!("BEYOND"))],
+        );
+        edit_run(&mut p, "ws-practice-0", &|l| {
+            l.header.set = "set-ws-0".to_owned();
+            if !listed {
+                l.header.marks.clear();
+            }
+        });
+        p
+    };
+    for listed in [true, false] {
+        let mut with = logs.clone();
+        with.extend(elsewhere(listed));
+        let (part, _) = ws_judge(&with);
+        assert_eq!(
+            part.verdict,
+            VerdictKind::Invalid,
+            "listed {listed}: {:?}",
+            part.why
+        );
+        assert!(
+            says(&part.why, "2 practice runs of one build"),
+            "listed {listed}: {:?}",
+            part.why
+        );
+    }
+    // The practice run, in another set, is the only one of the build: it is in force.
+    let mut moved = logs.clone();
+    edit_run(&mut moved, "ws-practice", &|l| {
+        l.header.set = "set-ws-0".to_owned();
+    });
+    let (part, _) = ws_judge(&moved);
+    assert_eq!(part.verdict, VerdictKind::Pass, "{:?}", part.why);
+    // The latest practice run of the build does not list the mark: INVALID.
+    let mut unlisted = logs.clone();
+    edit_run(&mut unlisted, "ws-practice", &|l| l.header.marks.clear());
+    let (part, _) = ws_judge(&unlisted);
+    assert_eq!(part.verdict, VerdictKind::Invalid, "{:?}", part.why);
+    assert!(
+        says(&part.why, "does not list PM-0001-18"),
+        "{:?}",
+        part.why
+    );
+}
+
+/// The advisor's FX-6 (the test-engineer's probe H): M_m, the band, the guard, the model and the commit come from
+/// marks.json's `ws_bound_line`, never from a log. A record whose bound line says M_m 620 ms (its moved numbers
+/// recomputed to match) would let a 730 ms median pass at 735 ms; a record whose line differs only in its commit is
+/// refused too.
+#[test]
+fn u4j_ws_the_record_carries_the_marks_bound_line() {
+    let mut w = ws_base();
+    let mut counted = vec![600_000_000; 239];
+    counted.push(730_000_000);
+    counted.extend(std::iter::repeat_n(750_000_000, 234));
+    counted.extend(std::iter::repeat_n(2_550_000_000, 6));
+    w.counted = counted;
+    let (part, _) = ws_part(&w);
+    assert_eq!(part.verdict, VerdictKind::Fail, "H0: {:?}", part.why);
+    let mut h1 = w.clone();
+    for (name, value) in [
+        (
+            "bound",
+            json!(WS_BOUND.replacen(r#""m_m_ns":654000000"#, r#""m_m_ns":620000000"#, 1)),
+        ),
+        ("x_moved_ns", json!(735_000_000)),
+        ("y_moved_ns", json!(2_742_000_000_i64)),
+        ("z_moved_ns", json!(7_568_000_000_i64)),
+    ] {
+        set_field(&mut h1, name, value);
+    }
+    let (part, _) = ws_part(&h1);
+    assert_eq!(part.verdict, VerdictKind::Invalid, "H1: {:?}", part.why);
+    assert!(
+        says(&part.why, "is not PM-0001-18's ws_bound_line"),
+        "H1: {:?}",
+        part.why
+    );
+    let mut commit = ws_base();
+    set_field(
+        &mut commit,
+        "bound",
+        json!(WS_BOUND.replacen(r#""commit":"bf9d248"#, r#""commit":"ff9d248"#, 1)),
+    );
+    let (part, _) = ws_part(&commit);
+    assert_eq!(part.verdict, VerdictKind::Invalid, "commit: {:?}", part.why);
+    assert!(
+        says(&part.why, "is not PM-0001-18's ws_bound_line"),
+        "commit: {:?}",
+        part.why
+    );
+}
+
+/// The ws-budget-x2 bite run of -18's WebSocket part: its run outcome.
+fn ws_bite(v: &npt_judge::Verdict) -> &npt_judge::eval::BiteOut {
+    v.bites
+        .iter()
+        .find(|b| b.id == "PM-0001-18/ws-budget-x2")
+        .expect("the WebSocket bite")
+}
+
+/// FX-3 (c) and TE-F15: a bite run takes the practice run in force for its set's measured runs that serve the mark
+/// with its layer, profile and transport (one build among them), so it is judged at its counted run's f.
+#[test]
+fn u4j_ws_a_bite_run_takes_its_counted_runs_practice_run() {
+    let logs = ws_logs(&ws_base());
+    let check_c = |r: &npt_judge::eval::RunOut| {
+        r.checks
+            .iter()
+            .find(|c| c.name.starts_with("WebSocket: half"))
+            .map(|c| c.needs.clone())
+            .expect("check c")
+    };
+    // (1) The same f as the measured run's.
+    let (part, v) = ws_judge(&logs);
+    let bite = ws_bite(&v);
+    assert_eq!(bite.status, BiteStatus::Shown, "{:?}", bite.runs);
+    assert_eq!(check_c(&bite.runs[0]), check_c(&part.runs[0]));
+    assert!(check_c(&bite.runs[0]).contains("x f 1.039755 = 697000000"));
+    // (2) The set's measured runs come from two builds: the bite cannot tell whose numbers are in force.
+    let mut two = logs.clone();
+    let mut other: Vec<Log> = logs
+        .iter()
+        .filter(|l| l.header.run == "ws-1")
+        .cloned()
+        .collect();
+    edit_run(&mut other, "ws-1", &|l| {
+        l.header.run = "ws-2".to_owned();
+        l.header.build.binary_sha256 = "dd".to_owned();
+    });
+    two.extend(other);
+    let (_, v) = ws_judge(&two);
+    let bite = ws_bite(&v);
+    assert_eq!(bite.status, BiteStatus::Invalid, "{:?}", bite.runs);
+    assert!(
+        says(&bite.runs[0].invalid_why, "come from 2 builds"),
+        "{:?}",
+        bite.runs[0].invalid_why
+    );
+    // (3) Practice runs of two builds in the set, measured runs of one: the measured build's practice run.
+    let mut practices = logs.clone();
+    let mut p = another_practice(&logs, "ws-practice-dd", "2026-11-20T17:00:00Z", &[]);
+    edit_run(&mut p, "ws-practice-dd", &|l| {
+        l.header.build.binary_sha256 = "dd".to_owned();
+    });
+    practices.extend(p);
+    let (_, v) = ws_judge(&practices);
+    let bite = ws_bite(&v);
+    assert_eq!(bite.status, BiteStatus::Shown, "{:?}", bite.runs);
+    assert_eq!(check_c(&bite.runs[0]), check_c(&part.runs[0]));
+    // (4) A practice run of an older build in another set changes nothing.
+    let mut older = logs.clone();
+    let mut p = another_practice(&logs, "ws-practice-ee", "2026-11-19T17:00:00Z", &[]);
+    edit_run(&mut p, "ws-practice-ee", &|l| {
+        l.header.build.binary_sha256 = "ee".to_owned();
+        l.header.set = "set-old".to_owned();
+    });
+    older.extend(p);
+    let (_, v) = ws_judge(&older);
+    let bite = ws_bite(&v);
+    assert_eq!(bite.status, BiteStatus::Shown, "{:?}", bite.runs);
+    assert_eq!(check_c(&bite.runs[0]), check_c(&part.runs[0]));
+}
diff --git a/.gate/u4j-prefix/spikes/netcode-proof/crates/judge/tests/fixtures.rs b/.claude/worktrees/agent-a3b8dfeb01ed52b50/spikes/netcode-proof/crates/judge/tests/fixtures.rs
index 1e577f5..8ae391f 100644
--- a/.gate/u4j-prefix/spikes/netcode-proof/crates/judge/tests/fixtures.rs
+++ b/.claude/worktrees/agent-a3b8dfeb01ed52b50/spikes/netcode-proof/crates/judge/tests/fixtures.rs
@@ -24,6 +24,10 @@ fn verdict(mark: &str, case: &str) -> Verdict {
     let dir = root().join("fixtures").join(mark.to_lowercase());
     let mut files: BTreeMap<String, PathBuf> = BTreeMap::new();
     for sub in ["base", case] {
+        // PM-0001-1's one case is a whole run of its own, so that mark has no base folder.
+        if sub == "base" && !dir.join(sub).exists() {
+            continue;
+        }
         for entry in std::fs::read_dir(dir.join(sub)).expect("fixture folder") {
             let path = entry.expect("entry").path();
             files.insert(
@@ -263,6 +267,11 @@ fn pm_0001_17_fails_on_a_difference_the_sentinel_an_orphan_and_nothing_compared(
             "none-compared",
             "no client logged a hash (0 compared); an empty comparison fails",
         ),
+        // The advisor's FX-12: pairing is by the room's slot, so two clients' entries swapped fail all six.
+        (
+            "permuted-slots",
+            "(0 mismatches; on L2 under the Arm rule) = 6, needs == 0",
+        ),
     ] {
         let v = verdict("PM-0001-17", case);
         assert_eq!(v.verdict, VerdictKind::Fail, "{case}: {:?}", v.why);
@@ -271,6 +280,151 @@ fn pm_0001_17_fails_on_a_difference_the_sentinel_an_orphan_and_nothing_compared(
     }
 }
 
+/// One part's verdict and reasons.
+fn part_of<'v>(v: &'v Verdict, part: &str) -> &'v npt_judge::eval::PartOut {
+    v.parts
+        .iter()
+        .find(|p| p.name == part)
+        .unwrap_or_else(|| panic!("{part}"))
+}
+
+/// A report's values in one part.
+fn reported_in<'v>(v: &'v Verdict, part: &str, name: &str) -> Vec<&'v str> {
+    v.report
+        .iter()
+        .filter(|r| r.part == part && r.name.starts_with(name))
+        .map(|r| r.value.as_str())
+        .collect()
+}
+
+/// The Arm rule on the real marks.json's L2 P2 part (the advisor's FX-5 on U4j; the test-engineer's F12, C1, C2 and
+/// O): each hash name from its own process (`server_hash` from `room`, `decoded_hash` from `client` or `bots`,
+/// `replay_hash` from `replay` on x86_64), the rule only for an LG client on aarch64 with the room on x86_64, a
+/// client hash the server never logged counted on every machine, and a missing x86-64 re-decode entry a gap. Every
+/// case's outcome is collected, so one run shows them all.
+#[test]
+fn pm_0001_17_the_arm_rule_holds_on_lg_only() {
+    let mut wrong = Vec::new();
+    // Cross-arch: the LG hash differs and the x86-64 re-decode equals the server's: valid, 0, aarch64-only 1.
+    let v = verdict("PM-0001-17", "lg-cross-arch");
+    let p = part_of(&v, "L2 P2");
+    let shown = (
+        p.verdict,
+        p.runs.first().map(|r| r.valid),
+        p.runs
+            .first()
+            .and_then(|r| r.checks.first())
+            .and_then(|c| c.value.clone()),
+        reported_in(&v, "L2 P2", "aarch64-only"),
+    );
+    if shown
+        != (
+            VerdictKind::Pass,
+            Some(true),
+            Some("0".to_owned()),
+            vec!["1"],
+        )
+    {
+        wrong.push(format!("lg-cross-arch: {shown:?} {:?}", p.why));
+    }
+    for (case, part, kind, expected) in [
+        (
+            "lg-replay-differs",
+            "L2 P2",
+            VerdictKind::Fail,
+            "L2 P2: run l2-p2-1: 100 % of decoded snapshots rebuild the server's quantised state (0 mismatches; on L2 under the Arm rule) = 1, needs == 0",
+        ),
+        (
+            "lg-replay-in-client-log",
+            "L2 P2",
+            VerdictKind::Invalid,
+            "hs.replay_hash in the bots log on LG1 (aarch64): only process replay on x86_64 writes it",
+        ),
+        (
+            "lg-replay-on-aarch64",
+            "L2 P2",
+            VerdictKind::Invalid,
+            "hs.replay_hash in the replay log on C (aarch64): only process replay on x86_64 writes it",
+        ),
+        (
+            "l1-aarch64-claim",
+            "L1 P2",
+            VerdictKind::Fail,
+            "L1 P2: run p2-1: 100 % of decoded snapshots rebuild the server's quantised state (0 mismatches; on L2 under the Arm rule) = 1, needs == 0",
+        ),
+        (
+            "lg-orphan",
+            "L2 P2",
+            VerdictKind::Fail,
+            "L2 P2: run l2-p2-1: 100 % of decoded snapshots rebuild the server's quantised state (0 mismatches; on L2 under the Arm rule) = 1, needs == 0",
+        ),
+        (
+            "lg-replay-missing-entry",
+            "L2 P2",
+            VerdictKind::Invalid,
+            "slot 1's LG aarch64 hash at tick 2 differs from the server's, and the x86-64 re-decode has no entry there",
+        ),
+        // FX-10 (f): an LG mismatch waiting for its re-decode never hides an orphan's count (NOT-RUN before).
+        (
+            "lg-orphan-no-replay",
+            "L2 P2",
+            VerdictKind::Fail,
+            "L2 P2: run l2-p2-1: 100 % of decoded snapshots rebuild the server's quantised state (0 mismatches; on L2 under the Arm rule) = 1, needs == 0",
+        ),
+    ] {
+        let v = verdict("PM-0001-17", case);
+        let p = part_of(&v, part);
+        if p.verdict != kind || !p.why.iter().any(|w| w.contains(expected)) {
+            wrong.push(format!(
+                "{case}: {part} is {:?}, wanted {kind:?} with {expected:?}: {:?}",
+                p.verdict, p.why
+            ));
+        }
+    }
+    assert!(wrong.is_empty(), "{wrong:#?}");
+}
+
+/// README §1's `null` rule, checked (the advisor's FX-10 on U4j; the test-engineer's probe S and R2): every `hs` and
+/// `tick.s.snap_incomplete` list holds the run's `clients` entries; a (slot, tick) is resolved by a client hash or by
+/// `snap_incomplete` 1, never both; every tick inside a slot's resolved span at which the room logged that slot's
+/// hash is resolved, and the span covers the room's measured ticks for the slot. Each case is INVALID.
+#[test]
+fn pm_0001_17_every_server_hash_is_resolved_on_the_client_side() {
+    let mut wrong = Vec::new();
+    for (case, expected) in [
+        (
+            "sparse",
+            "slot 0: the client side resolves ticks 1 to 1 only, not the room's measured ticks 1 to 3",
+        ),
+        (
+            "sampled",
+            "slot 0, tick 2: the room logged a hash and no client log resolves it",
+        ),
+        (
+            "slot-unlogged",
+            "slot 1: the room logged its hash at measured ticks 1 to 3, and no client log resolves any tick of it",
+        ),
+        (
+            "hashed-and-incomplete",
+            "slot 1, tick 2: a client hash and tick.s.snap_incomplete 1 at once",
+        ),
+        (
+            "short-lists",
+            "hs.server_hash lists hold 2 entries, not the run's 48 clients",
+        ),
+    ] {
+        let v = verdict("PM-0001-17", case);
+        let p = part_of(&v, "L1 P2");
+        if p.verdict != VerdictKind::Invalid || !p.why.iter().any(|w| w.contains(expected)) {
+            wrong.push(format!(
+                "{case}: L1 P2 is {:?}, wanted Invalid with {expected:?}: {:?}",
+                p.verdict, p.why
+            ));
+        }
+    }
+    assert!(wrong.is_empty(), "{wrong:#?}");
+}
+
 /// C4's cases (the advisor's U2 ruling): a measured run built with the bites, a measured run naming a bite, and a bite
 /// run from a measured build are INVALID; the fourth, a twin from the wrong build, is PM-0001-21's `twin-bites-build`.
 #[test]
@@ -346,6 +500,46 @@ fn pm_0001_21_the_forged_run_against_its_valid_parts_twin() {
     }
 }
 
+/// The advisor's FX-4 on U4j (the test-engineer's probe D): two forged measured runs that name each other, with equal
+/// state hashes, never pass -21's "0 effect beyond the valid part": the twin must be a run of role twin, else a gap.
+#[test]
+fn pm_0001_21_the_twin_must_be_role_twin() {
+    let v = verdict("PM-0001-21", "twin-is-measured");
+    assert_eq!(v.verdict, VerdictKind::Invalid, "{:?}", v.why);
+    for (run, twin) in [
+        ("forged-a-1", "forged-a-1-twin"),
+        ("forged-a-1-twin", "forged-a-1"),
+    ] {
+        assert!(
+            says(
+                &v,
+                &format!(
+                    "run {run}: a gap in the record: run {run}: {twin} has role measured, not twin"
+                )
+            ),
+            "{:?}",
+            v.why
+        );
+    }
+}
+
+/// The advisor's FX-14 (TE-F13): PM-0001-1's ML part, a laptop's log naming its machine in full with no address
+/// (only a VM gives its day address), passes its machine_named figure.
+#[test]
+fn pm_0001_1_a_laptop_needs_no_address() {
+    let v = verdict("PM-0001-1", "laptop");
+    let p = part_of(&v, "ML (the named laptop)");
+    assert_eq!(p.verdict, VerdictKind::Pass, "{:?}", p.why);
+    let run = part_run(&v, "ML (the named laptop)");
+    assert!(run.valid, "{:?}", run.invalid_why);
+    let named = run
+        .checks
+        .iter()
+        .find(|c| c.name.starts_with("every machine named"))
+        .expect("the machine_named check");
+    assert_eq!(named.value.as_deref(), Some("1"), "{named:?}");
+}
+
 /// machine_named on PM-0006-11: a fully named RS passes and the no-SMT-map bite fails it; a header with no kernel
 /// fails.
 #[test]
@@ -369,16 +563,24 @@ fn pm_0006_11_every_machine_named_and_its_bite_shown() {
 
 #[test]
 fn the_fixtures_read_through_the_command_line_folder_reader_too() {
-    // read_logs walks a folder the way the binary does; every base folder reads clean.
-    for mark in [
-        "pm-0001-4",
-        "pm-0001-17",
-        "pm-0001-21",
-        "pm-0001-30",
-        "pm-0006-11",
-        "pm-0006-14",
+    // read_logs walks a folder the way the binary does; every base folder reads clean, and so do the Arm rule's L2
+    // cases (whole runs of their own).
+    for (mark, case) in [
+        ("pm-0001-4", "base"),
+        ("pm-0001-17", "base"),
+        ("pm-0001-17", "lg-cross-arch"),
+        ("pm-0001-17", "lg-replay-differs"),
+        ("pm-0001-17", "lg-replay-in-client-log"),
+        ("pm-0001-17", "lg-replay-on-aarch64"),
+        ("pm-0001-17", "lg-orphan"),
+        ("pm-0001-17", "lg-replay-missing-entry"),
+        ("pm-0001-17", "lg-orphan-no-replay"),
+        ("pm-0001-21", "base"),
+        ("pm-0001-30", "base"),
+        ("pm-0006-11", "base"),
+        ("pm-0006-14", "base"),
     ] {
-        let logs = read_logs(&root().join("fixtures").join(mark).join("base")).expect("reads");
+        let logs = read_logs(&root().join("fixtures").join(mark).join(case)).expect("reads");
         assert!(!logs.is_empty());
         assert!(
             logs.iter().all(|l| l.footer.is_some()),
````

### The fixtures' README
_`git diff --no-index .gate/u4j-prefix/spikes/netcode-proof/crates/judge/fixtures/README.md .claude/worktrees/agent-a3b8dfeb01ed52b50/spikes/netcode-proof/crates/judge/fixtures/README.md`:  1 file changed, 21 insertions(+), 2 deletions(-)._

````diff
diff --git a/.gate/u4j-prefix/spikes/netcode-proof/crates/judge/fixtures/README.md b/.claude/worktrees/agent-a3b8dfeb01ed52b50/spikes/netcode-proof/crates/judge/fixtures/README.md
index cb7e33f..986b5c8 100644
--- a/.gate/u4j-prefix/spikes/netcode-proof/crates/judge/fixtures/README.md
+++ b/.claude/worktrees/agent-a3b8dfeb01ed52b50/spikes/netcode-proof/crates/judge/fixtures/README.md
@@ -19,23 +19,42 @@ each case with the real `marks/marks.json` and checks the verdict and its reason
 | PM-0001-4 | `measured-bites-build` | C4: run 1's room log comes from a bites build, naming no bite | INVALID |
 | PM-0001-4 | `measured-names-bite` | C4: run 2 is a measured run that names the B-S bite | INVALID |
 | PM-0001-4 | `bite-measured-build` | C4: the B-S bite run's room comes from the measured build (no log of it lists `bites`) | INVALID |
-| PM-0001-17 | `base` | one L1 P2 run in the stress scene (its precondition held): every client hash equals the server's at its tick (6 compared); the other parts and the bites not run | NOT-RUN, the part's run valid, 0 mismatches |
+| PM-0001-17 | `base` | one L1 P2 run in the stress scene (its precondition held): every client hash equals the server's at its tick (6 compared); every `hs` and `snap_incomplete` list holds the header's 48 entries (slots 2 to 47 not connected: `null` in both logs); the other parts and the bites not run | NOT-RUN, the part's run valid, 0 mismatches |
 | PM-0001-17 | `lost-group` | slot 1's tick 2 incomplete (`null`, `snap_incomplete` 1): not compared, not a mismatch (5 compared) | NOT-RUN, the part's run valid, 0 mismatches |
 | PM-0001-17 | `differ` | slot 1's tick 2 hash differs from the server's | FAIL |
 | PM-0001-17 | `refused-group` | slot 1's tick 2 is the sentinel 0 (a refused group) | FAIL |
 | PM-0001-17 | `orphan` | slot 0 logs a hash at tick 65,539, which the server never logged (a wrong tick rebuild) | FAIL |
 | PM-0001-17 | `none-compared` | every client entry `null`: 0 compared, an empty comparison | FAIL |
+| PM-0001-17 | `permuted-slots` | slots 0 and 1 swapped in the client log on every tick: pairing is by the room's slot | FAIL (6 mismatches) |
+| PM-0001-17 | `sparse` | one client hash of six, the rest `null` with `snap_incomplete` 0: the span misses the room's measured ticks | INVALID |
+| PM-0001-17 | `sampled` | the client log has no tick 2 record: a server hash inside the span that no client log resolves | INVALID |
+| PM-0001-17 | `slot-unlogged` | slot 1 `null` everywhere with `snap_incomplete` 0, while the room logged its hashes | INVALID |
+| PM-0001-17 | `hashed-and-incomplete` | slot 1's tick 2 has a hash and `snap_incomplete` 1 at once | INVALID |
+| PM-0001-17 | `short-lists` | 2-entry lists under the header's 48 clients | INVALID |
+| PM-0001-17 | `l1-aarch64-claim` | the L1 client (role C) says aarch64 and logs a hash at tick 9, which the server never logged; an x86-64 replay agrees elsewhere: the Arm rule is LG's only, and an orphan counts everywhere | FAIL |
+| PM-0001-17 | `lg-cross-arch` | an L2 P2 run (run `l2-p2-1`: RS's room on x86_64, LG's bots on aarch64, a `replay` log on x86_64; 2 clients): slot 1's tick 2 differs on LG and the x86-64 re-decode equals the server's | the L2 P2 part's run valid, 0 mismatches, aarch64-only 1 |
+| PM-0001-17 | `lg-replay-differs` | the same, the re-decode differing too | FAIL |
+| PM-0001-17 | `lg-replay-in-client-log` | LG's bots log carries its own `replay_hash` (no replay log) | INVALID (a name in another process's log) |
+| PM-0001-17 | `lg-replay-on-aarch64` | the replay log is on aarch64 | INVALID (a name in another process's log) |
+| PM-0001-17 | `lg-orphan` | LG logs a hash at tick 9, which the server never logged; the replay present | FAIL |
+| PM-0001-17 | `lg-replay-missing-entry` | LG's tick 2 mismatch, the replay log without tick 2 | INVALID (an incomplete re-decode) |
+| PM-0001-17 | `lg-orphan-no-replay` | an LG orphan and an LG mismatch, no replay log: the waiting mismatch never hides the orphan's count | FAIL |
 | PM-0001-21 | `base` | one forged run (option A, the stress scene's load, 11 kinds x 1,000 commands, all rejected) and its valid-parts twin with equal state hashes | NOT-RUN, the part's run valid, twin check 0 |
 | PM-0001-21 | `twin-differs` | the twin's tick 2 hash differs | FAIL |
 | PM-0001-21 | `twin-bites-build` | C4's fourth case: the measured run's twin comes from a bites build | INVALID |
 | PM-0001-21 | `twin-unnamed` | the forged run names no twin (`params.twin` missing): a gap | INVALID |
 | PM-0001-21 | `twin-missing` | the named twin is not in the set (another run's twin is) | NOT-RUN |
+| PM-0001-21 | `twin-is-measured` | two forged measured runs name each other, with equal state hashes: a twin must have role `twin` | INVALID |
+| PM-0001-1 | `laptop` | an ML log naming its machine in full with no address (a run of its own: PM-0001-1 has no base folder) | the ML part PASS |
 | PM-0006-11 | `base` | RS fully named (with its day address and SMT map); the no-smt-map bite run's header has an empty SMT map and fails `machine_named` | PASS, bite shown |
 | PM-0006-11 | `unnamed` | RS's header has no kernel | FAIL |
 
 PM-0001-18's `ws-practice` rule needs 480 heal records per run, too long to write by hand, so its PASS and INVALID cases
 are built in code and judged with the real `marks/marks.json` in `tests/engine.rs`
-(`u4j_ws_practice_puts_the_practice_runs_numbers_in_force`, `u4j_ws_practice_refuses_every_number_not_set_by_its_rules`).
+(`u4j_ws_practice_puts_the_practice_runs_numbers_in_force`, `u4j_ws_practice_refuses_every_number_not_set_by_its_rules`,
+`u4j_ws_far_better_past_the_cap_is_invalid`, `u4j_ws_a_practice_run_after_the_counted_run_changes_nothing`,
+`u4j_ws_one_practice_run_per_build_over_every_set`, `u4j_ws_a_bite_run_takes_its_counted_runs_practice_run`,
+`u4j_ws_the_record_carries_the_marks_bound_line`, `u4j_ws_the_tolerance_and_band_boundaries`).
 
 PM-0006-14 has no pass number, so it is never PASS or FAIL: with all three N run and padded right it is REPORTED
 (the engine tests cover that path); its fixture proves that short padding comes out INVALID (ADR 0006's PM-0006-14 row:
````

### npt-log (crates/log)
_`git diff --no-index .gate/u4j-prefix/spikes/netcode-proof/crates/log .claude/worktrees/agent-a3b8dfeb01ed52b50/spikes/netcode-proof/crates/log`: no change._

````diff

````

### marks/ (marks.json, SERIES.md, README.md)
_`git diff --no-index .gate/u4j-prefix/spikes/netcode-proof/marks .claude/worktrees/agent-a3b8dfeb01ed52b50/spikes/netcode-proof/marks`:  3 files changed, 65 insertions(+), 41 deletions(-)._

````diff
diff --git a/.gate/u4j-prefix/spikes/netcode-proof/marks/README.md b/.claude/worktrees/agent-a3b8dfeb01ed52b50/spikes/netcode-proof/marks/README.md
index 109c832..e5a7ea2 100644
--- a/.gate/u4j-prefix/spikes/netcode-proof/marks/README.md
+++ b/.claude/worktrees/agent-a3b8dfeb01ed52b50/spikes/netcode-proof/marks/README.md
@@ -20,7 +20,8 @@ header; tick, second and event records follow in time order. Every record has a
 - **`header`:**
   - `run` names the run; `set` names the run set that is judged together; `role` is the run's part in its set:
     `measured` (the default), `bite` (the bites build, `bite` naming the planted defect), `twin` (a run's twin from
-    the same build, named in the other's `params.twin`: PM-0001-3, -21), `clean` (a clean reference run),
+    the same build, named in the other's `params.twin`: PM-0001-3, -21; a run and its twin log `h` over the same
+    ticks, since a tick hashed in one only counts as a mismatch), `clean` (a clean reference run),
     `bracket-first` and `bracket-last` (PM-0006-13's lone room), `practice` (PM-0001-18's unjudged WebSocket
     practice run, §2.5), or any role a mark's checks name (`n24`, `logging-off`, `cubic`, …);
   - `marks` lists the marks this run serves; the judge reads a run only for those marks;
@@ -40,20 +41,30 @@ header; tick, second and event records follow in time order. Every record has a
 - **Names in `v` and `s`** are lowercase with underscores, and end in their unit: `_ns`, `_bytes`, `_bits`,
   `_permille`, `_count`, `_um` (micrometres) or `_microdeg` (millionths of a degree), or are a 0/1 flag with no unit
   suffix. `SERIES.md` lists every name a mark reads, with its unit and the crate that writes it.
-- **`hs`** in a tick record (U4j; PM-0001-17, -19, -58): name → one entry per client slot of the run (index = slot,
-  the same length in every record of the run), each 16 lowercase hex digits, or `null` where this log has no hash
-  for that slot at that tick (a client not connected, a tick it could not complete, a slot another log holds). Read
-  only by figures; npt-log's writer and reader refuse any other entry (a 64-bit hash does not fit JSON's safe
-  integers, so it is a string, as `h` is). Its names:
+- **`hs`** in a tick record (U4j; PM-0001-17, -19, -58): name → one entry per client slot of the run: exactly the
+  header's `clients` entries in every record of every log of the run, at index = the room's slot for that client (the
+  slot the room's `tick.s` lists use; a client takes it from `JoinAccepted`), the same in every log, each 16
+  lowercase hex digits, or `null` where this log has no hash for that slot at that tick (a client not connected, a
+  tick it could not complete, a slot another log holds). Read only by figures; npt-log's writer and reader refuse any
+  other entry (a 64-bit hash does not fit JSON's safe integers, so it is a string, as `h` is). Its names:
   - `server_hash` (the room's log, at tick t): the server's hash of what it meant that client to hold after decoding
     tick t (`npt_proto::client_hash` over baseline plus delta, the own-state section and the events as sent), never 0;
-  - `decoded_hash` (the clients' log, at the tick t the client rebuilt): its hash of the state it holds for t, or the
-    sentinel `0000000000000000` for a tick with a received group it could not decode; `null` for a tick with a group
-    lost and none refused (counted in `tick.s.snap_incomplete`). A client's rebuilt ticks can go back (a wrong 16-bit
-    rebuild), and records are strictly in tick order, so its writer files every hash under its rebuilt tick,
-    buffering and sorting before it writes; a client with two hashes for one tick writes the sentinel there;
-  - `replay_hash` (process `replay`, the x86-64 re-decode of an L2 client's logged bytes, R-8): the same as
-    `decoded_hash`, decoded again natively, for the Arm rule.
+    written only in the room's log (process `room`), for each connected client from its admission on, in every phase
+    the clients log; `null` for a client not connected and for a tick whose snapshot the room did not send that
+    client (one the WebSocket send budget skipped);
+  - `decoded_hash` (only in the clients' logs, process `client` on L1 and `bots` on L2, at the tick t the client
+    rebuilt): its hash of the state it holds for t, or the sentinel `0000000000000000` for a tick with a received
+    group it could not decode; `null` for a tick with a group lost and none refused (counted in
+    `tick.s.snap_incomplete`). A client's rebuilt ticks can go back (a wrong 16-bit rebuild), and records are strictly
+    in tick order, so its writer files every hash under its rebuilt tick, buffering and sorting before it writes; a
+    client with two different hashes for one tick writes the sentinel there (one hash met twice, as a duplicated
+    datagram gives, is written once), and no hash is ever dropped: the writer buffers until every earlier tick is
+    written, or ends its log `crash`. Every tick from a client's first snapshot to the end of its log is resolved for
+    that client: a hash, or `null` with `tick.s.snap_incomplete` 1 (a tick none of whose groups arrived counts too,
+    known from the next header's tick); after the measured phase the clients keep logging until every measured tick
+    is resolved, and a tick still in flight at the stop is not written;
+  - `replay_hash` (only in a `replay` log on x86_64, the x86-64 re-decode of an L2 client's logged bytes, R-8): the
+    same as `decoded_hash`, decoded again natively, for the Arm rule.
 - **Events the judge reads for validity** (§4): `admit` (`session`, `in_roster`), `control` (`source`: `socket` or
   `session`; `kind`; `applied`; `rostered`; `cap_valid` for session controls; `peer_ok` for socket controls),
   `affinity` (`when`: `start` or `end`; `cpu`; `alone`; `sibling_free`), `person` (`who`, `mark`, `verdict`:
@@ -81,6 +92,7 @@ header; tick, second and event records follow in time order. Every record has a
 | `bites` | list of objects | `id` and `what`: each planted defect that must fail the mark first; or, with `expect: "invalid"` and `rule`, one that must make its run INVALID by that rule alone (§5) |
 | `stand_in` | bool | true only for PM-0001-4: a pass on a lighter sim is STAND-IN (header `params.sim` = `stand-in`) |
 | `arm_rule` | bool | true for PM-0001-17, -19, -48 and -54 (ADR 0001's Arm rule) |
+| `ws_bound_line` | string | PM-0001-18 holds the WebSocket bound's line (§2.5) verbatim in its `ws_bound_line` (provisional until fixes.rs's bound lands: its `model` says so, so no counted run's header matches it), and `--check-marks` refuses the mark unless the line's X, k, Y and Z equal its `ws_bound` checks' and its `tolerance` the `ws-practice` rule's. A mark with `ws_bound` checks must have it, and only such a mark may |
 | `readings` | list of strings | the plan §15 readings that shaped the entry (`R-6`, …) |
 | `notes` | string | anything the data cannot say: the setup in short, a proposed reading, what is not yet built |
 
@@ -184,21 +196,22 @@ from the part's valid runs, never judged (B10).
   `z_ns` (Z: the smallest 100 ms step with P(none over Z) >= 0.98), `m_m_ns` (M_m, the median of the run medians),
   `band_low` and `band_high` (the 1st and 99th percentiles of a correct build's run median / M_m), `tolerance` (1.10,
   or lower by the guard; never above 1.10) and `guard_share` (the share of budget-x2 runs whose run median / M_m is at
-  most `tolerance`, at most 0.01). Times are integer ns; the four fractions are decimal strings (`"0.965"`), read
-  exactly. For example (bf9d248's first bound as numbers; its M_m, band and runs are placeholders):
+  most `tolerance`, at most 0.01). Times are integer ns; the four fractions are decimal strings with at most 6 digits
+  after the point (`"0.965"`), read exactly; band_low <= 1 <= band_high. For example (bf9d248's first bound as numbers; its M_m, band and runs are placeholders):
   `npt-ws-bound v1 {"model":"linux-6.8-tcp v3","commit":"<40 hex digits>","runs_count":2000,"client_bursts_count":480,"x_ns":670000000,"y_ns":2500000000,"k_count":5,"z_ns":6900000000,"m_m_ns":654000000,"band_low":"0.965","band_high":"1.035","tolerance":"1.10","guard_share":"0.0000"}`.
-  marks.json's PM-0001-18 holds the same X, k, Y, Z (its `ws_bound` checks) and tolerance (`ws-practice:<t>`),
-  named in its notes by the file's path and commit.
+  marks.json's PM-0001-18 holds this line verbatim in its `ws_bound_line` (provisional until fixes.rs's bound lands:
+  its `model` says so, so no counted run's header matches it), and `--check-marks` refuses the mark unless the line's
+  X, k, Y and Z equal its `ws_bound` checks' and its `tolerance` the `ws-practice` rule's.
 - **The WebSocket practice run's outcome record** (the run script writes it beside the run, from the run's series and
   the bound's line): one event `ws_practice` (phase `after`) in a log of the practice run (role `practice`, listing
   PM-0001-18), with fields `class` (`IN TOLERANCE`, `BEYOND`, `FAR BETTER` or `INVALID`, the advisor's R7), `m_p_ns`
   (M_p: the run's nearest-rank median over its measured `burst_heal.after_link_ns`), `bound` (the bound's line,
-  verbatim), `bound_file` and `bound_sha256` (where that line is committed), `x_moved_ns`, `y_moved_ns`, `z_moved_ns`
+  byte for byte the mark's `ws_bound_line`), `bound_file` and `bound_sha256` (where that line is committed), `x_moved_ns`, `y_moved_ns`, `z_moved_ns`
   and `k_count` (the numbers in force), and, only when they exist, `chance_ruling` (the advisor's ruling of chance,
   R8 (v)) and `second_ruling` (a ruling naming a found and fixed cause, for a second practice run of one build). The
-  record's other fields are for people (`s`, `f`, the band, the record count, the mean message size, the header
-  checks), and the judge reads none of them: it recomputes s = M_p / M_m and f = max(1, s) exactly, and each moved
-  number as the bound's times f rounded up to the next millisecond, from M_p and the bound's line.
+  record's other fields are for people (`s`, `f`, the band, the record count, the mean message size), and the judge
+  reads none of them: it recomputes s = M_p / M_m and f = max(1, s) exactly, and each moved number as the bound's
+  times f rounded up to the next millisecond, from M_p and the bound's line.
 
 ## 3. How a verdict is reached
 1. The judge reads every results log under the given folder and keeps, per mark, the runs whose header lists it.
@@ -263,19 +276,29 @@ The others apply where a mark lists them:
   condition 4 on U4; a reading of ADR 0001:1589, for the readings PR). The precondition must be a pass mark with no
   precondition of its own.
 - **`ws-practice:<tolerance>`** (PM-0001-18's WebSocket part; Mustafa's WS-BAR and the 10 % tolerance, 2026-11-07;
-  the advisor's rulings R4 to R8 on the docs PR): the run is INVALID unless its set holds its practice run and that
-  run's record (§2.5) puts the numbers in force. The practice run: role `practice`, listing the mark, built from the
-  same binaries (`build.binary_sha256` over its logs; a bite run takes the measured build's practice run of its set),
-  the same `scene`, `profile` and `params.burst_spacing_ns`, started (`params.started_utc`) before the run, holding
-  the four rules above, with exactly one `ws_practice` record; one practice run counts per build, so with more than
-  one the latest counts and only if its record has a `second_ruling`. The record: its bound line parses (§2.5), its
-  `tolerance` equals the rule's and its X, k, Y and Z equal the mark's `ws_bound` checks'; `m_p_ns` is the practice
-  run's own median heal time; with s = M_p / M_m, `IN TOLERANCE` needs band_low <= s <= tolerance (and the practice
-  run's heals at most k over Y' and none over Z', the moved numbers), `BEYOND` needs a `chance_ruling` and s <=
-  tolerance (a lone count or maximum failure), `FAR BETTER` needs a `chance_ruling` and s < band_low, and `INVALID`
-  never counts; f = max(1, s); its moved numbers equal the judge's (the bound's times f, rounded up to the next
-  millisecond; k unmoved). So no number is set by hand, and nothing in marks.json changes between the practice run
-  and the counted run.
+  the advisor's rulings R4 to R8 on the docs PR): the run is INVALID unless the runs the judge reads hold its practice
+  run and that run's record (§2.5) puts the numbers in force. The practice run: role `practice`, listing the mark,
+  built from the same binaries (`build.binary_sha256` over its logs), with the same `scene`, `profile`,
+  `params.burst_spacing_ns`, `params.ws_tcp_model` (equal to the bound's `model`), `params.config_hz` and RS's
+  `machine.kernel` (each present in both runs), started (`params.started_utc`) before the run, holding the four rules
+  above, with exactly the bound's `client_bursts_count` measured heal records and exactly one `ws_practice` record. A
+  bite run takes the practice run of its set's measured runs that serve the mark with its layer, profile and
+  `params.transport` (one build among them, else INVALID), so it is judged at its counted run's numbers. One practice
+  run counts per build: of that build's practice runs started before the run (in any set, listing the mark or not),
+  the latest counts, and with more than one only if its record has a `second_ruling`; a practice run started after the
+  run never changes its verdict. The record: its `bound` is the mark's `ws_bound_line` byte for byte; `m_p_ns` is the
+  practice run's own median heal time; with s = M_p / M_m:
+  - `IN TOLERANCE` needs band_low <= s <= tolerance, and the practice run's heals at most k over Y' and none over Z'
+    (the moved numbers);
+  - `BEYOND` needs a `chance_ruling` and s <= tolerance (a lone count or maximum failure);
+  - `FAR BETTER` needs a `chance_ruling` and s < band_low;
+  - `INVALID` never counts.
+
+  f = max(1, s) for IN TOLERANCE and BEYOND, and f = 1 for FAR BETTER (the advisor's R8 (v): at the bound). The judge
+  refuses any f above the tolerance. The record's moved numbers equal the judge's: the bound's times f, rounded up to
+  the next millisecond, with k unmoved. Checks (c) to (e) print the class, M_p, M_m, s, the band, f, the tolerance,
+  the practice run and any ruling's text beside the numbers in force. So no number is set by hand, and nothing in
+  marks.json changes between the practice run and the counted run.
 
 ## 5. Bites first
 Each bite of a mark needs at least one run with role `bite` whose header names that bite and lists the mark. That run
diff --git a/.gate/u4j-prefix/spikes/netcode-proof/marks/SERIES.md b/.claude/worktrees/agent-a3b8dfeb01ed52b50/spikes/netcode-proof/marks/SERIES.md
index 2e210a4..ddccf10 100644
--- a/.gate/u4j-prefix/spikes/netcode-proof/marks/SERIES.md
+++ b/.claude/worktrees/agent-a3b8dfeb01ed52b50/spikes/netcode-proof/marks/SERIES.md
@@ -261,9 +261,9 @@ row; the script fails otherwise). Marks: `-n` = PM-0001-n, `0004-67` = PM-0004-6
 | `alloc_twin_diff_count` | count | npt-judge | count of measured tick numbers where tick.v.tick_allocs_count differs between the run and its params.twin (role logging-off) | PM-0006-20 'on every tick after the warm-up ... equals the count with logging off', 'compared tick by tick' | 0006-20 |
 | `bulk_control_p99_rise_ns` | ns | npt-judge | p99(second.s.control_delay_bulk_ns) - p99(second.s.control_delay_idle_ns) | PM-0001-38 'raises the control stream's p99 message delay by at most 5 ms (p99 with the transfer minus p99 without)' | -38 |
 | `crash_close_max_ns` | ns | npt-judge | max over the run's clients of (its RoomEnded or close, event.room_end.server_est_ns on LG's server-clock estimate) minus the room's event.tick_panic.server_ns; a client with no room_end fails | PM-0001-59 'every client gets RoomEnded with a crash reason or has its connection closed within that second (0 left open)' | -59 |
-| `cross_arch_mismatch_count` | count | npt-judge | the hash kind (built at U4j): count of (client, tick) whose aarch64 client's `hs.decoded_hash` differs from `hs.server_hash` (or has no server hash there) while `hs.replay_hash`, the x86-64 re-decode (process `replay`), equals the server's entry (both absent is agreement); over every tick record of the run, any phase. A run with no `hs.decoded_hash` or no `hs.server_hash` is a gap, never 0; an aarch64 mismatch with no replay log in the run is NOT-RUN. The shot view test's and the throw landing's kinds join with the figures that read them (U4h: view_hit_registered_share, false_hit_share, throw_desync_count) | the Arm rule: 'reported as a cross-architecture finding and decides nothing' (-17, -19, -48, -54) | -17, -19, -48, -54 |
+| `cross_arch_mismatch_count` | count | npt-judge | the hash kind (built at U4j): count of (client, tick) whose LG aarch64 client's `hs.decoded_hash` differs from `hs.server_hash` while `hs.replay_hash`, the x86-64 re-decode (process `replay`), equals the server's entry; over every tick record of the run, any phase. A run with no `hs.decoded_hash` or no `hs.server_hash` is a gap, never 0; an aarch64 mismatch with no replay log in the run is NOT-RUN. The shot view test's and the throw landing's kinds join with the figures that read them (U4h: view_hit_registered_share, false_hit_share, throw_desync_count) | the Arm rule: 'reported as a cross-architecture finding and decides nothing' (-17, -19, -48, -54) | -17, -19, -48, -54 |
 | `decoded_hash_compared_count` | count | npt-judge | the client hashes decoded_hash_mismatch_count looked at: every non-null `hs.decoded_hash` entry, paired with the server's or not (0 is a real count here; the mismatch figure fails on it) | PM-0001-17, reported | -17 |
-| `decoded_hash_mismatch_count` | count | npt-judge | over every tick record of the run, any phase (-58's wraps fall in its first seconds), per client slot and tick: a non-null `hs.decoded_hash` (each client's hash of the state it holds, under the tick it rebuilt; the sentinel 0 for a tick with a refused group) counts when `hs.server_hash` at that (slot, tick) differs or is absent (a client hash at a tick the server never logged: a wrong tick label); equal hashes count nothing; a server hash with no client hash is loss, not compared (`tick.s.snap_incomplete` counts the ticks a client could not complete). On an aarch64 client's log (machine.arch), a mismatch counts only when `hs.replay_hash` at that (slot, tick) differs from the server's too (the Arm rule; else cross_arch_mismatch_count). Fails, never passes, on 0 compared (the check comes out `empty`: README §2.3). Gaps: no log with `hs.server_hash` or `hs.decoded_hash`, lists of different slot counts, two logs giving one (slot, tick) a hash, a server hash of 0. Inputs: `hs` (README §1; Q30) | PM-0001-17 '100 % of decoded snapshots rebuild the server's quantised state'; -58; -19's join hash; u4-build-plan.md §4.6 (the advisor's Q11 ruling) | -17, -58, -19 |
+| `decoded_hash_mismatch_count` | count | npt-judge | over every tick record of the run, any phase (-58's wraps fall in its first seconds), per client slot and tick: a non-null `hs.decoded_hash` (each client's hash of the state it holds, under the tick it rebuilt; the sentinel 0 for a tick with a refused group) counts when `hs.server_hash` at that (slot, tick) differs or is absent (a client hash at a tick the server never logged: a wrong tick label); equal hashes count nothing; a server hash with no client hash is loss, not compared (`tick.s.snap_incomplete` counts the ticks a client could not complete). On a client log written on LG (`machine.role` LG) on aarch64, in a run whose room log is on x86_64, a mismatch against a server hash counts only when `hs.replay_hash` (a `replay` log on x86_64) at that (slot, tick) differs from the server's too (the Arm rule; else cross_arch_mismatch_count); a replay log with no entry there is a gap. A client hash with no server hash counts on every machine. Fails, never passes, on 0 compared (the check comes out `empty`: README §2.3). Gaps: no log with `hs.server_hash` or `hs.decoded_hash`, lists of different slot counts, two logs giving one (slot, tick) a hash, a server hash of 0, a name in a log of another process (`server_hash` outside `room`, `decoded_hash` outside `client` and `bots`, `replay_hash` outside `replay` on x86_64); the coverage rule (README §1's `null` rule, checked): every `hs` list and `tick.s.snap_incomplete` list holds the run's `clients` entries; a (slot, tick) is resolved by a client hash (compared) or by `tick.s.snap_incomplete` 1 (lost, not compared), never both; per slot, every tick from its first resolved tick to its last at which the room logged that slot's hash is resolved, and that span covers the room's measured ticks for the slot; a mismatch still fails the run, and 0 compared still comes out `empty`. Inputs: `hs` (README §1; Q30) | PM-0001-17 '100 % of decoded snapshots rebuild the server's quantised state'; -58; -19's join hash; u4-build-plan.md §4.6 (the advisor's Q11 ruling) | -17, -58, -19 |
 | `down_client_mean_max_bits` | bits | npt-judge | max over client slots of the mean of second.s.down_bits[slot] over the measured seconds | PM-0001-12 'mean at most 160 kbps ... counted per client' | -12 |
 | `down_mean_ipv6_bits` | bits | npt-judge | mean over client-seconds of second.s.down_bits + 160 x second.s.down_datagrams_count (20 B more header per datagram) | PM-0001-12 'if LG reaches RS over IPv6, the IPv6 figure' | -12 |
 | `egress_interface_bytes` | bytes | npt-judge | event.iface.tx_bytes at the end minus at the start | PM-0006-17 'the interface's transmitted-byte counter read at the start and end' | 0006-17 |
@@ -303,7 +303,7 @@ row; the script fails otherwise). Marks: `-n` = PM-0001-n, `0004-67` = PM-0004-6
 | `event.pong_delayer.hold_ns` | ns | npt-bots | the 150 ms pong delayer's cheat client logs each pong it held, with the hold (Q21) | -49 |
 | `untimed_trap_count` | count | npt-judge (figure, U7) | Worker traps with no timed rejoin (`event.join.world_ns`) after them and before the next trap (Q12) | -19 |
 | `everyone_in_view_min_share` | fraction (exact) | npt-judge (figure, built in U1) | as below; built in U1 because PM-0001-2 is the precondition of the stress-scene marks (B8) | -2 |
-| `header.params.started_utc` | ISO 8601 UTC | the run script | the run's UTC start, with seconds and `Z`; the times-of-day rule's input (B14) | -27, 0006-13, 0006-14 |
+| `header.params.started_utc` | ISO 8601 UTC | the run script | the run's UTC start, with seconds and `Z`; the times-of-day rule's input (B14) | -18, -27, 0006-13, 0006-14 |
 | `header.params.cc` | `newreno` or `cubic` | the room unit's configuration | the congestion controller, a start-up setting never changed in a run (A2) | -24 |
 | `header.params.bite_size_ns`, `header.params.bite_pad_bytes` | ns, bytes | the bites build | a bite's B-S or B-N size, printed beside the bite, never judged (B10) | every B-S or B-N bite |
 
@@ -313,9 +313,9 @@ row; the script fails otherwise). Marks: `-n` = PM-0001-n, `0004-67` = PM-0004-6
 
 | Name | Unit | Written by (crate) | Meaning | Marks |
 |---|---|---|---|---|
-| `hs.server_hash` | 16 hex digits or null, per client slot | npt-room (the encoder's mirror, §4.6) | at tick t: the server's hash (`npt_proto::client_hash`) of what it meant that client to hold after decoding t; never 0; null where it sent that client nothing for t | -17, -19, -58 (figures) |
-| `hs.decoded_hash` | 16 hex digits or null, per client slot | npt-client (L1, through npt-lab's client log), npt-bots (L2), web (browser) | at the tick t the client rebuilt: its hash of the state it holds for t (its held floats re-gridded); the sentinel 0 for a tick with a received group it could not decode; null for none (a group lost, none refused) | -17, -19, -58 (figures) |
-| `hs.replay_hash` | 16 hex digits or null, per client slot | npt-lab (`replay`, process replay: the x86-64 re-decode of an L2 client's logged bytes, R-8) | the same as `decoded_hash`, decoded again natively; read for the Arm rule | -17, -19 (figures) |
+| `hs.server_hash` | 16 hex digits or null, per client slot | npt-room (the encoder's mirror, §4.6) | at tick t: the server's hash (`npt_proto::client_hash`) of what it meant that client to hold after decoding t; never 0; written only in the room's log (process `room`); null where it sent that client nothing for t | -17, -19, -58 (figures) |
+| `hs.decoded_hash` | 16 hex digits or null, per client slot | npt-client (L1, through npt-lab's client log), npt-bots (L2), web (browser) | only in the clients' logs, process `client` on L1 and `bots` on L2, at the tick t the client rebuilt: its hash of the state it holds for t (its held floats re-gridded); the sentinel 0 for a tick with a received group it could not decode; null for none (a group lost, none refused) | -17, -19, -58 (figures) |
+| `hs.replay_hash` | 16 hex digits or null, per client slot | npt-lab (`replay`, process replay: the x86-64 re-decode of an L2 client's logged bytes, R-8) | only in a `replay` log on x86_64: the same as `decoded_hash`, decoded again natively; read for the Arm rule | -17, -19 (figures) |
 | `tick.s.snap_incomplete` | flag | npt-client (L1, through npt-lab's client log), npt-bots (L2) | one per client slot, at the tick its groups' headers name: 1 when a group of that tick was lost and none refused, so the client logs no hash for it (§4.6) | -17 (reported) |
 | `event.burst_eviction.evicted` | flag | npt-room | per client per burst (fields `slot`, `burst`): 1 when the client's last acknowledged baseline had left the AckRing (ages 0 to 31) at the first tick after the burst, else 0; shows whether -18's no-fallback bite could bite (§4.5) | -18 (recorded) |
 | `event.clamp_report.x_um`, `.y_um`, `.z_um` | µm | npt-room (from npt-sim's `reports()`) | per clamp report (fields `slot`, `seq`): the aim point the sim clamped to the weapon's reach, its grid cm x 10,000; read from `reports()`, never from the hashed event list; the clamp twin sends exactly these points (§4.10, the advisor's condition 1) | -21 (the clamp twin's input) |
diff --git a/.gate/u4j-prefix/spikes/netcode-proof/marks/marks.json b/.claude/worktrees/agent-a3b8dfeb01ed52b50/spikes/netcode-proof/marks/marks.json
index 1777735..02c5a65 100644
--- a/.gate/u4j-prefix/spikes/netcode-proof/marks/marks.json
+++ b/.claude/worktrees/agent-a3b8dfeb01ed52b50/spikes/netcode-proof/marks/marks.json
@@ -4434,11 +4434,12 @@
       ],
       "stand_in": false,
       "arm_rule": false,
+      "ws_bound_line": "npt-ws-bound v1 {\"model\":\"provisional: the single-burst bound of bf9d248 (fixes.rs ws_bar_fix1), not the run-structured bound\",\"commit\":\"bf9d24807106c4524d1aa0849409d219259c3943\",\"runs_count\":2000,\"client_bursts_count\":480,\"x_ns\":670000000,\"y_ns\":2500000000,\"k_count\":5,\"z_ns\":6900000000,\"m_m_ns\":654000000,\"band_low\":\"0.965\",\"band_high\":\"1.035\",\"tolerance\":\"1.10\",\"guard_share\":\"0.0000\"}",
       "readings": [
         "R-5",
         "R-12"
       ],
-      "notes": "B1 with 48 clients, 10 bursts per run (proposed), each burst's start moved by a random offset of 0 to one tick period against the room's tick (the advisor's reading, 2026-11-07); L1 3 seeds in the stress scene (scene SS, 48 clients, pre:PM-0001-2: u4-build-plan.md §4.12, the advisor's Q22 ruling); L2 1 run per transport (R3, VS4), the WebSocket one through the accept path's delay wrapper, whose clients run Linux's TCP as the model's do. Each client writes one `burst_heal` event per burst: from the link's return to the decode of the first snapshot the room writes at or after the return instant (a fresh delta; deltas queued during the burst do not count); a client that never heals writes none, so the count catches it. WebTransport keeps the plain 500 ms limit. WebSocket (Mustafa's WS-BAR, 2026-11-07): the median at most X, at most k over Y, none over Z, from the run-structured bound for a correct build with fix 1 and one unjudged practice run of the same build (role practice, listing this mark, earlier, the same scene, profile and burst spacing). The judge reads that run's `ws_practice` record (README §2.5), recomputes s = M_p / M_m and f = max(1, s), and applies X, Y and Z times f, each rounded up to the next millisecond, k unmoved; checks (c) to (e) print the numbers in force with M_p, M_m, s and f beside them. The `ws-practice` rule makes the counted run INVALID unless the record is IN TOLERANCE, or names the advisor's chance ruling (R8 (v)), and its numbers are the judge's; one practice run counts per build (a second needs a ruling naming a found and fixed cause). The tolerance is 1.10 (Mustafa's 10 %), lowered only before the practice run by the bound's guard, never raised. PROVISIONAL: X 0.67 s, k 5 over Y 2.5 s and Z 6.9 s are bf9d248's first, single-burst bound (spikes/netcode-proof/results/u3-ws-fallback/bar-fix1-bf9d248.txt); the run-structured bound (fixes.rs, its own unit after U4r) replaces them, and the tolerance if its guard lowers it, with its committed machine-readable line, named here by path and commit, before R3's run plan. Bites: no-fallback on WebTransport's parts only; ws-budget-x2 on the WebSocket part, judged at the measured build's numbers in force (a bites build has no practice run of its own). On L2 the time uses the client's server-clock estimate, whose error bound is PM-0001-47's. The row's last reported item, the time until a decoded snapshot's extra delay is back under PM-0001-31's 250 ms, gets its series with its writer at U6; the client's fast reconnect over WebSocket is not measured here (OQ-0001-19)."
+      "notes": "B1 with 48 clients, 10 bursts per run (proposed), each burst's start moved by a random offset of 0 to one tick period against the room's tick (the advisor's reading, 2026-11-07); L1 3 seeds in the stress scene (scene SS, 48 clients, pre:PM-0001-2: u4-build-plan.md §4.12, the advisor's Q22 ruling); L2 1 run per transport (R3, VS4), the WebSocket one through the accept path's delay wrapper, whose clients run Linux's TCP as the model's do. Each client writes one `burst_heal` event per burst: from the link's return to the decode of the first snapshot the room writes at or after the return instant (a fresh delta; deltas queued during the burst do not count); a client that never heals writes none, so the count catches it. WebTransport keeps the plain 500 ms limit. WebSocket (Mustafa's WS-BAR, 2026-11-07): the median at most X, at most k over Y, none over Z, from the run-structured bound for a correct build with fix 1 and one unjudged practice run of the same build (role practice, listing this mark, earlier, the same scene, profile and burst spacing). The judge reads that run's `ws_practice` record (README §2.5), recomputes s = M_p / M_m and f = max(1, s), and applies X, Y and Z times f, each rounded up to the next millisecond, k unmoved; checks (c) to (e) print the numbers in force with M_p, M_m, s and f beside them. The `ws-practice` rule makes the counted run INVALID unless the record is IN TOLERANCE, or names the advisor's chance ruling (R8 (v)), and its numbers are the judge's; one practice run counts per build (a second needs a ruling naming a found and fixed cause). The tolerance is 1.10 (Mustafa's 10 %), lowered only before the practice run by the bound's guard, never raised. PROVISIONAL: X 0.67 s, k 5 over Y 2.5 s and Z 6.9 s are bf9d248's first, single-burst bound (spikes/netcode-proof/results/u3-ws-fallback/bar-fix1-bf9d248.txt); the run-structured bound (fixes.rs, its own unit after U4r) replaces them, and the tolerance if its guard lowers it, with its committed machine-readable line, named here by path and commit, before R3's run plan. The bound's line is held verbatim in `ws_bound_line`; until fixes.rs's run-structured bound replaces it, its model says provisional, so no counted run can match it. Bites: no-fallback on WebTransport's parts only; ws-budget-x2 on the WebSocket part, judged at the measured build's numbers in force (a bites build has no practice run of its own). On L2 the time uses the client's server-clock estimate, whose error bound is PM-0001-47's. The row's last reported item, the time until a decoded snapshot's extra delay is back under PM-0001-31's 250 ms, gets its series with its writer at U6; the client's fast reconnect over WebSocket is not measured here (OQ-0001-19)."
     },
     {
       "id": "PM-0001-19",
````

### The fixtures (crates/judge/fixtures, logs only)
_ 39 files changed, 208 insertions(+), 24 deletions(-); the diff itself is in `.checkpoints/netcode-proof-test/u4j-round2-packet-fixtures.diff` (`git diff --no-index .gate/u4j-prefix/spikes/netcode-proof/crates/judge/fixtures .claude/worktrees/agent-a3b8dfeb01ed52b50/spikes/netcode-proof/crates/judge/fixtures`)._

## What it is judged against

### The plan: u4-build-plan.md §5 U4j
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 644-663 (sha256 7375cdcfdb7c), as they are:_

````text
### U4j, the judge and marks change (npt-judge, npt-log, marks/; depends on nothing; size M; the main session or the first free lane)
- **Builds:** npt-log `hs` and its schema test. npt-judge: C4 with fixtures; the figures
  `decoded_hash_mismatch_count` (§4.6's rules), `decoded_hash_compared_count`, `twin_hash_mismatch_count`, and, with
  fixtures only since no U4 run feeds them, `cross_arch_mismatch_count` (L2) and `machine_named` (-1's parts select
  RS, LG, ML, RD and MAC). No `PRE-` id, no marks/prechecks.json and no `"kind": "precheck"` rule, nor their tests:
  -30's pre-check is a recorded npt-lab test the judge never reads (the advisor's condition 2).
  marks.json: -18's split, `burst_heal` and `burst_return`, the `ws-practice` rule (Q20); §4.12's table (Q22).
  README: the `loopback` layer value in §1, one line, since the pre-check's Loopback leg writes an npt-log header
  (the comparer reads both legs' room logs), with no judge change (the advisor's condition 2); `hs`, stating how it
  writes a client that has no hash at a tick (`null`), the form the six decoded-hash fixtures use (nit c). README §4's
  `pre:PM-0001-2` sentence (:193-197) gains -22's exception and its reason: -22's designed drop leaves 47 clients,
  which PM-0001-2's "a client on every soldier" refuses, so -22 carries no precondition (the advisor's condition 4).
  README and SERIES fix the format of the run-structured bound's machine-readable line and of the WebSocket practice
  run's outcome record (Q20; nit f). SERIES.md: the rows rev 2 adds (`snap_incomplete`, `burst_eviction`, the clamp
  events, the precheck suite) and the figure rows' formulas.
- **Tests:** PASS, FAIL, gap and INVALID fixtures per figure and rule; for the decoded hash six cases (equal; differing;
  a lost group, not compared; a refused group, sentinel; a client pair with no server pair; 0 compared, FAIL); C4's
  four cases; `--check-marks` on marks.json (the second file is dropped: the advisor's condition 2); the drift test.
- **Marks moved:** none judged here; every later judged set reads this judge.
- **Reviewers:** test-engineer, netcode-architect, security-auditor (validity rules).
````

### The advisor's rulings on round 1 (advisor-u4j.md)
_`.checkpoints/netcode-proof-test/advisor-u4j.md` lines 1-594 (sha256 a98dfdb4a4a3), as they are:_

````text
# Advisor: U4j (judge and marks change), round 1 rulings for the one fix pass

Date: 2026-11-11. Seat: day session, advisor agent (rules read from origin/main:.claude/agents/advisor.md).
Scope: U4j in worktree /home/user/Shamble-Frontlines/.claude/worktrees/agent-a3b8dfeb01ed52b50, base 411eeeed50d8c71e7e1f74366f0e1d5c27388444.
I rule; I edit no code, build nothing, run nothing, commit nothing.

## Plan
1. Read the main checkpoint's resume card, u4j-brief.md, u4j-build.md (readings (a)-(f)).
2. Read u4-build-plan.md §5 U4j, §4.6, §4.12; advisor-u4.md (conditions 2, 4; nits c, f; Q20, Q22, U4S-BUILDER);
   advisor-u4-wave1.md (W1-N2); advisor-ws-docs-r2.md (:375-378, R7-R11); history.md "Decisions taken" (WS-BAR, its
   refinement, NETEM-BAR (a)).
3. Read the diff in the worktree and the untracked files.
4. Read the three reviews whole (netcode, security, test) and the test probe outputs.
5. Write rulings: R1 the fix list (dedup, fix choice, proof by name); R2 the calls; R3 readings (a)-(f); R4 severity;
   R5 Decisions needed; then "What I checked, and what I did not".

## Progress
- [x] step 1 (the readings (a)-(f) are not in u4j-build.md; they are in the builder's handback, read from its
  transcript agent-a3b8dfeb01ed52b50.jsonl, "Readings for the advisor or reviewers", quoted in R3 below)
- [x] step 2 (plan §2, §4.6, §4.12, §5 U4j, Q11/Q20/Q22; advisor-u4.md conditions 1-13; W1-N2; ws-docs-r2 :375-400
  R3-R12; history.md :292-294 NETEM-BAR (a), :381-386 WS-BAR refinement, :387-391 WS-BAR; plan.md:359 the Arm rule)
- [x] step 3 (diff read: practice.rs whole; figures.rs :100-360; eval.rs :1-80, :420-470, :535-600, :980-1010,
  :1161-1200, :1386-1446, :1614-1650, :1700-1730; marks.rs :575-630; README diff; marks.json -17, -18, -21 parsed;
  fixtures pm-0001-17/base headers; the test-engineer's probe-run1/2 and plant-t outputs)
- [x] step 4 (three reviews read whole)  - [x] step 5 (rulings below, block by block). DONE.

## Verdict on round 1
**REVISE, as the three reviewers say.** One fix pass, by a fixer agent in U4j's worktree, closes FX-1 to FX-14 below
before the merge. Then round 2 runs on what changed (netcode-architect, security-auditor, test-engineer). F1 stays
High, so a round 3 is open to it alone if round 2 finds it still open. Nothing needs Mustafa now.

Ids: NC-n = u4j-review-netcode.md's Fn; SEC-Fn = u4j-review-security.md; TE-Fn and TE-Rn = u4j-review-test.md. Paths
are under spikes/netcode-proof/ in the worktree. "Fails today" means: the fixer writes the test or fixture first, runs
it on the unfixed code, and records its failure by name (the failing assert line) in its notes before the fix.

## R1. The fix pass's list (before the merge)

**How the fixer works** (u4j-brief.md's rules, unchanged): shared target and offline; touch before each build; files
only crates/judge, crates/log and marks/; no manifest, Cargo.lock or check.mjs edit; the Edit and Write tools; plants
in place, one at a time, restored and sha256-checked; commit nothing. At the end: check.mjs all 10 steps twice in a
row (counts as deltas on 304 / 327 / 48), `npt-judge --check-marks`, clippy -D warnings host and wasm32 in both
builds, `cargo +1.99.1 fmt --all --check`. Notes: u4j-fix.md (plan first, a line per item). It lists every new public
item by crate (A19) and every marks.json change as before and after. FX-1 first.

### FX-1 (High): FAR BETTER moves nothing, and no path moves the numbers past the tolerance
NC-1 = SEC-F1 = TE-F1. Probe A: band 2.0 to 2.0, s = 1.9 gives X' = 1.273 s, and a 1.2 s median PASSES.
- **The fix** (all three, as the reviewers agree):
  - (a) `parse_bound` refuses a band that leaves out 1: band_low <= 1 <= band_high. M_m is the median of the run
    medians, so a correct build's 1st to 99th percentile band holds 1 by definition.
  - (b) FAR BETTER gives f = 1. My R8 (v) says "FAR BETTER: at the bound". IN TOLERANCE and BEYOND keep f = max(1, s).
  - (c) Before `Ok`, `find` refuses any f above the tolerance. After (a) and (b) this guard cannot be reached; it stays
    anyway.
- **README §4's ws-practice paragraph:** FX-3 gives the whole paragraph.
- **Proof:**
  - `practice::tests::the_bound_line_refuses_a_band_that_leaves_out_1`: band 2.0 to 2.0, 1.01 to 1.2, and 0.5 to 0.9
    are each refused. Fails today.
  - engine.rs `u4j_ws_far_better_past_the_cap_is_invalid`: probe A committed. The record's bound line has band 2.0 to
    2.0, class FAR BETTER with a chance ruling, s = 1.9: the ws part is INVALID. Today it PASSES at 1.273 s.

### FX-2: a practice run started after the counted run changes nothing
NC-2 = TE-F2 (probes E1, E2: a FAIL becomes INVALID). My R9: "a counted WebSocket run that fails stands as FAIL".
- **The fix:** the candidates are only the practice runs that started before the run. The one-per-build rule counts
  over those. A run with no such candidate is INVALID ("no practice run of its build started before it").
- **Proof:** engine.rs `u4j_ws_a_practice_run_after_the_counted_run_changes_nothing`. Probe E0 FAILs. Adding a later
  practice run, with a second_ruling (E1) or without one (E2), leaves it FAIL. Fails today.

### FX-3: one practice run per build, over every run the judge reads; a bite run at its counted run's numbers
SEC-F3 = TE-F3 (probe F). TE-F15 is folded in, because the bite rule has to change with it. Reading (b) changes (R3).
- **The fix:**
  - (a) `find` takes every run the judge reads (`all`), not the counted run's set. My R7's "the results set holds its
    practice run" means the results the judge reads.
  - (b) A practice run of the build counts toward the rule whatever its set, and whether or not it lists the mark.
    The latest one counts; if it does not list PM-0001-18, the run is INVALID.
  - (c) A bite run takes the practice run in force for the measured runs of its own set: those that serve the mark
    with the bite run's layer, profile and `params.transport`.
    - More than one build among those measured runs: INVALID. None: INVALID.
    - The practice run is then found for that build exactly as for a measured run (FX-2, FX-3 (a), (b)).
    - So the bite run is judged at the same f as its counted run. A smaller f would show the bite falsely; a larger
      one would hide it.
- **The text: README §4's `ws-practice:<tolerance>` bullet, from "the run is INVALID unless" to "between the practice
  run and the counted run.", becomes:**
  > the run is INVALID unless the runs the judge reads hold its practice run and that run's record (§2.5) puts the
  > numbers in force. The practice run: role `practice`, listing the mark, built from the same binaries
  > (`build.binary_sha256` over its logs), with the same `scene`, `profile`, `params.burst_spacing_ns`,
  > `params.ws_tcp_model` (equal to the bound's `model`), `params.config_hz` and RS's `machine.kernel` (each present in
  > both runs), started (`params.started_utc`) before the run, holding the four rules above, with exactly the bound's
  > `client_bursts_count` measured heal records and exactly one `ws_practice` record. A bite run takes the practice
  > run of its set's measured runs that serve the mark with its layer, profile and `params.transport` (one build among
  > them, else INVALID), so it is judged at its counted run's numbers. One practice run counts per build: of that
  > build's practice runs started before the run (in any set, listing the mark or not), the latest counts, and with
  > more than one only if its record has a `second_ruling`; a practice run started after the run never changes its
  > verdict. The record: its `bound` is the mark's `ws_bound_line` byte for byte; `m_p_ns` is the practice run's own
  > median heal time; with s = M_p / M_m:
  > - `IN TOLERANCE` needs band_low <= s <= tolerance, and the practice run's heals at most k over Y' and none over
  >   Z' (the moved numbers);
  > - `BEYOND` needs a `chance_ruling` and s <= tolerance (a lone count or maximum failure);
  > - `FAR BETTER` needs a `chance_ruling` and s < band_low;
  > - `INVALID` never counts.
  >
  > f = max(1, s) for IN TOLERANCE and BEYOND, and f = 1 for FAR BETTER (the advisor's R8 (v): at the bound). The
  > judge refuses any f above the tolerance. The record's moved numbers equal the judge's: the bound's times f,
  > rounded up to the next millisecond, with k unmoved. Checks (c) to (e) print the class, M_p, M_m, s, the band, f,
  > the tolerance, the practice run and any ruling's text beside the numbers in force. So no number is set by hand,
  > and nothing in marks.json changes between the practice run and the counted run.
- **Proof:**
  - engine.rs `u4j_ws_one_practice_run_per_build_over_every_set`: probe F committed. A second practice run of the
    build, class BEYOND, sits in another set, with no second_ruling: INVALID. Fails today. A third case: the earlier
    run does not list the mark, still INVALID.
  - engine.rs `u4j_ws_a_bite_run_takes_its_counted_runs_practice_run`, four cases:
    - (1) the bite run's checks print the same f as its set's measured run;
    - (2) the set's measured runs come from two builds: INVALID. Fails today, which is valid;
    - (3) TE-F15: the set holds practice runs of two builds and measured runs of one. The bite takes the measured
      build's practice run and is valid. Fails today, which is INVALID ("cannot tell whose numbers");
    - (4) a practice run of an older build in another set leaves the bite run valid.
    - (1) and (4) pass today. They guard FX-3 (c): with FX-3 (a) alone, (4) would come out INVALID.

### FX-4: the twin of -21's twin-hash checks must have role twin
NC-4 = SEC-F5 = TE-F4 (probe D: two forged measured runs that name each other PASS). TE-F14 rides it.
- **The fix:** `twin_hash_mismatch_count` refuses a twin whose role is not `twin` as a gap, the way
  `fate_log_twin_equal` does. Its doc comment drops "any role (-30's twins are each other's measured runs)". -30's
  form (each other's measured runs, differing in `params.transport`) is added by U6 with -30: carry U6-a (the Carries list).
- **Proof:**
  - fixture `pm-0001-21/twin-is-measured`: two forged measured runs naming each other, with equal `h`. Judged in
    tests/fixtures.rs `pm_0001_21_the_twin_must_be_role_twin`: INVALID, and the reason names the role. Fails today.
  - TE-F14: `u4j_the_twin_hash_compares_every_tick_and_its_twin_whole` gains a twin with another profile and a twin
    with another bite, each a gap.

### FX-5: each hash name from its own process, and the Arm rule's key
SEC-F4 = TE-F5 + TE-F11 + TE-F12 + NC-6 + NC-8 + NC-12. Probes C1, C2 and O; C3, the mark's own flag, is FU-5.
Reading (e) changes (R3).
- **The fix (my call: R2.7):**
  - (a) The sources:
    - `hs.server_hash` is read only from logs of process `room`;
    - `hs.decoded_hash` only from logs of process `client` (L1) or `bots` (L2);
    - `hs.replay_hash` only from logs of process `replay` with machine.arch `x86_64`.
    - A name found in any other log is a gap.
  - (b) The Arm rule applies to a client hash only when all three hold:
    - its log's `machine.arch` is `aarch64`;
    - its log's `machine.role` is `LG` (ADR 0001:1605, -17: "one that shows only on LG's aarch64"; :1609, -19,
      judges ML's Chrome "as it stands");
    - the run's room log is on `x86_64`.
    - Every other mismatch counts.
  - (c) A client hash at a (slot, tick) where the server logged none (an orphan) counts on every machine. The key's
    excuse ("miss the server's logged hash too") needs a server hash for the replay to reproduce. This is a reading
    of ADR 0001:1605, and it is stricter. It joins the readings list (R5).
  - (d) An aarch64 mismatch against a server hash, in a run that has a replay log but no replay entry at that
    (slot, tick), is a gap: the re-decode is incomplete. Today it counts as a mismatch. A run with no replay log at
    all stays NOT-RUN (Waiting), as now.
- **The text:**
  - SERIES `decoded_hash_mismatch_count`: "On an aarch64 client's log (machine.arch), a mismatch counts only when
    `hs.replay_hash` at that (slot, tick) differs from the server's too (the Arm rule; else
    cross_arch_mismatch_count)." becomes:
    > On a client log written on LG (`machine.role` LG) on aarch64, in a run whose room log is on x86_64, a mismatch
    > against a server hash counts only when `hs.replay_hash` (a `replay` log on x86_64) at that (slot, tick) differs
    > from the server's too (the Arm rule; else cross_arch_mismatch_count); a replay log with no entry there is a gap.
    > A client hash with no server hash counts on every machine.
  - The same row's Gaps gain: "a name in a log of another process (`server_hash` outside `room`, `decoded_hash`
    outside `client` and `bots`, `replay_hash` outside `replay` on x86_64)".
  - SERIES `cross_arch_mismatch_count`: drop "(or has no server hash there)" and "(both absent is agreement)". Its
    count is the LG aarch64 mismatches against a server hash whose x86-64 replay entry equals that hash.
  - README §1's three `hs` items name their process as in (a) (FX-12 gives the words).
- **Proof:**
  - TE-F12: new fixtures under `fixtures/pm-0001-17/` on the real marks.json's L2 P2 part. Each holds an LG `bots`
    log, a room log of machine.role RS on x86_64 and, where named, a `replay` log:
    - `lg-cross-arch`: the LG hash differs and the x86-64 replay equals the server. The part's run is valid, the
      value is 0 and "aarch64-only" is 1.
    - `lg-replay-differs`: FAIL.
  - Each case below fails today (with today's result):
    - `lg-replay-in-client-log` (C1): FAIL or gap; today PASS.
    - `lg-replay-on-aarch64` (C2): gap; today PASS.
    - `l1-aarch64-claim` (probe O's L1 form, a role C client claiming aarch64): FAIL; today PASS.
    - `lg-orphan` (an aarch64 orphan, with the replay present): FAIL; today PASS.
    - `lg-replay-missing-entry`: INVALID; today FAIL.
  - Judged in tests/fixtures.rs `pm_0001_17_the_arm_rule_holds_on_lg_only`, and added to the folder-reader test's list.

### FX-6: marks.json pins the bound line; the record must carry it byte for byte
SEC-F2 = TE-F6 = NC-9. Probe H: a record saying M_m 620 ms turns a FAIL into a PASS. SEC-F11 rides it (same code).
- **The form (my call: R2.3):**
  - (a) A new optional mark field `ws_bound_line` (a string, the line verbatim). A mark with `ws_bound` checks must
    have it, and only such a mark may.
  - (b) `--check-marks` (marks.rs at load) parses it with `parse_bound`. It refuses the mark unless the line's X, k,
    Y and Z equal the `ws_bound` checks' numbers and its `tolerance` equals the `ws-practice` rule's. These checks
    move from run time to marks time, and `find` keeps them too.
  - (c) The ws-practice rule refuses a record whose `bound` is not that string byte for byte. M_m, the band, the
    guard, the model and the commit then come from reviewed marks.json, never from a log.
  - (d) The provisional line, so that no real counted run can be judged on placeholders. Its model text never equals
    a real run's `params.ws_tcp_model` (FX-13), so such a run comes out INVALID until fixes.rs's bound replaces the
    line. In marks.json, escaped as JSON needs:
    `npt-ws-bound v1 {"model":"provisional: the single-burst bound of bf9d248 (fixes.rs ws_bar_fix1), not the run-structured bound","commit":"bf9d24807106c4524d1aa0849409d219259c3943","runs_count":2000,"client_bursts_count":480,"x_ns":670000000,"y_ns":2500000000,"k_count":5,"z_ns":6900000000,"m_m_ns":654000000,"band_low":"0.965","band_high":"1.035","tolerance":"1.10","guard_share":"0.0000"}`
  - (e) -18's notes gain: "The bound's line is held verbatim in `ws_bound_line`; until fixes.rs's run-structured bound
    replaces it, its model says provisional, so no counted run can match it."
  - (f) SEC-F11: `compile_check` also pins each letter's shape:
    - x: stat p50, op `<=`;
    - y: stat count, a `where` with op `>`, op `<=`;
    - z: stat max, op `<=`.
    - Any other shape is refused.
  - (g) A new public item (A19): the field on `Mark`. List it for §4.1 at the merge.
- **The text (README §2.5):**
  - Replace "marks.json's PM-0001-18 holds the same X, k, Y, Z (its `ws_bound` checks) and tolerance
    (`ws-practice:<t>`), named in its notes by the file's path and commit." with:
    > marks.json's PM-0001-18 holds this line verbatim in its `ws_bound_line` (provisional until fixes.rs's bound
    > lands: its `model` says so, so no counted run's header matches it), and `--check-marks` refuses the mark unless
    > the line's X, k, Y and Z equal its `ws_bound` checks' and its `tolerance` the `ws-practice` rule's.
  - In the record bullet, "`bound` (the bound's line, verbatim)" becomes "`bound` (the bound's line, byte for byte
    the mark's `ws_bound_line`)".
  - README §2.1's mark-field table gains a `ws_bound_line` row in the same words.
- **Proof:**
  - engine.rs `u4j_ws_the_record_carries_the_marks_bound_line`: probe H committed. The record's line says M_m 620 ms,
    with its moved numbers recomputed to match: INVALID. Today it PASSES. A second case changes only the line's
    commit: INVALID.
  - marks.rs unit `ws_bound_line_agrees_with_its_checks`, each case refused:
    - the line's X differs from check (c);
    - its tolerance differs from the rule's;
    - no line on a mark with ws_bound checks;
    - a line on a mark without them;
    - band 2.0 to 2.0.
  - SEC-F11's shapes go in `ws_bound_checks_keep_their_shape`: x with op `>=`, x with stat max, z with p50, and y with
    a `where` op `>=`, each refused. Fails today.

### FX-7: the practice run needs exactly the bound's heal count
NC-7 = TE-F7 (probe G: 479 heals PASS). My R7: "Valid only with 480 heal records".
- **The fix:** `heal.len()` must equal the bound's `client_bursts_count`, else INVALID.
- **Proof:** `u4j_ws_practice_refuses_every_number_not_set_by_its_rules` gains 479 and 481 heals, each INVALID.
  Fails today.

### FX-8: no panic on a long decimal
SEC-F6 = TE-F8 (probe B: the judge panics at ratio.rs:27).
- **The fix:** `parse_bound` refuses a fraction with more than 6 digits after the point (an Err, never a panic).
- **The text:** README §2.5, "the four fractions are decimal strings (`"0.965"`), read exactly" becomes "the four
  fractions are decimal strings with at most 6 digits after the point (`"0.965"`), read exactly; band_low <= 1 <=
  band_high".
- **Proof:** `practice::tests::the_bound_line_refuses_more_than_6_decimals`: probe B's line is an Err. Fails today
  with a panic.

### FX-9: the tolerance and band boundaries are pinned by tests
TE-F9 (plant T: cap 1.10 to 1.12, band_low dropped and the Z' recheck dropped together survive all 65 tests).
TE-R1 rides it. My ruling (advisor-ws-docs-r2.md:378) asks for "the boundaries s = 1.00, 1.10 and 1.1001".
- **Proof:** engine.rs `u4j_ws_the_tolerance_and_band_boundaries`, five cases:
  - s = 1.00: PASS at the bound, f = 1;
  - s = 1.10 (probe T1a): PASS at 737 ms, 2.75 s and 7.59 s;
  - s = 1.1001 (T1b): INVALID;
  - IN TOLERANCE at s = 0.95, below band_low (T2): INVALID;
  - one practice heal over Z' only (T3): INVALID.
- These pass on today's code, which is right here. Their proof is plant T: the fixer re-runs it in place, and T1b, T2
  and T3 each fail by name.

### FX-10: the coverage floor, which checks README's own `null` rule
TE-F10 = SEC-F8 = NC-5 (i), with TE-R2. Probe S: one hash of six passes "100 % of decoded snapshots".
- **The form (my call: R2.1):** in `decoded`, over the logs FX-5 (a) admits:
  - (a) Every `hs` list and every `tick.s.snap_incomplete` list holds exactly the run's `clients` entries (the room
    log's header). Any other length is a gap, checked with the other structural gaps.
  - (b) Resolved: a (slot, tick) is resolved when a client log gives it a hash (compared) or a
    `tick.s.snap_incomplete` of 1 (lost, not compared). Both at once is a gap.
  - (c) Per slot, the span runs from the slot's first resolved tick to its last. Every tick inside the span at which
    the room logged a server hash for that slot must be resolved. If not, it is a gap: "slot s, tick t: the room
    logged a hash and no client log resolves it". That catches a null with snap_incomplete 0, and a missing tick
    record (a sampled log).
  - (d) Every slot the room logged at a measured tick has a span that covers the room's first and last measured
    server hash for it. A slot with no span, or a short span, is a gap.
  - (e) Ticks outside a slot's span are neither compared nor counted: before the client's first snapshot, and ticks
    still in flight at the stop.
  - (f) The order, so a gap never hides a failure (my C2 ruling):
    - First, (a) and the structural gaps (malformed entries, slot counts, two logs at one (slot, tick), a server 0,
      a name in the wrong process).
    - Then, any counted mismatch gives its count: FAIL.
    - Then, 0 compared gives `empty`.
    - Then, a coverage gap (b) to (d) or FX-5 (d) is a gap.
    - Then the value 0.
    - The Waiting case (an LG mismatch in a run with no replay log) also comes after the counted mismatches. Today it
      returns at once, so it can hide an orphan's or another log's FAIL as NOT-RUN. A further FX-5 fixture,
      `lg-orphan-no-replay` (an LG orphan plus an LG mismatch against a server hash, with no replay log), must FAIL.
      Today it is NOT-RUN.
- **The text:** the SERIES `decoded_hash_mismatch_count` row's Gaps gain:
  > the coverage rule (README §1's `null` rule, checked): every `hs` list and `tick.s.snap_incomplete` list holds the
  > run's `clients` entries; a (slot, tick) is resolved by a client hash (compared) or by `tick.s.snap_incomplete` 1
  > (lost, not compared), never both; per slot, every tick from its first resolved tick to its last at which the room
  > logged that slot's hash is resolved, and that span covers the room's measured ticks for the slot; a mismatch still
  > fails the run, and 0 compared still comes out `empty`.
  - README §1's writer side is FX-12's.
- **Proof:** new fixtures under `fixtures/pm-0001-17/`, each INVALID and each failing today:
  - `sparse` (probe S): one hash of six, nulls with snap_incomplete 0;
  - `sampled`: a client tick record missing inside the span;
  - `slot-unlogged`: a slot with measured server hashes and no client entry;
  - `hashed-and-incomplete`;
  - `short-lists`: 2 entries under `clients` 48.
  - They are judged in tests/fixtures.rs `pm_0001_17_every_server_hash_is_resolved_on_the_client_side`.
  - The six existing cases move to 48-entry lists. The extra slots may be null in both logs (clients not connected),
    so their results and counts stay as they are. `none-compared` still FAILs (empty), with its gaps beside it.

### FX-11: the verdict prints the practice outcome whole
SEC-F9 (the fix pass's half; the rest is follow-up FU-2) = NC-14.
- **The fix:** `Practice::describe` prints, in this form:
  > {n:?} {bound} x f {f} = {moved}, rounded up to the ms; practice run {id}: {class}, s = M_p {m_p} / M_m {m_m} =
  > {s}, band {low} to {high}, tolerance {tol}, f = {f} ({max(1, s) | FAR BETTER: at the bound})
  - Then "; chance ruling: {text:?}" and "; second ruling: {text:?}" when the record has them.
  - So a run judged under a ruling never reads as IN TOLERANCE.
- **The text:** SERIES's `header.params.started_utc` row gains -18 in its Marks column (NC-14).
- **Proof:** `u4j_ws_practice_puts_the_practice_runs_numbers_in_force` asserts that check (c)'s needs line holds the
  class, "band 0.965 to 1.035", "tolerance 1.10" and, in its BEYOND case, the chance ruling's text. Fails today.

### FX-12: README's `hs` writer rules, exact
NC-3 and NC-5 (ii) to (iv), with NC-11. The words below replace README §1's; my call: R2.6.
- **The `hs` bullet's first sentence:** "name → one entry per client slot of the run (index = slot, the same length
  in every record of the run)" becomes:
  > name → one entry per client slot of the run: exactly the header's `clients` entries in every record of every log
  > of the run, at index = the room's slot for that client (the slot the room's `tick.s` lists use; a client takes it
  > from `JoinAccepted`), the same in every log
- **The `server_hash` item:** after "never 0;" insert:
  > written only in the room's log (process `room`), for each connected client from its admission on, in every phase
  > the clients log; `null` for a client not connected and for a tick whose snapshot the room did not send that client
  > (one the WebSocket send budget skipped);
- **The `decoded_hash` item:**
  - "(the clients' log, at the tick t the client rebuilt)" becomes "(only in the clients' logs, process `client` on L1
    and `bots` on L2, at the tick t the client rebuilt)".
  - "a client with two hashes for one tick writes the sentinel there;" becomes:
    > a client with two different hashes for one tick writes the sentinel there (one hash met twice, as a duplicated
    > datagram gives, is written once), and no hash is ever dropped: the writer buffers until every earlier tick is
    > written, or ends its log `crash`. Every tick from a client's first snapshot to the end of its log is resolved for
    > that client: a hash, or `null` with `tick.s.snap_incomplete` 1 (a tick none of whose groups arrived counts too,
    > known from the next header's tick); after the measured phase the clients keep logging until every measured tick
    > is resolved, and a tick still in flight at the stop is not written;
- **The `replay_hash` item:** "(process `replay`, the x86-64 re-decode" becomes "(only in a `replay` log on x86_64,
  the x86-64 re-decode".
- **The `twin` role:** "named in the other's `params.twin`: PM-0001-3, -21)" becomes "named in the other's
  `params.twin`: PM-0001-3, -21; a run and its twin log `h` over the same ticks, since a tick hashed in one only counts
  as a mismatch)".
- **The SERIES rows** for `hs.server_hash`, `hs.decoded_hash` and `hs.replay_hash`: the same process words.
- **Proof:** fixture `pm-0001-17/permuted-slots` (two connected clients' entries swapped in the client log): FAIL. It
  passes today too. It documents that pairing is by the room's slot, and that the rule matters.

### FX-13: the practice run ran on the counted run's setup
TE-F16 (Low). It rides FX-3's change to `find`. My R7: "Valid only with ... a header naming the model, RS's kernel
and CONFIG_HZ".
- **The fix:** the practice run and the run (counted, or a bite run) must each have the same three things, else
  INVALID (absent or different):
  - `params.ws_tcp_model`, equal to the bound's `model`;
  - `params.config_hz`;
  - the room log's (machine.role RS) `machine.kernel`.
- **The text:** FX-3's paragraph already carries it. README §2.5's record bullet drops "the header checks" from the
  fields "for people".
- **Proof:** `u4j_ws_practice_refuses_every_number_not_set_by_its_rules` gains these cases, each INVALID and each
  failing today:
  - another kernel;
  - another config_hz;
  - another ws_tcp_model;
  - a ws_tcp_model that is not the bound's model;
  - each of the three absent.
  - The ws tests' headers gain the three params, with the provisional model's text (FX-6 (d)).

### FX-14: machine_named's remaining conditions, and a -1 fixture
TE-F13 (Low). It rides the push, because §5 U4j asks for machine_named "with fixtures" on -1's parts.
- **The fix:** tests only.
  - `u4j_machine_named_needs_every_field_and_a_vm_its_address`'s FAIL list gains role, name, plan and arch emptied,
    and mem_mb and threads_per_core at 0.
  - A fixture `pm-0001-1/laptop` (an ML or MAC log: every field, no address) PASSes its part's figure, in
    tests/fixtures.rs `pm_0001_1_a_laptop_needs_no_address`.
- **Proof:** the fixer plants "role not checked" once in figures.rs, in place and restored, and the new case fails by
  name.

### The fix pass's end, for the main session
- Every FX names its tests in the fixer's notes, with the failing line on the unfixed code (or the plant).
- fixtures/README.md's table is updated.
- The new public items are listed by crate (at least `Mark`'s `ws_bound_line`).
- marks.json's changes are stated as before and after (-18: `ws_bound_line`, the notes sentence).
- The checks in R1's preamble pass twice.

### Follow-ups (not before the merge; each goes in PR #40's report, with its owner)
- **FU-1 (SEC-F7, Low):** C4's bite direction, "at least one log" (reading (a), accepted). Once the run plans know
  which process carries each bite, marks.json names it per bite and C4 checks that log. Owner: R3's run-plan unit
  (the first L2 bite runs). Today it fails safe: such a run is usually Missed, so the mark FAILs.
- **FU-2 (SEC-F9, its later half, Medium):** rulings by id. Before any counted WebSocket run can rest on a ruling:
  - the advisor's rulings of chance and of a found cause live in a committed file, each with an id, the practice
    run's id and its text;
  - the record names the id;
  - the judge refuses an id that is not in the file, or one that names another run.
  - Owner: the unit that builds the run script's practice record, before R3's run plan goes to Mustafa.
- **FU-3 (SEC-F10, Low; it was there before U4j):** npt-log reads a duplicate JSON key silently, the last one
  winning. A refusing pass in `read_log`. Owner: the next change to npt-log; U4r's brief names it.
- **FU-4 (SEC-F12, Low):** a run of the WebTransport part labelled `ws`. Once npt-server writes `burst_return`, the
  `L2 B1 ws` part gains a setup check that the WsServer-only series exist (`event.burst_return.budget_bytes` count
  >= 480). Owner: R3's run-plan unit.
- **FU-5 (TE-F5's C3 and NC-6's second option, Low):** the mark's own `arm_rule`. `--check-marks` refuses a mark with
  `arm_rule` false whose check reads `decoded_hash_mismatch_count` on a part that can select an LG log. No mark does
  so today: -58 is L1 only. Owner: the first unit that adds such a part.
- **FU-6 (mine, A-U4J-1, Medium):** R7's message-size condition goes unchecked by the judge: "a mean WebSocket
  snapshot message size within 5 % of the size the bound used". The line has no size field, and the practice run has
  no size series. The run script's class enforces it today.
  - The fixes.rs bound unit adds the mean message size to the line. If v1 lines are in use by then, the tag becomes
    v2.
  - The judge checks the practice run's mean against it once U4r logs per-message sizes (§2's "Sizes for fixes.rs"
    row).
  - Owner: the fixes.rs bound unit, before R3's run plan.

### Carries
- **U4e (the room and the client hash):**
  - the room logs `server_hash` for each client from its admission, in every phase, and null for a snapshot it did
    not send;
  - the client files each hash under its rebuilt tick, at the room's slot from `JoinAccepted`;
  - two different hashes give the sentinel; no hash is dropped;
  - a tick none of whose groups arrived counts in `snap_incomplete`.
  - All of these are FX-12's words.
- **U4r (the lab's logs and the contract test):**
  - the writer rules of FX-5 (a), FX-10 and FX-12: each name in its own process; lists of `clients` entries; the
    after phase until every measured tick is resolved; a run and its twin hash the same ticks;
  - the contract test shows `decoded_hash_mismatch_count` resolving with no gap on every -17 suite;
  - NC-10 (R2.4).
- **U6:**
  - U6-a: -30's twin form (each other's measured runs, differing in `params.transport`) beside FX-4's role check
    (SEC-F5's second half);
  - U6-b (NC-13): C4's role list refuses PM-0006-20's logging-off twin of its planted-format bite run, while
    `twin_of` asks for the same bite; U6's `alloc_twin_diff` settles it;
  - U6-c: -19's join hash against FX-10's coverage rule. A trapped and rejoining client must leave no unresolved
    server tick, and ML's `browser` process joins FX-5 (a)'s list when -19's figure is built.

## R2. The calls you asked for
- **R2.1, the coverage floor and the null reconciliation (TE-F10 = SEC-F8 = NC-5 (i)): FX-10's form, before the
  merge.**
  - Why before:
    - It checks the README's own rule ("`null` for a tick with a group lost and none refused (counted in
      `tick.s.snap_incomplete`)"), so it adds no new bar.
    - Without it, "100 % of decoded snapshots" passes on 1 hash of 6.
    - It is the input contract U4r writes to, so fixing it later means writing the logs twice.
  - Why this form: it is per slot and per tick against the room's own hashes. The span keeps an honest log's head and
    tail (before the first snapshot; in flight at the stop) from being gaps. The measured-phase cover stops a log that
    starts late or skips a slot. The order (f) keeps C2: a gap never hides a mismatch or an empty comparison.
  - Not taken: a count-only floor (compared + incomplete = server pairs). It cannot say which tick is missing, and a
    sampled log can still balance it.
- **R2.2, rulings as any non-empty text (SEC-F9).**
  - Now: FX-11, the verdict prints the class and each ruling's text with M_p, M_m, s, the band, f and the tolerance.
    That meets my R8 (v), "the ruling is in the run's report for Mustafa".
  - Later: FU-2, rulings by id in a committed file, before R3's run plan goes to Mustafa.
  - It stays a security finding left open until FU-2, so the PR stays needs-mustafa. It is that already.
- **R2.3, pinning the bound line (SEC-F2 = TE-F6): FX-6's form.** The line goes verbatim in marks.json
  (`ws_bound_line`), and the record must equal it byte for byte.
  - Why not a sha256 or a few numbers: the verbatim line is readable in review, pins every field the judge reads
    (M_m, the band, the guard, the model, the commit), and needs no hash code.
  - The provisional model text makes the placeholders unusable for a real run (FX-6 (d)).
- **R2.4, NC-10 (a P0 setup check that `tick.s.snap_incomplete` sums to 0): not now; a U4r carry, evidence first.**
  - With FX-10 in, a refused group can hide only as "lost and counted". On L1 P0 (no loss: l1/src/lib.rs:116) that
    count must be 0.
  - U4r's contract test reports the count on each short L1 P0 run. If it is 0 on every one, U4r adds to -17's L1 P0
    part, with a bite (a client that writes null and incomplete 1 for a refused group):
    `{"name": "setup: L1 P0 loses no group (no client-tick left incomplete)", "series": "tick.s.snap_incomplete",
    "stat": "sum", "window": "all", "op": "==", "value": 0, "parts": ["L1 P0"], "setup": true}`.
    It goes in before -17's judged runs.
  - If the count is not 0, the cause comes to me first.
  - Why not now: a check whose writer is not built could make every correct L1 P0 run INVALID (admission, the stop)
    before anyone has seen one.
- **R2.5, TE-F16 (the kernel, config_hz and ws_tcp_model compared between the practice and the counted run): yes,
  now (FX-13).** It is my R7's own validity condition, it rides FX-3's change to `find`, and it is a few lines.
- **R2.6, README's `hs` writer rules (NC-3, NC-5 (ii) to (iv)): accepted whole, before the merge, in FX-12's words.**
  - NC-5 (i) is FX-10.
  - NC-11, the twin's same tick range, rides with them.
- **R2.7, the Arm rule's key (SEC-F4 = TE-F5, F11, F12; reading (e)): FX-5.**
  - The process per hash name.
  - LG and aarch64, with the room on x86_64.
  - An orphan counts everywhere.
  - A missing replay entry is a gap.
  - Fixtures on the real marks.json's L2 part.
  - The mark's own `arm_rule` is FU-5.
  - Why LG: ADR 0001's -17 and -19 rows say "LG's aarch64", and -19 judges ML's Chrome "as it stands", so the
    machine's role, not only its arch, is the key.

### Every finding, mapped (so nothing is dropped)
| Reviewer | Finding → where it goes |
|---|---|
| netcode | 1 → FX-1 · 2 → FX-2 · 3 → FX-12 · 4 → FX-4 · 5 (i) → FX-10 · 5 (ii) to (iv) → FX-12 · 6 → FX-5 (b), FU-5 · 7 → FX-7 · 8 → FX-5 (a) · 9 → FX-6 · 10 → R2.4 (U4r) · 11 → FX-12 · 12 → FX-5 fixtures · 13 → U6-b · 14 → FX-11 |
| security | F1 → FX-1 · F2 → FX-6 · F3 → FX-3 · F4 → FX-5 · F5 → FX-4, U6-a · F6 → FX-8 · F7 → FU-1 · F8 → FX-10 · F9 → FX-11, FU-2 · F10 → FU-3 · F11 → FX-6 (f) · F12 → FU-4 |
| test | F1 → FX-1 · F2 → FX-2 · F3 → FX-3 · F4 → FX-4 · F5 → FX-5 (C3: FU-5) · F6 → FX-6 · F7 → FX-7 · F8 → FX-8 · F9 → FX-9 · F10 → FX-10 · F11 → FX-5 (c) · F12 → FX-5 fixtures · F13 → FX-14 · F14 → FX-4 · F15 → FX-3 · F16 → FX-13 · R1 → FX-9 · R2 → FX-10 (a) |
| advisor | A-U4J-1 → FU-6 |

## R3. The builder's readings (a) to (f)
Quoted from the builder's handback ("Readings for the advisor or reviewers"). They are not in u4j-build.md, so the
main session should copy them into its notes at the merge.
- **(a) "C4's 'at least one log' form", because -4's U1 fixture has RS's room on the bites build while LG's bots run
  the measured build: ACCEPTED.**
  - netcode: right. security F7: sound in the measured direction, and weaker in the bite direction, where it fails
    safe.
  - Recorded as a risk; tightened by FU-1.
- **(b) "A bite run under ws-practice takes its set's measured-build practice run": CHANGED (FX-3 (c)).**
  - netcode: right, under R11.
  - Kept: the bite run is judged at the counted run's numbers.
  - Changed:
    - the practice run is the one in force for its set's measured runs on the same layer, profile and transport
      (one build), found over every run the judge reads, with FX-2's start rule;
    - so a practice run of an older build elsewhere cannot make it INVALID;
    - and the bite run cannot take an f other than its counted run's.
- **(c) "The ruling's `machine.config_hz` is written as `params.config_hz`", since a new Machine field would break
  npt-lab's machine.rs: ACCEPTED.**
  - No reviewer objected.
  - My ruling's text (advisor-ws-docs-r2.md:377, SERIES (5)) reads as `params.config_hz` from now on.
  - FX-13 compares it.
- **(d) "A bite run that compared nothing is INVALID": ACCEPTED.** All three agree. It can only lower a verdict. It
  follows README §2.3's rule for setup checks: a run that did not test the mark shows no bite.
- **(e) "The Arm rule is keyed on the client log's `machine.arch` = aarch64, not on `mark.arm_rule`": CHANGED
  (FX-5).**
  - netcode F6 (partly), security F4, test F5 and F11 (wrong).
  - The key is now:
    - an LG log on aarch64, with the room on x86_64;
    - each name from its own process;
    - an orphan counts everywhere;
    - a missing replay entry is a gap.
  - The mark's flag is FU-5.
- **(f) "The decoded-hash figures read every phase", because -58's wraps fall in its first seconds: ACCEPTED.**
  netcode: right, and it makes NC-5 (ii) a writer rule. That rule is in FX-12's `server_hash` words (from admission,
  in every phase the clients log).

## R4. Severity
- **F1 (NC-1 = SEC-F1 = TE-F1): High, confirmed.** netcode labelled it Medium; I set it High, in writing.
  - The reason: a rule loosened beyond Mustafa's words. A log can make the judge move X, Y and Z by 90 %, while his
    words cap the move:
    - WS-BAR's refinement (2026-11-07): "If the practice run is up to 10 % slower than the model, the numbers may move
      by that much; beyond 10 %, investigate first and never loosen".
    - NETEM-BAR (a) (2026-11-10): "The bar stays capped as WS-BAR says".
  - Probe A shows the move (X' 1.273 s), and nothing but a non-empty ruling text stands in the way.
- **How round 2 proves it closed:**
  1. The fixer's notes show `the_bound_line_refuses_a_band_that_leaves_out_1` and
     `u4j_ws_far_better_past_the_cap_is_invalid` failing on the unfixed code, then passing.
  2. The test-engineer re-runs probe A on the head: INVALID. It then runs three plants, one at a time, each caught by
     name:
     - plant T (cap 1.12, band_low dropped, Z' recheck dropped): caught by FX-9's test;
     - "parse_bound's band check removed": caught by FX-1's unit test and FX-6's marks test;
     - "FAR BETTER f = max(1, s), with the band check and the pin removed": caught by
       `u4j_ws_far_better_past_the_cap_is_invalid`.
  3. The security-auditor writes a table of every path to `Ok` in `find` (class × s range → f) showing f <= the
     tolerance on each, with the line pinned by FX-6.
  4. The netcode-architect checks README §4's new paragraph against my R6 and R8 (v) and Mustafa's two quotes above.
  - If any of these is open after round 2, F1 alone gets a round 3. If it is still open after round 3, the
    change-set stops for Mustafa.
- **The others:** the reviewers' labels stand. FX-2, FX-3, FX-4, FX-5, FX-6, FX-7, FX-8, FX-9 and FX-10 are Medium;
  FX-13 and FX-14 are Low riding the push; FX-12 and FX-11 are Medium (the text U4r writes to; the report for
  Mustafa).

## R5. Decisions needed
**None.** Every call above is technical, inside the approved plan and my rulings on it, and none loosens a bar: each
fix is stricter or restores a ruled rule. Two notes for the main session (no decision):
- **The readings list** (u4-build-plan.md §10's preamble and §11; the readings PR before R5) gains one entry, exact
  words: "PM-0001-17's Arm rule key ('counts only once that client's logged bytes, decoded again ... on x86-64, miss
  the server's logged hash too': ADR 0001:1605) read as applying only where the server logged a hash: a client hash at
  a tick the server never logged counts on every machine (the U4j fix pass, the advisor's FX-5 (c)). Stricter."
- **Open security findings** at the merge: FU-1 to FU-5 (SEC-F7, the later half of F9, F10, F12, and C3's flag). They
  keep PR #40 needs-mustafa, which it is already, and each is named in its report.

## For Mustafa
Nothing now.

## What I checked, and what I did not
**Checked (read only; nothing edited, built or run in the worktree):**
- My role, from origin/main:.claude/agents/advisor.md, word for word.
- The worktree is at 411eeeed50d8c71e7e1f74366f0e1d5c27388444: 11 files modified, 22 untracked (21 fixtures and
  practice.rs), plus u4j-build.md.
- The code:
  - practice.rs, whole: parse_bound :50-104, class_ok :384-391, f :406;
  - figures.rs :100-360: twin_of, twin_hash_mismatch_count with no role check :195-209, slot_hashes reading every
    log :215-252, decoded with the orphan-as-agreement compare and the early Waiting :257-336;
  - eval.rs: Run and serves; in_force; always_rules (C4) :541-576; the ws-practice rule :998-1002; set_of
    :1161-1163; judge_run; eval_bite; judge_mark; judge, which shows `all` holds every run the judge reads;
  - marks.rs's ws_bound shape check :587-621 (op not pinned: SEC-F11);
  - the README diff, whole;
  - marks.json -1, -17, -18, -21, parsed.
- The test-engineer's outputs probe-run1.txt, probe-run2.txt and plant-t.txt. Their results (A, B, C1-C3, D, E0-E2,
  F, G, H0-H1, T1a-T3, S, O) match the reviews' claims.
- The binding texts:
  - u4-build-plan.md: §2, §4.6, §4.12, §5 U4j, and Q11, Q20 and Q22's options;
  - advisor-u4.md: the verdict and conditions 1-13;
  - advisor-u4-wave1.md W1-N2;
  - advisor-ws-docs-r2.md :375-400 (R3-R12, and the -18, ws-practice and record lines);
  - history.md :292-294 (NETEM-BAR (a), verbatim) and :381-391 (WS-BAR and its refinement, verbatim);
  - plan.md:359 (the Arm rule);
  - ADR 0001 :1399, :1605 (-17's key) and :1609 (-19's ML "as it stands").
- Cites checked:
  - l1/src/lib.rs:114-116 (P0: "0 ms, no jitter, no loss");
  - the full SHA of bf9d248 (bf9d24807106c4524d1aa0849409d219259c3943, by `git rev-parse` in the worktree);
  - Machine has `kernel` (log/src/lib.rs:24-41);
  - L1's client process is `client` (fixture headers);
  - SERIES :306's started_utc row lacks -18 (NC-14).
- The builder's readings (a)-(f): from its handback in the session's agent transcript, since u4j-build.md has none.

**Not checked:**
- No probe or plant re-run, and no check.mjs run: I build and run nothing. The counts 304 / 327 / 48 and the probe
  results are the builder's and the test-engineer's, read from their notes.
- I did not read engine.rs or fixtures.rs whole. The test names I cite are from grep. The test-engineer's claim that
  65 committed tests survive plant T is theirs.
- Whether FX-10's coverage rule fits U4r's real writer at admission and at the stop. No such log exists yet. U4r's
  contract test is where that shows, and R2.4 keeps the one check with no evidence out until it does.
- u4-build-plan.md's W1 text batch (sha256 now a81b230f…, per the checkpoint) against U4j's section: I read the
  section as it stands, not the diff from 0d69332c.
- Whether `Mark`'s new field can be `pub String` without exposing practice.rs's pub(crate) `Bound`. That is the
  fixer's to settle, and to list under A19.





````

### Round 1's findings: netcode-architect
_`.checkpoints/netcode-proof-test/u4j-review-netcode.md` lines 1-102 (sha256 39c9db9fd7b9), as they are:_

````text
# U4j review, round 1: netcode-architect (read only)

Worktree /home/user/Shamble-Frontlines/.claude/worktrees/agent-a3b8dfeb01ed52b50, base 411eeeed50d8c71e7e1f74366f0e1d5c27388444.
Started 2026-11-11. No memory found; no earlier notes file.

## Plan
1. Read the sources: u4-build-plan.md §4.6, §4.12, §5 U4j (:626-646), §2 rows (:147-154); advisor-u4.md conditions
   2/4, nits c/f, Q20, Q22, U4S-BUILDER; advisor-u4-wave1.md W1-N2; plan.md PM rows for -17, -18, -20, -21, -22, -58;
   advisor-ws-docs-r2.md :373-383 (bf9d248's -18 numbers, ws-practice); .claude/rules/netcode.md; base README/SERIES.
2. Read the diff: log/src/lib.rs (hs), judge figures.rs (five figures), eval.rs (Empty, C4, ws_bound), marks.rs,
   practice.rs, marks.json changes, README/SERIES deltas, fixtures.
3. Check focus (1): the five figures against §4.6 and SERIES (pairing, complete ticks, sentinel, null, orphan, empty,
   server 0, twin per tick, cross-arch hash kind, machine_named).
4. Check focus (2): marks.json changes against PM rows and §4.12 (selects, -18 split, ws_bound numbers, WS-BAR cap,
   -22 exception, -58 W1-N2 text); no bar loosened.
5. Check focus (3): readings (a), (b), (d), (e), (f).
6. Check focus (4): writer rules for U4r and U4c L-c8 re-judge: complete and unambiguous; any rule a runner can meet
   while the figure misjudges.
7. Findings, one line each below as found; then the verdict.

## Findings (one line each, as found)
- Sources read: u4-build-plan.md §2 (:141-160), §4.6 (:296-318), §4.12 (:396-412), U4j (:626-645), U4r (:717-772),
  L-c8 (:574-576), Q11/Q20/Q22 (§10); advisor-u4.md verdict + conditions 1-13; advisor-ws-docs-r2.md R3-R11 and the
  -18 spec (:374-379); advisor-u4-wave1.md W1-N2 (:305-351) and NETEM-BAR (:534-547, :607-620); history.md:292-296
  (Mustafa's NETEM-BAR (a) words); plan.md:359 (Arm rule: "LG's aarch64 clients"); bar-fix1-bf9d248.txt (X 0.67 s,
  5 over 2.50 s, Z 6.90 s: matches marks.json).
- Diff read: log/src/lib.rs, judge figures.rs, eval.rs, marks.rs, practice.rs (whole), marks.json (-17, -18, -20, -21,
  -22, -58), README.md, SERIES.md; fixtures pm-0001-17/*; engine.rs decoded/twin tests.
- F1 (Medium, before merge) practice.rs:406 f = max(1, s) also for FAR BETTER; parse_bound (:87-90) never checks
  band_low <= 1 <= band_high, and nothing checks f <= tolerance. A bound line with band_low > tolerance (e.g. 1.2) plus a
  chance ruling on FAR BETTER at s = 1.15 gives f = 1.15 > 1.10: the only path where the judge moves numbers past
  Mustafa's cap. R8 (v) says FAR BETTER runs "at the bound" (f = 1). Fix: f = 1 for FAR BETTER, band_low <= 1 <=
  band_high in parse_bound, assert f <= tolerance. Proof: a unit test with that line.
- F2 (Medium, before merge) practice.rs:259-286,325: the latest practice run of the build is taken over ALL candidates,
  then refused if it started after the counted run. A practice run added to the set after the counted run (R8's
  investigation, same build) turns that counted run's FAIL into INVALID (or "2 practice runs" INVALID): R9 says a failed
  counted run stands as FAIL; a gap/invalid must never hide a failure (C2). Fix: only practice runs started before the
  run count (and the one-per-build rule over those). Proof: engine test, counted run FAILs, add a later practice.
- F3 (Medium, before merge, README text) README §1 hs "index = slot" never says whose slot: no admit field carries it,
  and hs is the first list paired across two logs (room vs client). U4r's runner could index the client log by spawn
  ordinal and the room by admission order: every pair mismatches (a correct build FAILs), or -19's slot reuse misaligns.
  Fix: one sentence: index = the room's slot for that client (the one the room's tick.s lists use; a client takes it from
  JoinAccepted), same in every log. Proof: a fixture with permuted client slots must FAIL (shows the rule matters).
- (process note: F1-F3 were appended with a shell heredoc by mistake; the Edit tool is not available here, so this
  file is rewritten whole with the Write tool from now on, F1-F3 kept word for word.)
- F4 (Medium, before merge) figures.rs:195-209 twin_hash_mismatch_count calls twin_of with no role check ("any role",
  for -30, which is NOT-RUN until U6 and whose pre-check is never judged). For -21 a forged run that names another
  forged run of the same seed/profile/bite (a re-run, a duplicate) compares forged with forged: equal hashes, PASS of
  "0 effect beyond the valid part" without ever meeting the valid-parts twin. README §1 and -21's notes say role twin;
  fate_log_twin_equal keeps the check (:159). Fix: require role twin now; -30's form at U6. Proof: engine case, twin
  role measured -> INVALID.
- F5 (Medium, before merge, README text; the log format U4r writes) README:43-56's hs writer rules leave gaps a runner
  can meet while the figure misjudges: (i) nothing says every completed tick carries its hash (hs is optional per
  record): a runner that samples every 10th tick passes -17's "100 % of decoded snapshots" on a 10 % sample; (ii) the
  figure reads every phase (reading f), so the room must log server_hash from each client's admission in every phase
  the clients log, else warm-up orphans FAIL a correct build; (iii) "two hashes for one tick writes the sentinel"
  (:54) should say two different hashes (an L2 duplicate datagram re-completing a tick would FAIL a correct build);
  (iv) a hash that arrives for a tick already written must never be dropped (buffer the whole run, or end `crash`).
- F6 (Low, follow-up; reading e) figures.rs:219 the Arm rule keys on machine.arch == aarch64 alone; plan.md:359 says
  "LG's aarch64 clients". A non-LG aarch64 client (ML if it is an ARM laptop: -19's ML Chrome part) would come out
  NOT-RUN ("no x86-64 re-decode") instead of FAIL on a real mismatch; an arm_rule=false mark (-58) would excuse one if a
  run ever ran on an aarch64 host. -58 is C-only today, so no live exposure. Fix: key on machine.role LG and aarch64
  (or pass the mark's arm_rule). Owner: before -19's ML part (U7).
- F7 (Medium, follow-up acceptable, fold in if cheap) practice.rs:372-376 the practice run's heal count is never checked
  against the bound's client_bursts_count (parsed at :23, unused). R7: "Valid only with 480 heal records". A practice
  run missing records gives M_p from a subset, so s and f move (either way, inside the cap) and a broken setup can
  pass as IN TOLERANCE (its count/max re-check is easier with fewer records). Fix: heal.len() == client_bursts_count.
  Proof: ws case with 479 heals -> INVALID.
- F8 (Low, follow-up) figures.rs:282 the replay log's arch is never checked (slot_hashes reads replay_hash from any
  log); the Arm rule is "replayed on x86-64". An aarch64 replay agreeing with the server would excuse a live-path
  mismatch as cross-arch. Fix: replay_hash only from x86_64 logs, else a gap.
- F9 (Low, follow-up, the fixes.rs unit) the record's bound line is tied to marks.json only by X, Y, Z, k and the
  tolerance; M_m and the band come from the line unchecked (bound_file/bound_sha256 are not read), so a stale bound
  file's M_m moves f anywhere in [1, 1.10]. Pin the bound's sha256 (or M_m) in marks.json when the real bound lands.
- F10 (Low, follow-up, for the advisor) -17: a client that logs null for a refused group (or for a group whose header,
  hence tick, it cannot read) hides the refusal; on L1 P0 (Profile P0 = 0 loss, l1/src/lib.rs:116) the incomplete
  count must be 0, so a P0-part check `tick.s.snap_incomplete sum == 0` would close it. Stricter, not in the plan.
- F11 (Low) twin hash: "a tick hashed in one only counts" (figures.rs:192) needs the writer rule that a run and its
  twin hash the same tick range; README does not say it (a twin stopped one tick early FAILs a correct build).
- F12 (Low) cross_arch_mismatch_count: U4j's Builds asks for fixtures ("with fixtures only since no U4 run feeds
  them"); only engine tests exist (tests/fixtures.rs has no aarch64 or replay case). Test-engineer's call.
- F13 (Low, U6 forward) C4's role list (eval.rs:555: bite, twin) will refuse PM-0006-20's logging-off twin of its
  planted-format bite run, while twin_of demands the same bite: U6's alloc_twin_diff must settle it.
- F14 (Low) -18's reports lack the practice outcome's band and tolerance (the ruling's list: M_p, M_m, s, the band, f,
  the tolerance); describe() (practice.rs:144-155) prints four of six. SERIES's header.params.started_utc row (:306)
  lacks -18 in its Marks column.
- Focus (2) checked: -17 L1 P0/P2/P3/B1 gain SS, 48, pre (S30 and L2 parts unchanged); -18 three parts with exactly
  the ruling's selects, checks (a)-(e), bf9d248's 670000000 / 5 over 2500000000 / 6900000000, "provisional" in every
  name and the notes, ws-practice:1.10 capped in marks.rs and parse_bound; -20 two L1 parts; -21 eight; -22 scene and
  clients only, README §4 exception written; -58 W1-N2 words verbatim. No bar loosened by the marks text.
- Readings: (a) right (L2's bite lives in RS's binary; "at least one" fails toward Missed, never a false show); (b)
  right (R11: the bite is judged at the counted run's numbers; its own practice would be BEYOND); (d) right (like a
  setup failure: the run did not test the bite; INVALID never passes); (e) see F6; (f) right (-58's notes say window
  all; stricter for -17) but it makes F5 (ii) a writer rule. None needs the advisor except F10 (a new check).
- Focus (1) holds otherwise: pairing by (slot, tick) over every non-null client entry; sentinel counts; null skipped;
  orphan counts; 0 compared -> Empty (fails, no bite shown); server 0 -> gap; twin per tick incl. one-sided ticks;
  cross_arch = aarch64 mismatches the replay does not reproduce (both absent = agreement, consistent with the rule);
  machine_named covers every Machine field (log/src/lib.rs:24-42).

## Verdict
REVISE (round 1): F1, F2, F3, F4 and F5 before merge (each cheap: guards in practice.rs, a role check, README text);
F7 recommended with them; F6, F8-F14 follow-ups. DONE.
````

### Round 1's findings: security-auditor
_`.checkpoints/netcode-proof-test/u4j-review-security.md` lines 1-150 (sha256 0b86a8bf62ef), as they are:_

````text
# U4j review, round 1: security-auditor (validity rules)

Reviewer: security-auditor (opus). Read only: no edit, build or run in the worktree
/home/user/Shamble-Frontlines/.claude/worktrees/agent-a3b8dfeb01ed52b50 (base 411eeeed50d8c71e7e1f74366f0e1d5c27388444).
Started 2026-11-11. No earlier notes file (fresh run). No agent memory folder found (memory: local; none on disk).
Paths below are under spikes/netcode-proof/ in that worktree. Line numbers are the worktree's.

## Plan
1. Read the diff (git diff base, git status untracked) and the files: log/src/lib.rs (hs), judge marks.rs, eval.rs,
   figures.rs, practice.rs, lib.rs; tests engine.rs and fixtures.rs only as needed; marks.json diff; README/SERIES diff.
2. Read the plan's rules: u4-build-plan.md §5 U4j (:626-646), §4.6, §4.12, §2 rows (:147-154); advisor-u4.md
   conditions 2/4, nits c/f, Q20, Q22, :365-371; advisor-u4-wave1.md W1-N2; advisor-ws-docs-r2.md :373-383 (ws-practice).
3. Focus items, one block each, a notes line per finding:
   (1) C4 rules and order; mixed-build hiding a bites log behind an honest one (reading a).
   (2) hs format: writer and reader, 16 lowercase hex or null, sentinel, duplicate / out-of-order ticks, uneven lengths.
   (3) Outcome::Empty and reading (d).
   (4) ws-practice: f bound by tolerance? WS-BAR cap; second practice run without a ruling; can a log supply a ruling.
   (5) reading (e): aarch64 claim dodging a check.
   (6) hostile input on parse_bound, practice record, bound line, hs: panics, overflow, NaN/inf, duplicate keys,
       trailing data.
   (7) redact check and marks drift test unweakened.
4. Anti-cheat-style coverage table for this unit: input -> where the judge validates -> gap.
5. Verdict and findings to StructuredOutput.

## Progress
- Read: git diff (11 files + 22 untracked), log/src/lib.rs, judge marks.rs, eval.rs, figures.rs, practice.rs, ratio.rs;
  u4j-build.md; u4j-brief.md; u4-build-plan §5 U4j, §2 C4 row, §4.6, §4.12; advisor-u2 C4 (:101-103);
  advisor-ws-docs-r2 :375-378, R7 :397, R8 :398; ADR 0001 -18 row :1608; marks.json -18, -21, -30; README diff;
  SERIES rows :266, :294 (head) and the base rows; redact-check.mjs header; marks.json base-vs-head per field.
- (Process note: the F1 line below was first appended with a shell heredoc by mistake; this file is now rewritten whole
  with the Write tool, content unchanged.)

## Findings
- F1 HIGH practice.rs:384-391 (class_ok), :406 (f), :87-90 (band check in parse_bound). FAR BETTER caps nothing.
  class_ok "FAR BETTER" needs only a non-empty chance_ruling and s < band_low; parse_bound accepts any band_low in
  (0, band_high] (no band_low <= 1 <= band_high), then f = max(1, s). Exploit: bound line band_low "2.0" / band_high
  "2.0", class FAR BETTER, chance_ruling "x", M_p = 1.9 x M_m -> f = 1.9: X, Y, Z move 90 % (X 0.67 s -> 1.273 s),
  past Mustafa's 10 % and R8 (v) ("FAR BETTER: at the bound"). Fix: FAR BETTER forces f = 1; parse_bound refuses
  band_low > 1 or band_high < 1; refuse any f > tolerance before Ok. Proof: engine test from
  u4j_ws_practice_puts_the_practice_runs_numbers_in_force with that line and class: must be INVALID (today PASS).
  Before merge.
- F2 MEDIUM practice.rs:331-338, :383 (M_m read from the record's copy); README §2.5 (bound_file, bound_sha256 read by
  nothing); marks.json -18 holds no M_m, band, guard or commit. The judge pins X, Y, Z, k and the tolerance to
  marks.json but takes M_m, the band, guard_share, model and commit from the log's verbatim copy. Exploit: a record
  carrying a stale or edited bound line (M_m 600 ms for a true 654 ms) turns a practice run at true s = 0.98 (f should
  be 1) into s = 1.07 IN TOLERANCE -> f = 1.07: numbers looser than "a slowdown measured within the tolerance"; a band
  with a very low band_low skips FAR BETTER's investigation. Fix: marks.json -18 carries the bound line verbatim (a
  string; no hash dependency needed) and the rule requires the record's `bound` to equal it byte for byte (the
  provisional line now; fixes.rs's later). Proof: engine test with m_m_ns changed in the record only -> INVALID.
  Before merge (it fixes the marks.json and record format fixes.rs and U4r build on).
- F3 MEDIUM practice.rs:259-268 (candidates from `set` only, `p.serves(&mark.id)`), eval.rs:1176. "One practice run
  counts per build" is checked inside the counted run's set only, and only among practice runs that list -18.
  Exploit: practice P1 (same build) BEYOND in set r3-vs4; practice P2 IN TOLERANCE at f = 1.08 in set r4-vs2 with the
  counted run: the judge sees one candidate, needs no second_ruling, applies f = 1.08 ("nobody can shop for a better
  factor", R7). Same with a first practice run whose header does not list the mark. Fix: count candidates over every
  run the judge was given (all sets), role practice with the same binaries, listing the mark or not. Proof: fixture
  with the two practice runs in two sets -> INVALID without second_ruling. Before merge (wrong verdict).
- F4 MEDIUM figures.rs:215-219 (arm = the log's own machine.arch), :271-282 (every name read from every log),
  :320-331 (orphan with no replay entry = agreement). Base SERIES contract was "with arm_rule on L2, only those the
  x86-64 re-decode (process `replay`) also misses"; reading (e) widens it. Exploits: (i) one client log with arch
  aarch64 that also carries hs.replay_hash equal to the server's turns every mismatch into cross_arch -> PASS where
  FAIL; (ii) -58 (arm_rule false, L1 only) gets the Arm rule if a log says aarch64; (iii) an aarch64 orphan (client
  hash, no server hash: a wrong tick label) with no replay entry counts as cross-arch, not a mismatch; (iv) a run whose
  room is aarch64 too still gets the leniency. Fix: read server_hash only from process room logs, replay_hash only
  from process replay logs on x86_64 (gap otherwise), decoded_hash only from client processes; apply the Arm rule only
  where the mark has arm_rule, the room log is x86_64 and the client log aarch64; an aarch64 mismatch is cross-arch
  only when the replay has an entry equal to the server's Some(hash) (absent entry: gap). Proof: fixture
  pm-0001-17/differ with the client log's arch aarch64 and an hs.replay_hash list in the same log -> expected FAIL,
  today PASS with "aarch64 only" 1. Before merge (wrong verdict; figure input rule later units build on).
- F5 MEDIUM figures.rs:195-209 (twin_hash_mismatch_count: twin_of with profile, seed, bite; any role). The twin's
  identity is not checked: SERIES :294 says "-21's twins are role twin, -30's each other's measured runs", the judge
  enforces neither. Exploit: two forged measured runs, same seed, naming each other in params.twin: both carry the same
  forged stream, so equal hashes hold even if a forged command leaked -> "forged: 0 effect" PASS where FAIL. -30: a
  WebTransport run naming another WebTransport replay of the same recording -> PASS without testing the other
  transport. Fix: the twin must be role twin, or the two runs name each other and differ in params.transport (the
  -30 form); gap otherwise. Proof: fixture pm-0001-21/twin-is-forged (two measured forged runs naming each other,
  equal h) -> gap/INVALID, today PASS. Before merge (wrong verdict).
- F6 MEDIUM practice.rs:56-68, :87-90, :387 with ratio.rs:25-28, :135-141. parse_bound panics on log data: two
  30-digit decimals (band_low "0.12345678901234567890123456789", band_high "0.99999999999999999999999999999") reach
  `band_low > band_high`, whose cross product (~1e57) overflows i128 and hits checked()'s expect: the judge crashes
  instead of INVALID. Same for `s >= band_low` with a long band_low. Fix: README §2.5 caps each fraction at 6 decimal
  places and parse_bound refuses more (or comparisons return Err). Proof: unit test in practice::tests with that line
  -> Err (today a panic). Before merge (format fixed now, nit f).
- F7 LOW eval.rs:549-576 (C4, reading (a)). Measured direction is sound: any bites log in a run naming no bite is
  INVALID, so no bites component hides behind an honest log. Bite direction is weaker: "at least one log" lets a bite
  run whose bite-carrying process (RS's room) ran the measured build pass C4 when only LG's bots are bites-built; such
  a run is usually Missed (mark FAIL, safe) and only a chance failure would show the bite. Accept reading (a) as a
  recorded risk; tighten later by naming each bite's process in marks.json. Follow-up.
- F8 MEDIUM figures.rs:298-343 (compared counts any non-null entry, every phase); marks.json -17 (compared and
  snap_incomplete only reported). No coverage floor: a client log with one matching hash and null everywhere else
  (or hashes only in warmup) gives compared 1, mismatches 0 -> PASS; nulls are never reconciled with
  tick.s.snap_incomplete or L1's fate log, so a refused group written as "lost" (null) hides. Fix (beyond the approved
  plan: the advisor's call): on P0 parts every server (slot, tick) is compared; elsewhere compared + incomplete =
  server pairs in the measured phase, as a check. Proof: fixture pm-0001-17/sparse (one entry, rest null) -> today
  PASS. Advisor's ruling, not a builder fix.
- F9 MEDIUM practice.rs:300 (second_ruling), :385 (chance_ruling), :144-155 (describe). Both rulings are any non-empty
  text in the log; the judge cannot tell the advisor's ruling from "x", and the verdict prints neither the class nor
  the ruling, so a run judged at f = s under a BEYOND + chance record looks like IN TOLERANCE (R8 (v): "the ruling is
  in the run's report for Mustafa"). Fix now: describe() prints the class and the ruling texts; later (advisor): rulings
  named by id in a committed file. Proof: engine test asserts the needs line contains the chance_ruling text. Before
  merge (cheap).
- F10 LOW log/src/lib.rs:470-471 (pre-existing). read_log parses each line into serde_json::Value first, so a
  duplicate key (two "h", two "hs", two "class" or "bound" in an event's f) is silently last-wins: the judge reads one
  value, a grep or a person another. Fix: a duplicate-key refusing pass in read_log. Follow-up.
- F11 LOW marks.rs:587-621. A ws_bound check's op and stat are not pinned (x: p50 with <=; y: count, where op >, op
  <=; z: max with <=): a reviewed marks.json is trusted, but a ">=" op would turn f into a tightening and a "max" on x
  would break the IN TOLERANCE recheck's assumptions. Fix: compile_check refuses any other shape. Follow-up.
- F12 LOW marks.json -18 parts "L2 B1 wt"/"L2 B1 ws" select on header params.transport alone. A WebTransport run
  labelled ws (with a practice run labelled ws) is judged at the WebSocket numbers. Fix: a setup check on the ws part
  that the WsServer-only series exist (event.burst_return.budget_bytes count >= 480). Follow-up (advisor).

## Checked and found sound
- hs: writer (log/src/lib.rs:368-369) and reader (:492) refuse anything but 16 lowercase hex digits or null; numbers
  and upper case refused (tests :706-729). Ticks strictly increasing per log (reader :493-499); across logs a second
  non-null (slot, tick) is a gap (figures.rs:239-246); uneven list lengths per name and across names are gaps
  (:229-236, :284-290); server 0 is a gap, client 0 always mismatches (:291-298). Empty lists give 0 compared -> empty.
- Empty (reading (d)): fails() counts it as a failure everywhere a Fail counted (eval.rs judge_run, eval_part,
  fail_lines); setup and pre treat it as INVALID; a bite run with only empty failures is INVALID, never shown
  (eval.rs:1459-1472). Agree with reading (d): it can only lower a verdict.
- ws-practice f: IN TOLERANCE and BEYOND need s <= tolerance; the tolerance is parsed in [1, 1.10] (marks.rs) and the
  bound's must equal it (practice.rs:333); X, Y, Z, k are pinned to marks.json (:339-371); M_p recomputed from the
  practice run's heals (:372-382); moved numbers recomputed (:414-428); C4 and the always rules hold on the practice
  run (:307-314). Only F1 breaks the cap; F2, F3, F9 loosen within it.
- parse_bound: exact tag, deny_unknown_fields, serde derive refuses duplicate fields, from_str refuses trailing data,
  i64/u64 types refuse floats, NaN/inf and overflow; Ratio::parse refuses exponents, '+', spaces. Only F6 panics.
- Drift test: marks.json's top level and every mark's draft row unchanged (base vs head per field); only -17, -18,
  -20, -21, -22, -58's parts, checks, bites, report, notes changed; -17/-20/-21 tightened (scene, clients, pre).
- Redact: check.mjs and redact-check untouched; new fixtures use 203.0.113.0/24 (allow-listed documentation range);
  machine_named needs the VM address, consistent with "redact all but the run's VMs".

## Coverage table (input -> where the judge validates -> gap)
| Input (log or record) | Validated at | Gap |
|---|---|---|
| hs entries | log/src/lib.rs check_hs (writer, reader) | none |
| hs source (which log gives which name) | nowhere | F4 |
| client arch for the Arm rule | figures.rs:219 (log's own claim) | F4 |
| decoded coverage (nulls) | nowhere | F8 |
| run's build vs bite (C4) | eval.rs:549-576 | F7 (bite direction) |
| twin identity | figures.rs twin_of (profile, seed, bite, always rules) | F5 |
| bound line numbers X, Y, Z, k, tolerance | practice.rs:333-371 vs marks.json | none |
| bound line M_m, band, guard, commit | parse_bound shape only | F1, F2, F6 |
| practice class | practice.rs:384-405 | F1 |
| chance / second ruling | non-empty text only | F9 |
| one practice run per build | within the set, listed runs only | F3 |
| transport label | none (select only) | F12 |
| duplicate JSON keys | none (last wins) | F10 |

## Verdict
REVISE: F1 (High) and the before-merge Mediums F2, F3, F4, F5, F6, F9. F7, F10, F11, F12 follow-ups; F8 to the advisor.
DONE.
````

### Round 1's findings: test-engineer
_`.checkpoints/netcode-proof-test/u4j-review-test.md` lines 1-169 (sha256 d3f280b8311d), as they are:_

````text
# U4j review, round 1: test-engineer (is it proven?)

Reviewer: test-engineer (opus). Worktree /home/user/Shamble-Frontlines/.claude/worktrees/agent-a3b8dfeb01ed52b50,
base 411eeeed50d8c71e7e1f74366f0e1d5c27388444. Started 2026-11-11. No local memory; no earlier notes file.
(The Edit tool is not available in this session: this file is rewritten whole with Write after each step.)

## Plan
1. Record sha256 of every changed and new file (git status --untracked-files=all; git diff --stat) before anything.
2. Read the plan §5 U4j (u4-build-plan.md:643-663 now), §4.6, §4.12, the advisor's conditions 2, 4, nits c, f, Q20,
   Q22, W1-N2; the two reviewers' notes (netcode, security).
3. Read the diff: log/src/lib.rs, judge marks.rs, eval.rs, figures.rs, practice.rs, tests/engine.rs, tests/fixtures.rs,
   fixtures, marks.json, README, SERIES.
4. Map each §5 U4j test/fixture to an existing test; judge power; claim · proof · gap table.
5. Probes and plants (in place, one at a time, restore, sha256 check): settle the other reviewers' findings by run;
   plant where a test's power is in doubt. Probes live in a TEMPORARY test file (tests/zz_review_probe*.rs),
   removed after (the Edit tool is missing, so no in-place edit of engine.rs).
6. Check builder's claims: P1-P4, counts (+18), A19 pub items.
7. check.mjs once at the end (idle wait, touch loop, provenance check, counts 304/327/48).
8. Final sha256 identity check. Verdict.

## Findings (a line after each)
- Step 1 done: 34 paths hashed (u4j-review-test/hashes-start.sha256); marks.json 97c5cab5, README 71e4c637, SERIES
  e95febab, eval.rs 4549e885, figures.rs c034fc8f, practice.rs 1e59bdea: all equal the builder's. Note: the plan file
  u4-build-plan.md is now f54125ec (builder read 0d69332c); U4j's section moved to :643-663 (W1 plan edits, 10:28).
- Step 2-3 read: plan §5 U4j, §2 C4/-18/hs rows, §4.6, §4.12; advisor-ws-docs-r2 :373-400 (R3-R10: R6 "f = max(1, s),
  used only when s is at most the tolerance"; R7 "Valid only with 480 heal records"; R8 (v) "FAR BETTER: at the
  bound"; R9 "a counted WebSocket run that fails stands as FAIL"); netcode and security notes; the whole diff.
- Test map (§5 U4j): six decoded-hash cases = fixtures pm-0001-17/{base, differ, lost-group, refused-group, orphan,
  none-compared} (tests/fixtures.rs pm_0001_17_x2): fixture bytes checked, each differs from base by exactly the one
  line it claims (sentinel 0000000000000000, null+snap_incomplete 1, tick 65539, all null). C4 four cases =
  fixtures pm-0001-4/{3} + pm-0001-21/twin-bites-build, and engine c4_the_build_matches_the_bite. --check-marks and the
  drift test = check.mjs marks step (to run). Per figure: decoded (PASS-part/FAIL/gap x5/INVALID/NOT-RUN arm),
  twin (PASS/FAIL/empty/NOT-RUN/gap x7), machine_named (PASS/FAIL x8 + C pass), cross_arch (engine only: report 1).
  -18 ws-practice: PASS, FAR BETTER f=1 FAIL, second practice INVALID/PASS, 20 INVALID cases, BEYOND+chance PASS.
- R1 (reading) the ws-practice tests never put s at the boundaries the ruling lists (s = 1.00, 1.10, 1.1001), never
  give IN TOLERANCE with s < band_low, and never exceed Z' alone: plant T below decides whether that code is proven.
- R2 (reading) the PM-0001-17 fixtures write 2-slot hs lists under header clients 48: README §1 says "one entry per
  client slot of the run"; the judge never ties the list length to `clients`, so a writer that logs 2 of 48 slots
  passes (the base fixture itself is the proof). Joins security F8 (no coverage floor).
- Probe run 1 (u4j-review-test/probe-run1.txt; 11 probes, all pass = today's behaviour confirmed; provenance: Compiling
  npt-log and npt-judge from this copy's path): A (FAR BETTER band 2.0, s 1.9: PASS at X' 1.273 s, f 1.9 > 1.10),
  B (29-decimal band: judge panics, ratio.rs:27), C1 (client log carries its own replay_hash: PASS), C2 (aarch64
  replay: PASS), C3 (arm_rule false still gets the Arm rule: PASS), D (two forged measured runs naming each other:
  PASS), E (FAIL -> INVALID when a later practice run joins the set), F (second practice run in another set unseen:
  PASS), G (479 heals: PASS), H (record's M_m 620 ms turns FAIL into PASS), T1a/T1b/T2/T3 (the code is right at s =
  1.10, s = 1.1001, below band_low, over Z' only).
- F1 HIGH practice.rs:389, :406 (+ parse_bound :87-90; README §4 "f = max(1, s)" for every class): FAR BETTER never
  checks s <= tolerance and parse_bound never checks band_low <= 1 <= band_high, so f = s up to any value (probe A: f
  1.9, X' 1.273 s, a 1.2 s median PASSES). Beyond Mustafa's WS-BAR cap (NETEM-BAR (a)) and R6/R8 (v) ("FAR BETTER: at
  the bound"). Fix: FAR BETTER f = 1; band_low <= 1 <= band_high; refuse f > tolerance. Proof: probe A committed,
  expected FAIL at 670 ms. Before merge. (= netcode F1, security F1.)
- F2 MEDIUM practice.rs:259-286, :325: the latest practice run of the build over ALL candidates is taken, then
  refused if it started after the counted run, so a practice run added later turns a FAIL into INVALID (probe E1,
  E2). R9: a failed counted run stands as FAIL. Fix: candidates only among practice runs started before the run.
  Proof: probe E committed. Before merge (wrong verdict). (= netcode F2.)
- F3 MEDIUM practice.rs:260-263: candidates come from the counted run's set only, so "one practice run per build"
  (R7) misses a second practice run in another set (probe F: PASS, no second_ruling asked). Fix: pass every run to
  find() and count the build's practice runs over all sets. Proof: probe F committed, expected INVALID. Before merge
  (wrong verdict). (= security F3.)
- F4 MEDIUM figures.rs:195-209: twin_hash_mismatch_count accepts a twin of any role (probe D: two forged measured runs
  naming each other PASS -21's "0 effect beyond the valid part"). Fix: role twin (the -30 form at U6). Proof: probe D
  committed, expected INVALID/gap. Before merge. (= netcode F4, security F5.)
- F5 MEDIUM figures.rs:215-219, :271-282: each hs name is read from every log, and the Arm rule from the client log's
  own arch: a client log carrying replay_hash excuses its own mismatch (C1), an aarch64 replay excuses (C2), a mark
  with arm_rule false gets the rule (C3). Fix: server_hash from process room only, replay_hash from process replay on
  x86_64 only (else gap), decoded_hash from client processes only; the Arm rule on L2 (or the mark's arm_rule). Proof:
  probes C1-C3 committed, expected FAIL or gap. Before merge for C1/C2 (the input rule U4r writes to); C3 Low today
  (-58 is C-only). (= security F4, netcode F6/F8.)
- F6 MEDIUM practice.rs:331-338, :383: M_m and the band come from the record's own copy of the bound line; only X, Y,
  Z, k and the tolerance are pinned to marks.json (probe H: a record saying M_m 620 ms turns a FAIL into a PASS, f
  1.0968 for a true 1.0398). Fix: marks.json -18 pins the bound line (or M_m, band and its sha256) and the rule
  requires the record's `bound` to equal it. Proof: probe H committed. Before merge (a marks.json rule and the record
  format later units build on; a wrong verdict). (= security F2, netcode F9.)
- F7 MEDIUM practice.rs:372-376: the practice run's heal count is never checked against the bound's
  client_bursts_count (parsed, unused) though R7 says "Valid only with 480 heal records" (probe G: 479 PASS). Fix:
  heal.len() == client_bursts_count. Proof: probe G committed, expected INVALID. Before merge (cheap; wrong verdict).
  (= netcode F7.)
- F8 MEDIUM practice.rs:56-68, :87-90 with ratio.rs:25-28, :135-141: two long decimals in the bound line panic the
  judge (probe B) instead of INVALID. Fix: README §2.5 and parse_bound cap each fraction at 6 decimals (or the
  comparisons return Err). Proof: probe B as a unit test expecting Err. Before merge (nit f fixes this format now).
  (= security F6.)
- (resume 2026-11-11, after a usage-limit cut: start hashes re-checked, 34/34 match; no probe file left in the copy.
  Process note: the F9 line was first appended with a shell heredoc by mistake; from here this file is rewritten whole
  with Write, F9 kept word for word.)
- F9 MEDIUM tests/engine.rs (the -18 ws-practice tests): plant T (u4j-review-test/plant-t.txt; practice.rs :387-388
  cap 1.10 -> 1.12 and band_low dropped, :436 Z' recheck dropped) SURVIVES all 65 committed judge tests; the probes
  T1b (s = 1.1001), T2 (s below band_low) and T3 (over Z' only) each bite it. Mustafa's 10 % cap (WS-BAR) is not
  pinned by any committed test. Fix: commit T1a/T1b/T2/T3. Before merge (the bar's own number; a loosened cap passes).
- Step 6 (builder's claims): counts +18 = log 2 (hs_holds_one_hash_or_null_per_slot, hs_refuses_a_bad_entry_...) +
  judge lib 3 (ws_bound_checks_keep_their_shape, the_bound_line_reads_as_readme_writes_it, moved_numbers_round_up_...)
  + engine 8 (c4, decoded gaps, arm rule, empty, twin hash, machine_named, ws puts, ws refuses) + fixtures 5 (pm_0001_17
  x2, pm_0001_4_c4, pm_0001_21, pm_0006_11): matches its notes :113. Public items added (diff grep): npt-log
  TickRecord.hs, set_hashes, hashes; npt-judge marks::BoundNumber, Check.ws_bound, Rule::WsPractice (variant),
  eval::Outcome::Empty (variant) + Outcome::fails, figures::Lack::Empty (variant); practice.rs is `mod practice`
  (private, pub(crate) items). The builder's notes file lists none by crate (A19 asks the report to): engine-architect's
  check, from this list. P1-P4: the named tests exist and each asserts the behaviour the plant breaks (read); not re-run.
- Probe run 2 (u4j-review-test/probe-run2.txt; 2 probes pass = today's behaviour; provenance from this copy's path;
  probe file removed, 34/34 hashes match): S (one client hash of six, null elsewhere, snap_incomplete 0: run valid,
  value 0, compared 1) and O (aarch64 orphan at a tick the server never logged, replay without that tick: valid,
  value 0, "aarch64-only" 1, on an L1 part).
- F10 MEDIUM figures.rs:298-333 with marks.json -17 (compared and incomplete only reported): no coverage floor and nulls
  never reconciled with tick.s.snap_incomplete: probe S passes "100 % of decoded snapshots rebuild" on 1 of 6 hashes
  while the client's own count says 0 incomplete. A runner that samples (netcode F5 (i)) or a client that writes null
  for a refused group passes. Fix (the advisor's call, beyond the plan's words): a check per run that compared +
  incomplete = the server's (slot, tick) pairs, or at least a null on a tick whose snap_incomplete entry is 0 is a gap.
  Proof: probe S committed as a fixture (pm-0001-17/sparse) expecting INVALID or FAIL. Before merge? Advisor's ruling
  (a marks.json rule later units build on). (= security F8, netcode F5 (i), R2.)
- F11 MEDIUM figures.rs:327: `at(rep, t, slot) == theirs` treats a replay with no entry at an orphan tick as agreeing
  with a server that has none, so an aarch64 orphan is excused (probe O) where the x86-64 one FAILs (fixture orphan).
  The Arm rule excuses a hash the x86-64 re-decode reproduces as the server's; an orphan has no server hash to
  reproduce. Fix: an orphan always counts (or needs a replay entry equal to Some(server)); fold into F5. Proof: probe O
  committed, expected FAIL. Before merge with F5. (= security F4 (iii).)
- F10 addendum: README:50-52 itself defines `null` as "a tick with a group lost and none refused (counted in
  tick.s.snap_incomplete)", so the reconciliation check enforces the README's own writer rule, not a new one.
- F9 addendum: advisor-ws-docs-r2.md:378 (the ruling on the outcome record) asks for "A unit test with a fixture for
  each class and the boundaries s = 1.00, 1.10 and 1.1001": the classes are there (built in code), the boundaries are
  not (engine.rs's nearest is s = 1.131498, :2017, which is why a 1.12 cap survives).
- F12 MEDIUM fixtures/ (none for the Arm rule): §5 U4j builds cross_arch_mismatch_count "with fixtures only since no U4
  run feeds them"; the builder's own plan (u4j-build.md:46) says "pm-0001-17 (six decoded-hash cases + two Arm-rule
  cases)", but fixtures/pm-0001-17 holds the six only; the Arm rule is proven by engine.rs:1401-1448 alone, never
  through the real marks.json's L2 parts (select machine.role LG) and the folder reader. Fix: two fixtures (LG aarch64
  client + RS room + x86-64 replay agreeing: part PASS, "aarch64-only" 1; replay wrong: FAIL), plus C1/C2/O as
  fixtures with F5's fix. Before merge with F5 (the same files prove it).
- F13 LOW engine.rs:1661-1670 (machine_named): the FAIL list names 8 of the 14 conditions; role, name, plan, arch,
  mem_mb and threads_per_core are never emptied (by reading: explicit list; no plant run), and PM-0001-1's five parts
  (RS, LG, ML, RD, MAC selects, which §5 U4j names) have no fixture (only PM-0006-11's RS part). Fix: six more list
  entries; one -1 fixture with an ML/MAC log (no address needed). Follow-up.
- F14 LOW figures.rs:197 twin_hash_mismatch_count's "profile" and "bite" field checks: no test anywhere gives a twin
  another profile or bite (grep "another profile|another bite": none); dropping either survives. Fix: two cases in
  u4j_the_twin_hash_... . Follow-up (with F4's role check).
- F15 LOW practice.rs:269-279: a bite run whose set holds practice runs of two builds ("cannot tell whose numbers are
  in force") is never tested. Fix: one case in u4j_ws_practice_refuses_... . Follow-up.
- F16 LOW practice.rs:316 (scene/profile only): the practice run is never compared with the counted run on RS's
  machine.kernel, params.config_hz or params.ws_tcp_model, the "header checks" the ruling (advisor-ws-docs-r2.md:378)
  lists in the record; README §2.5 leaves them "for people". A practice run on another RS kernel sets f for this one
  (within the cap). Fix: compare the three (gap if absent). Advisor's call; follow-up.
- Step 7 (u4j-review-test/check-final.txt): check.mjs 11:17:31-11:19:57 UTC, exit 0, all 10 steps pass; idle before
  and after; touch + 2 s loop; Compiling npt-log and npt-judge from this copy's path; measured 304 = 286 + 18, bites
  327 = 309 + 18, self 48 of 48; marks 81 read and checked, ADR rows unchanged (drift test); redact 10/10, 182 items, 0
  findings; deps: npt-judge ships npt-log and serde only.
- Step 8: after the run, 34 paths (same set), all 34 sha256 equal the start (marks.json 97c5cab5, README 71e4c637,
  SERIES e95febab); Cargo.toml, Cargo.lock, check.mjs: no diff against the base. The copy is byte-identical.
- Others' findings not settleable by run, weighed: netcode F3 (whose slot index) and F5 (ii)-(iv) (writer rules:
  room logs server_hash in every phase; "two different hashes"; never drop a late hash) are README text U4r writes
  to: agree, before merge. Security F9 / netcode F14 (describe() prints neither class, rulings, band nor tolerance):
  agree; Medium, follow-up unless the advisor reads R8 (v) as the judge's report. Security F7, F10, F11, F12: agree,
  follow-ups.

## Claim · proof · gap
| Claim | Proof | Gap |
|---|---|---|
| hs: 16 lowercase hex or null; anything else refused | log tests hs_holds_one_hash_or_null_per_slot, hs_refuses_a_bad_entry_or_name_... | none |
| decoded hash, §4.6's six cases | fixtures pm-0001-17 x6 (2 tests); P1 bites | no coverage floor (F10); source per name (F5) |
| decoded gaps (name missing, slot counts, two logs, server 0) | engine u4j_the_decoded_hash_gaps_and_its_every_phase | within-name uneven lengths untested (Low, in F13's spirit) |
| every phase read | engine warm-up FAIL | none |
| Arm rule | engine arm-rule test (PASS, FAIL, NOT-RUN, x86 FAIL) | no fixture (F12); log's own arch, replay from any log, orphan excused (F5, F11) |
| Empty fails, shows no bite | engine empty test; P4 bites | none |
| C4, four cases | engine c4 + fixtures pm-0001-4 x3, pm-0001-21/twin-bites-build; P2 bites | bite direction "at least one log" (security F7, Low) |
| twin hash | engine twin test + fixtures pm-0001-21 x5 | twin role (F4); profile/bite untested (F14) |
| machine_named | engine 8 FAIL + C PASS; fixtures pm-0006-11 x2 | 6 of 14 conditions, -1's parts (F13) |
| ws-practice numbers in force | engine puts test; P3 bites | FAR BETTER f uncapped (F1); cap and band unpinned (F9) |
| ws-practice refuses what its rules do not set | engine 20 INVALID cases | later practice (F2), other set (F3), record's M_m (F6), heal count (F7), panic (F8), two-build bite (F15), header checks (F16) |
| marks.json compiles; drift | check.mjs marks: 81, rows unchanged | none |
| +18 tests, both builds | check.mjs 304 / 327 / 48 | none |

## Verdict
REVISE (round 1). Before merge: F1 (High); F2, F3, F4, F5 (with F11, F12), F6, F7, F8, F9; F10 needs the advisor's
ruling on the form (it is the README's own rule). Follow-ups: F13-F16, security F7/F10/F11/F12, netcode F6 (if not
folded into F5), F8-F14. DONE.
````

### Mustafa's WS-BAR and its refinement, 2026-11-07 (plan.md §0.5, his words)
_`.checkpoints/netcode-proof-test/plan.md` lines 200-213 (sha256 4e74a1523570), as they are:_

````text
### 0.5 Mustafa's WebSocket decisions (2026-11-07, his words verbatim) and where each is met
About 13:45 UTC:
> WS-SCOPE: no. Measure fix 1 (the newest-snapshot-only buffer) in the proof test. The reconnect becomes a requirement
> for the real client, built and measured before the first online playtest, with today's model as its target (buffer +
> reconnect: 99 % within about 1.05 s). Record that in the docs PR so it can't get lost. RESUME-CRED: deferred with the
> reconnect. WS-BAR: the three-part form now; its numbers after one practice run on the real test setup, but never
> looser than the model predicts for a correct build with fix 1. If the practice run comes out worse than the model,
> find out why before setting the numbers.

About 14:30 UTC:
> Refinement to my WS-BAR rule: the model's bound has almost no margin (5 % slower already fails a correct build), so
> declare a tolerance now, before any practice run. If the practice run is up to 10 % slower than the model, the
> numbers may move by that much; beyond 10 %, investigate first and never loosen. The advisor may set a different
> tolerance with reasons, but it must be fixed before the run.
````

### Mustafa's NETEM-BAR (a), 2026-11-10 (his words)
_`.checkpoints/netcode-proof-test/history.md` lines 364-366 (sha256 666980a651a0), as they are:_

````text
- **Mustafa, 2026-11-10 about 20:40 UTC (verbatim):** "NETEM-BAR (may the WebSocket pass bar loosen when real Linux
  disagrees with the model): (a). The bar stays capped as WS-BAR says. If a WebSocket row fails because the kernel is
  slower than the model, report it to me as the kernel's, with the netem numbers, and I decide." And: "The night
````

### §4.6 of u4-build-plan.md (cited by §5 U4j)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 305-327 (sha256 7375cdcfdb7c), as they are:_

````text
### 4.6 The client-held hash (Q11)
- **What both sides hash** with one function, `client_hash`: the full 32-bit tick; the client's entity set for that
  tick in entity-id order, each entity's id, kind and grid integers; the own-state section; the event section's events
  as (32-bit event number, kind). The authority-only streams stay out (plan.md:531). The output is never 0.
- **The server side** hashes what the encoder intends the client to hold after decoding that tick: baseline B plus this
  tick's delta over the client's set (an entity skipped by the priority accumulator at B's value), the own-state
  section and the events as sent, with the server's own 32-bit tick and event numbers.
- **The client side** hashes its own held state for that tick: the frame it built, taken as the floats it holds for
  interpolation and prediction and turned back into grid integers through `grid`, with the 32-bit tick and event
  numbers as it rebuilt them. A correct client gives the same integers (an exhaustive round-trip test, U4s s2); a
  client that places an entity a cell off, rebuilds the wrong tick or numbers an event wrongly does not.
- **When the client logs:**
  - a tick whose every group decoded: its hash, under the tick it rebuilt;
  - a tick with a received group the client could not decode (no baseline, a bad header, a size or decode error): the
    sentinel 0, which always mismatches. In an honest run this never happens (the client holds every baseline the
    server can pick, and L1 neither corrupts nor duplicates: l1/src/lib.rs:42-43);
  - a tick with a group lost and none refused: nothing, and `tick.s.snap_incomplete` = 1 for that client.
- **The figure** (`decoded_hash_mismatch_count`, U4j) counts (client, tick) pairs whose hashes differ, plus every client
  hash at a (client, tick) the server never logged (a client that labels a frame with the wrong tick); server-only
  pairs are loss, not mismatches; 0 compared fails. `decoded_hash_compared_count` and the incomplete count are reported
  beside it.
- **-57's pairing (T20):** npt-lab pairs (client, tick, entity) only for fields that tick's snapshot carried and the
  client decoded, against the server's pre-grid value at that tick.
````

### §4.12 of u4-build-plan.md (cited by §5 U4j)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 408-424 (sha256 7375cdcfdb7c), as they are:_

````text
### 4.12 Logs, the contract test and the stress-scene precondition
- On L1 one npt-lab process writes one log per process value the checks select on (`room`, `client`, `lab`), each with
  its own header (marks/README.md:9; marks F28).
- **The contract test** (a short scenario per suite) shows that every check of every U4 L1 part resolves (no gap),
  every written name has its SERIES.md row, and **the validity inputs exist (T18):** one `admit` event per client (so
  the roster rule, README:171, is not vacuous), a footer per log, and `tick.v.late_count` wherever `late-zero` applies.
  It also resolves PM-0001-2's checks on every SS suite (T1).
- **Which marks gain the scene, the clients and `pre:PM-0001-2` (Q22, T1):**

| Mark | U4 L1 parts | Select gains | `pre:PM-0001-2` | Why |
|---|---|---|---|---|
| -17 | P0, P2, P3, B1 | scene SS, clients 48 | yes | the codec under the real load; shots hold (§4.13) |
| -18 | B1 | scene SS, clients 48 | yes | B1 under option A loses about 1.2 % of shots, inside the band (§4.13) |
| -20 | P3, A and B | scene SS, clients 48 | yes | redundancy repairs P3's losses |
| -21 | 8 suites | scene SS, clients 48 | yes | at most about 9 shots per second lost to refusals (§4.13) |
| -22 | never-ack | scene SS, clients 48 | **no** | the drop leaves 47 clients by design, which PM-0001-2's "a client on every soldier" refuses (the slot count of `everyone_in_view` changes too: figures.rs:26-36). README §4 (:193-197) puts the precondition on -22, so U4j writes this exception and its reason into README §4's sentence; -22's runs still log PM-0001-2's series (shots, projectiles, vehicles, teams, portals), and U4f's report states the load each run carried (the advisor's condition 4; a reading of ADR 0001:1589, for the readings PR: §10) |
| -14, -16, -19, -23 to -25, -27 | none at U4 | unchanged | unchanged | no U4 run feeds them; their scenes belong to the R run plans |
````

## The carries this packet holds

_Rule: every entry of `.checkpoints/netcode-proof-test/u4s-carries.md` whose text matches `U4j|npt-judge|marks\.json|SERIES\.md`._

### Carry 54
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 207-211 (sha256 f18a06927908), as they are:_

````text
54. **The encoder's refusals in the room (u4d-round2-security.md, 'Carries for U4e', second bullet; the advisor's U4d
    close ruling):** "The room handles `Err(OutOfRange)` from the encoder (a section mix-up, or an integer outside its
    field's range) without a panic, `unwrap` or `expect`: that client's snapshot for that tick is skipped, the error
    is counted per client and logged (under a name with its SERIES.md row), and the next tick encodes afresh. A test
    plants one out-of-range record for one client and shows every other client's snapshot for that tick unchanged."
````

## The carries this packet left out

Ids: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 55, 56, 57, 58, 59, 60.
