# checker-CL (R7 legacy harvest, fixes2 checker for area CL)
- Ran apply-ops.py CL: 3 ops applied, only CL14 what/where/evidence changed (diffed final vs after: no other row/field touched).
- Op texts equal the correction's quoted texts exactly (python compare: True x3); evidence replace swapped only the last sentence.
- CL14: spot-checked every cited line in core/game/editor + reports + commits 5695e56, b5f4ecf, a4d7289: all hold, EXCEPT
  'where' says "nature + baseCol" set at :1414, :1417, game:3070 and "toonify copies both" at :284. Code: those sites set only
  userData.nature; baseCol is set only at core:230 (sea) and is created fresh by toonify at :284 (m.color.clone() of the toon
  material), not copied. Problem reported with a replace op.
DONE
