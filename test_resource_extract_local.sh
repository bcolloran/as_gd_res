#!/bin/bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$REPO_ROOT/resource_test_godot_project"
EXPECTED="$PROJECT_DIR/rust_print_output_expected.txt"
LATEST="$PROJECT_DIR/rust_print_output_latest.tmp.txt"
DIFF_FILE="$PROJECT_DIR/rust_print_output_diff.tmp.txt"

# Clean up tmp files from previous runs
rm -f "$LATEST" "$DIFF_FILE"

if [ -z "${GODOT_BIN:-}" ]; then
  echo "GODOT_BIN not set" >&2
  exit 1
fi

# Preload plugin script classes by running the editor once
"$GODOT_BIN" --headless --editor --path "$PROJECT_DIR" --quit

OUTPUT=$(mktemp)
"$GODOT_BIN" --headless --path "$PROJECT_DIR" test_scene.tscn > "$OUTPUT"
awk '/--- Resource Extract Test ---/{flag=1;next} flag' "$OUTPUT" > "$LATEST"
# remove trailing newline to match expected file
truncate -s -1 "$LATEST"
rm -f "$OUTPUT"

echo ""
echo "--- Trimmed Output ---"
cat "$LATEST"

echo ""
echo "--- Diff with Expected ---"
# Use diff with -w to ignore all whitespace; save to diff file
diff -w -u "$EXPECTED" "$LATEST" > "$DIFF_FILE" 2>&1 || true
cat "$DIFF_FILE"

# For cmp, create temporary files with trimmed whitespace
EXPECTED_TRIM=$(mktemp)
OUTPUT_TRIM=$(mktemp)
sed 's/^[[:space:]]*//;s/[[:space:]]*$//' "$EXPECTED" > "$EXPECTED_TRIM"
sed 's/^[[:space:]]*//;s/[[:space:]]*$//' "$LATEST" > "$OUTPUT_TRIM"

if cmp -s "$EXPECTED_TRIM" "$OUTPUT_TRIM"; then
  echo "Output matches expected."
  rm -f "$EXPECTED_TRIM" "$OUTPUT_TRIM"
  exit 0
else
  echo "Output differs from expected." >&2
  rm -f "$EXPECTED_TRIM" "$OUTPUT_TRIM"
  exit 1
fi
