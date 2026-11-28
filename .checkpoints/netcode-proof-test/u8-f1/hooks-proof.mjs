// U8-F1 item 2's scratch proof (SR-1, SR-2; not counted, not part of the spike): on a throwaway repository under the
// builder's worktree's .gate/, `git rev-parse --git-path hooks`, run from a `results` folder inside a linked worktree
// (the laptop's `<results>`, .gate\npt-results\results), must change for each of (i)-(iv):
//   (i) `git config core.hookspath <x>` (lower case); (ii) `git config --worktree core.hooksPath <x>` with
//   extensions.worktreeConfig on; (iii) `git config --file .git/config core.hooksPath <x>`; (iv) GIT_CONFIG_COUNT,
//   GIT_CONFIG_KEY_0 and GIT_CONFIG_VALUE_0 in the child's environment.
// Guards: every git child gets an explicit environment (HOME in scratch, no system or global config, the ceiling at
// the scratch folder, nothing inherited but PATH), and each repository's top level is asserted before any config write.
// Usage: node hooks-proof.mjs <new scratch folder under .gate/>
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync, realpathSync } from "node:fs";
import { join, resolve } from "node:path";

const base = process.argv[2];
assert.ok(base && base.includes("/.gate/") && !existsSync(base), "a new folder under .gate/");
for (const d of ["home", "clone"]) mkdirSync(join(base, d), { recursive: true });
const root = realpathSync(base);
const env0 = {
  PATH: process.env.PATH, HOME: join(root, "home"), LC_ALL: "C", GIT_CEILING_DIRECTORIES: root,
  GIT_CONFIG_NOSYSTEM: "1", GIT_CONFIG_GLOBAL: join(root, "home", "no-global-config"),
  GIT_AUTHOR_NAME: "t", GIT_AUTHOR_EMAIL: "t@example.invalid", GIT_COMMITTER_NAME: "t", GIT_COMMITTER_EMAIL: "t@example.invalid",
};
const clone = join(root, "clone");
const wt = join(root, "results-wt");
const results = join(wt, "results");
const git = (cwd, args, env = env0) => execFileSync("git", args, { cwd, env, encoding: "utf8", stdio: ["ignore", "pipe", "pipe"] }).trim();
const top = (cwd) => git(cwd, ["rev-parse", "--show-toplevel"]);

git(clone, ["init", "-q", "-b", "main"]);
assert.equal(top(clone), clone, "the clone is its own repository, never an outer one");
git(clone, ["commit", "--allow-empty", "-q", "-m", "init"]);
git(clone, ["worktree", "add", "-q", wt, "-b", "claude/npt-results"]);
mkdirSync(results);
assert.equal(top(results), wt, "<results> sits in the linked worktree");
const read = (env = env0) => git(results, ["rev-parse", "--git-path", "hooks"], env);
const baseline = read();
console.log(`baseline (from ${results}): ${baseline}`);
assert.equal(realpathSync(resolve(results, baseline)), join(clone, ".git", "hooks"), "the clone's own hooks folder");

const out = {};
const form = (name, set, unset, envOf = () => env0) => {
  const x = join(root, `elsewhere-${name}`);
  set(x);
  const got = read(envOf(x));
  out[name] = got;
  console.log(`${name}: ${got}`);
  assert.notEqual(got, baseline, `${name} changes the printed path`);
  assert.ok(got.endsWith(`elsewhere-${name}`), `${name} prints its own value`);
  unset();
  assert.equal(read(), baseline, `${name} undone: back to the baseline`);
};
form("i", (x) => git(wt, ["config", "core.hookspath", x]), () => git(wt, ["config", "--unset", "core.hookspath"]));
form("ii", (x) => { git(clone, ["config", "extensions.worktreeConfig", "true"]); git(wt, ["config", "--worktree", "core.hooksPath", x]); },
  () => { git(wt, ["config", "--worktree", "--unset", "core.hooksPath"]); git(clone, ["config", "--unset", "extensions.worktreeConfig"]); });
form("iii", (x) => git(clone, ["config", "--file", ".git/config", "core.hooksPath", x]), () => git(clone, ["config", "--file", ".git/config", "--unset", "core.hooksPath"]));
form("iv", () => {}, () => {}, (x) => ({ ...env0, GIT_CONFIG_COUNT: "1", GIT_CONFIG_KEY_0: "core.hooksPath", GIT_CONFIG_VALUE_0: x }));
console.log(`git ${git(clone, ["--version"])}; all four forms change the printed path`);
