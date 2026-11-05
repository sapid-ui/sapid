use glory::{
    prelude::*,
    render::{NodeId, Props},
    style::{Color, Length},
};
fn find(app: &App, class: &str) -> NodeId {
    let tree = app.tree();
    let mut nodes = tree.roots().to_vec();
    while let Some(id) = nodes.pop() {
        let node = tree.node(id).unwrap();
        if node.descriptor.view().classes.iter().any(|v| v == class) {
            return id;
        }
        nodes.extend(&node.children);
    }
    panic!("missing {class}")
}

#[test]
fn rolling_number_font_changes_retain_reels_and_match_their_translation_and_line_boxes() {
    let app = App::memory();
    let style = app.scope().cage(Style {
        font_size: Some(38.),
        ..Style::new()
    });
    app.mount(sapid::rolling_number().value(19.5).style(style))
        .unwrap();
    let reel = find(&app, "sapid-rolling-number-reel");
    let digit = find(&app, "sapid-rolling-number-digit");
    let literal = find(&app, "sapid-rolling-number-literal");
    for (font, line) in [(38., 58.9), (24., 37.2)] {
        style.revise(|s| s.font_size = Some(font));
        app.flush().unwrap();
        assert_eq!(find(&app, "sapid-rolling-number-reel"), reel);
        let tree = app.tree();
        assert!(
            (match tree.node(digit).unwrap().descriptor.view().style.height {
                Some(Length::Px(value)) => value,
                _ => panic!(),
            } - line)
                .abs()
                < 0.001
        );
        assert!(
            (tree
                .node(literal)
                .unwrap()
                .descriptor
                .view()
                .style
                .line_height
                .unwrap()
                - line)
                .abs()
                < 0.001
        );
        let motion = tree
            .node(reel)
            .unwrap()
            .descriptor
            .view()
            .semantics
            .as_ref()
            .unwrap()
            .translation
            .unwrap();
        let glory::render::TranslationKind::To { target, .. } = motion.kind else {
            panic!()
        };
        assert!((target.y / -line - 5.).abs() < 0.001);
    }
    style.revise(|s| s.line_height = Some(72.));
    app.flush().unwrap();
    assert_eq!(
        app.tree()
            .node(literal)
            .unwrap()
            .descriptor
            .view()
            .style
            .line_height,
        Some(72.)
    );
    app.clear().unwrap();
    assert_eq!(app.stats().nodes, 0);
}

#[test]
fn rolling_font_override_scales_the_selected_theme_metrics_and_unicode_badge_lines_are_rejected() {
    let mut theme = sapid::Theme::blue_light();
    theme.tokens = theme
        .tokens
        .clone()
        .with_typography(
            sapid::TypographyRole::Label,
            sapid::Size::Medium,
            sapid::TypographyMetrics {
                font_size: 20.,
                line_height: 40.,
                weight: Some(400.),
            },
        )
        .unwrap();
    let app = App::memory();
    app.mount(sapid::rolling_number().theme(theme).value(1.).style(Style {
        font_size: Some(30.),
        ..Style::new()
    }))
    .unwrap();
    let digit = find(&app, "sapid-rolling-number-digit");
    assert_eq!(
        app.tree()
            .node(digit)
            .unwrap()
            .descriptor
            .view()
            .style
            .height,
        Some(Length::Px(60.))
    );
    for separator in ['\u{2028}', '\u{2029}'] {
        let app = App::memory();
        assert!(
            app.mount(
                sapid::badge()
                    .content("Ready")
                    .left_section(format!("a{separator}b"))
            )
            .is_err()
        );
        assert!(app.tree().is_empty());
    }
}
fn text(app: &App, id: NodeId) -> String {
    let tree = app.tree();
    let Props::Text(props) = tree.node(id).unwrap().descriptor.view().props else {
        panic!("text")
    };
    props.content.clone()
}
#[test]
fn rich_badge_has_bounded_passive_sections_retained_text_and_matching_icon_color() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let app = App::memory();
        let content = app.scope().cage("Ready".to_owned());
        let style = app.scope().cage(Style::new());
        app.mount(
            sapid::badge()
                .theme(theme)
                .content(content)
                .left_section(sapid::Icon::Check)
                .right_section("new")
                .color(Color(255, 225, 65, 255))
                .variant(sapid::BadgeVariant::Solid)
                .style(style),
        )
        .unwrap();
        let root = find(&app, "sapid-badge");
        let label = find(&app, "sapid-badge-content");
        let left = find(&app, "sapid-badge-left");
        let graphic = app.tree().node(left).unwrap().children[0];
        let tree = app.tree();
        let d = tree.node(root).unwrap().descriptor.view();
        assert_eq!(d.role, None);
        assert_eq!(d.style.color, Some(Color(0, 0, 0, 255)));
        assert!(
            matches!(tree.node(graphic).unwrap().descriptor.view().props,Props::Graphic(p) if p.stroke.as_ref().unwrap().color==Color(0,0,0,255))
        );
        drop(tree);
        content.revise(|v| *v = "Updated".to_owned());
        style.revise(|v| v.color = Some(Color(80, 20, 120, 255)));
        app.flush().unwrap();
        assert_eq!(find(&app, "sapid-badge"), root);
        assert_eq!(find(&app, "sapid-badge-content"), label);
        assert_eq!(text(&app, label), "Updated");
        assert_eq!(app.tree().node(left).unwrap().children[0], graphic);
        assert!(
            matches!(app.tree().node(graphic).unwrap().descriptor.view().props,Props::Graphic(p) if p.stroke.as_ref().unwrap().color==Color(80,20,120,255))
        );
        assert_eq!(app.stats().handlers, 0);
        app.clear().unwrap();
        assert_eq!(app.stats().nodes, 0);
    }
}
#[test]
fn badge_dot_and_section_validation_preserve_the_ordinary_button_content_contract() {
    let app = App::memory();
    app.mount(glory::ui::button().fill(sapid::badge().content("Ready")))
        .unwrap();
    app.clear().unwrap();
    let app = App::memory();
    app.mount(
        sapid::badge()
            .variant(sapid::BadgeVariant::Dot)
            .color(Color(200, 60, 40, 255))
            .content("Running"),
    )
    .unwrap();
    let marker = find(&app, "sapid-badge-dot-marker");
    assert!(
        matches!(app.tree().node(marker).unwrap().descriptor.view().props,Props::Graphic(p) if p.fill==Some(Color(200,60,40,255))&&p.stroke.is_none())
    );
    for invalid in [String::new(), "x".repeat(33), "line\nbreak".to_owned()] {
        let app = App::memory();
        assert!(app.mount(sapid::badge().left_section(invalid)).is_err());
        assert_eq!(app.stats().nodes, 0);
    }
    let app = App::memory();
    assert!(
        app.mount(
            glory::ui::button().fill(
                sapid::badge()
                    .variant(sapid::BadgeVariant::Dot)
                    .content("Status")
            )
        )
        .is_err()
    );
    assert_eq!(app.stats().nodes, 0);
}
#[test]
fn literal_numeric_affixes_preserve_identity_reject_invalid_updates_and_do_not_write_inputs() {
    let app = App::memory();
    let value = app.scope().cage(-1234.5);
    let prefix = app.scope().cage("Balance: ".to_owned());
    let suffix = app.scope().cage(" credits".to_owned());
    app.mount(
        sapid::number_formatter()
            .value(value)
            .prefix(prefix)
            .suffix(suffix),
    )
    .unwrap();
    let root = app.tree().roots()[0];
    assert_eq!(text(&app, root), "Balance: -1,234.5 credits");
    prefix.revise(|v| *v = "Available: ".to_owned());
    suffix.revise(|v| *v = " USD".to_owned());
    let writes = app.runtime_stats().source_writes;
    app.flush().unwrap();
    assert_eq!(app.runtime_stats().source_writes, writes);
    assert_eq!(text(&app, root), "Available: -1,234.5 USD");
    prefix.revise(|v| *v = "invalid\n".to_owned());
    assert!(app.flush().is_err());
    assert_eq!(text(&app, root), "Available: -1,234.5 USD");
    assert_eq!(prefix.get(), "invalid\n");
    prefix.revise(|v| v.clear());
    value.revise(|v| *v = 2500.);
    app.flush().unwrap();
    assert_eq!(text(&app, root), "2,500 USD");
    assert_eq!(app.tree().roots()[0], root);
    assert_eq!(app.stats().handlers, 0);
    for invalid in ["x".repeat(257), "paragraph\u{2028}break".to_owned()] {
        let app = App::memory();
        assert!(
            app.mount(sapid::number_formatter().suffix(invalid))
                .is_err()
        );
        assert_eq!(app.stats().nodes, 0);
    }
}
#[test]
fn icon_extent_updates_keep_the_graphic_and_recover_without_rewriting_the_source() {
    let app = App::memory();
    let size = app.scope().cage(18.);
    app.mount(sapid::theme_icon(sapid::Icon::Check).icon_size(size))
        .unwrap();
    let graphic = find(&app, "sapid-theme-icon-graphic");
    size.revise(|v| *v = 28.);
    app.flush().unwrap();
    assert_eq!(
        app.tree()
            .node(graphic)
            .unwrap()
            .descriptor
            .view()
            .style
            .width,
        Some(Length::Px(28.))
    );
    size.revise(|v| *v = f64::NAN);
    assert!(app.flush().is_err());
    assert!(size.get().is_nan());
    assert_eq!(
        app.tree()
            .node(graphic)
            .unwrap()
            .descriptor
            .view()
            .style
            .width,
        Some(Length::Px(28.))
    );
    size.revise(|v| *v = 12.);
    app.flush().unwrap();
    assert_eq!(find(&app, "sapid-theme-icon-graphic"), graphic);
    assert_eq!(
        app.tree()
            .node(graphic)
            .unwrap()
            .descriptor
            .view()
            .style
            .width,
        Some(Length::Px(12.))
    );
    for invalid in [0., 129., f64::INFINITY] {
        let app = App::memory();
        assert!(
            app.mount(sapid::theme_icon(sapid::Icon::Info).icon_size(invalid))
                .is_err()
        );
        assert_eq!(app.stats().nodes, 0);
    }
}
#[test]
fn description_layout_keeps_intrinsic_pairs_and_rejects_invalid_geometry_before_nodes() {
    for orientation in [
        sapid::DataListOrientation::Horizontal,
        sapid::DataListOrientation::Vertical,
    ] {
        let app = App::memory();
        app.mount(
            sapid::data_list()
                .orientation(orientation)
                .label_width(96.)
                .gap(8.)
                .item(
                    sapid::data_list_item()
                        .term("Project")
                        .fill(glory::ui::text().content("Workspace")),
                ),
        )
        .unwrap();
        assert_eq!(
            app.tree()
                .node(find(&app, "sapid-data-list"))
                .unwrap()
                .descriptor
                .view()
                .document_part(),
            Some(glory::render::DocumentPart::DescriptionList)
        );
        assert_eq!(app.stats().handlers, 0);
    }
    for (width, gap) in [
        (0., 8.),
        (513., 8.),
        (f64::NAN, 8.),
        (96., -1.),
        (96., 129.),
    ] {
        let app = App::memory();
        assert!(
            app.mount(sapid::data_list().label_width(width).gap(gap))
                .is_err()
        );
        assert_eq!(app.stats().nodes, 0);
    }
}
