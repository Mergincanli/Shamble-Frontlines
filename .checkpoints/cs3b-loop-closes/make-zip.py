"""The test zip of change-set 3a (plan section 4, "His browser check"): the built development page with the small
server, a README with Mustafa's Brave and Firefox steps and the commit, and SHA256SUMS.txt. His words of 2026-11-19,
16:08:04 UTC: the README asks him to keep the window in front and uncovered during the first run, and has one step
with the window covered on purpose. Never committed; sent through the
chat's file tool. Run from the repository root, after `build.mjs` wrote the development build with a clean tree:

    python3 -I .checkpoints/cs3a-rules-boundaries/make-zip.py .gate/dist-development .gate/cs3a/zip

It refuses a build whose commit is not HEAD's (or is marked "+changes"), and a stage folder that already exists.
The zip's root is flat: index.html, serve.mjs, assets/, README.txt, SHA256SUMS.txt.
"""

import hashlib
import os
import shutil
import subprocess
import sys
import zipfile

NAME = "shamble-v2-03a-rules-boundaries"


def git(*args):
    return subprocess.run(["git", *args], check=True, capture_output=True, text=True).stdout.strip()


def sha256(path):
    with open(path, "rb") as handle:
        return hashlib.sha256(handle.read()).hexdigest()


def files_of(folder):
    out = []
    for base, _dirs, names in os.walk(folder):
        for name in names:
            out.append(os.path.relpath(os.path.join(base, name), folder).replace(os.sep, "/"))
    return sorted(out)


README = """Shamble Frontlines v2, change-set 3a: rules and boundaries
Built from commit {commit} ({full}).

What this is: the same first page as last time, an empty lit ground with a 1 m grid. Change-set 3a put the
rules of the simulation under tests and tidied the edges between its parts, so there is almost nothing new
to see. Two lines of the readout changed: "frames" is now a count over time (in your Firefox it said 239.2
where the run's average was 179.9), and a value that rounds to zero no longer prints as "-0.00". Nothing
in this folder installs anything or goes online: the small server answers this computer only.

You need Node 18 or newer. To check: in PowerShell, type  node --version  and press Enter.

Steps (about 6 minutes of waiting; Brave first, then Firefox once)

 1. Unzip this zip into a folder of its own, for example C:\\shamble\\03a-rules. In that folder, click the
    address bar of the File Explorer window, type  powershell  and press Enter. In the window that opens,
    type  node serve.mjs  and press Enter. It should print:
      Shamble Frontlines: serving this folder at http://127.0.0.1:4173
    Open http://127.0.0.1:4173 in Brave. You should see the grey ground with its grid and, in the readout,
    "errors: 0" and the commit {commit}.
 2. Keep Brave's window in front and uncovered for 2 minutes: no other window over it, the screen on.
    Nothing should change, and the "frames" line should hold steady near your screen's rate.
 3. Now cover Brave's window completely with another window (for example a maximised File Explorer) for
    1 minute, then bring Brave back to the front. This step is on purpose: last time one run drew far
    fewer frames than it ticked and we could not tell why. Do not switch tabs here, only cover the window.
 4. Switch to another tab for 1 minute, then come back. "last pause" should show about 60 s hidden and
    "ticks while hidden" 0 or a very small number.
 5. Press "Copy report" and paste the result into the chat. With it, tell us two things your last check
    did not show: whether Brave Shields was up for this page (the lion icon), and what a double-click on
    index.html in the folder shows (expected: one line of text that says to run "node serve.mjs").
 6. In Firefox: open http://127.0.0.1:4173 , wait 2 minutes with the window in front, press "Copy report"
    and paste it. "frames" should now be close to the run's average (frames divided by the seconds run).
 7. Back in PowerShell, press Ctrl+C to stop the server.

If something else shows: a line of text over the picture, an error count above 0, a blank page, or a
readout that does not move: that is exactly what this check is for. Paste what you see. A line that starts
"The match stopped: the simulation stopped: panicked at" would be new in this build: it is the simulation's
own error text reaching the page, and we would want it word for word.

If the port is taken, the server says so; then type  node serve.mjs 4174  and use that number in the
addresses above.

SHA256SUMS.txt lists the sha256 of every file here. To compare one in PowerShell:
  Get-FileHash index.html -Algorithm SHA256
"""


def main():
    dist, parent = sys.argv[1], sys.argv[2]
    full = git("rev-parse", "HEAD")
    commit = git("rev-parse", "--short=9", "HEAD")
    changed = git("status", "--porcelain", "--", "crates", "packages", "Cargo.toml", "Cargo.lock", "package.json", "package-lock.json")
    if changed:
        sys.exit("the tree holds changes that are not committed:\n" + changed)
    names = files_of(dist)
    for needed in ("index.html", "serve.mjs"):
        if needed not in names:
            sys.exit(f"{dist} has no {needed}: is it the development build?")
    scripts = [open(os.path.join(dist, n), encoding="utf-8").read() for n in names if n.endswith(".js")]
    if not any(commit in text for text in scripts):  # the minifier picks the quotes
        sys.exit(f"no script of {dist} names commit {commit}: build again at this head")
    if any("+changes" in text for text in scripts):
        sys.exit(f"a script of {dist} was built from a tree with changes: build again")
    if sha256(os.path.join(dist, "serve.mjs")) != sha256("packages/client/serve.mjs"):
        sys.exit("the build's serve.mjs is not the committed one")

    stage = os.path.join(parent, f"{NAME}-{commit}")
    if os.path.exists(stage):
        sys.exit(f"{stage} exists already: every zip gets a folder of its own")
    shutil.copytree(dist, os.path.join(stage, "files"))
    files = os.path.join(stage, "files")
    with open(os.path.join(files, "README.txt"), "w", encoding="utf-8", newline="\r\n") as handle:
        handle.write(README.format(commit=commit, full=full))
    listed = [n for n in files_of(files) if n != "SHA256SUMS.txt"]
    with open(os.path.join(files, "SHA256SUMS.txt"), "w", encoding="utf-8", newline="\r\n") as handle:
        for name in listed:
            handle.write(f"{sha256(os.path.join(files, name))}  {name}\n")

    out = os.path.abspath(os.path.join(stage, f"{NAME}.zip"))
    top = sorted(os.listdir(files))
    subprocess.run([sys.executable, "-I", "-m", "zipfile", "-c", out, *top], check=True, cwd=files)

    # The control: unpack the zip elsewhere and compare every file with the list inside it.
    check = os.path.join(stage, "unpacked")
    with zipfile.ZipFile(out) as archive:
        members = sorted(i.filename for i in archive.infolist() if not i.is_dir())
        archive.extractall(check)
    sums = {}
    with open(os.path.join(check, "SHA256SUMS.txt"), encoding="utf-8", newline="") as handle:
        for line in handle.read().split("\r\n"):
            if line:
                digest, name = line.split("  ", 1)
                sums[name] = digest
    assert sorted(sums) == [m for m in members if m != "SHA256SUMS.txt"], (sorted(sums), members)
    for name, digest in sums.items():
        assert sha256(os.path.join(check, name)) == digest, name
    assert any(m.startswith("assets/") and m.endswith(".wasm") for m in members), "no .wasm in the zip"

    print(f"zip: {os.path.relpath(out)}")
    print(f"built from commit {commit} ({full})")
    print(f"size {os.path.getsize(out)} bytes, sha256 {sha256(out)}")
    print(f"{len(members)} files, each equal to its line in SHA256SUMS.txt after unpacking:")
    for name in members:
        print(f"  {os.path.getsize(os.path.join(check, name)):>9}  {name}")
    print(f"unpacked copy for the browser tests: {os.path.relpath(check)}")


main()
