#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
quickstart="${repo_root}/QUICKSTART.md"
snippet="${repo_root}/examples/quick-example/src/quickstart_snippet.rs"

if [[ ! -f "$quickstart" ]]; then
  echo "Missing QUICKSTART.md at ${quickstart}"
  exit 1
fi

if [[ ! -f "$snippet" ]]; then
  echo "Missing quick example snippet at ${snippet}"
  exit 1
fi

tmp="$(mktemp)"
trap 'rm -f "$tmp"' EXIT

awk '
  /^## Quick example$/ { found = 1; next }
  found && /^```rust$/ { in_block = 1; next }
  in_block && /^```$/ { exit }
  in_block { print }
' "$quickstart" > "$tmp"

if ! diff -u "$tmp" "$snippet"; then
  echo "QUICKSTART.md Quick example code does not match examples/quick-example/src/quickstart_snippet.rs"
  exit 1
fi

echo "QUICKSTART.md Quick example matches examples/quick-example/src/quickstart_snippet.rs"
