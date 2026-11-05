use glory::prelude::*;
use sapid::{expanded_tree_branches, filter_tree_nodes, stack, text_field, tree};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let filter_theme = theme.clone();
    widget(move |scope| {
        let query = scope.cage(String::new());
        let filtered = scope.bond(move || {
            filter_tree_nodes(&tree_detail_nodes(), &query.get()).expect("bounded filter query")
        });
        let expanded = scope.bond(move || {
            expanded_tree_branches(&filtered.get()).expect("filtered tree remains valid")
        });
        let empty = scope.bond(move || filtered.with(Vec::is_empty));
        stack()
            .theme(filter_theme.clone())
            .gap(8.0)
            .fill((
                text_field()
                    .theme(filter_theme.clone())
                    .name("Filter tree nodes")
                    .placeholder("Search files")
                    .value(query)
                    .on_value_change(move |next| query.revise(|old| *old = next)),
                tree("showcase-tree-filter")
                    .theme(filter_theme.clone())
                    .name("Filtered project tree")
                    .nodes(filtered)
                    .expanded(expanded),
                when(empty, || {
                    glory::ui::text()
                        .class("showcase-tree-empty")
                        .content("Nothing found")
                }),
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
