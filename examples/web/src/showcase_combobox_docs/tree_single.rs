use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("tree-single-field")
        .theme(theme)
        .label("Choose a project folder")
        .control(|field| {
            sapid::tree_select("tree-single")
                .field(field)
                .nodes(folders())
                .default_value(chosen(&["work", "design"]))
                .default_expanded(std::collections::BTreeSet::from(["work".into()]))
                .clearable(true)
                .clear_label("Clear project folder")
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
