TASK: Advisor ruling on pass 3 of A3 (kit-v3-3, PR #38): P3-1..P3-8, verifier P3-1/P3-2, where to fix, shapes, texts, check after.

## Plan (blocks)
B1 read mustafa-words.md, advisor-fix-pass2.md, advisor-fix-round.md
B2 read security-auditor-pass3.md, verifier-pass3.md
B3 read skeptic notes P3-1,2,3,5
B4 read code: parseShell, bash(), dateCheck, night commit, DIFF_NAMES, writtenBy clone; builder-pass2-*.md
B5 check probe outputs cited (spot), night texts (NIGHT_SHIFT.md, CHECKPOINTS.md, checkpoint.mjs commit form)
B6 write ruling: verdict, table, rows/mutants, texts, check after, Decisions needed, For Mustafa
DONE marker at end.

## Notes
- B1 done: WT-NIGHT (a) condition; pass-3 scope (routine full depth, day minor -> limit/follow-up); fix-round L1-L15, pass-2 L9/L11 widened, L16-L18; SD-1 'alone' rule night-only; A-4 follow-up. Next B2.
- B2 done: SA FAIL: P3-1 comment (#+quote swallows lines; night switch + CLAUDE.md writes on night/pr-advisor), P3-2 heredoc delimiter `<<E\ O`/`<<'E O'` (+D5 backslash-newline then `# '`), P3-3 ANSI-C $'\'' via date carrier (git/gh blocked by $ rule; dateCheck accepts any +word); P3-4 bg fetch & / run_in_background; P3-5 commit pathspec w/o -- (-i, -o, --pathspec-from-file, glob, :(top)); push check backstops (protectedInBranch origin/main...HEAD); P3-6 .gitmodules ignore=all hides gitlink; P3-7 clone cloneDir every positional; P3-8 stash&&switch main asks (no code). V-3, SD-1 wiring, SD-2 right. Verifier PASS WITH NOTES: suites 864/864, main's file 180/192 named, mutants 48/48, gate PASS; its P3-1=P3-5 (vcwd join too: `-- settings.json` from .claude/ null), P3-2=P3-7. Next B3.
- B3 done: skeptics: P3-1 major; ordinary apostrophe comment ("# what's changed") hides next line -> accidental reach, day too; hidden push to main stopped by branch protection (STATUS:785); add # rule in sub() closer scan; P3-2 major, accept `<<\EOF`; no routine text uses heredoc; P3-3 critical defensible, dateCheck alone is a carrier patch; P3-5 major + `-- .`, `.`, globs, :(top), '*.md' after -- (literal protectedRel); push backstop keeps commit local. Next B4 code.
- B4 part: code read: parseShell :140-242 (no #, delimiter loop drops quotes/backslash and stops at whitespace :221, `\` takes next char incl newline :186, ' to next ' :187, double-quote loop handles $( and ` but not ${ ); bash() :497-560 role pre-checks only subs/unterminated; dateCheck :476 any +word; NIGHT_ALONE :780; commit check :881-895 (named only after --, literal protectedRel); DIFF_NAMES :914; clone :617-630 made = pos.map(cloneDir).
- FINDING A-5 (mine, candidate): double-quoted "${x:-"}"..." nesting not modelled (parser closes outer quote at inner "); to probe in real bash. If true, the four named desyncs are not the full set -> denylist insufficient, argue lexical allowlist for roles.
- adv3-probe.out/adv3-probe2.out (real bash 5.2.21): A5 nested "${x:-"}"}" agrees (no desync found); A7 $(( )) and A8 case ) give the parser MORE commands (safe direction); A10 brace expansion CLAUDE.{md,x} not modelled (word-level differential); A12 backslash-newline: same count, word differs.
- Routine programs are only git, gh, npm ci, node/python on main's tracked scripts, date (routineProgram :759-770): a strict plain-form lexical gate for roles costs ~nothing.
- FINDING A-6 candidate: `>&word` (non-digit) = bash writes stdout+stderr to file; parser :226 skips digits then leaves the word as argv -> redirect missed. Probe next.
- FINDING P3-9 (mine, CONFIRMED, major, routine kit write; pre-existing on main 12d7961): `>&word` with a non-number word is bash's `&>word` (stdout+stderr to the file); parseShell :226 skips digits/'-' then leaves the word as an argv word, so no redirect is checked. adv3-probe4.out (head) and adv3-probe4-main.out (main), real git 2.43 + bash 5.2.21: night `git status >&CLAUDE.md` null -> CLAUDE.md overwritten; pr-advisor `git log -1 --oneline >&CLAUDE.md` null -> overwritten; night `git status >&.claude/settings.json` null -> settings.json overwritten with non-JSON (live settings file); day null too. Controls: `git status > CLAUDE.md` deny; `2>&1` null. Fix: parseShell reads `>&word`/`>>&`? as a file redirect when word is not digits or '-'; the roles' plain-form gate allows only digits or '-' after >& and <&.
- B4 done (code + builders' notes skipped: reviews cite them sufficiently; the shapes below do not depend on them). B5 done: night texts: NIGHT_SHIFT.md prescribes no commit form (step 7 "commit and push the checkpoint after every step"); checkpoint saves go through checkpoint.mjs (commit-tree + push of `${sha}:refs/heads/${branch}`, :132, :146-175), not git commit; the saver fetches nothing. Next B6 write ruling.
- Decisions in B6: fix inside A3 (fix pass 3); routines: plain-form lexical gate R-PLAIN before parseShell (allowlist, by construction; no heredocs for roles); parseShell learns comments, backslash-newline, $'..', quoted heredoc delimiter, >&word (day reads as bash; routines do not depend on it); dateCheck word rule; night commit option allowlist; P3-4 roles git foreground; P3-6 fix in named C6 edit; P3-7 fix (regression of SD-5 on a control); P3-8 name; corpus C-PLAIN now (routine form), day corpus follow-up; pass 4 scoped; no decision for Mustafa.

## Ruling (B6)

**Verdict: PROCEED with fix pass 3 inside A3, then pass 4, scoped below.** No question for Mustafa. A3 stays a draft.
The four upheld majors (P3-1, P3-2, P3-3, P3-5) and one more of the same kind that I found while checking the shapes
(P3-9, below, confirmed on real bash) are fixed now. The routine fix is one rule, by construction: a routine's shell
line must be in a plain form the guard reads exactly as bash does, or it is refused. That replaces the hunt for the
next parse slip with a fixed, small language.

What I read: main's advisor.md; mustafa-words.md; my fix-round and fix-pass-2 rulings; security-auditor-pass3.md;
verifier-pass3.md; the four skeptic notes; routine-guard.mjs at 01b158c (parseShell :140-242, dateCheck :476-482,
bash() :497-560, routineProgram :759-770, NIGHT_ALONE and git() :780-905, DIFF_NAMES :914, the clone branch
:617-630); NIGHT_SHIFT.md:26-62; CHECKPOINTS.md (commit lines); checkpoint.mjs :132, :146-175 (the saver commits with
commit-tree and pushes `<sha>:refs/heads/<branch>`; it runs no `git commit` and fetches nothing); STATUS.md:715-735.
What I ran: four small probes against the guard at 01b158c and main 12d7961 with real git 2.43.0 and bash 5.2.21
(adv3-probe.out, adv3-probe2.out, adv3-probe3.out, adv3-probe4.out, adv3-probe4-main.out). What I did not do: run the
suites or the gate (the verifier did: 864/864, gate with smoke PASS, mutants 48/48); I took the builders' notes as
cited by the reviews.

### 1. Where: inside A3, now (fix pass 3)
- **A3 cannot merge without these fixes anyway.** They defeat A3's own night switch check (WT-NIGHT (a), Mustafa's
  condition), SD-1's "alone" rule, the commit check that pass 2 rewrote, and L16's own words. A separate change-set
  would not let A3 merge any sooner.
- **The fixes sit on A3's code.** The night switch check, NIGHT_ALONE, DIFF_NAMES and the rewritten commit check exist
  only on A3's branch. A fix branched off main would edit parseShell, bash() and the commit check, the lines A3
  rewrote, in the same file. That means conflicts, a re-review of the rebased A3, and two of Mustafa's reconfirms
  instead of one.
- **Main's exposure until the merge is low.** The Night queue has no unchecked item, so tonight runs nothing. The PR
  Advisor reviews only ready PRs from this repository's own sessions (forks are skipped). Every bypass needs the
  routine agent itself to type the line; nobody outside types into a routine's shell. GitHub's branch protection on
  main (STATUS.md:785: PR plus gate and smoke required, no force push) stops a hidden push to main. A kit change pushed
  to a `cs/` branch still meets the PR review and Mustafa's go. No hold on the routines is needed.
- **Fallback (split condition):** if pass 4 finds a routine-side FAIL that the plain form cannot close by narrowing
  (section 5), the routine half (R-PLAIN, the commit allowlist, the `>&` fix and P3-4) leaves A3 as its own urgent
  change-set off main, and A3 rebases onto it. Nothing in that half depends on A3's day code.

### 2. Finding by finding
| Finding | Severity (ruled) | Ruling | Shape |
|---|---|---|---|
| P3-1: an unquoted `#` starts a comment for bash, not for the parser; a quote in the comment hides the next lines (night switch, CLAUDE.md writes by night and pr-advisor; on main too) | major (the skeptic). It is also accidental: "# what's changed" is enough. | **Fix now** | Routines: R-PLAIN (no unquoted `#` at all). Day: parseShell learns comments: an unquoted `#` that starts a word skips to the end of the line, and the newline itself is not skipped. The same rule goes into sub()'s closer scan (:158-169). |
| P3-2: here-document delimiter with a quoted or escaped space (`<<E\ O`, `<<'E O'`); backslash-newline before `# '` (D5) | major (the skeptic) | **Fix now** | Routines: R-PLAIN refuses every here-document and here-string (`<<`, `<<-`, `<<<`). No routine text uses one, and routine command substitution is refused already. Day: the parser reads the delimiter as one shell word: quotes and backslashes honoured, quoted whitespace kept, quotes removed afterwards. So `<<EOF`, `<<'EOF'`, `<<"EOF"`, `<<-EOF` and `<<\EOF` work as bash reads them. An unquoted backslash-newline is a line continuation: both characters are removed, outside quotes and inside double quotes, as bash does. |
| P3-3: ANSI-C `$'...'` read as `$` plus a plain single quote (`$'\''`); date is the carrier | major. "Critical" is defensible, and the action is the same either way. | **Fix now** | Routines: R-PLAIN (no `$` outside single quotes). Day: the parser reads `$'...'` as one quoted string in which a backslash escapes the next character. dateCheck: `+FORMAT` only as `/^\+[A-Za-z0-9%:_ .,\/-]*$/`, the word after quote removal (a second layer: a date word cannot carry a separator even inside quotes). |
| P3-9 (new, mine): `>&word` with a word that is not a number is bash's `&>word` (stdout and stderr into the file); the parser skips the digits and leaves the word as an argument, so no redirect is checked | **major** (a routine's kit write on one ordinary line, no disguise needed; on main too) | **Fix now** | adv3-probe4.out (head) and adv3-probe4-main.out (main 12d7961), real bash: night `git status >&CLAUDE.md`: no deny, CLAUDE.md overwritten. pr-advisor `git log -1 --oneline >&CLAUDE.md`: no deny, overwritten. Night `git status >&.claude/settings.json`: no deny, the live settings file overwritten with text that is not JSON. Day: no ask. Controls: `> CLAUDE.md` is refused; `2>&1` passes. Shape: the parser reads `>&` and `<&` followed by anything but digits or `-` as a file redirect (the target goes to redirects). R-PLAIN allows `>&` and `<&` only before digits or `-`. |
| P3-5 (and the verifier's P3-1): the night commit commits paths named without `--`, through `-i`/`-o`/`--pathspec-from-file`, and through pathspec forms after `--` (`.`, globs, `:(top)`, a path relative to a subfolder); the push check still stops the push | major (wider than reported, per the skeptic) | **Fix now, by construction** | A night `git commit` takes only these words: `-m <message>`, `--message <message>`, `--message=<message>`, `-a`/`--all`, `-q`/`--quiet`, `-s`/`--signoff`, `--allow-empty`, and short bundles of the letters `a`, `q`, `s` with an optional `m` last (`-am`, `-qam`; the message is attached or the next word). Any other word is refused, whether a positional word, `--`, `--pathspec-from-file`, `--pathspec-file-nul`, `-i`, `-o`, `-p`, `--amend`, `-C`, `-F` or `--fixup`: "routine (night-shift): git commit here is only git commit [-q] [-a] [-s] [--allow-empty] -m <message>; stage files with git add in its own call first". The guard then lists, with DIFF_NAMES, exactly what git commits (the staged list, plus the unstaged list with `-a`), so no pathspec is ever interpreted. The `named` list and its vcwd join go away. The night texts need nothing else: NIGHT_SHIFT.md prescribes no commit form, and checkpoints are saved by checkpoint.mjs (commit-tree), not by `git commit`. |
| P3-4: a background `&` or run_in_background on an earlier call's `git fetch` can move a ref while the next call's lone switch is checked | minor, routine side | **Fix now (cheap)** | Every role: a line with a background `&` (a new `background` flag from parseShell, apart from `forks`) that holds a git command is refused, and so is a Bash call whose tool_input has run_in_background true and whose line holds a git command: "routines run git in the foreground". The background saver (`node … checkpoint.mjs watch`, run_in_background) stays allowed: it is node, and it fetches nothing. L18's text is updated (section 4). |
| P3-6: a branch's `.gitmodules` `ignore = all` hides a gitlink added at a kit path from DIFF_NAMES | minor (the effect is an empty folder at a new kit path) | **Fix now, inside SD-2's named C6 edit** | DIFF_NAMES gains `--ignore-submodules=none`. It overrides `.gitmodules` and `diff.ignoreSubmodules`. It is one word, at every caller (protectedHit for the night switch, the night push, checkpoint save and watch, and the day main folder; and the commit lists). main's fakeIo keys change again: name each case of main's test file that fails because of it. |
| P3-7 (= the verifier's P3-2): SD-5's clone targets add cloneDir of every positional word, so `git clone <O> .gate/tools` and `git clone https://…/tools.git <dir>` ask | minor, day side, but **a regression of a pass-2 fix on the task's own control** (clone into `.gate/`) | **Fix now** | Targets: `pos.slice(1)` and the git-dir values, as now. Add `cloneDir(pos[0])` only when `pos.length === 1`. When the line holds an option the guard does not know, also add `cloneDir(pos[1])`. Unknown means: it starts with `-`, has no `=`, is not in CLONE_VALUE, and is not one of git 2.43's documented clone flags: `-q`, `--quiet`, `-v`, `--verbose`, `--progress`, `-n`, `--no-checkout`, `--bare`, `--mirror`, `-l`, `--local`, `--no-local`, `--no-hardlinks`, `-s`, `--shared`, `--dissociate`, `--recursive`, `--recurse-submodules`, `--single-branch`, `--no-single-branch`, `--no-tags`, `--tags`, `--shallow-submodules`, `--no-shallow-submodules`, `--remote-submodules`, `--no-remote-submodules`, `--sparse`, `--reject-shallow`, `--no-reject-shallow`, `--also-filter-submodules`. An unknown option may take the next word and shift the URL; that is the case SD-5 guards. (The verifier's "last positional only" would still ask on `.gate/tools`.) |
| P3-8: `git stash && git switch -q main` from the main folder now asks | nit, the shape I ruled (a fetch earlier on the line can move local main) | **Name it, no code** | Add it to the report's list of moved cases, by name, and to STATUS's SD-3 line (section 4). |
| Auditor's class note: a differential corpus against real bash, plus a cross-check by construction | — | **Routine half now, day half as a follow-up** | R-PLAIN is the cross-check by construction. Its claim (on the plain form, the parser's commands, words and redirect targets equal bash's) is proven now by the corpus test C-PLAIN (section 3). The day differential corpus over full bash is a follow-up (FU-P3a): the day side is a seatbelt, and L16 names what it misses. |

#### R-PLAIN, exactly (routines only; checked on the raw text before parseShell, in a function of its own, so a parser slip cannot turn it off)
A routine's Bash line is refused with "routine (<role>): write the command plainly (<what was found>): one command
per line or joined with && ; values in quotes; no comments, $, backslashes, here-documents, braces, brackets or
parentheses" unless all of these hold:
1. **Outside quotes:** only ASCII letters and digits, space, tab, newline, the characters `_ - . / : = @ % + , ^`, a
   `~` that does not start a word and does not follow `=` or `:`, and these operators: `&&`, `||`, `;`, `|`, `|&`,
   `&`, the redirects `>`, `>>`, `>|`, `<`, `&>`, `&>>` (each may follow a file-descriptor number with no space), and
   `>&` or `<&` followed only by digits or `-`. So these are refused: `#`, `$`, a backtick, a backslash, `( ) { } [ ]`,
   `! * ?`, a quote mark left unclosed, `<<` in every form, `<>`, carriage return and every other control character.
2. **Inside single quotes:** anything except a control character other than newline.
3. **Inside double quotes:** anything except `$`, a backtick, a backslash, and a control character other than newline.
4. Every quote is closed.
Why this is enough: in this form there are no comments, expansions, escapes, here-documents, groups, subshells,
arithmetic, `case` patterns or brace expansion. So bash splits commands only on the listed operators and on newlines,
and splits words only on spaces and tabs, exactly as parseShell does. Routines run only git, gh, `npm ci`, node or
python on main's tracked scripts, and date (routineProgram :759-770). Their lines need nothing outside this form: the
step-0 line, the NIGHT_NPM line, the checkpoint save and watch lines with quoted `-m` and `--trailer`, `git commit -m
"…"` with apostrophes, `#` or `<…>` inside the quotes, and `date +%H:%M`. `gh api` paths with `{owner}` must be
quoted.

#### parseShell, for the day (routines no longer depend on it)
Five rules, so the day reads lines as bash does: comments; an unquoted backslash-newline is a continuation; ANSI-C
`$'…'`; a here-document delimiter read as one quoted word; `>&word` and `<&word` as file redirects. Anything else it
does not model stays under L16 for the day.

### 3. Rows and mutants each fix needs
RB marks a real-bash row: run on a scratch repository (real git) to show the effect on the old guard (01b158c; the
probe outputs already show most) and the deny or ask on the new guard.

**R-PLAIN (routines)**
- Deny: the auditor's H1, H2, H5, H6 (night) and H7 (pr-advisor); the skeptic's A1 `git status # what's
  changed⏎git switch -q cs/evil` (RB), A2 (`#'` with no space), A3 and A4. D1 to D5 (D1 and D4 RB). A plain here-document
  on a role line (`git commit -F - <<EOF⏎m⏎EOF`). Q5 and Q6 (night) and Q7 (pr-advisor) (Q6 and Q7 RB). `date +$'x'`.
  P3-9's R1, R2 and R3 (R1 and R3 RB). `git add CLAUDE.{md,x}`.
- One deny row for each refused construct: unquoted `#`, `$`, backtick, backslash, `(`, `)`, `{`, `}`, `[`, `]`, `!`,
  `*`, `?`; a `~` that starts a word; `<<`, `<<-`, `<<<`, `<>`; CR; another control character; `>&word`; `$`, a
  backtick or a backslash inside double quotes; an unclosed quote.
- Null (the controls; the routine checks after the gate apply as before): pr-advisor's step-0 line; the NIGHT_NPM
  line; `node .claude/scripts/checkpoint.mjs save kit-v3-3 -m "step 2: what's next" --trailer "Claude-Session:
  https://claude.ai/code/session_x"`; `git commit -q -m "fix #16: it's done"` (staged docs only); `git commit -q -m
  'a⏎⏎b'` (a real newline inside single quotes); `git log -1 --format='%h %s'`; `git log HEAD~1..HEAD`;
  `git diff --name-only origin/main...HEAD`; `TZ=Europe/Amsterdam date +%H:%M`; `date +'%Y-%m-%d %H:%M'`;
  `git status 2>&1`; `git status >&2`; `git status⏎git log -1` (two plain lines).
- **C-PLAIN, the corpus test (node:test, Linux, bash 5):**
  - It runs bash with a PATH that holds only stub programs (git, gh, date, node, npm). Each stub writes its argv,
    NUL-separated, to a log and exits 0.
  - The cwd is a temp folder, so redirect targets land there.
  - Input (a) is a fixed corpus: every routine row in the suite that R-PLAIN accepts, plus the routine lines from
    NIGHT_SHIFT.md, CHECKPOINTS.md and advisor.md step 0.
  - Input (b) is 2,000 seeded random lines over the plain form, with no `||` (stubs exit 0, so every command runs).
    The words cover quoted spaces, apostrophes inside double quotes, `#`, `<` and `>` inside quotes, and newlines
    inside single quotes. The separators are `;`, `&&`, `|`, `|&` and newline; the redirects are `>`, `>>`, `2>&1`,
    `&>` and `>&2`.
  - It asserts that parseShell's commands (argv after quote removal), in order, equal the stubs' log, and that its
    redirect targets equal the files bash created.
  - It also asserts that R-PLAIN refuses every P3-1, P3-2, P3-3 and P3-9 line above.
- Mutants, each killed:
  - R-PLAIN removed;
  - R-PLAIN applied to day lines too (killed by a day row with a comment);
  - R-PLAIN run after parseShell on its words instead of on the raw text;
  - one mutant per refused class, each dropped from the allowlist: comment, dollar, backslash, backtick,
    parentheses/braces/brackets, here-document, `>&word`, double-quote content, control characters.

**parseShell (day)**
- Ask:
  - H8 (RB), and the skeptic's A1 on the day side;
  - `cat <<'E O'⏎E O⏎git switch -q cs/evil⏎E` from the main folder;
  - `printf $'a\'b'; git switch -q cs/evil` from the main folder;
  - the backslash-newline-then-`# '` form (D5) on the day side;
  - `git status >&CLAUDE.md` (R4, RB).
- Null:
  - `git status # note`;
  - `echo a#b`;
  - `git log --grep '#x'`;
  - `cat <<EOF⏎body⏎EOF`;
  - `cat <<-EOF⏎⇥body⏎⇥EOF`;
  - `cat <<\EOF⏎b⏎EOF`;
  - `cat <<"EOF"⏎b⏎EOF`;
  - `echo "${#x}"`.
- Mutants: each of the five parser rules reverted, one at a time; also the comment rule removed from sub().

**dateCheck**
- Deny: `date '+%s;x'` (it passes R-PLAIN because the `;` is quoted; the word rule refuses it) and `date "+a'b"`.
- Null: `date +%s`, `date '+%H:%M'` and `date +'%Y-%m-%d %H:%M'`.
- Mutant: back to any `+word`.

**Night commit allowlist** (real git; CLAUDE.md modified and unstaged unless a row says otherwise)
- Deny:
  - `git commit -q -m m CLAUDE.md` (RB: on the old guard the commit holds CLAUDE.md);
  - `git commit -m m docs/a.md CLAUDE.md`;
  - `-i`, `--include`, `-o`, `--only`;
  - `--pathspec-from-file=f`, `--pathspec-file-nul`;
  - `-p`, `--patch`, `--interactive`;
  - `-- .`, `.`, `-- 'CLAUDE.*'`, `-- ':(top)CLAUDE.md'`, `-- '*.md'`;
  - `-- settings.json` run from `.claude/` (the verifier's N3);
  - `--amend`, `-C HEAD`, `--fixup=HEAD`, `-F msg.txt`, `-qo -m m`.
- Deny still:
  - `git commit -m m` with CLAUDE.md staged;
  - `git commit -am m` with CLAUDE.md modified.
- Null (docs changes only):
  - `git commit -q -m "m"`;
  - `git commit -am m`;
  - `git commit -qam m`;
  - `git commit --message=m`;
  - `git commit -s -m a -m b`;
  - `git commit --allow-empty -m m`.
- Mutants:
  - the allowlist removed;
  - `--` accepted;
  - each of `-i`, `-o` and `--pathspec-from-file` accepted, one at a time;
  - the bundle letter set widened to `o`.

**P3-4**
- Deny:
  - night `git fetch -q origin cs/ok &`;
  - night `git fetch -q origin cs/ok` with run_in_background true;
  - the pr-advisor `git fetch -q origin main` with run_in_background true.
- Null:
  - the saver `node .claude/scripts/checkpoint.mjs watch kit-v3-3 --trailer "x"` with run_in_background true;
  - `git status 2>&1`.
- Mutants:
  - the `&` rule dropped;
  - the run_in_background rule dropped;
  - `background` read from `forks` (killed by a night `git status | node …` row that must not get the background reason).

**P3-6** (real git)
- Setup: a branch adds a `.claude/sub` gitlink together with `.gitmodules` `ignore = all`.
- Deny: the night switch to it and the night push of it.
- Ask: the day main-folder switch to it.
- Deny: a night commit of a staged gitlink at `.claude/sub` while `ignore = all`.
- Mutant: the flag dropped.

**P3-7**
- Null:
  - `git clone <O> .gate/tools`;
  - `git clone https://example.invalid/a/tools.git .gate/t`;
  - `git clone https://example.invalid/o/tools.git /tmp/t`.
- Ask:
  - SD-5's three `--revision` rows (agents refused);
  - `git clone --frobnicate x https://example.invalid/o/tools.git` from the project folder (an unknown option);
  - `git clone https://example.invalid/o/tools.git` with no folder, from the project folder.
- Mutant: back to every positional word.

**For the whole fix pass:**
- Main's test file runs on the new guard, with every failure named. The new ones should be only R-PLAIN or
  commit-allowlist movers and the P3-6 key.
- A routine no-looser diff, as in sap3-diff, head against 01b158c, over all its lines and both roles plus the rows
  above. Every changed row must be stricter.
- The builder lists, by name, every existing routine or day row whose expected value moves. That is the prompt
  cost.

### 4. Texts, word for word
**STATUS's A3 entry, a new rule line (after L18):**
- **R-PLAIN, routines' plain shell (fix pass 3):** before the guard reads a routine's Bash line, the line must be in a
  plain form that the guard splits exactly as bash does, or it is refused. Outside quotes it allows letters, digits,
  spaces, tabs, newlines, the characters `_ - . / : = @ % + , ^`, a `~` inside a word, and the operators `&&`, `||`,
  `;`, `|`, `|&`, `&`, `>`, `>>`, `>|`, `<`, `&>`, `&>>`, and `>&` or `<&` before a number or `-`. Inside double
  quotes it allows no `$`, backtick or backslash. Every quote must be closed. So routines use no comments,
  expansions, here-documents, braces, brackets or parentheses. A corpus test against real bash checks the claim.

**L16 (replaces the current text):**
- L16, deliberately disguised shell text: a day line written to hide what it does from a text reader (a variable
  name built by expansion, such as `export "${n}IR=…"`, `declare -n`, `printf -v`, `read` or `mapfile` into a git
  variable, a cd reached through tricks the cd rules do not list, or shell syntax the guard's reader does not model,
  such as brace expansion) can pass the day checks. The day ask is a seatbelt against accidents, not a wall. Routines
  are held to a plain shell form that the guard reads exactly as bash does (R-PLAIN), so they are refused such lines.

**L18 (replaces the current text):**
- L18, parallel tool calls: a `git fetch` running in one tool call can move a ref after the guard checked a night
  `git switch` in another call made at the same time. NIGHT_SHIFT.md step 4 says to fetch first and wait. A routine
  never runs git in the background (a background `&` or run_in_background on a line with git is refused), so a fetch
  made in an earlier call has finished before the next call is checked.

**STATUS's SD-3 line, add at the end:** "So `git stash && git switch main` asks too: a fetch earlier on the line can
move local main."

**STATUS, the night commit (add to the fix-pass-3 summary):** "A night `git commit` takes only `-m`/`--message`,
`-a`, `-q`, `-s` and `--allow-empty`; no paths or pathspec options, so the guard checks exactly the staged list (and
the unstaged list with `-a`)."

**NIGHT_SHIFT.md, step 7, add after "…at most 3 agents at a time, §4)":** "Write shell commands plainly, one per line
or joined with `&&`, values in quotes: no comments, `$`, backslashes, here-documents, braces or parentheses (the guard
refuses them). Commit with `git add <files>` in its own call, then `git commit -m "<message>"` alone; run git in the
foreground."

**Follow-ups (STATUS's follow-ups):**
- FU-P3a, the day reader against real bash: a differential corpus of day lines (comments, here-documents, ANSI-C,
  `${…}`, arithmetic, `case`, brace expansion, redirects) comparing parseShell with bash, and brace expansion in day
  write targets (`tee CLAUDE.{md,x}`); next kit change-set (test-engineer reviews it).
- A-4 (the day bootstrap when routine-guard.mjs is missing) and the earlier follow-ups stand as ruled.

**The report:** under "changes to his words", P3-1, P3-2, P3-3, P3-5 and P3-9 are listed as pre-existing on main
12d7961 and fixed here, with this reason: they defeat A3's own WT-NIGHT check and SD-1's rule. The moved cases
include P3-8 by name.

### 5. The check after the fixes: pass 4, scoped so it converges
- **Who:** security-auditor and the verifier (Opus 5.5, effort high) on only the fix-pass-3 diff (01b158c..the fix
  head), plus the automated security review on the fix commit.
- **security-auditor, routine side, one central question:** find a line that R-PLAIN accepts on which bash runs a
  command, or opens a file for writing, that parseShell's commands and redirects do not show. It also checks the
  commit allowlist (what git commits equals the listed sets), P3-4, P3-6 and P3-9 on routine lines, and that the
  pr-advisor and every existing routine line are no looser (the diff harness).
- **Day side, regressions of fix pass 3 only:** a control that newly asks, or a case moved without its name.
- **verifier:** the evidence. That means the rows above (RB rows run), C-PLAIN passing, every mutant killed, gate with
  smoke PASS, main's test file with every failure named, the movers listed, and the texts of section 4 word for word,
  with line references rechecked at the head.
- **Skeptics:** only for a routine-side major.
- **What makes a further finding a limit, not a pass 5:**
  1. A line that R-PLAIN accepts but bash reads differently is fixed by narrowing R-PLAIN (removing that character or
     operator), never by a new parse rule. The verifier alone re-checks it: its row, its mutant, the no-looser diff
     and C-PLAIN. No new auditor pass.
  2. On the routine side, any other finding that is pre-existing on main, outside the fix-pass-3 diff, and not a
     bypass of WT-NIGHT, the night commit check or the night push check becomes a follow-up.
  3. On the day side, any finding that is not a regression of fix pass 3 becomes a limit line or a follow-up (L16 or
     FU-P3a).
  4. A routine bypass of WT-NIGHT, the commit check or the push check of a different kind (git's semantics, not shell
     reading) comes back to me. My default: if one by-construction refusal closes it, fix it in A3 with the verifier's
     check; otherwise split it out (section 1).
- After pass 4 is clean: the docs-keeper's final pass, the report, my final-head review, then Mustafa's reconfirm.

### Decisions needed
None. WT-NIGHT and PC-SETUP are answered. Where to fix, the fix shapes, and the order of work are mine to decide.
Main's exposure until the merge is low (section 1) and needs no hold on the routines.

### For Mustafa
Nothing now. A3 (PR #38) stays a draft. After fix pass 3 and a clean pass 4, it comes to you to reconfirm the merge.
It changes the rules in `.claude/` and CLAUDE.md, and it now also closes five gaps in the routines' guard that are
on main today.

DONE
