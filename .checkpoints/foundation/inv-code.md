# Task: foundation 2.1 inventory agent A (code + tools): hits-a-code.txt + full read of tools/, CI, hooks, scripts, settings, root files, the five legacy files; design Q1-Q4.

## Plan (blocks, in order)
B1 read tools/gate.mjs
B2 read tools/smoke.mjs, battery.js
B3 read preview.js, pageshot.js, sim.js, reach.js, perfbots.js, tuning-dump.js, aabbs.js, village.py, README.md, package.json, gate-globals.json
B4 read .github/workflows/gate.yml, .claude/hooks/*.mjs, .claude/scripts/*.mjs, .claude/settings.json, .gitignore, .gitattributes
B5 read README.md, FIRST_PROMPTS.md, docs/ui_theme_mockup.html, five legacy files (links, hrefs, three URL, KVStore, map export)
B6 advisor.md :85-100, :150-160; gate-check-5 report :30-41; phase-0 report :25-27
B7 build-tuning.py at a715be2
B8 write files[] rows
B9 write design Q1-Q4
DONE marker at end

## Notes
- B1 gate.mjs: ROOT :33 (repo root), FILES :34, OLD_PREFIX :35, baseFile :92-95 tries [f, OLD_PREFIX+f], headFile :96-100 join(ROOT,f), toBuild :101; classicParts :144-157 resolves src against ROOT (:150) with outside-repo check :151 and LOCAL keyed by basename :152,:222; smokeCheck :190 runSmoke({root: ROOT}); 5d :351-353 THREE_URL + WANT basenames (src stays relative: NAME, no change); 5c :341 labels basenames. gate-globals read via TOOLS :289 (unchanged). git -C ROOT :68 (unchanged).
- finding: head has no fallback (die at :98 names f): under move must read ROOT/legacy/f.
- finding: base fallback mixes per file (each file independently); propose picking one prefix per base where all five exist.
- B1 done; next B2 smoke.mjs, battery.js
- B2 smoke.mjs: ROOT :18 repo root; ROOT_URL :23 + rel :24 only shorten messages (cosmetic: paths show as legacy/...); runSmoke default root=ROOT :118; CLI runSmoke() :201 (default); join(root,name) :130; inRepo :147 checks reads stay under `root` (passing legacy/ makes it stricter: fine, pages read only their own folder). newContext per page :134 = fresh storage (no KVStore carry-over).
- B2 battery.js: readBuild(dir) :64-68 and CLI :210-213 take a folder: no change (callers pass legacy). Comments :8,:12,:175 NAME. Gate uses only analyze/scanHtml/attrsOf/scriptKind/handlersOf (no paths). battery.js needs NO edit under (a).
- B2 done; next B3 other tools
- B3 preview.js: dir arg :17, reads dir+'/shamble-core.js' :93-94 (works with `legacy`); eval/pre snippet paths are cwd-relative :95,:99,:108 (tools/sim.js etc. still found from repo root). location stub :68 (search only). Comment :2 COMMAND (`preview.js . <cmd>`), :5,:13 NAME. No code change.
- B3 pageshot.js: dir arg :19, path.resolve(dir,pageFile) :36; three routed :33. Comment :5 NAME, :8 COMMAND (`pageshot.js . index.html`). No code change. newContext :29 fresh storage.
- B3 sim.js:1, reach.js:1, perfbots.js:2, tuning-dump.js:3, aabbs.js:1: header COMMAND `preview.js . eval`. No code paths (they run inside the booted game).
- B3 village.py: :2 COMMAND (+default doc), :10 BREAKS (default CORE_JS = <repo>/shamble-core.js -> FileNotFoundError when run with no args). Change: os.path.join(_REPO,'legacy','shamble-core.js'). Not protected.
- B3 tools/README.md: :3 NAME; :22 COMMAND (`battery.js .`), :23-:28 COMMAND (`preview.js .`, `pageshot.js .`), :56, :58 COMMAND; :36-37, :42, :61 NAME. :4 "from the repo root" still true. Not protected. (grep list missed :22,:24-:27,:58: no file name on those lines.)
- B3 package.json, gate-globals.json: no paths. Checked, nothing.
- B3 done; next B4 CI/hooks/scripts/settings
- B4 gate.yml: no file paths; `node tools/gate.mjs "$BASE" --no-smoke` (:51) and `--smoke-only` (:93) unchanged under (a). On the move PR itself BASE=origin/main = root layout, so the base fallback must find root files (NOT legacy/). Checked: no `paths:` filter. Protected (copy).
- B4 syntax-gate.mjs: path-independent (works on any file_path). Checked, nothing.
- B4 guard.mjs: no paths. routine-guard.mjs: GATE_FILES :26-27, DENIED_SCRIPTS :437, CANON :71 name tools/ files only; protectedRel :32-43 never matches legacy/ (no CLAUDE.md there): legacy files stay non-kit. Under (b) GATE_FILES/DENIED_SCRIPTS/CANON/NIGHT_NPM :28 and :534 must change -> tier 2 edit, collides with PR #46 (guard + tests).
- B4 routine-guard.test.mjs hits (138,684,688,730,741,759,763,767,784,958,1471,1481,1747,1755,1801): TESTDATA (fake /w,/p paths, temp repos). settings.test.mjs:232 uses the REAL repo root + 'shamble-game.js' as a non-kit path; file need not exist (realOf :45-51 walks to nearest ancestor) -> still passes; TESTDATA.
- B4 checkpoint.mjs/test: no legacy paths (test fixtures code.js etc.). Checked, nothing.
- finding B4: repo-map-check.mjs :22-38 requires every top-level path in docs/REPO_MAP.md: after the move `legacy/` is "missing from the map" and REPO_MAP.md:14-18 root entries are "in the map but not in the repository" -> exit 1. INDIRECT; REPO_MAP is docs (PR #40 also edits REPO_MAP: collision).
- B4 settings.json: allow rules name tools/ scripts only (:6-14, :44-49), `node .gate/*.js` :51, `ln -s ../../../tools/node_modules .gate/*/tools/node_modules` :74 (one level under .gate/: a worktree at .gate/foundation/<wt>/ needs ../../../../ and may prompt). Unchanged under (a). Protected.
- B4 .gitignore: .gate/, node_modules/, dist/ (no root-file names). .gitattributes: `* text=auto eol=lf` applies to legacy/ too (no path rules). Checked, nothing.
- B4 done; next B5 root docs + five files
- B5 README.md (not protected): :12 STALE ("Double-click `index.html`" -> `legacy/index.html`), :13 STALE (`editor.html` -> `legacy/editor.html`), :19-:22 STALE (layout table rows: paths -> legacy/... or one `legacy/` row). :24 ok.
- B5 FIRST_PROMPTS.md :29-31, :77, :81: HISTORY (the CS-1 era prompts). No change.
- B5 docs/ui_theme_mockup.html: self-contained (data: URIs only, no src/href to repo files). Checked, nothing. index.html:222/editor.html:222 mention it by bare name in a CSS comment: NAME.
- B5 index.html: three CDN absolute :1255; `<script src="shamble-core.js">` :1256, game :1257: RELATIVE, resolve to legacy/ after git mv (no change). Header :3,:5 NAME. editor.html: CDN :1256, srcs :1257-1259 relative, header :3-:8 NAME. No href/link between pages (grep href=/location/window.open: none).
- B5 JS: only location use = shamble-game.js:3189 (location.search, ?map=tri; path-independent). No fetch/XHR/import()/Worker in the three files. Header comments core :2,:22,:1477; editor :2-:15; game :2-:23: NAME.
- B5 KVStore shamble-core.js:28-41: window.storage -> localStorage -> memory. Keys: shamble_prog (+legacy read vanguard_prog) game:1953-1954, shamble_tutorial_seen game:2335/2338, shamble_charlab editor:444, vmap:<name> editor:696-699, vprefab:<name> editor:705-708, probe shamble_kv_probe core:38.
- B5 map export: editor.html:1058 #edExport -> shamble-editor.js:1100 JSON.stringify(serializeMap()) of the CURRENT map into #edJson + Copy (:1104). One map at a time; no map Import button (JSON "to paste into the code", editor.html:1058). Prefabs: no export. Char Lab: Export/Import (editor:444, :1030). openMaps note editor:702.
- B5 done; next B6 advisor.md + reports
- B6 advisor.md:90-91 (step 2: CI green does not stand when .github/ or a gate file changed -> needs-mustafa) and :148-152 (day-session advisor reproduces in a worktree; "If the PR changes a tool you reproduce with, use main's copy"), :153-156 (routine: CI green stands only if no gate file touched). => evidence must come from main's gate + differential runs, not CI.
- B6 phase-0 report :25-27: 4 mutations (refreshRect deleted -> [3] DROPPED; bootEditor(); in bootGame -> [4] FLAGGED; syntax error -> [1]; wrType copied into core -> [3] cross-file dup) + controls: --allow-drop refreshRect PASS; a pure move PASS listed as moved. Also :24 "PASS vs 8e58937 (pre-move layout)".
- B6 gate-check-5 report :32-41: 22 mutations by failing set (5a only x5; 5a+5b x2; 5a+5d x2; 5b only x8 if "bare localStorage and status" counts 2; 5b+5d x2; 5c x1; 5d x1) + 3 controls :40-41. Note: "image read from a file outside the repo" mutation (:33) becomes "outside legacy/" with root=legacy: still FAILs (stricter).
- B6 8e58937 has the CS-1-era layout (shamble-frontlines-repo/<f>); main = origin/main = 44446a5 (root layout).
- B6 done; next B7 build-tuning.py
- B7 build-tuning.py @a715be2 (141 lines). Hard-coded paths: DUMPS :5-6 (.checkpoints/r7-legacy-inventory/tuning/{cabana,tri}-1.json, input); COMMIT '48ffb51' :7 + `git show 48ffb51:shamble-{core,game}.js` :24 (root paths at a FIXED old commit: still valid after the move, do NOT change); outputs open('docs/legacy-tuning.json','w') :53, open(...,'a') :54, open('docs/LEGACY_TUNING.md','w') :140. Strings that must stay: produced_by `preview.js . eval` :42 and :76, `docs/legacy-tuning.json` in content :74, :99 (so a blind replace would change the output; replace only the three open() calls + the two DUMPS values). cwd must be the repo root (relative git show + paths). Header :2 run command.
- B7 checks: engine files unchanged 48ffb51..44446a5 (git diff --stat empty); docs/legacy-tuning.json + LEGACY_TUNING.md last touched 70e31f4 (ancestor of a715be2); builder + dumps unchanged 70e31f4..a715be2; nothing in a715be2..main touches the docs, tuning-dump.js or preview.js. So head dump through legacy/ should reproduce the committed reference byte for byte (UNKNOWN until run).
- B7 note: the `where` column comes from 48ffb51, not from the run: the oracle tests values/bootChanges/skipped only.
- B7 done; next B8 files rows, B9 design
- B6b (advisor follow-ups) mutation list, one per line (gate-check-5.md:31-39), counting "bare localStorage and status" and "const: same script / another script" as two each -> 22:
  5a: (1) PLAYER_FACTION TDZ (2) throw in deploy() (3) keys['KeyWW'] (4) WebSocket in deploy() (5) image read outside the repo
  5a+5b: (6) main's old shamble-editor.js (7) refreshMapList re-nested
  5a+5d: (8) core/game tags swapped (9) defer on game script
  5b: (10) undefined name in index handler (11) typo editor handler (12) typo in never-run code (13) implicit global (14) bare localStorage (15) bare status (16) THREE.CapsuleGeometry (17) const write same script (18) const write other script
  5b+5d: (19) unquoted onclick=noSuchFn() (20) unquoted <script src=shamble-editor.js>
  5c: (21) core calls game. 5d: (22) type="text/ecmascript" editor script in index.html.
  Controls (PASS): valid handlers; commented-out tag + on… variable + </script >; endlessly animated Deploy. The report's own grouping is ambiguous (21-23): say so.
  Mutation scripts: none in git history (git log --all name search for mutat*: empty) -> 2.1 re-authors them under .checkpoints/foundation/.
- B6b map import: editor.html:1248 #edJson is `readonly`, header "copy this into the code as a map definition": confirmed no map import in the editor.
- B6b asset reads: no url( outside data: in either page; no <img>/<audio>/<link>/<iframe>/@import; JS: no .src= / new Image / new Audio / loaders (r.src at game:2468-2478 is a scoreboard row field). So smoke root=legacy/ narrows nothing the pages use (from reading).
- B6b settings.json read whole: allow (:5-140) names tools/ scripts, `node .gate/*.mjs|*.js` :50-51, `mkdir -p .gate/*` :72, `sha256sum *` :73, ln :74; deny includes `git checkout *`, `git restore *`, `git reset --hard`, `git clean *`, `rm -rf *`, `git worktree remove *--force*/-f`, `npx *` (=> throwaway worktrees: undo a mutation with `git -C <wt> show HEAD:<p> > <wt>/<p>`, remove with plain `git worktree remove <wt>` once clean). Hooks run ${CLAUDE_PROJECT_DIR}/.claude/hooks/guard.mjs: no legacy paths. Unchanged under (a).
- B6b guard path logic: routine interpreter (routine-guard.mjs:959-984) runs `node <script>` only if tracked AND equal to origin/main's copy (:978); .gate/ scripts refused (:971). FINDING: after 2.1 merges, a routine (night shift) on a branch cut before 2.1 cannot run its own tools/gate.mjs (differs from main) and the night handoff switch is refused (branch gate files differ from main's: CLAUDE.md Never/NIGHT_SHIFT step 4) until that branch merges main. Matters for PR #40, PR #46, night queue.
- B6b hit counts by command: routine-guard.test 15, settings.test 1, FIRST_PROMPTS 5, README 6, editor.html 7, index.html 4, core 3, editor.js 6, game 4, tools/README 8, battery 5, gate 24, pageshot 3, preview 4, smoke 4, village 2 = 101. settings.test.mjs:232 TESTDATA from reading, not run.
- B6b done
- B7b Q4 CONTROL RUN (2026-11-10, this agent; wrote only .gate/foundation/q4/): patch script .checkpoints/foundation/patch-builder.py (exact-string, asserts one hit each) -> diff orig vs patched: only lines 5,6,53,54,140 differ; no open() to docs/. Ran `python3 -I .gate/foundation/q4/build-tuning.q4.py` (Python 3.13.18) on a715be2's own dumps -> "tables 60 md lines 723"; `cmp` vs docs/legacy-tuning.json: IDENTICAL; vs docs/LEGACY_TUNING.md: IDENTICAL (sha256 b04b370b06a02c3a / a510e5c75ffa1e67, prefixes). git status docs/: clean. => the oracle reproduces TODAY, so a post-move mismatch can be pinned on the move.
- B7b raw R7 dumps are one line each (wc -l = 1, starts {"map":"Cabana Republic",...}) = preview.js eval stdout (console.log(JSON.stringify(r)), preview.js:108). So the head's raw dumps can be cmp'd directly with a715be2's tuning/{cabana,tri}-1.json too (stronger, earlier).
- B7b done; next B8 files rows
