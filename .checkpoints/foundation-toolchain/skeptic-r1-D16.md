# Skeptic r1, D16 (= EA-5, Low): a static rule that client never imports net or the Wasm glue (for 2.3)

Plan: (1) read EA-5 in full from review-r1-findings.json; (2) re-read every ADR 0002 line it cites at head 4d78ad6
(:207-209, :218-219, :424, :1286 PM-0002-22, :1308 PM-0002-42) and part 5's two run-time rules plus spike item 1;
(3) check whether any other place (part 3 elsewhere, part 5, part 7, the migration plan's 2.3/G3, OQs, the other ADRs,
the plan) already gives the page-side static rule, which would make the finding not hold; (4) check the diff at 4d78ad6
(packages, tsconfig, any lint config) for whether anything in 2.2 enforces or contradicts it; (5) build a scratch copy
in .gate/skeptic-D16 and show what a client -> net import does today (passes or fails); (6) judge the proposed fix
(right? complete? what would it break: e.g. client may legitimately need protocol types, sim-worker message types, or
the Wasm glue for a fallback; does "the Wasm glue" name a real package?); (7) verdict and severity.

## Checks

1. EA-5's full text read (review-r1-findings.json, return 3, EA-5): Low; "Proposed, for 2.3's plan; this is not a defect
   in the diff"; fix: 2.3's plan weighs one more no-restricted-imports entry (client imports neither net nor the Wasm
   glue) plus a PM-0002-42 planted defect; the ADR owner words it into part 3 once the Advisor agrees; no change in PR #48.
2. ADR 0002 at 4d78ad6 (saved to .gate/skeptic-D16/adr0002-at-head.md, 1691 lines): unchanged by the PR
   (`git diff --stat origin/main 4d78ad6 -- docs/adr/` touches only docs/adr/README.md) and unchanged 4d78ad6..HEAD.
   - :207-209 as quoted: the TS rules are "client never reaches editor" (ESLint no-restricted-imports + Vite manifest
     walk), "sim-worker and net never import rendering or DOM packages", "nothing imports legacy/ or spikes/". No rule on
     client -> net, client -> sim-wasm glue, or client -> sim-worker. HOLDS.
   - :218-219 as quoted ("a mechanical check is the only kind that holds the boundaries R1 and R2 ask for (the legacy
     fence leaked, row CS11)"). HOLDS.
   - :424 as quoted: part 7's no-restricted-imports is "for part 3's layering rules" only. HOLDS.
   - Part 5 :280-281 "never instantiates Wasm and makes zero calls into it"; :369-373 "The Worker owns the network
     connection"; part 6 :391 "The page never handles a network packet". These are the page-side rules. HOLDS.
   - :1308 PM-0002-42 (G3 bites) planted defects: "an import of editor from client", "rendering or DOM code imported by
     sim-worker or net", "an import from legacy/ or spikes/"; no client -> net and no client -> glue defect. HOLDS.
   - :1286 PM-0002-22 (counter, page stubs for WebSocket/WebTransport; "Bites: the page opening the WebSocket fallback
     itself") and :1265 PM-0002-2 (0 Wasm calls/instantiations by the page) sit under the table's section
     "The Rust <-> TypeScript boundary spike" (:1263), i.e. the throwaway spike (:1127 "throwaway, in spikes/").
     Spike item 1 (:1136 "the page makes zero calls into Wasm") and item 6 (:1167 "the page handles zero network
     messages (a counter)"). HOLDS: both are run-time counts.
   - Migration plan: 2.5's done-when (:1471 "the page makes zero Wasm calls") and slice 1's (:1484 "the boundary marks of
     the spike's items 1-3 hold on the real slice at N = 12") are one-time milestones, not lasting gate checks.
3. NOT CITED BY EA-5, found here: PM-0002-64 (ADR 0002 :1330, phase 2.5) keeps "PM-0002-2's and PM-0002-22's stubs" as a
   lasting check: "the page makes 0 Wasm calls and handles 0 network messages", in G12 "with the test-hook API's
   counters", "every gate run", in Chromium, Firefox and WebKit. So the page side is held by a MECHANICAL (run-time)
   check on every gate run from 2.5 on, not only by the throwaway spike. Limits of that check: it runs "a 1-minute local
   match" only, where nothing on the page has a reason to open a socket (the network counter is near-trivially 0); G12's
   online runs over both backends (part 9 :581; PM-0002-51/-73, from slice 3) are not said to carry the stubs; and any
   run-time count sees only the paths the run executes. EA-5's "checked only at run time" stays TRUE; its evidence
   understates the run-time coverage (spike item 1 and PM-0002-22 are the spike's; the lasting row is PM-0002-64).
4. Part 3's Why (:218) says "mechanical", not "static": PM-0002-64 is mechanical too. The case for a static rule is
   therefore narrower than EA-5 words it: an import rule catches the dependency on every path at lint time, including
   the online-only paths a local-match G12 run never executes. The finding's direction stands on that narrower ground.
5. No other doc gives the static page-side rule: `git grep` at 4d78ad6 for no-restricted-(imports|globals) outside
   legacy/.checkpoints: only ADR 0002 :208, :424, :974 and r5 :234, :260, :1181, :1268-1269; r5's REC list (:1268-1269)
   is "client -/-> editor; sim-worker and net -/-> rendering and DOM; nothing -/-> legacy/". R1-BND-REC1 (r1 :2252-2264)
   and R1-BND-REC6 (r1 :2326-2338) propose run-time spike marks only ("zero calls into Wasm"; "a counter"). ADR 0001
   :631 restates the boundary ("the page never handles a network packet and makes no Wasm calls; the Worker owns the
   connection") with no check of its own. ADRs 0003, 0005, 0006: no page-side import rule. HOLDS: nothing says client
   never imports net or the glue.
6. Would the rule break a legitimate page need? The lobby (ADR 0006 :270-284, ADR 0001's diagram) is HTTPS (guest id,
   room create, code lookup, join token), not net's game transport (ADR 0002 :198-199: net = "the browser's two
   backends of ADR 0001's thin transport interface, WebTransport and WebSocket"). ADR 0001 :468 says the WebTransport
   certificate hash is one "the page fetches from its own HTTPS origin"; since the Worker opens the connection (:631),
   that fetch belongs on the Worker's connect path; if it lived in net and the page called it, the rule would flag a
   real breach of :631, not a legitimate use. No conflict found.
7. The tree at 4d78ad6: packages client, sim-worker, net, types, e2e are one-line stubs (`export const PACKAGE = ...`);
   net's package.json says "Empty until slice 3"; client's says "Empty until the skeleton step". No tsconfig*.json, no
   eslint* config (`git ls-tree` at head lists only package.json, package-lock.json, .gitignore at the root for these
   patterns). tools/gate.mjs at head has no packages/ check (its eslint-scope use is legacy check 5b); toolchain.yml
   runs `npm test --workspaces` and `npx --no -- tsc --version` only. npm workspaces put a symlink for every package in
   the root node_modules/@shamble/ (seen in the worktree and in a scratch `npm ci --offline` of the head's lock in
   .gate/skeptic-D16/tree). The symlink exists; resolution of a bare `@shamble/net` import was NOT shown (the stubs'
   package.json files have no `main` or `exports`), so I claim nothing about whether such an import resolves today. A
   planted client -> net import run through the head's `npm test --workspaces` was denied by the permission system; not
   retried. Not needed for the verdict: a lint rule reads import specifiers, not resolution, and nothing static exists
   at head to catch one, which EA-5 itself says is as planned until 2.3's G3.
8. The ADR's own principle, :893-895: "The legacy fence leaked because nothing checked it (row CS11); here every rule is
   a gate check with a planted defect that proves it (G5, G8, `--prove`)." CS11 (LEGACY_INVENTORY :180) was an
   unchecked comment fence. Part 5's page rules have a lasting gate check (PM-0002-64, G12) but NO `--prove` planted
   defect: PM-0002-51 (G12 bites, :1317) lists a throw in deploy, a console error, an external request, a broken
   WebSocket backend, +20 % draw calls; no page Wasm call, no page network message (PM-0002-2's and -22's "Bites" are
   spike-run bites, not mutations). So EA-5's PM-0002-42 planted defect would give the import half of the page rule
   the proof :895 asks for. This supports EA-5's direction more firmly than its own citation (:218) does.
9. The glue's location: ADR 0002 :342-343 is the only place that places it ("Generated types, from three sources, into
   one committed package packages/types: wasm-bindgen's .d.ts and glue are build output (ignored)"); client legitimately
   imports types (the layout and its hash, :343-345: "the page checks the hash at startup"). INFERENCE: that sentence
   can be read as putting the glue, ignored, inside packages/types; IF so, "client never imports the Wasm glue" must name
   the glue's files by path, not a package. Either way the glue's path is fixed no earlier than 2.4 or 2.5 (sim-wasm has
   no crate type or exports at 4d78ad6), so 2.3 can plant it only against a provisional path; the net half is settleable
   at 2.3 (net is a package; its code enters at slice 3, but a planted import of `@shamble/net` in client bites now).
10. r5 R5-LAYOUT-REC2 (:1176-1193): the editor rule is "checked twice" because "a lint rule catches it before the build
   (R5-TSLINT-F14)" and "the built manifest shows what the player page really loads (F15)". R5-TSLINT-F14 (:234-238):
   no-restricted-imports "Disallow[s] specified modules when loaded by `import`" by path pattern. INFERENCE from F14's
   wording (ESLint's rule page not read at the source by me): it reads the linted file's own import specifiers, so a
   client file importing a sim-worker module that itself imports net or the glue would not be caught (sim-worker may
   import both). engine-architect's own notes (:43) record EA-5 was kept "without the Vite mechanism".
11. The carrier. EA-5's fix says "No change in this PR". `grep` of the checkpoint, plan.md, findings-2-3.md,
   findings-2-3-c4.md, docs/STATUS.md, STATUS_ARCHIVE.md, ROADMAP.md: only engine-architect-review-r1.md names the rule.
   At 4d78ad6 the report's "Findings for 2.3" table (docs/reports/2026-11-12-foundation-2-2.md :156-180, 21 rows) and
   STATUS_ARCHIVE "From foundation 2.2" (:551 on; its one `net` hit is CARGO-NET-OFFLINE) carry no page-side import
   rule. So nothing yet carries EA-5 to 2.3's plan: the fix for this PR is one row in that table and in the archive's
   list (the session's dedupe reading, "a 2.3 finding", already implies it); "no change" holds for the ADR and the code.
12. How the page starts the Worker: ADR 0002 fixes no mechanism beyond "Vite bundles it" (:790) and "the Worker and the
   Wasm load as files" (:431). Either usual form (a `new Worker(new URL(...))` call, which an import rule does not read,
   or a worker-suffix import of a sim-worker file, a sim-worker specifier) is untouched by a rule on net and the glue
   only, so the rule as worded breaks no Worker start. Closing check 10's transitive route by banning sim-worker from
   client wholesale WOULD break the import form; 2.3 would need to allow the Worker entry only, or check the page's
   built graph (the editor rule's second, manifest-level check).

## Verdict
HOLDS, Low (unchanged). Every premise checks out at 4d78ad6 (no static page-side rule anywhere: ADR 0002 part 3,
part 7, PM-0002-42, r5's REC list, R1-BND-REC1/REC6, ADR 0001 :631; part 5's page rules are held only at run time).
EA-5 omits PM-0002-64 (:1330), the lasting G12 counter on every gate run from 2.5, so its "why" is narrower than worded:
the static rule's value is the paths a local-match G12 run never executes (online-only code) plus the `--prove` proof
:893-895 asks for, which no G12 mutation gives the page rules today (PM-0002-51). Not a defect in PR #48; a proposal
for 2.3's plan. Fix: right in direction, partial. Limits for 2.3's plan to weigh (not new findings): direct
`WebSocket`, `WebTransport` or `WebAssembly` use on the page (PM-0002-22's own bite) is not an import; a client import
that reaches net or the glue through a sim-worker module is not caught per file; PM-0002-64's counters still have no
`--prove` defect; the glue half waits for 2.4/2.5's path. Breaks nothing found (lobby HTTPS, certificate hash, types,
Worker start). Carrier: one row in the report's "Findings for 2.3" and STATUS_ARCHIVE "From foundation 2.2". Wording
part 3 edits a Proposed decision doc: its PR needs Mustafa's go, not only the Advisor's agreement.

Not verified: the planted client -> net run (denied by the permission system; not retried); whether a bare
`@shamble/net` import resolves today; no-restricted-imports' per-file behaviour at ESLint's own page (inferred from r5
F14's wording); where the glue will actually land (one reading of :342-343).
