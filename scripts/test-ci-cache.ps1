param([string]$ProjectRoot = $PWD.Path)
$ErrorActionPreference = 'Stop'
. "$PSScriptRoot/ci-cache-paths.ps1" -ProjectRoot $ProjectRoot -OutputFile '' | Out-Null
Push-Location $ProjectRoot
try { $metadata = & cargo metadata --no-deps --format-version 1 | ConvertFrom-Json }
finally { Pop-Location }
if ($LASTEXITCODE) { throw 'Cache test metadata failed' }
$cacheTestRoot = [IO.Path]::GetFullPath((Join-Path $metadata.target_directory "verification/cache-tests-$([Guid]::NewGuid())"))
$relative = [IO.Path]::GetRelativePath([IO.Path]::GetFullPath($metadata.target_directory), $cacheTestRoot)
if ($relative.StartsWith('..') -or [IO.Path]::IsPathRooted($relative)) { throw 'Invalid cache fixture root' }
$cacheWorkspaceRoot = Join-Path $cacheTestRoot 'workspace'
$cacheTargetRoot = Join-Path $cacheWorkspaceRoot 'target'
function Assert-CacheTest([bool]$Condition, [string]$Message) { if (-not $Condition) { throw $Message } }
function New-CacheTestFile([string]$Relative) {
    $file = Join-Path $cacheTestRoot $Relative
    New-Item -ItemType Directory -Force (Split-Path -Parent $file) | Out-Null
    Set-Content -LiteralPath $file -Value 'disposable cache fixture'
}
try {
    $removed = @('workspace/target/debug/test.exe', 'workspace/target/debug/deps/test.pdb',
        'workspace/target/release/deps/test.exe', 'workspace/target/debug/examples/demo.ilk')
    $retained = @('workspace/source.exe', 'workspace/target/debug/build/native/build-script.exe',
        'workspace/target/debug/deps/proc_macro.dll', 'workspace/target/debug/deps/native.lib',
        'workspace/target/debug/deps/library.rlib', 'workspace/target/wasm32-unknown-unknown/release/app.wasm')
    foreach ($file in ($removed + $retained)) { New-CacheTestFile $file }
    Remove-CiCacheBinaries -Targets @($cacheTargetRoot) -WorkspaceRoot $cacheWorkspaceRoot
    foreach ($file in $removed) { Assert-CacheTest (-not (Test-Path -LiteralPath (Join-Path $cacheTestRoot $file))) "Not pruned: $file" }
    foreach ($file in $retained) { Assert-CacheTest (Test-Path -LiteralPath (Join-Path $cacheTestRoot $file)) "Required cache file removed: $file" }

    New-CacheTestFile 'workspace/target/debug/prefix.exe'
    New-CacheTestFile 'outside/debug/outside.exe'
    $rejected = $false
    try { Remove-CiCacheBinaries -Targets @($cacheTargetRoot, (Join-Path $cacheTestRoot 'outside')) -WorkspaceRoot $cacheWorkspaceRoot }
    catch { $rejected = $_.Exception.Message -match 'outside the workspace' }
    Assert-CacheTest $rejected 'An external target must be rejected'
    Assert-CacheTest (Test-Path -LiteralPath (Join-Path $cacheTargetRoot 'debug/prefix.exe')) 'Preflight rejection must precede every deletion'

    $linkedTarget = Join-Path $cacheWorkspaceRoot 'linked-target'
    New-Item -ItemType Directory -Force (Join-Path $linkedTarget 'debug') | Out-Null
    New-Item -ItemType Junction -Path (Join-Path $linkedTarget 'debug/deps') -Target (Join-Path $cacheTestRoot 'outside/debug') | Out-Null
    $rejected = $false
    try { Remove-CiCacheBinaries -Targets @($linkedTarget) -WorkspaceRoot $cacheWorkspaceRoot }
    catch { $rejected = $_.Exception.Message -match 'directory links' }
    Assert-CacheTest $rejected 'A linked artifact directory must be rejected'
    Assert-CacheTest (Test-Path -LiteralPath (Join-Path $cacheTestRoot 'outside/debug/outside.exe')) 'A junction target must remain untouched'
    Write-Output 'CI cache cleanup boundaries passed'
} finally {
    $junction = Join-Path $cacheWorkspaceRoot 'linked-target/debug/deps'
    if (Test-Path -LiteralPath $junction) { Remove-Item -LiteralPath $junction -Force }
    # The resolved unique fixture root was checked against Cargo's actual target.
    if (Test-Path -LiteralPath $cacheTestRoot) { Remove-Item -LiteralPath $cacheTestRoot -Recurse -Force }
}
