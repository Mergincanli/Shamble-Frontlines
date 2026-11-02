# touchup2 engine-architect synthesis fixes (fixcheck-d2 SF1)

### FIX SF1
FILE: docs/SPIKE_PLAN.md
BEFORE:
  plain glTF/GLB exports load on both backends with the engine's shared materials per slot class, every model listed
  by id, a broken link refused at load, and Mustafa's pick per asset family (OQ-0005-AP2).
AFTER:
  plain glTF/GLB exports load on both backends with the engine's shared materials, one per slot content (ADR 0005 GPU
  part 1), every model listed by id, a broken link refused at load, and Mustafa's pick per asset family (OQ-0005-AP2).
END

