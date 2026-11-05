use glory::prelude::*;
use sapid::{stack, tree};
use std::collections::BTreeSet;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let controlled_theme = theme.clone();
    widget(move |scope| {
        let expanded = scope.cage(BTreeSet::from(["src".to_owned()]));
        let selected = scope.cage(BTreeSet::from(["main".to_owned()]));
        stack()
            .theme(controlled_theme.clone())
            .gap(8.0)
            .fill((
                tree("showcase-tree-controlled")
                    .theme(controlled_theme.clone())
                    .name("Controlled project tree")
                    .nodes(tree_detail_nodes())
                    .expanded(expanded)
                    .selected(selected)
                    .on_expanded_change(move |next| expanded.revise(|old| *old = next))
                    .on_selection_change(move |next| selected.revise(|old| *old = next)),
                glory::ui::text()
                    .class("showcase-tree-controlled-value")
                    .content(scope.bond(move || {
                        format!(
                            "Selected: {}",
                            selected
                                .get()
                                .iter()
                                .next()
                                .cloned()
                                .unwrap_or_else(|| "none".into())
                        )
                    })),
            ))
            .show_in(scope)
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
