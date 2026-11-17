# Part 15. The second lane (proposed 2026-11-17; it waits for Mustafa's answers)

_An addition to the build plan for foundation steps 2.4 and 2.5 and Slice 1 (revision 2, commit 68addf9). Written by
the first builder chat (session_019YNMUV9LFhASseJ9EP12kL) on Mustafa's words of 2026-11-17, 20:10 (his CHANGE 2 and
CHANGE 4; `docs/DECISIONS.md` once change-set 0 is merged). Nothing here is decided until he answers; the six
questions are at the end. Sources: the plan's Parts 1, 3.2, 7, 11 and 12; a reading of the Shamble VFX repository at
commit 92ec2dc by graphics-engineer (Opus 5.5, effort max; nothing was built or run), cited as "the digest"._

His decision, word for word: "Two chats build the game on my personal account. The first builder chat keeps Fable 5.2
as its main model. A second builder chat has Opus 5.5 as its main model, with Fable 5.2 answering its advisor tool,
and it also writes game and engine code." and "Every file has one owner: the first builder chat's plan says which
change-sets and folders belong to the second chat, and neither chat edits the other's files."

Below, **lane 1** is the first builder chat (branch `claude/wonderful-hamilton-jdmlnf`) and **lane 2** the second
(branch `claude/optimistic-albattani-mdoonz`).

## 15.1 His proposal, and where I agree

His proposal: lane 2 "takes the preview hosting check and step 2.6, the carry of my look decisions into
docs/LOOK_V2.md, the comparisons that stay (Part 3.2), and from change-set 6 on the picture track: 6 Look, 10 Body, 11
Feel, and M. You keep 0 to 5, the simulation track (7 Lag, 8 Shoot, 9 Throw) and 12."

**I agree with all of it.** The cut follows the one seam the engine already has: the simulation hands the page one
block of data per tick and a list of events, and the picture is drawn from those alone. Lane 1 owns everything up to
that hand-off; lane 2 owns what is drawn and heard from it. The two tracks then run side by side from change-set 5
on: while lane 1 builds 7, 8 and 9 (about 51 hours), lane 2 builds 6, M, 10 and 11.

Four conditions make his rule of one owner per file hold in practice. They are additions, not objections.

1. **The client package is cut by folder from its first day.** Both tracks live in `packages/client`. Change-set 2's
   plan lays its `src/` out by owner (the table in 15.2), so the hand-over after change-set 5 is a list of folders,
   not a rewrite. The two sides meet at one small interface file (what the app gives the picture each frame: the two
   newest blocks, the events, the time; what the picture offers back: start, warm up, draw, set quality). Lane 1
   owns that file; engine-architect reviews every change to it.
2. **Build files and dependencies stay with lane 1**: every `Cargo.toml` and `package.json`, both lockfiles, the Vite
   config, the Playwright config. A new dependency is a security review each time, and one lockfile cannot have two
   authors. When lane 2 needs a dependency it asks (15.3); lane 1 adds it in a small pull request of its own.
3. **The plain visuals of Shoot and Throw stay with lane 1**: tracers as plain lines, the ammo counter, health bars
   and the plain hit marker (the page's own HTML), the fake-lag switch, the bench table. They exist so that 8 and 9 can
   be judged without waiting for the picture track. Change-set 11 replaces the plain tracers and flash with lane 2's
   effects; the HTML counters stay lane 1's.
4. **A comparison is never merged and never edits a game file on main.** Lane 2 builds each one in a scratch copy
   after the change-set that gives it its subject is merged, and delivers a report with the patch as a text file
   beside it. If a comparison says the other side wins, redoing the game's code is lane 1's work.

What the split does not do: it does not shorten the work (the plan's 188 to 199 hours stay, plus reading-in for lane
2, about half a day per unit). It shortens the calendar, and it doubles the moments that need your Brave check.

## 15.2 Who owns what

**Lane 1 owns:** `crates/**` (all Rust); `packages/types`, `packages/sim-worker`, `packages/net`; in `packages/client`:
the app shell, input, the HTML overlay and counters, the development panels, and until change-set 5 is merged also
the first drawing code (the lit ground, the plain stand-in, the camera and its tuning panel); `content/` for
movement, weapons and the test scene; every build file (condition 2); `.github/workflows/gate.yml`,
`.github/workflows/toolchain.yml` and the gate's files (15.4); `docs/reports/` for its own change-sets.

**Lane 2's units.** Folder names inside `packages/client/src/` are the ones change-set 2's plan starts from;
engine-architect reviews them there.

| Unit | It owns | Needs merged first | Must not touch |
|---|---|---|---|
| The preview hosting check (running) | its test repository; in this repository only its report | nothing | anything else here |
| Step 2.6, the previews | the workflow file it adds for previews (a new file), the host's config files it adds, its runbook in `docs/` | the hosting check's report and your answer to it; change-set 2 (the first build there is to deploy). It calls lane 1's build script and copies no build step | `gate.yml`, `toolchain.yml`, the gate's files, `crates/`, `packages/` |
| The carry of your look decisions | `docs/LOOK_V2.md`, from the merge of change-set 0 on (change-set 0 changes one line of its section 8); the carry's own lines in the rendering draft | change-set 0 | `docs/research/asset-export-rules.md` until change-set 0 is merged |
| The comparisons that stay (Part 3.2's rows inside Slice 1) | one report per comparison in `docs/reports/`, with its patch file; the kept builds | PM-0002-25: change-set 3, then 5 · PM-0004-33's builds and PM-0005-16's size half: 4 · PM-0002-10 and PM-0002-11: 5 (PM-0002-10 again after 12) | `crates/` and `packages/` on main (condition 4) |
| 6 Look | from the merge of change-set 5: `render/` (the renderer's set-up, passes, materials, ink, light and shadow, quality tiers, the warm-up), `soldier/` (the code-built Conscript and both code-built guns), `camera/` with its panel, the GPU half of the measurement harness, `content/look/`, its browser tests and their pictures | 5; change-set 0 (the names of colour areas and joints); the carry; the machines named (your answer MACHINES) | lane 1's list above |
| M The first model | `assets/` (the loader, the model checks, the asset list), the model files and their entries, `docs/research/asset-export-rules.md` from the merge of change-set 0 on | 6; change-set 0; a model file | lane 1's list |
| 10 Body | `soldier/anim/` (the procedural animation port and its goldens) | 6. For recoil and the death fall also 8 (they answer to its shot and death events): its own plan says whether it waits for 8 or comes in two halves | lane 1's list |
| 11 Feel | `fx/` (the bridge from events to effects), `audio/`, the hit marker's and kill confirm's final look, `content/feel/`; the copy of the VFX runtime if that route is chosen (15.5) | 8, 9 and 10; the VFX proofs of 15.5 | lane 1's list |

**What lane 2 needs from lane 1's code, said early.** Body and Feel read fields and events that lane 1 puts in the
block (how fast and which way a soldier moves against where it faces, whether it is on the ground, the aim, each shot,
hit, death and blast with its place and direction). Lane 2 sends its list of needs before change-set 4's plan is
reviewed, so the block carries them from the start and not as later repairs.

**Gun models.** Three different things carry that name:
- *The code-built guns* (the rifle and the SMG as simple shapes from the Art lead's briefs, in the Dark material):
  lane 2, in change-set 6, with the code-built soldier. Change-set 8 needs both in the soldier's hands.
- *The gun files* (GLB): no builder chat can make them in Blender, because both run in the cloud and Blender runs on
  your PC. Your words of 2026-11-14: the models are made "by Claude through the Blender MCP on my PC from the Art
  project's brief". So the files come from the chat that has that connection (question GUN-FILES).
- *Taking a gun file into the game* (checking it against change-set 0's rules, writing its entry, committing it,
  hanging it on the hand): lane 2, in M or a small follow-up of M.
Lane 1 never reads a gun model: in the simulation a shot's start is data of its own (your words of 2026-11-17).

## 15.3 How the two chats' pull requests take turns

- **One open pull request per lane** that is ready for review; drafts may exist beside it.
- **First ready, first merged.** If both are ready and one needs the other, the needed one goes first; otherwise
  lane 1's goes first, because the picture track rests on the simulation track far more often than the reverse.
- **After every merge the other lane takes main in** (a plain merge of main into its branch), runs its checks again
  and has the advisor agent review the new head, before your go. That is today's rule for a later push.
- **No game file is shared**, so the merges can only meet in the record files: `docs/DECISIONS.md`, `docs/STATUS.md`,
  `docs/ROADMAP.md`, `docs/REPO_MAP.md`, the decision drafts with their register, `docs/SPIKE_PLAN.md`. Every
  change-set of both lanes, and of the netcode lane, writes its own lines there. They cannot have one owner
  (question RECORD-FILES).
- **Asking the other lane** for something in its files (a field in the block, a dependency, a job in CI): one
  written request, answered by a pull request of the owner (question LANE-REQUESTS). A request is never your word.

## 15.4 Every CI and workflow file has one owner

| File | Owner | Why |
|---|---|---|
| `.github/workflows/gate.yml`, `.github/workflows/toolchain.yml` | lane 1 | the edits that follow change-sets 1, 2 and 3 are written from their numbers; your answer GATE-FOLLOW: "You ask me for each gate and CI edit with its exact text" |
| `tools/gate.mjs` and the other gate files | lane 1, while the 2.3 chat is stopped | the same answer. If the 2.3 chat starts again, this line is asked again |
| the preview workflow that step 2.6 adds (a new file) | lane 2 | it is step 2.6's own file; lane 2 runs on Opus 5.5, so your words of 2026-11-12 on CI work hold there as they are |
| `.claude/`, `CLAUDE.md`, the guard | neither lane | the kit is frozen; its queued change is step 2.3's |

If a lane 2 unit needs a job in `gate.yml` or `toolchain.yml` (a picture test on both backends, the asset budget
check), it sends the need; lane 1 writes the edit and asks you with its exact text.

## 15.5 How change-set 11 (Feel) uses Shamble VFX

**Your lean holds, and I take it:** the game plays Shamble VFX's exported effects through its runtime and builds no
effect a second time. The digest's findings:

- **It fits the engine's drafts.** The runtime uses three.js only, with four materials written the way the rendering
  draft asks; one source serves WebGPU and the WebGL 2 fallback; all effects together cost at most four draw calls;
  the game passes in the seed and the time; it has Low, Medium and High.
- **It does not yet meet the marks change-set 11 must meet.** It allocates memory every frame (the mark asks for
  none); it has no caps and no pools; each of its four materials compiles at first sight unless the game's loading
  screen shows it first; its main cost is building triangles on the processor each frame, which counts against the
  page's own time and not only the graphics card's; its line widths assume 100.7 pixels per metre, the editor
  camera's scale.
- **It cannot be installed as it stands.** Every package is private and without a licence and ships as source; it
  expects three.js 0.187, so the game would pin that version (change-set 2 picks the pin); no exported folder is in
  the repository, its event names are proposals, and sound, camera shake and hit-stop are listed as "Not yet".

**What has to be proven**, each a check that passes or fails:

| # | The check | When |
|---|---|---|
| 1 | The same picture in the game as in the editor: one event at a fixed seed and time, the game's page against the VFX test page, same camera and colour set-up; the pixel difference under a bar written down before the run | a small proof unit by lane 2, right after change-set 6 is merged |
| 2 | No compile after the loading screen: the counts of shader programs unchanged after every exported event has played at every tier | the same unit |
| 3 | No allocation per frame in the runtime's calls. By the digest's reading of the code it fails today | the same unit; the fix follows (question VFX-FIX) |
| 4 | Both backends give the same picture, on real hardware | the same unit, on your laptop; again on the named machines |
| 5 | Its cost inside the frame budget on the minimum laptop at Low: graphics time within the effects row, and its processor time as its own line | rough in the proof unit; judged when the machines are named |
| 6 | The export folder loads as content: served under its hash, cached, a stale or altered file refused; a broken export fails the build | inside change-set 11 |
| 7 | Event names: every event the game sends names exactly one exported effect, and back | inside change-set 11 |
| 8 | A re-export reaches the running page within a second | inside change-set 11 |

**How the runtime's code reaches this repository** is question VFX-ROUTE. The digest weighs a copy first.

**What it changes in the sizes.** Change-set 11 no longer builds the drawing of flashes, tracers, puffs and fragments.
It builds the bridge from the game's events to the runtime, the loading, the warm-up, the caps the runtime lacks, the
reload and the tests: about 1,000 to 1,650 lines by the digest's estimate. The hit marker, the flinch, camera kick,
the shake, the sounds and the feel kit's data stay as planned. So change-set 11 stays near its 2,500 lines; I do not
lower it before the proof unit has run. New work outside it: the proof unit (about one day), reviewing the copied
runtime if that route is chosen (about 4,250 lines that nobody here writes), and the effects themselves for the
SMG, the rifle, the grenade, the bow and the shotgun, which are made in Shamble VFX and not in this repository.

**`docs/EDITOR_V2.md` section 7.** Line 116 needs the dated pointer: the entry that begins "`[LATER]` **A VFX
editor**, as a future tool, after Editor v2's map features (Mustafa, 2026-11-06)". Your words stay as written. The
pointer, after line 117, states facts only: "2026-11-17: Shamble VFX, an effects editor with a runtime made to ship
in the game and an export folder whose effects are played by event name, exists in its own repository." It can ride
lane 2's docs change-set for the look carry.

## Decisions needed

1. **LANE-SPLIT: is the split of 15.2 the one the two chats work by?**
   - Why it matters: lane 2 may not write game code in this repository before its folders are named.
   - Options: (a) as 15.2 has it: your proposal with the four conditions of 15.1; (b) your proposal without the
     conditions: I then need your word on who adds dependencies and who draws the plain tracers and counters;
     (c) another cut.
   - Recommendation: (a).
2. **RECORD-FILES: who writes the record files both lanes need?**
   - Why it matters: your rule is one owner per file, and these files (DECISIONS, STATUS, ROADMAP, the repository
     map, the decision drafts and their register, SPIKE_PLAN) get lines from every change-set of every lane.
   - Options: (a) an exception for these files: each pull request writes only its own lines, in its docs pass, after
     taking main in; (b) lane 1 writes all of them: lane 2 hands its lines over in its report and a docs pull
     request of lane 1 follows each of its merges.
   - Recommendation: (a). It is what the lanes do today, and (b) doubles the pull requests.
3. **LANE-REQUESTS: how does one chat ask the other for a change in the other's files?**
   - Why it matters: the chats cannot see each other, and you should be able to see what they ask.
   - Options: (a) one GitHub issue, "Lane requests", read by both chats at the start of each change-set; (b) you
     paste each request from one chat into the other; (c) the chats message each other directly, which you see only
     by opening them.
   - Recommendation: (a).
4. **VFX-ROUTE: how does the VFX runtime's code reach this repository?**
   - Why it matters: the game's rules ask for exact versions, one lockfile and an install that needs no network
     beyond the package cache; and whoever reviews must see the code that ships.
   - Options: (a) a read-only copy in a package of this repository, pinned to a commit of the VFX repository and
     re-copied by a script: every line is reviewed here, nothing changes there; the two can drift if someone edits
     the copy; (b) a published npm package: needs a licence, a public or paid private package and a release for every
     fix; reviewers here see a version number; (c) a git link to the VFX repository: needs that repository cut so the
     runtime stands alone; reviewers here see a commit number.
   - Recommendation: (a). It adds a sixth TypeScript package to the five the structure draft names; engine-architect
     rules on that in change-set 11's plan.
5. **VFX-FIX: who fixes what the proofs find in the runtime?**
   - Why it matters: the digest expects the allocation check to fail, and finds no caps, a fixed line-width scale and
     no way to move an effect that is already playing.
   - Options: (a) the VFX chat fixes them in its repository and the copy here is renewed: the editor and the game
     keep running the same code; (b) lane 2 fixes the copy here: quicker, but the editor then no longer shows what
     the game draws.
   - Recommendation: (a). It needs you to pass the findings to the VFX chat.
6. **GUN-FILES: which chat makes the gun files (and the soldier's) in Blender?**
   - Why it matters: neither builder chat can reach Blender; change-set M and the guns' real shapes wait for files.
   - Options: (a) the chat on your PC that has the Blender MCP makes them from the Art lead's briefs and change-set
     0's rules; lane 2 takes each file into the game; (b) a builder chat writes them by script, as the rifle
     prototype was written: possible without your PC, but a script-written file cannot prove the Blender export
     settings.
   - Recommendation: (a), with the code-built guns of change-set 6 standing in until the files exist.

## For Mustafa

1. Answer the six questions above.
2. Give lane 2 this Part 15 once you have answered it, and ask it for its list of needs from the block (15.2) before
   change-set 4's plan is reviewed.
3. If you choose VFX-FIX (a): pass the digest's findings to the VFX chat.
