# QCStatusBar 一键安装脚本
# 用法: irm https://raw.githubusercontent.com/18haventgirl/qwencode_statusbar/master/install.ps1 | iex

$ErrorActionPreference = "Stop"

$repo = "18haventgirl/qwencode_statusbar"
$installDir = "$env:USERPROFILE\.cargo\bin"

Write-Host "🚀 QCStatusBar 安装脚本" -ForegroundColor Cyan
Write-Host ""

# 检测平台
$arch = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "x86" }
if ($arch -ne "x86_64") {
    Write-Host "❌ 仅支持 x86_64 架构" -ForegroundColor Red
    exit 1
}

# 获取最新版本
Write-Host "📡 获取最新版本..."
try {
    $release = Invoke-RestMethod "https://api.github.com/repos/$repo/releases/latest"
    $version = $release.tag_name
} catch {
    Write-Host "❌ 无法获取版本信息，请检查网络" -ForegroundColor Red
    exit 1
}

Write-Host "📦 最新版本: $version"

# 下载
$assetName = "qcstatusbar-x86_64-windows.zip"
$asset = $release.assets | Where-Object { $_.name -eq $assetName }

if (-not $asset) {
    Write-Host "❌ 未找到适用于 Windows 的构建产物" -ForegroundColor Red
    exit 1
}

$downloadUrl = $asset.browser_download_url
$tempZip = "$env:TEMP\qcstatusbar.zip"
$tempExe = "$env:TEMP\qcstatusbar.exe"

Write-Host "⬇️  下载中..."
Invoke-WebRequest -Uri $downloadUrl -OutFile $tempZip -UseBasicParsing

# 解压
Write-Host "📂 解压中..."
Expand-Archive -Path $tempZip -DestinationPath $env:TEMP -Force

# 安装
if (-not (Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir -Force | Out-Null
}

Copy-Item -Path "$env:TEMP\qcstatusbar.exe" -Destination "$installDir\qcstatusbar.exe" -Force

# 清理
Remove-Item -Path $tempZip -Force -ErrorAction SilentlyContinue
Remove-Item -Path "$env:TEMP\qcstatusbar.exe" -Force -ErrorAction SilentlyContinue

# 检查 PATH
$pathParts = [Environment]::GetEnvironmentVariable("Path", "User") -split ";"
if ($pathParts -notcontains $installDir) {
    Write-Host "⚠️  $installDir 不在 PATH 中，正在添加..."
    $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
    [Environment]::SetEnvironmentVariable("Path", "$currentPath;$installDir", "User")
    $env:Path = "$env:Path;$installDir"
    Write-Host "✅ 已添加到 PATH（重启终端生效）"
}

# 验证
Write-Host ""
Write-Host "✅ 安装完成！" -ForegroundColor Green
& "$installDir\qcstatusbar.exe" --version

Write-Host ""
Write-Host "📋 下一步:" -ForegroundColor Yellow
Write-Host "  1. 在 ~/.qwen/settings.json 中添加:"
Write-Host '     "ui": { "statusLine": { "type": "command", "command": "qcstatusbar" } }'
Write-Host "  2. 重启 Qwen Code"
Write-Host ""
Write-Host "💡 自定义短语: 编辑 ~/.qwen/qcstatusbar/config.toml 然后运行 qcstatusbar --sync-phrases"
