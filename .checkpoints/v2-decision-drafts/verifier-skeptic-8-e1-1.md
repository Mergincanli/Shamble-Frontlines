# verifier-skeptic-8 e1-1 (F2)

Notes: started; reading verdict r4 and STOP-ANSWERS.

## F2 verdict block
- Doc 0002:1086-1087 (HEAD 59d41e0): "the sim core spike's sim, which exists by then, serves the marks that need the real sim (the advisor's reading, 2026-11-03)".
- Verdict r4:169 (SPIKE-ORDER): "The boundary spike now uses the sim core spike's sim, so its 'stand-in until then' clause may go." Says nothing about "the real sim".
- Verdict r4:77 "deciding run at slice 6 on the real sim"; r4:160 "Slice 6 always runs the final 60 Hz measurement on the real sim". Mustafa (STOP-ANSWERS, TICK-60-SETUP): "measured again with the real sim at slice 6".
- ADR 0002 itself: 1294 PM-0002-70 "hold on slice 1's real sim and page"; 1348-1349 "real sim decides ... at slice 6". 1178-1180 (rewritten this round) now separates "the game's sim, from slice 1 on" from "the sim core spike's sim ... the BX rows run again with it". Before 03ef222 line 1165 read "the real sim's cost is unknown until ADR 0004's sim core spike; once that sim runs in this Worker" - the older, looser use; the author replaced it, so in the current doc "real sim" = the game's sim (slice 1/6), never the throwaway spike sim.
- So 1087 contradicts the doc's own PM-0002-70: marks that need the real sim run at slice 1, not in the spike. Over-read upheld.
- Nuance: the verdict's gloss "slice 6's real crate" is a bit narrow (ADR 0002 also uses slice 1's real sim), but the finding stands.
- Fix: as given works; a more anchored option: "serves the BX rows' second run (BX-P's note)". As given accepted.
OUTCOME: upheld. END
Notes: F2 done; returning result.
DONE
