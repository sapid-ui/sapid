# Cloudflare Pages

The public showcase is https://sapid.taidge.org/ and runs as the `sapid-showcase`
Direct Upload Pages project. Its custom domain belongs to the Cloudflare-managed
`taidge.org` zone in the same account as Glory.

Pushes to `main` and pull requests run `.github/workflows/ci.yml`. Widget/native
checks and the reusable showcase build run in parallel, with library and shared
adapter tests executed once in the widget job. The showcase job builds release
WASM and static entry/SSR documents against `dependencies.json` and accepts those
exact files. Production deployment waits for both jobs, downloads the accepted
artifact from the same run, and skips commits superseded on `main`. Pull requests
and release-tag validation never receive Cloudflare credentials or deploy.

Every commit retains all 110 widget browser suites, DSL hydration/native checks,
and all 38 showcase behavior suites. Showcase category checks use blue/light at
both 1280px and 390px; the shared site suite still checks palette/dark switching,
representative navigation, SSR adoption, dirty editors and mobile navigation.
The full cross-product of palettes/schemes and both 181-page navigation/rendering
sweeps run in `.github/workflows/deploy-showcase.yml` daily at 02:43 Asia/Shanghai,
on manual dispatch (default `full`), and during frozen release validation.
Manual showcase dispatch validates and preserves artifacts; it does not deploy.

Browser jobs use two workers. Showcase clipboard suites run exclusively, theme
export compilation waits for the actual copied export, and a failed suite stops
new work while in-flight workers settle. Each report names its coverage mode and
records suite durations, so quick success is distinguishable from full evidence.

Repository Actions secrets:

- `CLOUDFLARE_ACCOUNT_ID`: selected account's 32-character ID.
- `CLOUDFLARE_API_TOKEN`: Account / Cloudflare Pages / Edit, scoped to that account.

The token is provided only to the deployment step. No DNS permission is needed
for routine uploads. Configure `sapid.taidge.org` in the Pages project's Custom
domains screen; keep its CNAME pointing to the project's actual pages.dev host.

Rust 1.98.0, wasm-bindgen 0.2.130, Playwright 1.64.1, and Wrangler 4.152.0 are
pinned. Node 22 or newer is required; CI selects Node 24 before installing tools.
Verification output and pinned tools use `target/verification/sapid` on the
Windows runner (build root `pages-ci`); Cargo home and compiler temporary files
inherit the caller environment. Cargo's actual output
directory is checked before compilation. Individual assets must fit Pages'
25 MiB limit. HTML revalidates; content-addressed asset directories are immutable.

Cargo caches use `cargo metadata --no-deps --format-version 1` in the explicit
library, web, native and Glory workspaces to find their actual target directories.
They retain compiled debug/release/WASM outputs and the caller's Cargo downloads,
with separate compiler/workload keys based on manifests, resolved dependency
locks and build/check configuration. Source-only commits reuse that key, avoiding a full cache upload per commit.
Completed CI jobs prune disposable test/application executables and debug symbols;
build-script executables, proc-macro DLLs and native libraries remain cached.
Cleanup preflights every target under the workspace and rejects directory links.
The cache does not contain project lockfiles or move Cargo home/target/TEMP. Verified bindgen downloads,
npm downloads and Chromium have a separate tools cache; Node modules are always
installed from the committed lockfile before a fresh receipt is recorded.

`build-tools.json` pins the official Windows wasm-bindgen archive SHA-256 and
the extracted executable SHA-256. `scripts/install-build-tools.ps1` verifies both
on every installation and verification, including cache hits. A damaged archive
or executable fails before extraction or execution; it is never silently reused
or replaced. The archive digest is the official release asset's SHA-256; the
executable digest was derived from that verified archive.

`deploy/tools/package-lock.json` fixes the complete Wrangler/Playwright dependency
tree and registry integrity hashes. Installation runs `npm ci --ignore-scripts`
in the external build cache without deployment or registry credentials. It
records every installed module file's SHA-256 in `tools-receipt.json`; CI retains
the receipt digest separately in `SAPID_TOOLS_RECEIPT_SHA256`. Build, acceptance
and deployment verify the cached tools against this trusted digest and the
committed lockfile before using them. Missing tools, modified transitive modules
and replaced receipts fail verification. Preserve the digest returned by the
installer outside the mutable tool cache for local use as well.

The deployment step invokes the verified local Wrangler entrypoint through Node.
It never runs `npx` or installs packages while holding the Cloudflare token, and
it verifies local tools before asset-retention requests or Cloudflare API calls.
Wrangler logs, configuration, cache and working files remain inside the build
root. Production browser acceptance uses `scripts/showcase-acceptance.cjs`,
including the actual theme export compilation. Accepted site artifacts are kept
for seven days and acceptance reports for fourteen days.

For an isolated local release build:

```powershell
$buildRoot = 'target/verification/sapid/pages-local'
$tools = ./scripts/install-build-tools.ps1 -BuildRoot $buildRoot
$env:SAPID_TOOLS_RECEIPT_SHA256 = $tools.ReceiptSha256
$env:SAPID_PLAYWRIGHT = $tools.Playwright
$env:PLAYWRIGHT_BROWSERS_PATH = "$buildRoot/browsers"
node (Join-Path $tools.Playwright 'cli.js') install chromium
./scripts/install-build-tools.ps1 -BuildRoot $buildRoot -VerifyOnly
./scripts/check-showcase.ps1 -Release -BuildRoot $buildRoot -Bindgen $tools.Bindgen
node scripts/showcase-acceptance.cjs --directory "$buildRoot/showcase" --output "$buildRoot/acceptance" --mode full --jobs=2
```

Use `--mode quick` for the per-commit coverage. Omitting `--mode` preserves full
local acceptance; `--jobs` accepts 1–4 and defaults to serial execution.

Publish only that build's `showcase` directory. Query-based guide and theme URLs
are supported by `showcase.html`; representative `ssr-*.html` documents exercise
adoption and do not imply server rendering for all 181 guides. DNS and TLS must
be active before the custom domain is considered ready.
