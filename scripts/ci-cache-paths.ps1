# Discover cache locations from each maintained workspace and the caller's Cargo
# configuration. Never redirect Cargo home, target, or temporary directories.
param(
    [string]$OutputFile = $env:GITHUB_OUTPUT,
    [string]$ProjectRoot = (Split-Path -Parent $PSScriptRoot),
    [switch]$Prune
)
$ErrorActionPreference = 'Stop'

function Remove-CiCacheBinaries {
    param([string[]]$Targets, [string]$WorkspaceRoot)
    $workspace = [IO.Path]::GetFullPath($WorkspaceRoot).TrimEnd([IO.Path]::DirectorySeparatorChar)
    $directories = foreach ($target in ($Targets | Sort-Object -Unique)) {
        $target = [IO.Path]::GetFullPath($target)
        $relative = [IO.Path]::GetRelativePath($workspace, $target)
        if ([IO.Path]::IsPathRooted($relative) -or $relative -eq '..' -or $relative.StartsWith("..$([IO.Path]::DirectorySeparatorChar)")) {
            throw "Cache cleanup target is outside the workspace: $target"
        }
        foreach ($suffix in @('', 'debug', 'release', 'debug/deps', 'release/deps', 'debug/examples', 'release/examples')) {
            $directory = if ($suffix) { Join-Path $target $suffix } else { $target }
            if (Test-Path -LiteralPath $directory) {
                if ((Get-Item -LiteralPath $directory -Force).Attributes -band [IO.FileAttributes]::ReparsePoint) {
                    throw "Cache cleanup refuses directory links: $directory"
                }
                if ($suffix) { $directory }
            }
        }
    }
    # Only direct files in these generated directories: build-script executables,
    # proc-macro DLLs and native libraries remain available to Cargo.
    $count = 0; $bytes = 0L
    foreach ($directory in $directories) {
        foreach ($file in Get-ChildItem -LiteralPath $directory -File -Force) {
            if ($file.Extension -notin @('.exe', '.pdb', '.ilk')) { continue }
            $bytes += $file.Length; $count++
            Remove-Item -LiteralPath $file.FullName -Force
        }
    }
    Write-Host "Cargo cache cleanup: removed $count disposable binaries/symbols ($bytes bytes)."
}

if ($Prune -and $env:GITHUB_ACTIONS -ne 'true') { throw 'Cache pruning is restricted to GitHub Actions' }
$targets = foreach ($manifest in @('Cargo.toml', 'examples/web/Cargo.toml', 'examples/native/Cargo.toml', '../glory/Cargo.toml')) {
    Push-Location (Split-Path -Parent (Join-Path $projectRoot $manifest))
    try {
        $metadata = & cargo metadata --no-deps --format-version 1 | ConvertFrom-Json
        if ($LASTEXITCODE) { throw "Cache metadata failed: $manifest" }
        $lockfile = Join-Path $metadata.workspace_root 'Cargo.lock'
        if (-not $Prune -and -not (Test-Path -LiteralPath $lockfile)) {
            & cargo --config 'resolver.incompatible-rust-versions="fallback"' generate-lockfile
            if ($LASTEXITCODE) { throw "Cache lock preparation failed: $manifest" }
        }
        [IO.Path]::GetFullPath($metadata.target_directory)
    } finally { Pop-Location }
}
if ($Prune) {
    if (-not $env:GITHUB_WORKSPACE) { throw 'Missing GitHub workspace for cache cleanup' }
    Remove-CiCacheBinaries -Targets $targets -WorkspaceRoot $env:GITHUB_WORKSPACE
    return
}
$cargoDirectory = if ($env:CARGO_HOME) { [IO.Path]::GetFullPath($env:CARGO_HOME) } else { Join-Path ([Environment]::GetFolderPath('UserProfile')) '.cargo' }
$paths = @(
    (Join-Path $cargoDirectory 'registry'),
    (Join-Path $cargoDirectory 'git')
    foreach ($target in ($targets | Sort-Object -Unique)) {
        # Verification artifacts and browser installations have separate lifetimes.
        foreach ($subdirectory in @('debug', 'release', 'wasm32-unknown-unknown')) {
            Join-Path $target $subdirectory
        }
    }
)
if ($OutputFile) {
    @('paths<<SAPID_CACHE_PATHS') + $paths + @('SAPID_CACHE_PATHS') | Add-Content -LiteralPath $OutputFile
}
$paths
