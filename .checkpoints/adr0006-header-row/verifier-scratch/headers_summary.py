# Summarise a check-headers.mjs headers.json: per record, per form, every path's failures and headers.
import json, sys
d = json.load(open(sys.argv[1]))
print("checked_at", d["checked_at"], "summary", d["summary"])
for r in d["records"]:
    print("REC", r["deployment_id"][:8], r["leg"], "mode", r["mode"], "expect", r["expect"], "env", r["environment"])
    for f in r["forms"]:
        paths = [x["path"] for x in f["responses"]]
        fails = {x["path"]: x["failures"] for x in f["responses"] if x["failures"]}
        xr = sorted(set(str(x["headers"].get("x-robots-tag")) for x in f["responses"]))
        coop = sorted(set(str(x["headers"].get("cross-origin-opener-policy")) for x in f["responses"]))
        coep = sorted(set(str(x["headers"].get("cross-origin-embedder-policy")) for x in f["responses"]))
        st = sorted(set(x["status"] for x in f["responses"]))
        print("  FORM", f["form"], f["base"], "result", f["result"], "n_paths", len(paths), "status", st)
        print("    paths", paths)
        print("    x-robots", xr, "coop", coop, "coep", coep)
        wasm = [x["headers"].get("content-type") for x in f["responses"] if x["path"].endswith(".wasm")]
        print("    wasm ct", wasm)
        # distinct failure messages
        msgs = sorted(set(m for v in fails.values() for m in v))
        print("    fail paths", len(fails), "distinct msgs", msgs)
