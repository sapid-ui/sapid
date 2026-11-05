use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(chosen(&["work", "design"]));
        let open = scope.cage(false);
        let expanded = scope.cage(std::collections::BTreeSet::from(["work".to_owned()]));
        let accept = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::field("tree-controlled-field")
                    .theme(theme.clone())
                    .label("Controlled project folder")
                    .control(move |field| {
                        sapid::tree_select("tree-controlled")
                            .field(field)
                            .nodes(folders())
                            .value(value)
                            .open(open)
                            .expanded(expanded)
                            .on_value_change(move |v| {
                                if accept.get() {
                                    value.revise(|old| *old = v)
                                }
                            })
                            .on_open_change(move |v| open.revise(|old| *old = v))
                            .on_expanded_change(move |v| expanded.revise(|old| *old = v))
                            .clearable(true)
                    }),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept folder choices")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|old| *old = v)),
                sapid::text().theme(theme.clone()).content(value.map(|v| {
                    v.paths()
                        .first()
                        .map(|p| format!("Accepted folder: {}", p.keys().join(" / ")))
                        .unwrap_or_else(|| "No folder selected".into())
                })),
            ))
            .show_in(scope)
    })
}
fn folders() -> Vec<sapid::TreeNode> {
    vec![
        sapid::TreeNode::new("work", "Work").kind(sapid::TreeKind::Branch),
        sapid::TreeNode::new("design", "Design").parent("work"),
        sapid::TreeNode::new("engineering", "Engineering").parent("work"),
        sapid::TreeNode::new("archive", "Archive")
            .parent("work")
            .disabled(true),
        sapid::TreeNode::new("personal", "Personal").kind(sapid::TreeKind::Branch),
        sapid::TreeNode::new("notes", "Notes").parent("personal"),
        sapid::TreeNode::new("remote", "Remote").kind(sapid::TreeKind::Lazy),
    ]
}
fn chosen(path: &[&str]) -> sapid::TreeChoices {
    sapid::TreeChoices::new([sapid::TreePath::new(path.iter().map(|s| (*s).to_owned())).unwrap()])
        .unwrap()
}
