# SEC-SB-3's setting: the docs record (advisor-u8-stopb.md section 5, :182-189)
Read 2026-11-23 by a Claude Code docs agent for the main session; quotes word for word from code.claude.com/docs/en/
permissions, permission-modes, settings-reference, tools-reference, sandboxing (each read that day).

## The setting
- **Name and place:** `permissions.blockReadsOutsideWorkingDirectories`, a Boolean inside `"permissions"` (not top
  level); example `{"permissions": {"blockReadsOutsideWorkingDirectories": true}}`. Scope: "Any file. A `true` in any
  file applies". Default "unset, so reads outside the working directories follow your permission mode".
- **Version:** "Requires Claude Code v2.1.257 or later." (settings-reference)
- **What it covers:** "Claude Code denies `Read`, `Grep`, `Glob`, and `LSP` calls on those paths" in every mode;
  "Claude Code doesn't refuse shell commands the same way": "recognized file-reading Bash commands prompt even in auto
  mode and `bypassPermissions` mode", and "A command the shell parser can't trace ... prompts the same way"
  (permission-modes, "Actions no mode auto-approves"). PowerShell: not documented (every statement names Bash).

## The read-only set (why the seat was right)
- "Claude Code recognizes a built-in set of Bash commands as read-only and runs them without a permission prompt in
  every mode, except as `permissions.blockReadsOutsideWorkingDirectories` changes for paths outside your working
  directories. The set includes `ls`, `cat`, `echo`, `pwd`, `head`, `tail`, `grep`, `find`, `wc`, `which`, `diff`,
  `stat`, `du`, `cd`, and read-only forms of `git`. The set is not configurable" (permissions, "Read-only commands").
  PowerShell's read-only cmdlet list: not documented.
- "Deny rules block in every mode, including `bypassPermissions`." (permission-modes)

## Deny rules' reach
- "Read and Edit deny rules apply to Claude's built-in file tools. In a Bash command Claude Code can analyze, they also
  apply to redirection targets and to file commands it recognizes, such as `cat`, `head`, `tail`, `sed`, and `tee`."
  "They don't apply to a command that reads files without naming them, such as `grep -r pattern .` ... [or] a Python or
  Node script that opens files itself." Whether they reach PowerShell commands: not documented ("a `Bash` rule doesn't
  restrict the PowerShell tool, which has its own permission rules").
- Paths: `~/path` is "Path from home directory"; whether `~` expands on native Windows inside a rule: not documented.
  "On Windows, paths are normalized to POSIX form before matching. `C:\Users\alice` becomes `/c/Users/alice`, so use
  `//c/**/.env` ... To match across all drives, use `//**/.env`." A single leading `/` anchors at the settings file.
- Bash rules match the command text: a rule "isn't a security boundary around the program". "On native Windows, Claude
  Code runs commands unsandboxed." (sandboxing)

## What item 8 takes from this (the ruling's terms applied)
- The setting covers the file tools and the read-only Bash set for outside paths: set it true in ops/operator/
  settings.json; Stop B reads the laptop's `claude --version` and stops below 2.1.257.
- PowerShell's coverage is not documented, so the ruling's fallback applies to that shell: deny rules for each read form
  naming `npt_operator` or `.ssh` (Get-Content and its aliases gc, cat, type; Select-String; Format-Hex; Copy-Item), and
  rules.md says so.
- `Read` denies in both forms, `Read(~/.ssh/**)` and `Read(//**/.ssh/**)` (the `~` expansion on native Windows is not
  documented).
- rules.md names the documented gaps (a search that names no file, a script that opens files itself, no sandbox on
  native Windows) and the wall that holds either way: the passphrase, with the key held in Windows' ssh-agent from
  Stop B.
