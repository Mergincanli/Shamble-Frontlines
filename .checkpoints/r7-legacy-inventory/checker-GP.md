checker-GP: R7 GP fixes2 checker
- apply ran rc=0; only the 13 expected fields of GP2/3/4/10/15 changed
- GP2: ok (786, 1169, 3255, 3293-3294 verified; text exact)
- GP3: ok (core 67/76/85/128-137 values, 2748-2752 yaw, 2783-2787 stamina, 1116 verified; text exact; tuning appended via replace of last item with '; ')
- GP4: ok (camShake/camRecoil/fovKick/camKickVel greps, core 68-75, 92, camPosLean only at core 68/73; text exact)
- GP10: ok (2141-2142, 2962-2964, 2850-2854, updateBaseHUD callers 848-851/1561-1562/1850/3362 event-driven, GP7/GE16/PG3/PG4/PG10 cross-refs)
- GP15: ok (3055-3372 sub-ranges 3056/3059-3188/3189/3192-3204/3208-3218 (4x3=12 slots)/3225-3235/3236/3368-3371; AUDIT_BUGS_PERF:88-91, START_HERE:133, game:26, 683, index:1259, editor:1261; text exact)
Result: ok=true, no problems.
DONE
