use glory::{
    apace::shell::WindowApp,
    prelude::*,
    render::{NodeId, Props, Query, QueryValue, Rect},
};
use std::{cell::RefCell, rc::Rc};
fn bounds(app: &App, id: NodeId) -> Rect {
    let saved = Rc::new(RefCell::new(None));
    let out = saved.clone();
    app.request(Query::Bounds(id), move |v| {
        *out.borrow_mut() = Some(v.unwrap())
    })
    .unwrap();
    app.pump().unwrap();
    let QueryValue::Bounds(rect) = saved.borrow_mut().take().unwrap() else {
        panic!("bounds")
    };
    rect
}
fn find(app: &App, predicate: impl Fn(glory::render::DescriptorView<'_>) -> bool) -> NodeId {
    let tree = app.tree();
    let mut pending = tree.roots().to_vec();
    while let Some(id) = pending.pop() {
        let n = tree.node(id).unwrap();
        if predicate(n.descriptor.view()) {
            return id;
        }
        pending.extend(n.children.iter().copied());
    }
    panic!("node")
}
#[test]
fn short_tokens_and_the_real_editor_share_a_line_in_multi_tags_and_pills_inputs() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        for kind in 0..3 {
            let host = WindowApp::new();
            host.renderer().set_viewport(360, 180, 1.).unwrap();
            let style = Style::new().width(Length::Px(320.));
            let values = vec!["rust".to_owned(), "ui".to_owned()];
            let widget: Box<dyn Widget> = match kind {
                0 => Box::new(
                    sapid::multi_select("token-layout")
                        .theme(theme.clone())
                        .name("Skills")
                        .style(style)
                        .default_value(values)
                        .options([
                            sapid::select_option("rust", "Rust"),
                            sapid::select_option("ui", "UI"),
                        ]),
                ),
                1 => Box::new(
                    sapid::tags_input("token-layout")
                        .theme(theme.clone())
                        .name("Skills")
                        .style(style)
                        .default_value(values),
                ),
                _ => Box::new(
                    sapid::pills_input("token-layout")
                        .theme(theme.clone())
                        .name("Skills")
                        .style(style)
                        .default_value(values),
                ),
            };
            host.mount(widget).unwrap();
            let app = host.app();
            app.settle(16).unwrap();
            let editor = find(app, |d| matches!(d.props, Props::Input(_)));
            let label = find(app, |d| d.classes.iter().any(|c| c == "sapid-tag-label"));
            let a = bounds(app, editor);
            let b = bounds(app, label);
            assert!(
                (a.y + a.height / 2. - b.y - b.height / 2.).abs() < 5.,
                "kind {kind}: tokens {b:?}, editor {a:?}"
            );
            assert!(a.x > b.x, "editor must follow selected tokens");
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0);
        }
    }
}

#[test]
fn wrapping_and_clearing_preserve_the_real_editor_and_bound_long_token_labels() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        for kind in 0..3 {
            let host = WindowApp::new();
            host.renderer().set_viewport(360, 600, 1.).unwrap();
            let app = host.app();
            let values = app.scope().cage(vec!["rust".to_owned(), "ui".to_owned()]);
            let state = app.scope().cage(sapid::TagsInputState {
                values: values.get(),
                search: "draft".into(),
            });
            let reference = app.scope().cage(None);
            let style = Style::new().width(Length::Px(320.));
            let long = "a".repeat(200);
            let expanded = std::iter::once(long.clone())
                .chain((0..8).map(|i| format!("framework-{i}")))
                .collect::<Vec<_>>();
            let options = values
                .get()
                .into_iter()
                .chain(expanded.clone())
                .map(|s| {
                    sapid::select_option(
                        if s.len() > 100 {
                            "long".to_owned()
                        } else {
                            s.clone()
                        },
                        s,
                    )
                })
                .collect::<Vec<_>>();
            let widget: Box<dyn Widget> = match kind {
                0 => Box::new(
                    sapid::multi_select("wrapping-tokens")
                        .theme(theme.clone())
                        .name("Skills")
                        .style(style)
                        .node_ref(reference)
                        .value(values)
                        .default_search("draft")
                        .options(options),
                ),
                1 => Box::new(
                    sapid::tags_input("wrapping-tokens")
                        .theme(theme.clone())
                        .name("Skills")
                        .style(style)
                        .node_ref(reference)
                        .state(state),
                ),
                _ => Box::new(
                    sapid::pills_input("wrapping-tokens")
                        .theme(theme.clone())
                        .name("Skills")
                        .style(style)
                        .node_ref(reference)
                        .state(state),
                ),
            };
            host.mount(widget).unwrap();
            app.settle(16).unwrap();
            let editor = reference.get().unwrap();
            let physical = host
                .renderer()
                .engine_id(editor, glory::html::Part::Main)
                .unwrap();
            let frame = find(app, |d| {
                d.classes.iter().any(|c| {
                    c == if kind == 0 {
                        "sapid-multi-select-shell"
                    } else {
                        "sapid-tags-input-shell"
                    }
                })
            });
            let initial = bounds(app, frame);
            for next in [expanded, Vec::new()] {
                if kind == 0 {
                    values.revise(|old| {
                        *old = next
                            .iter()
                            .map(|s| {
                                if s.len() > 100 {
                                    "long".to_owned()
                                } else {
                                    s.clone()
                                }
                            })
                            .collect()
                    });
                } else {
                    state.revise(|old| old.values = next.clone());
                }
                app.settle(16).unwrap();
                let outer = bounds(app, frame);
                assert!(outer.width <= 320.1);
                if next.is_empty() {
                    assert!((outer.height - initial.height).abs() < 1.);
                } else {
                    assert!(outer.height > initial.height + 20.);
                    let label = find(app, |d| matches!(d.props,Props::Text(p)if p.content==long));
                    let label = bounds(app, label);
                    assert!(
                        label.x >= outer.x && label.x + label.width <= outer.x + outer.width + 0.1
                    );
                }
                assert_eq!(reference.get(), Some(editor));
                assert_eq!(
                    host.renderer()
                        .engine_id(editor, glory::html::Part::Main)
                        .unwrap(),
                    physical
                );
                assert!(
                    matches!(app.tree().node(editor).unwrap().descriptor.view().props,Props::Input(p)if p.value=="draft")
                );
            }
            app.clear().unwrap();
            assert_eq!(app.stats().nodes, 0);
            assert_eq!(app.runtime_stats().tasks, 0);
        }
    }
}

#[test]
fn inset_clear_and_loader_align_with_the_last_editor_row_and_clear_restores_focus() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        for kind in 0..3 {
            for size in [
                sapid::Size::ExtraSmall,
                sapid::Size::Small,
                sapid::Size::Medium,
                sapid::Size::Large,
                sapid::Size::ExtraLarge,
            ] {
                let host = WindowApp::new();
                host.renderer().set_viewport(360, 180, 1.).unwrap();
                let app = host.app();
                let reference = app.scope().cage(None);
                let loading = app.scope().cage(false);
                let style = Style::new().width(Length::Px(320.));
                let widget: Box<dyn Widget> = match kind {
                    0 => Box::new(
                        sapid::multi_select("inset-tokens")
                            .theme(theme.clone())
                            .size(size)
                            .name("Skills")
                            .style(style)
                            .node_ref(reference)
                            .default_value(vec!["rust".into(), "ui".into()])
                            .options([
                                sapid::select_option("rust", "Rust"),
                                sapid::select_option("ui", "UI"),
                            ])
                            .clearable(true)
                            .loading(loading),
                    ),
                    1 => Box::new(
                        sapid::tags_input("inset-tokens")
                            .theme(theme.clone())
                            .size(size)
                            .name("Skills")
                            .style(style)
                            .node_ref(reference)
                            .default_value(vec!["Rust".into(), "UI".into()])
                            .clearable(true)
                            .loading(loading),
                    ),
                    _ => Box::new(
                        sapid::pills_input("inset-tokens")
                            .theme(theme.clone())
                            .size(size)
                            .name("Skills")
                            .style(style)
                            .node_ref(reference)
                            .default_value(vec!["Rust".into(), "UI".into()])
                            .clearable(true)
                            .loading(loading),
                    ),
                };
                host.mount(widget).unwrap();
                app.settle(16).unwrap();
                let editor = reference.get().unwrap();
                let er = bounds(app, editor);
                let clear = find(app, |d| {
                    d.classes.iter().any(|c| {
                        c == if kind == 0 {
                            "sapid-multi-select-clear"
                        } else {
                            "sapid-tags-input-clear"
                        }
                    })
                });
                let cr = bounds(app, clear);
                assert_eq!((cr.width, cr.height), (24., 24.));
                assert!(
                    (er.y + er.height / 2. - cr.y - cr.height / 2.).abs() < 0.5,
                    "clear {kind}/{size:?}: {er:?}, {cr:?}"
                );
                loading.revise(|v| *v = true);
                app.flush().unwrap();
                let loader = find(app, |d| {
                    d.classes.iter().any(|c| {
                        c == if kind == 0 {
                            "sapid-combobox-loading"
                        } else {
                            "sapid-tags-input-loading"
                        }
                    })
                });
                let lr = bounds(app, loader);
                assert_eq!((lr.width, lr.height), (16., 16.));
                assert!(
                    (er.y + er.height / 2. - lr.y - lr.height / 2.).abs() < 0.5,
                    "loader {kind}/{size:?}: {er:?}, {lr:?}"
                );
                let remove = find(app, |d| d.classes.iter().any(|c| c == "sapid-tag-remove"));
                app.dispatch(remove, glory::render::Event::Activate)
                    .unwrap();
                app.settle(16).unwrap();
                let focus = Rc::new(RefCell::new(None));
                let out = focus.clone();
                app.request(Query::Focus(editor), move |v| {
                    *out.borrow_mut() = Some(v.unwrap())
                })
                .unwrap();
                app.pump().unwrap();
                assert_eq!(
                    focus.borrow_mut().take(),
                    Some(QueryValue::Focus(true)),
                    "remove {kind}/{size:?}"
                );
                app.dispatch(clear, glory::render::Event::Activate).unwrap();
                app.settle(16).unwrap();
                let saved = Rc::new(RefCell::new(None));
                let out = saved.clone();
                app.request(Query::Focus(editor), move |v| {
                    *out.borrow_mut() = Some(v.unwrap())
                })
                .unwrap();
                app.pump().unwrap();
                assert_eq!(saved.borrow_mut().take(), Some(QueryValue::Focus(true)));
                assert_eq!(reference.get(), Some(editor));
                app.clear().unwrap();
                assert_eq!(app.stats().nodes, 0);
            }
        }
    }
}

#[test]
fn disclosure_keeps_focus_on_the_real_editor_and_proposes_opening_once() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.renderer().set_viewport(360, 260, 1.).unwrap();
        let app = host.app();
        let editor = app.scope().cage(None);
        let open = app.scope().cage(false);
        let accept = app.scope().cage(false);
        let requests = app.scope().cage(0);
        host.mount(
            sapid::multi_select("native-disclosure")
                .theme(theme)
                .name("Skills")
                .style(Style::new().width(Length::Px(320.)))
                .node_ref(editor)
                .dropdown_label("Open skill options")
                .open(open)
                .on_open_change(move |v| {
                    requests.revise(|old| *old += 1);
                    if accept.get() {
                        open.revise(|old| *old = v)
                    }
                })
                .options([
                    sapid::select_option("rust", "Rust"),
                    sapid::select_option("ui", "UI"),
                ]),
        )
        .unwrap();
        app.settle(16).unwrap();
        let editor = editor.get().unwrap();
        let disclosure = find(app, |d| d.name.as_deref() == Some("Open skill options"));
        let physical = host
            .renderer()
            .engine_id(editor, glory::html::Part::Main)
            .unwrap();
        for (accepted, count) in [(false, 1), (true, 2)] {
            accept.revise(|v| *v = accepted);
            app.scope()
                .action(glory::render::Action::Focus(disclosure))
                .unwrap();
            app.settle(16).unwrap();
            app.dispatch(disclosure, glory::render::Event::Activate)
                .unwrap();
            app.settle(16).unwrap();
            assert_eq!(requests.get(), count);
            assert_eq!(open.get(), accepted);
            let value = Rc::new(RefCell::new(None));
            let out = value.clone();
            app.request(Query::Focus(editor), move |v| {
                *out.borrow_mut() = Some(v.unwrap())
            })
            .unwrap();
            app.pump().unwrap();
            assert_eq!(value.borrow_mut().take(), Some(QueryValue::Focus(true)));
            assert_eq!(
                host.renderer()
                    .engine_id(editor, glory::html::Part::Main)
                    .unwrap(),
                physical
            );
        }
        let panel = find(app, |d| d.role == Some(glory::render::Role::Listbox));
        let rect = bounds(app, panel);
        assert!(rect.x >= 0. && rect.x + rect.width <= 360.1);
        assert!(
            (rect.width - 320.).abs() < 1.,
            "listbox must match the complete token surface: {rect:?}"
        );
        assert!(
            app.tree()
                .resolve_active_descendant(editor)
                .unwrap()
                .is_some()
        );
        app.clear().unwrap();
        assert_eq!(app.stats().nodes, 0);
    }
}
