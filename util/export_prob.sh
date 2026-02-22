#!/usr/bin/env bash
set -e

SRC_FILE="$1"

if [[ -z "$SRC_FILE" ]]; then
  echo "Usage: ./export_prob.sh ./src/problems/_123_two_sum.rs"
  exit 1
fi

if [[ ! -f "$SRC_FILE" ]]; then
  echo "File not found: $SRC_FILE"
  exit 1
fi

DEST_DIR=".leetcode"
mkdir -p "$DEST_DIR"

# ---- Parse filename ----
BASENAME=$(basename "$SRC_FILE")        # _868_binary_gap.rs
NAME_NO_EXT="${BASENAME%.rs}"           # _868_binary_gap

# validate filename format
if [[ ! "$NAME_NO_EXT" =~ ^_[0-9]+_.+ ]]; then
  echo "Expected filename like _123_two_sum.rs"
  exit 1
fi

NAME_NO_UNDERSCORE="${NAME_NO_EXT#_}"   # 868_binary_gap

ID="${NAME_NO_UNDERSCORE%%_*}"          # 868
SLUG_PART="${NAME_NO_UNDERSCORE#*_}"    # binary_gap
SLUG="${SLUG_PART//_/-}"                # binary-gap

NEW_FILENAME="${ID}.${SLUG}.rs"
DEST_PATH="$DEST_DIR/$NEW_FILENAME"

# ---- Copy file ----
cp "$SRC_FILE" "$DEST_PATH"

# ---- Trim marker and everything after marker ----
MARKER_TEXT="Code below here is only for local use"

if grep -q "$MARKER_TEXT" "$DEST_PATH"; then
  awk -v marker="$MARKER_TEXT" '
    $0 ~ marker { exit }   # stop BEFORE printing marker
    { print }
  ' "$DEST_PATH" > "$DEST_PATH.tmp" && mv "$DEST_PATH.tmp" "$DEST_PATH"

  echo "Removed marker and everything after it"
else
  echo "Marker not found — file copied without trimming"
fi

echo "Exported to $DEST_PATH"
echo "Done."
