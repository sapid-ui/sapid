use glory::apace::shell::WindowApp;
use glory::prelude::*;
use glory::render::{Action, Event, NodeId, Props, Query, QueryValue, Rect};
use sapid::*;
use std::boxed::Box;
use std::{cell::RefCell, rc::Rc};
fn provider() -> DatesProvider {
    DatesProvider::new(CivilDate::new(2024, 2, 29).unwrap())
}
fn query(app: &App, query: Query) -> QueryValue {
    let output = Rc::new(RefCell::new(None));
    let reply = output.clone();
    app.request(query, move |value| {
        *reply.borrow_mut() = Some(value.unwrap())
    })
    .unwrap();
    app.pump().unwrap();
    output.borrow_mut().take().unwrap()
}
fn bounds(app: &App, id: NodeId) -> Rect {
    let QueryValue::Bounds(rect) = query(app, Query::Bounds(id)) else {
        panic!("bounds")
    };
    rect
}
fn button(app: &App, name: &str) -> NodeId {
    let tree = app.tree();
    let mut pending = tree.roots().to_vec();
    while let Some(id) = pending.pop() {
        let node = tree.node(id).unwrap();
        let descriptor = node.descriptor.view();
        if matches!(descriptor.props, Props::Button(_)) && descriptor.name.as_deref() == Some(name)
        {
            return id;
        }
        pending.extend(node.children.iter().copied());
    }
    panic!("missing {name}")
}

#[test]
fn inset_actions_fit_all_five_real_controls_and_clear_returns_actual_focus_without_replacement() {
    for theme in [
        Theme::blue_light(),
        Theme::blue_dark(),
        Theme::green_light(),
        Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.renderer().set_viewport(390, 600, 1.).unwrap();
        let app = host.app();
        let select_ref = app.scope().cage(None);
        let date = app.scope().cage(None);
        let time = app.scope().cage(None);
        let month = app.scope().cage(None);
        let year = app.scope().cage(None);
        host.mount(
            glory::ui::view()
                .style(
                    Style::new()
                        .width(Length::Percent(100.))
                        .padding(Edges::all(20.)),
                )
                .fill(
                    sapid::stack().theme(theme.clone()).gap(12.).fill((
                        sapid::select("select")
                            .theme(theme.clone())
                            .label("Language")
                            .node_ref(select_ref)
                            .clearable(true)
                            .default_value(Some("rust".into()))
                            .options([select_option("rust", "Rust"), select_option("go", "Go")]),
                        date_picker_input("date", provider())
                            .theme(theme.clone())
                            .name("Appointment")
                            .node_ref(date)
                            .clearable(true)
                            .default_value(DateSelection::Single(Some(
                                CivilDate::new(2024, 2, 29).unwrap(),
                            ))),
                        date_time_picker("time", provider(), DateTimePolicy::Civil)
                            .theme(theme.clone())
                            .name("Meeting")
                            .node_ref(time)
                            .clearable(true)
                            .default_value(Some(
                                parse_local_date_time("2024-02-29T09:30").unwrap(),
                            )),
                        month_picker_input("month", provider())
                            .theme(theme.clone())
                            .name("Month")
                            .node_ref(month)
                            .clearable(true)
                            .default_value(MonthSelection::Single(Some(
                                YearMonth::new(2024, 2).unwrap(),
                            ))),
                        year_picker_input("year", provider())
                            .theme(theme.clone())
                            .name("Year")
                            .node_ref(year)
                            .clearable(true)
                            .default_value(YearSelection::Single(Some(
                                CivilYear::new(2024).unwrap(),
                            ))),
                    )),
                ),
        )
        .unwrap();
        app.settle(16).unwrap();
        for (name, reference) in [
            ("Clear selection", select_ref),
            ("Clear dates", date),
            ("Clear date and time", time),
            ("Clear months", month),
            ("Clear years", year),
        ] {
            let id = reference.get().unwrap();
            let input = bounds(app, id);
            if name == "Clear selection" {
                let label = {
                    let tree = app.tree();
                    tree.node(id)
                        .unwrap()
                        .children
                        .iter()
                        .copied()
                        .find(|node| {
                            tree.node(*node)
                                .unwrap()
                                .descriptor
                                .view()
                                .classes
                                .iter()
                                .any(|class| class == "sapid-select-value")
                        })
                        .unwrap()
                };
                let label = bounds(app, label);
                assert!(
                    label.x - input.x <= 16. && label.width < input.width / 2.,
                    "selection label starts at input padding: {label:?}"
                );
            }
            let clear = button(app, name);
            let action = bounds(app, clear);
            assert!(
                input.width > 250. && input.width <= 350.1,
                "control width: {input:?}"
            );
            assert!(action.x >= input.x && action.x + action.width <= input.x + input.width + 1.);
            assert!(
                action.y >= input.y - 1. && action.y + action.height <= input.y + input.height + 1.
            );
            assert!(action.width <= 36.1 && action.height <= input.height + 1.);
            let physical = host
                .renderer()
                .engine_id(id, glory::html::Part::Main)
                .unwrap();
            let clear_physical = host
                .renderer()
                .engine_id(clear, glory::html::Part::Main)
                .unwrap();
            assert_eq!(
                host.renderer().inspect(|dom| dom
                    .hit(
                        (action.x + action.width / 2.) as f32,
                        (action.y + action.height / 2.) as f32
                    )
                    .unwrap()
                    .node_id),
                clear_physical,
                "passive icons cannot intercept action hit targets"
            );
            app.scope().action(Action::Focus(clear)).unwrap();
            app.settle(16).unwrap();
            app.dispatch(clear, Event::Activate).unwrap();
            app.settle(16).unwrap();
            assert_eq!(query(app, Query::Focus(id)), QueryValue::Focus(true));
            assert_eq!(
                host.renderer()
                    .engine_id(id, glory::html::Part::Main)
                    .unwrap(),
                physical
            );
            assert_eq!(reference.get(), Some(id));
            assert!(!app.tree().enabled(clear));
        }
        app.clear().unwrap();
        assert_eq!(app.stats().nodes, 0);
    }
}

#[test]
fn percentage_dimensions_and_margins_apply_once_to_the_complete_control() {
    for date_control in [false, true] {
        let host = WindowApp::new();
        host.renderer().set_viewport(600, 400, 1.).unwrap();
        let app = host.app();
        let reference = app.scope().cage(None);
        let parent = app.scope().cage(None);
        let style = app.scope().cage(
            Style::new()
                .width(Length::Percent(50.))
                .height(Length::Percent(50.))
                .margin(Edges::all(6.)),
        );
        let control: Box<dyn Widget> = if date_control {
            Box::new(
                date_picker_input("sized", provider())
                    .style(style)
                    .node_ref(reference),
            )
        } else {
            Box::new(
                sapid::select("sized")
                    .style(style)
                    .node_ref(reference)
                    .option(select_option("rust", "Rust")),
            )
        };
        host.mount(
            glory::ui::view()
                .node_ref(parent)
                .style(
                    Style::new()
                        .width(Length::Px(400.))
                        .height(Length::Px(200.)),
                )
                .fill(control),
        )
        .unwrap();
        app.settle(16).unwrap();
        let id = reference.get().unwrap();
        let origin = bounds(app, parent.get().unwrap());
        let rect = bounds(app, id);
        assert!(
            (rect.width - 200.).abs() < 0.2 && (rect.height - 100.).abs() < 0.2,
            "percentage geometry: {rect:?}"
        );
        assert!((rect.x - origin.x - 6.).abs() < 0.2 && (rect.y - origin.y - 6.).abs() < 0.2);
        style.revise(|old| {
            *old = Style::new()
                .width(Length::Percent(75.))
                .height(Length::Percent(25.))
                .margin(Edges::all(12.))
        });
        app.settle(16).unwrap();
        let rect = bounds(app, id);
        assert!(
            (rect.width - 300.).abs() < 0.2 && (rect.height - 50.).abs() < 0.2,
            "updated geometry: {rect:?}"
        );
        assert!((rect.x - origin.x - 12.).abs() < 0.2 && (rect.y - origin.y - 12.).abs() < 0.2);
        assert_eq!(reference.get(), Some(id));
    }
}

#[test]
fn flex_growth_reserves_one_control_footprint_beside_other_content() {
    let host = WindowApp::new();
    let app = host.app();
    let reference = app.scope().cage(None);
    host.mount(
        glory::ui::view()
            .style(
                Style::new()
                    .display(Display::Flex)
                    .width(Length::Px(400.))
                    .gap(8.),
            )
            .fill((
                glory::ui::view().style(
                    Style::new()
                        .width(Length::Px(100.))
                        .min_width(Length::Px(100.)),
                ),
                sapid::select("growing")
                    .node_ref(reference)
                    .option(select_option("rust", "Rust"))
                    .style(Style {
                        grow: Some(1.),
                        ..Style::new().width(Length::Auto)
                    }),
            )),
    )
    .unwrap();
    app.settle(16).unwrap();
    assert!((bounds(app, reference.get().unwrap()).width - 292.).abs() < 0.2);
}
