param(
    [switch]$Browser,
    [switch]$Native,
    [string]$Bindgen = 'wasm-bindgen',
    [string]$BuildRoot = ''
)
. "$PSScriptRoot/local-build-paths.ps1"

$ErrorActionPreference = 'Stop'
if (-not $BuildRoot) { $BuildRoot = Join-Path (Get-NormalCargoTarget) 'verification/sapid' }
$taskBuild = [IO.Path]::GetFullPath($BuildRoot)
$saved = @{}
foreach ($key in @('SAPID_DSL_SITE')) {
    $saved[$key] = [Environment]::GetEnvironmentVariable($key, 'Process')
}
function Invoke-Cargo {
    param([string[]]$Arguments)
    $manifestIndex = [Array]::IndexOf($Arguments, '--manifest-path')
    $directory = Split-Path -Parent $PSScriptRoot
    if ($manifestIndex -ge 0) {
        $Arguments[$manifestIndex + 1] = [IO.Path]::GetFullPath((Join-Path $directory $Arguments[$manifestIndex + 1]))
        $directory = Split-Path -Parent $Arguments[$manifestIndex + 1]
    }
    Push-Location $directory
    try {
        & cargo @Arguments
        if ($LASTEXITCODE -ne 0) { throw "cargo $Arguments failed" }
    } finally { Pop-Location }
}
function Confirm-Target([string]$manifest) {
    $target = Get-NormalCargoTarget -ManifestPath (Join-Path (Split-Path -Parent $PSScriptRoot) $manifest)
    Write-Output "Verified target: $target"
}
function Set-DslBuildPaths([string]$Root, [switch]$Native) {
    $absolute = [IO.Path]::GetFullPath($Root)

    $target = Get-NormalCargoTarget
    $env:SAPID_DSL_SITE = Join-Path $absolute 'dsl'
    New-Item -ItemType Directory -Force -Path $target,$env:SAPID_DSL_SITE,$env:TEMP | Out-Null
}
Push-Location (Split-Path -Parent $PSScriptRoot)
try {
    Set-DslBuildPaths -Root $taskBuild

    Confirm-Target 'Cargo.toml'
    & python "$PSScriptRoot/library_lock.py" --manifest-path Cargo.toml
    if ($LASTEXITCODE -ne 0) { throw 'Library lock preparation failed' }
    Invoke-Cargo -Arguments @('test','--locked','--test','dsl')
    Invoke-Cargo -Arguments @('clippy','--locked','--example','dsl','--','-D','warnings')
    if ($Browser) {
        $manifest = 'examples/web/Cargo.toml'
        Confirm-Target $manifest
        $webTarget = Get-NormalCargoTarget -ManifestPath (Join-Path (Split-Path -Parent $PSScriptRoot) $manifest)
        Invoke-Cargo -Arguments @('clippy','--locked','--manifest-path',$manifest,'--lib','--target','wasm32-unknown-unknown','--','-D','warnings')
        Invoke-Cargo -Arguments @('build','--locked','--manifest-path',$manifest,'--lib','--target','wasm32-unknown-unknown')
        New-Item -ItemType Directory -Force -Path $env:SAPID_DSL_SITE | Out-Null
        if ((& $Bindgen --version).Trim() -ne 'wasm-bindgen 0.2.130') { throw 'Expected wasm-bindgen 0.2.130' }
        & $Bindgen "$webTarget/wasm32-unknown-unknown/debug/sapid_web_example.wasm" --target web --out-dir $env:SAPID_DSL_SITE
        if ($LASTEXITCODE -ne 0) { throw 'wasm-bindgen failed' }
        Copy-Item -LiteralPath styles/sapid.css -Destination "$env:SAPID_DSL_SITE/sapid.css"
        Invoke-Cargo -Arguments @('run','--locked','--manifest-path',$manifest,'--bin','dsl-ssr','--',$env:SAPID_DSL_SITE)
        node examples/web/dsl-acceptance.cjs
        if ($LASTEXITCODE -ne 0) { throw 'DSL browser acceptance failed' }
    }
    if ($Native) {
        Set-DslBuildPaths -Root $taskBuild -Native

        Confirm-Target 'examples/native/Cargo.toml'
        Invoke-Cargo -Arguments @('test','--locked','--manifest-path','examples/native/Cargo.toml','--test','mixed_dsl','--','--test-threads=1')
    }
} finally {
    Pop-Location
    foreach ($key in $saved.Keys) { [Environment]::SetEnvironmentVariable($key, $saved[$key], 'Process') }
}
