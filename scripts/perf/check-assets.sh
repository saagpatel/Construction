#!/usr/bin/env bash
set -euo pipefail

# codex-os-managed
max_image_bytes="${ASSET_MAX_BYTES:-350000}"
max_code_bytes="${ASSET_MAX_CODE_BYTES:-500000}"
asset_dirs=()
if [[ -d public ]]; then
  asset_dirs+=(public)
fi
if [[ -d dist/assets ]]; then
  asset_dirs+=(dist/assets)
fi

if (( ${#asset_dirs[@]} == 0 )); then
  echo "No asset directories found (expected public or dist/assets); skipping asset check."
  exit 0
fi

fail=0
while IFS= read -r file; do
  size=$(wc -c < "$file")
  limit="$max_image_bytes"
  if [[ "$file" == *.js || "$file" == *.css ]]; then
    limit="$max_code_bytes"
  fi

  if (( size > limit )); then
    echo "Asset too large (>${limit} bytes): $file"
    fail=1
  fi
done < <(find "${asset_dirs[@]}" -type f \( -name "*.png" -o -name "*.jpg" -o -name "*.jpeg" -o -name "*.webp" -o -name "*.avif" -o -name "*.svg" -o -name "*.js" -o -name "*.css" \))

exit $fail
