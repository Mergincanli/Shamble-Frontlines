# engine-architect: ADR 0005 third fix round, legacy inventory rows (completeness critic item 5; condition 18)

Against `docs/adr/0005-content-abilities-assets.md` at 9ed8d67; each BEFORE copied by script, occurring exactly once.

### FIX GW6, GW7, GW8, GW11, GE7 (the table's introduction)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  as engine-architect, the author of the content data and asset pipeline sections, marks them, plus ED5 and ED7,
  which no draft had placed:
AFTER:
  as engine-architect, the author of the content data and asset pipeline sections, marks them, plus ED5, ED7, GW6,
  GW7, GW8, GW11 and GE7, which no draft had placed:
END

### FIX GW6, GW7, GW8, GW11, GE7 (the rows, after ED7)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  | ED7 (342) | port to TypeScript (pending) | unchanged here; pending becomes "OQ-0005-4" | EDITOR_V2 keeps the lab (lines 4–5); it tunes the cosmetic animation that ADR 0003's GA1–GA15 rows port, and its own note asks for "a data file the lab writes and the client loads", which client content (CD-6) can hold |
AFTER:
  | ED7 (342) | port to TypeScript (pending) | unchanged here; pending becomes "OQ-0005-4" | EDITOR_V2 keeps the lab (lines 4–5); it tunes the cosmetic animation that ADR 0003's GA1–GA15 rows port, and its own note asks for "a data file the lab writes and the client loads", which client content (CD-6) can hold |
  | GW6, GW8 (194, 196) | redesign (pending) | unchanged: the registries and the placement glue split into the map document (CD-11), the sim's colliders, never read from a model (AP-1), and a per-entity scene on the page (EDITOR_V2 §2 rules 1, 3 and 4); pending becomes "OQ-0005-AP2" for the crates, walls, roads, houses, covers and props, and "the tree style, LOOK_V2 §8 `[OPEN]`" for the trees | LOOK_V2 §8 allows authored models without requiring them and names none of these pieces; the tree style is Mustafa's open item |
  | GW7 (195) | port to TypeScript (pending) | unchanged: the platform and wedge builders take the map document's parameters (w, d, h0, h1, rot; CD-11), and the slope that movement, shots and sight use stays the sim's (GW14's row); pending becomes "OQ-0005-AP2 (the ground family)", and until it is answered the greybox reads height through brightness (LOOP_V2 §9) | LOOK_V2 §3 asks for verticality and sets no look for ramps or platforms |
  | GW11 (199) | redesign (pending) | unchanged (EDITOR_V2 §1–2: per-entity commands instead of a whole-map rebuild); for the environment the pending note is lifted: the mood is map data (CD-11), lit by ADR 0003 (its part 5.g), and its values are tuning in ADR 0003's look proof test; for the terminal mesh, pending becomes "OQ-0005-AP2" | LOOK_V2 §6.5 `[DEFAULT]` sets "different moods per map"; the structures' look follows the families' answer |
  | GE7 (293) | port to TypeScript (pending) | unchanged (the technique carries over to uplinks and harvesters; the silos and the tech box go with LOOP_V2 §4); the pending note is lifted for the glow, since objectives are emissive through emissive slots (AP-3; the GPU section's part 2), and stays for the shapes as "OQ-0005-AP2"; their colours are ADR 0003's tuning | LOOK_V2 §6.4 `[DEFAULT]`: "objectives, faction tech and muzzle flashes are emissive"; §5's iconic silhouettes come with the families' answer |
END
