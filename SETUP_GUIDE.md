# Setting up Claude Code for Shamble Frontlines (Windows)

About an hour. You end with: a private GitHub repo, Claude Code running on your laptop with the agent kit, and a first
change-set (Phase 0) that makes the toolkit portable and proves every gate works.

## 1. Install (once)
- **Git for Windows**, **Node.js LTS** (22 or newer) and **Python 3.12** (needed by village.py and the security plugin;
  tick "Add to PATH").
- **Claude Code**: follow the official setup page, code.claude.com/docs/en/setup. Native Windows works; WSL is not
  required. Keep it current with `claude update` (ultracode needs a recent version).
- Optional: **GitHub Desktop** if you prefer clicking to git commands.

## 2. Create the repository (about 5 minutes)
1. Unzip **shamble-frontlines-repo.zip** into a folder such as `C:\dev\shamble-frontlines`. Not inside OneDrive:
   syncing fights git. It's the complete project: the five game files (UI step 1 included), every doc (plus the new
   DESIGN_NOTES.md and the Renegade X reference), the toolkit and the Claude Code kit. It was verified before delivery:
   battery green, the game boots from the folder, every path CLAUDE.md points at exists.
2. Copy **KANESWRATH_REFERENCE.md** from your earlier downloads into `docs/reference/`. It's the one file I couldn't include.
3. **GitHub Desktop:** File → Add local repository → pick the folder. It says the folder isn't a repository yet: choose
   "create a repository", then Create repository. If files appear under Changes, commit them as `baseline`. Then
   **Publish repository** and keep **Private** ticked.
   Terminal alternative: `git init`, `git add -A`, `git commit -m "baseline"`, then create an empty private repository
   on github.com and run the "push an existing repository" commands it shows.
The earlier kit zip is superseded: everything in it is already inside this repository.

## 3. First start
Open a terminal in the repo and run `claude`. Accept the folder-trust prompt (project hooks and agents need it). Then:
- `/doctor` runs a setup checkup.
- `/plugin install security-guidance@claude-plugins-official`, choose **user** scope, then `/reload-plugins` if asked.
  It reviews Claude's own edits for vulnerabilities as it works; the kit adds a project threat model for it.
- `/model`: Opus 5.5 (the default) suits most work. For long architecture or migration sessions, use Fable 5.1 if your
  plan includes it (it can draw on usage credits; Claude Code asks first).
- Type `@` to see the six project agents: verifier, art-director, perf-auditor, security-auditor, netcode-architect,
  docs-keeper. `/gate`, `/changeset`, `/render-ab` and `/advisor-report` are the project skills.

## 4. Run Phase 0
Paste **Prompt 0** from FIRST_PROMPTS.md. Claude plans first and waits for your OK. It installs the toolkit's
packages (it asks before running npm install). Approve, then follow progress with
`/tasks` (subagents) and `/workflows` (workflow runs). It ends with an advisor report in `docs/reports/`: paste that
into the Advisor chat (this Project).

## 5. Ultracode: use it on purpose
Ultracode makes Claude turn every substantive task into a multi-agent workflow. Brilliant for wide work, wasteful for
narrow work, and it uses your limits much faster.

| Turn it on (`ultracode:` in one prompt, or `/effort ultracode` for a session) | Leave it off |
|---|---|
| Codebase audits (performance, security, dead code, doc drift) | A single change-set in the engine files |
| Big mechanical migrations, done on isolated copies | UI and visual tuning that waits on your screenshots |
| Drafting ADRs from several angles with adversarial review | Quick fixes |
| Verification sweeps (sim runs, render matrices, fuzzing) | Anything you want to approve halfway |

- Workflows take **no input mid-run**: each approval stage is its own run.
- Try a small slice first and watch token use in `/workflows`; stop a run there at any time.
- After a big push: `/effort ultracode off`. On the Pro plan, enable "Dynamic workflows" in `/config` first.

## 6. Safety net
- Auto mode is the default permission mode (a classifier reviews actions). The kit adds hard rules: no force-push,
  hard reset, `git clean`, `rm -rf`, or reading `.env`/secrets; pushes, merges and installs ask you first.
- Every change-set gets its own branch. `/rewind` undoes edits inside a session; git undoes everything else.
- A syntax hook checks every JS and HTML file the moment Claude writes it (about 0.1 s per file).

## 7. Working with the Advisor (this claude.ai Project)
- **Connect the repo:** in this Project, Project knowledge → "+" → GitHub → choose the repository → select `docs/`,
  `CLAUDE.md`, `.claude/` and the five game files. Then remove the old manually uploaded docs from Project knowledge,
  so there is one source of truth.
- **Sync is manual:** click "Sync now" in Project knowledge after Claude Code finishes a change-set and before you ask
  me for a review. No more uploading files by hand.
- Replace this Project's instructions with ADVISOR_PROJECT_INSTRUCTIONS.md.
- After each change-set: tell me which report to read (docs/reports/…), and add Brave screenshots for visual work.
  I review it and write the next prompt. At each phase end, ask for a vision review.
