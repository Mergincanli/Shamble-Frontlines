# Mutation sweep for the G9 lint in hosting/preview.test.mjs (round-2 fix): each mutant weakens one rule of the lint;
# the suite must fail on every one. A mutant whose anchor is not found exactly once is reported as BAD ANCHOR, never as
# caught. Usage: python3 mutate_lint.py <scratch suite dir with hosting/ and .github/workflows/preview.yml>
import subprocess, sys, pathlib
SUITE = pathlib.Path(sys.argv[1]); TEST = SUITE / 'hosting/preview.test.mjs'; SRC = TEST.read_text()
M = [
  ("L2 places check off", r"if (places.join(', ') !== 'planned 1, planned 2, planned 3')", r"if (false)"),
  ("L2 a step known by a command it holds", r"rs[0].job === job && run === command", r"rs[0].job === job && run.includes(command.split('\n')[0])"),
  ("L2 both secrets not required on a planned step", r"return at >= 0 && both ?", r"return at >= 0 ?"),
  ("L2 any secret line accepted in a step env", r"else if (!SECRET_LINES.includes(r.line.trim()))", r"else if (false)"),
  ("L2 secrets matched in lower case only", r"const namesSecrets = (r) => /\bsecrets\b/i.test(r.line);", r"const namesSecrets = (r) => /\bsecrets\b/.test(r.line);"),
  ("L3 npm ci matched as a prefix", r"const NPM_CI = /^npm ci(?: --(?:ignore-scripts|no-audit|no-fund))+$/;", r"const NPM_CI = /^npm ci\b/;"),
  ("L3 any flag accepted after npm ci", r"const NPM_CI = /^npm ci(?: --(?:ignore-scripts|no-audit|no-fund))+$/;", r"const NPM_CI = /^npm ci(?: --[a-z-]+)+$/;"),
  ("L3 --ignore-scripts not required", r" && commandOf(r).split(' ').includes('--ignore-scripts')", r""),
  ("L3 no npm configuration check", r"if (/npm_config_|npmrc/i.test(r.line)) flag(", r"if (false) flag("),
  ("L3 npm configuration in lower case only", r"if (/npm_config_|npmrc/i.test(r.line)) flag(", r"if (/npm_config_|npmrc/.test(r.line)) flag("),
  ("L3 no npm word check", r"if (/\bnpm\b/i.test(r.line) && !plainNpmCi(r))", r"if (false)"),
  ("L3 run block lines skipped", r"    if (!(r.step >= 0 && r.stepKey === 'run')) continue;", r"    if (!(r.step >= 0 && r.stepKey === 'run' && !r.inBlock)) continue;"),
  ("reader: a commented block header not a block", r"return /^[|>][0-9+-]*(?:\s+#.*)?$/.test(value);", r"return /^[|>][0-9+-]*$/.test(value);"),
  ("commandOf: the block header read as a command", r"return r.key === 'run' && !blockHeader(r.value) ? r.value : '';", r"return r.key === 'run' ? r.value : '';"),
]
bad = []
for name, find, repl in M:
    n = SRC.count(find)
    if n != 1: print(f"BAD ANCHOR x{n}: {name}", flush=True); bad.append(name); continue
    TEST.write_text(SRC.replace(find, repl))
    r = subprocess.run(['node', '--test', 'hosting/preview.test.mjs'], cwd=SUITE, capture_output=True, text=True, timeout=300)
    fails = [l for l in r.stdout.splitlines() if l.startswith('# fail ')]
    first = next((l.strip() for l in r.stdout.splitlines() if l.lstrip().startswith('not ok ') and 'planted' in l or 'lint' in l and l.lstrip().startswith('not ok ')), '')
    v = 'caught' if r.returncode != 0 else 'SURVIVED'
    print(f"{v:9} {fails[0] if fails else ''}  {name}  [{first[:110]}]", flush=True)
    if r.returncode == 0: bad.append(name)
TEST.write_text(SRC)
print(f"{len(M)} mutants; survivors or bad anchors: {bad}")
