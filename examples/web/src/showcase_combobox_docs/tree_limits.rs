use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("tree-limit-field")
        .theme(theme)
        .label("At most two folders")
        .description("Selected leaves disappear from the current choices.")
        .control(|field| {
            sapid::tree_select("tree-limit")
                .field(field)
                .nodes(folders())
                .multiple(true)
                .max_values(2)
                .hide_picked(true)
                .searchable(true)
                .default_expanded(std::collections::BTreeSet::from([
                    "work".into(),
                    "personal".into(),
                ]))
                .clearable(true)
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
