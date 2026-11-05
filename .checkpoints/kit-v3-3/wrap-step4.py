import sys
p = sys.argv[1]
s = open(p).read()
R = [
("const CANON = new Map(['CLAUDE.md', '.npmrc', '.mcp.json', '.github', '.git', '.claude', 'agent-memory-local', 'worktrees',\n  'tools', ...[...GATE_FILES].map((f) => f.slice(6))].map((s) => [s.toLowerCase(), s]));\nconst protectedFold = (rel, io) => protectedRel(isWin(io) ? rel.split('/').map((s) => CANON.get(s.toLowerCase()) ?? s).join('/') : rel);",
 "const CANON = new Map(['CLAUDE.md', '.npmrc', '.mcp.json', '.github', '.git', '.claude', 'agent-memory-local',\n  'worktrees', 'tools', ...[...GATE_FILES].map((f) => f.slice(6))].map((s) => [s.toLowerCase(), s]));\nconst protectedFold = (rel, io) =>\n  protectedRel(isWin(io) ? rel.split('/').map((s) => CANON.get(s.toLowerCase()) ?? s).join('/') : rel);"),
("const ask = (reason) => ({ ask: reason }); // day sessions only (kit-v3-3); guard.mjs prints any other ask as a deny (C5)",
 "// day sessions only (kit-v3-3); guard.mjs prints an ask anywhere else as a deny (C5)\nconst ask = (reason) => ({ ask: reason });"),
("  if (prog === 'mv') { const { pos, tdir } = words(a, ['-S', '--suffix']); return tdir !== null ? [...pos, tdir] : pos; }",
 "  if (prog === 'mv') {\n    const { pos, tdir } = words(a, ['-S', '--suffix']);\n    return tdir !== null ? [...pos, tdir] : pos;\n  }"),
("    first = ask('kit and gate files ask first in a day session: a GIT_DIR= or GIT_WORK_TREE= prefix points git at another folder, and the guard cannot follow it');",
 "    first = ask('kit and gate files ask first in a day session: a GIT_DIR= or GIT_WORK_TREE= prefix points git at another ' +\n      'folder, and the guard cannot follow it');"),
("  const targets = [...c.redirects.map((t) => ['a redirect', t]), ...writtenBy(argv).map((t) => [basename(argv[0]) === 'git' ? `git ${argv[1]}` : basename(argv[0]), t])];",
 "  const prog = basename(argv[0] ?? '');\n  const writer = prog === 'git' ? `git ${argv[1]}` : prog;\n  const targets = [...c.redirects.map((t) => ['a redirect', t]), ...writtenBy(argv).map((t) => [writer, t])];"),
("    if (/[$`*?[{]/.test(raw)) { first ??= ask(`kit and gate files ask first in a day session: ${what} writes ${raw}, which the guard cannot resolve (a variable, substitution or glob)`); continue; }",
 "    if (/[$`*?[{]/.test(raw)) {\n      first ??= ask(`kit and gate files ask first in a day session: ${what} writes ${raw}, which the guard cannot ` +\n        'resolve (a variable, substitution or glob)');\n      continue;\n    }"),
("      if (pos[0] && pos[0] !== 'origin/main') { why = `it starts ${create} from ${pos[0]}, not origin/main`; ref = pos[0]; }",
 "      if (pos[0] && pos[0] !== 'origin/main') {\n        why = `it starts ${create} from ${pos[0]}, not origin/main`;\n        ref = pos[0];\n      }"),
("        if (h) why = h.failed ? \"the guard can't compare HEAD with origin/main\" : `HEAD's ${h.hit} differs from origin/main's`;",
 "        if (h) why = h.failed ? \"the guard can't compare HEAD with origin/main\"\n          : `HEAD's ${h.hit} differs from origin/main's`;"),
("      const h = protectedHit(['origin/main', pos[0]], vcwd, io); // no name is exempt: a stale local main is an old commit\n      if (h) why = h.failed ? `the guard can't compare ${pos[0]} with origin/main` : `${pos[0]}'s ${h.hit} differs from origin/main's`;",
 "      // no name is exempt: a stale local main is an old commit\n      const h = protectedHit(['origin/main', pos[0]], vcwd, io);\n      if (h) why = h.failed ? `the guard can't compare ${pos[0]} with origin/main`\n        : `${pos[0]}'s ${h.hit} differs from origin/main's`;"),
("  const reason = `the main folder stays on main's reviewed rules: ${say} would move it (${why}). Work there in a worktree ` +\n    `instead:",
 "  const reason = `the main folder stays on main's reviewed rules: ${say} would move it (${why}). Work there in a ` +\n    `worktree instead:"),
]
for a, b in R:
    n = s.count(a)
    if n != 1: sys.exit(f"count {n} for: {a[:60]}")
    s = s.replace(a, b)
open(p, 'w').write(s)
print("ok")
