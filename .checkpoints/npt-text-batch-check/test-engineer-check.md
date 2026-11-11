# Task: night item 6 npt-text-batch-check (test-engineer check of the applied text batch d7a54cd against advisor-u4d-close.md)

## Plan (blocks, in order)
- B0 read the log, script and ruling
- B1 u4s-carries.md lines (log 11-18)
- B2 u4-build-plan.md lines (log 19-28)
- B3 plan.md lines (log 29-36)
- B4 u4d-report.md lines (log 37-38)
- B5 advisor-u4-wave1.md line (log 39)
- B6 unexplained changes
- B7 the two readings
- B8 summary

Method: `git diff` and `git diff --word-diff=plain` of 238145d..d7a54cd per file; the ruling, the script and the five
files read at 21cc90f (`.gate/npt-spike-1112`); every long joined string also checked mechanically with the Grep tool
(multiline, every space as `\s+`): found once in the ruling and once in its target file. No build, test, script or web.
File:line below is at 21cc90f (= d7a54cd for these five files); "ruling :N" is advisor-u4d-close.md line N.

## The 29 log lines (text-batch-log.md:11-39)

| Log line | File | What it did | Result |
|---|---|---|---|
| :11 B1.2 carry 52's tail | u4s-carries.md:297-303 | span "(per 50 s window:" to the closing quote replaced by ruling :143-149, rewrapped from the entry's line 5 | OK. Label :293-294 (Mustafa's words) byte-unchanged (outside the diff). The hand rewrap ("tick 0." at :300) changes no word (word diff). Mechanical match. |
| :12 B2 carry 51's span | u4s-carries.md:189-194 | span "npt-sim is not U4e's lane, so if" .. "which adds it in its own commit." replaced by ruling :237-241 | OK. Label :184-185 (Mustafa's words) byte-unchanged; rewrap starts at :189; the tail "A test shows that slot k's client ..." kept. Mechanical match. |
| :13 B2 carries 53-57 | u4s-carries.md:195-228 | ruling :250-261, :266-270, :275-279, :287-292, :297-302 appended after 51, in order | OK. 34 lines, line-for-line copies, inside §U4e (:147-229), before "## U4r". |
| :14 B3/B1.2 carries 58-60 | u4s-carries.md:244-259 | ruling :356-360, :157-162, :376-380 appended after 49 | OK. 16 lines, order 58, 59, 60, inside §U4r (:230-260), as ruling :386-387 says. |
| :15 F3 carry 40's text | u4s-carries.md:236 | "holds by construction at U4r, and its RTT argument" -> "... at U4r for the tolerance and the cap, and its RTT argument" | OK (ruling :322-323), in the quoted text. |
| :16 F3 carry 40's label | u4s-carries.md:231-233 | label clause replaced (ruling :325-327); whole entry rewrapped | OK. Carry 40 holds no words of Mustafa's; the word diff shows only the two F3 changes in the entry. |
| :17 F7 line 14 | u4s-carries.md:14 | sentence replaced (ruling :345-347) | OK; one line, 117 characters as the ruling says. |
| :18 F7 carry 13 | u4s-carries.md:114-115 | note after the closing quote (:113), indented four spaces | OK. The ruling says "on a new line"; it wraps to two lines at 120 (line breaks aside), words exact. |
| :19 B1.5 :184 (1) | u4-build-plan.md:184 | ", `Weapon` (TUNE-1)" after "`replay::Step`" | OK (ruling :201-202), in the plain-data types list. |
| :20 B1.5 :184 (2), F11 | u4-build-plan.md:184 | accessor list extended, W1-PUB's items added | OK (ruling :203-207); still in the row's first cell. Mechanical match. |
| :21 B1.5 :184 (3) | u4-build-plan.md:184 | "; TUNE-1: ..." after "... W1-PUB rulings" in the Why cell | OK (ruling :208-210). Mechanical match. |
| :22 B1.5 :185 (1) | u4-build-plan.md:185 | "`Recording` (with `GENERATOR`, ...), `StateDump`, `replay::Replayer` and its methods" | OK (ruling :211-212). Mechanical match. |
| :23 B1.5 :185 (2), F11 | u4-build-plan.md:185 | "; W1-PUB: `synthetic` is in the library ... (§4.15)" after "A12's one driver" | OK (ruling :212-213). Mechanical match. |
| :24 F5 :192 | u4-build-plan.md:192 | "§4.7's items" -> "plan §4.7's items" | OK (ruling :333-334), the npt-proto row's Why cell. |
| :25 F6 :230 | u4-build-plan.md:230 | ", for every fire origin below the bound World's doc states (TUNE-2)." | OK (ruling :338-339). |
| :26 F6 :365 | u4-build-plan.md:365 | same clause after "idempotent" | OK (ruling :340-341). |
| :27 F10 :247 | u4-build-plan.md:247 | "never what" -> "not what" | OK (ruling :316-318); nothing else on the line moved. |
| :28 F9 | u4-build-plan.md:493-494 | two lines replaced by ruling :366-367 | OK, byte copies; :495 unchanged. |
| :29 F4 :1248 | plan.md:1248 | "advisor-u3q.md:143-144" -> ":142-143" | OK (ruling :329); the old cite no longer occurs in the line. |
| :30 NA1 :1424 (R-31) | plan.md:1424 | U3D-L0's parenthetical replaced | OK (ruling :402-410); the trailing ";" after "))" kept. Mechanical match. |
| :31 NA1 :1248 (sampling case) | plan.md:1248 | the sampling case inserted between "(U3D-MISS);" and "-3's own L2 P1 checks" | OK (ruling :411-419). Mechanical match. |
| :32 NA5 :1248 (C-U8-6) | plan.md:1248 | the bound-socket and missing-row clause after the /proc/net/snmp anchor | OK (ruling :425-429), one space after the anchor. Mechanical match. |
| :33 NA5 :1248 (C-U8-4) | plan.md:1248 | ", and its row found at both;" | OK (ruling :430-431). Mechanical match. |
| :34 NA4 :1248 (run conditions) | plan.md:1248 | the policy-at-start-edge condition before "no queue" | OK (ruling :436-441). Mechanical match. |
| :35 TE2-TE4 :1246 (C-U6-8) | plan.md:1246 | C-U6-8 appended to the U6 row | OK as a copy (ruling :448-454; mechanical match). See F-1 (Low): its `||` cuts the rendered table row. |
| :36 TUNE-1 :1246 (U6 header) | plan.md:1246 | header item extended with the weapon and health | OK (ruling :382-384). Mechanical match. |
| :37 B1.6 | u4d-report.md:65-68 | ruling :221-224 as the last bullet of "Other findings, by route" (:43-69) | OK, byte copies. |
| :38 B1.5's report line (the main session's words) | u4d-report.md:53-57 | "The advisor's answers" after the "For the advisor" bullet | Marked as the main session's words by the log; text = text_batch.py:178-182. States B1.1-B1.6 correctly, with one omission: F-2 (Low). |
| :39 B1.4 | advisor-u4-wave1.md:627-632 | blank line + ruling :188-192 at the file's end | OK, byte copies; no line cite into the file moves. |

## Findings

- **F-1, Low. plan.md:1246** (the U6 row, C-U6-8, log :35). The ruling's own text, copied exactly, puts
  `` `p999_pass || would == "MISS"` `` inside a Markdown table row. In GitHub's table syntax an unescaped pipe splits a
  cell even inside a code span, so the rendered U6 row would end at "`p999_pass" and drop the rest of C-U6-8 (TE4's
  plant, "Each plant is run once and fails by name", T7's doc, "drops out"). It is the only table row in plan.md with a
  pipe in a code span (Grep; :574 and :1160 are list lines). The raw file, which sessions read, is right. Not a copying
  error: the bytes are the ruling's (ruling :451-452); a fix (`\|\|`) changes those bytes, so it is the advisor's
  call. The GitHub rule is from my knowledge of the GFM tables spec, not checked tonight (no web). Cheapest proof: open
  plan.md at 21cc90f in GitHub's rendered view and read the U6 row's end; or read plan.md:1246 for `p999_pass ||`.
- **F-2, Low. u4d-report.md:53** (the day session's own line, log :38). "(A) the estimate scene's knobs are approved as
  stand-in tuning, with conditions (B1.1)" sits under :49's "`aim_one_in` 11 and `grenade_fuse_ms` 1,515". B1.1 approves
  `aim_one_in`'s knob but not its value: "the knob is approved; its value is not final" (ruling :35-37); U4d-F sets it
  on the judged window (ruling :105-106). The line is not wrong ("with conditions" points there), but a reader can take
  11 as approved. A short clause would close it (for example "`aim_one_in`'s value is set by U4d-F"). Cheapest proof:
  read u4d-report.md:49 and :53 beside advisor-u4d-close.md:35-37.

## Unexplained changes
None. Every hunk maps to a log line: u4s-carries.md 4 hunks (:14; :114-115; :189-259; :297-303), +75 -16 as the log
header; u4-build-plan.md 8 lines (184, 185, 192, 230, 247, 365, 493, 494), +8 -8; plan.md 3 lines (1246, 1248, 1424),
+3 -3, and the word diff of those lines shows only the 8 logged edits; u4d-report.md 2 hunks, +9 -0; advisor-u4-wave1.md
1 hunk, +6 -0. The rewraps in carries 40, 51 and 52 (and carry 52's hand rewrap) change no word, and the label lines
holding Mustafa's words (carry 51 :184-185, carry 52 :293-294) are byte-unchanged. No double or trailing spaces on any
changed line (Grep).

## The two readings
- R-31 (plan.md:1424): no pass bar's number or direction moved: the bar is still E at or below 10,000 ns per
  direction, judged with L0 = 0, as at 238145d; only the explanation changed, from "E matches -3's result exactly and
  can never pass where -3 fails" to "E matches it on average", so on one leg near the bar the two may disagree either way.
- U3D-MISS's sampling case (plan.md:1248): no pass bar's number or direction moved: ImpairSocket's bar (E at or below
  10,000 ns) and -3's own check are unchanged, and a MISS with -3 passing still takes the MISS route; the new text only
  says how to read "E passes, -3 fails" on one leg: from 9,000 to 10,000 ns both results stand as chance, and below
  9,000 ns the leg goes to the advisor before any L2 mark counts (9,000 ns is a new number, but it only sends a case to
  review; it passes nothing).

## Summary
- 29 log lines checked: 27 OK with no finding; 2 OK as copies or as marked, each with a Low finding (F-1 on log :35,
  the ruling's own text; F-2 on log :38, the day session's words).
- Findings: Critical 0, High 0, Medium 0, Low 2. No word of Mustafa's changed; no rule loosened beyond the ruling.
- Unexplained changes: none.
- Not checked: the log header's sha256 values (git only, no hash tool for sha256); that the regenerated dry-run log
  equals the applied run's output (only the applied result in d7a54cd was checked); GitHub's rendering of F-1 (no web);
  exact line lengths (the script's own 120-character check claims none over, for non-table lines); the substance of
  the ruling itself (out of scope).

## Notes
- B0 done: ruling and text_batch.py unchanged 238145d -> d7a54cd -> 21cc90f (only the log added after d7a54cd).
- B1 done: log :11-18 OK. B2 done: log :19-28 OK. B3 done: log :29-36 OK; F-1 found (plan.md:1246, Low).
- B4 done: log :37 OK, log :38 marked, F-2 found (u4d-report.md:53, Low). B5 done: log :39 OK.
- B6 done: none unexplained. B7 done: the two readings above. B8 done: summary above.

DONE
