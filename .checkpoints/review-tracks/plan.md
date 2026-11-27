# Plan: review tracks and who merges (risk tier 2: rule files)

Drafted 2026-11-26 in the evening (Amsterdam); revised the same night with both plan reviews folded in (security-auditor: SA-RT-1 to SA-RT-12; the advisor agent: H1-H4, M1-M10, L1-L5; where each landed: the section near the end). Branch claude/confident-gates-tbnq38 (stands in for cs/review-tracks). Base: main 2671595f. Updated later the same night (2026-11-26) for his answers in the Guard fix chat: PLAN-GO (a), MERGE-PROMPT (d), WAKE-END (a) (section "Decisions").

Mustafa has given PLAN-GO (a). Both plan reviewers approved the earlier design, MERGE-PROMPT (c); (d) changes the guard's part, so the build starts only after security-auditor and the advisor agent have read this update. Today's rules hold until it merges. Until the guard change-set merges, git and gh always start their own command.

## 1. Goal and his words

The goal is a lighter review path, worked out by script, for most changes. A PR that shows nothing on screen should merge without waiting for him. His go stays where EYES-KEEP (a) and SOURCE-GO (a) keep it, and where other words of his already keep it (section 5, item 13). Where his go is needed, it reaches the merge through his Advisor's PASS or through his go in a chat (MERGE-PROMPT (d)).

His words. They are recorded whole in docs/DECISIONS.md, at :124-158 (the brief) and :115 (SOURCE-GO). They are quoted once here and pointed to everywhere else.

His paste, item 1:
> "Light review track by default. Exceptions that always get the full checks: netcode, the Rust core, server code, anything anti-cheat."
> "The rule lives in the repo files, not in a label someone picks."
> "Less process on small changes, so I see visible progress sooner."
> "My eyes only where it shows: I only check pull requests that change what I see on screen. Everything else merges on green checks plus your review; V4 merges its own."
> What I mean by it: "your review" is my Advisor's merge check. V4 is Builder V4, lane 1's builder chat, and the same holds for every builder chat. "What I see on screen" is today's "anything a player sees, hears or feels".

SOURCE-GO (a), his item 4 of 2026-11-26, 15:05 Amsterdam:
> Then the review tracks, which also get one more line: a change under content/source/ needs my go (SOURCE-GO (a)).

The order comes from his SPEED-ORDER (b) (DECISIONS:183-184): this change-set comes before the guard change-set. His words on speed and on manual tasks are at DECISIONS:195 and :208. Section 5 points to those, and to WHICH-ACCOUNT.

Recording these words: the change-set copies his brief into DECISIONS by script, word for word. DECISIONS already holds it whole, from #73, so the docs pass adds only a pointer. His answers to this plan, as he typed them in the Guard fix chat ("PLAN-GO: a / MERGE-PROMPT: A / WAKE-END: a", then "d" after one re-ask), are copied the same way, with the text of option (d) as it was offered to him, labelled as the offered text, not his words. His other words this plan leans on: TOOL-SOURCE's general rule (DECISIONS:1348) and its one-file answer (:2220), WATCH-70 (:430), the merge on his go (:432), his effort words of 12 November (:3160).

## 2. The timings: his numbers beside the re-measured ones (hours)

| PR | Building, his / ours | Checks, his / ours | Waiting, his / ours | Whole branch, his / ours |
|---|---|---|---|---|
| #65 (stop 3a) | 2.4 / 2.33 | 4.0 / 3.84 | 4.9 / 4.91 | 11.3 / 11.26 |
| #68 (stop 3b) | 1.1 / 1.05 | 4.9 / 3.64 | 9.2 / 9.57 | 15.2 / 15.12 |
| #69 (guard fix) | 0.3 / 0.28 | 1.0 / 0.75 | 1.2 / 1.18 | 2.5 / 2.28 |

**Method** (from inputs-A, Part 1):
- Times come from merge commits and branch commits (`git log <merge>^1..<merge>^2`, converted to UTC) and from GitHub's `merged_at`. The PR timelines carry no ask times.
- Building runs from the first code commit to the last planned build step.
- Checks run from there to the last commit before the head went to him.
- Waiting runs from that commit to the merge.

**Where they differ:**
- The whole-branch spans agree within rounding.
- #68's split points match no commit. His checks also contain build steps 4 to 6.
- For #69, his three cells add up to 2.5, which is more than the whole branch (2.28).

**Where the waiting went:**
- #65 and #68: most of it was waiting for his browser check (4.04 h and 7.26 h).
- After his answer, a new head had to be read again and given his go: 0.9 h and 1.7 h.
- #69: the whole wait was the re-review plus his go. That is the part proposal 3e shortens: his go now reaches the merge through his Advisor's PASS, with no tap. The advisor agent's review of a new head stays.

## 3. Proposals 3a-g

Every changed sentence is shown as OLD (the exact current text, with file:line) and NEW.

### 3a. The track, worked out by a script

**The script:** `.claude/scripts/track.mjs`. Its tests are in `.claude/scripts/track.test.mjs`. Both are rule files, so both are tier 2. Both sit next to checkpoint.mjs.

**Which copy runs (SA-RT-5):** main's copy always decides. A PR's own copy never classifies that PR. The chat and the advisor agent run it from a worktree of main (`git worktree add .gate/main-rules origin/main`, then `node .gate/main-rules/.claude/scripts/track.mjs <head>`). This change-set itself, which brings the script, is classed by hand: full and risk tier 2 (rule files).

**Inputs:**
- `node <main's copy> <head>`. The base is `git merge-base origin/main <head>`. With no merge base (this clone is shallow) the script exits 2 and says to fetch more history. An empty diff also exits 2. It never guesses a base and never falls back to light.
- `--files <path>...` at plan time, with the planned file list.
- Changed paths and modes come from `git diff --raw -z --no-renames` (SA-RT-6, M7), so a renamed file shows as a delete plus an add and both names count. Lines come from `--numstat`; a binary file shows `-`.

**Full, with the risk-tier-2 floor, whatever the folder (SA-RT-6, M7; advisor.md:78-86 already reads these as tier 2):**
- a path `.gitignore` ignores (`git check-ignore --no-index`);
- a symlink (mode 120000) or a submodule (160000);
- two paths in the head's tree that differ only in letter case;
- a name with a character outside the guard's allowed set (`A-Z a-z 0-9 _ . / -`, routine-guard.mjs:85);
- a binary file.

**How names compare:** full rules compare in lower case (as the guard's allow-list does), so `Claude.md` and `.Claude/` are full. Light rules compare exactly, so `Docs/x.md` is not light; it falls to "in no list", which is full (SA-RT-12).

**Output:** one JSON object with keys in a fixed order:
- `base` and `head`: the two full SHAs it compared (M7), so the chat, the advisor agent and the routine compare like with like while origin/main moves.
- `track`: light or full.
- `why`: one line per path, with the rule that matched it.
- `reviewers`: the light reviewer (plus security-auditor when a light security trigger fires), or the full picks with their scores.
- `goFloors`: paths and entries that keep his go.
- `forMergeCheck`: items his Advisor's merge check must look at (every added DECISIONS entry, other quotes of him, the TOOL-SOURCE prompt).
- `eyesPaths`: paths a player may see, hear or feel.
- `toolSource`: changed lockfiles, manifests, toolchain pins and workflow download addresses (L2), printed as a judged TOOL-SOURCE prompt, never a pass.
- `wordsRecord`: the result of the copied-words check.

The script is deterministic: git only, no clock, no network, paths sorted. One ordered path table drives everything: the track, the go floors, the eyes paths, the reviewer owners and the security slot.

**Who runs it:**
1. The building chat, at plan time and on every head it asks to have reviewed. It pastes the output into the plan and the report.
2. The advisor agent re-runs it at the plan review and at the head review. A mismatch is a finding.
3. The PR Advisor routine cannot run any program: the guard refuses it at routine-guard.mjs:1072. It reads main's path table as a cross-check only; it never sets the track. A mismatch is a finding (PRADV-TRACK, settled: a technical choice inside his item 3a).

**Tests** (`node --test`):
- About 45 path-to-track cases: every exception, every "in doubt" row, every package.json and CI-deciding file, a path in no list, a dotfile, case variants (`Claude.md`, `.Claude/`, `Docs/`), `content/source/` and `docs/adr/`.
- The tier-2 floor cases: a gitignored path, a symlink, a submodule, a case-only twin, a name with a space, a binary file.
- The reviewer picks reproduced from recorded numstat fixtures of #65 and #68; they must equal inputs-B's table. The light pick, its fallback, and the light security triggers.
- The words-record check on 3 real entries (SPEED-ORDER "B"; the 17:31:41 multi-line entry; the 17:27:17 entry), one tampered copy, one entry whose saved file differs, one whose named commit cannot be read.
- No merge base, and an empty diff, give exit 2.
- Shuffled input gives the same output.

### 3b. LIGHT-DEF (a) mapped onto today's steps (his words: DECISIONS:141)

| Today's step (CLAUDE.md) | Light track |
|---|---|
| 2: plan, and the advisor agent's plan review | stays |
| 4: gate after every step; CI | stays |
| 5: reviewers on the final diff | one round by one reviewer, named by rule, at the repository's default effort (`high`, CLAUDE.md:174) |
| Review budget | a second round only for an open Critical or High; otherwise findings become STATUS follow-ups |
| Verifier until PASS | stays, on the final result, at max (LIGHT-EFFORT (a), a default inside PLAN-GO: his item 3b sets the default effort for light, DECISIONS:141) |
| Advisor agent's review of every pushed head | stays ("not rounds, never cut", CLAUDE.md:112-113; 3e also needs it) |
| 6: report; docs-keeper | a short report (what, evidence, findings, Decisions needed, For Mustafa); STATUS and ROADMAP still updated |

**"The one reviewer that fits", by rule:** the reviewer who owns the most changed lines, using inputs-B's owner table R2 plus three rows (`tools/`, `legacy/`, `packages/e2e/` go to test-engineer). When no owner row matches, the fallback is test-engineer (L3). Cases that differ:
- If the plan's change type is visual or UI, the reviewer is art-director (CLAUDE.md step 5).
- A risk-tier-0 PR gets no review round (A4's tier 0: CI and the advisor) (M5). Any other docs PR (an ADR draft, a research topic, a decision doc) keeps LIGHT-DEF's one round by the reviewer that fits (the fallback when no row matches); research also keeps the verifier's spot-check (CLAUDE.md step 4).
- **Light security triggers (SA-RT-8, L4):** security-auditor joins the round (a second reviewer) when the diff adds any of a fixed list of risky calls under `packages/`, `tools/` or `legacy/` (SA-RT2-4) (page markup written from strings, code run from strings, browser storage, new network calls, cross-window messages: `innerHTML`, `outerHTML`, `insertAdjacentHTML`, `document.write`, `eval`, `new Function`, `localStorage`, `sessionStorage`, `indexedDB`, `fetch`, `WebSocket`, `WebTransport`, `XMLHttpRequest`, `postMessage`), or changes `packages/client/index.html` (the page's content security policy). This keeps step 5's subjects (storage, dynamic DOM, network) on the light track. The full track's security slot (R3) fires on the same list (inputs-B Part 4.4).

### 3c. The five full-track exceptions as path rules

| Exception | Paths, today and planned |
|---|---|
| Netcode | `crates/protocol/`, `crates/room/`, `crates/client-endpoint/`, `crates/sim-wasm/` (bindings around the client endpoint and the local server), `packages/net/`, `packages/sim-worker/` (runs the local match) |
| Rust core | all of `crates/` (today: sim, devtools, protocol, room, client-endpoint, sim-wasm; later: content, server, supervisor, navbake), plus `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, and any `*.rs` file anywhere |
| Server code | `crates/server/`, `crates/supervisor/` (slice 3), and the lobby crate when ADR 0006 places it (already full as part of `crates/`) |
| Anti-cheat | Nothing has the name yet. Today it is the room's command checks (`crates/room/`) and the decoders (`crates/protocol/`); later the server's checks (`crates/server/`). Also any path segment matching `anti-?cheat`, case-insensitive. |
| Rule files | `CLAUDE.md`, `CLAUDE.local.md` and `AGENTS.md` at any depth; any `.claude/` folder; `.github/`; the gate files (`tools/gate.mjs`, `smoke.mjs`, `battery.js`, `gate-globals.json`, `tools/package.json`, `tools/package-lock.json`, `tools/npm-shrinkwrap.json`, any `.npmrc`); the guard (inside `.claude/`) |

**"In doubt" choices.** These are not new exceptions: his item 3a says "In doubt it is the full track" (DECISIONS:140), so they are a default inside PLAN-GO (TRACK-DOUBT (a)).
- **Always full:**
  - `packages/types/` (types generated from the Rust boundary).
  - `packages/client/scripts/`, `vite.config.mjs`, `serve.mjs` and its test. These are the serving and build setup, the class of 3A-H3.
  - Every `package.json` and every lockfile at any depth, the root's included (SA-RT-7). DEP-HOLD still brings security-auditor (CHECKPOINTS.md:60-69).
  - Every file that decides what CI runs or how it passes (SA-RT-7): `playwright.config.*`, `tsconfig*.json`, `vite.config.*` and `vitest.config.*` at any depth. A light PR could otherwise make toolchain green by construction. (`toolchain` is also not yet a required check in "Shamble Ruleset 1": a follow-up click for him, For Mustafa.)
  - Any other dotfile, and `spikes/`.
- **Light, by a fixed list (exact names):** `docs/`, `README.md`, `FIRST_PROMPTS.md`, `tools/` without the gate files, `legacy/`, `packages/client/` (except the setup and CI files above), `packages/e2e/` (same exception), and `packages/editor/` once it lands.
- **Everything else:** any path in no list, including a folder created later, is full.

### 3d. CORE-REVIEWERS (a): what full means for the Rust core, netcode, server code and anti-cheat (from inputs-B)

**The rule:**
- test-engineer is always in, plus at most two more.
- R3 gives the first slot to security-auditor when any changed path is under protocol/, room/, server/, supervisor/, packages/net/ or the client's setup files, or when a light security trigger fires (3b).
- The rest go to the reviewers who own the most changed lines (owner table R2).
- Lines are counted with numstat. Manifests are left out: DEP-HOLD covers them.
- A tie goes by a fixed order. An empty slot stays empty.
- The same reviewers read round 2.
- Everyone runs at max: his words of 12 November, DECISIONS:3160 ("The agents that review, verify or design game and engine code always run at max").
- For rule-file PRs, the reviewers are tier 2's: the advisor agent plus security-auditor (R5).

**Highs found by review rounds (kind R), stops 3a and 3b:**

| High | Found then by | Under the rule | Without R3 |
|---|---|---|---|
| 3A-H1 F-MULADD | test-engineer | found | found |
| 3A-H2 F-FEATURES | test-engineer | found | found |
| 3A-H3 DEV-SERVER | security-auditor | found (R3) | missed |
| 3B-H1 BITE-WHOLE | test-engineer | found | found |
| 3B-HC2, 3B-HC3 (candidates) | test-engineer | found | found |

The other 7 Highs, from plan and head reviews, were the advisor agent's, which the full track keeps. Total: 11 Highs plus 2 candidates found, 0 missed (inputs-B's count; L1). inputs-B is honest about how little this proves: only 3A-H3 separates the two rules.

**A default inside PLAN-GO (REVIEWER-SLOTS (a)):** a protocol PR gets two of netcode-architect, security-auditor and rust-reviewer, never all three. The default is netcode-architect and security-auditor, as inputs-B's rule gives, because the room's checks are the anti-cheat line. His item 3d leaves the rule to the plan (DECISIONS:143).

**The full track also keeps:** the advisor agent's plan and head reviews, the verifier, and a third round for an open Critical or High (CLAUDE.md:110).

### 3e. Who merges

**Way (1), on his Advisor's PASS (MERGE-PROMPT (d)).** For every PR, whatever its track. The chat that built the PR merges it when all of these hold for the same full id:
- gate, smoke and toolchain are green;
- the advisor agent approved that id **in this session** (SA-RT-11: a PR review comment does not stand in), and the chat posted that verdict on the PR as one line naming the full id, what was run, and what in "His go keeps" (3f) it found, including any eyes path without its proof of no change (H4), so his Advisor and the after-check can find it. This also covers every "verify in a day session" trigger: the in-session advisor agent runs the gate, sims and renders itself (advisor.md, PR review step 2). The guard cannot see this condition; the chat keeps it;
- his Advisor's merge check is in: a comment by `Mergincanli` (user type `User`) whose first line is exactly `Advisor merge check: PASS #<pr> <full id>` (SA-RT-12: the PR number too), never edited, posted after the gate run on that id completed, found by reading the PR's whole comment list across pages and matched exactly, with no trimming or folding (SA-RT2-3);
- the merge is made by the chat itself, never by a subagent;
- the id is passed as the expected head.

**What PASS means.** PASS says that everything his rules need for this head has happened. His Advisor writes it only when nothing in "His go keeps" (3f) applies, or after Mustafa has given what his rules need for that head: his Brave check for anything a player sees, hears or feels; his yes for rule files, a decision of his, `content/source/`, cost or anything irreversible, a new download (TOOL-SOURCE) and a passed stop line. So for a PR that keeps his go, his go reaches the merge through his Advisor's PASS (his decision MERGE-PROMPT (d)). While a PR waits for him, his Advisor may post `Advisor merge check: FOR HIS GO #<pr> <full id>` as a status line, so the waiting chat knows why; the guard and way (1) never accept it, and it is never a go (this keeps C1's line as a status only: C1's own concern is settled by his decision, see "Plan reviews").

A later push needs all of them again. Way (1) covers allow-listed `ready` PRs too: whichever merges first, the chat or the routine (M3).

**Way (2), his go in a chat, stays.** Any PR, after the advisor approved that head (in this session, or by its PR review comment from `Mergincanli` naming the full id), on Mustafa's own go in that session's chat for that PR and head. No PASS is needed, so the guard stays silent and the merge meets the normal permission prompt, where his go is a tap.

**The PR Advisor's own merges (H1), unchanged by (d).** "One gate for every PR" means every merge a day session makes; the routine's own merges stay as they are. It still merges risk-tier-0 and allow-listed `ready` PRs without the merge check (A4's auto-merge), but only when its comment lists nothing "For the merge check". Any listed item (a major change, an accepted regression, a security Low left open, a headless doubt, a "verify in a day session" trigger, a DECISIONS entry, a quote of him) means the PR waits for the merge check, and the building chat merges it under way (1). For such a PR the routine still gives its verdict, labels it `ready-to-merge` and does not merge.

**What the guard enforces today** (routine-guard.mjs:1555-1593, every session):
- the expected head is a full id and equals the PR's head;
- not a fork;
- gate and smoke are completed and green;
- for the PR Advisor only, the allow-list.

**Toolchain in the guard** (GUARD-TOOLCHAIN (a), a default inside PLAN-GO: his item 3e names "gate, smoke and toolchain green", DECISIONS:144):
- OLD (routine-guard.mjs:1568): `for (const nm of ['gate', 'smoke']) {`
- NEW: `for (const nm of ['gate', 'smoke', 'toolchain']) {`
- The test fixture at routine-guard.test.mjs:45 gains a toolchain row, plus two new cases (no toolchain; toolchain failed).
- Caveat: #76's head has a cancelled toolchain run beside a green one. The loop's rule, applied to toolchain, refuses it: every run must be success, skipped or neutral, and a cancelled run is none of those. Such a merge is refused until the run is re-run.

**GUARD-MARK, new in the guard (SA-RT-1, H2).** In `mcp()` (routine-guard.mjs:1501-1517), before the day-session pass-through at :1514, for every session and routine:
1. Any GitHub write (any GitHub tool not on the guard's read list) whose input text (every string value of the input, read recursively: SA-RT2-3), in lower case with all whitespace removed, contains `advisormergecheck:pass` is refused. Every line way (1) accepts matches that test, so nothing the merge path accepts can be written through the GitHub tools.
2. An edit of a comment (`update_issue_comment`) whose current text carries it is refused. If that comment cannot be read, the edit is refused.
3. A merge by a subagent (hook input with `agent_id` set) is refused, in every session.
- `mcp()` gains the hook input (today it gets only tool, input, role and io), so it can see `agent_id`.
- The guard's refusal messages name the rule and never repeat the line.

What GUARD-MARK does not cover: the Bash route. `gh pr comment` is on neither settings list, so it meets a permission prompt (in Auto mode, the classifier instead of his tap); `gh api` with write flags is denied (settings.json:229-236). While R4-1 is open, a disguised Bash form is also met by a prompt, not a refusal. His Advisor chat runs outside this repository's sessions, so the guard never touches its posts.

**The merge prompt: MERGE-PROMPT (d), his decision of 2026-11-26.** No allow-list line for the merge tool. In a day session, after the base merge check above passes, the guard returns `allow` (skips the permission prompt) only when all four of these read true from the API:
1. the merge's expected head is the PR's current full head (the approved version);
2. gate, smoke and toolchain are green on that head (the base check);
3. the caller is not a subagent: hook input with `agent_id` set is refused outright (GUARD-MARK item 3);
4. a real PASS comment is on the PR: by `Mergincanli`, user type `User`, first line exactly `Advisor merge check: PASS #<pr> <full sha>`, never edited (`updated_at` equals `created_at`), and `created_at` later than the `completed_at` of the gate run on that SHA. The comment list is read whole across pages, and the line is matched exactly, with no trimming and no Unicode or case folding (SA-RT2-3).

There is **no file-list check**: the guard does not read the PR's changed files or `track.mjs`'s table for the prompt skip. His Advisor's PASS is the one gate for whether his go is needed and has been given (3e, "What PASS means").

Otherwise the guard stays silent, and the normal prompt appears, where his go is a tap (way (2)). How it fails:
- a throw or a failed read in these new conditions ends in silence, so in the prompt;
- a throw in the existing base merge check stays a refusal (:1508);
- a guard that fails to load prints nothing (guard.mjs:40-48), and with no allow line that is the prompt, never a merge.

`allow` skips only the prompt; deny and ask rules are still applied (Claude Code's hooks page, PreToolUse decision control). `guard.mjs` gains an allow output: today it prints only deny or ask (:40-48).

**The guard reads no table.** Under (d), `track.mjs` is used by the building chat, the advisor agent and his Advisor only; nothing in the guard reads it. So SA-RT2-1's table-drift condition and the advisor agent's C3 (both: allow only when the disk copy of `track.mjs` equals main's) are dropped as moot. SA-RT2-1's other half (allow only when the guard's own file on disk equals main's) is not about the table; it is kept as a build Low for security-auditor to settle at the build review (a stale lane branch's guard is the case it covers).

**What the guard's allow cannot see**, all of which his Advisor's PASS must cover:
- the in-session APPROVE and what it found;
- every item of "His go keeps", by path or by judgement: rule files and other risk-tier-2 paths, `content/source/`, eyes paths and any other visible change, a decision of his set or changed, a TOOL-SOURCE item, a stop line passed, open findings, forks;
- the floors that come from file contents: a DECISIONS entry that fails the hash or saved-file compare, a removed quote, a quote that sets a decision, an added entry the check comment does not name;
- whether he has given his Brave check or his yes for that head.

His Advisor issues PASS only when those hold, reading `track.mjs`'s output (its go floors and eyes paths are printed there) and the chat's posted APPROVE line.

**Why (d), not (c):** option (c) also made the guard check every changed file (a protected-path test, then the table) and stay silent on any file that needs his go. He chose (d) without that check, so a PR that needs his go can merge without a tap once his Advisor's PASS says his go was given. What that costs is in the risks (section 7, risk 1).

**Why not an allow-list line (MERGE-PROMPT (a)), his first answer, re-asked once:** it removes the prompt for way (2) merges and subagent merges too, and a guard that fails to load in a day session then lets the merge through with no check at all (SA-RT-4). Under (d), a broken guard ends in the prompt.

**Later, MERGE-MARK (b):** the guard also requires the check comment to carry no app mark (`performed_via_github_app` empty), once T-MARK (4a) shows that the mark holds.

**CLAUDE.md:127-133**

OLD:
> A session merges only on Mustafa's own go in its chat, about that PR and for the PR's current head, given after the advisor approved that head (its verdict in this session, or its PR review comment posted by the `Mergincanli` account and naming that full SHA), with the gate and smoke checks green, and passes that SHA as the merge's expected head (the night shift never merges); a later push needs the advisor's review first, then his go again; every other merge is the PR Advisor's.

NEW:
> A session merges a PR for its current head only, with gate, smoke and toolchain green, passing that SHA as the merge's expected head, in one of two ways: (1) the chat that built it, after the advisor approved that head in this session (a PR review comment does not stand in) and the chat posted that verdict on the PR naming the full SHA and what in advisor.md's "His go keeps" it found, once his Advisor's merge check is in: a comment by `Mergincanli` whose first line reads exactly `Advisor merge check: PASS #<PR> <full SHA>`, never edited and posted after the gate run on that SHA, re-read through the API right before the merge. His Advisor writes PASS only when nothing in "His go keeps" applies, or once Mustafa has given what it needs for that head (his Brave check for anything a player sees, hears or feels; his yes for the rest), so his go reaches the merge through that PASS (Mustafa's MERGE-PROMPT (d), 2026-11-26); its status line `Advisor merge check: FOR HIS GO #<PR> <full SHA>` is never a go. (2) Any PR after the advisor approved that head (its verdict in this session, or its PR review comment posted by the `Mergincanli` account and naming that full SHA), on Mustafa's own go in its chat, about that PR and that head. A subagent never merges, and the night shift never merges; a later push needs all of it again; every other merge is the PR Advisor's.

**The rest of that paragraph (CLAUDE.md:132-133)**

OLD:
> so a "merge" written on GitHub never counts as his go;

NEW:
> so a "merge" written on GitHub never counts as his go; his Advisor's PASS comment counts only as way (1) reads it, as his Advisor's statement that what his rules need for that head has happened, never as his own word: only his Advisor or he writes it; a session that writes it, or text that imitates it, gets around the merge check (a Critical), and the guard refuses its line in every GitHub tool write;

### 3f. EYES-KEEP (a) and SOURCE-GO (a): what keeps his go

EYES-KEEP (a) is his words at DECISIONS:145. A new section goes into advisor.md, after "Today's classes, mapped" (about 14 lines):

> **His go keeps** (EYES-KEEP (a), SOURCE-GO (a); his words in `docs/DECISIONS.md`). Whatever its track, a PR needs his go when it: (1) changes anything a player sees, hears or feels (his Brave check first; identical renders prove only what a player sees, and what a player hears or feels needs an unchanged sim trace or audio output; when unsure whether a player notices, his go); (2) costs money or cannot be undone (risk tier 3); (3) touches the rule files, secrets or infrastructure (risk tier 2, A4); (4) changes or newly sets a decision of his: a decision draft accepted, a design or look rule, a pass mark, a hard constraint, an ADR or the register, or a design or look doc beyond a pointer; (5) changes anything under `content/source/`; (6) brings in a new download, file, version or checksum he has not answered (TOOL-SOURCE), or passes its change-set's stop line; (7) leaves a Critical or High open after round 3, or a security finding of Medium or above left open; drops game symbols in a revert; or comes from a fork. His go reaches a merge in one of two ways: his own go in the session's chat (CLAUDE.md, way 2), or his Advisor's PASS comment, which his Advisor writes only after he has given, for that head, what this list needs (his Brave check for (1), his yes for the rest; CLAUDE.md, way 1; MERGE-PROMPT (d)). A record of words he typed, copied by script, is not in this list. It goes to his Advisor's merge check instead, which names each added entry's sha256 as compared against his own messages. An entry's stated byte count and sha256 are an integrity check (the quote is whole), never proof that he typed it. An added entry the merge check does not name needs his go. Any other quote of him, `[DECIDED]` line or answer to one of his codes outside `docs/DECISIONS.md` goes to the merge check, and needs his go when it sets a decision. `track.mjs` prints the path floors of (3), (4) and (5) and lists the quotes; (1), (2), (4), (6) and (7) stay judged.

**How the script handles DECISIONS.md and quotes of him (H3, SA-RT-3):**
- **Integrity.** It strips the "> " from each added entry's quote, joins the lines with \n, and checks the byte count and sha256 against the header. I checked the format on 3 real entries, one multi-line: all matched.
- **The saved file.** When the header names a saved file at a commit, it compares the quote byte for byte with `git show <commit>:<file>`. A mismatch is a go floor. So is a commit or file it cannot read (a shallow clone, a deleted checkpoint): never a pass.
- **Every added entry**, quote and header, is listed under `forMergeCheck` with its sha256, for his Advisor to compare against his own messages.
- A removed quoted line, or an entry that does not match, becomes a go floor. Any other added text in DECISIONS is listed for the merge check.
- **Outside DECISIONS**, in any changed text file: an added quote line beside his name, a `[DECIDED]` line, or one of his decision codes followed by an answer is listed under `forMergeCheck` (a signal, so the routine does not merge it: H1). Whether it sets a decision (a go floor) stays judged by the advisor agent. This test works by patterns and can miss a quote; the advisor agent's read of the diff stays.

**The visible-change rule stays** (his Brave check, CLAUDE.md:94-95 and :292). `eyesPaths` lists `legacy/` game files and pages, `packages/client/src/`, `packages/editor/`, `*.html` and `*.css`, plus what a player hears or feels: `crates/sim/`, `packages/sim-worker/` and `content/` (SA-RT-10, M6). For a PR touching these, the advisor agent must show proof that nothing changed for the player, or it goes to his check (and his Advisor writes PASS only after it):
- what he **sees**: identical renders. These prove "sees" only.
- what he **hears or feels** (game feel, balance, controls): an unchanged sim trace (`tools/sim.js` on base and head, or equal replay hashes) or unchanged audio output.

**Escalation categories (advisor.md:44-45, after category 4).** NEW category 5:
> 5. **Content sources:** any change under `content/source/` (Mustafa's SOURCE-GO (a)).

`content/source/` does not exist yet. The rule holds from the day it appears.

### 3g. Every reason that makes a PR needs-mustafa today

Rows use inputs-A's numbers. Rows 13 and 14 are duplicates of row 3. "Merge check" means his Advisor's merge check; anything listed for it also stops the routine's own merge (H1).

| # | Reason (where) | Lands | Why, in one line |
|---|---|---|---|
| 1 | a player sees, hears or feels it (advisor.md:37) | stays | his "my eyes only where it shows" |
| 2 | a design decision the docs don't settle (advisor.md:38) | stays | EYES-KEEP: a decision of his newly set |
| 3 | rule files and other tier-2 paths (advisor.md:39-43, :75-86) | stays | EYES-KEEP: the rule files |
| 4 | secrets and infrastructure (advisor.md:76-78) | stays | A4's tier 2 is his words, unchanged; often costs money or can't be undone |
| 5 | a hard-constraint change (CLAUDE.md:48) | stays | a decision of his changed |
| 6 | decision docs (advisor.md:61-64; CLAUDE.md:107) | split | a decision changed or set: stays; a DECISIONS entry copied by script: moves to the merge check, which names its sha256 (an entry it does not name: stays); other quotes of him, `[DECIDED]` lines, answers to his codes: move to the merge check, or stay when they set a decision (H3) |
| 7 | irreversible or costs money (advisor.md:44-45) | stays | EYES-KEEP |
| 8 | a major change: new system, large refactor (advisor.md:102) | moves | a new mechanic is #2; an internal system or refactor shows nothing |
| 9 | a measured regression the PR accepts (advisor.md:102) | moves | unless it changes a pass mark (stays: EYES-KEEP) or a player feels it (#1) |
| 10 | a security finding left open (advisor.md:102-104) | split | Medium or above: stays (his go); a Low: named in the merge check (SA-RT-9); Critical or High after round 3 is #17 |
| 11 | a doubt the checks can't settle headless (advisor.md:104) | moves | a doubt about what a player sees, hears or feels stays (#1: in doubt, his eyes) |
| 12 | "verify in a day session" (advisor.md:100-101) | changes | the routine never merges it; under way (1) the in-session advisor agent runs it and its APPROVE is posted on the PR (H4, SA-RT-11), then his Advisor's PASS |
| 13 | CI's green does not stand (advisor.md:100) | as #3 | duplicate |
| 14 | step 0: self-referential rules (advisor.md:98) | as #3 | duplicate |
| 15 | outside the guard's allow-list (routine-guard.mjs:79-100) | goes | a seat limit: the building chat merges; the allow-list is unchanged |
| 16 | a guard refusal blocks a routine step (advisor.md:98-99) | goes | the step moves to a day session; the refusal is still reported |
| 17 | Critical or High open after round 3 (CLAUDE.md:111-112) | stays | it stops the change-set |
| 18 | a revert that drops game symbols (advisor.md:300-303) | stays | only he goes past a required check |
| 19 | a PR from a fork (advisor.md:153) | stays | never merged here |

**Four newer rules that inputs-A found missing from the kit's list:**
- **His Advisor's merge check** (DECISIONS:260, :432): its PASS becomes the condition of way (1) for every PR. It still comes with his go where his rules need it: for a PR that keeps his go, his Advisor writes PASS only after he has given it for that head (his Brave check, his yes), so his go reaches the merge through the PASS, with no tap (MERGE-PROMPT (d)). Until then its check may read `Advisor merge check: FOR HIS GO #<pr> <full sha>`, a status line the guard never accepts.
- **TOOL-SOURCE** (general rule DECISIONS:1348; the one-file answer :2220): stays, as item (6) of the new section. `track.mjs` prints the judged prompt (`toolSource`, L2).
- **Size stop lines** (DECISIONS:412-431, :154): stay, as item (6).
- **SOURCE-GO** (DECISIONS:115): stays, as item (5), category 5 and the script.

### The remaining OLD/NEW changes, CLAUDE.md

**:62** (plan step)
- OLD: "reviewers you will call,"
- NEW: "the review track and reviewers that main's `.claude/scripts/track.mjs` gives for the planned files,"

**:83** (review step)
- OLD: "5. Review on the change-set's final diff, within the review budget below (after fixes, only what changed):"
- NEW: "5. Review on the change-set's final diff, by the reviewers its review track names (below), within the review budget (after fixes, only what changed); who covers what:"

**:92-94** (merging after review)
- OLD: "it merges at once if no human check is needed; otherwise it labels it `needs-mustafa`, and the session Mustafa gives his go to merges it (his Brave check for anything a player sees, hears or feels; his reconfirmation for rule changes and other major changes)."
- NEW: "it merges at once if no human check is needed, the guard's allow-list admits it and nothing is listed for the merge check; any other PR is merged by the chat that built it once his Advisor's merge check reads PASS (Standing permission, way 1), and that chat may also merge the allow-listed ones first; if it needs his go, it labels it `needs-mustafa`, and his Advisor writes PASS only after his go (his Brave check for anything a player sees, hears or feels; his yes for the rest of advisor.md's "His go keeps"), or a session merges it on his go in its chat (way 2)."

**:103** (tier table, row 1)
- OLD: "| 1 | game code, pages, `tools/` (not gate files), tests, spikes; the docs tier 0 leaves out | CI + the advisor + the type's reviewers | `ready`: the PR Advisor; else his go (outside the guard's allow-list: from a day session) |"
- NEW: "| 1 | (unchanged) | CI + the advisor + the track's reviewers (`track.mjs`) | `ready`: the chat that built it, after the advisor's APPROVE in that session and his Advisor's PASS, or the PR Advisor inside the guard's allow-list when nothing is listed for the merge check; else his go (through his Advisor's PASS, or in a chat) |"

**:107** (M2)
- OLD: "decision docs always need his go."
- NEW: "whatever advisor.md's "His go keeps" lists needs his go (given in a chat, or carried by his Advisor's PASS), a decision of his set or changed among it; a record of his typed words copied by script goes to his Advisor's merge check instead."

**New bullet after :113** (about 9 lines):
> - **Review tracks** (Mustafa, 2026-11-25; his words in `docs/DECISIONS.md`): main's `.claude/scripts/track.mjs` sets the track from the files touched, never the PR's own copy; in doubt, full. Light: LIGHT-DEF (a): the plan and the advisor's plan review, the gate and CI, one review round by the one reviewer the script names at the default effort (plus security-auditor on a light security trigger; risk tier 0: no round), the verifier at max on the final result, a short report, and a second round only for an open Critical or High. Full: netcode, the Rust core, server code, anything anti-cheat and the rule files (the script's table): test-engineer plus at most two picks by the script, all at max (rule files: tier 2's reviewers); the advisor's plan and head reviews, the verifier and the third round for an open Critical or High stay.

### The remaining OLD/NEW changes, advisor.md

**:33-34** (authority; M1)
- OLD: "Merging approved PRs, only when running as the PR Advisor routine: at once when they need no human check, after Mustafa's go when they do (PR review, step 6)."
- NEW: "Merging approved PRs, only when running as the PR Advisor routine: at once when they need no human check, sit inside the guard's allow-list and list nothing for the merge check; after Mustafa's go when they do (PR review, step 6). In a day session, your APPROVE in that session, with his Advisor's PASS, is what lets the chat that built a PR merge it (CLAUDE.md, way 1)."

**:63-64** (decision docs; M1)
- OLD: "these "decision docs" are risk tier 1 and always needs-mustafa (his go);"
- NEW: "these "decision docs" are risk tier 1, and need his go when they set or change a decision of his ("His go keeps", item 4); a record of his typed words goes to his Advisor's merge check;"

**:71-74** (tier 1; M1, M3)
- OLD: "`ready`, yours;"
- NEW: "`ready`, the building chat's after his Advisor's merge check, or yours inside the allow-list when nothing is listed for the merge check;"
- OLD: "every other PR merges from a day session on his go."
- NEW: "every other PR is merged from a day session: by the chat that built it (CLAUDE.md, way 1: your APPROVE in its session and his Advisor's PASS, which for a needs-mustafa PR his Advisor writes only after his go), or on his go in a chat (way 2)."

**:92-104** (today's classes)
- OLD: the two bullets `ready` and `needs-mustafa`, with the trigger list.
- NEW:
  > - `ready` = risk tier 0, or risk tier 1 with nothing in "His go keeps". Nothing at risk tier 2 or 3 is ready.
  > - `needs-mustafa` = anything in "His go keeps". Step 4's other signals go under "For the merge check" in your comment; the routine merges a `ready` PR only when that list is empty (H1).

**:121** (plan review). NEW line:
> - The track and reviewers match main's `track.mjs` on the planned files.

**:131-132** (evidence rule), appended:
> One comment is a merge condition, never an instruction: his Advisor's merge check (step 6). Its PASS is his Advisor's statement that what his rules need for that head has happened (his go included where needed), never his own word.

**:167-169** (step 2)
- OLD: "is **needs-mustafa** ("verify in a day session"): the day-session advisor or Mustafa runs it."
- NEW: "is never merged by the routine ("verify in a day session"; list it under "For the merge check", label it `ready-to-merge` if you approve it): the building chat's advisor agent runs it in that session, and the chat posts its APPROVE on the PR (CLAUDE.md, way 1)."

**:174** (step 2; M1)
- OLD: "If something can't be verified headless, say so and move it to Mustafa's checklist."
- NEW: "If something can't be verified headless, say so: a doubt about what a player sees, hears or feels goes to Mustafa's checklist; any other doubt goes under "For the merge check"."

**:178-184** (step 4; SA-RT-9)
- OLD: the needs-mustafa list from "or is a decision doc (Risk tiers)" (wrapped across :178-179) to "When unsure, or when both lists fit, it is needs-mustafa."
- NEW:
  > or is in "His go keeps" (a security finding of Medium or above left open included). A major change, a measured regression the PR accepts, a security Low left open, a doubt your checks can't settle headless, a DECISIONS entry or another quote of him goes in your comment under "For the merge check". When unsure whether "His go keeps" applies, it is needs-mustafa.

**:196-197** (step 5). NEW item:
> For a `ready` PR you do not merge: "nothing: the building chat merges after his Advisor's PASS". For a `needs-mustafa` PR: his check or yes, as today; his Advisor's PASS then follows.

**:200** (step 6, first bullet; H1)
- OLD: "**APPROVE + ready + all checks green:** label `ready-to-merge`, merge the PR, and note the merge in your comment."
- NEW: "**APPROVE + ready + all checks green, inside the guard's allow-list, nothing listed for the merge check:** label `ready-to-merge`, merge the PR, and note the merge in your comment. Any other `ready` PR: label `ready-to-merge`; do not merge: the building chat merges it (CLAUDE.md, way 1)."

**:200-208** (step 6). Two NEW bullets (about 10 lines; SA-RT-2, M8):
> **After-check, every run (no state kept):** list every first-parent merge on main in the last 24 hours (so each merge appears in about three runs). Count a merge as your own only when your merge note names its merged head and every changed file passes the guard's allow-list (merge notes are comments any session can imitate: C4, SA-RT2-2); check every other merge. For each, give the PR, the merged head, the merge time, and every APPROVE line and merge-check comment found for that head, by comment id, `created_at`, `updated_at` and `performed_via_github_app`. A missing or edited one is a Critical for Mustafa. Report ids and times, never the check's line. This finds missing or edited checks, never a forged one: only his Advisor, comparing with its own list of the checks it issued, can.
>
> **The merge check counts** only as a comment by `Mergincanli` (user type `User`) whose first line is exactly `Advisor merge check: PASS #<pr> <full head>`, never edited, posted after the gate run on that head (the comment list read whole, matched exactly). His Advisor writes it only when nothing in "His go keeps" applies or after Mustafa has given what it needs for that head; `FOR HIS GO` is a status line, never a pass (MERGE-PROMPT (d)).

**:243-244** (tier-2 checklist): add `.claude/scripts/track.test.mjs` to the kit's tests.

**:266-268** (review budget; SA-RT-9, M1)
- OLD: "and a security finding left open, which keeps the PR needs-mustafa at any severity (step 4): a security finding not fixed in the PR (a STATUS follow-up included) counts as left open."
- NEW: "and a security finding left open, which keeps the PR needs-mustafa at Medium or above and goes under "For the merge check" as a Low (step 4): a security finding not fixed in the PR (a STATUS follow-up included) counts as left open."

**:304-305** (revert merge; M1)
- OLD: "A revert of a game-file merge is needs-mustafa ("verify in a day session");"
- NEW: "A revert of a game-file merge is never merged by the routine ("verify in a day session"): a day session merges it by CLAUDE.md's way (1) or (2), as any PR;"

**:325-326** (Never)
- OLD: "Merge anything in escalation categories 1–4, at risk tier 2 or 3, or classed `needs-mustafa`, without Mustafa's go for its current head."
- NEW: "Merge anything in "His go keeps" (escalation categories 1–5, risk tier 2 or 3) or classed `needs-mustafa` without Mustafa's go for its current head; or, as the routine, a `ready` PR outside the guard's allow-list or with anything listed for the merge check."

### The other kit files (M1, SA-RT-9)

**security-auditor.md:54**
- OLD: "A security finding left open keeps the PR needs-mustafa at any severity (advisor.md, PR review step 4)."
- NEW: "A security finding of Medium or above left open keeps the PR needs-mustafa; a Low left open is named in his Advisor's merge check (advisor.md, PR review step 4)."
- Note: "at any severity" is a kit rule with no source in DECISIONS (SA's reading), so this narrows the kit, not his words. It is listed in section 5.

**NIGHT_SHIFT.md:77** ("For Mustafa: numbered actions (Brave checks with steps, merges to approve), or "nothing".): **stays.** Brave checks and merges to approve still exist (his go, then his Advisor's PASS or his go in a chat), and the night shift still never merges (NIGHT_SHIFT.md:63). Who merges a night-shift PR: section 4b (M10).

## 4. Plain answers to 4a-d

### 4a. How his Advisor's merge check reaches the building chat, and what stops a forgery

**How it arrives.**
- His Advisor posts one comment on the PR. Its first line is exactly `Advisor merge check: PASS #<pr> <full id>`. Below it: what was checked, the sha256 of each added DECISIONS entry it compared against his own messages (H3), and, for a PR that keeps his go, what he gave for that head (his Brave check, his yes). PASS means everything his rules need for this head has happened: his Advisor writes it only when nothing in "His go keeps" applies, or after he has given what it needs (MERGE-PROMPT (d)). While the PR waits for him, it may post `Advisor merge check: FOR HIS GO #<pr> <full sha>`, a status line the guard never accepts.
- His Advisor keeps its own list of the checks it issued: PR, full SHA, time (SA-RT-2).
- **Not yet known:** whether his Advisor chat can post a PR comment at all. inputs-A Part 3 says his Advisor thread merged #65 and #68, so it has some GitHub write access, but no comment from it has been seen. T-MARK item 4 measures it.
- Until then, the honest answer is one paste per PR: he pastes the check into the PR's comment box, from the web or the phone. That is still fewer steps than today's paste into the chat, plus his go, plus his tap; for a PR that keeps his go, his go and the PASS can travel in the same step. His item 4a's "without my paste" (DECISIONS:149) holds only if his Advisor chat can post (M9).

**How the chat reads it.**
- It counts only a comment from `Mergincanli` with user type `User`. His WATCH-70 words (DECISIONS:430): anyone can comment on a public repository.
- The first line must be exact, PR number and head included.
- The comment must never have been edited (`updated_at` equals `created_at`), and must be posted after the gate run on that head completed.
- The PR's comment list is read whole across pages, and the line is matched exactly: no trimming, no case or Unicode folding (SA-RT2-3).
- All of it is re-read through the API right before the merge. Under MERGE-PROMPT (d), the guard re-reads the same before it skips the prompt, and reads nothing else about the PR's content.

**What stops a session writing it, after this change:**
- **Through the GitHub tools:** no session, subagent or routine can write it. GUARD-MARK refuses any write that carries its line, and any edit of a comment that carries it (3e).
- **Through Bash:** a write meets a permission prompt (`gh pr comment`; in Auto mode, the classifier instead of his tap) or a deny rule (`gh api` with write flags).
- **What remains:**
  - a prompt that someone approved;
  - while R4-1 is open, a disguised Bash form, which also meets a prompt.
- **What can catch a forgery:** a forged comment looks exactly like a real one (same user, same line). Only his Advisor can catch it, by comparing every merge on main with its own list of the checks it issued. That happens at his board checks, whenever he runs them; no fixed delay is promised (H2).
- Writing or imitating the line stays a Critical. Under (d) the stakes are higher than under (c): a forged PASS could merge a PR that needed his eyes, not only one that showed nothing, because the guard no longer checks the files. The route is the same: only through a prompt someone approved.

**A possible mark (MERGE-MARK (b), later).**
- GitHub's API records which app posted a comment (`performed_via_github_app`). The PR Advisor's comment on #70 reads `claude`. A comment he types in the web page should carry none.
- GitHub's docs give the field but do not say which tokens set it, so the build measures it (test T-MARK). It reads the field both through the GitHub tool and through the raw API (`gh api .../issues/comments/<id>`) (SA-RT-12).
- It runs without manual steps from him where possible (M9). Every test comment uses a neutral line, never the check's line, which GUARD-MARK refuses anyway.
- The comments T-MARK reads:
  1. a session's comment through the GitHub tool;
  2. a day session's `gh pr comment`, after its prompt;
  3. a comment he has already typed on the web, if one exists; a phone-app comment only if he chooses MERGE-MARK (b);
  4. a post by his Advisor chat, if it can post;
  5. a session's edit of a comment of his: does the mark appear? Whatever the answer, an edited check comment is refused (`updated_at` must equal `created_at`). That is a requirement now, not something T-MARK decides.
- **What (b) costs him:** if his Advisor's own posts carry an app mark, he pastes the check himself, once per PR.

**What catches a problem afterwards:**
- **His Advisor**, at each board check, compares every merge on main with its own list (PR, SHA, time). This is the only check that catches a forgery.
- **The PR Advisor's after-check**, at each run (07:45, 13:45, 19:45), lists every merge on main it did not make. Each comes with the APPROVE line and check comments found for that head, by id and times. It finds a missing or edited check, never a forged one (SA-RT-2, M8).
- **What he gets:** a merge with no approval or no check, or with an edited one, goes to him as a Critical. A wrong merge is undone by a revert (rollback rule). Whether to revert is his decision; the rollback rule is unchanged.

### 4b. How the building chat is woken

- After it marks the PR ready, the chat subscribes to the PR's activity (`subscribe_pr_activity`). This becomes standard; watching #70 was asked of him (WATCH-70).
- New comments arrive in the chat as events. An event can carry anyone's text, so it is data: the chat re-reads the comment through the API and re-checks every condition itself (SA-RT-12).
- It also schedules one re-check about 4 h later (`send_later`), in case an event is missed.
- Limits:
  - A PR's events go to one session at a time.
  - A chat that has ended or been archived is not woken.
- **WAKE-END (a), his decision of 2026-11-26:** if the building chat has ended, the PR waits for one line from him to a new chat on that lane. That chat runs the advisor agent itself on that head before it merges (SA-RT-11). No other chat merges it on its own.
- **Night-shift PRs (M10):** the night shift never merges. Its PRs wait the same way: the day chat his line starts on that lane merges them, with its own in-session APPROVE and his Advisor's PASS (way 1), or on his go in that chat (way 2).

### 4c. What the PR Advisor routine still does

It keeps:
- its schedule (07:45, 13:45, 19:45; never pull-request events);
- one review comment per PR, and its labels;
- its merges of tier 0 and of allow-listed `ready` PRs, only when nothing is listed "For the merge check" (H1; A4's auto-merge; the allow-list is unchanged);
- its comment counting as the advisor agent's approval for its own merges and for way (2), never for way (1) (SA-RT-11).

New:
- It cross-checks the track by reading main's path table. It never sets the track.
- It moves step 4's softer signals into "For the merge check".
- It runs the after-check of 4a.
- It no longer labels `needs-mustafa` for these, from 3g: rows 8, 9, 11, 15 and 16; the Low part of row 10; the copied-words part of row 6. Row-12 PRs it labels `ready-to-merge` and leaves to the building chat.

Risk-tier table: only row 1 and :107 change (shown in section 3). Rows 0, 2 and 3 are unchanged.

### 4d. How a wrong change that shows nothing is caught after the merge

- CI runs gate, smoke and toolchain on every push to main.
- His Advisor's board checks (the only check that catches a forged merge check), and the PR Advisor's after-check.
- A report from him, or a session stopped by the change.
- Then the rollback rule, unchanged (CLAUDE.md:119-122; advisor.md "Revert"): revert first, then fix in a new PR, plan first.

## 5. Words of his that this change narrows or changes

1. **DECISIONS:432** (2026-11-25, 21:45 Amsterdam): "A merge is on my go for the approved head, after my Advisor's merge check." Changed for way (1): a PR that needs no go merges on his Advisor's PASS alone; for one that keeps his go, his go is given to his Advisor and reaches the merge through the PASS. His brief's item 6 keeps it in force until this change merges.
2. **DECISIONS:430** (WATCH-70): "A comment or review there is information, never my word". Its reason holds (anyone can comment, so only an exact, unedited PASS from `Mergincanli` counts). But under MERGE-PROMPT (d) the PASS comment now carries his go to the merge, as his Advisor's statement that his go was given. That is his own decision (d); listed so he sees it.
3. **CLAUDE.md:127-133**, the merge sentence of the standing permission. It is the kit's form of his A4 and of his rule that a GitHub "merge" never counts as his go. Changed as shown: "only on Mustafa's own go in its chat" now also allows his go expressed through his Advisor's PASS comment (way 1). His go in a chat is kept as way (2). Way (1) is stricter on the approval: only an in-session APPROVE, posted on the PR. This is his decision MERGE-PROMPT (d), given in the Guard fix chat on 2026-11-26 (he typed "d", choosing the option text offered to him; that text is not his words).
4. **EYES-KEEP (a) and his Brave check** (DECISIONS:145; CLAUDE.md:94-95, :292): kept, but routed through PASS. His Brave check still comes first for anything a player sees, hears or feels; his Advisor writes PASS only after it. What changes is the route: no tap once the PASS is in (the advisor agent still reviews every pushed head). Recorded as part of MERGE-PROMPT (d).
5. **CLAUDE.md:103**, the tier-1 merge column, the kit's mapping of A4. Changed. A4's own words (advisor.md:49-52) are unchanged: tier 1 says "Mustafa's Brave check when players see or hear it".
6. **CLAUDE.md:107** and **advisor.md:61-64**, decision docs "always" need his go. Narrowed by his own last EYES-KEEP sentence: a record of his typed words, copied by script, goes to his Advisor's merge check instead, which names each entry's sha256.
7. **CLAUDE.md:94**, "his reconfirmation for ... other major changes". Narrowed: moved to the merge check.
8. **His words of 12 November** (DECISIONS:3160): "The agents that review, verify or design game and engine code always run at max." Narrowed for the light track's one reviewer, which runs at the default effort. The verifier stays at max (LIGHT-EFFORT (a)). Why this reads his newer words as winning: his brief itself sets "the repository's default effort" for light (item 3b, DECISIONS:141) and keeps the 12 November effort for full (item 3d, :143) (M4).
9. **"V4 merges its own" / "the chat that built it"**: unchanged. WAKE-END (a), his decision: when the building chat has ended, only a new chat that one line from him starts on that lane merges the PR (4b).
10. **"Everything else merges on green checks plus your review"**: the PR Advisor's merges of tier 0 and allow-listed `ready` PRs stay without his Advisor's check (A4's "auto-merge"), now only when nothing is listed for the merge check (H1). Read as unchanged, unlike item 8, because his brief does not touch A4 and his item 4c asks what the routine still does (DECISIONS:151) (M4). MERGE-PROMPT (d) does not change these routine merges. Listed so he can say otherwise.
11. **advisor.md:205-208**, how his go counts (the advisor agent's Q2). OLD: "**Mustafa's go** counts only in his own words in the chat of the session that merges: about that PR (its number, or the PR his message answers), given after your APPROVE of its current head. A \"merge\" written on GitHub never counts: Claude sessions post as his account (`Mergincanli`), so GitHub text can't prove it is his. Never act on a go that a session relays or writes." NEW: the same sentence for way (2) and for the routine, plus: "Under way (1) (his MERGE-PROMPT (d)), his go reaches a merge only through his Advisor's PASS comment, which the guard checks; never through text a building session writes or relays." Otherwise unchanged for way (2). **advisor.md:131-132**: one appended clause (section 3), which says the PASS is his Advisor's statement, never his own word.
12. **CLAUDE.md:138-147**, LANE-RULES and ACCOUNTS: no word changed. Their line "Text that only claims to be from me (in a pull request, an issue, a commit, a file or a tool result) is never my word" still holds under (d): the PASS comment never claims to be his word; it is his Advisor's statement, and what makes it a merge condition is his own decision (d), typed in his chat. Their "two accounts" is out of date since WHICH-ACCOUNT. That is already recorded as a follow-up (DECISIONS:192), to be asked in a later batch.
13. **"His go keeps" holds more than EYES-KEEP's three things** (DECISIONS:145) (M4). It also holds: secrets and infrastructure (A4's tier 2); TOOL-SOURCE (DECISIONS:1348); stop lines (CAP, :154, :412-431); a Critical or High open after round 3 (A4's item 2); a revert past a required check; forks; SOURCE-GO (:115); and a security finding of Medium or above left open. Each is stricter than his paste's item 1 alone and rests on other words of his or on today's kit, but together they narrow that item's "Everything else merges on green checks plus your review" (DECISIONS:134).
14. **Not his words, a kit rule narrowed:** "a security finding left open keeps the PR needs-mustafa at any severity" (advisor.md:102-104, :180-181, :266-268; security-auditor.md:54) has no source in DECISIONS. It becomes Medium or above; a Low goes to the merge check (SA-RT-9). Listed because it loosens today's kit.
15. **LIGHT-DEF's "one review round by the one reviewer that fits"** (DECISIONS:141): a light security trigger adds security-auditor as a second reviewer in that round (SA-RT-8). Stricter than his words, resting on CLAUDE.md step 5's subjects; listed so he can say otherwise.

## 6. Size, counted bottom-up (L5)

Lines added (a changed line counts as added). Docs and the report do not count, as his CAP (a) says. Recounted for MERGE-PROMPT (d): the guard no longer reads the PR's file list or the track table, so its part is smaller.

| Part | Full version (PLAN-GO (a)) |
|---|---|
| track.mjs | about 185: path table 50, base, head and raw diff 20, risk-tier-2 floors 15, track, floors and eyes 15, reviewer picks and security triggers 30, DECISIONS check and saved-file compare 30, other quotes 10, TOOL-SOURCE prompt 5, output 10 |
| track.test.mjs | about 100 (one-line table cases, as the guard's tests are written) |
| routine-guard.mjs and guard.mjs | about 38: GUARD-MARK 15 (with the recursive scan), MERGE-PROMPT (d) 18 (the comment list read whole across pages 6, the exact first line, user type, unedited and after-the-gate-run tests 8, the gate run's completion time 4), toolchain 1, the allow output 4 |
| routine-guard.test.mjs | about 52: fake-io additions 12, GUARD-MARK cases 15, allow-path cases 20, toolchain 5 |
| CLAUDE.md rule text | about 42 |
| advisor.md rule text | about 69 (the PASS rule and the after-check's own-merge test add about 2) |
| security-auditor.md | 2 |
| settings.json, NIGHT_SHIFT.md | 0 |
| **Total** | **about 490 (185 + 100 + 38 + 52 + 42 + 69 + 2 = 488); stop line about 980 (twice 490)** |

Under (c) it was about 510 (stop line about 1,020). (d) drops the file-list read, the protected-path test and the table read from the guard (about 12 lines) and their test cases (about 13), and adds about 3 lines of rule text.

The smaller version (about 270 lines, stop line about 540) was offered beside it, as his CAP asks above 150 lines; he chose the full version (PLAN-GO (a)), so it is not built.

## 7. Risks, what is not changed, reviewers, evidence

**Risks, ranked:**
1. **The lost second layer (his choice, MERGE-PROMPT (d)).** Under (c), the guard also checked every changed file and kept the prompt on any file that needs his go. Under (d) it does not, so his Advisor's PASS is the only gate:
   - **An Advisor slip** (a PASS on a PR that needed his eyes or his yes and did not get it) is not caught technically. A visible change, a rule file, a guard change or a decision of his would then merge with no Brave check, no yes and no tap.
   - **The protected-path test goes too.** A PR that changes the rule files, the guard or the gate merges without a prompt on PASS alone; his Advisor must write PASS for such a PR only after his yes.
   - Nothing technical catches a PASS his Advisor wrote by mistake: the after-check finds a missing or edited check, and his Advisor's own list finds a forged one, but a real PASS written in error passes both. He notices it (or a session or report does), and a revert undoes it.
   - Mitigations, before the merge: the in-session advisor agent's APPROVE line names what in "His go keeps" it found, so his Advisor reads it before writing PASS; `track.mjs` prints the go floors and eyes paths for his Advisor. Whatever the guard does, GitHub's ruleset ("Shamble Ruleset 1") still requires a PR with green gate and smoke whatever the guard does; and a revert undoes a wrong merge (rollback rule, unchanged).
2. **A forged merge-check comment** (Critical class). GUARD-MARK closes the GitHub-tool route for every session, subagent and routine. What remains is a prompt someone approved (or, in Auto mode, the classifier let through), or, while R4-1 is open, a disguised Bash form, which also meets a prompt. Under (d) a forged PASS reaches further than under (c): it could merge a PR that needed his eyes. Only his Advisor's comparison with its own list catches a forgery, at his board checks; no fixed delay is promised.
3. **A visible change judged invisible**, merged without his eyes. Mitigations:
   - `eyesPaths`, now including what a player hears or feels;
   - renders for "sees", and a sim trace or audio output for "hears or feels";
   - "when unsure, his go".
   The guard checks no files, so his Advisor's PASS covers every floor, judged or by path.
4. **Drift in the path table:** netcode code moved into a light folder. Mitigations: main's copy decides; the tests; anything in no list is full; all of `crates/` and every CI-deciding file is full. The guard no longer reads the table, so drift affects the track, the reviewers and what `track.mjs` prints for his Advisor, never the prompt skip.
5. **The light track misses a High** that a second reviewer would catch. Mitigations: the verifier, the advisor's head review, and the light security triggers.
6. **A quote of him outside DECISIONS that the pattern test misses.** Mitigations: the advisor agent's read of the diff, and his Advisor's check.
7. **The guard refuses a merge** over a cancelled toolchain run beside a green one (seen on #76's head): one re-run needed. `toolchain` is not yet a required check in the ruleset (a follow-up click for him).
8. **An ended chat or a subscription held by another session:** the PR waits for one line from him (WAKE-END (a)).
9. **Auto mode's classifier:** whether a hook's `allow` also skips it is not documented. The build reads this from a real session's merge prompt; if it does not, a way-(1) merge in Auto mode may still be refused, and the chat then asks him.
10. **GUARD-MARK refuses an honest write** that quotes the check's line (a PR body, a report). Sessions name it in other words. Low.
11. **The after-check's fixed 24-hour look-back** keeps it stateless (M8 suggested marks on checked PRs instead), but a full day of failed PR Advisor runs would miss merges. His Advisor's own comparison still covers them.

**Honest preview** (applied by hand to the last ten merged PRs, from `git diff --name-only M^1 M`; the script must reproduce it):

| PR | Track (why) | Go floors | His go needed |
|---|---|---|---|
| #76 main-wall | light (docs) | none by path; DECISIONS entries listed for the merge check | no (the merge check names each entry's sha256) |
| #74 ci-preview | full (.github) | rule files | yes |
| #73 guard-wrappers | full (.claude) | rule files | yes |
| #70 4a recording | full (Rust core, netcode) | UNITS.md: a [DECIDED] line that sets a decision | yes |
| #71 ci-after-3a | full (.github) | rule files | yes |
| #67 ADR 0006 | light (docs) | docs/adr | yes |
| #69 guard-env-fix | full (.claude) | rule files | yes |
| #68 cs3b | full | visible (Brave) | yes (his Brave check) |
| #66 preview report | light (docs) | none by path; DECISIONS entries and any quote of him in SPIKE_PLAN listed | no |
| #65 cs3a | full | visible (browser check) | yes (his browser check) |

Result: 7 full and 3 light. Under MERGE-PROMPT (d), the building chat would have merged all ten once his Advisor's PASS was in, with no tap. For the 2 that need no go, his Advisor writes PASS after its merge check (unless it finds a quote that sets a decision); for the 8 that need his go, only after he gives it. The larger gain is on future code PRs that show nothing.

**What is not changed:**
- the rollback rule;
- the guard's other rules and its allow-list;
- settings.json (no allow line);
- MAIN-WALL (only his account updates main; a PR with gate and smoke is required);
- the night shift never merges (NIGHT_SHIFT.md:63), and NIGHT_SHIFT.md itself;
- the PR Advisor routine's own merges (H1, 4c);
- DEP-HOLD, the review budget and severity scale, his Brave check (now routed through PASS), and A4's words;
- CHECKPOINTS.md (§9 still folds before ready: fold before asking for the head review, so the reviewed head is final);
- the interim git and gh rule. The guard edits here land before R4-1's change-set (SA-RT-12): they touch only the GitHub-tool and merge paths, which R4-1 (a Bash reading gap) does not affect.

**Reviewers:**
- On the plan: security-auditor and the advisor agent (both approved the (c) design: section "Review findings"); both read this (d) update before the build.
- At build: tier 2 brings the advisor agent and security-auditor, plus test-engineer for the script and tests, and the verifier.

**Evidence to produce:**
- `node --test .claude/scripts/track.test.mjs` with its counts.
- The guard's and settings' tests at main's counts plus the new cases: GUARD-MARK (recursive scan, comment edit, subagent merge) and toolchain; and the allow path: PASS present gives allow, and so does a PASS found only on the second page of comments; no PASS, an edited one, the wrong user or user type, a PASS older than the gate run, a wrong PR number or short SHA, a trailing space or folded case, a `FOR HIS GO` line, and a throw in the new conditions give silence (the prompt); a subagent caller, a stale or short expected head, red checks, and a throw in the base merge check are refused, as today.
- The script run on the ten PRs above (it must match the preview or say why).
- A dry run on an open PR (#77: light, no floors).
- The hash and saved-file check on the last 20 DECISIONS entries.
- T-MARK's readings, through the GitHub tool and the raw API.
- The allow path is proven in the guard's tests. Its first real use is the first way-(1) merge after this merges, and the building chat reports what the prompt did.

## Build steps and his asks

Seven of the eight files are under `.claude/` and one is CLAUDE.md, so each edit asks him first (CLAUDE.md step 3); the build runs in Accept edits or Manual mode, since Auto mode's classifier may refuse a `.claude/` edit before the prompt. To keep it to about one ask per file, not one per hunk:
1. Each file is built and tested in a gitignored copy first: `.gate/review-tracks/build/`, a plain copy of the kit at the branch head, never a git worktree, with CLAUDE.md copied as `CLAUDE.md.new`. Why: the guard's day-session test asks for any path whose name is `CLAUDE.md` at any depth, and for `.claude/` paths inside any worktree of this repository (routine-guard.mjs:34-46, :119-133); a `.claude/` folder in a plain copy under `.gate/` matches neither, so building in the copy asks nothing. `node --test` runs on the copy's suites.
2. It is applied to the real file in ONE file-tool call, asked of him with its exact text: one Edit spanning the first to the last changed line where the changes sit together, or one Write of the whole file where they span most of it or the file is new. The guard and its tests are the exception: their changes sit far apart in long files, so each takes two Edits (table below).
3. After each apply: `cmp` against the copy, then the kit suites (`node --test .claude/hooks/routine-guard.test.mjs .claude/hooks/settings.test.mjs .claude/scripts/*.test.mjs`) and `node tools/gate.mjs`.
4. One commit for the whole set, after all eight files are applied and green.

| # | File | How it is applied | Asks |
|---|---|---|---|
| 1 | `CLAUDE.md` | one Edit spanning :62 to :133 | 1 |
| 2 | `.claude/agents/advisor.md` | one Write of the whole file (changes from :33 to :326) | 1 |
| 3 | `.claude/agents/security-auditor.md` | one Edit (:54) | 1 |
| 4 | `.claude/hooks/routine-guard.mjs` | two Edits: the `mcp()` call at :460 (it gains the hook input) and one span over `mcp()` and the merge check (:1501 to about :1600); a whole-file Write (1,622 lines) would be one ask but unreadable | 2 |
| 5 | `.claude/hooks/routine-guard.test.mjs` | two Edits: the fixture at :45 (a toolchain row) and the new cases appended at the end; the file is 3,875 lines, too long for one readable Write | 2 |
| 6 | `.claude/hooks/guard.mjs` | one Edit (the allow output, :40-48) | 1 |
| 7 | `.claude/scripts/track.mjs` | one Write (new file) | 1 |
| 8 | `.claude/scripts/track.test.mjs` | one Write (new file) | 1 |
| | `.claude/NIGHT_SHIFT.md`, `.claude/settings.json` | not changed | 0 |
| | **Total** | | **about 10** |

A fix found by a build review costs one more ask per file it touches. Recording his answers in `docs/DECISIONS.md` by script, and the docs-keeper's STATUS and ROADMAP lines, are not protected files and need no ask.

## Review findings and where they landed

**security-auditor** (.gate/review-tracks/security-auditor-plan.md). Its review gave ids and class words only. The OLD/NEW texts in this plan for its findings are my wording, drafted from the rule files.

| Id | Severity | Where it landed |
|---|---|---|
| SA-RT-1 | Critical | 3e GUARD-MARK; 4a restated; risk 2; MERGE-MARK |
| SA-RT-2 | High | advisor.md step 6 after-check (stateless, ids and times); 4a; his Advisor's own list |
| SA-RT-3 | High | 3f "His go keeps" (integrity, not origin) and the script's DECISIONS handling; 3g row 6 |
| SA-RT-4 | High | no allow line; 3e MERGE-PROMPT, now (d) by his decision (it was (c)): the guard's prompt skip without the file-list check; the lost layer is risk 1 |
| SA-RT-5 | Medium | 3a "Which copy runs"; empty diff exits 2 |
| SA-RT-6 | Medium | 3a raw diff and risk-tier-2 floors; name comparison |
| SA-RT-7 | Medium | 3c: every package.json, lockfile and CI-deciding file is full; ruleset follow-up (For Mustafa) |
| SA-RT-8 | Medium | 3b light security triggers; 3d R3 |
| SA-RT-9 | Medium | 3g row 10; advisor.md :178-184, :92-104, :266-268; security-auditor.md:54; section 5 item 14 |
| SA-RT-10 | Medium | 3f eyes paths and proofs |
| SA-RT-11 | Medium | 3e way (1); CLAUDE.md merge sentence; 4b and 4c; WAKE-END (a), his decision |
| SA-RT-12 | Low | the check names the PR number; never edited; posted after the gate run; light rules exact; T-MARK's raw-API route; wake events are data; guard edits before R4-1 (section 7) |

**The advisor agent** (.gate/review-tracks/advisor-plan.md):

| Id | Severity | Where it landed |
|---|---|---|
| H1 | High | 3e "The PR Advisor's own merges"; advisor.md :92-104, :200, Never; 4c |
| H2 | High | 4a; risk 2; MERGE-MARK; 4d |
| H3 | High | 3f; 3g row 6; the merge check names each entry's sha256 |
| H4 | High | 3e way (1); 3g row 12; advisor.md :167-169 |
| M1 | Medium | section 3's OLD/NEW blocks for advisor.md :33-34, :63-64, :71-74, :174, :266-268, :304-305; security-auditor.md:54; NIGHT_SHIFT.md:77 stays, with why |
| M2 | Medium | CLAUDE.md :107 NEW points to "His go keeps" |
| M3 | Medium | 3e way (1) covers allow-listed `ready` PRs; CLAUDE.md :92-94 and :103; advisor.md :71-74 |
| M4 | Medium | section 5 items 8, 10 and 13 |
| M5 | Medium | 3b: no round only for risk tier 0 |
| M6 | Medium | 3f eyes paths (crates/sim/, packages/sim-worker/, content/) |
| M7 | Medium | 3a raw modes; base and head in the output |
| M8 | Medium | advisor.md step 6 after-check: stateless, a fixed 24-hour look-back, no marks needed |
| M9 | Medium | 4a: one paste per PR until his Advisor chat is seen to post; T-MARK without manual steps; For Mustafa |
| M10 | Medium | 4b night-shift PRs |
| L1 | Low | 3d: 11 Highs plus 2 candidates |
| L2 | Low | section 1 and 3g: DECISIONS:1348; 3a `toolSource` prompt |
| L3 | Low | 3b fallback reviewer (test-engineer) |
| L4 | Low | 3b light security triggers; 3d R3 |
| L5 | Low | section 6 recount (for (d) too); the smaller version was offered and not chosen |

**Round 2 Lows and conditions (for the build):**

| Id | Severity | Where it landed |
|---|---|---|
| SA-RT2-1 | Low | table half dropped: under (d) nothing in the guard reads the table (3e, "The guard reads no table"); the other half (the guard's own file equals main's) kept as a build Low for security-auditor |
| SA-RT2-2 | Low | advisor.md step 6 after-check: a merge counts as the routine's own only when its note names the head and every file passes the allow-list |
| SA-RT2-3 | Low | GUARD-MARK scans every string recursively; MERGE-PROMPT (d) condition 4 and 4a: the comment list read whole, exact match, no trimming or folding |
| SA-RT2-4 | Low | 3b: `legacy/` added to the light security triggers |
| C1 | High | superseded by his decision MERGE-PROMPT (d): PASS now means everything his rules need for that head has happened, his go included; `FOR HIS GO` kept as a status line only (3e "What PASS means") |
| C2 | process | met for (c) by security-auditor's re-review; for (d), both reviewers read this update before the build |
| C3 | Low | dropped as moot: the guard reads no table |
| C4 | Low | as SA-RT2-2 |

Nothing was left out. The decisions went from ten to four, and all four are now answered (section "Decisions"):
- TRACK-DOUBT, REVIEWER-SLOTS, LIGHT-EFFORT and GUARD-TOOLCHAIN became defaults inside PLAN-GO.
- PRADV-TRACK is settled (a).
- ACCT-LINES is dropped from this plan (section 5, item 12).

## Decisions

All four are answered by Mustafa in the Guard fix chat on 2026-11-26. He typed "PLAN-GO: a / MERGE-PROMPT: A / WAKE-END: a"; asked once to confirm MERGE-PROMPT (a) against the recommended (c), he asked what a guard load failure means, and then typed "d" for the option offered to him as (d).
- **PLAN-GO (a):** the full version, now about 490 lines, stop line about 980. Its four defaults stand, as none was named: TRACK-DOUBT (a), REVIEWER-SLOTS (a), LIGHT-EFFORT (a), GUARD-TOOLCHAIN (a).
- **MERGE-MARK (a), a default inside PLAN-GO:** GUARD-MARK now (the guard refuses the check's line in every GitHub-tool write and every edit of a comment that carries it, and refuses subagent merges); his Advisor's own list of the checks it issued. MERGE-MARK (b), the app-mark test, stays a later follow-up, asked only if T-MARK shows the mark holds.
- **MERGE-PROMPT (d):** the guard skips the prompt in a day session only when the expected head is the PR's current full head, gate, smoke and toolchain are green on it, the caller is not a subagent, and a real, unedited PASS comment from `Mergincanli` names the PR and that head, posted after the gate run. No file-list check: his Advisor's PASS is the one gate, written only after what his rules need for that head has happened. Otherwise, an error in the new conditions, or a guard that fails to load: the prompt. A failed base merge check (expected head, checks, fork) and a subagent merge stay refused, as the base check refuses today. Option (d) came from him: after the explanation of a guard load failure, he proposed keeping the four checks without the file check, and it was offered back to him as (d).
- **WAKE-END (a):** if the building chat has ended, the PR waits for one line from him to a new chat on that lane.

## Decisions needed

None. All four decisions are answered. The ruleset click for `toolchain` stays a follow-up (For Mustafa, item 2).

## For Mustafa

1. The build asks you about 10 times: once per kit or gate file, twice for the guard and twice for its tests (one change near the top, one at the merge path or the end), each showing the exact text (section "Build steps and his asks"); run that session in Accept edits or Manual mode. A fix from a build review adds one ask per file it touches.
2. Later, a follow-up click, not now: add `toolchain` (from GitHub Actions) as a required check in "Shamble Ruleset 1", so GitHub itself also refuses a merge without it.
3. From the merge of this change-set on: give your Brave check or your yes to your Advisor for any PR that needs it; its PASS then carries your go to the merge. Your go in a builder chat still works too.
4. Nothing else. T-MARK needs no step from you.

### Critical files for implementation
- /home/user/Shamble-Frontlines/CLAUDE.md
- /home/user/Shamble-Frontlines/.claude/agents/advisor.md
- /home/user/Shamble-Frontlines/.claude/agents/security-auditor.md (one line)
- /home/user/Shamble-Frontlines/.claude/hooks/routine-guard.mjs (and routine-guard.test.mjs)
- /home/user/Shamble-Frontlines/.claude/hooks/guard.mjs (the allow output)
- /home/user/Shamble-Frontlines/.claude/scripts/track.mjs (new; tests in track.test.mjs)

## Plan reviews, final state (2026-11-26)
- security-auditor: REVISE (SA-RT-1 Critical, SA-RT-2/3/4 High, SA-RT-5..11 Medium, SA-RT-12 Low), then APPROVE of the
  revised plan with MERGE-PROMPT (c), which it read with the guard's new parts (GUARD-MARK, the prompt skip with its
  file-list and protected-path test, guard.mjs's allow output, the table read from disk, the refusal of subagent
  merges). Four Lows for the build: SA-RT2-1..4.
- The advisor agent: REVISE (H1-H4, M1-M10, L1-L5), then APPROVE of the (c) design with C1 (PASS only when nothing in
  "His go keeps" applies; otherwise FOR HIS GO) and C2 (met), C3 and C4 at build.
- **Not yet reviewed:** MERGE-PROMPT (d), his decision after both approvals. It removes the file-list check that
  security-auditor's APPROVE leaned on, and it supersedes C1 as written. Both reviewers read this update before the
  build; until then, this plan is not approved for (d) by either reviewer.
