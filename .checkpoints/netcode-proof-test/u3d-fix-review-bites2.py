# U3d fix-pass review (test-engineer), add-on to u3d-fix-review-bites.py: my build review's S4 and S5 (finding 2: cause
# mislabelled as the loss band) re-run on the fixed code, and the env verdict word fixed (the first script's F4-R line
# printed "INVALID (informative)" because its "no INVALID" text contains the word; the data line beside it is right).
# It loads the first script's definitions (everything above its last dispatch lines) and runs only these bites.
# Run: python3 -I <this> <spike dir>
import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent
src = (HERE / "u3d-fix-review-bites.py").read_text()
cut = src.index("\nstart_check()\n")
body = src[:cut].replace('verdict = "INVALID (informative)" if "INVALID" in inv',
                         'verdict = "INVALID (informative)" if inv.startswith("INVALID")')
sys.argv = [sys.argv[0], sys.argv[1], "bites"]
g = {"__file__": str(HERE / "u3d-fix-review-bites.py"), "__name__": "bites2"}
exec(compile(body, "u3d-fix-review-bites.py", "exec"), g)
g["BITES"][:] = [
    ("F2-S4", "fate log capacity n (cap = n): my review's S4", "self",
     [(g["TESTS"], "    let cap = (2 * n + 1_000) as usize;", "    let cap = n as usize;")], True,
     "the fate log filled"),
    ("F2-S5", "the pre-touch's drain removed (log full from the start): my review's S5", "self",
     [(g["TESTS"], g["DRAIN"], "    let rt = runtime(4);\n")], True, "the fate log filled"),
]
g["start_check"]()
g["group_bites"]()
for p, h in g["WANT"].items():
    assert g["sha"](p) == h, p
g["log"]("end: every file == fixed state (bites2)")
