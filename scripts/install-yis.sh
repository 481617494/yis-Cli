#!/usr/bin/env bash
# Yis Cli installer (macOS / Linux) — downloads from GitHub Releases.
#
# Usage:
#   curl -fsSL https://github.com/481617494/yis-Cli/releases/latest/download/install.sh | bash
#   curl -fsSL .../install.sh | bash -s v0.1.0
#
# Env:
#   YIS_BIN_DIR   install directory (default: ~/.local/bin)
#   YIS_REPO      owner/repo (default: 481617494/yis-Cli)
#   YIS_VERSION   version tag without or with leading v (optional)

set -euo pipefail

REPO="${YIS_REPO:-481617494/yis-Cli}"
BIN_DIR="${YIS_BIN_DIR:-$HOME/.local/bin}"
VERSION="${1:-${YIS_VERSION:-latest}}"

# Normalize version: accept 0.1.0 or v0.1.0
if [[ "$VERSION" != "latest" && "$VERSION" != v* ]]; then
  VERSION="v${VERSION}"
fi

OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"
case "$ARCH" in
  x86_64|amd64) ARCH="x64" ;;
  arm64|aarch64) ARCH="arm64" ;;
  *)
    echo "不支持的 CPU 架构: $ARCH" >&2
    exit 1
    ;;
esac

case "$OS" in
  darwin)
    ASSET="yis-darwin-${ARCH}"
    ;;
  linux)
    # 当前 Release 流水线默认只打 mac/win；若以后补 linux 资产可直接用
    ASSET="yis-linux-${ARCH}"
    ;;
  mingw*|msys*|cygwin*)
    echo "Windows 请使用 PowerShell: irm https://github.com/${REPO}/releases/latest/download/install.ps1 | iex" >&2
    exit 1
    ;;
  *)
    echo "不支持的系统: $OS" >&2
    exit 1
    ;;
esac

if [[ "$VERSION" == "latest" ]]; then
  URL="https://github.com/${REPO}/releases/latest/download/${ASSET}"
else
  URL="https://github.com/${REPO}/releases/download/${VERSION}/${ASSET}"
fi

echo "Yis Cli 安装"
echo "  仓库:   ${REPO}"
echo "  版本:   ${VERSION}"
echo "  资源:   ${ASSET}"
echo "  目录:   ${BIN_DIR}"
echo "  URL:    ${URL}"

mkdir -p "$BIN_DIR"
TMP="$(mktemp "${TMPDIR:-/tmp}/yis.XXXXXX")"
cleanup() { rm -f "$TMP"; }
trap cleanup EXIT

if command -v curl >/dev/null 2>&1; then
  curl -fL --progress-bar -o "$TMP" "$URL"
elif command -v wget >/dev/null 2>&1; then
  wget -q --show-progress -O "$TMP" "$URL"
else
  echo "需要 curl 或 wget" >&2
  exit 1
fi

# Basic sanity: not an HTML error page
if head -c 32 "$TMP" | grep -qi '<!DOCTYPE\|<html'; then
  echo "下载失败：返回了 HTML（资源可能不存在或版本号错误）" >&2
  exit 1
fi

install -m 755 "$TMP" "${BIN_DIR}/yis"

echo
echo "✓ 已安装: ${BIN_DIR}/yis"
if ! command -v yis >/dev/null 2>&1; then
  echo
  echo "请把安装目录加入 PATH，例如："
  echo "  echo 'export PATH=\"${BIN_DIR}:\$PATH\"' >> ~/.zshrc && source ~/.zshrc"
fi
echo
echo "首次使用："
echo "  yis models setup"
echo "  yis"
echo
