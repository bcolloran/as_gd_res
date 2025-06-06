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

echo ""
echo "--- Trimmed Output ---"
cat "$OUTPUT.trimmed"

echo ""
echo "--- Diff with Expected ---"
# Use diff with -w to ignore all whitespace
diff -w -u "$EXPECTED" "$OUTPUT.trimmed" || true

# For cmp, we need to create temporary files with trimmed content
EXPECTED_TRIM=$(mktemp)
OUTPUT_TRIM=$(mktemp)

# Remove leading/trailing whitespace from both files
sed 's/^[[:space:]]*//;s/[[:space:]]*$//' "$EXPECTED" > "$EXPECTED_TRIM"
sed 's/^[[:space:]]*//;s/[[:space:]]*$//' "$OUTPUT.trimmed" > "$OUTPUT_TRIM"

if cmp -s "$EXPECTED_TRIM" "$OUTPUT_TRIM"; then
  echo "Output matches expected."
  rm "$EXPECTED_TRIM" "$OUTPUT_TRIM"
  exit 0
else
  echo "Output differs from expected." >&2
  rm "$EXPECTED_TRIM" "$OUTPUT_TRIM"
  exit 1
fi
