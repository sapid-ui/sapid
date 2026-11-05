use glory::prelude::*;
use sapid::{expanded_tree_branches, move_tree_node, tree};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let drag_theme = theme.clone();
    widget(move |scope| {
        let data = scope.cage(tree_detail_nodes());
        let expanded = expanded_tree_branches(&data.get()).expect("static tree data");
        let status = scope.cage("Drag a row before, inside or after another row".to_owned());
        tree("showcase-tree-drag")
            .theme(drag_theme.clone())
            .name("Reorderable project tree")
            .nodes(data)
            .default_expanded(expanded)
            .allow_drop(|proposal| proposal.target != "readme")
            .on_drag_drop(move |proposal| {
                let next = move_tree_node(&data.get(), &proposal);
                if let Ok(next) = next {
                    status.revise(|value| {
                        *value = format!(
                            "Moved {} {:?} {}",
                            proposal.dragged, proposal.position, proposal.target
                        )
                    });
                    data.revise(|current| *current = next);
                }
            })
            .show_in(scope)?;
        glory::ui::text()
            .class("showcase-tree-drag-status")
            .content(status)
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
