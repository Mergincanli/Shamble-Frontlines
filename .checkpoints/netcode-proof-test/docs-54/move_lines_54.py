"""DOCS-54: the plan's two scripted moves (docs-54-plan-draft.md §1 and §3 step 3, with C9's fold place), run once in
the worktree, each checked by reading the file back.
(1) docs/ROADMAP.md: the three oldest "Recent work" entries (b1-plan-view :213, kit-v3-3 :215, kit-v3-2 :217 on main
    82c605e, with their blank lines :212, :214, :216) move into "Build history" right after main's :84 (the last dated
    entry, B3), in date order (kit-v3-2, kit-v3-3, b1-plan-view), each line kept byte for byte with "- " in front.
(2) docs/STATUS_ARCHIVE.md: two "Follow-ups" subsections right after main's :869 (the end of "From foundation 2.2"):
    "From KEPT-LOGS" with PR #53's description lines 88-105 and "From the night-shift lines" with PR #54's lines 23-27,
    each line copied byte for byte from the GitHub MCP results recorded in this session's transcript (the last
    pull_request_read of each PR), with two spaces in front so they nest under the subsection's lead line. The lead
    lines are written for the archive (the session's text).
It writes only when both files still equal main's blobs (so a second run exits 1). Run from the main folder's root:
  python3 -I .checkpoints/netcode-proof-test/docs-54/move_lines_54.py <worktree>
Exit codes: 0 done · 1 a check failed or a file is not main's · 2 usage."""
import hashlib
import json
import os
import subprocess
import sys

REPO = "/home/user/Shamble-Frontlines"
T = "/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2.jsonl"
MAIN = "82c605e2e8a7f284e6925718368d3fe569f89ae9"
FAILS = []


def check(ok, what):
    print(("ok   " if ok else "FAIL ") + what)
    if not ok:
        FAILS.append(what)
    return ok


def sha(s):
    return hashlib.sha256(s.encode("utf-8")).hexdigest()


def git(*args):
    return subprocess.run(["git", *args], cwd=REPO, capture_output=True, check=True).stdout.decode("utf-8")


def blob_of(path):
    return subprocess.run(["git", "hash-object", path], capture_output=True, check=True).stdout.decode().strip()


def pr_body(pr):
    """The body of the last pull_request_read 'get' result for this PR in the transcript, with its line and id."""
    found = []
    with open(T, encoding="utf-8") as fh:
        for n, line in enumerate(fh, 1):
            if f'\\"number\\":{pr},' not in line and f'"number":{pr},' not in line:
                continue
            r = json.loads(line, strict=False)
            c = (r.get("message") or {}).get("content")
            if not isinstance(c, list):
                continue
            for b in c:
                if not (isinstance(b, dict) and b.get("type") == "tool_result"):
                    continue
                cc = b.get("content")
                texts = [cc] if isinstance(cc, str) else [x.get("text") for x in cc or [] if isinstance(x, dict)]
                for t in texts:
                    if isinstance(t, str) and t.startswith('{"number":%d,' % pr):
                        found.append((n, r.get("timestamp"), b.get("tool_use_id"), json.loads(t)["body"]))
    n, ts, tid, body = found[-1]
    print(f"PR #{pr}: transcript line {n}, {ts}, tool_use_id {tid}, body {len(body.split(chr(10)))} lines")
    return body.split("\n"), (n, ts, tid)


def main():
    if len(sys.argv) != 2:
        print(__doc__)
        return 2
    wt = sys.argv[1]
    rm_path, ar_path = os.path.join(wt, "docs/ROADMAP.md"), os.path.join(wt, "docs/STATUS_ARCHIVE.md")
    check(git("rev-parse", "origin/main").strip() == MAIN, f"origin/main is {MAIN}")
    rm_main, ar_main = git("show", f"{MAIN}:docs/ROADMAP.md"), git("show", f"{MAIN}:docs/STATUS_ARCHIVE.md")
    rm_ok = check(blob_of(rm_path) == git("rev-parse", f"{MAIN}:docs/ROADMAP.md").strip(),
                  "the worktree's ROADMAP.md is main's blob")
    ar_ok = check(blob_of(ar_path) == git("rev-parse", f"{MAIN}:docs/STATUS_ARCHIVE.md").strip(),
                  "the worktree's STATUS_ARCHIVE.md is main's blob")
    if not (rm_ok and ar_ok):
        print("REFUSED: nothing written")
        print("FAILED: " + "; ".join(FAILS))
        return 1

    # (1) ROADMAP's fold.
    L = rm_main.split("\n")          # L[i] is line i+1; the last element is "" after the final newline
    check(len(L) == 218 and L[217] == "", "ROADMAP on main: 217 lines and a final newline")
    entries = {213: "**EDITOR_V2's plan view and Mustafa's editor decisions [2026-11-06, MERGED]**",
               215: "**Kit v3.3, the guard's follow-ups [2026-11-05, MERGED]**",
               217: "**Kit v3.2, checkpoint rules for every session [2026-11-04, MERGED]**"}
    for ln, head in entries.items():
        check(L[ln - 1].startswith(head), f"ROADMAP:{ln} starts {head[:40]!r}...")
    check(L[211] == L[213] == L[215] == "" and L[210].startswith("**VISION_V2's community-servers note"),
          "ROADMAP:212, :214, :216 blank; :211 is b2-community-servers' entry")
    check(L[83].endswith("report `docs/reports/2026-11-04-b3-design-walk.md`.")
          and L[84].startswith("- **UI theme step 1 (2026-10-02):**"), "ROADMAP:84 ends B3's entry; :85 is UI theme step 1")
    moved = [L[216], L[214], L[212]]  # kit-v3-2, kit-v3-3, b1-plan-view: date order
    new_rm = L[:84] + ["- " + x for x in moved] + L[84:211] + [""]
    rm_text = "\n".join(new_rm)

    # (2) The archive's two subsections.
    A = ar_main.split("\n")
    check(A[868].endswith("it [verified 2026-11-13].") and A[869] == "" and A[870] == "## Decisions pending: the rest",
          "ARCHIVE:869 ends the Follow-ups' last item; :870 blank; :871 the next section")
    b53, src53 = pr_body(53)
    b54, src54 = pr_body(54)
    r53, r54 = b53[87:105], b54[22:27]   # lines 88-105 and 23-27
    check(r53[0].startswith("- **security-auditor: APPROVE WITH FINDINGS**") and r53[-1] == "  - KLT-8: the links (fixed in this report).",
          "#53's lines 88-105: from the security-auditor's bullet to KLT-8")
    check(sum(x.startswith("  - SA-KL-") for x in r53) == 4 and sum(x.startswith("  - KLT-") for x in r53) == 8,
          "#53's lines hold SA-KL-1 to -4 and KLT-1 to -8")
    check(r54[0].startswith("- **The advisor: APPROVE WITH FINDINGS**") and r54[-1].startswith("  - The other five are follow-ups."),
          "#54's lines 23-27: from the advisor's bullet to the five follow-ups")
    lead53 = [
        "- **From KEPT-LOGS** (PR #53, merged as 53272a1 on 2026-11-15; its one review round on a637c7a; Mustafa accepted",
        "  these as left open for that pull request, `docs/DECISIONS.md`: the entry \"Taken by Mustafa, 2026-11-15, 19:46:36",
        "  UTC: KL-MERGE (a), merge #53, and the security and test findings accepted as left open\"; for the kit's next change",
        "  after the freeze, queued in `docs/DECISIONS.md`; copied by script from the pull request's description, its lines",
        "  88-105):",
    ]
    lead54 = [
        "- **From the night-shift lines** (PR #54, merged as 82c605e on 2026-11-17; its one review round on 20f9d59; the",
        "  seven Lows were deferred in writing by the session, and the pull request merged on his go, `docs/DECISIONS.md`:",
        "  the entry \"Taken by Mustafa, 2026-11-17, 07:00:54 UTC: MERGE-54, his go for #54\"; ADV-NS-L1 and SA-NS-1 went to",
        "  him as NS-TRAILER and NS-FIRST, his (b) for both, queued in `docs/DECISIONS.md` for the kit's next change after",
        "  the freeze; ADV-NS-L5 is carried out by DOCS-54, the docs pass that adds this; copied by script from the pull",
        "  request's description, its lines 23-27):",
    ]
    ins = lead53 + ["  " + x for x in r53] + lead54 + ["  " + x for x in r54]
    new_ar = A[:869] + ins + A[869:]
    ar_text = "\n".join(new_ar)

    if FAILS:
        print("REFUSED: nothing written")
        print("FAILED: " + "; ".join(FAILS))
        return 1
    with open(rm_path, "w", encoding="utf-8") as fh:
        fh.write(rm_text)
    with open(ar_path, "w", encoding="utf-8") as fh:
        fh.write(ar_text)

    # Read back.
    rb = open(rm_path, encoding="utf-8").read()
    check(rb == rm_text, "ROADMAP read back equals what was written")
    R = rb.split("\n")
    check([x[2:] for x in R[84:87]] == moved and all(x.startswith("- **") for x in R[84:87]),
          "ROADMAP:85-87 are the three entries with '- ' in front, byte for byte")
    check(R[87].startswith("- **UI theme step 1") and R[83] == L[83], "ROADMAP: B3's entry above, UI theme step 1 below")
    check(not any(x in R[87:] for x in moved) and R[-2].startswith("**VISION_V2's community-servers note"),
          "ROADMAP: the three are gone from Recent work, which now ends with b2-community-servers' entry")
    check(rb.replace("\n" + "\n".join("- " + x for x in moved), "", 1) == "\n".join(L[:211]) + "\n",
          "ROADMAP: removing the three moved lines gives main's lines 1-211 exactly")
    for x in moved:
        print(f"moved: sha256 {sha(x)} {x[:70]!r}...")
    ab = open(ar_path, encoding="utf-8").read()
    check(ab == ar_text, "ARCHIVE read back equals what was written")
    B = ab.split("\n")
    got53 = B[869 + len(lead53):869 + len(lead53) + len(r53)]
    o54 = 869 + len(lead53) + len(r53)
    got54 = B[o54 + len(lead54):o54 + len(lead54) + len(r54)]
    check([x[2:] for x in got53] == r53 and all(x.startswith("  ") for x in got53),
          "ARCHIVE: #53's 18 lines, two spaces off, equal the description's lines 88-105 byte for byte")
    check([x[2:] for x in got54] == r54 and all(x.startswith("  ") for x in got54),
          "ARCHIVE: #54's 5 lines, two spaces off, equal the description's lines 23-27 byte for byte")
    check(B[:869] == A[:869] and B[869 + len(ins):] == A[869:], "ARCHIVE: every other line is main's, unchanged")
    print(f"#53 lines 88-105: sha256 {sha(chr(10).join(r53))} (source, transcript line {src53[0]}, {src53[2]})")
    print(f"#54 lines 23-27: sha256 {sha(chr(10).join(r54))} (source, transcript line {src54[0]}, {src54[2]})")
    print(f"ROADMAP after: {rb.count(chr(10))} lines, sha256 {sha(rb)}; ARCHIVE after: {ab.count(chr(10))} lines, sha256 {sha(ab)}")
    print(("FAILED: " + "; ".join(FAILS)) if FAILS else "ALL CHECKS PASS")
    return 1 if FAILS else 0


if __name__ == "__main__":
    sys.exit(main())
