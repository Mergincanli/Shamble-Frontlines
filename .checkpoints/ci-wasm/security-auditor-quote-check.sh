#!/usr/bin/env bash
# security-auditor, part ci-text (2026-11-19): how the outer shell hands the cut's script (proposed toolchain.yml lines
# 139-190) to `bash -c`. The privileged prefix (sudo, unshare, setpriv, env -i) is replaced by a function that only
# prints the arguments it gets; the script `bash -c` would receive is then only PARSED (`bash -n`), never run.
# Run from the repository's root: bash .checkpoints/ci-wasm/security-auditor-quote-check.sh [file]
set -u
F="${1:-.gate/ci-wasm/proposed/.github/workflows/toolchain.yml}"
inner=$(sed -n '139,190p' "$F")
case "$inner" in *"bash --norc --noprofile -euo pipefail -c '"*) ;; *) echo "line 139 is not the cut's bash -c line"; exit 2 ;; esac
show_fn='show() { while [ "$#" -gt 0 ] && [ "$1" != "-c" ]; do shift; done; shift
  printf "bash -c gets %s argument(s) after -c\n" "$#"
  printf "  the script ends with: %s\n" "$(printf "%s" "$1" | tail -c 60 | tr "\n" " ")"
  if [ "$#" -gt 1 ]; then printf "  extra words: [%s] [%s] [%.40s...]\n" "$2" "${3-}" "${4-}"; fi
  printf "%s" "$1" > "$OUT"; }'
script="$show_fn
show${inner#*bash}"
OUT=$(mktemp -p "$PWD/.gate/ci-wasm/security" inner-XXXXXX.sh)
echo "== the outer run text: bash -n"; printf '%s\n' "$script" | bash -n && echo "  the outer text parses (exit 0)"
echo "== the arguments bash -c receives"; printf '%s\n' "$script" | OUT="$OUT" bash --noprofile --norc -eo pipefail
echo "== the script bash -c receives: bash -n (parse only)"; bash -n "$OUT"; echo "  exit $?"
