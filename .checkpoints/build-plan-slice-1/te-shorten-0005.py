"""Replace column 3 ("what is measured") of chosen rows in marks-0005.md with shorter text (numbers unchanged)."""
import sys

PATH = sys.argv[1]
NEW = {
    "2": "100 % of doubles give the same 64-bit pattern in native Rust, Wasm and `JSON.parse`; corpus: 2,038 legacy numbers, 10,000 generated (5,000 random bit patterns, 5,000 decimals ≤ 17 digits), −0, largest and smallest finite",
    "3": "100 % of PM-0005-2's corpus written back as the same text; writer byte-identical native and Wasm, idempotent; 9,007,199,254,740,993 (2^53 + 1) in a content file refused at load, naming file and field",
    "4": "over C (10,000 files): 0 crashes, 0 accepted invalid, 100 % controls accepted; native = Wasm in verdict and error record; changed path named 100 %; 50 MB, 10,000-deep refused by stage; missing client entry refused",
    "5": "every bounded collection: one over refused 100 %, at bound accepted 100 %; 0 unbounded collections; per test mode, each pool's reported worst case (active effects: reserve per target) equals a hand count 100 %; unbounded pools named, never defaulted",
    "7": "100 % of garde-ranged fields: same minimum and maximum (inclusive or exclusive) in the schema; just outside (bound ± 1 integers, next double floats) refused by Rust and flagged by the schema-driven field; at bound accepted by both",
    "8": "regenerated schema and `content`'s ts-rs types: 0 changed bytes; `tsc --strict` passes; rename without regenerating, or a hand edit to a generated file, fails the gate; 100 % of import files, typed against them, pass Rust validation",
    "9": "Cabana, DM_Tri, every map Mustafa supplies migrate and validate 100 %; migrate → save → load → save byte-identical 100 %; proptest 0 failures in 10,000 cases; future version refused naming file, version, newest known; goldens, insta pass",
    "10": "20 Shabzon 8 → 9 saves validated within 1 s, used next match, running-match hashes = no-save control; 20 malformed saves: error within 1 s; 20 feel-kit saves shown within 1 s; local play too",
    "11": "client Wasm sim applies the change at the server's tick, 0 mispredictions beyond it, under P2; 50 random valid edits in a 5-minute bot match: 0 crashes, 0 desyncs (server and sampled client hashes)",
    "12": "3 tampering cases (stale file, changed byte, missing file) ×10: refused or re-fetched pre-match, 0 matches on wrong hash; repeat visit `transferSize` 0; record hash resolves to archived set, and mode id; Rust SHA-256 = WebCrypto 100 %",
    "13": "C via the private-room override path: 0 accepted invalid, 0 server crashes; 100 % valid test-allow-list overrides accepted; off-list tunable refused 100 %; `__proto__` refused; document layer leaves `Object.prototype` and content prototypes unchanged",
    "14": "100 % non-look, non-state values mapped (v2 path, legacy `where`) or dropped (reason), per leaf: 725 leaves + every `boot_changes` entry; 2,011 look leaves listed; dead keys CD4, CD6, CD7, CD10, CD12, CD14 handled; kept values bit-exact",
    "16": "load time of the imported legacy tables + Cabana, DM_Tri + two modes (24v24, 16v16v16), median and max of 20 cold loads, native and Worker; bytes serde_json and garde add to the Worker's Wasm, raw and gzip",
    "19": "test-only Riot Shield block on a vehicle: 30 % of an SMG hit inside the arc (cos 0.30), 100 % behind; Banzai buff on another class: 1.3 × speed for 10 s, then 1 ×; 0 Rust changes",
    "20": "effect removed on the tick its source's death is processed, 1,000/1,000; last-hit and last-periodic-tick triggers move victim's item to source, 1,000/1,000; two kills of one victim in one tick resolved alike native and Wasm",
    "21": "Shabzon saved as 9: 0 Rust changes; running match hit deals 8, next match 9; a Conscript (100 HP) dies on the 12th body hit, not the 13th (9 × 12 = 108 ≥ 100; 8 × 12 = 96 < 100)",
    "22": "seven legacy weapons + Juggernaut cannon: head 2.6, body 1.0, legs 0.65 (Bagpipe head 1.2); Shabzon falloff at 10, 15, 25, 35, 50 m; pellets, magazine, reload, burst: integers exact, floats 1e-6 relative; shots within one tick (33.33 ms); bot = human",
    "23": "`WEAPONS`, `UNIT_TYPES`, `VEHICLE_TYPES`, ability fields convert: 0 unmapped, 0 mapped twice; unknown id, tag or out-of-range value fails at load naming file, field, every kind incl. tracks; 0 panics running C's accepted definitions 300 ticks",
    "24": "ability press before Tech 1, T2 purchase before Tech 2: refused 1,000/1,000; after the tier accepted 1,000/1,000; team without cashout reaches Tech 1 at tick 9,000 (5 × 60 × 30): refused at 8,999, accepted from 9,000",
    "25": "Warrior kills Conscript 0.86 s ± one tick (13 hits; 12 × 0.072 = 0.864 s); Peacekeeper kills Warrior 0.85 s with 11 of 16 pellets (11 × 11 = 121 ≥ 120 HP), 10 leave it alive; Shield: 63 SMG hits (62 leave 1.2 HP), 19 without; Molotov equal, Shield up or down; 35 m: Bagpipe 0, AMK 34; spawn protection 0 damage, full after 90 ticks (3 × 30); knockback within 1e-6 relative",
    "29": "looted X lost on death (fixture drop rule), another X taken: player's X picks apply; a second player taking the first gun gets own X picks, none of the first's; 1,000/1,000 cycles",
    "31": "offers hold the data's size (3) of qualifying candidates; short track per its data; forged or closed-offer picks refused 1,000/1,000; offered accepted 1,000/1,000; same seed: byte-identical offer logs, two runs and native vs Wasm",
    "32": "ability, effect and weapon step's p99 per tick in SS at 8, 16 and 32 active effects per soldier (384, 768, 1,536 at N = 48); bracket where it reaches 1 ms (points 4 apart added)",
    "33": "100 % of test models named by id in content and in the asset list; dangling asset id and unknown slot class each fail at load naming file, field (1/1 each); 0 stand-in models drawn",
    "34": "0 console errors per start; 0 listed-asset meshes with a material other than their slot content's shared material (cache identity); 0 model requests beyond listed glTF/GLB and their buffers, images; 0 outside the client's host",
    "41": "planted glTF reader dependency in `sim`, and in `server`, each fails the dependency check; planted client import of the asset tool's package fails the manifest walk; control: the tool depending on a glTF reader passes",
    "42": "0 unclassed slots; each faction slot's colour equals the faction's data colour (CD-13), every faction and model, read from materials; unlit pixel probe within 2/255 per channel on each backend; 0 console errors",
    "45": "per ability and variant at P2: input-to-own-response time (median, p95 over 200 presses); share confirmed by the server; rollbacks per 100 presses; largest correction from a rolled-back Shield slow; R1-ABL-REC4's marks if predicted variant built",
}

with open(PATH, encoding="utf-8") as f:
    lines = f.read().split("\n")
done = set()
for i, line in enumerate(lines):
    if not line.startswith("| PM-0005-"):
        continue
    cells = line.split(" | ")
    pid = cells[0][len("| PM-0005-"):]
    if pid in NEW:
        cells[2] = NEW[pid]
        lines[i] = " | ".join(cells)
        done.add(pid)
missing = sorted(set(NEW) - done)
with open(PATH, "w", encoding="utf-8") as f:
    f.write("\n".join(lines))
print("replaced", len(done), "missing", missing)
