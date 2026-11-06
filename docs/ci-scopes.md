# CI by change scope

The shared [scope policy](../scripts/ci_scope.py) reads the complete Git diff,
including both sides of renames and every commit in a push. Pull requests compare
the merge base with the tested merge commit. The diff also includes every change
since a successful ancestor CI run: a CSS push after an unvalidated Rust push
cannot bypass that Rust change. Missing validation/history, unknown paths,
manual checks and release validation select the full path.

| Changes | Required checks |
| --- | --- |
| Repository Markdown only | Scope regression tests, newly broken local links, generated inventory and capability documentation |
| Public showcase CSS, static media or documentation browser tests | Release web/SSR asset build, asset integrity checks and quick browser behavior acceptance; deploy after success |
| Public showcase Rust examples/pages | The same web checks plus showcase Rust tests, formatting and Clippy |
| Library code, shared widget CSS, manifests/locks, dependency pins, scripts/CI configuration or unknown files | Existing complete widget, SSR, native, DSL and browser checks, showcase checks, MSRV and dependency audit |
| Release refs | Full Rust/native/widget checks and the full page/palette acceptance matrix |

Mixed changes take the union of required checks. Ordinary Markdown does not build
or deploy the website. Compiled documentation examples are showcase Rust changes,
not plain Markdown. Shared `styles/sapid.css` remains a library change; only the
public site's `examples/web/site.css` takes the stylesheet path.

Styles-only runs still compile the web bundle and run the SSR asset generator:
CSS participates in the immutable asset directory and SSR references must agree.
They skip showcase Rust tests and Clippy because their Rust sources are unchanged.
All quick behavior suites, both responsive widths, asset validation and deployment
revision checks remain required. Scheduled acceptance still checks every page and
palette, and scheduled/manual dependency quality runs retain the full checks.

Deployment explicitly permits a skipped widget job only when the successful
scope job classified the diff as non-runtime. A failed classifier, failed required
job or cancelled run cannot deploy. Release acceptance cannot use the styles-only
shortcut or an empty widget provenance result.

Documentation checks reject newly introduced broken repository links and links
whose targets were removed. They preserve historical broken references, skip code
examples and external/machine-local URLs, and make no network requests.
