// security-auditor pass 3: parseShell "alone" probe. usage: node sap3-parse.mjs <guard.mjs>
import { pathToFileURL } from 'node:url';
const { parseShell } = await import(pathToFileURL(process.argv[2]).href);
const lines = [
  "git switch cs/ok # '\ngit fetch -q origin cs/x # '",
  "git push origin cs/ok #'\ngit push origin HEAD:main #'",
  "git commit -m m # \"\ngit add CLAUDE.md # \"",
  "git switch cs/ok\\\n; touch X",
  "git commit -m 'a\nb'",
  "git commit -F - <<E\nbody\nE",
  "git commit -F - <<E\nbody\nE\ngit push",
  "git commit -F - <<'E O'\nE O\ntouch X\nE",
  "git commit -F - <<E O\nE\ntouch X",
  "time git switch cs/ok",
  "git switch cs/ok |& cat",
  ";git switch cs/ok",
  "git switch cs/ok;;",
  "{ git switch cs/ok; }",
  "git switch cs/ok git fetch",
  "git commit -m m <<<'x'",
  "git switch cs/ok 2>&1",
  "git switch cs/ok &>/dev/null",
  "git switch cs/ok >&2",
  "coproc git fetch; git switch cs/ok",
  "git switch cs/ok\r\ngit fetch",
  "git commit -F- <<E\nx\nE\n",
  "git commit -m m #x\ngit push",
];
for (const l of lines) { const p = parseShell(l); console.log(JSON.stringify(l), '-> cmds', p.cmds.length, 'forks', p.forks, 'subs', p.subs, 'unterm', p.unterminated, 'alone', p.cmds.length === 1 && !p.forks && !p.subs, JSON.stringify(p.cmds.map((c) => c.argv))); }
