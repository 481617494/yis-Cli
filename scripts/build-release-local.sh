#!/usr/bin/env bash
# 本机打 release 包（当前架构）。用于快速自测，正式发布请用 GitHub Actions。
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
export PATH="${HOME}/.cargo/bin:${PATH}"

PROFILE="${1:-release}"
OUT_DIR="${ROOT}/dist"
mkdir -p "$OUT_DIR"

echo "Building yis (profile=${PROFILE})..."
cargo build -p xai-yis-pager-bin --profile "$PROFILE"

OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"
case "$ARCH" in
  x86_64|amd64) ARCH=x64 ;;
  arm64|aarch64) ARCH=arm64 ;;
esac

if [[ "$PROFILE" == "release" ]]; then
  SRC="${ROOT}/target/release/yis"
elif [[ "$PROFILE" == "release-dist" ]]; then
  SRC="${ROOT}/target/release-dist/yis"
else
  SRC="${ROOT}/target/${PROFILE}/yis"
fi

test -f "$SRC"
strip "$SRC" 2>/dev/null || true

ASSET="yis-${OS}-${ARCH}"
if [[ "$OS" == "darwin" ]]; then
  ASSET="yis-darwin-${ARCH}"
fi
cp "$SRC" "${OUT_DIR}/${ASSET}"
ls -lh "${OUT_DIR}/${ASSET}"
echo "OK → ${OUT_DIR}/${ASSET}"
