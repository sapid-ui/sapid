//! The documentation website is a Glory application, including its navigation.
use glory::prelude::*;
use glory::render::{
    Action, AnchorAlign, AnchorPlacement, Current, Event, EventKind, FocusMode, Layer, LayerFocus,
    LayerPlacement, NodeId, Role, SemanticId, Semantics, TextKind,
};
use sapid::{ColorScheme, Theme};

use crate::showcase_catalog::{ENTRIES, Entry, find};
use crate::theme_studio::model::ThemeRecord;

pub fn route(slug: &str, theme: &Theme) -> String {
    format!(
        "showcase.html?widget={slug}&theme={}&scheme={}",
        theme.name,
        theme.scheme.name(),
    )
}

fn label(entry: &Entry) -> &'static str {
    entry.name.split(" / ").next().unwrap_or(entry.name)
}

fn link(label: impl Into<String>, href: impl Into<String>, class: &str) -> sapid::Anchor {
    let href = href.into();
    let internal = href.starts_with("showcase.html?") || href.starts_with('#');
    let anchor = sapid::anchor()
        .unstyled()
        .label(label.into())
        .href(href)
        .class(class.to_owned());
    if internal {
        anchor.on_navigate(crate::showcase_host::request)
    } else {
        anchor
    }
}

fn section_title(id: &str, value: &str) -> impl Widget {
    text()
        .kind(TextKind::Heading(2))
        .class("site-section-title")
        .content(value.to_owned())
        .semantics(Some(Box::new(Semantics {
            id: Some(SemanticId::new(id).expect("static section identifier")),
            ..Default::default()
        })))
}

fn matching(entry: &Entry, query: &str) -> bool {
    let query = query.trim().to_lowercase();
    query.is_empty()
        || format!("{} {}", entry.name, entry.group)
            .to_lowercase()
            .contains(&query)
}

fn sidebar(
    query: Cage<String>,
    page: Cage<crate::showcase_navigation::Page>,
    theme: Theme,
) -> impl Widget {
    widget(move |scope| {
        let groups = [
            "Layout",
            "Inputs",
            "Combobox",
            "Buttons",
            "Navigation",
            "Feedback",
            "Overlays",
            "Data display",
            "Typography",
            "Miscellaneous",
            "Charts",
            "Dates",
            "Extensions",
        ];
        let mut children: Vec<Box<dyn Widget>> = Vec::new();
        // The document drawer contains both site destinations and the catalogue.
        // These links stay out of the desktop sidebar's tab sequence via CSS.
        children.push(Box::new(view().class("site-sidebar-primary").fill((
            link(
                "Theme studio",
                route("theme-studio", &theme),
                "site-mobile-link",
            ),
            link("Overview", route("", &theme), "site-mobile-link"),
            link("Glory", "https://glory.taidge.org/", "site-mobile-link"),
            link(
                "GitHub",
                "https://github.com/taidge/sapid",
                "site-mobile-link",
            ),
        ))));
        children.push(Box::new(
            text().class("site-sidebar-heading").content("Widgets"),
        ));
        for category in groups {
            let shown = scope.bond(move || {
                ENTRIES
                    .iter()
                    .any(|entry| entry.group == category && matching(entry, &query.get()))
            });
            let theme = theme.clone();
            children.push(Box::new(when(shown, move || {
                let theme = theme.clone();
                widget(move |scope| {
                    let items = scope.bond(move || {
                        ENTRIES
                            .iter()
                            .filter(|entry| {
                                entry.group == category && matching(entry, &query.get())
                            })
                            .map(|entry| entry.slug)
                            .collect::<Vec<_>>()
                    });
                    let theme = theme.clone();
                    view()
                        .class("site-nav-group")
                        .fill((
                            text().class("site-nav-title").content(category),
                            each(items).key(|id| *id).children(move |entry| {
                                let id = entry.value;
                                let entry = find(id.get()).expect("catalogued navigation entry");
                                link(label(entry), route(entry.slug, &theme), "site-nav-link")
                                    .class(page.map(move |page| {
                                        if page.route.widget == entry.slug {
                                            "site-nav-link is-active".to_owned()
                                        } else {
                                            "site-nav-link".to_owned()
                                        }
                                    }))
                                    .current(page.map(move |page| {
                                        (page.route.widget == entry.slug).then_some(Current::Page)
                                    }))
                            }),
                        ))
                        .show_in(scope)
                })
            })));
        }
        let none = scope.bond(move || !ENTRIES.iter().any(|entry| matching(entry, &query.get())));
        children.push(Box::new(when(none, || {
            text()
                .class("site-nav-empty")
                .content("No widgets found. Try another search.")
        })));
        view()
            .role(Role::Navigation)
            .name("Widget navigation")
            .class("site-sidebar")
            .fill(children)
            .show_in(scope)
    })
}

fn search_results(query: Cage<String>, opened: Cage<bool>, theme: Theme) -> impl Widget {
    widget(move |scope| {
        let theme = theme.clone();
        let items = scope.bond(move || {
            ENTRIES
                .iter()
                .filter(|entry| matching(entry, &query.get()))
                .take(8)
                .map(|entry| entry.slug)
                .collect::<Vec<_>>()
        });
        let none = scope.bond(move || !ENTRIES.iter().any(|entry| matching(entry, &query.get())));
        view()
            .role(Role::Dialog)
            .focus(FocusMode::Programmatic)
            .class("site-search-results")
            .name("Search results")
            .semantics(Some(Box::new(Semantics {
                id: Some(SemanticId::new("site-search-results").expect("static id")),
                layer: Some(Layer {
                    modal: false,
                    focus: LayerFocus::Preserve,
                    anchor: Some(SemanticId::new("site-search-anchor").expect("static id")),
                    placement: LayerPlacement::Anchor(AnchorPlacement {
                        align: AnchorAlign::End,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            })))
            .on(EventKind::Dismiss, move |_| {
                opened.revise(|value| *value = false)
            })
            .fill((
                each(items).key(|id| *id).children(move |entry| {
                    let id = entry.value;
                    let entry = find(id.get()).expect("catalogued search result");
                    link(
                        format!("{} · {}", label(entry), entry.group),
                        route(entry.slug, &theme),
                        "site-search-result",
                    )
                }),
                when(none, || {
                    text()
                        .class("site-nav-empty")
                        .content("No matching widgets")
                }),
            ))
            .show_in(scope)
    })
}

fn header(
    query: Cage<String>,
    menu: Cage<bool>,
    selected: String,
    theme: Theme,
    catalog: Cage<Vec<ThemeRecord>>,
) -> impl Widget {
    let mut opposite = theme.clone();
    opposite.scheme = if theme.scheme == ColorScheme::Dark {
        ColorScheme::Light
    } else {
        ColorScheme::Dark
    };
    let theme_label = if theme.scheme == ColorScheme::Dark {
        "Switch to light theme"
    } else {
        "Switch to dark theme"
    };
    widget(move |scope| {
        let opened = scope.cage(false);
        let focused = scope.cage(None::<NodeId>);
        let focus_loss = scope.cage(false);
        // FocusWithin reports each physical source, not a boundary transition.
        // Reconcile after the complete host tick so input -> result focus stays
        // inside the search instead of retiring the newly focused result.
        scope.effect(move || {
            if focus_loss.get() && focused.get().is_none() {
                opened.revise(|value| *value = false);
            }
        });
        let has_query = scope.bond(move || opened.get() && !query.get().trim().is_empty());
        let is_empty = scope.bond(move || query.get().is_empty());
        let menu_name = scope.bond(move || {
            if menu.get() {
                "Close navigation".to_owned()
            } else {
                "Open navigation".to_owned()
            }
        });
        let search_theme = theme.clone();
        let toggle_name = theme.name.clone();
        let opposite_mode = opposite.scheme;
        let toggle_supported = scope.bond(move || {
            catalog.with(|list| {
                list.iter()
                    .find(|r| r.id == toggle_name)
                    .is_none_or(|r| r.supports(opposite_mode))
            })
        });
        let toggle_route = route(&selected, &opposite);
        let toggle_unsupported = scope.bond(move || !toggle_supported.get());

        view()
            .role(Role::Banner)
            .class("site-header")
            .fill(
                view().class("site-header-inner").fill((
                    view().class("site-brand-wrap").fill((
                        image()
                            .source(crate::assets::media("sapid-logo.svg"))
                            .decorative()
                            .class("site-logo"),
                        link("Sapid", route("", &theme), "site-brand"),
                    )),
                    view()
                        .role(Role::Navigation)
                        .name("Primary navigation")
                        .class("site-topnav")
                        .fill((
                            link(
                                "Widgets",
                                route("button", &theme),
                                if selected == "theme-studio" {
                                    "site-topnav-link"
                                } else {
                                    "site-topnav-link is-active"
                                },
                            )
                            .current(if selected == "theme-studio" {
                                None
                            } else {
                                Some(Current::Location)
                            }),
                            link(
                                "Theme studio",
                                route("theme-studio", &theme),
                                if selected == "theme-studio" {
                                    "site-topnav-link is-active"
                                } else {
                                    "site-topnav-link"
                                },
                            )
                            .current(if selected == "theme-studio" {
                                Some(Current::Page)
                            } else {
                                None
                            }),
                            link("Glory", "https://glory.taidge.org/", "site-topnav-link"),
                            link(
                                "GitHub",
                                "https://github.com/taidge/sapid",
                                "site-topnav-link",
                            ),
                        )),
                    view().class("site-header-tools").fill((
                        text()
                            .class("site-version")
                            .content(format!("v{}", sapid::VERSION)),
                        view()
                            .class("site-search")
                            .semantics(Some(Box::new(Semantics {
                                id: Some(SemanticId::new("site-search-anchor").expect("static id")),
                                ..Default::default()
                            })))
                            .on(EventKind::FocusWithin, move |event| {
                                if let Event::FocusWithin {
                                    node,
                                    focused: active,
                                } = event
                                {
                                    if active {
                                        focused.revise(|value| *value = Some(node));
                                        focus_loss.revise(|value| *value = false);
                                    } else if focused.get_untracked() == Some(node) {
                                        focused.revise(|value| *value = None);
                                        focus_loss.revise(|value| *value = true);
                                    }
                                }
                            })
                            .fill((
                                when(is_empty, || {
                                    text()
                                        .class("site-search-placeholder")
                                        .content("Search widgets…")
                                }),
                                sapid::text_field()
                                    .theme(theme.clone())
                                    .name("Search widgets")
                                    .value(query)
                                    .on_value_change(move |value| {
                                        query.revise(|current| {
                                            *current = value.chars().take(96).collect()
                                        });
                                        opened.revise(|value| *value = true);
                                    })
                                    .on(EventKind::FocusChanged, move |event| {
                                        if matches!(event, Event::FocusChanged(true)) {
                                            opened.revise(|value| *value = true);
                                        }
                                    }),
                                when(has_query, move || {
                                    search_results(query, opened, search_theme.clone())
                                }),
                            )),
                        palette_choices(&theme, &selected, catalog),
                        when(toggle_supported, move || {
                            link(
                                if opposite_mode == ColorScheme::Light {
                                    "☀"
                                } else {
                                    "☾"
                                },
                                toggle_route.clone(),
                                "site-theme-toggle",
                            )
                            .name(theme_label)
                        }),
                        when(toggle_unsupported, || {
                            sapid::button()
                                .unstyled()
                                .class("site-theme-toggle")
                                .label("◐")
                                .name("Alternate mode is not supported")
                                .enabled(false)
                        }),
                        sapid::button()
                            .unstyled()
                            .class("site-menu-toggle")
                            .label("☰")
                            .name(menu_name)
                            .on_press(move || menu.revise(|open| *open = !*open)),
                    )),
                )),
            )
            .show_in(scope)
    })
}

fn palette_choices(theme: &Theme, selected: &str, catalog: Cage<Vec<ThemeRecord>>) -> impl Widget {
    let theme = theme.clone();
    let selected = selected.to_owned();
    widget(move |scope| {
        // Saved order is independent of selection; never promote the active palette.
        let choices = scope.bond(move || catalog.get());
        let custom_theme = theme.clone();
        let custom_selected = selected.clone();
        view()
            .class("site-palettes")
            .name("Color palette")
            .fill((
                group().fill(
                    ["blue", "green"]
                        .into_iter()
                        .map(|name| {
                            let choice = crate::themes::select(name, theme.scheme);
                            palette_link(name, &choice, &selected, name == theme.name)
                        })
                        .collect::<Vec<_>>(),
                ),
                each(choices).key(|r| r.id.clone()).children(move |entry| {
                    let record = entry.value;
                    let r = record.get();
                    let mode = if r.supports(custom_theme.scheme) {
                        custom_theme.scheme
                    } else {
                        r.default_scheme
                    };
                    let choice = r.theme(mode).expect("validated saved theme");
                    palette_link(
                        &r.title,
                        &choice,
                        &custom_selected,
                        r.id == custom_theme.name,
                    )
                }),
            ))
            .show_in(scope)
    })
}
fn palette_link(name: &str, choice: &Theme, selected: &str, active: bool) -> sapid::Anchor {
    sapid::anchor()
        .unstyled()
        .label("")
        .href(route(selected, choice))
        .class(if active {
            "site-palette is-active"
        } else {
            "site-palette"
        })
        .on_navigate(crate::showcase_host::palette_request)
        .style(Style {
            background: Some(choice.accent),
            ..Style::new()
        })
        .name(format!("Use {name} palette"))
        .current(if active {
            Some(Current::Location)
        } else {
            None
        })
}
fn saved_actions(
    theme: Theme,
    selection: String,
    catalog: Cage<Vec<ThemeRecord>>,
    notice: Cage<String>,
    editor: crate::theme_studio::EditorState,
) -> impl Widget {
    widget(move |scope| {
        let name = theme.name.clone();
        let active = scope.bond(move || {
            catalog.with(|list| {
                list.iter()
                    .filter(|r| r.id == name)
                    .cloned()
                    .collect::<Vec<_>>()
            })
        });
        let theme = theme.clone();
        let selection = selection.clone();
        each(active)
            .key(|r| r.id.clone())
            .children(move |entry| {
                let r = entry.value;
                let record = r.get();
                let id = record.id.clone();
                let fallback = route(&selection, &crate::themes::select("blue", theme.scheme));
                let href = format!(
                    "{}&edit={}&return_to={}",
                    route("theme-studio", &theme),
                    record.id,
                    if find(&selection).is_some() {
                        &selection
                    } else {
                        "button"
                    }
                );
                view().class("site-saved-actions").fill((
                    text().content(format!("{} · Saved locally", record.title)),
                    sapid::copy_button()
                        .theme(theme.clone())
                        .size(sapid::Size::Small)
                        .label("Copy theme code")
                        .source(record.code()),
                    link("Edit theme", href, "site-inline-link"),
                    sapid::button()
                        .theme(theme.clone())
                        .size(sapid::Size::Small)
                        .variant(sapid::ButtonVariant::Danger)
                        .label("Delete theme")
                        .on_press(move || {
                            let result = crate::theme_studio::host::remove(&id).and_then(|()| {
                                editor.forget_saved(&id);
                                crate::showcase_host::navigate(&fallback)
                            });
                            if let Err(error) = result {
                                notice.revise(|value| *value = error);
                            }
                        }),
                ))
            })
            .show_in(scope)
    })
}

fn source(entry: &'static Entry, theme: Theme, full: bool) -> impl Widget {
    widget(move |scope| {
        let expanded = scope.cage(full);
        let class = scope.bond(move || {
            if expanded.get() {
                "site-code is-expanded".to_owned()
            } else {
                "site-code".to_owned()
            }
        });
        let toggle = scope.bond(move || {
            if expanded.get() {
                "Collapse code".to_owned()
            } else {
                "Expand code".to_owned()
            }
        });
        view()
            .class(class)
            .fill((
                view().class("site-code-header").fill((
                    text().content("Example.rs"),
                    sapid::copy_button()
                        .theme(theme.clone())
                        .size(sapid::Size::ExtraSmall)
                        .label("Copy code")
                        .source(entry.code),
                )),
                view().class("site-code-body").fill(
                    sapid::code_highlight()
                        .theme(theme.clone())
                        .source(entry.code)
                        .language(sapid::CodeLanguage::Rust),
                ),
                when(!full, move || {
                    sapid::button()
                        .unstyled()
                        .class("site-code-toggle")
                        .label(toggle)
                        .on_press(move || expanded.revise(|value| *value = !*value))
                }),
            ))
            .show_in(scope)
    })
}

fn support_status(value: &'static str) -> &'static str {
    match value {
        "tested" => "Tested",
        "model-only" => "Model only",
        "rejected" => "Not supported",
        other => other,
    }
}

fn support(entry: &'static Entry, theme: &Theme, id: &str) -> impl Widget {
    view().fill((
        section_title(id, "Host support"),
        view().class("site-support").fill((
            text().class("site-doc-note").content("Choose the host that fits your application. Availability is recorded for each widget and its supported variants."),
            view().class("site-support-row").fill((text().content("Web"), text().content(support_status(entry.web)))),
            view().class("site-support-row").fill((text().content("Native"), text().content(support_status(entry.native)))),
            text().class("site-doc-note").content(entry.native_note),
            link("Browse all widgets →", route("button", theme), "site-inline-link"),
        )),
    ))
}

fn api_props(entry: &'static Entry, theme: Theme) -> impl Widget {
    let api = crate::showcase_api::find(entry.id).expect("catalogued API metadata");
    view()
        .class("site-doc-content")
        .fill((
            section_title("builder-api", &format!("{} properties", label(entry))),
            text().class("site-doc-note").content("Sapid builders and their parameters are the component's properties. These public Rust signatures come from the implementation used by this guide."),
            when(!api.signatures.is_empty(), move || {
                view().class("site-api-tokens").fill(api.signatures.iter().map(|signature| sapid::code().theme(theme.clone()).content(*signature)).collect::<Vec<_>>())
            }),
            when(api.signatures.is_empty(), || {
                text().class("site-doc-note").content("This guide uses composition and shared builder properties rather than widget-specific parameters.")
            }),
        ))
}

fn api_styles(entry: &'static Entry, theme: Theme) -> impl Widget {
    let api = crate::showcase_api::find(entry.id).expect("catalogued API metadata");
    let common = ["class", "style", "styles", "theme", "size", "unstyled"];
    let common = common
        .into_iter()
        .filter(|name| api.builders.contains(name))
        .collect::<Vec<_>>();
    view().class("site-doc-content").fill((
        section_title("styles-api", "Styles API"),
        text().class("site-doc-note").content("Compose typed Style declarations and theme tokens. Explicit styles preserve widget classes and unspecified declarations."),
        view().class("site-api-tokens").fill(common.into_iter().map(|name| sapid::code().theme(theme.clone()).content(name)).collect::<Vec<_>>()),
        text().kind(TextKind::Heading(2)).class("site-section-title").content("Widget classes"),
        text().class("site-doc-note").content("These classes occur in the widget implementation and can be targeted by the application stylesheet."),
        view().class("site-api-tokens").fill(api.classes.iter().map(|name| sapid::code().theme(theme.clone()).content(*name)).collect::<Vec<_>>()),
    ))
}

fn documentation(
    entry: &'static Entry,
    theme: Theme,
    tab: Cage<u8>,
    main_ref: Cage<Option<NodeId>>,
) -> impl Widget {
    widget(move |scope| {
        let mut tabs = Vec::new();
        for (index, label) in [
            (0, "Documentation"),
            (1, "Properties"),
            (2, "Styles API"),
            (3, "Source"),
            (4, "Capabilities"),
        ] {
            let class = scope.bond(move || {
                if tab.get() == index {
                    "site-tab is-active".to_owned()
                } else {
                    "site-tab".to_owned()
                }
            });
            tabs.push(
                sapid::button()
                    .unstyled()
                    .class(class)
                    .label(label)
                    .pressed(scope.bond(move || tab.get() == index))
                    .on_press(move || tab.revise(|value| *value = index)),
            );
        }
        let usage = scope.bond(move || tab.get() == 0);
        let props = scope.bond(move || tab.get() == 1);
        let styles = scope.bond(move || tab.get() == 2);
        let code = scope.bond(move || tab.get() == 3);
        let capabilities = scope.bond(move || tab.get() == 4);
        let demo_theme = theme.clone();
        let props_theme = theme.clone();
        let styles_theme = theme.clone();
        let code_theme = theme.clone();
        let caps_theme = theme.clone();
        view().role(Role::Main).class("site-doc-main")
        .focus(FocusMode::Programmatic)
        .node_ref(main_ref)
        .semantics(Some(Box::new(Semantics {
            id: Some(SemanticId::new("site-main").expect("static id")),
            ..Default::default()
        })))
        .fill((
            text().class("site-breadcrumb").content(format!("Widgets  /  {}", entry.group)),
            text().kind(TextKind::Heading(1)).class("site-title").content(label(entry)),
            text().class("site-lead").content(entry.description),
            view().class("site-doc-meta").fill((
                text().content("Source"), sapid::button().unstyled().class("site-inline-link").label("View source code").on_press(move || tab.revise(|value| *value = 3)),
                text().content("Package"), text().content("sapid"),
                text().content("Built with"), text().content("Glory · handwritten Rust builders"),
            )),
            view().role(Role::Group).class("site-doc-tabs").name("Documentation views").fill(tabs),
            when(usage, move || {
                let details = crate::showcase_examples::detail_sections(entry.slug, demo_theme.clone())
                    .into_iter()
                    .map(|section| {
                        view().class("site-component-detail").fill((
                            section_title(section.id, section.title),
                            text().class("site-doc-note").content(section.note),
                            view().class("site-preview").fill(view().class("site-demo").fill(section.example)),
                            view().class("site-doc-code").fill(
                                sapid::code_highlight().theme(demo_theme.clone()).language(sapid::CodeLanguage::Rust).source(section.source),
                            ),
                        ))
                    })
                    .collect::<Vec<_>>();
                view().class("site-doc-content").fill((
                    section_title("usage", "Usage"),
                    view().class("site-preview").fill(view().class("site-demo").fill(crate::showcase_examples::demo(entry.slug, demo_theme.clone()))),
                    view().semantics(Some(Box::new(Semantics { id: Some(SemanticId::new("example-source").expect("static id")), ..Default::default() }))).fill(source(entry, demo_theme.clone(), false)),
                    view().class("site-component-details").fill(details),
                    section_title("composition", "Compose your interface"),
                    text().class("site-doc-note").content("Use Cage, Bond and Vary for reactive values. Compose these builders with other Sapid widgets and keep application data in your own model."),
                    view().class("site-doc-code").fill(
                        sapid::code_highlight().theme(demo_theme.clone()).language(sapid::CodeLanguage::Rust).source("use glory::prelude::*;\nuse sapid::*;\n\n// Widgets are ordinary Rust builders.\n// Your application chooses Web, SSR or a native host."),
                    ),
                    support(entry, &demo_theme, "support"),
                ))
            }),
            when(props, move || api_props(entry, props_theme.clone())),
            when(styles, move || api_styles(entry, styles_theme.clone())),
            when(code, move || view().class("site-doc-content").fill((section_title("source", "Rust source"), source(entry, code_theme.clone(), true)))),
            when(capabilities, move || view().class("site-doc-content").fill(support(entry, &caps_theme, "capabilities-support"))),
            view().role(Role::ContentInfo).class("site-footer").fill((
                text().content("Sapid · Built with Glory"),
                link("Back to overview →", route("", &theme), "site-inline-link"),
            )),
        )).show_in(scope)
    })
}

pub fn site(theme: Theme, selection: String) -> impl Widget {
    let initial = crate::showcase_navigation::Route::new(selection.clone(), &theme).location();
    site_at(theme, selection, initial)
}
pub fn site_at(theme: Theme, selection: String, initial: glory::routing::Location) -> impl Widget {
    widget(move |scope| {
        let definitions = crate::showcase_navigation::routes();
        let router = glory::routing::Router::new(
            scope,
            definitions.clone(),
            std::rc::Rc::new(glory::routing::MemoryHistory::new(initial.clone())),
        )
        .map_err(|error| glory::render::Error::Backend(error.to_string()))?;
        let page = crate::showcase_navigation::Page::builtin(theme.clone(), selection.clone());
        let state = crate::showcase_navigation::State::new(scope, page, Vec::new(), false);
        crate::showcase_navigation::routed_app(state, definitions, router.handle()).show_in(scope)
    })
}
pub fn site_with_state(
    theme: Theme,
    selection: String,
    catalog: Cage<Vec<ThemeRecord>>,
    return_to: String,
    notice: Cage<String>,
    editor: crate::theme_studio::EditorState,
    page: Cage<crate::showcase_navigation::Page>,
) -> impl Widget {
    widget(move |scope| {
        let query = scope.cage(String::new());
        let menu = scope.cage(false);
        let tab = scope.cage(0u8);
        let main_ref = scope.cage(None);
        let shell_revision = page.with(|page| page.shell_revision);
        let revisions = scope.bond(move || {
            page.with(|page| {
                // A departing shell must not build a destination view while its
                // outer Each is still retiring the previous Owner.
                if page.shell_revision == shell_revision {
                    vec![page.revision]
                } else {
                    Vec::new()
                }
            })
        });
        scope.effect(move || {
            page.with(|page| page.revision);
            tab.revise(|value| *value = 0);
            query.revise(|value| value.clear());
            menu.revise(|value| *value = false);
        });
        let contents = scope.bond(move || tab.get() == 0);
        let root_scheme = if theme.scheme == ColorScheme::Dark {
            "site-dark"
        } else {
            "site-light"
        };
        let theme_class = theme.class();
        let class = scope.bond(move || {
            format!(
                "sapid-site {root_scheme} {}{}",
                theme_class,
                if menu.get() { " menu-open" } else { "" }
            )
        });
        let selected = find(&selection);
        let content: Box<dyn Widget> = if selection == "theme-studio" {
            Box::new(crate::theme_studio::editor(
                theme.clone(),
                editor,
                return_to.clone(),
            ))
        } else if selection.is_empty() {
            Box::new(crate::showcase_home::home(theme.clone()))
        } else if selected.is_some() {
            Box::new(
                view().class("site-doc-layout").fill((
                    sidebar(query, page, theme.clone()),
                    each(revisions).key(|revision| *revision).children({
                        let theme = theme.clone();
                        move |_| {
                            let entry = page
                                .with(|page| find(&page.route.widget))
                                .expect("document route");
                            documentation(entry, theme.clone(), tab, main_ref)
                        }
                    }),
                    each(revisions)
                        .key(|revision| *revision)
                        .children(move |_| {
                            let toc_selection = page.with(|page| page.route.widget.clone());
                            when(contents, move || {
                                view()
                                    .role(Role::Navigation)
                                    .name("On this page")
                                    .class("site-toc")
                                    .fill((
                                        text().class("site-toc-title").content("Table of contents"),
                                        link("Usage", "#usage", "site-toc-link"),
                                        link("Example source", "#example-source", "site-toc-link"),
                                        view().fill(
                                            crate::showcase_examples::detail_navigation(
                                                &toc_selection,
                                            )
                                            .into_iter()
                                            .map(|(label, target)| {
                                                link(label, target, "site-toc-link")
                                            })
                                            .collect::<Vec<_>>(),
                                        ),
                                        link("Composition", "#composition", "site-toc-link"),
                                        link("Host support", "#support", "site-toc-link"),
                                    ))
                            })
                        }),
                )),
            )
        } else {
            Box::new(
                view().role(Role::Main).class("site-not-found").fill((
                    text()
                        .kind(TextKind::Heading(1))
                        .content("Widget not found"),
                    text().content("Explore the widget catalogue to find what you need."),
                    link(
                        "Browse widgets",
                        route("button", &theme),
                        "site-inline-link",
                    ),
                )),
            )
        };
        let weak = scope.downgrade();
        view()
            .class(class)
            .fill((
                when(selected.is_some(), move || {
                    let weak = weak.clone();
                    sapid::anchor()
                        .unstyled()
                        .label("Skip to main content")
                        .href("#site-main")
                        .class("site-skip-link")
                        .on_navigate(move |_| {
                            let Some(scope) = weak.upgrade() else { return };
                            if let Some(main) = main_ref.get_untracked()
                                && let Err(error) = scope.action(Action::Focus(main))
                            {
                                scope.report_error(error);
                            }
                        })
                }),
                each(revisions).key(|revision| *revision).children({
                    let theme = theme.clone();
                    move |_| {
                        header(
                            query,
                            menu,
                            page.with(|page| page.route.widget.clone()),
                            theme.clone(),
                            catalog,
                        )
                    }
                }),
                text().class("site-host-notice").content(notice),
                each(revisions).key(|revision| *revision).children({
                    let theme = theme.clone();
                    move |_| {
                        saved_actions(
                            theme.clone(),
                            page.with(|page| page.route.widget.clone()),
                            catalog,
                            notice,
                            editor,
                        )
                    }
                }),
                if selection.is_empty() || selection == "theme-studio" {
                    let theme = theme.clone();
                    Box::new(when(menu, move || {
                        view()
                            .role(Role::Navigation)
                            .name("Mobile navigation")
                            .class("site-mobile-nav")
                            .fill((
                                link("Widgets", route("button", &theme), "site-nav-link"),
                                link(
                                    "Theme studio",
                                    route("theme-studio", &theme),
                                    "site-nav-link",
                                ),
                                link("Charts", route("area-chart", &theme), "site-nav-link"),
                                link("Glory", "https://glory.taidge.org/", "site-nav-link"),
                            ))
                    })) as Box<dyn Widget>
                } else {
                    Box::new(group()) as Box<dyn Widget>
                },
                content,
            ))
            .show_in(scope)
    })
}
