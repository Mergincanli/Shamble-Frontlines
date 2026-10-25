Task: R7 legacy harvest, reader ED (editor: shamble-editor.js) at 48ffb51
## code read (editor.js same as 48ffb51, verified git diff)
- header 1-16; orchard/poprow tools 18-45; ed state 49-66; selection 67-76; gizmo/handles/ring 77-127; move/rotate/scale 128-160; mirror 161-167; array 168; marquee 178; describe/spawnDesc/dup/copy/paste/nudge 185-222; enterEditor 224
- Char Lab state 240-261, poseLab 245 POSE_CTRL 249, CL_CTRL 262, clDispose 277, clAssembled 278, clRebuild 280, clHideWorld 292, clCam 296, clUndo 297-299, clBuildPosePanel 301, clExportPose 316, clTick 317
- Anim Lab 318-390: alOn 322, _alSt 323, AL_ACTIONS 324, AL_KNOBS 327, AL_HOLD 331, alCopy 335, alReset 337, alEnsureUI 338, alRefresh 361, alPrepare 371, alTick 376 (treadmill: feet slide back)
- clBuildPanel 397, enterCharLab 432 (DEV.enabled only), exitCharLab 443, clExport 444 (writes KV shamble_charlab: write-only, nobody reads), clImport 445 (no validation), editorPlay 451, setEdTool 457
- overlays 459-581 (measure 475, spawn dist 494, symmetry 529, cover heatmap 557); edGround 585, edPlaceAt 588, edRemoveOne 626, edDeleteSel 639, edSetTeam 640, ortho 645-653, Tab preview 655-675 (edPreviewUpdate 662), updateEditor 676
- MapStore 695 (vmap:), openMaps 702, PrefabStore 704 (vprefab:), edStampPrefab 710, edSelectPrefab 711; World panel ENV_SCHEMA 715 edBuildWorld 748; Rules RULES_SCHEMA 770 edBuildRules 788; Building Maker _bk* 801-843; undo 844-849 (_EDUNDO_MAX 60, _EDCO_WIN 1200, full serializeMap+buildMap snapshot); inspector 863-972; edLeaveToMenu 974; edFlashHint 985; refreshMapList 987 (dedupeMapDef on load); edSavePrefab 1004; refreshPrefabList 1010; bootEditor 1022-1252 (event wiring, CL pointer, chrome IC icons 1141, tooltips 1232, inspector 1246)
- game hooks: loop 3053 (updateEditor/clTick/edOrthoCam behind mode), keydown 3255 (editor shortcuts behind mode==='editor', typeof setEdTool/edUpdateGizmo/edSetGizmoModeKey), wheel 3293, backToMenu 2985 typeof edLeaveToMenu; header game.js 20-22; index.html:878 dvCharLab button present but unwired in player build
## evidence gathered (run 2)
- line numbers re-verified by grep at 48ffb51 (all functions above match)
- EDITOR_V1_INVENTORY: 107 rows (keep 32 / improve 68 / drop 7) + 10 hidden items; its lines are main 67cf55b, not 48ffb51
- EDITOR_V2 header: Char Lab + Anim Lab "stay as they are" (scope map editor only); §2 five rules; §4 quality bars
- undo-ground report: draw calls boot/1/2 rebuilds base 1992/2406/3502 head 1992x3; heap 42.9->119.5 MB base, 42.7->44.5 head; inventory N3 ~300 ms/undo Cabana [S], 280-850 ms [D0]; N2 snapshot ~12 KB
- hotfix report: 42/42 checks PASS, symbols +2 (_edCanRot,_edToolKind)
- STATUS:96 N1 editorPlay never rebuilds nav grid; :97 _disposeObj3D disposes shared BK textures; :93 prefab stack dedupe deferred; :108 reachability check recommendation; :124 health row
- inventory K3 cover heatmap 172 cells ~4.6 s; P2 NEW-9/NEW-11 Play bugs
## rows (run 2)
- ED1 placement+scatter: 19 _orchardCells,39 _rowCells,457 setEdTool,585 edGround,586 _edToolKind,588 edPlaceAt,626 edRemoveOne,639 edDeleteSel,640 edSetTeam,856 edSetLibMode,862 edBuildRecent; inv C; BUG-1 fixed PR#13; M; redesign EDITOR_V2 §2 r2-3,§3, §6 3.3; look F
- ED2 select/gizmo/transform/mirror/array: 73 edSetSel,79-80 handles,85/94 gizmo,115-123 ring,124 edSetGizmoModeKey,129 edMoveBy,158 edRotateSel,160 edScaleSel,163-167 mirror,168 edArraySel,178 marquee,222 edNudge; hotfix 42/42; inv D,E,H; M; redesign §3 select/transform+120° symmetry; look F
- ED3 descriptors copy/paste: 185 edDescribe,199 edSpawnDesc,213 dup,219 copy,220 paste; NEW-3 spec lost, dead d.spec; M; redesign §2 r1; look F
- ED4 lifecycle/test: 224 enterEditor,451 editorPlay,644 edZoom,647-653 ortho,657/660/662 preview,676 updateEditor,974 edLeaveToMenu; STATUS:96 N1, inv P1-P4 NEW-9/11; M; redesign §3 Test; look F
- ED5 Character Lab: 240-261 state,262 CL_CTRL(13 sliders),277 clDispose,278 clAssembled,280 clRebuild,292 clHideWorld,296 clCam,297-299 clUndo(50),317 clTick,397 clBuildPanel,432 enterCharLab(DEV.enabled),443 exit,444 clExport(KV shamble_charlab write-only),445 clImport(no validation),446 clSetSex; TECHNICAL §5.4, :202; perf none recorded; C/D; port TS; look T
- ED6 pose tool: 244 poseLabActive,245 poseLab,249 POSE_CTRL(18),301 clBuildPosePanel,314 clTogglePose,316 clExportPose; ANIMATION_PLAN §7 legacy; C; drop; look F
- ED7 Anim Lab: 322-390, AL_ACTIONS 17, AL_KNOBS 8, AL_HOLD 7, alCopy 335, alReset 337, alEnsureUI 338, alRefresh 361, alPrepare 371, alTick 376 treadmill; ANIMATION_PLAN §2,§8.3; ROADMAP:70 awaiting Brave; C; port TS; look T
- ED8 overlays: 464 ovLabel,477-492 measure,496 ovObjectives,509 ovBuildSpawn,525 toggleSpawnDist,543 ovBuildSym,555 toggleSymmetry,563 ovBuildCover,581 toggleCover; inv K1-K4 (9/27, 172 cells 4.6 s); STATUS:108; M; redesign §3 validation; look F
- ED9 World+Rules: 715 ENV_SCHEMA,748 edBuildWorld,767 sync,770 RULES_SCHEMA,788 edBuildRules,799 sync; inv L1 22 ctrls, M1 9 sliders; M; redesign LOOP_V2 §3-4 + look brief; look T
- ED10 Building Maker: 801-843 _bkPlaceAt 806,_bkBuildPreview 812,_bkRoll 819,_bkBuildPanel 820 (BK.PARTS 827/BK.DIMS 830, core BK 1296),_bkCloseMaker 841,_bkToggle 842; inv J1-J7; M; redesign §2 r1/§3 parity; look T
- ED11 undo: 844 consts(60,1200),846 edPushUndo,847 _edApplySnap,848/849; undo-ground report; N2 12KB,N3 300 ms/280-850; M; redesign §2 r2,§4; look F
- ED12 inspector: 850 edStatusSync,865 edKindLabel,866 _edCanRot,871 edMoveSelBy,872 edRotateSelTo,874 edSetHP,875 edResize,881 edReshapeRamp,885 edBuildInspector,963 edSyncInspector; hotfix H1/H2; inv G; M; redesign §3; look F
- ED13 storage+prefabs: 695 MapStore,702 openMaps,704 PrefabStore,710 stamp,711 select,985 edFlashHint,987 refreshMapList,1004 edSavePrefab,1010 refreshPrefabList; dedupe report 280/144; STATUS:93; inv O1,O5,I1-I3; M; redesign §2 r1,§4 migration; look F
- ED14 bootEditor: 1022-1252 (CL orbit 1025-1028, IC 1141, tooltips 1231+), dead handlers inv hidden 2-4; C; redesign; look F
- ED15 game hooks: game 20-22, 2985, 3053, 3255, 3293; index.html:878; STATUS:125-126 44 refs guarded; M; drop; look F
- not_covered: per-feature rows (in EDITOR_V1_INVENTORY), game-side serializeMap/buildMap/dedupeMapDef + KVStore (other readers), editor.html DOM/CSS, VISION_V2.md absent in checkout
DONE
