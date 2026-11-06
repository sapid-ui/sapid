use glory::prelude::*;
use glory::render::{Event, EventKind, InputProps, NodeId, Props, RangeCommand, Role, Semantics};
use sapid::{ColorEyeDropper, ColorFormat, Hsva, color_input, color_picker};
use std::{cell::RefCell, rc::Rc};
fn nodes(app: &App, class: &str) -> Vec<NodeId> {
    let tree = app.tree();
    let mut todo = tree.roots().to_vec();
    let mut out = Vec::new();
    while let Some(id) = todo.pop() {
        let node = tree.node(id).unwrap();
        if node.descriptor.view().classes.iter().any(|c| c == class) {
            out.push(id);
        }
        todo.extend(&node.children);
    }
    out
}
fn slider(app: &App, name: &str) -> NodeId {
    let tree = app.tree();
    let mut todo = tree.roots().to_vec();
    while let Some(id) = todo.pop() {
        let node = tree.node(id).unwrap();
        if node.descriptor.view().role == Some(Role::Slider)
            && node.descriptor.view().name.as_deref() == Some(name)
        {
            return id;
        }
        todo.extend(&node.children);
    }
    panic!("missing {name}")
}
fn metadata(app: &App, id: NodeId) -> Semantics {
    app.tree()
        .node(id)
        .unwrap()
        .descriptor
        .view()
        .semantics
        .as_deref()
        .cloned()
        .unwrap_or_default()
}
fn number(app: &App, id: NodeId) -> f64 {
    metadata(app, id).slider.unwrap().range.value
}
fn key(app: &App, id: NodeId, command: RangeCommand) {
    app.dispatch(id, Event::Range { from: id, command })
        .unwrap();
}
fn input(app: &App, id: NodeId) -> InputProps {
    let Props::Input(value) = app.tree().node(id).unwrap().descriptor.view().props.clone() else {
        panic!("input")
    };
    value
}
fn edit(app: &App, id: NodeId, value: &str) {
    app.dispatch(
        id,
        Event::ValueChanged {
            value: value.into(),
            revision: input(app, id).revision,
            composing: false,
        },
    )
    .unwrap();
    app.flush().unwrap();
}
#[test]
fn local_picker_has_four_true_keyboard_sliders_and_bounded_passive_plane() {
    let app = App::memory();
    let root = app.scope().cage(None);
    app.mount(
        color_picker("local")
            .name("Precise color")
            .show_precision_sliders(true)
            .node_ref(root)
            .default_value(Hsva {
                hue: 120.123456789,
                saturation: 0.5,
                value: 0.75,
                alpha: 0.4,
            }),
    )
    .unwrap();
    assert_eq!(
        app.tree()
            .node(root.get().unwrap())
            .unwrap()
            .descriptor
            .view()
            .role,
        Some(Role::Group)
    );
    assert_eq!(nodes(&app, "sapid-color-plane-hue").len(), 1);
    assert_eq!(nodes(&app, "sapid-color-plane-saturation").len(), 1);
    assert_eq!(nodes(&app, "sapid-color-plane-value").len(), 1);
    assert!(nodes(&app, "sapid-color-plane-cell").is_empty());
    assert_eq!(
        metadata(&app, nodes(&app, "sapid-color-plane")[0]).drag_capture,
        Some(glory::render::DragCapture::Press)
    );
    let saturation = slider(&app, "Saturation");
    app.scope()
        .batch(|| {
            key(&app, saturation, RangeCommand::Increase);
            key(&app, saturation, RangeCommand::Increase);
        })
        .unwrap();
    assert!((number(&app, saturation) - 0.52).abs() < 1e-12);
    assert_eq!(number(&app, slider(&app, "Hue")), 120.123456789);
    key(&app, slider(&app, "Brightness"), RangeCommand::Maximum);
    assert_eq!(number(&app, slider(&app, "Brightness")), 1.);
}

#[test]
fn standalone_picker_supports_swatches_only_full_width_and_atomic_selection() {
    let app = App::memory();
    let source = app.scope().cage(Hsva::default());
    let commits = app.scope().cage(0_u32);
    let root = app.scope().cage(None);
    app.mount(
        color_picker("palette")
            .value(source)
            .show_picker(false)
            .full_width(true)
            .swatches(["#ff0000", "#00ff00", "#0000ff"])
            .swatches_per_row(3)
            .node_ref(root)
            .on_value_change(move |next| source.revise(|value| *value = next))
            .on_value_commit(move |_| commits.revise(|value| *value += 1)),
    )
    .unwrap();

    assert!(nodes(&app, "sapid-color-plane").is_empty());
    assert_eq!(nodes(&app, "sapid-color-picker-swatches").len(), 1);
    let swatches = nodes(&app, "sapid-color-picker-swatch");
    assert_eq!(swatches.len(), 3);
    assert_eq!(
        app.tree()
            .node(root.get().unwrap())
            .unwrap()
            .descriptor
            .view()
            .style
            .width,
        Some(glory::style::Length::Percent(100.))
    );

    app.dispatch(swatches[1], Event::Activate).unwrap();
    assert_eq!(
        source.get().to_rgba().unwrap(),
        glory::style::Color(0, 255, 0, 255)
    );
    assert_eq!(commits.get(), 1);
}

#[test]
fn picker_channel_labels_are_configurable_on_the_real_keyboard_controls() {
    let app = App::memory();
    app.mount(
        color_picker("labels")
            .show_precision_sliders(true)
            .saturation_label("Color intensity")
            .brightness_label("Light output")
            .hue_label("Color angle")
            .alpha_label("Transparency"),
    )
    .unwrap();
    for name in [
        "Color intensity",
        "Light output",
        "Color angle",
        "Transparency",
    ] {
        assert_eq!(
            app.tree()
                .node(slider(&app, name))
                .unwrap()
                .descriptor
                .view()
                .role,
            Some(Role::Slider)
        );
    }
}
#[test]
fn controlled_refusal_emits_one_change_then_commit_and_never_mutates_source() {
    let app = App::memory();
    let source = app.scope().cage(Hsva::default());
    let log = Rc::new(RefCell::new(Vec::new()));
    let a = log.clone();
    let b = log.clone();
    app.mount(
        color_picker("controlled")
            .show_precision_sliders(true)
            .value(source)
            .on_value_change(move |v| a.borrow_mut().push(("change", v)))
            .on_value_commit(move |v| b.borrow_mut().push(("commit", v))),
    )
    .unwrap();
    let sat = slider(&app, "Saturation");
    key(&app, sat, RangeCommand::Decrease);
    assert_eq!(number(&app, sat), 1.);
    assert_eq!(source.get(), Hsva::default());
    let log = log.borrow();
    assert_eq!(log.len(), 2);
    assert_eq!(log[0].0, "change");
    assert_eq!(log[1].0, "commit");
    assert_eq!(log[0].1, log[1].1);
}
#[test]
fn commit_only_accepted_changes_accumulate_across_channels_in_one_tick() {
    let app = App::memory();
    let source = app.scope().cage(Hsva {
        hue: 80.,
        saturation: 0.5,
        value: 0.75,
        alpha: 0.4,
    });
    app.mount(
        color_picker("accepted")
            .show_precision_sliders(true)
            .value(source)
            .on_value_commit(move |next| source.revise(|value| *value = next)),
    )
    .unwrap();
    let sat = slider(&app, "Saturation");
    let value = slider(&app, "Brightness");
    app.scope()
        .batch(|| {
            key(&app, sat, RangeCommand::Increase);
            key(&app, sat, RangeCommand::Increase);
            key(&app, value, RangeCommand::Decrease);
        })
        .unwrap();
    assert!((source.get().saturation - 0.52).abs() < 1e-12);
    assert!((source.get().value - 0.74).abs() < 1e-12);
    assert_eq!(source.get().alpha, 0.4);
}
#[test]
fn readonly_and_disabled_preserve_values_and_refuse_generic_drag_and_range_observers() {
    for readonly in [true, false] {
        let app = App::memory();
        let observed = Rc::new(RefCell::new(0));
        let seen = observed.clone();
        app.mount(
            color_picker("blocked")
                .show_precision_sliders(true)
                .readonly(readonly)
                .enabled(readonly)
                .on_value_change(move |_| *seen.borrow_mut() += 1),
        )
        .unwrap();
        let sat = slider(&app, "Saturation");
        let result = app.dispatch(
            sat,
            Event::Range {
                from: sat,
                command: RangeCommand::Decrease,
            },
        );
        if readonly {
            result.unwrap();
        } else {
            assert!(result.is_err());
        }
        assert_eq!(number(&app, sat), 1.);
        assert_eq!(*observed.borrow(), 0);
        assert!(metadata(&app, nodes(&app, "sapid-color-plane")[0]).disabled);
    }
}
#[test]
fn invalid_program_source_keeps_accepted_tree_and_later_recovers_without_callbacks() {
    let app = App::memory();
    let source = app.scope().cage(Hsva::default());
    app.mount(color_picker("invalid").value(source)).unwrap();
    let hue = slider(&app, "Hue");
    source.revise(|value| value.hue = f64::NAN);
    assert!(app.flush().is_err());
    assert_eq!(number(&app, hue), 0.);
    source.revise(|value| value.hue = 240.);
    app.flush().unwrap();
    assert_eq!(number(&app, hue), 240.);
    assert_eq!(slider(&app, "Hue"), hue);
}
#[test]
fn malformed_hex_remains_editable_and_error_link_recovers_with_same_input_identity() {
    let app = App::memory();
    app.mount(color_input("draft").default_value("#g")).unwrap();
    let id = nodes(&app, "sapid-color-editor")[0];
    let error = nodes(&app, "sapid-color-validation")[0];
    assert_eq!(input(&app, id).value, "#g");
    assert!(metadata(&app, id).invalid);
    assert_eq!(metadata(&app, id).error_message, metadata(&app, error).id);
    edit(&app, id, "#123456");
    assert!(!metadata(&app, id).invalid);
    assert!(metadata(&app, error).hidden);
    assert_eq!(nodes(&app, "sapid-color-editor")[0], id);
    edit(&app, id, "");
    assert_eq!(input(&app, id).value, "");
    assert!(!metadata(&app, id).invalid);
}

#[test]
fn color_input_popup_keeps_the_continuous_picker_and_every_declared_swatch() {
    let app = App::memory();
    app.mount(
        color_input("complete-popup")
            .default_open(true)
            .swatches(["#fa5252", "#12b886", "#228be6"]),
    )
    .unwrap();
    assert_eq!(nodes(&app, "sapid-color-picker").len(), 1);
    assert_eq!(nodes(&app, "sapid-color-swatch").len(), 4);
    assert_eq!(nodes(&app, "sapid-color-picker-swatch").len(), 0);
}
#[test]
fn hex_controlled_refusal_keeps_source_and_typed_change_precedes_generic_observer() {
    let app = App::memory();
    let source = app.scope().cage("#ff0000".to_owned());
    let log = Rc::new(RefCell::new(Vec::new()));
    let a = log.clone();
    let b = log.clone();
    app.mount(
        color_input("refuse")
            .value(source)
            .on_value_change(move |_| a.borrow_mut().push("change"))
            .on(EventKind::ValueChanged, move |_| {
                b.borrow_mut().push("observer")
            }),
    )
    .unwrap();
    let id = nodes(&app, "sapid-color-editor")[0];
    edit(&app, id, "#00ff00");
    assert_eq!(source.get(), "#ff0000");
    assert_eq!(input(&app, id).value, "#ff0000");
    assert_eq!(*log.borrow(), vec!["change", "observer"]);
}
#[test]
fn finite_keys_dimensions_and_drafts_are_rejected_before_unbounded_construction() {
    for side in [f64::NAN, 0., 127., 513.] {
        assert!(
            App::memory()
                .mount(color_picker("size").plane_size(side))
                .is_err()
        );
    }
    assert!(App::memory().mount(color_picker("x".repeat(49))).is_err());
    assert!(
        App::memory()
            .mount(color_picker("bad-label").hue_label(" "))
            .is_err()
    );
    assert!(
        App::memory()
            .mount(color_picker("empty-palette").show_picker(false))
            .is_err()
    );
    assert!(
        App::memory()
            .mount(color_picker("bad-palette").swatches(["not-a-color"]))
            .is_err()
    );
    assert!(
        App::memory()
            .mount(color_picker("too-many-columns").swatches_per_row(17))
            .is_err()
    );
    assert!(App::memory().mount(color_input("x".repeat(47))).is_err());
    assert!(
        App::memory()
            .mount(color_input("budget").default_value("x".repeat(257)))
            .is_err()
    );
    let app = App::memory();
    app.mount(color_picker("no-alpha").show_alpha(false).unstyled())
        .unwrap();
    assert_eq!(nodes(&app, "sapid-alpha-slider").len(), 0);
    assert!(nodes(&app, "sapid-color-plane-cell").is_empty());
}
#[test]
fn disabled_capture_still_delivers_terminal_cancel_cleanup_to_generic_observers() {
    let app = App::memory();
    let seen = Rc::new(RefCell::new(Vec::new()));
    let observe = seen.clone();
    app.mount(
        color_picker("cancel")
            .enabled(false)
            .on(EventKind::Drag, move |event| {
                observe.borrow_mut().push(event)
            }),
    )
    .unwrap();
    let id = nodes(&app, "sapid-color-plane")[0];
    app.dispatch(
        id,
        Event::Drag {
            from: id,
            phase: glory::render::DragPhase::Cancel,
            x: 3.,
            y: 4.,
        },
    )
    .unwrap();
    assert!(matches!(
        &seen.borrow()[0],
        Event::Drag {
            phase: glory::render::DragPhase::Cancel,
            ..
        }
    ));
}
#[test]
fn hex_only_mode_keeps_field_contract_and_rejects_conflicting_popup_configuration() {
    let app = App::memory();
    app.mount(
        sapid::field("hex")
            .label("Accent")
            .description("A color")
            .required(true)
            .readonly(true)
            .form_name("accent")
            .control(|field| color_input("hex").show_picker(false).field(field)),
    )
    .unwrap();
    let id = nodes(&app, "sapid-color-editor")[0];
    let state = metadata(&app, id);
    assert!(
        state.required
            && state.readonly
            && state.labelled_by.is_some()
            && state.described_by.is_some()
    );
    assert_eq!(
        state.form_control.as_ref().map(|value| value.name.as_str()),
        Some("accent")
    );
    assert!(nodes(&app, "sapid-popover").is_empty());
    assert!(
        App::memory()
            .mount(
                color_input("bad-popup")
                    .show_picker(false)
                    .default_open(true)
            )
            .is_err()
    );
    app.clear().unwrap();
    assert_eq!(app.stats().nodes, 0);
}
#[test]
fn composition_and_field_readonly_disable_the_live_popup_picker_channels() {
    let app = App::memory();
    app.mount(color_input("compose").default_open(true))
        .unwrap();
    let editor = nodes(&app, "sapid-color-editor")[0];
    assert!(nodes(&app, "sapid-popover-trigger").is_empty());
    assert!(nodes(&app, "sapid-color-input-preview").len() == 1);
    let hue = slider(&app, "Hue");
    assert!(!metadata(&app, hue).disabled);
    app.dispatch(editor, Event::Composition { active: true })
        .unwrap();
    app.flush().unwrap();
    assert!(metadata(&app, hue).disabled);
    app.dispatch(editor, Event::Composition { active: false })
        .unwrap();
    app.flush().unwrap();
    assert!(!metadata(&app, hue).disabled);
    app.clear().unwrap();
    assert_eq!(app.stats().nodes, 0);
}
#[test]
fn editor_focus_proposes_controlled_popup_open_and_readonly_refuses_it() {
    for readonly in [false, true] {
        let app = App::memory();
        let open = app.scope().cage(false);
        let proposals = Rc::new(RefCell::new(Vec::new()));
        let out = proposals.clone();
        app.mount(
            color_input("focus-proposal")
                .open(open)
                .readonly(readonly)
                .on_open_change(move |next| out.borrow_mut().push(next)),
        )
        .unwrap();
        let editor = nodes(&app, "sapid-color-editor")[0];
        app.dispatch(editor, Event::FocusChanged(true)).unwrap();
        assert_eq!(
            *proposals.borrow(),
            if readonly { vec![] } else { vec![true] }
        );
        assert!(!open.get());
        let panel = nodes(&app, "sapid-popover")[0];
        assert!(metadata(&app, panel).hidden);
    }
}

#[test]
fn standalone_picker_formats_a_live_real_input_form_proxy_without_losing_hsva() {
    let app = App::memory();
    let source = app.scope().cage(Hsva {
        hue: 120.123456789,
        saturation: 0.5,
        value: 0.75,
        alpha: 0.4,
    });
    app.mount(
        color_picker("submitted")
            .value(source)
            .form_name("accent")
            .form_format(ColorFormat::Rgba)
            .on_value_change(move |next| source.revise(|value| *value = next)),
    )
    .unwrap();
    let proxy = nodes(&app, "sapid-color-picker-form-proxy")[0];
    assert_eq!(
        metadata(&app, proxy)
            .form_control
            .as_ref()
            .map(|control| (control.name.as_str(), control.value.as_deref())),
        Some(("accent", None))
    );
    assert_eq!(input(&app, proxy).value, "rgba(96, 191, 96, 0.40)");
    key(&app, slider(&app, "Hue"), RangeCommand::Maximum);
    app.flush().unwrap();
    assert_eq!(source.get().hue, 360.0);
    assert_eq!(input(&app, proxy).value, "rgba(191, 96, 96, 0.40)");
}

#[test]
fn color_input_uses_only_the_explicit_eye_dropper_service_and_commits_its_result() {
    let executor = Rc::new(glory::core::executor::Executor::new(8));
    let app = App::with_host(executor.clone(), glory::render::memory::Memory::new);
    let changes = app.scope().cage(0_u32);
    let commits = app.scope().cage(0_u32);
    app.mount(
        color_input("eyedropper")
            .eye_dropper(ColorEyeDropper::new(|| async {
                Ok(Some(glory::style::Color(18, 184, 134, 255)))
            }))
            .eye_dropper_label("Sample a color")
            .eye_dropper_icon(glory::ui::text().content("E"))
            .on_value_change(move |_| changes.revise(|value| *value += 1))
            .on_commit(move |_| commits.revise(|value| *value += 1)),
    )
    .unwrap();
    let button = nodes(&app, "sapid-color-input-eye-dropper")[0];
    assert_eq!(
        app.tree()
            .node(button)
            .unwrap()
            .descriptor
            .view()
            .name
            .as_deref(),
        Some("Sample a color")
    );
    app.dispatch(button, Event::Activate).unwrap();
    assert!(executor.poll_one());
    app.flush().unwrap();
    let editor = nodes(&app, "sapid-color-editor")[0];
    assert_eq!(input(&app, editor).value, "#12b886");
    assert_eq!((changes.get(), commits.get()), (1, 1));
}

#[test]
fn controlled_invalid_blur_proposes_the_restored_value_before_commit() {
    let app = App::memory();
    let source = app.scope().cage("#228be6".to_owned());
    let log = Rc::new(RefCell::new(Vec::new()));
    let changes = log.clone();
    let commits = log.clone();
    app.mount(
        color_input("controlled-correction")
            .value(source)
            .on_value_change(move |v| {
                changes.borrow_mut().push(format!("change:{v}"));
                source.revise(|old| *old = v);
            })
            .on_commit(move |v| commits.borrow_mut().push(format!("commit:{v}"))),
    )
    .unwrap();
    let editor = nodes(&app, "sapid-color-editor")[0];
    edit(&app, editor, "unfinished");
    log.borrow_mut().clear();
    app.dispatch(
        editor,
        Event::Commit {
            value: "unfinished".into(),
            revision: input(&app, editor).revision,
        },
    )
    .unwrap();
    assert_eq!(source.get(), "#228be6");
    assert_eq!(*log.borrow(), ["change:#228be6", "commit:#228be6"]);
    edit(&app, editor, "incomplete after Enter");
    log.borrow_mut().clear();
    app.dispatch(editor, Event::FocusChanged(false)).unwrap();
    assert_eq!(source.get(), "#228be6");
    assert_eq!(*log.borrow(), ["change:#228be6", "commit:#228be6"]);
    app.dispatch(editor, Event::FocusChanged(false)).unwrap();
    assert_eq!(log.borrow().len(), 2);
}

#[test]
fn blur_correction_respects_opt_out_composition_and_current_unavailable_state() {
    for mode in 0..5 {
        let app = App::memory();
        let value = app.scope().cage("#228be6".to_owned());
        let readonly = app.scope().cage(false);
        let enabled = app.scope().cage(true);
        let commits = app.scope().cage(0);
        app.mount(
            color_input("blur-policy")
                .show_picker(false)
                .value(value)
                .fix_on_blur(mode != 1)
                .readonly(readonly)
                .enabled(enabled)
                .on_value_change(move |v| value.revise(|old| *old = v))
                .on_commit(move |_| commits.revise(|n| *n += 1)),
        )
        .unwrap();
        let editor = nodes(&app, "sapid-color-editor")[0];
        edit(&app, editor, "draft");
        app.scope()
            .batch(|| {
                match mode {
                    2 => {
                        app.dispatch(editor, Event::Composition { active: true })
                            .unwrap();
                    }
                    3 => readonly.revise(|v| *v = true),
                    4 => enabled.revise(|v| *v = false),
                    _ => {}
                }
                app.dispatch(editor, Event::FocusChanged(false)).unwrap();
            })
            .unwrap();
        assert_eq!(
            value.get(),
            if mode == 0 { "#228be6" } else { "draft" },
            "mode {mode}"
        );
        assert_eq!(commits.get(), usize::from(mode == 0));
    }
}

#[test]
fn swatches_only_input_accepts_controlled_popup_state_and_proposes_selection_and_close() {
    let app = App::memory();
    let opened = app.scope().cage(true);
    let value = app.scope().cage("#228be6".to_owned());
    let requests = app.scope().cage(Vec::new());
    app.mount(
        color_input("controlled-palette-only")
            .show_picker(false)
            .swatches(["#fa5252", "#12b886"])
            .open(opened)
            .value(value)
            .close_on_color_swatch_click(true)
            .on_value_change(move |v| value.revise(|old| *old = v))
            .on_open_change(move |v| {
                requests.revise(|values| values.push(v));
                opened.revise(|old| *old = v);
            }),
    )
    .unwrap();
    assert!(nodes(&app, "sapid-color-plane").is_empty());
    let swatch = nodes(&app, "sapid-color-swatch")
        .into_iter()
        .find(|id| {
            app.tree()
                .node(*id)
                .unwrap()
                .descriptor
                .view()
                .name
                .as_deref()
                == Some("#fa5252")
        })
        .unwrap();
    app.dispatch(swatch, Event::Activate).unwrap();
    assert_eq!(value.get(), "#fa5252");
    assert_eq!(requests.get(), [false]);
    assert!(!opened.get());
    let preview = nodes(&app, "sapid-color-input-preview")[0];
    app.dispatch(preview, Event::Activate).unwrap();
    assert_eq!(requests.get(), [false, true]);
    assert!(opened.get());
}

#[test]
fn palette_pressed_state_follows_the_accepted_color_and_controlled_close_proposes_once() {
    let app = App::memory();
    let source = app.scope().cage(Hsva::default());
    app.mount(
        color_picker("selection-mark")
            .value(source)
            .swatches(["#fa5252", "#12b886"])
            .on_value_change(move |v| source.revise(|old| *old = v)),
    )
    .unwrap();
    let swatches = nodes(&app, "sapid-color-picker-swatch");
    let first = swatches[0];
    app.dispatch(first, Event::Activate).unwrap();
    assert_eq!(metadata(&app, first).pressed, Some(true));
    assert_eq!(metadata(&app, swatches[1]).pressed, Some(false));
    app.clear().unwrap();
    let opened = app.scope().cage(true);
    let requests = app.scope().cage(Vec::new());
    app.mount(
        color_input("swatch-close")
            .open(opened)
            .swatches(["#fa5252"])
            .close_on_color_swatch_click(true)
            .on_open_change(move |v| {
                requests.revise(|old| old.push(v));
                opened.revise(|old| *old = v);
            }),
    )
    .unwrap();
    let swatch = nodes(&app, "sapid-color-swatch")
        .into_iter()
        .find(|id| {
            app.tree()
                .node(*id)
                .unwrap()
                .descriptor
                .view()
                .name
                .as_deref()
                == Some("#fa5252")
        })
        .unwrap();
    app.dispatch(swatch, Event::Activate).unwrap();
    assert!(!opened.get());
    assert_eq!(requests.get(), vec![false]);
}

#[test]
fn inset_preview_reopens_a_focused_editor_once_and_rechecks_a_replaced_loading_slot() {
    let app = App::memory();
    let opened = app.scope().cage(false);
    let loading = app.scope().cage(false);
    let requests = app.scope().cage(Vec::new());
    app.mount(
        color_input("preview-action")
            .open(opened)
            .loading(loading)
            .loading_position(sapid::ColorInputSectionPosition::Left)
            .on_open_change(move |v| {
                requests.revise(|values| values.push(v));
                opened.revise(|old| *old = v);
            }),
    )
    .unwrap();
    let editor = nodes(&app, "sapid-color-editor")[0];
    let preview = nodes(&app, "sapid-color-input-preview")[0];
    app.dispatch(editor, Event::FocusChanged(true)).unwrap();
    opened.revise(|v| *v = false);
    app.flush().unwrap();
    requests.revise(Vec::clear);
    app.dispatch(preview, Event::Activate).unwrap();
    app.dispatch(preview, Event::Activate).unwrap();
    assert_eq!(requests.get(), vec![true]);
    opened.revise(|v| *v = false);
    app.flush().unwrap();
    requests.revise(Vec::clear);
    app.scope()
        .batch(|| {
            loading.revise(|v| *v = true);
            app.dispatch(preview, Event::Activate).unwrap();
        })
        .unwrap();
    assert!(requests.get().is_empty());
    assert!(!opened.get());
}

#[test]
fn color_input_picker_keeps_precise_channels_when_the_formatted_color_is_accepted() {
    for format in [
        ColorFormat::Hex,
        ColorFormat::Hexa,
        ColorFormat::Rgb,
        ColorFormat::Rgba,
        ColorFormat::Hsl,
        ColorFormat::Hsla,
    ] {
        for controlled in [false, true] {
            let app = App::memory();
            let initial = sapid::format_color(Color(34, 139, 230, 179), format);
            let source = app.scope().cage(initial.clone());
            let mut widget = color_input("precise-channels")
                .format(format)
                .default_open(true);
            widget = if controlled {
                widget
                    .value(source)
                    .on_value_change(move |next| source.revise(|old| *old = next))
            } else {
                widget.default_value(initial)
            };
            app.mount(widget).unwrap();
            let hue = slider(&app, "Hue");
            for value in [183., 199., 306., 360.] {
                key(&app, hue, RangeCommand::Set(value));
                app.flush().unwrap();
                assert_eq!(
                    number(&app, hue),
                    value,
                    "{format:?}, controlled={controlled}"
                );
            }
            if format.alpha() {
                let alpha = slider(&app, "Opacity");
                for value in [0.25, 0.4, 0.65, 0.85] {
                    key(&app, alpha, RangeCommand::Set(value));
                    app.flush().unwrap();
                    assert_eq!(
                        number(&app, alpha),
                        value,
                        "{format:?}, controlled={controlled}"
                    );
                }
            }
        }
    }
}

#[test]
fn color_input_picker_refusal_and_external_colors_override_proposed_coordinates() {
    let app = App::memory();
    let source = app.scope().cage("#228be6b3".to_owned());
    app.mount(
        color_input("refused-channels")
            .format(ColorFormat::Hexa)
            .value(source)
            .default_open(true),
    )
    .unwrap();
    let hue = slider(&app, "Hue");
    let initial = number(&app, hue);
    key(&app, hue, RangeCommand::Set(183.));
    app.flush().unwrap();
    assert_eq!(number(&app, hue), initial);
    assert_eq!(source.get(), "#228be6b3");
    source.revise(|value| *value = "#00ff0080".into());
    app.flush().unwrap();
    assert_eq!(number(&app, hue), 120.);
    assert_eq!(number(&app, slider(&app, "Opacity")), 128. / 255.);
}
