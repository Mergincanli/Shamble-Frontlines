Task: R7 legacy harvest, skeptic for area ED (editor), rows ED1-ED15, main 48ffb51
Checkout: HEAD dac3948, shamble-*.js/docs identical to 48ffb51 (git diff --stat empty). editor 1252 lines.
ED1 confirmed: all symbols at cited lines (decl dump); STATUS:95 PR#13; hotfix report l.16-17 42/42 PASS, 0 regressions 24 runs; inventory N3 draw calls [D0].
ED2 confirmed: lines ok; hotfix H1/H2 l.29-30, symbols +2 l.18, leak fix l.15; edClearBoxes :72 disposes material.
ED3 confirmed: :185-210 read; house desc has no spec (195), d.spec branch 209; wall branch 186.
ED4 corrected evidence: P4 stale -- hotfix E2 (report table) + game.js:3255 Escape never leaves editor (tool->select, clear sel, nothing). N1 confirmed: deploy game.js:1107 calls navBuild, resetRound 1838 doesn't, editorPlay 451-456 sets mode itself. P1 confirmed :657.
ED5 corrected (evidence): CL_CTRL 13 (:262-275), clUndo 50 (:298), DEV.enabled :433, clImport only o.def :445, TECHNICAL:202 write-only key (only writer editor:444), GD §7.5 IMPLEMENTED. A11 doesn't literally say "only through dev panel"; code does: only caller dvCharLab bound at :1030.
ED6 confirmed: POSE_CTRL 18; applyRifleHold called only from editor :315/:317 (game.js:479 def); ANIM.aim.carry/ads unread by game (only adsK game:290). ANIMATION_PLAN §7 l.78-79, TECHNICAL:169.
ED7 confirmed: 17 actions, 8 knobs, 7 hold; alTick calls updateSoldierAnim(g,st,d) (game:271); ROADMAP:70.
ED8 confirmed: 15% at :519 (>0.15), cover step max(12,span/16) :565, 'squares' :489; K1-K4 figures ok; STATUS:108.
ED9 confirmed: ENV 9+9+4=22, RULES 5+4=9.
ED10 confirmed: :827 BK.PARTS loop, :830 BK.DIMS loop, core:1296 const BK.
ED11 confirmed: :844 consts; report l.26-30 numbers match; N2/N3.
ED12 confirmed.
ED13 confirmed: try{}catch(e){} silent on save :697; dedupe :994; reports figures l.29-30, prefab-list l.15 +3.
ED14 confirmed (minor: orphan comment :1248-1250 inside bootEditor; Export/Reset at :1104-1105 not mentioned).
ED15 corrected: where misses game.js:1107 (deploy typeof edGhost) and :774 (bots hidden mode==='editor'); evidence add hotfix report security L: "Check 4: 51/51 references guarded" (later than STATUS 44 on 8675367). index.html:845 charLab DOM too.
Missing: snap/rot step :54-55,:458,:582-584; Export/Copy/Reset bootEditor :1104-1108; edFootprint :61 + edGhost :235; pick plumbing :67-70; overlay infra :460-473; drag-tool pointer logic :1060-1093; inspector column :971-972,:1244-1245; orphan comments :46-48 (code in game:611-613), :1248-1250.
DONE
