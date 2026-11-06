param()
. "$PSScriptRoot/local-build-paths.ps1"

$ErrorActionPreference='Stop'
$projectRoot=Split-Path -Parent $PSScriptRoot
$exportRoot=if($env:SAPID_EXPORT_ROOT){$env:SAPID_EXPORT_ROOT}else{"$PSScriptRoot/../target/verification/sapid/export-check"}
$sourcePath=if($env:SAPID_STUDIO_EXPORT){$env:SAPID_STUDIO_EXPORT}else{"$PSScriptRoot/../target/verification/sapid/studio-export.rs"}
if (-not (Test-Path -LiteralPath $sourcePath)) { throw 'Run theme-studio-browser.cjs to copy a real export first.' }
$saved=@{}
$names=@('CARGO_HOME','CARGO_TARGET_DIR','CARGO_PROFILE_DEV_DEBUG','CARGO_INCREMENTAL','TEMP','TMP','CARGO_BUILD_JOBS')
foreach($name in $names){$saved[$name]=[Environment]::GetEnvironmentVariable($name,'Process')}
try {

    New-Item -ItemType Directory -Force -Path "$exportRoot/src" | Out-Null
    $sapidPath=$projectRoot.Replace('\','/')
    $gloryPath=(Join-Path (Split-Path -Parent $projectRoot) 'glory/crates/glory').Replace('\','/')
    @"
[package]
name="sapid-studio-export-check"
version="0.0.0"
edition="2024"
[workspace]
[dependencies]
sapid={path="$sapidPath"}
glory={path="$gloryPath",default-features=false}
"@ | Set-Content -LiteralPath "$exportRoot/Cargo.toml"
    $source=Get-Content -LiteralPath $sourcePath -Raw
    $checks=@'
fn main() {
    let (scheme,base)=VIOLET_DUSK.default();
    assert_eq!(scheme,sapid::ColorScheme::Light);
    assert_eq!(base.shades[6],Color(124,58,237,255));
    assert_eq!(base.canvas,Color(250,247,255,255));
    let dark=sapid::Theme::new(&VIOLET_DUSK,sapid::ColorScheme::Dark).unwrap();
    assert_eq!(dark.canvas,Color(23,19,34,255));
    assert_eq!(dark.foreground,Color(242,237,250,255));
    assert_eq!(dark.warning,Color(254,220,152,255));
    assert!(dark.stylesheet().unwrap().contains("--sp-canvas:#171322ff;"));
    println!("Copied Rust definition compiles and resolves the browser's exact base and dark override values.");
}
'@
    ($source+"`n"+$checks) | Set-Content -LiteralPath "$exportRoot/src/main.rs"
    Push-Location $exportRoot
    try {
    $meta=& cargo metadata --offline --no-deps --format-version 1 | ConvertFrom-Json
    if($LASTEXITCODE -ne 0){throw 'export metadata failed'}
    if($env:SAPID_EXPORT_TARGET -and [IO.Path]::GetFullPath($meta.target_directory) -ne [IO.Path]::GetFullPath($env:SAPID_EXPORT_TARGET)){throw "Unexpected export target: $($meta.target_directory)"}
    Write-Output "Export target verified: $($meta.target_directory)"
    & cargo run --offline
    if($LASTEXITCODE -ne 0){throw 'copied Rust theme did not compile or resolve correctly'}
    } finally { Pop-Location }
} finally {
    foreach($name in $names){[Environment]::SetEnvironmentVariable($name,$saved[$name],'Process')}
}
