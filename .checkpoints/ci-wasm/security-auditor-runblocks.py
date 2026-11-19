# security-auditor, part ci-text (2026-11-19): writes each step's `run:` text of a workflow file to its own file
# under the given output folder, as GitHub would hand it to `bash --noprofile --norc -eo pipefail {0}`, so that each
# can be checked with `bash -n` (parse only). Reads the YAML; runs nothing.
# Usage: python3 -B -I .checkpoints/ci-wasm/security-auditor-runblocks.py <workflow.yml> <out folder>
import pathlib
import sys

import yaml

workflow = yaml.safe_load(pathlib.Path(sys.argv[1]).read_text(encoding="utf-8"))
out = pathlib.Path(sys.argv[2])
out.mkdir(parents=True, exist_ok=False)
for job_name, job in workflow["jobs"].items():
    for i, step in enumerate(job["steps"], 1):
        if "run" in step:
            target = out / f"{job_name}-step{i}.sh"
            target.write_text(step["run"], encoding="utf-8")
            print(f"{target.name}: {step.get('name', '(no name)')[:70]}")
