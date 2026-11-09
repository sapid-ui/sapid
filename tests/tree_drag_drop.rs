use glory::render::{memory::Memory, *};
use glory::{Cage, ui::App};
use sapid::{TreeDrop, TreeDropPosition, TreeNode, tree};
use std::{cell::RefCell, rc::Rc};

#[derive(Default)]
struct Wire {
    requests: Vec<(QueryToken, u64)>,
    replies: Vec<Input>,
}
struct Host {
    memory: Memory,
    wire: Rc<RefCell<Wire>>,
}
impl Render for Host {
    fn style_profile(&self) -> glory::style::Profile {
        glory::style::Profile::RECORD
    }
    fn capabilities(&self) -> Capabilities {
        Capabilities {
            measure: true,
            ..Default::default()
        }
    }
    fn apply(&mut self, mutations: &[Mutation]) -> Result<Receipt, Error> {
        self.memory.apply(mutations)
    }
    fn request(&mut self, query: Query, token: QueryToken) -> Result<(), Error> {
        query.validate_target(&self.memory.node(query.node())?.descriptor)?;
        self.wire
            .borrow_mut()
            .requests
            .push((token, self.memory.receipt().applied));
        Ok(())
    }
    fn drain(&mut self, inputs: &mut Vec<Input>) {
        inputs.append(&mut self.wire.borrow_mut().replies);
    }
}
fn fixture() -> (App, Rc<RefCell<Wire>>) {
    let wire = Rc::new(RefCell::new(Wire::default()));
    let out = wire.clone();
    (
        App::new(move |runtime| Host {
            memory: Memory::new(runtime),
            wire: out,
        }),
        wire,
    )
}
fn named(app: &App, name: &str) -> NodeId {
    let tree = app.tree();
    let mut pending = tree.roots().to_vec();
    while let Some(id) = pending.pop() {
        let node = tree.node(id).unwrap();
        if node.descriptor.view().name.as_deref() == Some(name)
            && node.descriptor.view().role == Some(Role::TreeItem)
        {
            return id;
        }
        pending.extend(&node.children);
    }
    panic!("missing {name}")
}
fn drop_on(app: &App, from: NodeId, target: NodeId) {
    app.dispatch(
        target,
        Event::Drag {
            from,
            phase: DragPhase::Drop,
            x: 10.,
            y: 90.,
        },
    )
    .unwrap();
}
fn reply(wire: &RefCell<Wire>, index: usize) {
    let (token, applied) = wire.borrow().requests[index];
    wire.borrow_mut().replies.push(Input::Response {
        token,
        applied,
        result: Ok(QueryValue::Bounds(Rect {
            x: 0.,
            y: 0.,
            width: 100.,
            height: 100.,
        })),
    });
}
fn source(app: &App) -> Cage<Vec<TreeNode>> {
    app.scope()
        .cage(vec![TreeNode::new("a", "A"), TreeNode::new("b", "B")])
}

#[test]
fn delayed_drop_requires_current_permission_model_and_row_identity() {
    for scenario in [
        "disabled",
        "disable-restore",
        "source-disabled",
        "target-disabled",
        "same-key-model",
        "source-removed",
        "target-removed",
        "unflushed-model",
        "retired",
    ] {
        let (app, wire) = fixture();
        let rows = source(&app);
        let enabled = app.scope().cage(true);
        let drops = Rc::new(RefCell::new(Vec::new()));
        let output = drops.clone();
        app.mount(
            tree("delayed-drag")
                .nodes(rows)
                .enabled(enabled)
                .on_drag_drop(move |proposal| output.borrow_mut().push(proposal)),
        )
        .unwrap();
        let from = named(&app, "A");
        let target = named(&app, "B");
        drop_on(&app, from, target);
        assert_eq!(wire.borrow().requests.len(), 1, "{scenario}");
        match scenario {
            "disabled" | "disable-restore" => enabled.revise(|value| *value = false),
            "source-disabled" => rows.revise(|rows| rows[0].disabled = true),
            "target-disabled" => rows.revise(|rows| rows[1].disabled = true),
            "same-key-model" | "unflushed-model" => {
                rows.revise(|rows| rows[1].label = "Updated B".into())
            }
            "source-removed" => rows.revise(|rows| {
                rows.remove(0);
            }),
            "target-removed" => rows.revise(|rows| {
                rows.pop();
            }),
            "retired" => app.clear().unwrap(),
            _ => unreachable!(),
        }
        if scenario != "unflushed-model" {
            app.flush().unwrap();
        }
        if scenario == "disable-restore" {
            enabled.revise(|value| *value = true);
            app.flush().unwrap();
        }
        reply(&wire, 0);
        app.pump().unwrap();
        assert!(
            drops.borrow().is_empty(),
            "{scenario}: {:?}",
            drops.borrow()
        );
    }
}

#[test]
fn newest_drop_supersedes_pending_geometry_and_commits_once() {
    let (app, wire) = fixture();
    let rows = source(&app);
    let drops = Rc::new(RefCell::new(Vec::new()));
    let output = drops.clone();
    app.mount(
        tree("latest-drag")
            .nodes(rows)
            .on_drag_drop(move |proposal| output.borrow_mut().push(proposal)),
    )
    .unwrap();
    let from = named(&app, "A");
    let target = named(&app, "B");
    drop_on(&app, from, target);
    drop_on(&app, from, target);
    assert_eq!(wire.borrow().requests.len(), 2);
    reply(&wire, 0);
    app.pump().unwrap();
    assert!(drops.borrow().is_empty());
    reply(&wire, 1);
    app.pump().unwrap();
    assert_eq!(
        *drops.borrow(),
        [TreeDrop::new("a", "b", TreeDropPosition::After)]
    );
    reply(&wire, 1);
    app.pump().unwrap();
    assert_eq!(drops.borrow().len(), 1);
}

#[test]
fn allow_drop_reentry_cannot_commit_after_permission_or_model_changes() {
    for mutate_model in [false, true] {
        let (app, wire) = fixture();
        let rows = source(&app);
        let enabled = app.scope().cage(true);
        let drops = Rc::new(RefCell::new(Vec::new()));
        let output = drops.clone();
        app.mount(
            tree("reentrant-drag")
                .nodes(rows)
                .enabled(enabled)
                .allow_drop(move |_| {
                    if mutate_model {
                        rows.revise(|rows| rows[1].disabled = true);
                    } else {
                        enabled.revise(|value| *value = false);
                    }
                    true
                })
                .on_drag_drop(move |proposal| output.borrow_mut().push(proposal)),
        )
        .unwrap();
        drop_on(&app, named(&app, "A"), named(&app, "B"));
        reply(&wire, 0);
        app.pump().unwrap();
        assert!(drops.borrow().is_empty());
    }
}
