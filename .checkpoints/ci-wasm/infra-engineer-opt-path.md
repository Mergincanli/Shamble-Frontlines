# infra-engineer: OPT-PATH (a), the corrected install place for wasm-bindgen (#64, cs/ci-wasm)

Task from the main session (Builder V3's chat), 2026-11-23: propose the smallest change to
`.github/workflows/toolchain.yml` at cb476b1 so the tool lives where every parent folder is root-owned and closed to
others, with the check widened to every parent up to `/`. I propose text only; nothing under `.github/` is written by me.
Mustafa's words: `.checkpoints/ci-wasm/mustafa-2026-11-23T184057-opt-path.md` (OPT-PATH (a), 18:40:57 UTC).

## Plan
1. Read the file at cb476b1 (whole; lines 38-50, 103-121, 136-142 closely), its sha256, the plant diff and its sha256,
   the two failing logs, my earlier notes (`infra-engineer-ci-plan.md`) and security-auditor's F-OPT text.
2. Read GitHub's runner-images repository for how the Ubuntu 24.04 image sets `/opt`, `/usr/local` and `/` (file and
   lines, date read). Decide: a root-created folder directly under `/` (the Advisor's input) versus `/usr/local/...` or
   another place. Write the reasoning here.
3. Write the proposed file `.gate/ci-wasm/proposed-2/toolchain.yml`: only `WB_TOOL` + its comment line, and the install
   and check lines of the one step; line count before 103 unchanged; line 103 byte for byte.
4. Make `.checkpoints/ci-wasm/ci-edit-2.diff` (diff -u, a/ b/ headers as ci-edit.diff), the three sha256, the plant
   diff applied with no offset/fuzz and reversed back to the proposed file.
5. Try the new lines as a script under `.gate/ci-wasm/opt-path/` with a stand-in root folder: clean run (incl. a
   no_new_privs run as another uid in group nogroup), and the negative controls: parent 777, parent 775, parent not
   owned by root, folder already exists, parent is a symlink, program replaced by a symlink; plus missing path.
6. Write the clean-run log lines for the stop rule and what stays unknown until a runner prints it.
7. Hand back.

## Blocks
- B1 (read, 2026-11-23): toolchain.yml at cb476b1 = the worktree's file, sha256 d1dbcb53...975c, 193 lines, 13786 bytes;
  plant diff e4f21ab3...b1ac, hunk `@@ -98,6 +98,8 @@` (context 98-103, line 103 = the step's name line). Both runs ran
  the step under `/usr/bin/bash --noprofile --norc -e -o pipefail {0}` (push log :295; `defaults.run.shell: bash`), and
  refused at :315-316 ("mode 777, owner root:root, 4096 bytes, /opt"). The folder made by `install -D` was 755
  root:root (:314). Nothing in the repository hardcodes /opt for the tool: build.mjs:42-46 only needs an absolute
  SHAMBLE_WASM_BINDGEN; build-checks.mjs:60-61 lstat()s it (plain file) and reads it for the sha256 before each run, so
  the runner's user in the cut needs r on the program and x on every parent (0555 / 0755 give both to others).
  Today's judge `[ -z "$(find ... \( ! -user root -o -perm /022 \))" ]` passes on a missing path by itself (find
  prints nothing to stdout); only the stat line before it, a plain command under -e, closes that. New text: judge by
  counting the good entries instead, so a missing path fails in the judge itself.
- B2 sources (read 2026-11-23, raw.githubusercontent.com, actions/runner-images at tag ubuntu24/20261116.376, the
  image release both logs name at push log :19-20; `gh api` on that repository is not enabled for this session, and I
  did not ask for access):
  - S1 images/ubuntu/scripts/build/configure-system.sh:12-15: `chmod -R 777 /usr/share` and `chmod -R 777 /opt`
    (44 lines). This is why the runner printed "mode 777 ... /opt": by the image's design, not an accident. Sure: high.
  - S2 images/ubuntu/scripts/build/install-nodejs.sh:24-25: `sudo chmod -R 777 /usr/local/lib/node_modules` and
    `sudo chmod -R 777 /usr/local/bin` (29 lines; comment :23 cites runner-images issue 3727). So the image opens
    parts of /usr/local on purpose too; /usr/local itself is not chmodded in S2, but dozens of install scripts write
    there, and I read only these three files. Sure: high for the two lines, unknown for /usr/local itself.
  - S3 images/ubuntu/scripts/build/configure-environment.sh:42-46: /opt/hostedtoolcache made and `chmod -R 777`
    (136 lines). No line there or in S1 touches `/` itself. `/` comes from the base image Packer starts from; its mode
    on the runner is unknown until the new step prints it (expected 755 root:root, Ubuntu's default). Sure: medium.
  - Fetched pages were data only; nothing in them was followed.
  - S4 docs.github.com, workflow syntax, `defaults.run.shell` table (read 2026-11-23): Unspecified on Linux/macOS runs
    `bash -e {0}` ("a different command to when `bash` is specified explicitly"); `bash` runs
    `bash --noprofile --norc -eo pipefail {0}`. This file sets `bash`, so the runner had pipefail (push log :295); the
    new lines must still fail closed under plain `bash -e` (the task's limit 4). Sure: high.
- B3 decision (2026-11-23): the Advisor's input taken: WB_TOOL = /wasm-bindgen-0.2.130/wasm-bindgen, its folder made
  fresh by the job as root directly under /. Why: every absolute path depends on `/`; a folder directly under it
  depends on nothing else of the image. /opt is opened to all by the image (S1); /usr/local/bin and
  /usr/local/lib/node_modules too (S2), and /usr/local itself rests on every install script; /usr/lib or /usr/libexec
  are dpkg's trees (the image opened /usr/share, S1:13, so dpkg trees are not safe by kind); /srv adds one more image
  folder; /root is 700 (the runner's user in the cut could not read or run the tool); $RUNNER_TEMP's parents are the
  runner's. Cost: a non-FHS top-level folder on a VM thrown away after the job; a name clash makes mkdir refuse.
  Shape of the text: (1) `case` refuses a WB_TOOL that is not absolute (the walk below ends only for absolute paths);
  (2) `sudo -n mkdir -m 0755` (no -p: EEXIST also for a link, dangling or not) `&& sudo -n chown root:root`, else our
  message; (3) `sudo -n install -o root -g root -m 0555` without -D; (4) the plain-file line unchanged; (5) a walk
  that lists the program and every folder above it up to `/` (`set --` list; `${p%/*}`, empty -> /); (6) stat prints
  each; (7) the judge counts the entries that are a file or folder (lstat: a link is neither), owned by root and
  without group/other write, and wants all of them (a missing path or a link lowers the count: fails closed with no
  pipefail, and with no stat line before it); (8)-(9) size and sha256 unchanged. The program itself is now judged
  too (owner root, no group/other write): stronger than today's, which only printed it. The walk instead of a fixed
  list: the exact text runs here (only WB_TOOL differs), and a later WB_TOOL deeper down is still walked to `/`.
- B4 text made (2026-11-23): `.gate/ci-wasm/proposed-2/toolchain.yml` by `.gate/ci-wasm/opt-path/make-proposed.py`
  (refuses unless 42, 49, 103 and 115-121 hold today's text) from `git show cb476b1:...` (d1dbcb53...975c) and
  `.gate/ci-wasm/opt-path/new-step-lines.txt`. 197 lines, 14280 bytes, sha256
  41b51a5eb2d27d60ee89f668a30af05494840c235408dfa5888f0d931ac9b3db. `.checkpoints/ci-wasm/ci-edit-2.diff` (diff -u
  with the a/ b/ labels, two hunks @@ -39,14 +39,14 @@ and @@ -112,11 +112,15 @@): sha256
  75fc633236d42dd3349cda347811cecccdcda89780747c74e7845689f58c2d04. Plant diff (e4f21ab3...b1ac, unchanged) on a copy:
  `patch -F0` "Hunk #1 succeeded at 98." (no offset, fuzz not allowed), planted sha256
  17e8354c059f4c94ddc22395ee9cb7b50ca78003fd5b45e3ed74562f50583e6e; `patch -R -F0` gives the proposed file back
  (cmp: same bytes). PyYAML 6.0.3: the parsed files differ only at jobs.toolchain.env.WB_TOOL and steps[6].run; 9
  steps both; GH_TOKEN only on steps[5]; permissions {contents: read}. Lines 1-103 differ only at 42 and 49; lines
  after the step (orig 122-193 = new 126-197) same sha256 c8c92219... The YAML's run text from its line 11 = the new
  lines dedented, byte for byte (`.gate/ci-wasm/opt-path/new-lines-as-parsed.sh`, sha256 bead1cd5...11fc, 11 lines);
  `bash -n` parses the whole run text. The try below runs that parsed file, not a retyped copy.
