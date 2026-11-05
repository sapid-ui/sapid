param(
    [switch]$Serve,
    [switch]$Browser,
    [switch]$Release,
    [string]$BuildRoot = '',
    [string]$Bindgen = 'wasm-bindgen'
)
$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $false
$projectRoot = Split-Path -Parent $PSScriptRoot
Push-Location "$projectRoot/examples/web"
try { $metadata = & cargo metadata --no-deps --format-version 1 | ConvertFrom-Json; if ($LASTEXITCODE -ne 0) { throw 'showcase metadata failed' } }
finally { Pop-Location }
$showcaseTarget = [IO.Path]::GetFullPath($metadata.target_directory)
if (-not $BuildRoot) { $BuildRoot = $showcaseTarget }
$BuildRoot = [IO.Path]::GetFullPath($BuildRoot)
foreach ($directory in @($BuildRoot, $showcaseTarget)) {
    $normalized = $directory.Replace('\','/').ToLowerInvariant()
}
$showcaseBuild = [IO.Path]::GetFullPath((Join-Path $BuildRoot 'showcase'))
if ([IO.Path]::GetRelativePath($BuildRoot, $showcaseBuild) -ne 'showcase') { throw 'Invalid showcase output directory' }
$savedEnvironment = @{}
$environmentNames = @('SAPID_BUILD_ROOT','SAPID_BROWSER_OUTPUT','SAPID_SHOWCASE_BASE','SAPID_PLAYWRIGHT','SAPID_SHOWCASE_MEDIA')
foreach ($name in $environmentNames) { $savedEnvironment[$name] = [Environment]::GetEnvironmentVariable($name, 'Process') }
function Invoke-ShowcaseCargo {
    param([string[]]$Arguments)
    $manifestIndex = [Array]::IndexOf($Arguments, '--manifest-path')
    $directory = $projectRoot
    if ($manifestIndex -ge 0) {
        $Arguments[$manifestIndex + 1] = [IO.Path]::GetFullPath((Join-Path $projectRoot $Arguments[$manifestIndex + 1]))
        $directory = Split-Path -Parent $Arguments[$manifestIndex + 1]
    }
    Push-Location $directory
    try {
        & cargo @Arguments
        if ($LASTEXITCODE -ne 0) { throw "showcase cargo failed: $($Arguments -join ' ')" }
    } finally { Pop-Location }
}
Push-Location $projectRoot
try {
    $env:SAPID_BUILD_ROOT = $BuildRoot
    $env:SAPID_BROWSER_OUTPUT = $BuildRoot
    & python "$PSScriptRoot/library_lock.py" --manifest-path Cargo.toml
    if ($LASTEXITCODE -ne 0) { throw 'Library dependency resolution failed' }
    # A clean output prevents removed assets or obsolete HTML from leaking into a release.
    if (Test-Path -LiteralPath $showcaseBuild) { Remove-Item -LiteralPath $showcaseBuild -Recurse -Force }
    New-Item -ItemType Directory -Force -Path $showcaseBuild | Out-Null
    $mediaDirectory = & python scripts/site-assets.py prepare $showcaseBuild examples/web/assets
    if ($LASTEXITCODE -ne 0) { throw 'Media fingerprint failed' }
    $env:SAPID_SHOWCASE_MEDIA = "/$mediaDirectory"
    Write-Output "Showcase target verified: $($metadata.target_directory)"
    & python scripts/showcase-catalog.py --check
    if ($LASTEXITCODE -ne 0) { throw 'showcase catalog is stale' }
    Invoke-ShowcaseCargo -Arguments @('fmt','--manifest-path','examples/web/Cargo.toml','--','--check')
    Invoke-ShowcaseCargo -Arguments @('run','--locked','--manifest-path','examples/web/Cargo.toml','--bin','theme_audit')
    Invoke-ShowcaseCargo -Arguments @('test','--locked','--manifest-path','examples/web/Cargo.toml','--test','showcase_site')
    Invoke-ShowcaseCargo -Arguments @('test','--locked','--manifest-path','examples/web/Cargo.toml','--test','showcase_routing')
    Invoke-ShowcaseCargo -Arguments @('test','--locked','--manifest-path','examples/web/Cargo.toml','--lib')
    Invoke-ShowcaseCargo -Arguments @('clippy','--locked','--manifest-path','examples/web/Cargo.toml','--all-targets','--','-D','warnings')
    Invoke-ShowcaseCargo -Arguments @('clippy','--locked','--manifest-path','examples/web/Cargo.toml','--lib','--features','public-showcase','--target','wasm32-unknown-unknown','--','-D','warnings')
    if ((& $Bindgen --version).Trim() -ne 'wasm-bindgen 0.2.130') { throw 'showcase requires wasm-bindgen-cli 0.2.130' }
    $profileName = if ($Release) { 'release' } else { 'debug' }
    $buildArguments = @('build','--locked','--manifest-path','examples/web/Cargo.toml','--lib','--features','public-showcase','--target','wasm32-unknown-unknown')
    if ($Release) { $buildArguments += '--release' }
    Invoke-ShowcaseCargo -Arguments $buildArguments
    $bindgenArguments = @("$showcaseTarget/wasm32-unknown-unknown/$profileName/sapid_web_example.wasm", '--target', 'web', '--out-dir', $showcaseBuild)
    if ($Release) { $bindgenArguments += '--remove-name-section' }
    & $Bindgen @bindgenArguments
    if ($LASTEXITCODE -ne 0) { throw 'showcase wasm-bindgen failed' }
    Copy-Item -LiteralPath examples/web/site.css -Destination "$showcaseBuild/site.css"
    Copy-Item -LiteralPath styles/sapid.css -Destination "$showcaseBuild/sapid.css"
    # Keep bindgen snippets and the relative WASM URL in one immutable asset graph.
    $assetDirectory = & python scripts/site-assets.py bundle $showcaseBuild
    if ($LASTEXITCODE -ne 0) { throw 'Asset fingerprint failed' }
    $generateArguments = @('run','--locked','--manifest-path','examples/web/Cargo.toml','--bin','showcase_ssr')
    # This native build tool is not published. Reuse its validated dev build;
    # the browser bundle above still uses the requested release profile.
    Invoke-ShowcaseCargo -Arguments ($generateArguments + @('--',$showcaseBuild,$assetDirectory))
    if (Test-Path -LiteralPath deploy/_headers) { Copy-Item -LiteralPath deploy/_headers -Destination "$showcaseBuild/_headers" }
    & python scripts/site-assets.py validate $showcaseBuild
    if ($LASTEXITCODE -ne 0) { throw 'Asset validation failed' }
    $oversized = @(Get-ChildItem -LiteralPath $showcaseBuild -Recurse -File | Where-Object Length -GT (25 * 1024 * 1024))
    if ($Release -and $oversized.Count) { throw "Cloudflare Pages assets exceed 25 MiB: $($oversized.Name -join ', ')" }
    $buildFiles = Get-ChildItem -LiteralPath $showcaseBuild -File -Recurse | Where-Object Name -ne 'asset-hashes.json'
    $assetHashes = @($buildFiles | Sort-Object FullName | ForEach-Object { @{ path=[IO.Path]::GetRelativePath($showcaseBuild,$_.FullName);sha256=(Get-FileHash -LiteralPath $_.FullName -Algorithm SHA256).Hash;bytes=$_.Length } })
    $assetHashes | ConvertTo-Json -Depth 4 | Set-Content -LiteralPath "$showcaseBuild/asset-hashes.json"
    Write-Output "Showcase built separately at $showcaseBuild (old 8794 fixtures were not touched)"
    if ($Serve) {
        $listener = Get-NetTCPConnection -LocalAddress 127.0.0.1 -LocalPort 8795 -State Listen -ErrorAction SilentlyContinue
        if (-not $listener) {
            $server = Start-Process -FilePath 'python' -ArgumentList @('-m','http.server','8795','--bind','127.0.0.1','--directory',$showcaseBuild) -WindowStyle Hidden -PassThru -RedirectStandardOutput "$BuildRoot/showcase-server.log" -RedirectStandardError "$BuildRoot/showcase-server-errors.log"
            $server.Id | Set-Content -LiteralPath "$BuildRoot/showcase-server.pid"
        }
        $serverReady = $false
        for ($attempt=0; $attempt -lt 30; $attempt++) {
            try {
                $response = Invoke-WebRequest -Uri 'http://127.0.0.1:8795/showcase.html' -TimeoutSec 2
                if ($response.Content -notlike '*Sapid showcase*') { throw 'port 8795 belongs to another service' }
                $serverReady = $true
                break
            } catch { Start-Sleep -Milliseconds 100 }
        }
        if (-not $serverReady) { throw 'showcase server did not become ready on 8795' }
        Write-Output 'Showcase URL: http://127.0.0.1:8795/'
    }
    if ($Browser) {
        & node scripts/showcase-acceptance.cjs --directory $showcaseBuild --output "$BuildRoot/showcase-acceptance"
        if ($LASTEXITCODE -ne 0) { throw 'showcase acceptance matrix failed' }
    }
} finally {
    Pop-Location
    foreach ($name in $environmentNames) { [Environment]::SetEnvironmentVariable($name, $savedEnvironment[$name], 'Process') }
}
