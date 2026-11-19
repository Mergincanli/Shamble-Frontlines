"""The test zip of change-set 2, second send: the same built files as the first zip, with the Firefox steps added to
the README (Mustafa's words of 2026-11-19, 13:36:37 UTC, point 4: "Put the Firefox steps in the zip's README and say
what I paste back."). Nothing is built here. Run from the repository root:

    python3 -I .checkpoints/cs2-first-page/make-zip-2.py <first stage folder> <new stage folder>

    <first stage folder>  the folder make-zip.py wrote: it holds files/ and the first zip
    <new stage folder>    must not exist yet

It refuses unless: the code at HEAD equals the code of the commit the first zip was built from; every built file is
byte for byte the one the first zip lists; and every line of the first README stands in the new one, in order (lines
are only added).
"""

import hashlib
import os
import shutil
import subprocess
import sys
import zipfile

NAME = "shamble-v2-02-first-page"
CODE = ["crates", "packages", "Cargo.toml", "Cargo.lock", "package.json", "package-lock.json"]

# Added before the line that starts the steps.
PARTS = """There are two parts: the steps in your own browser (1 to 12), then once in Firefox (F1 to F7, further
down). Do the Firefox steps before step 12, or start the server again for them.

"""

# Added after step 12.
FIREFOX = """Once in Firefox (about 8 minutes, 4 of them waiting)

The game has to work in Firefox too. Firefox was not run where this zip was built (there is no Firefox
there), so each "expected" below is what the page is built to do, not something that was seen. Whatever
differs is a finding.

 F1. The server from step 3 must be running (if you stopped it: type  node serve.mjs  again).
 F2. Open index.html of this folder as a file in Firefox: press Ctrl+O in Firefox and pick it, or drag
     the file into a Firefox window. Expected: the same one line of text about "node serve.mjs", and no
     picture. Firefox treats a page opened as a file differently from Brave, so it may show something
     else, for example the grid with a line that starts "The match stopped:". Note which.
 F3. Open http://127.0.0.1:4173 in Firefox. Expected: the same ground, grid and readout as in step 4,
     with errors 0. The readout's first line names what draws the picture: "backend: WebGPU", or
     "WebGL 2 fallback" with its reason. Either is an answer; which one it is, is what I need to know.
 F4. Leave it for 2 minutes. Errors should still be 0, and blocks still about equal to ticks.
 F5. Switch to another tab for 1 minute, then come back. "last pause" should say about 60 s hidden, and
     "ticks while hidden" 0 or a very small number.
 F6. Press "Copy report" and paste the result into the chat. (If the button cannot reach the clipboard,
     the text box appears, as in step 8.)
 F7. Open http://127.0.0.1:4173/?backend=webgl , wait 30 seconds, press "Copy report" and paste that too.
     The readout should say "WebGL 2 fallback, asked for by ?backend=webgl", with errors 0.

What to paste back from Firefox:
  - one line on what F2 showed;
  - the report of F6 and the report of F7. Each has a line "browser:" that names Firefox and its
    version, so nothing else is needed for that;
  - if something went wrong (a line of text over the picture, an error count above 0, a blank page, a
    readout that does not move): a screenshot of the page, and a screenshot of Firefox's console
    (press Ctrl+Shift+K) with its red lines.

"""

STEPS_LINE = "Steps (about 18 minutes, 9 of them waiting)"
LAST_STEP = "12. Back in PowerShell, press Ctrl+C to stop the server."


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


def sums_of(text):
    sums = {}
    for line in text.split("\r\n"):
        if line:
            digest, name = line.split("  ", 1)
            sums[name] = digest
    return sums


def main():
    first, stage = sys.argv[1].rstrip("/"), sys.argv[2].rstrip("/")
    first_zip = os.path.join(first, f"{NAME}.zip")
    if os.path.exists(stage):
        sys.exit(f"{stage} exists already: every zip gets a folder of its own")

    # What the first zip itself says: its README (the commit) and its list of sums.
    with zipfile.ZipFile(first_zip) as archive:
        old_readme = archive.read("README.txt").decode("utf-8")
        old_sums = sums_of(archive.read("SHA256SUMS.txt").decode("utf-8"))
    old_lines = old_readme.split("\r\n")
    assert old_lines[1].startswith("Built from commit "), old_lines[1]
    full = old_lines[1].split("(")[1].split(")")[0]
    if git("status", "--porcelain", "--", *CODE):
        sys.exit("the tree holds code changes that are not committed")
    if subprocess.run(["git", "diff", "--quiet", full, "HEAD", "--", *CODE]).returncode != 0:
        sys.exit(f"the code at HEAD is not the code of {full}: this zip must be built again, not repacked")

    # The README: the first one with two blocks added, and no other change. Checked before anything is written.
    at_steps = old_lines.index(STEPS_LINE)
    at_last = next(i for i, line in enumerate(old_lines) if line.strip() == LAST_STEP)
    assert old_lines[at_last + 1] == "", "a blank line is expected after step 12"
    new_lines = (
        old_lines[:at_steps]
        + PARTS.split("\n")[:-1]
        + old_lines[at_steps : at_last + 2]
        + FIREFOX.split("\n")[:-1]
        + old_lines[at_last + 2 :]
    )
    rest = iter(new_lines)
    assert all(any(line == other for other in rest) for line in old_lines), "a line of the first README is gone"
    widest = max(len(line) for line in old_lines)
    assert all(len(line) <= widest for line in new_lines), f"a README line is longer than the first README's widest ({widest})"

    # The built files: copied from the first stage, each checked against the first zip's own list.
    files = os.path.join(stage, "files")
    shutil.copytree(os.path.join(first, "files"), files)
    built = [n for n in files_of(files) if n not in ("README.txt", "SHA256SUMS.txt")]
    assert sorted(built) == sorted(n for n in old_sums if n != "README.txt"), (built, sorted(old_sums))
    for name in built:
        assert sha256(os.path.join(files, name)) == old_sums[name], name
    with open(os.path.join(files, "README.txt"), "w", encoding="utf-8", newline="") as handle:
        handle.write("\r\n".join(new_lines))
    listed = [n for n in files_of(files) if n != "SHA256SUMS.txt"]
    with open(os.path.join(files, "SHA256SUMS.txt"), "w", encoding="utf-8", newline="\r\n") as handle:
        for name in listed:
            handle.write(f"{sha256(os.path.join(files, name))}  {name}\n")

    out = os.path.abspath(os.path.join(stage, f"{NAME}.zip"))
    top = sorted(os.listdir(files))
    subprocess.run([sys.executable, "-I", "-m", "zipfile", "-c", out, *top], check=True, cwd=files)

    # The control: unpack the new zip elsewhere; compare every file with the list inside it and with the first zip.
    check = os.path.join(stage, "unpacked")
    with zipfile.ZipFile(out) as archive:
        members = sorted(i.filename for i in archive.infolist() if not i.is_dir())
        archive.extractall(check)
    with open(os.path.join(check, "SHA256SUMS.txt"), encoding="utf-8", newline="") as handle:
        sums = sums_of(handle.read())
    assert sorted(sums) == [m for m in members if m != "SHA256SUMS.txt"], (sorted(sums), members)
    for name, digest in sums.items():
        assert sha256(os.path.join(check, name)) == digest, name
    same = sorted(n for n in sums if sums[n] == old_sums.get(n))
    differs = sorted(n for n in sums if sums[n] != old_sums.get(n))
    assert differs == ["README.txt"] and sorted(sums) == sorted(old_sums), (differs, sorted(sums), sorted(old_sums))

    print(f"zip: {os.path.relpath(out)}")
    print(f"first zip: {os.path.relpath(first_zip)}, sha256 {sha256(first_zip)}")
    print(f"built from commit {full[:9]} ({full}); the code at HEAD ({git('rev-parse', 'HEAD')}) is the same")
    print(f"size {os.path.getsize(out)} bytes, sha256 {sha256(out)}")
    print(f"{len(members)} files, each equal to its line in SHA256SUMS.txt after unpacking:")
    for name in members:
        print(f"  {os.path.getsize(os.path.join(check, name)):>9}  {name}")
    print(f"equal to the first zip's file, by sha256 ({len(same)}): {', '.join(same)}")
    print("differs from the first zip: README.txt (and with it SHA256SUMS.txt, which lists it)")
    print(f"README.txt: {len(old_lines)} lines before, {len(new_lines)} now; every line of the first one stands, in order")
    print(f"unpacked copy for the browser tests: {os.path.relpath(check)}")


main()
