use glory::render::{Event, NodeId, Props};
use glory::ui::App;
use sapid::Theme;
use sapid_web_example::{showcase, showcase_catalog::ENTRIES};

fn nodes(app: &App) -> Vec<NodeId> {
    let mut pending = app.tree().roots().to_vec();
    let mut result = Vec::new();
    while let Some(id) = pending.pop() {
        pending.extend(app.tree().node(id).unwrap().children.iter().copied());
        result.push(id);
    }
    result
}

#[test]
fn catalogue_has_exactly_181_unique_routes_and_real_example_sources() {
    let mut ids = std::collections::BTreeSet::new();
    let mut slugs = std::collections::BTreeSet::new();
    assert_eq!(ENTRIES.len(), 181);
    for entry in ENTRIES {
        assert!(ids.insert(entry.id), "duplicate {}", entry.id);
        assert!(slugs.insert(entry.slug), "duplicate {}", entry.slug);
        assert!(!entry.name.is_empty() && !entry.description.is_empty());
        assert!(!entry.code.trim().is_empty(), "missing source {}", entry.id);
        let route = showcase::route(entry.slug, &Theme::blue_light());
        assert!(route.contains(&format!("widget={}", entry.slug)));
        assert!(!route.contains(&format!("widget={}", entry.id)));
    }
}

#[test]
fn docs_navigation_is_live_semantic_links_and_mobile_menu_is_rust_state() {
    let app = App::memory();
    app.mount(showcase::site(Theme::blue_light(), "button".into()))
        .unwrap();
    let all = nodes(&app);
    let navigation: Vec<_> = all
        .iter()
        .copied()
        .filter(|id| {
            app.tree()
                .node(*id)
                .unwrap()
                .descriptor
                .view()
                .classes
                .iter()
                .any(|c| c == "site-nav-link")
        })
        .collect();
    assert_eq!(navigation.len(), 181);
    assert!(navigation.iter().all(|id| matches!(
        app.tree().node(*id).unwrap().descriptor.view().props,
        Props::Link(_)
    )));
    let toggle = all
        .into_iter()
        .find(|id| {
            app.tree()
                .node(*id)
                .unwrap()
                .descriptor
                .view()
                .name
                .as_deref()
                == Some("Open navigation")
        })
        .unwrap();
    app.dispatch(toggle, Event::Activate).unwrap();
    app.flush().unwrap();
    assert_eq!(
        app.tree()
            .node(toggle)
            .unwrap()
            .descriptor
            .view()
            .name
            .as_deref(),
        Some("Close navigation")
    );
    app.dispatch(toggle, Event::Activate).unwrap();
    app.flush().unwrap();
    assert_eq!(
        app.tree()
            .node(toggle)
            .unwrap()
            .descriptor
            .view()
            .name
            .as_deref(),
        Some("Open navigation")
    );
}

#[test]
fn documentation_tab_switches_directly_to_host_support() {
    let app = App::memory();
    app.mount(showcase::site(Theme::blue_light(), "button".into()))
        .unwrap();
    let capabilities = nodes(&app)
        .into_iter()
        .find(|id| {
            matches!(
                app.tree().node(*id).unwrap().descriptor.view().props,
                Props::Button(button) if button.label == "Capabilities"
            )
        })
        .unwrap();
    app.dispatch(capabilities, Event::Activate).unwrap();
    app.flush().unwrap();
    assert!(nodes(&app).into_iter().any(|id| {
        matches!(
            app.tree().node(id).unwrap().descriptor.view().props,
            Props::Text(text) if text.content == "Host support"
        )
    }));
}

#[test]
fn props_and_styles_tabs_expose_only_real_sapid_rust_api() {
    let app = App::memory();
    app.mount(showcase::site(Theme::blue_light(), "button".into()))
        .unwrap();
    for (label, expected) in [
        ("Properties", "Button properties"),
        ("Styles API", "Widget classes"),
    ] {
        let tab = nodes(&app)
            .into_iter()
            .find(|id| {
                matches!(
                    app.tree().node(*id).unwrap().descriptor.view().props,
                    Props::Button(button) if button.label == label
                )
            })
            .unwrap();
        app.dispatch(tab, Event::Activate).unwrap();
        app.flush().unwrap();
        assert!(nodes(&app).into_iter().any(|id| {
            matches!(
                app.tree().node(id).unwrap().descriptor.view().props,
                Props::Text(text) if text.content == expected
            )
        }));
    }
}

#[test]
fn representative_ssr_documents_render_in_all_themes_without_frames() {
    for theme in [
        Theme::blue_light(),
        Theme::blue_dark(),
        Theme::green_light(),
        Theme::green_dark(),
    ] {
        for id in ["", "button", "text-input", "mini-calendar", "area-chart"] {
            let html = glory::ssr::snapshot(showcase::site(theme.clone(), id.into()))
                .unwrap()
                .html;
            assert!(html.contains("sapid-site"));
            assert!(!html.contains("<iframe"));
            assert!(html.contains("Search widgets"));
            assert!(html.contains(&format!("v{}", sapid::VERSION)));
            assert!(html.contains("v0.7.0"));
            assert!(!html.contains("v0.6.1"));
            if !id.is_empty() {
                assert!(html.contains("site-code"));
                assert!(html.contains(">Tested<") || html.contains(">Model only<"));
                assert!(!html.contains(">tested<"));
            }
        }
    }
}

#[test]
fn unknown_route_is_a_friendly_rust_rendered_page() {
    let html = glory::ssr::snapshot(showcase::site(Theme::blue_light(), "missing".into()))
        .unwrap()
        .html;
    assert!(html.contains("Widget not found"));
    assert!(html.contains("showcase.html?widget="));
}

#[test]
fn every_displayed_source_builds_its_preview_and_fits_the_highlighter() {
    for entry in ENTRIES {
        assert!(entry.code.contains("pub fn demo("), "{}", entry.id);
        assert!(
            !entry.code.to_ascii_lowercase().contains("mantine"),
            "{} exposes a comparison-source brand in user-facing example code",
            entry.id
        );
        assert!(
            !entry.code.contains("crate::"),
            "{} must be self-contained",
            entry.id
        );
        sapid::tokenize_code(entry.code, sapid::CodeLanguage::Rust)
            .unwrap_or_else(|error| panic!("{} source: {error}", entry.id));
        let app = App::memory();
        app.mount((entry.build)(Theme::blue_light()))
            .unwrap_or_else(|error| panic!("{} preview: {error}", entry.id));
        assert!(!app.tree().is_empty(), "{} preview", entry.id);
        if entry.code.contains("glory::ui::button()") {
            for id in nodes(&app) {
                let tree = app.tree();
                let descriptor = tree.node(id).unwrap().descriptor.view();
                if matches!(descriptor.props, Props::Button(_)) {
                    assert!(
                        descriptor.style.color.is_some() && descriptor.style.font_family.is_some(),
                        "{} has an unthemed adapted button",
                        entry.id
                    );
                }
            }
        }
        app.clear().unwrap();
        assert_eq!(app.stats().nodes, 0, "{} cleanup", entry.id);
        assert_eq!(app.runtime_stats().tasks, 0, "{} tasks", entry.id);
    }
}

#[test]
fn focused_guides_render_their_own_widget_content() {
    let cases = [
        ("P1-01", "Email address"),
        ("P1-03", "Personal information"),
        ("P1-04", "Tell us a little about yourself."),
        ("P1-05", "Use at least 12 characters"),
        ("P2-02", "Notifications"),
        ("P2-03", "Delivery"),
        ("P2-05", "Billing interval"),
        ("P2-06", "Languages"),
        ("P3-02", "Centered in both directions"),
        ("P3-03", "Project overview occupies two columns."),
        ("P3-07", "Review the project brief"),
        ("P3-08", "Good design gives people"),
        ("P3-09", "count.get()"),
        ("P3-09-highlight", "important"),
        ("P3-09-mark", "important details"),
        ("P3-11", "A lakeside weekend"),
        ("P3-12", "or continue with email"),
        ("P3-16", "Open search"),
        ("P3-18", "Project facts"),
        ("P3-19", "Release progress"),
        ("P3-20", "$12,500.50"),
        ("P5-03", "Save draft"),
        ("P5-04", "About Ada"),
        ("P5-05", "Your storage is nearly full"),
        ("P5-10", "sapid-skeleton"),
        ("P5-12", "Background sync"),
        ("P7-01", "Quantity"),
        ("P7-02", "Volume"),
        ("P7-03", "Price range"),
        ("P7-05", "Product rating"),
        ("P10-01", "Choose files"),
        ("P10-03", "Copy invite link"),
        ("P5-08", "Project upload"),
        ("P6-01", "Semantic widgets"),
        ("P6-02", "Example pages"),
        ("P6-10", "Archive selected"),
        ("P6-10-toolbar", "Document formatting"),
        ("P11-02", "Revenue"),
        ("P11-02-barchart", "Revenue"),
        ("P11-02-linechart", "Revenue"),
        ("P11-02-compositechart", "Revenue"),
        ("P11-03", "72%"),
        ("P11-03-piechart", "Product A"),
        ("P11-04", "OHLC per trading day"),
        ("P11-04-scatterchart", "Coordinates, series and units"),
        ("P11-05", "Visitors"),
        ("P11-05-radarchart", "Reliability"),
        ("P11-06", "Project contributions"),
        ("P11-06-matrixchart", "Product A"),
        ("P11-06-wafflechart", "Product A"),
        ("P11-07", "Returns"),
        ("P11-07-bulletchart", "Capacity"),
        ("P11-08", "Regions"),
        ("P11-08-sankeychart", "Source"),
    ];
    for (id, expected) in cases {
        let entry = sapid_web_example::showcase_catalog::find_by_id(id).unwrap();
        let html = glory::ssr::snapshot((entry.build)(Theme::blue_light()))
            .unwrap_or_else(|error| panic!("{id}: {error}"))
            .html;
        assert!(html.contains(expected), "{id} must render {expected:?}");
        if id == "P1-05" {
            assert_eq!(html.matches("type=\"password\"").count(), 1);
            assert_eq!(html.matches("sapid-password-toggle").count(), 1);
            assert!(html.contains("Show password") && html.contains("<svg"));
            assert!(!html.contains("site-button-playground"));
        }
    }
}

#[test]
fn chart_guides_do_not_mount_the_shared_acceptance_workbench() {
    let charts = ENTRIES.iter().filter(|entry| entry.group == "Charts");
    assert_eq!(charts.clone().count(), 22);
    for entry in charts {
        let id = entry.id;
        let html = glory::ssr::snapshot((entry.build)(Theme::blue_light()))
            .unwrap()
            .html;
        assert!(!html.contains("Accept chart proposals"), "{id}");
        assert!(!html.contains("Accept plot proposals"), "{id}");
        assert!(!html.contains("Toggle chart owner"), "{id}");
        assert!(!html.contains("Toggle plot owner"), "{id}");
        assert!(!html.contains("chart-mode-Line"), "{id}");
        assert!(!html.contains("plot-secondary"), "{id}");
        if entry.name == "Heatmap" {
            assert_eq!(html.matches("sapid-heatmap-grid").count(), 1);
            assert_eq!(html.matches("role=\"gridcell\"").count(), 91);
            assert_eq!(html.matches("site-chart-card").count(), 0);
        } else {
            assert_eq!(html.matches("site-chart-card").count(), 0, "{id}");
            assert_eq!(
                html.matches("sapid-chart-plot").count()
                    + html.matches("sapid-plot-drawing").count(),
                1,
                "{id}"
            );
            if entry.name == "Sparkline" {
                assert!(html.contains("Weekly values: 4, 7, 5, 9, 8, 12, 15"));
                assert!(!html.contains("sapid-plot-controls"));
            } else {
                assert!(html.contains("<table"), "{id}");
                assert!(html.contains("Next data point"), "{id}");
            }
        }
    }
}

#[test]
fn playground_helpers_are_embedded_only_when_the_preview_uses_them() {
    for (id, expected) in [
        ("P0-01", false),
        ("P0-02", false),
        ("P0-03", false),
        ("P0-07", false),
        ("P2-08", false),
        ("P3-07", false),
        ("P7-05", false),
    ] {
        let entry = sapid_web_example::showcase_catalog::find_by_id(id).unwrap();
        assert_eq!(entry.code.contains("mod controls {"), expected, "{id}");
    }
}

#[test]
fn number_input_size_examples_apply_to_the_actual_editors() {
    let sections =
        sapid_web_example::showcase_examples::detail_sections("number-input", Theme::blue_light());
    let sizes = sections
        .into_iter()
        .find(|s| s.id == "number-sizes")
        .unwrap();
    let app = App::memory();
    app.mount(sizes.example).unwrap();
    let inputs = nodes(&app)
        .into_iter()
        .filter(|id| {
            matches!(
                app.tree().node(*id).unwrap().descriptor.view().props,
                Props::Input(_)
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(inputs.len(), 5);
    for size in ["extra-small", "small", "medium", "large", "extra-large"] {
        assert!(inputs.iter().any(|id| {
            app.tree()
                .node(*id)
                .unwrap()
                .descriptor
                .view()
                .classes
                .iter()
                .any(|class| class == &format!("sapid-size-{size}"))
        }));
    }
}

#[test]
fn studio_builds_real_role_inputs_mode_controls_and_retires_all_state() {
    for theme in [Theme::blue_light(), Theme::green_dark()] {
        let app = App::memory();
        app.mount(showcase::site(theme, "theme-studio".into()))
            .unwrap();
        let all = nodes(&app);
        let input = all
            .iter()
            .find(|id| {
                app.tree()
                    .node(**id)
                    .unwrap()
                    .descriptor
                    .view()
                    .name
                    .as_deref()
                    == Some("Base canvas")
            })
            .unwrap();
        assert!(matches!(
            app.tree().node(*input).unwrap().descriptor.view().props,
            Props::Input(_)
        ));
        let tab = all
            .iter()
            .find(|id| {
                app.tree()
                    .node(**id)
                    .unwrap()
                    .descriptor
                    .view()
                    .name
                    .as_deref()
                    == Some("Edit Dark colors")
            })
            .unwrap();
        app.dispatch(*tab, Event::Activate).unwrap();
        app.flush().unwrap();
        assert!(nodes(&app).iter().any(|id| {
            app.tree()
                .node(*id)
                .unwrap()
                .descriptor
                .view()
                .name
                .as_deref()
                == Some("Dark canvas")
        }));
        app.clear().unwrap();
        assert_eq!(app.stats().nodes, 0);
        assert_eq!(app.stats().handlers, 0);
        assert_eq!(app.runtime_stats().tasks, 0);
    }
}

#[test]
fn route_changes_preserve_one_editor_workspace_without_retaining_old_views() {
    use sapid_web_example::showcase_navigation::{Page, State, app as route_app};
    use sapid_web_example::theme_studio::model::{Layer, Target};
    let app = App::memory();
    let state = State::new(
        app.scope(),
        Page::builtin(Theme::blue_dark(), "theme-studio".into()),
        Vec::new(),
        false,
    );
    app.mount(route_app(state)).unwrap();
    state.editor.draft.revise(|d| {
        d.record.title = "Kept draft".into();
        d.set_text(Layer::Base, Target::Role(0), "#1".into());
    });
    state.editor.layer.revise(|v| *v = Layer::Dark);
    let mut next = Page::builtin(Theme::green_light(), "button".into());
    next.revision = 1;
    next.shell_revision = 1;
    state.page.revise(|v| *v = next);
    app.flush().unwrap();
    let mut next = Page::builtin(Theme::green_dark(), "theme-studio".into());
    next.revision = 2;
    next.shell_revision = 2;
    state.page.revise(|v| *v = next);
    app.flush().unwrap();
    assert_eq!(
        state.editor.draft.with(|d| d.record.title.clone()),
        "Kept draft"
    );
    assert_eq!(
        state
            .editor
            .draft
            .with(|d| d.text(Layer::Base, Target::Role(0))),
        "#1"
    );
    assert_eq!(state.editor.layer.get(), Layer::Dark);
    app.clear().unwrap();
    assert_eq!(app.stats().nodes, 0);
    assert_eq!(app.stats().handlers, 0);
    drop(app);
    assert!(state.editor.draft.try_get().is_err());
}

#[test]
fn document_navigation_preserves_sidebar_and_updates_current_link() {
    use sapid_web_example::showcase_navigation::{Page, State, app as route_app};
    let app = App::memory();
    let state = State::new(
        app.scope(),
        Page::builtin(Theme::blue_light(), "alert".into()),
        Vec::new(),
        false,
    );
    app.mount(route_app(state)).unwrap();
    let sidebar = nodes(&app)
        .into_iter()
        .find(|id| {
            app.tree()
                .node(*id)
                .unwrap()
                .descriptor
                .view()
                .classes
                .iter()
                .any(|class| class == "site-sidebar")
        })
        .unwrap();
    for (revision, widget) in [(1, "skeleton"), (2, "loading-overlay"), (3, "alert")] {
        state.page.revise(|page| {
            page.route.widget = widget.into();
            page.revision = revision;
        });
        app.flush().unwrap();
        assert!(app.tree().node(sidebar).is_ok());
        let current = nodes(&app)
            .into_iter()
            .filter(|id| {
                app.tree()
                    .node(*id)
                    .unwrap()
                    .descriptor
                    .view()
                    .classes
                    .iter()
                    .any(|class| class == "site-nav-link")
                    && app
                        .tree()
                        .node(*id)
                        .unwrap()
                        .descriptor
                        .view()
                        .classes
                        .iter()
                        .any(|class| class == "is-active")
            })
            .collect::<Vec<_>>();
        assert_eq!(current.len(), 1);
        assert!(
            matches!(app.tree().node(current[0]).unwrap().descriptor.view().props, Props::Link(link) if link.target.contains(widget))
        );
    }
    app.clear().unwrap();
    assert_eq!(app.stats().nodes, 0);
}

#[test]
fn button_pages_have_independent_compiled_examples_and_matching_navigation() {
    for theme in [
        Theme::blue_light(),
        Theme::blue_dark(),
        Theme::green_light(),
        Theme::green_dark(),
    ] {
        for (slug, count) in [
            ("action-icon", 4),
            ("button", 5),
            ("close-button", 3),
            ("copy-button", 4),
            ("file-button", 3),
            ("toggle", 3),
            ("unstyled-button", 3),
        ] {
            let sections =
                sapid_web_example::showcase_examples::detail_sections(slug, theme.clone());
            let navigation = sapid_web_example::showcase_examples::detail_navigation(slug);
            assert_eq!(sections.len(), count, "{slug}");
            assert_eq!(navigation.len(), count, "{slug}");
            for section in sections {
                assert!(
                    section.source.contains("pub fn demo(theme:"),
                    "{}",
                    section.id
                );
                assert!(
                    navigation
                        .iter()
                        .any(|(_, href)| *href == format!("#{}", section.id))
                );
                let app = App::memory();
                app.mount(section.example)
                    .unwrap_or_else(|e| panic!("{slug}/{}: {e}", section.id));
                app.clear().unwrap();
                assert_eq!(app.stats().nodes, 0);
                assert_eq!(app.runtime_stats().tasks, 0);
            }
        }
    }
}

#[test]
fn combobox_guides_have_full_compiled_sources_matching_their_navigation_and_public_api() {
    for theme in [
        Theme::blue_light(),
        Theme::blue_dark(),
        Theme::green_light(),
        Theme::green_dark(),
    ] {
        for (slug, count) in [
            ("autocomplete", 6),
            ("combobox", 4),
            ("combobox-popover", 4),
            ("multi-select", 5),
            ("pill", 4),
            ("pills-input", 5),
            ("tags-input", 6),
            ("tree-select", 9),
            ("cascader", 5),
        ] {
            let sections =
                sapid_web_example::showcase_examples::detail_sections(slug, theme.clone());
            let navigation = sapid_web_example::showcase_examples::detail_navigation(slug);
            assert_eq!(sections.len(), count, "{slug}");
            assert_eq!(navigation.len(), count, "{slug}");
            for section in sections {
                assert!(
                    section.source.contains("pub fn demo(theme:"),
                    "{}",
                    section.id
                );
                assert!(
                    navigation
                        .iter()
                        .any(|(_, href)| *href == format!("#{}", section.id)),
                    "{}",
                    section.id
                );
                let app = App::memory();
                app.mount(section.example)
                    .unwrap_or_else(|e| panic!("{slug}/{}: {e}", section.id));
                app.clear().unwrap();
                assert_eq!(app.stats().nodes, 0);
                assert_eq!(app.runtime_stats().tasks, 0);
            }
        }
    }
    for (slug, methods) in [
        (
            "pills-input",
            vec!["state", "placeholder", "max_tags", "field", "size"],
        ),
        (
            "cascader",
            vec![
                "theme",
                "size",
                "field",
                "clearable",
                "with_columns",
                "change_on_select",
                "clear_label",
            ],
        ),
        ("multi-select", vec!["option_content", "dropdown_label"]),
    ] {
        let entry = sapid_web_example::showcase_catalog::ENTRIES
            .iter()
            .find(|e| e.slug == slug)
            .unwrap();
        let api = sapid_web_example::showcase_api::find(entry.id).unwrap();
        for method in methods {
            assert!(api.builders.contains(&method), "{slug} missing {method}");
        }
    }
}

#[test]
fn remaining_navigation_guides_compile_every_source_and_retire_registered_headings() {
    for theme in [
        Theme::blue_light(),
        Theme::blue_dark(),
        Theme::green_light(),
        Theme::green_dark(),
    ] {
        for (slug, count) in [("stepper", 4), ("table-of-contents", 3), ("tree", 8)] {
            let sections =
                sapid_web_example::showcase_examples::detail_sections(slug, theme.clone());
            let navigation = sapid_web_example::showcase_examples::detail_navigation(slug);
            assert_eq!(sections.len(), count, "{slug}");
            assert_eq!(navigation.len(), count, "{slug}");
            for section in sections {
                assert!(
                    section.source.contains("pub fn demo(theme:"),
                    "{}",
                    section.id
                );
                assert!(
                    navigation
                        .iter()
                        .any(|(_, href)| *href == format!("#{}", section.id))
                );
                let app = App::memory();
                app.mount(section.example)
                    .unwrap_or_else(|e| panic!("{slug}/{}: {e}", section.id));
                app.clear().unwrap();
                assert_eq!(app.stats().nodes, 0);
                assert_eq!(app.stats().handlers, 0);
                assert_eq!(app.runtime_stats().tasks, 0);
            }
        }
    }
}

#[test]
fn feedback_guides_compile_full_sources_and_match_their_navigation() {
    for theme in [
        Theme::blue_light(),
        Theme::blue_dark(),
        Theme::green_light(),
        Theme::green_dark(),
    ] {
        for (slug, count) in [
            ("alert", 4),
            ("empty-state", 3),
            ("loader", 4),
            ("notification", 3),
            ("skeleton", 3),
            ("progress", 9),
            ("ring-progress", 9),
            ("semi-circle-progress", 9),
        ] {
            let sections =
                sapid_web_example::showcase_examples::detail_sections(slug, theme.clone());
            let navigation = sapid_web_example::showcase_examples::detail_navigation(slug);
            assert_eq!(sections.len(), count, "{slug}");
            assert_eq!(navigation.len(), count, "{slug}");
            for section in sections {
                assert!(
                    section.source.contains("pub fn demo(theme:"),
                    "{}",
                    section.id
                );
                assert!(
                    navigation
                        .iter()
                        .any(|(_, href)| *href == format!("#{}", section.id)),
                    "{}",
                    section.id
                );
                let app = App::memory();
                app.mount(section.example)
                    .unwrap_or_else(|e| panic!("{slug}/{}: {e}", section.id));
                app.clear().unwrap();
                assert_eq!(app.stats().nodes, 0);
                assert_eq!(app.stats().handlers, 0);
                assert_eq!(app.runtime_stats().tasks, 0);
            }
        }
    }
}

#[test]
fn overlay_guides_compile_complete_sources_match_navigation_and_release_state() {
    for theme in [
        Theme::blue_light(),
        Theme::blue_dark(),
        Theme::green_light(),
        Theme::green_dark(),
    ] {
        for (slug, count) in [
            ("modal", 6),
            ("drawer", 5),
            ("popover", 5),
            ("dialog", 3),
            ("loading-overlay", 3),
            ("overlay", 3),
            ("tooltip", 5),
            ("hover-card", 4),
            ("action-bar", 3),
            ("floating-indicator", 3),
            ("tour", 3),
        ] {
            let sections =
                sapid_web_example::showcase_examples::detail_sections(slug, theme.clone());
            let navigation = sapid_web_example::showcase_examples::detail_navigation(slug);
            assert_eq!(sections.len(), count, "{slug}");
            assert_eq!(navigation.len(), count, "{slug}");
            for section in sections {
                assert!(
                    section.source.contains("pub fn demo(theme:"),
                    "{}",
                    section.id
                );
                assert!(
                    navigation
                        .iter()
                        .any(|(_, href)| *href == format!("#{}", section.id)),
                    "{}",
                    section.id
                );
                let app = App::memory();
                app.mount(section.example)
                    .unwrap_or_else(|e| panic!("{slug}/{}: {e}", section.id));
                app.clear().unwrap();
                assert_eq!(app.stats().nodes, 0);
                assert_eq!(app.stats().handlers, 0);
                assert_eq!(app.runtime_stats().tasks, 0);
            }
        }
    }
}

#[test]
fn data_display_guides_mount_complete_public_examples_in_four_themes_and_release_owners() {
    for theme in [
        Theme::blue_light(),
        Theme::blue_dark(),
        Theme::green_light(),
        Theme::green_dark(),
    ] {
        for (slug, count) in [
            ("avatar", 4),
            ("badge", 4),
            ("card", 3),
            ("color-swatch", 3),
            ("data-list", 3),
            ("image", 4),
            ("indicator", 3),
            ("kbd", 3),
            ("number-formatter", 3),
            ("overflow-list", 3),
            ("rolling-number", 3),
            ("spoiler", 3),
            ("theme-icon", 3),
        ] {
            let sections =
                sapid_web_example::showcase_examples::detail_sections(slug, theme.clone());
            let navigation = sapid_web_example::showcase_examples::detail_navigation(slug);
            assert_eq!(sections.len(), count, "{slug}");
            assert_eq!(navigation.len(), count, "{slug}");
            for section in sections {
                assert!(
                    section.source.contains("pub fn demo(theme:"),
                    "{}",
                    section.id
                );
                assert!(
                    navigation
                        .iter()
                        .any(|(_, href)| *href == format!("#{}", section.id)),
                    "{}",
                    section.id
                );
                let app = App::memory();
                app.mount(section.example)
                    .unwrap_or_else(|e| panic!("{slug}/{}: {e}", section.id));
                app.clear().unwrap();
                assert_eq!(app.stats().nodes, 0);
                assert_eq!(app.stats().handlers, 0);
                assert_eq!(app.runtime_stats().tasks, 0);
            }
        }
    }
}

#[test]
fn typography_guides_mount_complete_document_examples_match_navigation_and_retire() {
    for theme in [
        Theme::blue_light(),
        Theme::blue_dark(),
        Theme::green_light(),
        Theme::green_dark(),
    ] {
        for (slug, count) in [
            ("blockquote", 3),
            ("code", 3),
            ("highlight", 4),
            ("list", 4),
            ("mark", 3),
            ("table", 6),
            ("text", 4),
            ("title", 3),
            ("typography", 3),
        ] {
            let sections =
                sapid_web_example::showcase_examples::detail_sections(slug, theme.clone());
            let navigation = sapid_web_example::showcase_examples::detail_navigation(slug);
            assert_eq!(sections.len(), count, "{slug}");
            assert_eq!(navigation.len(), count, "{slug}");
            for section in sections {
                assert!(
                    section.source.contains("pub fn demo(theme:"),
                    "{}",
                    section.id
                );
                assert!(
                    navigation
                        .iter()
                        .any(|(_, href)| *href == format!("#{}", section.id)),
                    "{}",
                    section.id
                );
                let app = App::memory();
                app.mount(section.example)
                    .unwrap_or_else(|e| panic!("{slug}/{}: {e}", section.id));
                app.clear().unwrap();
                assert_eq!(app.stats().nodes, 0);
                assert_eq!(app.stats().handlers, 0);
                assert_eq!(app.runtime_stats().tasks, 0);
            }
        }
    }
}

#[test]
fn miscellaneous_guides_mount_complete_document_examples_match_navigation_and_retire() {
    for theme in [
        Theme::blue_light(),
        Theme::blue_dark(),
        Theme::green_light(),
        Theme::green_dark(),
    ] {
        for (slug, count) in [
            ("box", 3),
            ("collapse", 4),
            ("divider", 4),
            ("focus-trap", 3),
            ("marquee", 3),
            ("paper", 3),
            ("portal", 3),
            ("scroll-area", 4),
            ("toolbar", 4),
            ("visually-hidden", 3),
        ] {
            let sections =
                sapid_web_example::showcase_examples::detail_sections(slug, theme.clone());
            let navigation = sapid_web_example::showcase_examples::detail_navigation(slug);
            assert_eq!(sections.len(), count, "{slug}");
            assert_eq!(navigation.len(), count, "{slug}");
            for section in sections {
                assert!(
                    section.source.contains("pub fn demo(theme:"),
                    "{}",
                    section.id
                );
                assert!(
                    navigation
                        .iter()
                        .any(|(_, href)| *href == format!("#{}", section.id)),
                    "{}",
                    section.id
                );
                let app = App::memory();
                app.mount(section.example)
                    .unwrap_or_else(|e| panic!("{slug}/{}: {e}", section.id));
                app.clear().unwrap();
                assert_eq!(app.stats().nodes, 0);
                assert_eq!(app.stats().handlers, 0);
                assert_eq!(app.runtime_stats().tasks, 0);
            }
        }
    }
}

#[test]
fn date_guides_mount_complete_document_examples_match_navigation_and_retire() {
    for theme in [
        Theme::blue_light(),
        Theme::blue_dark(),
        Theme::green_light(),
        Theme::green_dark(),
    ] {
        for (slug, count) in [
            ("date-picker", 11),
            ("inline-date-time-picker", 6),
            ("date-input", 5),
            ("month-picker", 4),
            ("year-picker", 4),
            ("time-input", 5),
            ("time-picker", 15),
            ("time-value", 3),
            ("calendar", 5),
        ] {
            let sections =
                sapid_web_example::showcase_examples::detail_sections(slug, theme.clone());
            let navigation = sapid_web_example::showcase_examples::detail_navigation(slug);
            assert_eq!(sections.len(), count, "{slug}");
            assert_eq!(navigation.len(), count, "{slug}");
            for section in sections {
                assert!(
                    section.source.contains("pub fn demo(theme:"),
                    "{}",
                    section.id
                );
                assert!(
                    navigation
                        .iter()
                        .any(|(_, href)| *href == format!("#{}", section.id)),
                    "{}",
                    section.id
                );
                let app = App::memory();
                app.mount(section.example)
                    .unwrap_or_else(|e| panic!("{slug}/{}: {e}", section.id));
                app.clear().unwrap();
                assert_eq!(app.stats().nodes, 0);
                assert_eq!(app.stats().handlers, 0);
                assert_eq!(app.runtime_stats().tasks, 0);
            }
        }
    }
}
