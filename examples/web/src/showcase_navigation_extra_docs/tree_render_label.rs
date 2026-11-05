use glory::prelude::*;
use sapid::{TreeKind, group, tree};
use std::collections::BTreeSet;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    tree("showcase-tree-labels")
        .theme(theme.clone())
        .name("Custom file tree")
        .nodes(tree_detail_nodes())
        .default_expanded(BTreeSet::from(["src".into(), "components".into()]))
        .file_icons(true)
        .render_label(move |node| {
            group().theme(theme.clone()).gap(6.0).fill((
                glory::ui::text().content(node.with(|node| node.label.clone())),
                sapid::badge()
                    .theme(theme.clone())
                    .content(node.with(|node| match node.kind {
                        TreeKind::Leaf => "file".to_owned(),
                        TreeKind::Branch | TreeKind::Lazy => "folder".to_owned(),
                    })),
            ))
        })
}
fn tree_detail_nodes() -> Vec<sapid::TreeNode> {
    use sapid::{TreeKind, TreeNode};
    vec![
        TreeNode::new("src", "src").kind(TreeKind::Branch),
        TreeNode::new("components", "components")
            .parent("src")
            .kind(TreeKind::Branch),
        TreeNode::new("button", "Button.rs").parent("components"),
        TreeNode::new("input", "Input.rs").parent("components"),
        TreeNode::new("main", "main.rs").parent("src"),
        TreeNode::new("tests", "tests").kind(TreeKind::Branch),
        TreeNode::new("button-test", "button_test.rs").parent("tests"),
        TreeNode::new("readme", "README.md"),
    ]
}
