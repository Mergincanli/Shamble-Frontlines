"""The test zip of change-set 2 (plan section 8, step 8, and section 13): the built development page with the small
server, a README with Mustafa's Brave steps and the commit, and SHA256SUMS.txt. Never committed; sent through the
chat's file tool. Run from the repository root, after `build.mjs` wrote the development build with a clean tree:

    python3 -I .checkpoints/cs2-first-page/make-zip.py packages/client/dist .gate/cs2-first-page/zip

It refuses a build whose commit is not HEAD's (or is marked "+changes"), and a stage folder that already exists.
The zip's root is flat: index.html, serve.mjs, assets/, README.txt, SHA256SUMS.txt.
"""

import hashlib
import os
import shutil
import subprocess
import sys
import zipfile

NAME = "shamble-v2-02-first-page"


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


README = """Shamble Frontlines v2, change-set 2: the first page
Built from commit {commit} ({full}).

What this is: the first page of the new engine. The simulation (Rust, built as WebAssembly) runs in a
background worker at 30 ticks a second; the page draws an empty ground with a 1 m grid. The colours are
placeholders, not the look. A readout in the corner shows what runs. Nothing in this folder installs
anything or goes online: the small server answers this computer only.

You need Node 18 or newer. To check: in PowerShell, type  node --version  and press Enter.

Steps (about 18 minutes, 9 of them waiting)

 1. Unzip this zip into a folder of its own, for example C:\\shamble\\02-first-page.
 2. Double-click index.html in that folder. Brave opens it as a file. Expected (it is what our test
    browser does): one line of text that tells you to run "node serve.mjs", and no picture. Note what
    you see, and any other message: this is the first answer, for Brave, to whether a built page opens
    from a file.
 3. In that folder, click the address bar of the File Explorer window, type  powershell  and press Enter.
    In the window that opens, type  node serve.mjs  and press Enter.
    It should print: Shamble Frontlines: serving this folder at http://127.0.0.1:4173
 4. Open http://127.0.0.1:4173 in Brave, with Shields at your usual setting (note the setting).
    You should see an empty ground with a grid of 1 m squares, lit from above, and a readout in a corner:
      backend: WebGPU   (or: WebGL 2 fallback, with the reason)
      ticks: counting up, "at 30 a second"; blocks: the same number as ticks (within 1)
      frames: your screen's rate, for example 60.0 a second
      errors: 0
      last pause: none yet
      isolation: no, and the commit: {commit}
 5. Leave it for 2 minutes. Errors should still be 0, and blocks still about equal to ticks.
 6. Switch to another tab for 1 minute, then come back. The readout's "last pause" line should say
    about 60 s hidden, and "ticks while hidden" 0 or a very small number (a match that ran on would
    show about 1800, and the line says so). The tick counter counts on from where it stood.
 7. The same with 6 minutes away: about 360 s hidden, ticks while hidden 0 or a very small number.
 8. Press "Copy report" and paste the result into the chat. (If the button cannot reach the clipboard,
    a text box appears with the report selected: copy it from there.)
 9. Open http://127.0.0.1:4173/?backend=webgl
    The readout should say "WebGL 2 fallback, asked for by ?backend=webgl", with the same scene and
    errors 0. Press "Copy report" and paste that too.
10. Open http://127.0.0.1:4173/?pad=4096 , wait 30 seconds, press "Copy report" and paste it.
    Then the same with http://127.0.0.1:4173/?pad=65536 .
    These two read what it costs on your laptop to hand a larger block to the page. Readings, not judged.
11. Open brave://version in a new tab and paste its first line.
12. Back in PowerShell, press Ctrl+C to stop the server.

If something else shows: a line of text over the picture, an error count above 0, a blank page, or a
readout that does not move: that is exactly what this check is for. Paste what you see.

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
