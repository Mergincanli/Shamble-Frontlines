#!/usr/bin/env bash
# usage: fetch.sh <name> <url>   raw page goes to the session scratchpad pages dir (not kept); audit row appended here
P=/tmp/claude-0/-home-user-Shamble-Frontlines/a12ff881-d66e-5ffe-b01e-ff5fa7f27269/scratchpad/pages
A=/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/verifier-fixcheck-d0-scratch/audit.tsv
name="$1"; url="$2"
t=$(TZ=Europe/Amsterdam date +%H:%M:%S)
host=$(printf '%s' "$url" | sed -E 's#^https?://([^/]+).*#\1#')
w=$(curl -sSL --max-time 30 -o "$P/$name" -w '%{http_code}\t%{size_download}\t%{time_total}' "$url" 2>"$P/$name.err") || w="ERR($(head -c 200 "$P/$name.err" | tr '\n\t' '  '))\t0\t0"
printf '%s\t%s\t%s\t%s\n' "$t" "$host" "$url" "$w" >> "$A"
printf '%s\t%s\t%s\n' "$name" "$url" "$w"
