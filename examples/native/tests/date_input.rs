use glory::apace::{Raster, shell::WindowApp};
use glory::prelude::*;
use glory::render::{
    Action, InputEdit, InputEditIntent, NodeId, Props, Query, QueryValue, SelectionDirection,
    TextSelection,
};
use sapid::prelude::*;
use std::{cell::RefCell, rc::Rc};
fn query(app: &App, q: Query) -> QueryValue {
    let reply = Rc::new(RefCell::new(None));
    let out = reply.clone();
    app.request(q, move |v| *out.borrow_mut() = Some(v.unwrap()))
        .unwrap();
    app.pump().unwrap();
    reply.borrow_mut().take().unwrap()
}
fn edit(app: &App, id: NodeId, value: &str, after: usize, before: usize, intent: InputEditIntent) {
    let revision = {
        let tree = app.tree();
        let Props::Input(p) = tree.node(id).unwrap().descriptor.view().props else {
            panic!()
        };
        p.revision
    };
    app.dispatch(
        id,
        Event::InputEdit(InputEdit::Value {
            value: value.into(),
            revision,
            selection: TextSelection {
                start: after,
                end: after,
                direction: SelectionDirection::None,
            },
            before: Some(TextSelection {
                start: before,
                end: before,
                ..Default::default()
            }),
            intent,
            composing: false,
        }),
    )
    .unwrap();
}

fn date(v: &str) -> CivilDate {
    CivilDate::parse_iso(v).unwrap()
}
#[test]
fn editable_date_picker_action_is_inside_the_editor_border_in_all_four_themes() {
    for theme in [
        Theme::blue_light(),
        Theme::blue_dark(),
        Theme::green_light(),
        Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.renderer().set_viewport(320, 700, 1.).unwrap();
        let app = host.app();
        let reference = app.scope().cage(None);
        host.mount(
            glory::ui::view()
                .style(Style::new().width(Length::Px(280.)))
                .fill(
                    date_input("inset-date", DatesProvider::new(date("2026-11-04")))
                        .theme(theme)
                        .node_ref(reference)
                        .picker_label("Choose appointment"),
                ),
        )
        .unwrap();
        app.settle(16).unwrap();
        let input = reference.get().unwrap();
        let mut todo = app.tree().roots().to_vec();
        let mut action = None;
        while let Some(id) = todo.pop() {
            let tree = app.tree();
            let d = tree.node(id).unwrap();
            todo.extend(d.children.iter().copied());
            let v = d.descriptor.view();
            if matches!(v.props,Props::Button(p) if p.label=="Choose appointment")
                || v.name.as_deref() == Some("Choose appointment")
            {
                action = Some(id);
            }
        }
        let action = action.unwrap();
        let QueryValue::Bounds(editor) = query(app, Query::Bounds(input)) else {
            panic!()
        };
        let QueryValue::Bounds(action_rect) = query(app, Query::Bounds(action)) else {
            panic!()
        };
        assert!(
            action_rect.x >= editor.x
                && action_rect.x + action_rect.width <= editor.x + editor.width + 0.5
                && action_rect.y >= editor.y
                && action_rect.y + action_rect.height <= editor.y + editor.height + 0.5,
            "editor {editor:?}, action {action_rect:?}"
        );
        assert!(action_rect.width >= 24. && action_rect.width <= 40.);
        let identity = host
            .renderer()
            .engine_id(input, glory::html::Part::Main)
            .unwrap();
        app.dispatch(action, Event::Activate).unwrap();
        app.settle(16).unwrap();
        assert_eq!(host.renderer().inspect(|d| d.top_layers().len()), 1);
        assert_eq!(
            host.renderer()
                .engine_id(input, glory::html::Part::Main)
                .unwrap(),
            identity
        );
        app.clear().unwrap();
        assert!(app.tree().is_empty());
    }
}
#[test]
fn native_date_input_four_themes_preserve_drafts_actual_editor_caret_geometry_and_pixels() {
    for theme in [
        Theme::blue_light(),
        Theme::blue_dark(),
        Theme::green_light(),
        Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.renderer().set_viewport(700, 260, 1.).unwrap();
        let app = host.app();
        let r = app.scope().cage(None);
        let value = app.scope().cage(Some(date("2024-02-26")));
        host.mount(
            date_input("native-draft", DatesProvider::new(date("2024-02-29")))
                .with_picker(false)
                .theme(theme.clone())
                .name("Native civil draft")
                .node_ref(r)
                .value(value)
                .on_value_change(move |v| value.revise(|old| *old = v)),
        )
        .unwrap();
        app.settle(16).unwrap();
        let id = r.get().unwrap();
        let engine = host
            .renderer()
            .engine_id(id, glory::html::Part::Main)
            .unwrap();
        let QueryValue::Bounds(rect) = query(app, Query::Bounds(id)) else {
            panic!()
        };
        assert!(rect.width >= 239. && rect.height >= 35.);
        app.scope().action(Action::Focus(id)).unwrap();
        app.settle(16).unwrap();
        edit(app, id, "2024-", 5, 10, InputEditIntent::Replace);
        app.settle(16).unwrap();
        assert_eq!(value.get(), Some(date("2024-02-26")));
        host.renderer().inspect(|dom| {
            assert_eq!(
                dom.get_node(engine)
                    .unwrap()
                    .element_data()
                    .unwrap()
                    .text_input_data()
                    .unwrap()
                    .editor
                    .raw_text(),
                "2024-"
            )
        });
        edit(app, id, "2023-02-29", 10, 5, InputEditIntent::Replace);
        app.settle(16).unwrap();
        assert!(
            app.tree()
                .node(id)
                .unwrap()
                .descriptor
                .view()
                .semantics
                .as_ref()
                .unwrap()
                .invalid
        );
        edit(app, id, "2024-02-29", 10, 10, InputEditIntent::Replace);
        app.settle(16).unwrap();
        assert_eq!(value.get(), Some(date("2024-02-29")));
        app.scope()
            .set_input_selection(
                id,
                TextSelection {
                    start: 2,
                    end: 8,
                    direction: SelectionDirection::Backward,
                },
            )
            .unwrap();
        app.settle(16).unwrap();
        assert_eq!(
            query(app, Query::Selection(id)),
            QueryValue::Selection { start: 2, end: 8 }
        );
        assert_eq!(
            host.renderer()
                .engine_id(id, glory::html::Part::Main)
                .unwrap(),
            engine
        );
        let hit = host.renderer().inspect(|dom| {
            dom.hit(
                (rect.x + rect.width / 2.) as f32,
                (rect.y + rect.height / 2.) as f32,
            )
            .unwrap()
            .node_id
        });
        assert_eq!(hit, engine);
        let mut raster = Raster::new();
        raster.draw(host.renderer()).unwrap();
        let frame = raster.frame().unwrap();
        let at = ((rect.y as usize + rect.height as usize / 2) * frame.width as usize
            + rect.x as usize
            + rect.width as usize
            - 10)
            * 4;
        for (a, b) in raster.pixels()[at..at + 4].iter().zip([
            theme.input.0,
            theme.input.1,
            theme.input.2,
            theme.input.3,
        ]) {
            assert!(a.abs_diff(b) <= 2);
        }
        app.clear().unwrap();
        assert!(r.get().is_none());
        assert_eq!(app.stats().queries, 0);
    }
}
#[test]
fn native_date_input_default_dialog_updates_same_editor_and_readonly_disabled_unstyled_are_real() {
    let host = WindowApp::new();
    host.renderer().set_viewport(800, 650, 1.).unwrap();
    let app = host.app();
    let reference = app.scope().cage(None);
    let readonly = app.scope().cage(false);
    let enabled = app.scope().cage(true);
    host.mount(
        date_input("native-popup", DatesProvider::new(date("2024-02-29")))
            .unstyled()
            .size(Size::ExtraSmall)
            .node_ref(reference)
            .readonly(readonly)
            .enabled(enabled)
            .picker_label("Choose native civil date")
            .picker_title("Native date input"),
    )
    .unwrap();
    app.settle(16).unwrap();
    let id = reference.get().unwrap();
    let physical = host
        .renderer()
        .engine_id(id, glory::html::Part::Main)
        .unwrap();
    let QueryValue::Bounds(rect) = query(app, Query::Bounds(id)) else {
        panic!()
    };
    assert!(rect.width >= 239. && rect.height >= 23.);
    let mut todo = app.tree().roots().to_vec();
    let mut trigger = None;
    let mut day = None;
    while let Some(n) = todo.pop() {
        let t = app.tree();
        let node = t.node(n).unwrap();
        todo.extend(node.children.iter().copied());
        if matches!(node.descriptor.view().props, Props::Button(_))
            && node.descriptor.view().name.as_deref() == Some("Choose native civil date")
        {
            trigger = Some(n);
        }
        if node.descriptor.view().name.as_deref() == Some("Thursday, February 29, 2024") {
            day = Some(n);
        }
    }
    let trigger = trigger.unwrap();
    app.scope().action(Action::Focus(trigger)).unwrap();
    app.settle(16).unwrap();
    app.dispatch(trigger, Event::Activate).unwrap();
    app.settle(16).unwrap();
    assert_eq!(host.renderer().inspect(|doc| doc.top_layers().len()), 1);
    app.dispatch(day.unwrap(), Event::Activate).unwrap();
    app.settle(16).unwrap();
    assert_eq!(host.renderer().inspect(|doc| doc.top_layers().len()), 0);
    assert_eq!(query(app, Query::Focus(trigger)), QueryValue::Focus(true));
    host.renderer().inspect(|dom| {
        assert_eq!(
            dom.get_node(physical)
                .unwrap()
                .element_data()
                .unwrap()
                .text_input_data()
                .unwrap()
                .editor
                .raw_text(),
            "2024-02-29"
        )
    });
    readonly.revise(|v| *v = true);
    app.settle(16).unwrap();
    edit(app, id, "2024-02-28", 10, 10, InputEditIntent::Replace);
    app.settle(16).unwrap();
    assert_eq!(
        host.renderer()
            .engine_id(id, glory::html::Part::Main)
            .unwrap(),
        physical
    );
    enabled.revise(|v| *v = false);
    app.settle(16).unwrap();
    assert!(app.scope().action(Action::Focus(id)).is_err());
    app.clear().unwrap();
    assert_eq!(app.stats().queries, 0);
}
