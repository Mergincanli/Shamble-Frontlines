# advisor: VERSION-RULE ruling (C2, 4c-2)

## Plan
1. Read movement.rs, read.rs header, the sheet I2, STATUS_ARCHIVE 4B-FIRST-DEFAULT, rust-reviewer 11.4.
2. Grep ADRs and DECISIONS for content versioning rules.
3. Read report.ts, content-ties test, content.ts, schema, frozen bundle, recordings headers.
4. Weigh (a)/(b)/(c); verdict with exact changes and test plant.

## Findings
- F1 movement.rs:12-14 VERSION=1; read() calls header(FORMAT, VERSION); keys closed (finish()); no default anywhere.
- F2 read.rs:259-284 header takes whole_in(1, newest); WrongVersion{bits,newest} (error.rs:160-163, :370 "newest: N"); callers: map.rs:171, movement.rs:61, read.rs tests.
- F3 schema.rs:211-215 writes version as `const <VERSION>`: the schema already admits the newest only, unlike the reader (1..newest). Harmless at 1; matters at 2.
- F4 content.ts:44-48,236: page checks 1 <= version <= kind.newest; content-ties.test.mjs:23-30,50-59 ties FORMAT/VERSION by regex `^pub const VERSION: u32 = (\d+);$`.
- F5 report.ts:84,188 shows `${format} ${version}` from the bundle: a bump shows "shamble-movement 2" on the page.
- F6 ADR 0005 CD-4 (docs/adr/0005:358-366, Proposed; signed as basis for spikes 2026-11-03): additive = default, no bump; breaking = bump + one migration N->N+1 on the untyped tree; newer refused, never guessed.
- F7 STATUS_ARCHIVE:2300-2302 4B-FIRST-DEFAULT: first default goes to the advisor with the change to read.rs's "nothing missing" rule.
- F8 DECISIONS.md: no rule of Mustafa's on content-format versions (grep "version": only VISION-RULE versions, tool/package versions, the Testlines Export's own format). Technical choice inside CD-4: the advisor's.
- F9 Precedent: recording.rs:150,298-300 "this build reads version {VERSION} only" (exact-version refusal).
- F10 origin/main dae6d107 (PR #79, 4b-2) already ships movement v1 with 15 fields and the frozen bundle at v1: v1's 15-field form is public history.
- F11 13 new fields are required with no default: a breaking change under CD-4, not additive.
- F12 ADR 0005 :740-742 CD-4's pass mark binds the files that exist (Cabana, DM_Tri, his saved maps) and "a file with a future version is refused with a clear message". ADR 0005 :284: format and version read, then migrated, THEN the typed load. So the typed reader should take the newest version only; migrations bring older trees up first. read.rs's "1 to newest" is a latent fault: at the first bump it would read a v1 tree as v2 with no migration behind it.
- F13 bundle.rs:167 `.whole("version", 1, VERSION)` has the same range; content.ts:111 already checks the bundle exactly; content.ts:236 uses the range for kinds.
- F14 step-11 ruling (git show 59f6cc7f:.checkpoints/cs4b-content/advisor-step11.md, B2): "the first additive field (CD-4) brings the column AND the change to read.rs's nothing-missing rule". These 13 are required, not additive: 4B-FIRST-DEFAULT is not triggered; it stays open.
- F15 Departures from a Proposed ADR draft went to him in the report's list (DECISIONS.md :909-932, DEPART-4 (a) "all fourteen stand"). The missing 1->2 migration step is such a departure: listed in 4c-2's report, not a blocker for C2.
- F16 Literal sites: testlines.mjs:42 HEADER version 1 (and its --check compares soldier.json's header to it, :194); testlines.test.mjs:26,41,54 (:15 is the Export's own format_version, untouched); content.test.ts:125,182; report.test.ts:46,57,134,148 (:138 the mixed case already uses 2); skeleton.spec.ts:111-112; read.rs tests :411-431; TEST_TEXT; soldier.json; the frozen bundle. packages/sim-worker/pkg is untracked (git ls-files empty).
- F17 4c-1 (STATUS:34,47): his Brave check on Sunday 29 Nov on the scene's final head, which shows "shamble-movement 1"; 4c-2 is built on top.

## Verdict: (b), APPROVE, in its smallest form: VERSION 2, the typed read takes the newest version only (for every kind)
Why not (a): 13 required fields with no default is a breaking change (CD-4, adr 0005:358-360); one version number would name two formats, and v1 is on main (F10).
Why not (c): defaults put 13 numbers in Rust code, a second home beside the content rows NO-SOURCE (b) asks for (sheet :731-740; movement.rs:3-5); it would trigger 4B-FIRST-DEFAULT and the "nothing missing" change for files that do not exist; a reader of v1 serves no VISION_V2 §6 need today ("for later"); and the schema's `const` (F3) would disagree with the reader.
Not his to decide: no rule of his on content versions (F8); a technical choice inside CD-4. The missing migration step is a departure he hears about in the report (F15).

## Conditions for C2
1. movement.rs: VERSION = 2; its comment says "the one version the typed read takes". TEST_TEXT at "version": 2 with all 28 fields.
2. read.rs `header`: whole_in(node, &path, newest, newest) (exact), and the doc comment at :258 says older versions are migrated before the typed read (CD-4), so the typed read takes the newest only. Map stays 1 and reads as today. No new `oldest` const, no new field on Kind::WrongVersion (its "(newest: N)" message stays right).
3. bundle.rs:167: the same exact rule (VERSION, VERSION): zero change in behaviour at 1, one rule for every reader.
4. content.ts:236: `version !== kind.newest` in place of the range (the kinds' `newest` key stays; content-ties is unchanged and ties MOVEMENT_VERSION = 2 to movement.rs).
5. testlines.mjs:42 HEADER version 2, and a test that ties HEADER.version to movement.rs's VERSION (the same regex content-ties uses, or content-ties importing HEADER).
6. Re-made in C2: content/movement/soldier.json, the generated movement schema (const 2, zero diff after regeneration), the frozen first-loop bundle and its recordings; every literal site in F16 moved to 2, report.test.ts's mixed case (:138) changed so it still tests a mix.
7. 4c-2's Brave checklist expects "movement/soldier.json shamble-movement 2" on the readout. 4c-1's checklist keeps "1" only if 4c-1's pull request is opened from the scene's head before C2; if it is opened from a later head, its step 4 says 2.
8. 4c-2's report lists the departure from CD-4 (a breaking bump with no 1->2 migration step: no version-1 movement file exists outside git history, and a migration would have to invent the 13 values) for his call, beside the others.
9. 4B-FIRST-DEFAULT stays open (these fields are required, not additive).

## Test and plant
- read.rs: header("shamble-test", 2): version 2 reads; versions 1, 3, 0, 1.5, -1 give WrongVersion { newest: 2 }. Plant: put back the lower bound 1 -> the version-1 case fails.
- movement.rs: TEST_TEXT reads; TEST_TEXT with "version": 1 gives WrongVersion { newest: 2 } (the 28-field text is otherwise valid, so only the version rule can refuse it). Plant: the same lower bound 1.
- content.test.ts: a bundle whose movement file says version 1 is a fault. Plant: put back `version < 1 || version > kind.newest`.
- The converter's tie: plant HEADER.version = 1 -> the tie test fails (and testlines --check on soldier.json).

## Not checked
No build, test or plant run; the generated TS types not opened (Movement has no version field in movement.rs:39-56); recording headers beyond content_hash not read; 4c-1's final head not identified.
DONE
