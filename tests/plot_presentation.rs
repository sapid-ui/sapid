#![cfg(feature = "charts")]
use glory::{
    prelude::*,
    render::{Event, PathCommand, Props},
};
use sapid::charts::*;
fn values() -> PlotData {
    PlotData::values([
        ValueDatum::new("a", "First", 1.),
        ValueDatum::new("b", "Second", 3.),
    ])
}
fn scene(
    data: &PlotData,
    kind: PlotKind,
    options: PlotOptions,
) -> std::result::Result<PlotScene, ChartError> {
    PlotScene::build(
        data,
        kind,
        ChartFrame::default(),
        &options,
        &ChartTheme::from_theme(&sapid::Theme::blue_light()),
    )
}
#[test]
fn default_configuration_preserves_geometry_and_explicit_arcs_change_exact_point_positions() {
    for kind in [PlotKind::Pie, PlotKind::Donut, PlotKind::RadialBar] {
        let original = scene(&values(), kind, PlotOptions::default()).unwrap();
        assert_eq!(
            original,
            scene(
                &values(),
                kind,
                PlotOptions {
                    angles: Some(PlotAngles::default()),
                    ..Default::default()
                }
            )
            .unwrap()
        );
    }
    let options = PlotOptions {
        angles: Some(PlotAngles {
            start: 0.,
            sweep: 180.,
        }),
        donut_inner_ratio: Some(0.5),
        center_label: Some("Completion".into()),
        ..Default::default()
    };
    let result = scene(&values(), PlotKind::Donut, options).unwrap();
    let point = &result.marks[0];
    let mid = (140. + 70.) / 2.;
    let angle = std::f64::consts::PI / 8.;
    assert!((point.x - (320. + mid * angle.cos())).abs() < 1e-8);
    assert!((point.y - (160. + mid * angle.sin())).abs() < 1e-8);
    assert_eq!(result.labels[0].text, "Completion");
    let PathCommand::Move(outer) = result.paths[0].drawing.commands[0] else {
        panic!()
    };
    assert!((outer.x - 460.).abs() < 1e-8);
    assert!((outer.y - 160.).abs() < 1e-8);
    assert!(result.marks[0].value_text.contains("25%"));
}
#[test]
fn twenty_cell_waffle_retains_exact_total_and_deterministic_remainders() {
    let data = PlotData::values([
        ValueDatum::new("a", "A", 1.),
        ValueDatum::new("b", "B", 1.),
        ValueDatum::new("c", "C", 1.),
    ]);
    let options = PlotOptions {
        waffle_grid: Some(WaffleGrid {
            rows: 4,
            columns: 5,
            gap: 3.,
        }),
        ..Default::default()
    };
    let result = scene(&data, PlotKind::Waffle, options).unwrap();
    assert_eq!(
        result
            .paths
            .iter()
            .flat_map(|p| &p.drawing.commands)
            .filter(|c| matches!(c, PathCommand::Move(_)))
            .count(),
        20
    );
    assert!(result.marks[0].value_text.contains("7 of 20"));
    assert!(result.marks[1].value_text.contains("7 of 20"));
    assert!(result.marks[2].value_text.contains("6 of 20"));
    let maximum = scene(
        &data,
        PlotKind::Waffle,
        PlotOptions {
            waffle_grid: Some(WaffleGrid {
                rows: 20,
                columns: 20,
                gap: 1.,
            }),
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(
        maximum
            .paths
            .iter()
            .flat_map(|p| &p.drawing.commands)
            .filter(|c| matches!(c, PathCommand::Move(_)))
            .count(),
        400
    );
    assert!(
        maximum
            .paths
            .iter()
            .all(|p| p.drawing.commands.len() <= 256)
    );
}
#[test]
fn bounded_bubble_radii_keep_position_and_exact_size_independent() {
    let data = PlotData::Points(vec![
        PointDatum {
            key: "a".into(),
            label: "A".into(),
            x: 2.,
            y: 4.,
            size: 1.,
        },
        PointDatum {
            key: "b".into(),
            label: "B".into(),
            x: 3.,
            y: 8.,
            size: 4.,
        },
    ]);
    let original = scene(&data, PlotKind::Bubble, PlotOptions::default()).unwrap();
    let result = scene(
        &data,
        PlotKind::Bubble,
        PlotOptions {
            bubble_radius: Some(BubbleRadius { min: 1., max: 10. }),
            ..Default::default()
        },
    )
    .unwrap();
    for (i, radius) in [5.5, 10.].into_iter().enumerate() {
        assert_eq!(result.marks[i], original.marks[i]);
        let PathCommand::Move(start) = result
            .paths
            .iter()
            .filter(|p| p.key != "axes")
            .nth(i)
            .unwrap()
            .drawing
            .commands[0]
        else {
            panic!()
        };
        assert!((start.x - result.marks[i].x - radius).abs() < 1e-8);
    }
}
#[test]
fn presentation_rejects_nonfinite_out_of_range_wrong_kind_and_multiline_configuration() {
    let cases = [
        (
            PlotKind::Donut,
            PlotOptions {
                donut_inner_ratio: Some(f64::NAN),
                ..Default::default()
            },
        ),
        (
            PlotKind::Donut,
            PlotOptions {
                donut_inner_ratio: Some(0.96),
                ..Default::default()
            },
        ),
        (
            PlotKind::Pie,
            PlotOptions {
                donut_inner_ratio: Some(0.5),
                ..Default::default()
            },
        ),
        (
            PlotKind::Pie,
            PlotOptions {
                angles: Some(PlotAngles {
                    start: f64::INFINITY,
                    sweep: 180.,
                }),
                ..Default::default()
            },
        ),
        (
            PlotKind::Pie,
            PlotOptions {
                angles: Some(PlotAngles {
                    start: 0.,
                    sweep: 0.,
                }),
                ..Default::default()
            },
        ),
        (
            PlotKind::Radar,
            PlotOptions {
                angles: Some(PlotAngles::default()),
                ..Default::default()
            },
        ),
        (
            PlotKind::Waffle,
            PlotOptions {
                waffle_grid: Some(WaffleGrid {
                    rows: 0,
                    columns: 10,
                    gap: 2.,
                }),
                ..Default::default()
            },
        ),
        (
            PlotKind::Waffle,
            PlotOptions {
                waffle_grid: Some(WaffleGrid {
                    rows: 21,
                    columns: 10,
                    gap: 2.,
                }),
                ..Default::default()
            },
        ),
        (
            PlotKind::Waffle,
            PlotOptions {
                waffle_grid: Some(WaffleGrid {
                    rows: 10,
                    columns: 10,
                    gap: f64::NAN,
                }),
                ..Default::default()
            },
        ),
        (
            PlotKind::Pie,
            PlotOptions {
                waffle_grid: Some(WaffleGrid::default()),
                ..Default::default()
            },
        ),
        (
            PlotKind::Bubble,
            PlotOptions {
                bubble_radius: Some(BubbleRadius { min: 11., max: 10. }),
                ..Default::default()
            },
        ),
        (
            PlotKind::Bubble,
            PlotOptions {
                bubble_radius: Some(BubbleRadius { min: 0., max: 65. }),
                ..Default::default()
            },
        ),
        (
            PlotKind::Scatter,
            PlotOptions {
                bubble_radius: Some(BubbleRadius::default()),
                ..Default::default()
            },
        ),
    ];
    for (kind, options) in cases {
        let data = match kind {
            PlotKind::Bubble | PlotKind::Scatter => PlotData::Points(Vec::new()),
            _ => PlotData::Values(Vec::new()),
        };
        assert_eq!(scene(&data, kind, options), Err(ChartError::Presentation));
    }
    for label in [
        "é".repeat(33),
        "bad\nlabel".into(),
        "bad\u{2028}label".into(),
    ] {
        assert_eq!(
            scene(
                &values(),
                PlotKind::Donut,
                PlotOptions {
                    center_label: Some(label),
                    ..Default::default()
                }
            ),
            Err(ChartError::CenterLabel)
        );
    }
    assert!(
        scene(
            &values(),
            PlotKind::Donut,
            PlotOptions {
                center_label: Some("é".repeat(32)),
                ..Default::default()
            }
        )
        .is_ok()
    );
}
fn nodes(app: &App) -> Vec<glory::render::NodeId> {
    let tree = app.tree();
    let mut todo = tree.roots().to_vec();
    let mut out = Vec::new();
    while let Some(id) = todo.pop() {
        todo.extend(&tree.node(id).unwrap().children);
        out.push(id);
    }
    out
}
#[test]
fn invalid_center_label_update_retains_the_admitted_drawing_label_and_live_editorless_widget() {
    let app = App::memory();
    let options = app.scope().cage(PlotOptions {
        center_label: Some("72%".into()),
        ..Default::default()
    });
    app.mount(donut_chart("retained").data(values()).options(options))
        .unwrap();
    let label=nodes(&app).into_iter().find(|id|matches!(app.tree().node(*id).unwrap().descriptor.view().props,Props::Text(p) if p.content=="72%")).unwrap();
    let graphic = nodes(&app)
        .into_iter()
        .find(|id| {
            matches!(
                app.tree().node(*id).unwrap().descriptor.view().props,
                Props::Graphic(_)
            )
        })
        .unwrap();
    let original = app
        .tree()
        .node(graphic)
        .unwrap()
        .descriptor
        .view()
        .props
        .clone();
    options.revise(|v| {
        v.center_label = Some("x".repeat(65));
        v.donut_inner_ratio = Some(0.9);
    });
    assert!(app.flush().is_err());
    assert!(
        matches!(app.tree().node(label).unwrap().descriptor.view().props,Props::Text(p) if p.content=="72%")
    );
    assert_eq!(
        *app.tree().node(graphic).unwrap().descriptor.view().props,
        original
    );
    options.revise(|v| {
        v.center_label = Some("Ready".into());
        v.donut_inner_ratio = Some(0.5);
    });
    app.pump().unwrap();
    assert!(
        matches!(app.tree().node(label).unwrap().descriptor.view().props,Props::Text(p) if p.content=="Ready")
    );
    let next=nodes(&app).into_iter().find(|id|matches!(app.tree().node(*id).unwrap().descriptor.view().props,Props::Button(p) if p.label=="Next data point")).unwrap();
    app.dispatch(next, Event::Activate).unwrap();
    app.clear().unwrap();
    assert_eq!(app.stats().nodes, 0);
}

#[test]
fn scatter_axes_include_portable_grid_ticks_and_exact_current_domains() {
    let data = PlotData::Points(vec![PointDatum {
        key: "a".into(),
        label: "A".into(),
        x: 2.,
        y: 4.,
        size: 1.,
    }]);
    let result = scene(
        &data,
        PlotKind::Scatter,
        PlotOptions {
            x_domain: Some(ChartDomain::new(0., 8.).unwrap()),
            domain: Some(ChartDomain::new(0., 16.).unwrap()),
            ..Default::default()
        },
    )
    .unwrap();
    let axes = result.paths.iter().find(|p| p.key == "axes").unwrap();
    assert_eq!(axes.drawing.commands.len(), 20);
    assert!(axes.drawing.stroke.is_some());
    assert_eq!(result.labels.len(), 12);
    assert!(result.labels.iter().any(|l| l.text == "x: 0 .. 8"));
    assert!(result.labels.iter().any(|l| l.text == "y: 0 .. 16"));
    assert_eq!(result.labels[4].text, "4");
    assert_eq!(result.labels[5].text, "8");
}
#[test]
fn explicit_error_callback_handles_rejection_without_losing_a_valid_chart_or_fallback_errors() {
    use std::{cell::Cell, rc::Rc};
    for cartesian in [false, true] {
        let app = App::memory();
        let errors = Rc::new(Cell::new(0));
        let output = errors.clone();
        if cartesian {
            let source = app.scope().cage(ChartData {
                categories: vec![ChartCategory::new("a", "January")],
                series: vec![ChartSeries::new("s", "Revenue", [Some(4.)])],
            });
            app.mount(
                line_chart("handled")
                    .data(source)
                    .on_error(move |_| output.set(output.get() + 1)),
            )
            .unwrap();
            source.revise(|d| d.series[0].values[0] = Some(f64::NAN));
            app.flush().unwrap();
            assert_eq!(errors.get(), 1);
            source.revise(|d| d.series[0].values[0] = Some(8.));
            app.flush().unwrap();
        } else {
            let source = app.scope().cage(values());
            app.mount(
                donut_chart("handled")
                    .data(source)
                    .on_error(move |_| output.set(output.get() + 1)),
            )
            .unwrap();
            source.revise(|d| *d = PlotData::Points(Vec::new()));
            app.flush().unwrap();
            assert_eq!(errors.get(), 1);
            source.revise(|d| *d = values());
            app.flush().unwrap();
        }
        assert_eq!(errors.get(), 1);
        app.clear().unwrap();
        assert_eq!(app.stats().handlers, 0);
    }
    let app = App::memory();
    assert!(
        app.mount(
            donut_chart("initial")
                .data(PlotData::Points(Vec::new()))
                .on_error(|_| panic!("initial rejection must fail construction"))
        )
        .is_err()
    );
}

#[test]
fn matrix_scene_labels_each_axis_and_keeps_absent_cells_distinct_from_zero() {
    let data = PlotData::Matrix(MatrixData {
        rows: vec![
            ChartCategory::new("north", "North"),
            ChartCategory::new("south", "South"),
        ],
        columns: vec![
            ChartCategory::new("q1", "Q1"),
            ChartCategory::new("q2", "Q2"),
        ],
        cells: vec![
            MatrixCell {
                key: "n1".into(),
                row: "north".into(),
                column: "q1".into(),
                value: 0.,
            },
            MatrixCell {
                key: "s2".into(),
                row: "south".into(),
                column: "q2".into(),
                value: -3.,
            },
        ],
    });
    let result = scene(&data, PlotKind::Matrix, PlotOptions::default()).unwrap();
    assert_eq!(
        result
            .labels
            .iter()
            .take(4)
            .map(|v| v.text.as_str())
            .collect::<Vec<_>>(),
        ["North", "South", "Q1", "Q2"]
    );
    assert_eq!(result.labels.len(), 6);
    assert_eq!(result.marks.len(), 2);
    assert_eq!(result.marks[0].value_text, "0");
    assert!(result.labels[0].x < result.marks[0].x);
    assert!(result.labels[2].y < result.marks[0].y);
    assert_eq!(
        result.paths.len(),
        1,
        "zero has a reading but no invented area; absent cells have no reading"
    );
}
