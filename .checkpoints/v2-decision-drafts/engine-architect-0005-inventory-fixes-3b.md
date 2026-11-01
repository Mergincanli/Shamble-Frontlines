# engine-architect: ADR 0005 third fix round (b), legacy inventory row GW10

Against `docs/adr/0005-content-abilities-assets.md` at aa85bf0; each BEFORE copied by script, occurring exactly once.

### FIX GW10 (the table's introduction)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  as engine-architect, the author of the content data and asset pipeline sections, marks them, plus ED5, ED7, GW6,
  GW7, GW8, GW11 and GE7, which no draft had placed:
AFTER:
  as engine-architect, the author of the content data and asset pipeline sections, marks them, plus ED5, ED7, GW6,
  GW7, GW8, GW10, GW11 and GE7, which no draft had placed:
END

### FIX GW10 (the row, after GW7)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  | GW7 (195) | port to TypeScript (pending) | unchanged: the platform and wedge builders take the map document's parameters (w, d, h0, h1, rot; CD-11), and the slope that movement, shots and sight use stays the sim's (GW14's row); pending becomes "OQ-0005-AP2 (the ground family)", and until it is answered the greybox reads height through brightness (LOOP_V2 §9) | LOOK_V2 §3 asks for verticality and sets no look for ramps or platforms |
AFTER:
  | GW7 (195) | port to TypeScript (pending) | unchanged: the platform and wedge builders take the map document's parameters (w, d, h0, h1, rot; CD-11), and the slope that movement, shots and sight use stays the sim's (GW14's row); pending becomes "OQ-0005-AP2 (the ground family)", and until it is answered the greybox reads height through brightness (LOOP_V2 §9) | LOOK_V2 §3 asks for verticality and sets no look for ramps or platforms |
  | GW10 (198) | redesign (pending) | unchanged: spawns become team objectives in the map document (CD-11; EDITOR_V2 §3, "objectives as data"), and spawning inside the barracks is the sim's rule (LOOP_V2 §2; ADR 0004); the pending note is lifted, since the markers are drawn only on the editor's layer, whose scene EDITOR_V2's own architecture plan builds (CD-11) | no LOOK_V2 item names editor-only markers, and ADR 0003 draws nothing of this row |
END
