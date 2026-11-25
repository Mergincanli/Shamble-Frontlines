# Scratch (print only; reads two files, writes nothing): check 4 (b) of security-u7-0b-answer.md Q1, the wasm-bindgen
# CLI's packaged lock against the step 7 log. Run 2026-11-25 11:45 UTC from the main folder (dependency-check.md's
# addendum): 254 blocks, 254 pairs, 0 git sources, 0 git updates, 48 downloaded, 0 outside the lock, 161 compiled.
#   python3 -I check4b.py <the packaged Cargo.lock> <step7-cli.log>
import re
import sys

lock_path, log_path = sys.argv[1], sys.argv[2]
with open(lock_path, encoding="utf-8") as f:
    lock = f.read()
with open(log_path, encoding="utf-8", errors="replace") as f:
    log = f.read()

blocks = sum(1 for line in lock.splitlines() if line.strip() == "[[package]]")
git_sources = [line for line in lock.splitlines() if line.strip().startswith('source = "git+')]
pkgs = set()
name = None
for line in lock.splitlines():
    m = re.match(r'^name = "([^"]+)"$', line)
    if m:
        name = m.group(1)
        continue
    m = re.match(r'^version = "([^"]+)"$', line)
    if m and name is not None:
        pkgs.add((name, m.group(1)))
        name = None

git_updates = [line for line in log.splitlines() if "Updating git repository" in line]
downloaded = re.findall(r"Downloaded (\S+) v(\S+)$", log, flags=re.M)
outside = [d for d in downloaded if d not in pkgs]
compiling = sum(1 for line in log.splitlines() if line.strip().startswith("Compiling "))

print(f"lock [[package]] blocks: {blocks} (want 254)")
print(f"lock name/version pairs read: {len(pkgs)}")
print(f"lock 'source = \"git+' lines: {len(git_sources)} (want 0)")
print(f"log 'Updating git repository' lines: {len(git_updates)} (want 0)")
print(f"log 'Downloaded <name> v<version>' lines: {len(downloaded)}")
print(f"of those, not in the lock: {len(outside)} {outside[:10]} (want 0)")
print(f"log 'Compiling' lines: {compiling}")
