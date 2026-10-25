# checker-PG notes (R7 PG checker)
PG3: ops match correction exactly for what/where/v2_note; only those 3 fields changed in after-PG.json; 20 rows same. Now spot-checking lines.
PG3 spot-check: all new game.js citations hold (1043-1047, 1096-1098, 1820, 1542-1545, 1827/1846 reloadBar opacity, 1856, 1858 dryFire, 1861-1865, 2342, 2413-2416, 2495, 2499-2506, 2606-2608, 2687-2689, 2784-2787, 3045, 3309 pipWrap); index.html 199-202, 204-206 #terminalPrompt, 704-716, shake :716 hold; CLAUDE.md:4 Foxhole, LOOP_V2.md:92 hold.
PG3 problem: where says ".slot rules :80-85" but index.html:80 is '#slots{display:flex;gap:5px}' (live, matches #slots at :798); .slot rules are :81-85. Fix op: replace "(.slot rules :80-85 match no element)" -> "(.slot rules :81-85 match no element; :80 is the live #slots flex rule)".
DONE
