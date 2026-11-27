# SEC-UM4-1's probe, kept (copied 2026-11-27 20:1x UTC from the scratchpad's gitdir-probe/probe.mjs)
The security seat's probe for SEC-UM4-1 (review-um4-security.md:41-50), kept here because a restart can clear the
scratchpad. It is stored as text on purpose, so it cannot run by accident.

Run it ONLY as a .mjs file in a fresh, empty scratch folder outside every repository. Beside it make two folders:
"real" holding a.txt and b.txt, and "tmp-repo" holding verdict.txt. Never run it inside this repository, never in
this shell's repository folder, and never with GIT_DIR or any other git variable exported around the real ops step:
that would commit a tree-deleting change here, and the saver would push it.

Its recorded output (git 2.43.0): "real before: 1 commit(s); tree: a.txt,b.txt", "real after: 803357a results /
b40973f two files", "real HEAD tree: verdict.txt"; tmp-repo has no repository of its own.

```js
// Probe (security-auditor, UM-4 read): do operator.test.mjs's git calls (operator.test.mjs:157-168, cwd = a temp
// folder, env inherited) act on the repository GIT_DIR names instead of the temp folder? Everything is under this
// scratch folder: "real" stands in for the repository, "tmp-repo" for withTemp's folder.
import { execFileSync } from "node:child_process";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const HERE = dirname(fileURLToPath(import.meta.url));
const REAL = join(HERE, "real");
const TMP = join(HERE, "tmp-repo");
const plain = (cwd, ...a) => execFileSync("git", ["-c", "user.name=r", "-c", "user.email=r@example.invalid", ...a], { cwd, encoding: "utf8", env: { ...process.env, GIT_DIR: undefined } }).trim();

// The stand-in repository: two tracked files, one commit.
plain(REAL, "init", "-q");
plain(REAL, "add", ".");
plain(REAL, "commit", "-q", "-m", "two files");
console.log("real before:", plain(REAL, "log", "--oneline").split("\n").length, "commit(s); tree:", plain(REAL, "ls-tree", "--name-only", "HEAD").split("\n").join(","));

// operator.test.mjs's own calls, word for word, with GIT_DIR in the inherited environment.
const env = { ...process.env, GIT_DIR: join(REAL, ".git") };
const git = (...a) => execFileSync("git", ["-c", "user.name=t", "-c", "user.email=t@example.invalid", ...a], { cwd: TMP, encoding: "utf8", env }).trim();
console.log("init:", git("init", "-q") || "(quiet)");
git("add", ".");
git("commit", "-q", "-m", "results");

console.log("real after:", plain(REAL, "log", "--oneline"));
console.log("real HEAD tree:", plain(REAL, "ls-tree", "--name-only", "HEAD").split("\n").join(","));
let tmpGit = "absent";
try { tmpGit = plain(TMP, "rev-parse", "--git-dir"); } catch { /* no repository in the temp folder */ }
console.log("tmp-repo's own .git:", tmpGit);
console.log(plain(REAL, "version"));
```
