# Yis Cli installer for Windows (PowerShell)
#
# Usage:
#   irm https://github.com/481617494/yis-Cli/releases/latest/download/install.ps1 | iex
#   $env:YIS_VERSION="v0.1.0"; irm .../install.ps1 | iex
#
# Env:
#   YIS_BIN_DIR  install dir (default: %USERPROFILE%\.local\bin)
#   YIS_REPO     owner/repo (default: 481617494/yis-Cli)
#   YIS_VERSION  tag e.g. v0.1.0 or latest

param(
    [Parameter(Position = 0)]
    [string]$Version
)

$ErrorActionPreference = 'Stop'
[Net.ServicePointManager]::SecurityProtocol = [Net.ServicePointManager]::SecurityProtocol -bor [Net.SecurityProtocolType]::Tls12
$ProgressPreference = 'SilentlyContinue'

if (-not $Version -and $env:YIS_VERSION) {
    $Version = $env:YIS_VERSION
}
if (-not $Version) {
    $Version = 'latest'
}

$Repo = if ($env:YIS_REPO) { $env:YIS_REPO } else { '481617494/yis-Cli' }
$BinDir = if ($env:YIS_BIN_DIR) { $env:YIS_BIN_DIR } else { Join-Path $env:USERPROFILE '.local\bin' }

# Normalize version
if ($Version -ne 'latest' -and -not $Version.StartsWith('v')) {
    $Version = "v$Version"
}

$Asset = 'yis-windows-x64.exe'
if ($Version -eq 'latest') {
    $Url = "https://github.com/$Repo/releases/latest/download/$Asset"
} else {
    $Url = "https://github.com/$Repo/releases/download/$Version/$Asset"
}

Write-Host "Yis Cli 安装"
Write-Host "  仓库: $Repo"
Write-Host "  版本: $Version"
Write-Host "  资源: $Asset"
Write-Host "  目录: $BinDir"
Write-Host "  URL:  $Url"

New-Item -ItemType Directory -Force -Path $BinDir | Out-Null
$OutFile = Join-Path $BinDir 'yis.exe'

Write-Host "下载中..."
Invoke-WebRequest -Uri $Url -OutFile $OutFile -UseBasicParsing

# Reject tiny/HTML error pages
$info = Get-Item $OutFile
if ($info.Length -lt 1MB) {
    $head = Get-Content -Path $OutFile -TotalCount 5 -ErrorAction SilentlyContinue
    Remove-Item -Force $OutFile -ErrorAction SilentlyContinue
    throw "下载失败：文件过小（可能 404）。响应: $head"
}

Write-Host ""
Write-Host "✓ 已安装: $OutFile"

# PATH hint
$pathUser = [Environment]::GetEnvironmentVariable('Path', 'User')
if ($pathUser -notlike "*$BinDir*") {
    Write-Host ""
    Write-Host "建议将安装目录加入用户 PATH："
    Write-Host "  [Environment]::SetEnvironmentVariable('Path', `$env:Path + ';$BinDir', 'User')"
    Write-Host "然后重新打开终端，运行: yis"
} else {
    Write-Host "可直接运行: yis"
}

Write-Host ""
Write-Host "首次使用："
Write-Host "  yis models setup"
Write-Host "  yis"
Write-Host ""
