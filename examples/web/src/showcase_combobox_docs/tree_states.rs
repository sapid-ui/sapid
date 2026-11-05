use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill((
        sapid::field("tree-disabled-field")
            .theme(theme.clone())
            .label("Unavailable folder")
            .control(|field| {
                sapid::tree_select("tree-disabled")
                    .field(field)
                    .nodes(folders())
                    .default_value(chosen(&["work", "design"]))
                    .enabled(false)
            }),
        sapid::field("tree-readonly-field")
            .theme(theme.clone())
            .label("Read-only folder")
            .control(|field| {
                sapid::tree_select("tree-readonly")
                    .field(field)
                    .nodes(folders())
                    .default_value(chosen(&["work", "design"]))
                    .readonly(true)
            }),
        sapid::field("tree-error-field")
            .theme(theme)
            .label("Required folder")
            .required(true)
            .error("Choose a project folder")
            .control(|field| {
                sapid::tree_select("tree-error")
                    .field(field)
                    .nodes(folders())
            }),
    ))
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
