use glory::{
    apace::shell::WindowApp,
    prelude::*,
    render::{ChoiceCommand, Event, NodeId, Props, Query, QueryValue, Rect},
};
use std::{cell::RefCell, rc::Rc};
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
#[test]
fn custom_button_content_keeps_its_real_choice_control_and_native_caption_identity() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.renderer().set_viewport(360, 220, 1.).unwrap();
        let app = host.app();
        let value = app.scope().cage(None::<String>);
        let trigger = app.scope().cage(None);
        let caption = app.scope().cage(None);
        host.mount(
            sapid::select("native-custom-trigger")
                .theme(theme.clone())
                .name("Framework")
                .node_ref(trigger)
                .style(Style::new().width(Length::Px(320.)).gap(8.))
                .value(value)
                .on_value_change(move |v| value.revise(|old| *old = v))
                .options([
                    sapid::select_option("glory", "Glory"),
                    sapid::select_option("dioxus", "Dioxus"),
                ])
                .trigger_content(move |context| {
                    glory::ui::group().fill((
                        glory::ui::graphic()
                            .drawing(sapid::Icon::Info.drawing(theme.accent_text))
                            .style(Style::new().width(Length::Px(16.)).height(Length::Px(16.))),
                        text().node_ref(caption).content(context.label),
                    ))
                }),
        )
        .unwrap();
        app.settle(16).unwrap();
        let trigger = trigger.get().unwrap();
        let caption = caption.get().unwrap();
        let physical = host
            .renderer()
            .engine_id(caption, glory::html::Part::Main)
            .unwrap();
        app.scope()
            .action(glory::render::Action::Focus(trigger))
            .unwrap();
        app.settle(16).unwrap();
        app.dispatch(
            trigger,
            Event::Choice {
                from: trigger,
                command: ChoiceCommand::OpenLast,
            },
        )
        .unwrap();
        app.settle(16).unwrap();
        let panel = find(app, |d| d.role == Some(glory::render::Role::Listbox));
        assert!((bounds(app, panel).width - 320.).abs() < 1.);
        app.dispatch(
            trigger,
            Event::Choice {
                from: trigger,
                command: ChoiceCommand::Commit,
            },
        )
        .unwrap();
        app.settle(16).unwrap();
        assert_eq!(value.get(), Some("dioxus".into()));
        assert!(
            matches!(app.tree().node(caption).unwrap().descriptor.view().props,Props::Text(p)if p.content=="Dioxus")
        );
        assert_eq!(
            host.renderer()
                .engine_id(caption, glory::html::Part::Main)
                .unwrap(),
            physical
        );
        assert!(bounds(app, caption).width > 20.);
        app.clear().unwrap();
        assert_eq!(app.stats().nodes, 0);
    }
}
#[test]
fn empty_option_list_has_a_real_anchored_passive_message_in_all_themes() {
    for theme in [
        sapid::Theme::blue_light(),
        sapid::Theme::blue_dark(),
        sapid::Theme::green_light(),
        sapid::Theme::green_dark(),
    ] {
        let host = WindowApp::new();
        host.renderer().set_viewport(360, 180, 1.).unwrap();
        host.mount(
            sapid::select("native-empty")
                .theme(theme)
                .name("Empty frameworks")
                .style(Style::new().width(Length::Px(320.)))
                .empty_message("No frameworks available")
                .default_open(true),
        )
        .unwrap();
        let app = host.app();
        app.settle(16).unwrap();
        let message = find(
            app,
            |d| matches!(d.props,Props::Text(p)if p.content=="No frameworks available"),
        );
        let rect = bounds(app, message);
        assert!(rect.width > 100. && rect.height > 10.);
        app.clear().unwrap();
        assert_eq!(app.stats().nodes, 0);
    }
}
