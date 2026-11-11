// Builds e3-mutations.json from the 44446a5 files (read through git show; nothing in the working tree is touched).
const path = require('path'), fs = require('fs'), cp = require('child_process');
const ROOT = path.resolve(__dirname, '../..'), T = path.join(ROOT, 'tools');
const acorn = require(require.resolve('acorn', { paths: [T] }));
const show = (rev, f) => cp.execFileSync('git', ['-C', ROOT, 'show', `${rev}:${f}`], { encoding: 'utf8', maxBuffer: 64 << 20 });
const BASE = '44446a5';
const src = f => show(BASE, f);
const fnText = (f, name) => { const code = src(f), ast = acorn.parse(code, { ecmaVersion: 'latest', sourceType: 'script' });
  const n = ast.body.find(s => s.type === 'FunctionDeclaration' && s.id && s.id.name === name); if (!n) throw new Error(name);
  return code.slice(n.start, n.end); };
const G = 'shamble-game.js', C = 'shamble-core.js', E = 'shamble-editor.js', I = 'index.html', H = 'editor.html';
const DEPLOY = 'function deploy(){ started=true;';
const DEVRESET = 'function devReset(){ Prog.xp=0;';
const devReset = add => ({ file: G, find: DEVRESET, replace: 'function devReset(){ ' + add + ' Prog.xp=0;' });
const PF = "const PLAYER_FACTION='Allies';   // the player's faction: uniform + vehicle livery\n";
const PT = "const PLAYER_TEAM=(TEAMS.find(t=>t.faction===PLAYER_FACTION)||TEAMS[0]).id;   // the player's team id (D3: allies); TEAMS loads with core, so no TDZ\n";
const TAGS = '<script src="shamble-core.js"></script>\n<script src="shamble-game.js"></script>\n';
const GAMETAG = '<script src="shamble-game.js"></script>\n';
const refreshRect = 'function refreshRect(){ _cvR=renderer.domElement.getBoundingClientRect(); }\n';
const wrType = fnText(G, 'wrType'), rml = fnText(E, 'refreshMapList'), aLerp = fnText(G, '_aLerp');
const HYP2 = 'function hyp2(a,b){ return Math.sqrt(a*a+b*b); }';
const P = s => 'PREDICTED ON 44446a5: ' + s;

const M = [
 { id: 'P1', what: 'delete the top-level function refreshRect (shamble-game.js)', edits: [{ file: G, find: refreshRect, replace: '' }], expected: ['3'],
   notes: P('["3","5a","5b"]. The Phase 0 report (2026-10-13) predates check 5. refreshRect is still referenced at shamble-game.js:3275 (inside bootGame, runs at boot) and in both pages\' inline resize handlers (index.html:1263, editor.html:1266): 5b reports it not defined and 5a gets a ReferenceError at boot.') },
 { id: 'P2', what: 'bootEditor(); added at the end of bootGame (unguarded editor-only call on the player build)', edits: [{ file: G, find: 'navPath(-79,-90,-79,-70,w); }\n}\n', replace: 'navPath(-79,-90,-79,-70,w); }\n  bootEditor();\n}\n' }], expected: ['4'],
   notes: P('["4","5a"]. index.html runs bootGame, so the call throws a ReferenceError at boot (5a). 5b exempts index.html\'s editor-only names (check 4\'s job) and editor.html declares bootEditor, so 5b stays green. On editor.html bootEditor then runs twice (once inside bootGame, once from the inline script); whether that also errors there is not known (index.html fails 5a anyway).') },
 { id: 'P3', what: 'syntax error: a stray ) in the PLAYER_FACTION declaration (shamble-game.js line 26)', edits: [{ file: G, find: "const PLAYER_FACTION='Allies';   //", replace: "const PLAYER_FACTION='Allies');   //" }], expected: ['1'],
   notes: P('["1","2","3","4","5a","5b"]. A syntax error cascades: check 2 concatenation, check 3 (battery.js cannot parse the head), check 4 (not run, counted as FAIL), 5a (the game script never loads), 5b (cannot analyse). 5c and 5d stay green (core parses; the gate skips an unparsable upper file in 5c). The Phase 0 report named only check 1 (its gate had checks 1-4; it did not record what 2-4 printed).') },
 { id: 'P4', what: 'the wrType function declaration copied into shamble-core.js (before hyp2)', edits: [{ file: C, find: HYP2, replace: wrType + '\n' + HYP2 }], expected: ['3'],
   notes: P('["3","5c"]. Check 3: cross-file duplicate wrType (core, game). wrType references WR, mode and sfxTypeTick, which are game top-levels, so 5c also fails. At runtime the game\'s declaration replaces the core one, so 5a should pass.') },
 { id: 'M1', what: 'const PLAYER_FACTION moved below its first use (PLAYER_TEAM), a TDZ ReferenceError at load', edits: [{ file: G, find: PF + PT, replace: PT + PF }], expected: ['5a'], notes: '' },
 { id: 'M2', what: 'deploy() throws as its first statement', edits: [{ file: G, find: DEPLOY, replace: "function deploy(){ throw new Error('E3 mutation M2: deploy throws'); started=true;" }], expected: ['5a'], notes: '' },
 { id: 'M3', what: "player movement dead: keys['KeyW'] -> keys['KeyWW'] in the player's move code (update)", edits: [{ file: G, find: "if(keys['KeyW']){wish.x+=fwdX;wish.z+=fwdZ;}", replace: "if(keys['KeyWW']){wish.x+=fwdX;wish.z+=fwdZ;}" }], expected: ['5a'],
   notes: 'The smoke fails on its own 30 s wait for the player to move 0.1 m.' },
 { id: 'M4', what: 'deploy() opens a WebSocket', edits: [{ file: G, find: DEPLOY, replace: "function deploy(){ new window.WebSocket('ws://127.0.0.1:9/e3'); started=true;" }], expected: ['5a'],
   notes: 'Written as window.WebSocket because tools/gate-globals.json does not list WebSocket: a bare name would also fail 5b. How the 2026-10-14 session wrote it is not recorded.' },
 { id: 'M5', what: 'deploy() loads an image from a file outside the repo (file:///e3-outside-repo.png)', edits: [{ file: G, find: DEPLOY, replace: "function deploy(){ const e3img=new window.Image(); e3img.src='file:///e3-outside-repo.png'; started=true;" }], expected: ['5a'],
   notes: 'window.Image because gate-globals.json does not list Image. Relies on Chromium issuing a file:// request event for the image (smoke.mjs flags any file: request outside the root); a missing file may also log a console error, which fails 5a as well. The path is outside any build folder unless the folder is the filesystem root.' },
 { id: 'M6', what: "the whole shamble-editor.js replaced by main's version of 2026-10-14 before the editor fix (refreshPrefabList, refreshMapList, edSavePrefab nested in bootEditor)", edits: [{ file: E, from_commit: '58588648e4b65b4c77d4e18255facceb3bcfc31b' }], expected: ['5a', '5b'],
   notes: 'from_commit = 5858864 (main after PR #3, 2026-10-14 16:52 +0200; blob e37b6d48, identical at 2d30e4a, b23a133 and dbe24f8^; path shamble-editor.js at the root). dbe24f8 (cs/editor-prefab-list, merged to main by PR #4 at 20:58) hoisted the three. ' +
     P('["3","5a","5b"]. Against base 44446a5 the old file drops 7 editor top-level symbols (_edCanRot, _edToolKind, edFlashHint, edSavePrefab, ovHostileSpawn, refreshMapList, refreshPrefabList; battery.analyze in memory), so check 3 FAILS too. The 2026-10-14 run saw only 5a+5b because its default base was the merge-base with main (2d30e4a/5858864), where the three were still nested. 5b should list exactly the three names. The old file is weeks older than today\'s game: other 5a errors beyond the three ReferenceErrors are possible.') },
 { id: 'M7', what: 'refreshMapList re-nested: its top-level declaration moved into the start of bootEditor()', edits: [{ file: E, find: rml + '\n', replace: '' }, { file: E, find: 'function bootEditor(){\n', replace: 'function bootEditor(){\n' + rml + '\n' }], expected: ['5a', '5b'],
   notes: 'Edits apply in order (remove, then insert). ' + P('["3","5a","5b"]. Check 3 reports refreshMapList DROPPED against base 44446a5 (battery counts top-level declarations only); 2026-10-14\'s base had it nested, hence no check 3 then. 5b: openMaps (shamble-editor.js:702) references it; 5a: the Maps list never fills.') },
 { id: 'M8', what: "index.html: core and game script tags swapped (game loads before core)", edits: [{ file: I, find: TAGS, replace: '<script src="shamble-game.js"></script>\n<script src="shamble-core.js"></script>\n' }], expected: ['5a', '5d'],
   notes: 'Applied to index.html only; the report does not say which page. The game throws at load (TEAMS, a core const, is read at shamble-game.js:27).' },
 { id: 'M9', what: 'index.html: defer on the game script tag', edits: [{ file: I, find: GAMETAG, replace: '<script src="shamble-game.js" defer></script>\n' }], expected: ['5a', '5d'],
   notes: 'index.html only; the report does not say which page. The inline bootGame(); runs before the deferred game script.' },
 { id: 'M10', what: 'index.html: an onclick handler calling an undefined name (on the dev panel FPS readout)', edits: [{ file: I, find: '<span id="dvFps">', replace: '<span id="dvFps" onclick="e3NoSuchHandler()">' }], expected: ['5b'], notes: 'The smoke never clicks #dvFps. Not check 4: the name is not an editor-only symbol.' },
 { id: 'M11', what: 'editor.html: an onclick handler with a typo (edUndoDoo for edUndoDo) on the Undo button', edits: [{ file: H, find: 'id="edUndo" data-ic="undo"', replace: 'id="edUndo" onclick="edUndoDoo()" data-ic="undo"' }], expected: ['5b'], notes: 'The smoke never clicks #edUndo.' },
 { id: 'M12', what: 'typo in code the smoke never runs: devReset calls updateCareerChp (for updateCareerChip)', edits: [{ file: G, find: DEVRESET + 'Prog.level=1;Prog.prestige=0;Prog.kills=0;Prog.deaths=0;Prog.headshots=0;Prog.weapon={}; updateCareerChip();', replace: DEVRESET + 'Prog.level=1;Prog.prestige=0;Prog.kills=0;Prog.deaths=0;Prog.headshots=0;Prog.weapon={}; updateCareerChp();' }], expected: ['5b'],
   notes: 'devReset runs only from the dev panel\'s Reset button (shamble-game.js:3351), which the smoke never clicks.' },
 { id: 'M13', what: 'implicit global: devReset assigns an undeclared name (e3ImplicitGlobal=1)', edits: [devReset('e3ImplicitGlobal=1;')], expected: ['5b'], notes: 'In devReset (never run by the smoke); the files are sloppy-mode scripts, so it would not throw even if run.' },
 { id: 'M14', what: 'bare localStorage in devReset (persistence must go through KVStore)', edits: [devReset("localStorage.removeItem('e3');")], expected: ['5b'], notes: 'localStorage is not in gate-globals.json; devReset never runs in the smoke.' },
 { id: 'M15', what: 'bare status (a window property) read in devReset', edits: [devReset('void status;')], expected: ['5b'], notes: 'status is deliberately not in gate-globals.json.' },
 { id: 'M16', what: 'THREE.CapsuleGeometry referenced in devReset (not in three r128)', edits: [devReset('void THREE.CapsuleGeometry;')], expected: ['5b'], notes: 'Referenced, not called, and in code the smoke never runs.' },
 { id: 'M17', what: 'assignment to a const of the same script: devReset assigns PLAYER_FACTION (a shamble-game.js const)', edits: [devReset("PLAYER_FACTION='Axis';")], expected: ['5b'], notes: '' },
 { id: 'M18', what: 'assignment to a const from another script: devReset (game) assigns GROUND_W (a shamble-core.js const)', edits: [devReset('GROUND_W=1;')], expected: ['5b'], notes: 'game -> core reference, so not a layering (5c) case.' },
 { id: 'M19', what: 'index.html: unquoted handler onclick=noSuchFn() (on the dev panel triangle readout)', edits: [{ file: I, find: '<span id="dvTris">', replace: '<span id="dvTris" onclick=noSuchFn()>' }], expected: ['5b', '5d'],
   notes: 'The report lists M19 and M20 together under "5b + 5d"; it does not say which check each one fails. ' + P('["5b"] for this one: an unquoted handler on a <span> is a 5b case only; nothing in 5d reads handlers.') },
 { id: 'M20', what: 'index.html: an extra unquoted <script src=shamble-editor.js></script> after the game script', edits: [{ file: I, find: GAMETAG, replace: GAMETAG + '<script src=shamble-editor.js></script>\n' }], expected: ['5b', '5d'],
   notes: 'The report lists M19 and M20 together under "5b + 5d" and does not say where the tag went. ' + P('["5d"] for this one: the editor\'s names all resolve in index.html\'s build (0 unresolved by eslint-scope, e3-analyze.cjs), and the pages share their DOM, so 5b and 5a are expected to stay green. With the tag left unclosed it would swallow the inline script instead (a different mutation); this entry closes it.') },
 { id: 'M21', what: 'shamble-core.js calls a game function: makeInputCommand calls deploy()', edits: [{ file: C, find: 'function makeInputCommand(seq,tick){ return {', replace: 'function makeInputCommand(seq,tick){ if(seq<0)deploy(); return {' }], expected: ['5c'],
   notes: 'makeInputCommand is declared and never called anywhere (one occurrence in the repo), so nothing runs. 5b resolves deploy in the page build.' },
 { id: 'M22', what: 'index.html: an extra <script type="text/ecmascript" src="shamble-editor.js"></script> (a legacy JavaScript MIME type) after the game script', edits: [{ file: I, find: GAMETAG, replace: GAMETAG + '<script type="text/ecmascript" src="shamble-editor.js"></script>\n' }], expected: ['5d'],
   notes: 'Read as "the editor script tag, with a legacy JS type, added to index.html". battery.scriptKind treats text/ecmascript as classic JavaScript, so 5d sees an extra script. Same runtime effect as M20 (only the tag form differs).' },
 { id: 'C1', what: 'control: valid inline handlers on index.html (double and single quotes, &amp; &quot; &#39; entities, event, return)', edits: [{ file: I, find: '<span id="dvCalls">', replace: '<span id="dvCalls" onclick="if(event&amp;&amp;mode===&quot;menu&quot;)return false;" ondblclick=\'return typeof deploy===&#39;function&#39;&amp;&amp;!!event\'>' }], expected: [],
   notes: 'index.html only (check 4 reads index.html handlers, 5b reads both pages). mode and deploy are game top-levels. The smoke never clicks #dvCalls.' },
 { id: 'C2', what: 'control: a commented-out editor script tag, an on… variable in the inline script, and the inline script closed with </script > (index.html)', edits: [
     { file: I, find: '<script src="https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js"></script>\n', replace: '<!-- <script src="shamble-editor.js"></script> -->\n<script src="https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js"></script>\n' },
     { file: I, find: "<script>\nbootGame();\n", replace: "<script>\nvar onE3Probe=\"e3NoSuchFn()\";\nbootGame();\n" },
     { file: I, find: '</script>\n</body>', replace: '</script >\n</body>' }], expected: [],
   notes: 'onE3Probe is a new top-level var of index.html\'s inline script: check 3 should print it as +added (not a failure). Its string value looks like a handler to a naive scanner.' },
 { id: 'C3', what: 'control: the Deploy button (#mPlay) animated endlessly (a CSS pulse in index.html)', edits: [{ file: I, find: '</head>', replace: '<style>#mPlay{animation:e3pulse .6s ease-in-out infinite}@keyframes e3pulse{0%,100%{transform:scale(1)}50%{transform:scale(1.06)}}</style>\n</head>' }], expected: [],
   notes: 'If an existing rule on #mPlay is more specific, the pulse could be overridden; check the shot or computed style if C3 passes suspiciously.' },
 { id: 'PC1', what: 'control: P1 (refreshRect deleted) with --allow-drop refreshRect', edits: [{ file: G, find: refreshRect, replace: '' }], flags: ['--allow-drop', 'refreshRect'], expected: [],
   notes: P('["5a","5b"], not a PASS. The flag only turns check 3\'s drop into "allowed"; refreshRect is still referenced (shamble-game.js:3275 inside bootGame, index.html:1263, editor.html:1266), so 5b and 5a fail. The Phase 0 report\'s PASS was on checks 1-4 only. The control holds if check 3 passes with the drop listed as allowed; the plan\'s "every control passes in both" cannot hold for PC1 as written.') },
 { id: 'PC2', what: 'control: pure move of _aLerp from shamble-game.js to shamble-core.js (placed before hyp2)', edits: [{ file: G, find: aLerp + '\n', replace: '' }, { file: C, find: HYP2, replace: aLerp + '\n' + HYP2 }], expected: [],
   notes: '_aLerp(a,b,t) uses only its parameters, so core references nothing above it (5c) and the player build resolves it. Check 3 should list "~ moved: _aLerp (game -> core)".' },
];
fs.writeFileSync(path.join(__dirname, 'e3-mutations.json'), JSON.stringify(M, null, 1) + '\n');
console.log('wrote', M.length, 'entries');
