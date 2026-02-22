#!/usr/bin/env bash
set -e

# ---- Argument ----
SRC_FILE="$1"

if [[ -z "$SRC_FILE" ]]; then
  echo "Usage: ./util/move_prov.sh src/file.rs"
  exit 1
fi

if [[ ! -f "$SRC_FILE" ]]; then
  echo "File not found: $SRC_FILE"
  exit 1
fi

# ---- Paths ----
PROBLEMS_DIR="./src/problems"
LIB_FILE="./src/lib.rs"
MOD_FILE="$PROBLEMS_DIR/mod.rs"

# ---- Ensure problems directory exists ----
mkdir -p "$PROBLEMS_DIR"

# ---- Move file ----
FILENAME=$(basename "$SRC_FILE")
mv "$SRC_FILE" "$PROBLEMS_DIR/$FILENAME"
echo "Moved $SRC_FILE → $PROBLEMS_DIR/$FILENAME"

# ---- Copy lib.rs contents into problems/mod.rs and truncate if present ----
if [[ -f "$LIB_FILE" ]]; then
  cat "$LIB_FILE" >> "$MOD_FILE"
  echo "Appended contents of $LIB_FILE → $MOD_FILE"
  : > "$LIB_FILE"
  echo "Truncated $LIB_FILE"
else
  echo "Warning: $LIB_FILE not found; skipping truncate"
fi

echo "Done."
