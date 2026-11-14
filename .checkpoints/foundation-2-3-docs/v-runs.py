# Verifier (read-only): parse a saved actions_list (list_workflow_runs) page; print id, workflow name, event, head
# sha (7), status, conclusion, created/updated times.
#   python3 -I .checkpoints/foundation-2-3-docs/v-runs.py <page.txt>
import json, sys
raw = open(sys.argv[1], encoding='utf-8').read()
a = raw.find('{'); b = raw.rfind('}')
d = json.loads(raw[a:b+1])
runs = d.get('workflow_runs') or d.get('runs') or []
print('total_count', d.get('total_count'), 'listed', len(runs))
for r in sorted(runs, key=lambda r: r.get('id')):
    print(r.get('id'), (r.get('name') or '')[:12], r.get('event'), (r.get('head_sha') or '')[:7], r.get('status'), r.get('conclusion'), r.get('created_at'), r.get('updated_at'), 'attempt', r.get('run_attempt'))
