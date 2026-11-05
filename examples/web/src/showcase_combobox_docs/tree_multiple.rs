use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("tree-multiple-field")
        .theme(theme)
        .label("Project folders")
        .control(|field| {
            sapid::tree_select("tree-multiple")
                .field(field)
                .nodes(folders())
                .multiple(true)
                .default_expanded(std::collections::BTreeSet::from([
                    "work".into(),
                    "personal".into(),
                ]))
                .default_value(
                    sapid::TreeChoices::new([
                        sapid::TreePath::new(["work".into(), "design".into()]).unwrap(),
                        sapid::TreePath::new(["personal".into(), "notes".into()]).unwrap(),
                    ])
                    .unwrap(),
                )
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
