# verifier-skeptic-5 notes (plan step 7, skeptic of fixcheck-c Findings 2)
Started 11:08 Amsterdam. Docs read at 47eadd9 (extracted to .gate/skeptic5/). Finding: README:269-270 code families.
Parts: (a) G1-G13 vs G0 · (b) S1-S3 vs S1-S5 · (c) B1-B11 called options.

## Verdicts as found (all at 47eadd9; HEAD differs from 47eadd9 only in docs/reports/, so the ADRs and README are the same)
- README:269-270 at 47eadd9: "ADR 0002's gate checks G1–G13, ADR 0004's spike claims S1–S22 and ADR 0006's option
  labels (G1–G4, S1–S3, K1–K4, B1–B11); a register code always carries its word". Origin: copied from
  engine-architect-consistency-findings.md:193-194 (F12 outcome), which had the same ranges. Line added in 47eadd9.
- (a) UPHELD. 0002:449-463 table "| Id | Stage | Check |" starts "| G0 | setup |" (0002:450). 0002 itself counts G0 as a
  check id: :445 "`--quick` (G0–G5 …)", :1082 M-0002-39 "(parts 2 and 9: G0)", :1095 "one control per check id
  (G0–G12; G13 from slice 2)", :1225 "G0–G12 (G13 joins …)". G0 there since 789bedb (first draft). Harm small: no
  register code G0 exists (gaps G1, G2, G3, G4, G5, G7, G8, G9, G11, G15), so no ambiguity; still a wrong range.
- (b) UPHELD (range); collision point overstated. 0006 Q10 table: :503 "S1. … (proposed), S2 Parquet compaction only
  when needed", :504 "S3. Cloudflare R2 instead", :505 "S4. A SQL database", :506 "S5. A hosted analytics warehouse".
  Labelled since 5bdad85 (first draft). Unresearched rows elsewhere go unlabelled (0006:439 Netcup row, :455 dns-01,
  :458 tls-alpn-01), so S4/S5 are deliberate labels. Collision: 0006 itself cites "seam S3" (:85, :164) and "seam S4"
  (:90) and every seam cite carries the word (:85, :90, :91, :92, :164, :354), so README:270's rule does disambiguate;
  S1 and S3 meet seams S1 and S3 just as S4 meets seam S4. The missing part is only S4-S5 in the list.
- (c) UPHELD (low). 0006:375 table header "| # | Item that waits | What the baseline decides | Interim position | Owner
  draft |", rows B1-B11 at 0006:377-387, inside part 13 "The security baseline: open, with interim positions" (0006:369),
  not under "## Options considered" (0006:404). 0006 calls them items: :639 "item B5", :676 "as part 13's table lists
  them (B1–B11)", :1062/:1068/:1072/:1077/:1082 "(part 13, B1/B4/B11/B6/B1)". Function (marking them local) holds; the
  word "option" is wrong.
- Beyond the finding (observations): ADR 0002 has its own option labels (0002:575-724: A1-A3, B1-B4, C1-C2, D1-D4,
  E1-E2, G1-G3 for Q13 at :662-664, G1-G3 again for Q17 at :702-704, H1-H2, L1-L2, T1-T2, W1-W2, P1-P5); 0002 cites
  "option G1" for two things (:296 isolation off, :474 CI) beside gate check G1 (:451). ADR 0006 has more families than
  listed (Q1-Q13, A-E, (i)-(iii), C1-C3 twice: Q5 :454-457 and Q8 :482-484, L1-L2, M1-M3, T1-T2, P1-P2, W1-W3, R-a/R-b,
  N1-N2). 0006:1001 cites ADR 0001's network profile B1 ("profiles (P0–P4, B1)", from 0001:399, :1060) in the same file
  as baseline item B1 (0006:377). So the line, read as a complete list, is incomplete; "for example"/"among others" needed.
