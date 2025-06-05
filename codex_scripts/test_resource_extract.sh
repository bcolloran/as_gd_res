#!/bin/bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EXPECTED="$REPO_ROOT/resource_test_godot_project/expected_rust_print_output.txt"
OUTPUT=$(mktemp)

if [ -z "${GODOT_BIN:-}" ]; then
  echo "GODOT_BIN not set" >&2
  exit 1
fi

# Preload plugin script classes by running the editor once
xvfb-run "$GODOT_BIN" --headless --editor --path "$REPO_ROOT/resource_test_godot_project" --quit

xvfb-run "$GODOT_BIN" --headless --path "$REPO_ROOT/resource_test_godot_project" test_scene.tscn > "$OUTPUT"
awk '/--- Resource Extract Test ---/{flag=1;next} flag' "$OUTPUT" > "$OUTPUT.trimmed"
# remove trailing newline to match expected file
truncate -s -1 "$OUTPUT.trimmed"
diff -u "$EXPECTED" "$OUTPUT.trimmed"
