// Stop B's dry run (stopb-plan.md rev 0.2, B1): applies fills to a DRY-RUN WORKTREE only, never the main tree.
// Usage: node stopb-dryfill.mjs <worktree> <shape>, shape one of:
//   list   r1-dist.sha256 gains U7's real seven lines (u7-report.md:134-141) in rs.yaml:221's form
//   commit rs.yaml's and lg.yaml's /etc/npt/commit zeros become the worktree's own HEAD (any 40 hex)
//   url    R1.md step 7's placeholder becomes the production directory's meta.termsOfService (read 22:07:10Z)
// Each replacement must match exactly once, else it stops with exit 1. Prints each file and its change.
import { readFileSync, writeFileSync } from "node:fs";
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const [wt, shape] = process.argv.slice(2);
//   standins  the two pins' stand-ins, each one line rewritten in place (B6's STOPB-4)
//   notes     SEC-U8F1-3 (PROMPT-R1.md:129 re-wrapped as two lines) and SEC-U8F1-4's line after R1.md:88
//   unurl     url's change undone (to go from the X shape back to the Y shape)
const SHAPES = ["list", "commit", "url", "standins", "notes", "unurl"];
if (!wt || !wt.includes("/.gate/") || !SHAPES.includes(shape)) {
  console.error(`usage: node stopb-dryfill.mjs <a worktree under .gate/> ${SHAPES.join("|")}`);
  process.exit(1);
}
const URL_FROM = "he reads it first, at `<the production ACME directory's\n   meta.termsOfService: filled at Stop B>`.";
const URL_TO =
  "he reads it first, at the production ACME directory's\n   meta.termsOfService, `https://letsencrypt.org/documents/LE-SA-v1.8-July-06-2026.pdf` (read 2026-11-27).";
const OPS = join(wt, "spikes", "netcode-proof", "ops");
function replaceOnce(file, from, to) {
  const text = readFileSync(file, "utf8");
  const n = text.split(from).length - 1;
  if (n !== 1) {
    console.error(`${file}: expected exactly one match, found ${n}`);
    process.exit(1);
  }
  writeFileSync(file, text.replace(from, to));
  console.log(`${file}: replaced (${JSON.stringify(from).slice(0, 60)} -> ${JSON.stringify(to).slice(0, 60)})`);
}
if (shape === "list") {
  const seven = [
    ["hooks.js", "be1c4ce736532704c8c1c9dd042b4fd0273d6b90eee4651158c33a00f83cf814"],
    ["index.html", "46febaacfc3b2fa43a3b342c39293f0f25ca861efa98cc2f4267cca475f35f95"],
    ["lib.js", "4f374eafe567444bf53c802cb94b2e104fe9b7743e0fbc26c6387c073fdc4bbe"],
    ["main.js", "c9be4a1666f9f1086999f46ed0dc38ccd702450bb3f975c7c1cf33685e41ccec"],
    ["npt_client_wasm.js", "b355d85ca53dd9534b460d3e5c3b180296a740ecd4043741d070a058be103965"],
    ["npt_client_wasm_bg.wasm", "901a649231dc9c217204b121a2800641c3c6aa9cd3cb9d42eb5657452b0a0e77"],
    ["worker.js", "4f060d1851c530f86767efdcf5f3fee68e0e702ef393fcf5fede3e653ae71857"],
  ];
  const file = join(OPS, "runs", "r1-dist.sha256");
  const text = readFileSync(file, "utf8");
  if (text.split("\n").some((l) => l && !l.startsWith("#"))) {
    console.error(`${file}: already holds a non-comment line`);
    process.exit(1);
  }
  writeFileSync(file, text + seven.map(([n, h]) => `${h}  ./${n}\n`).join(""));
  console.log(`${file}: seven lines added under its ${text.split("\n").filter(Boolean).length}-line comment`);
}
if (shape === "commit") {
  const head = execFileSync("git", ["rev-parse", "HEAD"], { cwd: wt, encoding: "utf8" }).trim();
  if (!/^[0-9a-f]{40}$/.test(head)) process.exit(1);
  for (const f of ["rs.yaml", "lg.yaml"]) {
    replaceOnce(join(OPS, "cloud-init", f), "    content: |\n      0000000000000000000000000000000000000000\n", `    content: |\n      ${head}\n`);
  }
}
if (shape === "url") replaceOnce(join(OPS, "runs", "R1.md"), URL_FROM, URL_TO);
if (shape === "unurl") replaceOnce(join(OPS, "runs", "R1.md"), URL_TO, URL_FROM);
if (shape === "standins") {
  replaceOnce(
    join(OPS, "tests", "browser.test.mjs"),
    '  const rs = text("cloud-init/rs.yaml");\n',
    '  const rs = text("cloud-init/rs.yaml").replace(/ {6}[0-9a-f]{40}\\n/, `      ${"0".repeat(40)}\\n`);\n',
  );
  replaceOnce(
    join(OPS, "tests", "r1sh.test.mjs"),
    '    assert.match(run(text("runs/r1-dist.sha256")), /differs/, "the committed list, unfilled, is empty: refused");\n',
    '    assert.match(run(text("runs/r1-dist.sha256").replace(/^(?!#).*\\n/gm, "")), /differs/, "the comment alone: refused");\n',
  );
}
if (shape === "notes") {
  replaceOnce(
    join(OPS, "operator", "PROMPT-R1.md"),
    "   holding pre-push; anything else: stop and tell him. Then push to `claude/npt-results` after the pre-push hook passes (a prompt).\n",
    "   holding pre-push; anything else: stop and tell him.\n   Then, in the same PowerShell, push to `claude/npt-results` after the pre-push hook passes (a prompt).\n",
  );
  replaceOnce(
    join(OPS, "runs", "R1.md"),
    "   PowerShell, `git -C <results> rev-parse --git-path hooks` prints the clone's own hooks folder, prompting, not denied.\n",
    "   PowerShell, `git -C <results> rev-parse --git-path hooks` prints the clone's own hooks folder, prompting, not denied.\n   Its printed path and `git --version` are recorded.\n",
  );
}
