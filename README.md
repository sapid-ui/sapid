<p align="center">
  <img src="examples/web/assets/sapid-logo.svg" width="112" alt="Sapid stingray logo">
</p>

<h1 align="center">Sapid</h1>

<p align="center"><strong>Composable Rust widgets for interfaces built with Glory.</strong></p>

<p align="center">
  <a href="https://sapid.taidge.org/">Showcase</a> ·
  <a href="docs/widgets-roadmap.md">Widget catalog</a> ·
  <a href="docs/widget-guide.md">Builder guide</a> ·
  <a href="docs/dsl.md">XML syntax</a> ·
  <a href="https://github.com/glory-rs/glory">Glory</a>
</p>

---

Sapid adds ready-to-use controls, layouts, navigation, themes and specialized
widgets to [Glory](https://glory.taidge.org/). Compose them with Rust builders or
Glory's optional `view!` syntax, while your application chooses its rendering host.

- **One widget model.** Sapid and Glory use the same `Widget` contract and
  `Cage` / `Bond` / `Vary` reactive state.
- **Explicit state ownership.** Controlled inputs propose changes through typed
  callbacks; local defaults are available when the widget owns its state.
- **Themes you control.** Blue and Green presets support light and dark modes.
  Complete custom palettes, size variants and style overrides stay explicit.
- **Documented capabilities.** Each catalog item records implementation scope,
  tests and host boundaries instead of promising uniform platform support.

> **Sapid 0.7.1 targets Glory 0.7.0.**
> Rust **1.95+** is required. For source development, keep `sapid`, `glory`,
> `apace`, `takia` and `wingy` as sibling repositories. See [development versions](docs/development-versions.md).

## Start building

Add Sapid and Glory to your application:

```toml
[dependencies]
sapid = { version = "=0.7.1" }
glory = { version = "=0.7.0", default-features = false }
```

For sibling-source development, add `path = "../sapid"` and
`path = "../glory/crates/glory"` to the respective dependencies.

Select Glory host features such as `web`, `ssr`, `hydrate` or `apace-shell` in
that application. Sapid leaves rendering hosts, transport, authorization and
external services under application control.

### A form in ordinary Rust

```rust
use glory::prelude::*;
use sapid::{button, checkbox, text_field};

fn preferences() -> impl Widget {
    widget(|scope| {
        let name = scope.cage(String::new());
        let updates = scope.cage(false);

        col().fill((
            text_field()
                .name("Display name")
                .value(name)
                .on_value_change(move |next| name.revise(|value| *value = next)),
            checkbox()
                .label("Email updates")
                .checked(updates)
                .on_checked_change(move |next| updates.revise(|value| *value = next)),
            button().label("Save").on_press(move || {
                println!("Saving preferences for {}", name.get());
            }),
        )).build(scope)
    })
}

fn main() -> Result {
    let app = App::memory();
    app.mount(preferences())?;
    Ok(())
}
```

`App::memory()` uses Glory's reference interpreter. To display the widget, mount
it in your application's browser or native host. Browser hosts install
[structural CSS](styles/sapid.css) and the selected theme stylesheet explicitly.

A supplied `.value(...)` or `.checked(...)` is controlled: the callback proposes
an update and the source decides the displayed value. Omit it to use local state,
initialized with `.default_value(...)` or `.default_checked(...)`. Details live
in the [state and event guide](docs/widget-guide.md#state-and-events).

### Or use the same widgets in XML

Enable Glory's `dsl` feature:

```rust
fn save_button() -> impl glory::Widget {
    glory::view! { <sapid::Button label="Save"/> }
}
```

Imported types (`<Button/>`), concrete factories (`<{sapid::button()}/>`), raw
HTML and ordinary `impl Widget` factories compose without a second widget API.
The [DSL guide](docs/dsl.md) covers children, controlled inputs, keyed lists and
shared browser/SSR/native examples.

## Find your widgets

The inventory covers **242 upstream entries across 137 work items**. Completion
records a finite implementation; it does not imply full Mantine/shadcn API parity
or support on every host.

| Build with | Examples | Guides |
|---|---|---|
| Controls and forms | Buttons, text fields, choices, validation and form state | [Basic controls](docs/basic-controls.md) · [Forms](docs/form-widgets.md) |
| Layout and content | Stack, Grid, Card, typography, tables and trees | [Layout](docs/layout-widgets.md) · [DataTable](docs/data-table.md) · [Tree](docs/tree.md) |
| Navigation and layers | Tabs, menus, dialogs, drawers, sidebars and app shells | [Tabs](docs/tabs.md) · [Layers](docs/widget-guide.md#dialog-drawer-and-popover) · [AppShell](docs/sidebar-app-shell.md) |
| Dates and schedules | Date/time inputs, calendars and resource scheduling | [Dates](docs/date-picker.md) · [Schedules](docs/schedule.md) |
| Data and media | Charts, JSON/code views, files and a finite rich document editor | [Charts](docs/charts.md) · [Files](docs/files.md) · [Editor](docs/rich-editor.md) |

Browse the [showcase](https://sapid.taidge.org/) for widget guides, live Rust
examples, Props and Styles API references, source panels, search, theme controls
and Mantine-style component scenarios.
The [catalog](docs/widgets-roadmap.md), [capability ledger](docs/widget-capabilities.md)
and [API comparison](docs/mantine-api-comparison.md) provide the implementation
and compatibility details.

## Enable only what you need

Default features are empty. Specialized families are opt-in:

| Cargo feature | Scope |
|---|---|
| `json` | Bounded JSON input/viewing and explicit parsing |
| `code` | Code blocks and highlighting |
| `dates` | Civil date/time controls, calendars and resource schedules |
| `charts` | Charts, bounded sampling and portable `libm` geometry |
| `files` | File controls with explicit permissions and budgets |
| `editor` | Finite rich documents and Web editing; native editing is rejected |

## Make it yours

Use `Theme::from_default(&BLUE)?` or `Theme::new(&GREEN, ColorScheme::Dark)?`
with a widget's `.theme(...)` builder. A `ThemeDefinition` owns a complete base
palette and independent light/dark overrides; unsupported modes return an error.
Theme names are 1–64 ASCII lowercase letters, digits or hyphens, starting with a
letter.

Applications install `theme.stylesheet()?` beside `styles/sapid.css` in the outer
host. Widgets do not inject stylesheets or load fonts. `.size(...)`, `.class(...)`,
`.style(...)` and `.unstyled(...)` offer per-widget configuration.

The showcase's **Theme Studio** edits complete palettes, previews saved themes
across guides and exports Rust definitions. See the [theme guide](docs/theme-definitions.md)
for palette rules and evidence, or the [builder guide](docs/widget-guide.md#theme-definitions)
for a custom-theme example.

## Pick a host with confidence

Sapid uses Glory's standard Web, SSR/hydration, streaming and native Widget hosts.
Support is tracked per widget and operation:

- **Browser:** DOM interaction, hydration and Chromium accessibility-tree checks
  have dedicated acceptance suites.
- **SSR and streams:** widgets share Glory's HTML mapping and final-state adoption
  contract; interactive behavior begins in the corresponding browser host.
- **Native:** engine, raster and AccessKit checks cover documented subsets.
  Unsupported facets return typed errors; the [capability ledger](docs/widget-capabilities.md)
  records tested, rejected and pending behavior.

These checks do not establish OS IME, real-device or screen-reader speech
acceptance. The [verification record](docs/widgets-verification.md) separates
those boundaries from catalog completion and historical machine-local evidence.

## Develop and verify

From a Windows checkout with the sibling sources available:

```powershell
# Library checks
./scripts/check.ps1

# Public showcase and browser acceptance at http://127.0.0.1:8795/
./scripts/check-showcase.ps1 -Serve -Browser
```

See [development checks](docs/development-checks.md) for formatting, optional
Web/native gates and versioned build tools. Verification artifacts use the owning workspace target directory. Cargo home and
compiler temporaries inherit the caller environment. Library and example lockfiles are
prepared locally, ignored by Git and retained for `--locked` checks.

The [inventory](docs/widget-inventory.json) and generated catalog are validated by
`python scripts/widget-plan.py --check`. The older Web galleries on port 8794
remain acceptance fixtures; the Rust showcase on port 8795 is the public site.

CI selects checks from the [complete change scope](docs/ci-scopes.md). Markdown-only
changes run documentation checks. Showcase styles and pages run website checks;
library changes retain the complete Rust/native/widget checks. Pushes to `main`
deploy the [accepted showcase artifact](deploy/README.md) after all checks required
for that change pass. Full page/palette sweeps run daily and for releases.
Its [pinned dependencies](deploy/dependencies.json) describe the reproducible
production source set, separately from ordinary sibling-source development.

## Explore further

- [Widget and theme guide](docs/widget-guide.md) — builder APIs, state, fields, composites and layers.
- [DSL guide](docs/dsl.md) — one Widget contract across Rust and XML syntax.
- [Capability ledger](docs/widget-capabilities.md) — per-widget host support and evidence.
- [API surface](docs/mantine-api-surface.md) — comparison of properties and local entry points.
- [WASM size baseline](docs/wasm-size.md) — reproducible builds and actual-widget comparisons.
- [Release instructions](RELEASING.md) — dependency checks, packaging and evidence requirements.

## License

[Apache-2.0](LICENSE). See [NOTICE](NOTICE) and
[licensing details](docs/licensing.md) for retained third-party notices.
