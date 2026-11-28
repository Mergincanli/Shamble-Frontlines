"""The test zip of change-set 4c-1, the page's scene (his item 6 a: the scene's final head and the steps to see it in
Brave): the built development page with the small server, a README with his Brave and Firefox steps (art-director's,
from its review of the scene) and the commit, and SHA256SUMS.txt. Change-set 3b's make-zip.py in its structure; it
calls no git: the commit's ids come in as arguments, read by the caller with `git rev-parse` in calls of their own.
Never committed as a zip; sent through the chat's file tool. Run from the worktree's root, after `build.mjs` wrote the
development build from a clean tree at the final head:

    python3 -I <this file> <built folder> <stage parent> <commit, 9 characters> <commit, 40 characters>

every path in full. It refuses a build whose scripts do not name the commit, or name it with "+changes"; a small
server that is not the committed one; and a stage folder that exists already. The zip's root is flat: index.html,
serve.mjs, assets/, content.json, README.txt, SHA256SUMS.txt.
"""

import hashlib
import os
import re
import shutil
import subprocess
import sys
import zipfile

NAME = "shamble-v2-04c1-the-scene"
SERVE = "/home/user/Shamble-Frontlines/.gate/cs4b-2/packages/client/serve.mjs"


def sha256(path):
    with open(path, "rb") as handle:
        return hashlib.sha256(handle.read()).hexdigest()


def files_of(folder):
    out = []
    for base, _dirs, names in os.walk(folder):
        for name in names:
            out.append(os.path.relpath(os.path.join(base, name), folder).replace(os.sep, "/"))
    return sorted(out)


README = """Shamble Frontlines v2, change-set 4c-1: the page's scene
Built from commit {commit} ({full}), the head the scene's pull request is opened at.

What this is: the test scene of 4b, the street corner on the left and the training yard on the right, drawn as
plain grey blocks. Height reads through brightness: a higher top is never darker. The street and the yard's lane
have a grey of their own. No soldier yet: he comes in 4c-2. Nothing in this folder installs anything or goes
online: the small server answers this computer only.

You need Node 18 or newer. To check: in PowerShell, type  node --version  and press Enter.

Open it
 1. Unzip this zip into a folder of its own, for example C:\\shamble\\04c1-scene. In that folder, click the
    address bar of the File Explorer window, type  powershell  and press Enter. In the window that opens, type
    node serve.mjs  and press Enter. It should print:
      Shamble Frontlines: serving this folder at http://127.0.0.1:4173
 2. Open http://127.0.0.1:4173 in Brave. A double-click on index.html does not run the game: the page says so.

Check (about 6 minutes; Brave first, then Firefox once)
 1. Maximise the Brave window (at least about 600 px tall: in a shorter window the black readout panel lies over
    the houses and the yard's back row and darkens them). Expected: the whole map in one view from above at an
    angle, the street corner on the left and the training yard on the right. The readout's first line names the
    backend ("backend: WebGPU (...)" on most Windows computers; "WebGL 2 fallback, because ..." is also fine: say
    which); a line of the readout ends with "commit: {commit}"; and "errors: 0".
 2. Look at the blocks. Expected: two houses on the left with the narrow alley between them, each a bright slab
    (the near side of its roof) over a darker front wall, with the slanted end of the gable on its right; the ridge
    itself does not show from this angle. A low cover wall on the grass just past the street, below them. In the
    yard: two small dark squares (the 0.55 m and 0.56 m steps, the same grey), three ledges just in front of them
    (the same light grey), a thin wall like a line, two platforms with their ramps (the ramp a little darker than
    its platform), six thin posts along the lane at the bottom right. Height reads in steps of brightness: the steps
    darkest, then the platforms, then the ledges, then the roofs lightest; blocks within about 0.2 m of each other
    look the same. On every block the top is lighter than its sides; a tall block's wall can look as dark as a low
    step's top (only the tops carry height).
 3. Look at the ground. Expected (your STREET-GREY (b)): two strips a little darker than the grass around them:
    the street from the left edge to the middle, between the houses and the cover wall, and the yard's lane at the
    bottom right, under the posts. The difference is small but should be plain; if you want it stronger or weaker,
    say so (it goes into the soldier's pull request).
 4. Read the readout's two new lines. Expected: "content: maps/test-scene.json shamble-map 1 ·
    movement/soldier.json shamble-movement 1" and "scene: test-scene · 17 boxes · 6 ramps · 6 regions".
 5. Press "Copy report" and paste the result into the chat. Expected: the same rows (content.*, scene.*),
    "errors: 0", and the render counts.
 6. Open http://127.0.0.1:4173 once in Firefox. Expected: the same scene, greys and rows, and "errors: 0"; say
    its backend line. If Brave and Firefox both said WebGPU, open the page once more in Brave at
    http://127.0.0.1:4173/?backend=webgl (the first line then reads "backend: WebGL 2 fallback, asked for by
    ?backend=webgl ...") and check that the greys look the same: the pictures made on the build machine could only
    show WebGL 2.
 7. Back in PowerShell, press Ctrl+C to stop the server.

If something else shows: a line of text over the picture, an error count above 0, a blank page, a block missing
or out of place: that is exactly what this check is for. Paste what you see. A line that starts "The match
stopped:" we would want word for word.

If the port is taken, the server says so; then type  node serve.mjs 4174  and use that number in the addresses
above.

SHA256SUMS.txt lists the sha256 of every file here. To compare one in PowerShell:
  Get-FileHash index.html -Algorithm SHA256
"""


def main():
    if len(sys.argv) != 5:
        sys.exit("usage: make-zip-4c1.py <built folder> <stage parent> <commit, 9 characters> <commit, 40 characters>")
    dist, parent, commit, full = sys.argv[1:5]
    for given in (dist, parent):
        if not os.path.isabs(given) or ".." in given.split("/"):
            sys.exit(f"{given}: give every path in full")
    if not re.fullmatch(r"[0-9a-f]{9}", commit) or not re.fullmatch(r"[0-9a-f]{40}", full) or not full.startswith(commit):
        sys.exit("the commit's two ids do not agree, or are not ids")
    names = files_of(dist)
    for needed in ("index.html", "serve.mjs", "content.json"):
        if needed not in names:
            sys.exit(f"{dist} has no {needed}: is it the development build?")
    scripts = [open(os.path.join(dist, n), encoding="utf-8").read() for n in names if n.endswith(".js")]
    if not any(commit in text for text in scripts):  # the minifier picks the quotes
        sys.exit(f"no script of {dist} names commit {commit}: build again at this head")
    if any("+changes" in text for text in scripts):
        sys.exit(f"a script of {dist} was built from a tree with changes: build again")
    if sha256(os.path.join(dist, "serve.mjs")) != sha256(SERVE):
        sys.exit("the build's serve.mjs is not the committed one")

    stage = os.path.join(parent, f"{NAME}-{commit}")
    if os.path.exists(stage):
        sys.exit(f"{stage} exists already: every zip gets a folder of its own")
    files = os.path.join(stage, "files")
    shutil.copytree(dist, files)
    with open(os.path.join(files, "README.txt"), "w", encoding="utf-8", newline="\r\n") as handle:
        handle.write(README.format(commit=commit, full=full))
    listed = [n for n in files_of(files) if n != "SHA256SUMS.txt"]
    with open(os.path.join(files, "SHA256SUMS.txt"), "w", encoding="utf-8", newline="\r\n") as handle:
        for name in listed:
            handle.write(f"{sha256(os.path.join(files, name))}  {name}\n")

    out = os.path.join(stage, f"{NAME}.zip")
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

    print(f"zip: {out}")
    print(f"built from commit {commit} ({full})")
    print(f"size {os.path.getsize(out)} bytes, sha256 {sha256(out)}")
    print(f"{len(members)} files, each equal to its line in SHA256SUMS.txt after unpacking:")
    for name in members:
        print(f"  {os.path.getsize(os.path.join(check, name)):>9}  {name}")
    print(f"unpacked copy for the browser tests: {check}")


main()
