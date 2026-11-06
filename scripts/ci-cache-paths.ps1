# Discover cache locations from each maintained workspace and the caller's Cargo
# configuration. Never redirect Cargo home, target, or temporary directories.
param(
    [string]$OutputFile = $env:GITHUB_OUTPUT,
    [string]$ProjectRoot = (Split-Path -Parent $PSScriptRoot)
)
$ErrorActionPreference = 'Stop'
$targets = foreach ($manifest in @('Cargo.toml', 'examples/web/Cargo.toml', 'examples/native/Cargo.toml', '../glory/Cargo.toml')) {
    Push-Location (Split-Path -Parent (Join-Path $projectRoot $manifest))
    try {
        $metadata = & cargo metadata --no-deps --format-version 1 | ConvertFrom-Json
        if ($LASTEXITCODE) { throw "Cache metadata failed: $manifest" }
        [IO.Path]::GetFullPath($metadata.target_directory)
    } finally { Pop-Location }
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
