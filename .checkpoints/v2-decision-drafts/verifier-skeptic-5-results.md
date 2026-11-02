# verifier-skeptic-5: results (plan step 7, skeptic of verifier-fixcheck-c Findings 2 / row B25)

Read at 47eadd9 (branch claude/laughing-lovelace-ox61se). Between 47eadd9 and HEAD 6203b21 only docs/reports/ changed,
so these lines read the same at HEAD. The text under test, README:269–270: "Codes local to one draft are not register
codes: ADR 0002's gate checks G1–G13, ADR 0004's spike claims S1–S22 and ADR 0006's option labels (G1–G4, S1–S3,
K1–K4, B1–B11); a register code always carries its word ("gap G5", "seam S1")." The line was added in 47eadd9. It
copies the outcome of consistency finding F12 (engine-architect-consistency-findings.md:193–194), which had the same
ranges. G0 has been in ADR 0002 since its first draft (789bedb), and S4 and S5 in ADR 0006 since its first draft
(5bdad85), so the README was wrong on the day it was written.

| part | upheld / refuted / partly | the evidence (quote and place) | the smallest correct text, if upheld |
|---|---|---|---|
| (a) G1–G13 against G0 | upheld (low: no ambiguity results) | ADR 0002's check table, "Its checks, fastest first" (0002:445–463), begins "\| G0 \| setup \| toolchain equals `rust-toolchain.toml` …" (0002:450). ADR 0002 counts G0 as a check id itself: "`--quick` (G0–G5 …)" (0002:445), M-0002-39 "(parts 2 and 9: G0)" (0002:1082), "one mutation and one control per check id (G0–G12; G13 from slice 2)" (0002:1095), "G0–G12 (G13 joins …)" (0002:1225). The defense fails: G0 is a gate check under the draft's own wording. The only mitigation is that no register code is called G0 (the gaps are G1, G2, G3, G4, G5, G7, G8, G9, G11, G15; README:310–319), so a bare G0 cannot be mistaken for one. | "ADR 0002's gate checks G0–G13" |
| (b) S1–S3 against S1–S5 | upheld for the range; the collision point is overstated | ADR 0006's Q10 table: "S1. Compressed JSON lines in Hetzner Object Storage (proposed), S2 Parquet compaction only when needed" (0006:503), "S3. Cloudflare R2 instead" (:504), "S4. A SQL database" (:505), "S5. A hosted analytics warehouse" (:506). The labels on S4 and S5 are deliberate: the draft leaves other unresearched rows without a label (0006:439 Netcup row, :455 dns-01, :458 tls-alpn-01). On the collision: ADR 0006 itself cites "seam S3" (:85, :164) and "seam S4" (:90), and every one of its seam cites carries the word (:85, :90, :91, :92, :164, :354). So README:270's own rule ("a register code always carries its word") does tell option S4 apart from seam S4. Option S4 meets seam S4 exactly as options S1 and S3 meet seams S1 and S3, and the README already accepts those two. Wrong: the list's range, which leaves out S4 and S5. Not quite right: "S4 meets seam S4 unexplained". The general rule explains it; only the list leaves it out. | "S1–S5" in ADR 0006's labels |
| (c) B1–B11 called options | upheld (low: wrong category, right scope) | The rows sit in part 13, "The security baseline: open, with interim positions" (0006:369). Their table is headed "\| # \| Item that waits \| What the baseline decides \| Interim position \| Owner draft \|" (0006:375), rows at :377–387. That is outside "## Options considered" (0006:404 on). ADR 0006 calls them items: "item B5" (:639), "as part 13's table lists them (B1–B11)" (:676), "(part 13, B1 …)" / "B4" / "B11" / "B6" / "B1" (:1062, :1068, :1072, :1077, :1082). They are local to ADR 0006, so the line does its main job of marking them local. Only "option labels" is wrong. | "… ADR 0006's option labels (G1–G4, K1–K4, S1–S5) and its baseline items B1–B11 (part 13)" |

**The three fixes in one line (README:269–270):** "Codes local to one draft are not register codes, for example ADR
0002's gate checks G0–G13, ADR 0004's spike claims S1–S22, ADR 0006's option labels (G1–G4, K1–K4, S1–S5) and its
baseline items B1–B11 (part 13); a register code always carries its word ("gap G5", "seam S1")."

**Why "for example" (outside the finding; added so the line becomes true, not as a new finding):** after the colon,
the line reads as a complete list, and it is not complete:
- ADR 0002 has its own option labels: A1–A3, B1–B4, C1–C2, D1–D4, E1–E2, H1–H2, L1–L2, T1–T2, W1–W2 and P1–P5
  (0002:575–724), plus two G1–G3 sets, Q13 at 0002:662–664 and Q17 at 0002:702–704. So "option G1" means two things in
  ADR 0002: isolation off (0002:296) and the CI shape (0002:474). Gate check G1 is a third (0002:451).
- ADR 0006 has more families than the list names: Q1–Q13, A–E, (i)–(iii), C1–C3 twice (Q5 at 0006:454–457 and Q8 at
  0006:482–484), L1–L2, M1–M3, T1–T2, P1–P2, W1–W3, R-a/R-b and N1–N2.
- ADR 0001's network profile B1 ("B1 = P2 plus one burst …", 0001:399, :1060) is cited in ADR 0006 ("profiles (P0–P4,
  B1)", 0006:1001), in the same file as baseline item B1 (0006:377). Neither is a register code, so README:270's rule
  does not tell them apart. ADR 0006 does name the profile ("ADR 0001's profiles"), so in context the reader can tell.

No web page was needed or fetched. No doc was edited.
