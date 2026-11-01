#!/usr/bin/env python3
"""Build ADR 0005's draft: the frame (top.md, bottom.md) around the four sections, copied byte for byte.
Each section is sliced from its author's file up to (not including) "### Appendix A rows (for the frame)" (the
content file holds two sections, split at "## The asset pipeline (engine-architect)"). The sections' Appendix A rows
and the content section's inventory rows are copied verbatim into the frame's placeholders.
Run from anywhere: python3 build_draft.py"""
import pathlib, sys

HERE = pathlib.Path(__file__).resolve().parent
D = HERE.parent
APX = '### Appendix A rows (for the frame)'
FTF = '### For the frame'

def read(p):
    return (D / p).read_text(encoding='utf8')

content_file = read('engine-architect-adr0005-content-section.md')
abil_file = read('game-designer-adr0005-abilities-section.md')
gpu_file = read('graphics-engineer-adr0005-assets-section.md')

AP_HEAD = '## The asset pipeline (engine-architect)'
assert content_file.count(AP_HEAD) == 1 and content_file.count(APX) == 1
i_ap = content_file.index(AP_HEAD)
i_cx = content_file.index(APX)
sec_content = content_file[:i_ap]
sec_assets = content_file[i_ap:i_cx]
assert abil_file.count(APX) == 1 and gpu_file.count(APX) == 1
sec_abil = abil_file[:abil_file.index(APX)]
sec_gpu = gpu_file[:gpu_file.index(APX)]
sections = [('Content data', sec_content), ('The ability and effect system', sec_abil),
            ('The asset pipeline', sec_assets), ('The asset pipeline: GPU rules', sec_gpu)]
for name, s in sections:
    assert APX not in s and FTF not in s, name
    assert s.endswith('\n\n'), name  # each slice ends with the blank line before the next heading

def table_rows(text, start_marker, header_prefix='| id | disposition | where or why |'):
    """Rows of the first table after start_marker, below its header and separator, verbatim (with newline)."""
    t = text[text.index(start_marker):]
    lines = t.split('\n')
    out, state = [], 0
    for ln in lines:
        s = ln.strip()
        if state == 0 and s.startswith(header_prefix):
            state = 1; continue
        if state == 1:
            assert s.startswith('|---'), ln
            state = 2; continue
        if state == 2:
            if s.startswith('|'):
                out.append(ln)
            else:
                break
    return out

apx_rows = (table_rows(content_file, APX) + table_rows(abil_file, APX) + table_rows(gpu_file, APX))
assert len(apx_rows) == 34, len(apx_rows)
ftf = content_file[content_file.index(FTF):]
inv_rows = table_rows(ftf, '**Legacy inventory rows**',
                      header_prefix='| Row id (`LEGACY_INVENTORY.md` line) | Mark now | Proposed mark | Why |')
assert len(inv_rows) == 7, len(inv_rows)

top = (HERE / 'top.md').read_text(encoding='utf8').rstrip('\n') + '\n\n'
bottom = (HERE / 'bottom.md').read_text(encoding='utf8')
assert bottom.count('{{INV_ROWS}}\n') == 1 and bottom.count('{{APPENDIX_ROWS}}\n') == 1
bottom = bottom.replace('{{INV_ROWS}}\n', '\n'.join(inv_rows) + '\n')
bottom = bottom.replace('{{APPENDIX_ROWS}}\n', '\n'.join(apx_rows) + '\n')
assert '{{' not in bottom

draft = top + ''.join(s for _, s in sections) + bottom
out = D / 'engine-architect-adr0005-draft.md'
out.write_text(draft, encoding='utf8')

# checks: every section appears exactly once, byte for byte, in order; every copied row appears once
pos = -1
for name, s in sections:
    assert draft.count(s) == 1, name
    p = draft.index(s); assert p > pos, name; pos = p
for r in apx_rows + inv_rows:
    assert draft.count(r + '\n') == 1, r[:60]
print(f'wrote {out.name}: {len(draft.splitlines())} lines; sections ' +
      ', '.join(f'{n} ({len(s.splitlines())} lines)' for n, s in sections) +
      f'; Appendix A rows {len(apx_rows)}; inventory rows {len(inv_rows)}')
