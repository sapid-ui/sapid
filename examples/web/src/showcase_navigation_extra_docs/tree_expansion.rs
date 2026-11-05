use glory::prelude::*;
use sapid::{button, expanded_tree_branches, group, stack, tree};
use std::collections::BTreeSet;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let expansion_theme = theme.clone();
    widget(move |scope| {
        let nodes = tree_detail_nodes();
        let all = expanded_tree_branches(&nodes).expect("static tree data");
        let expanded = scope.cage(BTreeSet::new());
        stack()
            .theme(expansion_theme.clone())
            .gap(8.0)
            .fill((
                group().theme(expansion_theme.clone()).gap(8.0).fill((
                    button()
                        .theme(expansion_theme.clone())
                        .label("Expand all")
                        .on_press(move || expanded.revise(|value| *value = all.clone())),
                    button()
                        .theme(expansion_theme.clone())
                        .label("Collapse all")
                        .on_press(move || expanded.revise(BTreeSet::clear)),
                )),
                tree("showcase-tree-expansion")
                    .theme(expansion_theme.clone())
                    .name("Bulk expansion tree")
                    .nodes(nodes)
                    .expanded(expanded)
                    .on_expanded_change(move |next| expanded.revise(|old| *old = next)),
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
