use glory::render::{NodeId, PathCommand, Props, Role, StrokeCap};
use glory::style::{Color, Length, Style};
use glory::ui::App;
use sapid::{
    ProgressOrientation, ProgressSection, SemiCircleFillDirection, SemiCircleLabelPosition,
    SemiCircleOrientation, Theme, progress, ring_progress, semi_circle_progress,
};
fn find(app: &App, class: &str) -> NodeId {
    let tree = app.tree();
    let mut pending = tree.roots().to_vec();
    while let Some(id) = pending.pop() {
        let node = tree.node(id).unwrap();
        if node
            .descriptor
            .view()
            .classes
            .iter()
            .any(|value| value == class)
        {
            return id;
        }
        pending.extend(&node.children);
    }
    panic!("missing {class}")
}
fn find_all(app: &App, class: &str) -> Vec<NodeId> {
    let tree = app.tree();
    let mut pending = tree.roots().to_vec();
    let mut found = Vec::new();
    while let Some(id) = pending.pop() {
        let node = tree.node(id).unwrap();
        if node
            .descriptor
            .view()
            .classes
            .iter()
            .any(|value| value == class)
        {
            found.push(id);
        }
        pending.extend(&node.children);
    }
    found
}
#[test]
fn indeterminate_motion_preserves_graphics_and_stops_on_known_values_or_pause() {
    for kind in 0..3 {
        let app = App::memory();
        let unknown = app.scope().cage(true);
        let animated = app.scope().cage(true);
        let widget = match kind {
            0 => progress(40.),
            1 => ring_progress(40.),
            _ => semi_circle_progress(40.),
        };
        app.mount(widget.indeterminate(unknown).animated(animated))
            .unwrap();
        let root = app.tree().roots()[0];
        let graphic = find(
            &app,
            if kind == 0 {
                "sapid-progress-activity"
            } else {
                "sapid-progress-arc"
            },
        );
        for (motion, indeterminate, expected) in [
            (true, true, true),
            (false, true, false),
            (true, false, false),
            (true, true, true),
        ] {
            animated.revise(|v| *v = motion);
            unknown.revise(|v| *v = indeterminate);
            app.flush().unwrap();
            assert_eq!(
                find(
                    &app,
                    if kind == 0 {
                        "sapid-progress-activity"
                    } else {
                        "sapid-progress-arc"
                    }
                ),
                graphic
            );
            assert!(
                matches!(app.tree().node(graphic).unwrap().descriptor.view().props,Props::Graphic(p) if p.animation.is_some()==expected)
            );
            assert_eq!(
                app.tree()
                    .node(root)
                    .unwrap()
                    .descriptor
                    .view()
                    .semantics
                    .as_ref()
                    .unwrap()
                    .progress
                    .as_ref()
                    .unwrap()
                    .current
                    .is_none(),
                indeterminate
            );
        }
        assert_eq!(app.runtime_stats().tasks, 0);
    }
}
#[test]
fn section_labels_reject_unknown_duplicate_and_unbounded_text_before_mounting() {
    let sections = [
        ProgressSection::new(20., Color(255, 255, 0, 255)),
        ProgressSection::new(30., Color(0, 0, 20, 255)),
    ];
    for widget in [
        progress(50.).sections(sections).section_label(2, "Unknown"),
        progress(50.)
            .sections(sections)
            .section_label(0, "One")
            .section_label(0, "Two"),
        progress(50.)
            .sections(sections)
            .section_label(0, "x".repeat(257)),
        progress(50.)
            .sections(sections)
            .section_label(0, "Line\nbreak"),
        progress(50.).sections(sections).section_label(0, " "),
        progress(50.)
            .vertical(true)
            .sections(sections)
            .section_label(0, "Vertical"),
        ring_progress(50.)
            .sections(sections)
            .section_label(0, "Ring"),
    ] {
        let app = App::memory();
        assert!(app.mount(widget).is_err());
        assert_eq!(app.stats().nodes, 0);
    }
    let app = App::memory();
    app.mount(
        progress(50.)
            .thickness(24.)
            .sections(sections)
            .section_label(0, "Documents")
            .section_label(1, "Photos"),
    )
    .unwrap();
    let labels = find_all(&app, "sapid-progress-section-label");
    assert_eq!(labels.len(), 2);
    assert!(labels.iter().all(|id| matches!(
        app.tree().node(*id).unwrap().descriptor.view().props,
        Props::Text(_)
    )));
    let consumed = std::rc::Rc::new(std::cell::Cell::new(0));
    let count = consumed.clone();
    let widget = progress(50.)
        .sections(std::iter::repeat(sections[0]).inspect(move |_| count.set(count.get() + 1)));
    assert_eq!(consumed.get(), 17);
    let app = App::memory();
    assert!(app.mount(widget).is_err());
    assert_eq!(app.stats().nodes, 0);
}
#[test]
fn linear_progress_has_finite_semantics_actual_fraction_and_no_range_actions() {
    let app = App::memory();
    app.mount(
        progress(30.)
            .name("Upload")
            .range(10., 50.)
            .value_text("Half uploaded"),
    )
    .unwrap();
    let tree = app.tree();
    let root = tree.node(tree.roots()[0]).unwrap().descriptor.view();
    assert_eq!(root.role, Some(Role::ProgressBar));
    let value = root.semantics.as_ref().unwrap().progress.as_ref().unwrap();
    assert_eq!(value.fraction().unwrap(), Some(0.5));
    assert_eq!(value.value_text.as_deref(), Some("Half uploaded"));
    drop(tree);
    let fill = find(&app, "sapid-progress-fill");
    assert_eq!(
        app.tree().node(fill).unwrap().descriptor.view().style.width,
        Some(Length::Percent(50.))
    );
    assert_eq!(app.stats().handlers, 0);
    let html =
        glory::ssr::render(progress(30.).range(10., 50.).value_text("Half uploaded")).unwrap();
    assert!(
        html.contains("role=\"progressbar\"")
            && html.contains("aria-valuemin=\"10\"")
            && html.contains("aria-valuemax=\"50\"")
            && html.contains("aria-valuenow=\"30\"")
            && html.contains("aria-valuetext=\"Half uploaded\"")
    );
    assert!(!html.contains("tabindex"));
}
#[test]
fn circular_progress_uses_bounded_real_paths_for_all_themes_and_endpoints() {
    for theme in [
        Theme::blue_light(),
        Theme::blue_dark(),
        Theme::green_light(),
        Theme::green_dark(),
    ] {
        for semi in [false, true] {
            for value in [0., 25., 50., 100.] {
                let app = App::memory();
                let widget = if semi {
                    semi_circle_progress(value)
                } else {
                    ring_progress(value)
                };
                app.mount(widget.theme(theme.clone()).size(120.).thickness(12.))
                    .unwrap();
                let arc = find(&app, "sapid-progress-arc");
                let tree = app.tree();
                let Props::Graphic(props) = tree.node(arc).unwrap().descriptor.view().props else {
                    panic!()
                };
                props.validate().unwrap();
                assert!(props.commands.len() <= 6);
                assert_eq!(props.commands.is_empty(), value == 0.);
                assert!(props.alternative.is_none());
                assert_eq!(props.stroke.unwrap().color, theme.accent);
                assert_eq!(props.height, if semi { 66. } else { 120. });
            }
        }
    }
}

#[test]
fn semicircle_orientation_fill_direction_colors_and_labels_are_real_geometry() {
    let filled = Color(240, 62, 62, 255);
    let empty = Color(64, 192, 87, 255);
    for (orientation, direction, expected_start) in [
        (
            SemiCircleOrientation::Up,
            SemiCircleFillDirection::LeftToRight,
            (6., 60.),
        ),
        (
            SemiCircleOrientation::Up,
            SemiCircleFillDirection::RightToLeft,
            (114., 60.),
        ),
        (
            SemiCircleOrientation::Down,
            SemiCircleFillDirection::LeftToRight,
            (6., 6.),
        ),
        (
            SemiCircleOrientation::Down,
            SemiCircleFillDirection::RightToLeft,
            (114., 6.),
        ),
    ] {
        let app = App::memory();
        app.mount(
            semi_circle_progress(50.)
                .size(120.)
                .thickness(12.)
                .semi_orientation(orientation)
                .fill_direction(direction)
                .filled_segment_color(filled)
                .empty_segment_color(empty)
                .label("50%")
                .label_position(SemiCircleLabelPosition::Center),
        )
        .unwrap();
        let tree = app.tree();
        let Props::Graphic(arc) = tree
            .node(find(&app, "sapid-progress-arc"))
            .unwrap()
            .descriptor
            .view()
            .props
        else {
            panic!()
        };
        let PathCommand::Move(start) = arc.commands[0] else {
            panic!()
        };
        assert!((start.x - expected_start.0).abs() < 0.001);
        assert!((start.y - expected_start.1).abs() < 0.001);
        assert_eq!(arc.stroke.unwrap().color, filled);
        let Props::Graphic(track) = tree
            .node(find(&app, "sapid-progress-track"))
            .unwrap()
            .descriptor
            .view()
            .props
        else {
            panic!()
        };
        assert_eq!(track.stroke.unwrap().color, empty);
        let label = tree
            .node(find(&app, "sapid-progress-label"))
            .unwrap()
            .descriptor
            .view();
        assert_eq!(label.style.top, Some(Length::Px(0.)));
        assert_eq!(label.style.height, Some(Length::Percent(100.)));
    }
    let html = glory::ssr::render(
        semi_circle_progress(50.)
            .label("Half complete")
            .label_position(SemiCircleLabelPosition::Bottom),
    )
    .unwrap();
    assert!(html.contains("sapid-progress-label") && html.contains("Half complete"));
}
#[test]
fn compound_line_and_ring_sections_keep_order_colors_and_one_progress_semantic() {
    let sections = [
        ProgressSection::new(20., Color(240, 62, 62, 255)),
        ProgressSection::new(30., Color(34, 139, 230, 255)),
        ProgressSection::new(10., Color(64, 192, 87, 255)),
    ];
    for ring in [false, true] {
        let app = App::memory();
        let widget = if ring {
            ring_progress(60.).sections(sections)
        } else {
            progress(60.).sections(sections)
        };
        app.mount(widget).unwrap();
        let parts = find_all(&app, "sapid-progress-section");
        assert_eq!(parts.len(), 3);
        let tree = app.tree();
        let semantic = tree
            .node(tree.roots()[0])
            .unwrap()
            .descriptor
            .view()
            .semantics
            .as_ref()
            .unwrap()
            .progress
            .as_ref()
            .unwrap();
        assert_eq!(semantic.current, Some(60.));
        if ring {
            for id in parts {
                let Props::Graphic(graphic) = tree.node(id).unwrap().descriptor.view().props else {
                    panic!()
                };
                graphic.validate().unwrap();
                assert!(!graphic.commands.is_empty());
            }
        }
    }
}

#[test]
fn ring_start_angle_gap_caps_root_color_and_center_label_are_finite() {
    let root_color = Color(134, 142, 150, 255);
    let app = App::memory();
    app.mount(
        ring_progress(60.)
            .size(120.)
            .thickness(12.)
            .sections([
                ProgressSection::new(20., Color(240, 62, 62, 255)),
                ProgressSection::new(30., Color(34, 139, 230, 255)),
                ProgressSection::new(10., Color(64, 192, 87, 255)),
            ])
            .start_angle(0.)
            .section_gap(18.)
            .round_caps(true)
            .root_color(root_color)
            .label("60%"),
    )
    .unwrap();
    let parts = find_all(&app, "sapid-progress-section");
    assert_eq!(parts.len(), 3);
    let tree = app.tree();
    let mut caps = Vec::new();
    let mut starts = Vec::new();
    for id in parts {
        let Props::Graphic(graphic) = tree.node(id).unwrap().descriptor.view().props else {
            panic!()
        };
        caps.push(graphic.stroke.unwrap().cap);
        let PathCommand::Move(start) = graphic.commands[0] else {
            panic!()
        };
        starts.push(start);
    }
    assert_eq!(
        caps.iter().filter(|cap| **cap == StrokeCap::Round).count(),
        2
    );
    assert_eq!(
        caps.iter().filter(|cap| **cap == StrokeCap::Butt).count(),
        1
    );
    assert!(starts.iter().any(|point| (point.x - 114.).abs() < 0.001));
    let Props::Graphic(track) = tree
        .node(find(&app, "sapid-progress-track"))
        .unwrap()
        .descriptor
        .view()
        .props
    else {
        panic!()
    };
    assert_eq!(track.stroke.unwrap().color, root_color);
    assert_eq!(track.stroke.unwrap().cap, StrokeCap::Butt);
    let label = tree
        .node(find(&app, "sapid-progress-label"))
        .unwrap()
        .descriptor
        .view();
    assert_eq!(label.style.height, Some(Length::Percent(100.)));
}

#[test]
fn vertical_line_grows_from_bottom_and_compound_sections_preserve_order() {
    let app = App::memory();
    app.mount(
        progress(60.)
            .orientation(ProgressOrientation::Vertical)
            .length(Length::Px(240.))
            .thickness(12.)
            .sections([
                ProgressSection::new(20., Color(240, 62, 62, 255)),
                ProgressSection::new(30., Color(34, 139, 230, 255)),
                ProgressSection::new(10., Color(64, 192, 87, 255)),
            ]),
    )
    .unwrap();
    let root = app.tree().roots()[0];
    let root_view = app.tree().node(root).unwrap().descriptor.view().to_owned();
    assert!(
        root_view
            .classes
            .iter()
            .any(|class| class == "sapid-progress-vertical")
    );
    assert_eq!(root_view.style.width, Some(Length::Px(12.)));
    assert_eq!(root_view.style.height, Some(Length::Px(240.)));
    let sections = app.tree().node(root).unwrap().children.clone();
    assert_eq!(sections.len(), 3);
    let tree = app.tree();
    for (id, bottom, height) in sections
        .into_iter()
        .zip([0., 20., 50.])
        .zip([20., 30., 10.])
        .map(|((id, bottom), height)| (id, bottom, height))
    {
        let style = &tree.node(id).unwrap().descriptor.view().style;
        assert_eq!(style.bottom, Some(Length::Percent(bottom)));
        assert_eq!(style.height, Some(Length::Percent(height)));
        assert_eq!(style.width, Some(Length::Percent(100.)));
    }
}

#[test]
fn vertical_indeterminate_uses_the_main_axis_and_circular_views_reject_it() {
    let app = App::memory();
    app.mount(
        progress(40.)
            .vertical(true)
            .height(Length::Px(180.))
            .indeterminate(true),
    )
    .unwrap();
    let fill = app
        .tree()
        .node(find(&app, "sapid-progress-fill"))
        .unwrap()
        .descriptor
        .view()
        .to_owned();
    assert_eq!(fill.style.bottom, Some(Length::Percent(35.)));
    assert_eq!(fill.style.height, Some(Length::Percent(30.)));
    assert_eq!(fill.style.width, Some(Length::Percent(100.)));

    for widget in [
        ring_progress(40.).vertical(true),
        semi_circle_progress(40.).vertical(true),
    ] {
        let app = App::memory();
        assert!(app.mount(widget).is_err());
        assert!(app.tree().is_empty());
    }
}
#[test]
fn controlled_value_indeterminate_and_text_reject_invalid_input_and_recover() {
    let app = App::memory();
    let value = app.scope().cage(25.);
    let unknown = app.scope().cage(false);
    let text = app.scope().cage("Quarter".to_string());
    app.mount(progress(value).indeterminate(unknown).value_text(text))
        .unwrap();
    let root = app.tree().roots()[0];
    let fill = find(&app, "sapid-progress-fill");
    value.revise(|value| *value = 101.);
    text.revise(|value| *value = " ".into());
    assert!(app.flush().is_err());
    assert_eq!(value.get(), 101.);
    assert_eq!(
        app.tree().node(fill).unwrap().descriptor.view().style.width,
        Some(Length::Percent(25.))
    );
    value.revise(|value| *value = 50.);
    text.revise(|value| *value = "Waiting".into());
    unknown.revise(|value| *value = true);
    app.flush().unwrap();
    assert!(
        app.tree()
            .node(root)
            .unwrap()
            .descriptor
            .view()
            .semantics
            .as_ref()
            .unwrap()
            .progress
            .as_ref()
            .unwrap()
            .current
            .is_none()
    );
    assert_eq!(
        app.tree().node(fill).unwrap().descriptor.view().style.width,
        Some(Length::Percent(30.))
    );
    assert_eq!(find(&app, "sapid-progress-fill"), fill);
    unknown.revise(|value| *value = false);
    app.flush().unwrap();
    assert_eq!(
        app.tree().node(fill).unwrap().descriptor.view().style.width,
        Some(Length::Percent(50.))
    );
    assert_eq!(app.stats().handlers, 0);
}
#[test]
fn invalid_initial_ranges_geometry_and_strings_create_no_visible_nodes() {
    for widget in [
        progress(f64::NAN),
        progress(f64::INFINITY),
        progress(-1.),
        progress(101.),
        progress(20.).range(50., 10.),
        progress(20.).range(0., f64::MAX).size(f64::NAN),
        ring_progress(20.).size(15.),
        ring_progress(20.).thickness(51.),
        semi_circle_progress(20.).thickness(0.),
        semi_circle_progress(20.).label(" "),
        semi_circle_progress(20.).label("x".repeat(1025)),
        progress(20.).label("not a semicircle"),
        ring_progress(20.).fill_direction(SemiCircleFillDirection::RightToLeft),
        ring_progress(20.).start_angle(f64::NAN),
        ring_progress(20.).section_gap(-1.),
        progress(20.).root_color(Color(1, 2, 3, 255)),
        semi_circle_progress(20.).round_caps(true),
        progress(20.).name(" "),
        progress(20.).value_text("x".repeat(1025)),
        progress(50.).sections([ProgressSection::new(20., Color(1, 2, 3, 255))]),
        progress(50.).sections([ProgressSection::new(0., Color(1, 2, 3, 255))]),
        semi_circle_progress(50.).sections([ProgressSection::new(50., Color(1, 2, 3, 255))]),
    ] {
        let app = App::memory();
        assert!(app.mount(widget).is_err());
        assert!(app.tree().roots().is_empty());
    }
}
#[test]
fn unstyled_keeps_progress_content_and_explicit_overrides() {
    let app = App::memory();
    app.mount(
        progress(50.)
            .theme(Theme::green_dark())
            .unstyled()
            .class("custom-progress")
            .style(Style::new().height(Length::Px(10.))),
    )
    .unwrap();
    let root = app.tree().roots()[0];
    assert_eq!(
        app.tree()
            .node(root)
            .unwrap()
            .descriptor
            .view()
            .style
            .height,
        Some(Length::Px(10.))
    );
    assert!(
        app.tree()
            .node(root)
            .unwrap()
            .descriptor
            .view()
            .style
            .background
            .is_none()
    );
    let fill = find(&app, "sapid-progress-activity");
    assert!(
        matches!(app.tree().node(fill).unwrap().descriptor.view().props,Props::Graphic(p) if p.fill==Some(Theme::green_dark().foreground))
    );
    let app = App::memory();
    app.mount(ring_progress(50.).unstyled()).unwrap();
    let arc = find(&app, "sapid-progress-arc");
    let tree = app.tree();
    let Props::Graphic(value) = tree.node(arc).unwrap().descriptor.view().props else {
        panic!()
    };
    assert_ne!(value.stroke.unwrap().color, Color(0, 0, 0, 0));
    assert!(tree.node(tree.roots()[0]).unwrap().children.len() == 1);
}
