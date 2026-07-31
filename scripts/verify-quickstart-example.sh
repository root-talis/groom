#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
snippet="${repo_root}/examples/quick-example/src/quickstart_snippet.rs"

if [[ ! -f "$snippet" ]]; then
  echo "Missing quick example snippet at ${snippet}"
  exit 1
fi

extract_after_quick_example_heading() {
  awk '
    /^## Quick example$/ { found = 1; next }
    found && /^```rust$/ { in_block = 1; next }
    in_block && /^```$/ { exit }
    in_block { print }
  ' "$1"
}

extract_first_rust_block() {
  awk '
    /^```rust$/ { in_block = 1; next }
    in_block && /^```$/ { exit }
    in_block { print }
  ' "$1"
}

check_inclusion() {
  local file="$1" extractor="$2" description="$3"
  local tmp

  if [[ ! -f "$file" ]]; then
    echo "Missing ${file}"
    exit 1
  fi

  tmp="$(mktemp)"
  trap 'rm -f "$tmp"' RETURN
  "$extractor" "$file" > "$tmp"

  if [[ ! -s "$tmp" ]]; then
    echo "No snippet block found in ${file}"
    exit 1
  fi

  if ! diff -u "$tmp" "$snippet"; then
    echo "${description} does not match ${snippet}"
    exit 1
  fi

  echo "${description} matches ${snippet}"
}

check_inclusion \
  "${repo_root}/README.md" \
  extract_after_quick_example_heading \
  "README.md Quick example"

check_inclusion \
  "${repo_root}/docs/quickstart.md" \
  extract_first_rust_block \
  "docs/quickstart.md Quick example"
