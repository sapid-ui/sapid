use glory::{
    prelude::*,
    render::{Event, NodeId, Props},
};
use sapid::{ContentPolicy, transition};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
struct MotionHost(glory::render::memory::Memory);
impl glory::render::Render for MotionHost {
    fn request(
        &mut self,
        query: glory::render::Query,
        token: glory::render::QueryToken,
    ) -> std::result::Result<(), glory::render::Error> {
        self.0.request(query, token)
    }
    fn capabilities(&self) -> glory::render::Capabilities {
        glory::render::Capabilities {
            presence_completion: true,
            presence_transforms: true,
            ..Default::default()
        }
    }
    fn apply(
        &mut self,
        mutations: &[glory::render::Mutation],
    ) -> std::result::Result<glory::render::Receipt, glory::render::Error> {
        self.0.apply(mutations)
    }
    fn drain(&mut self, out: &mut Vec<glory::render::Input>) {
        self.0.drain(out)
    }
}
#[test]
fn reversal_and_duplicate_completion_do_not_retire_reopened_body_or_repeat_callbacks() {
    let app = App::new(|runtime| MotionHost(glory::render::memory::Memory::new(runtime)));
    let mounted = app.scope().cage(true);
    let reference = app.scope().cage(None);
    let entered = Rc::new(Cell::new(0));
    let exited = Rc::new(Cell::new(0));
    let e = entered.clone();
    let x = exited.clone();
    app.mount(
        transition()
            .mounted(mounted)
            .node_ref(reference)
            .on_entered(move || e.set(e.get() + 1))
            .on_exited(move || x.set(x.get() + 1))
            .content(glory::ui::input),
    )
    .unwrap();
    let root = reference.get().unwrap();
    let original = input(&app).unwrap();
    mounted.revise(|v| *v = false);
    app.flush().unwrap();
    assert_eq!(input(&app), Some(original));
    assert!(!app.tree().active(original));
    mounted.revise(|v| *v = true);
    app.flush().unwrap();
    assert!(
        !app.dispatch(root, Event::PresenceComplete { revision: 1 })
            .unwrap()
    );
    assert_eq!(input(&app), Some(original));
    app.dispatch(root, Event::PresenceComplete { revision: 2 })
        .unwrap();
    app.dispatch(root, Event::PresenceComplete { revision: 2 })
        .unwrap();
    assert_eq!(entered.get(), 1);
    assert_eq!(exited.get(), 0);
    mounted.revise(|v| *v = false);
    app.flush().unwrap();
    app.dispatch(root, Event::PresenceComplete { revision: 3 })
        .unwrap();
    assert!(input(&app).is_none());
    assert_eq!(exited.get(), 1);
    mounted.revise(|v| *v = true);
    app.flush().unwrap();
    assert_ne!(input(&app), Some(original));
    app.clear().unwrap();
    assert_eq!(entered.get(), 1);
}
#[test]
fn terminal_typed_callback_can_retire_its_owner_before_generic_observers() {
    let app = App::new(|runtime| MotionHost(glory::render::memory::Memory::new(runtime)));
    let mounted = app.scope().cage(false);
    let reference = app.scope().cage(None);
    let owner = app.scope().child().unwrap();
    let retire = owner.downgrade();
    let trace = Rc::new(RefCell::new(Vec::new()));
    let typed = trace.clone();
    let generic = trace.clone();
    transition()
        .mounted(mounted)
        .node_ref(reference)
        .on_entered(move || {
            typed.borrow_mut().push("typed");
            if let Some(owner) = retire.upgrade() {
                owner.dispose();
            }
        })
        .on(glory::render::EventKind::PresenceComplete, move |_| {
            generic.borrow_mut().push("late")
        })
        .content(glory::ui::input)
        .show_in(&owner)
        .unwrap();
    mounted.revise(|v| *v = true);
    app.flush().unwrap();
    let root = reference.get().unwrap();
    app.dispatch(root, Event::PresenceComplete { revision: 1 })
        .unwrap();
    assert_eq!(*trace.borrow(), ["typed"]);
    assert_eq!(app.stats().nodes, 0);
    assert!(reference.get().is_none());
}
fn descendants(app: &App) -> Vec<NodeId> {
    let tree = app.tree();
    let mut out = Vec::new();
    let mut pending = tree.roots().to_vec();
    while let Some(id) = pending.pop() {
        pending.extend(tree.node(id).unwrap().children.iter().copied());
        out.push(id)
    }
    out
}
fn input(app: &App) -> Option<NodeId> {
    descendants(app).into_iter().find(|id| {
        matches!(
            app.tree().node(*id).unwrap().descriptor.view().props,
            Props::Input(_)
        )
    })
}
#[test]
fn readonly_mounted_source_controls_owner_lifetime_and_static_lifecycle_once() {
    for policy in [ContentPolicy::Unmount, ContentPolicy::KeepMounted] {
        let app = App::memory();
        let mounted = app.scope().cage(false);
        let events = Rc::new(RefCell::new(Vec::new()));
        let builds = Rc::new(Cell::new(0));
        let drops = Rc::new(Cell::new(0));
        let reference = app.scope().cage(None);
        let mut widget = transition()
            .mounted(mounted)
            .content_policy(policy)
            .node_ref(reference)
            .content({
                let builds = builds.clone();
                let drops = drops.clone();
                move || {
                    let builds = builds.clone();
                    let drops = drops.clone();
                    glory::ui::widget(move |scope| {
                        builds.set(builds.get() + 1);
                        let drops = drops.clone();
                        scope.on_cleanup(move || drops.set(drops.get() + 1))?;
                        glory::ui::input().show_in(scope)
                    })
                }
            });
        let e = events.clone();
        widget = widget.on_enter(move || e.borrow_mut().push("enter"));
        let e = events.clone();
        widget = widget.on_entered(move || e.borrow_mut().push("entered"));
        let e = events.clone();
        widget = widget.on_exit(move || e.borrow_mut().push("exit"));
        let e = events.clone();
        widget = widget.on_exited(move || e.borrow_mut().push("exited"));
        app.mount(widget).unwrap();
        assert!(events.borrow().is_empty());
        assert!(!mounted.get());
        mounted.revise(|v| *v = true);
        app.flush().unwrap();
        let first = input(&app).unwrap();
        app.dispatch(
            first,
            Event::ValueChanged {
                value: "retained draft".into(),
                composing: false,
                revision: 0,
            },
        )
        .unwrap();
        mounted.revise(|v| *v = false);
        app.flush().unwrap();
        assert!(!app.tree().active(reference.get().unwrap()));
        if policy == ContentPolicy::Unmount {
            assert!(input(&app).is_none());
            assert_eq!(drops.get(), 1);
        } else {
            assert_eq!(input(&app), Some(first));
        }
        mounted.revise(|v| *v = true);
        app.flush().unwrap();
        if policy == ContentPolicy::KeepMounted {
            assert_eq!(input(&app), Some(first));
            assert_eq!(builds.get(), 1);
        } else {
            assert_ne!(input(&app), Some(first));
            assert_eq!(builds.get(), 2);
        }
        assert_eq!(
            *events.borrow(),
            ["enter", "entered", "exit", "exited", "enter", "entered"]
        );
    }
}
#[test]
fn invalid_both_direction_timing_retains_accepted_configuration_and_no_callbacks() {
    let app = App::memory();
    let mounted = app.scope().cage(true);
    let exit = app.scope().cage(100u16);
    let reference = app.scope().cage(None);
    let called = Rc::new(Cell::new(0));
    let out = called.clone();
    app.mount(
        transition()
            .mounted(mounted)
            .exit_duration(exit)
            .node_ref(reference)
            .on_exit(move || out.set(out.get() + 1))
            .content(|| glory::ui::text().content("body")),
    )
    .unwrap();
    let id = reference.get().unwrap();
    let before = app.tree().node(id).unwrap().descriptor.view().to_owned();
    app.scope()
        .batch(|| {
            exit.revise(|v| *v = 2001);
            mounted.revise(|v| *v = false);
        })
        .unwrap();
    assert!(app.flush().is_err());
    assert_eq!(app.tree().node(id).unwrap().descriptor, before);
    assert_eq!(called.get(), 0);
}
