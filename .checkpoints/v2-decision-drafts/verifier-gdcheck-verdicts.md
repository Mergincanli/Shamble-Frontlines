# verifier-gdcheck verdicts

## Round 1

Checked: /home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/touchup2-game-designer-fixes.md (5 FIX blocks, 0 NOTE blocks)
against docs/adr/0005-content-abilities-assets.md on disk at HEAD cda0577 (docs identical to 41665c7).

| block | finding | holds? | problem |
|---|---|---|---|
| F1b | fixcheck-d2 F1b (high): AB-3's heading lost between AB-2's Need (0005:900) and AB-3's What (0005:901) | yes | none. BEFORE once (on-disk 900–901, inside 746–1432). The inserted line is byte-identical to 47eadd9:0005:832 (`diff` empty) and sits at applied 903, between AB-2's Need (902) and AB-3's What (904). Headings AB-1..AB-10 now each appear once, in order. |
| LOST-AB1-WHY | own lost line: RR-4 (game-designer-fixes-final.md lines 3–44) took AB-1's Why first half into its BEFORE and left it out of its AFTER, leaving the dangle 0005:883 "    names, with no generic-engine feature." | yes | none. BEFORE once (on-disk 882–883, inside the section). The restored pair (applied 884–885) is byte-identical to 47eadd9:0005:819–820 (`diff` empty); no wording change, so no reason needed. Position: after the "When a pool is full" item that RR-4 inserted between What and Why, so AB-1 reads What, When a pool is full, Why, Research, Need, the order RR-4 intended. The dangle is gone (it is now the Why line's own second half). |
| F10 | fixcheck-d2 F10 (low): AB-5's Need line twice (0005:948 and 0005:949) | yes | none. BEFORE once (on-disk 948–949). Applied AB-5 (934–951) has one Need line, 951 "Need: VISION_V2 §6.5; the faction's list: §6.7.", which contains the deleted line's §6.5 and adds §6.7; nothing the two said is lost. |
