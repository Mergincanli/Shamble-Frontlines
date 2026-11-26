# Advisor report: ci-preview-handover (2026-11-26, branch `claude/keen-tesla-v807cf` standing in for `cs/preview-handover`, base main 2483e9a)

A small CI pull request by the chat Mustafa calls CI Builder (session_019YNMUUemXge56777MiUWFz, Opus 5.5), on his block
of 2026-11-26, 13:20 Amsterdam: the hand-over step lane 2 (Builder V2) asked for on issue #59 (comment 7172595904 of
2026-11-25, 19:39 UTC, information, not his word), with TOKEN-WORDS and FIREFOX-REPLAY in the same pull request. Risk
tier 2 (a `.github/` file). No pull request yet: its title and text are in the notes, and his main Advisor opens the
draft and watches the first run.

## Outcome
- **The hand-over:** two steps at the end of job `toolchain`, after the network cut, for pull requests from this
  repository only.
  - A check that the tested production build (`.gate/dist-production`) holds only plain files and folders, none
    named with a leading dot. It uses absolute paths and prints its count. It is not a boundary.
  - The upload of that build as the artifact `preview-site` (`actions/upload-artifact` v7.0.2 at
    cf430e030ddbb5b0abf93d22962f4752f3646cd9; 7 days; no hidden files; an empty upload fails), for lane 2's
    `preview.yml`.
  - Two comments that would have turned false were corrected: line 4, and the step name at line 130.
- **TOKEN-WORDS:** "The only run step with the token" (line 94).
- **FIREFOX-REPLAY:** the firefox entry runs the replay test beside the rule tests (`rules/**/*.spec.ts`,
  `replay/**/*.spec.ts`). It passed on its first run in Firefox, in the container (161) and on GitHub (160).
- **Size:** 35 lines added and 5 removed in the workflow; 6 added and 4 removed in the test configuration (50
  changed lines outside notes and docs, against his stop line of 300; `git diff --numstat` against main 2483e9a).
  (Ask 2 said 33 added: its count left out the two added blank lines. The text he approved is unchanged.)

## Plan and approval
- **Plan revision 1** (eb62201): the advisor agent's REVISE. Its High: steps run after the network cut for the first
  time, so two comments turn false, and the guard's programs and the upload's code are reachable from inside the cut.
  It added five more changes.
- **Revision 2** (e59b0c2): APPROVE, with three Low conditions:
  - N1: the request's "files under .gate/ may count as hidden" is not how the action works, and is corrected in the
    text.
  - N2: the read-only copy of the action's code is a download that was never used, deleted at the merge.
  - N3: the report describes the final shape.
- **Departure:** the checkpoint script was never run, since it pushes to the local branch's name. The card and notes
  were committed by path and pushed with `git push origin cs/preview-handover:claude/keen-tesla-v807cf`.

## His answers in this chat (word for word in `docs/DECISIONS.md`)
- **FF-REFETCH (a), 14:00 Amsterdam:** the same Firefox 161 file again, for the container's trial only.
- **UPLOAD-SOURCE (a):** the action at that pin, for the hand-over step only.
- **What ask 2 must hold:** the texts with their sha256, the tie line confirmed by security-auditor, the guard and the
  hidden-files setting each as its own choice, and security-auditor's answers on the steps after the cut.
- **CI-WRITE-3 (a), GUARD-STEP (a), HIDDEN-FILES (b), 15:05 Amsterdam.** With them:
  - The Actions setting is tested by the first run.
  - CUT-ESCAPE stays accepted, its later fix a STATUS follow-up.
  - A red on the new steps or on Firefox's replay test comes to him first.

## What changed
- **`.github/workflows/toolchain.yml`:** written at 14:09:14 UTC on CI-WRITE-3 (a). The permission mode was read right
  before the write: `acceptEdits`. The file is then sha256 d017b83b82d412befb1cbb983a93215440e2b0e1608d2c8c749445db53e87fd8,
  his value.
- **`packages/e2e/playwright.config.ts`:** sha256 35abfd54fddb43d7bf8077b281c153c8625c22d863d04922afd0b7b1ecfe80c8, his
  value.
- **Docs (docs-keeper):** `docs/DECISIONS.md` (his four messages, copied by script and checked byte for byte),
  `docs/STATUS.md`, `docs/STATUS_ARCHIVE.md` (the block "From ci-preview-handover"), `docs/REPO_MAP.md` and
  `docs/ROADMAP.md`; this report.

## Evidence
- **The tie of v7.0.2 to its commit:** security-auditor's own `git ls-remote`:
  `cf430e030ddbb5b0abf93d22962f4752f3646cd9	refs/tags/v7.0.2`. It is a lightweight tag.
- **The action's own text at that commit** (a read-only copy in the scratchpad, never run):
  - no token input (`action.yml:4-53`, `src/upload/input-helper.ts:8-58`);
  - symbolic links followed (`src/shared/search.ts:16`);
  - dot names dropped unless asked (`:19`; a test on each item from the build folder down,
    `dist/upload/index.js:130173`);
  - the runner's upload key read from its environment (`:82477`).
- **The guard, tried in the container** on the real production build: exit 0, "5 files, 892209 bytes". Seven
  plants each exit 1 with their own line: a symbolic link, a fifo, an empty build, the build folder as a link,
  `.gate` as a link, a dot-named file, a dot-named folder (`plant-test.sh`).
- **Firefox 161 in the container:** 3 passed, the replay test "first-loop.rec: 1800 steps, every shared hash as in
  the file". Chromium: 17 passed. The test lists are Chromium 17, Firefox 3, node 1. The e2e type check passes.
- **GitHub, the push run on d12e3a8:** toolchain 45836151624 and gate and smoke 45836152061, all green.
  - The set-up passed with the upload action named, so the Actions setting allows it.
  - The two new steps were skipped on a push, as meant.
  - The runner's Firefox 160 gave "3 passed (12.5s)", with the replay test "first-loop.rec: 1800 steps, every shared
    hash as in the file" (815 ms).
  - The job took 5 min 45 s, against its 15-minute limit.
- **Not yet shown:** the two new steps running. The pull request run, once the draft is open, is their first: the
  guard's count, the upload's "there will be 5 files uploaded", its digest and the artifact id.

## Reviews (Opus 5.5, effort high; one round on the tried texts, before ask 2)
- **security-auditor:** nothing stops the change; no Critical or High.
  - SA-1 Medium: code inside the cut can change what later steps run, the upload's code included. This is the
    existing accepted risk CUT-ESCAPE, not new with this change. The fix is later: the cut's code as its own
    throwaway user, with its processes ended with the step.
  - SA-2 Low: CUT-ESCAPE's acceptance now also rests on lane 2's preview treating the build as data, as read on lane
    2's branch at 9ded5a8. This went to the archive as PH-SA-2.
  - SA-3 to SA-6 (Low), the wording and the dot-name check: taken into the text he approved.
  - SA-7: two citations corrected.
- **infra-engineer:** APPROVE.
  - The one thing it could not read, the Actions setting, the push run settled.
  - IE-1 to IE-3 overlap with security-auditor's points and are in the text.
  - IE-4, a re-run with the same artifact name, is a follow-up (PH-IE-4).
  - IE-5: compare file counts, not bytes.
- **The advisor agent:** the plan (above). Its read of the final head comes with the head's full id.

## Deviations and records
- **docs-keeper and the interim git rule:** docs-keeper reports one breach. A Python script of its own ran a read-only
  `git diff` inside itself, instead of git starting its own command. Nothing was written by it. Every other git and
  gh call started its own command.
- **Lines of the shared records moved or corrected (REC-OLD, OLD-LINES):** listed with their old and new text in
  `docs-keeper-old-new.md` in the notes, at the notes' last commit (the parent of the head).

## Not verified
- The two new steps running on GitHub: the pull request run is their first.
- The re-run case (PH-IE-4).
- That the runner's action folder is writable by the cut's user: inferred, not measured.
- Lane 2's `preview.yml` as it will be on main; it was read on lane 2's branch.

## Follow-ups (in `docs/STATUS_ARCHIVE.md`, "From ci-preview-handover")
- **CUT-ESCAPE-FIX** (his point 6).
- **PH-SA-2, PH-IE-4.**
- **PH-KEEP-NAMES:** step 2.3 keeps the workflow name `toolchain`, its path and the artifact `preview-site`.
- **PH-MERGE-REF:** the artifact is the build of the pull request merged into main, for lane 2.
- **PH-TIME, PH-ACTIONS-SETTING** (settled by the push run).
- **PH-DELETE:** the Firefox folder and the read-only copy of the action's code, deleted at the merge.

## Decisions needed
None.

## For Mustafa
1. Your main Advisor opens the draft from `.checkpoints/ci-preview-handover/pr-title.txt` and `pr-body.md` at the
   notes' last commit (the parent of the head, named in the chat), and watches the first pull request run. A red on
   the new steps or on Firefox's replay test comes to you first.
2. Your go for the head's full id, after the advisor agent approves it with gate, smoke and toolchain green on it.
   Nothing a player sees changes, so there is no browser check.
