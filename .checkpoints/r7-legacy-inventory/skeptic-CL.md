Task: R7 legacy inventory, skeptic for area CL (core look pipeline), rows CL1-CL13
Checks: git diff 48ffb51..HEAD on shamble-*.js and docs/ empty. State tags (plan.md:18): C code read, M measured, D doc not re-checked.
CL1 corrected: lines ok (239-242,294,233; game:3379 shim writes NPR/COMIC/CHAR_OUTLINE, comment game:3374-3378 V8 const-tracking). state D->C (code read). evidence ok (AD:15-18, T:45, T:48).
CL2 corrected: lines ok; state D->C; also core header :234 says "3-band" (drift). AD:48 header, :50, :53 ok.
CL3 corrected: toonify call is :283 not :285. rest ok (254, 255-266, cache key :265; T:44, T:219).
CL4 corrected: toonify 278-286 (not 287); callers stylizeScene :404, stylizeOne :415 (not 403/414); 0 refs in game/editor (row says 1). evidence ok (A:112, T:44, T:53 gain 1.56). _natureGain :179.
CL5 corrected: 268-277 ok; callers game:3364, editor:288 ok; A:17 ok; AD:72 ok. "nothing sets en.elite" -> game:3212 creates every enemy with elite:false; nothing sets true. 3211 'no elite (D6)' ok.
CL6 corrected: makeOutlineMat 295-303 (not 305); addHull 389-396 ok; "AUDIT CS12: saved count 2 per merged mesh" not in audit -> code core:386. STATUS:84 120/soldier ok; TTC:50 1,480 / 5,843 ok; steps1-6:57 5% vs 15-25% ok.
CL7 corrected: clear at :344 (not 346); ensureOutlineNormals 333-347 (not 352); addHull call :393 (not 394). editor 877/882 ok, T:47/49/50 ok, hyp3 744 ok.
CL8 corrected: stylizeOne calls in game = 25 on 21 lines (grep -o 'stylizeOne(' ), editor 4. Rest ok (397-408, 411-419, game:3363, EU:27, S:124, steps1-6:89-95, S:91 PR#8).
CL9 confirmed: 353, 356-359, 360-388, callers 577/583/598/604/608; A:112 figures; S:121; T:249; vertexColors only at core:359. saved formula core:387.
CL10 corrected: weapon :453 (not 452). CHAR_MALE/FEMALE are the in-game soldier bodies via assembleChar core:628-629, buildSoldier game:126-130; CHAR_DEFAULT parts never built: only .weapon (game:128, editor:282) and .hit (core:1482) read. state D->C.
CL11 confirmed: 455, 462, 506, 554, 1482; BODY used in hit tests core:1497-1502; _BODY_HIT not read by hit test. TP:159 ok.
CL12 corrected: GUN_WORDS :45, KILL_WORDS :46, makeComicTex :48 (canvas :49); callers add comicGun game:1473 (fireOnce, player only), comicKill game:1536 (onDamageEvent). pre-warm sentence A:65 not 62-64. B2 A:23 ok, A:81 ok.
CL13 corrected: lines ok; state D->C.
Missing: userData tag contract (matKind core:687, :754; outlineW core:755; part; noOutline; isOutline; _toon; noMerge; nature). Halftone covered CD16, groundGrad CD20, comicPost GP6.
DONE
