use glory::prelude::*;
use sapid::{
    TreeChoiceModel, TreeChoicePolicy, TreeChoices, TreeModel, TreeSelectionMode,
    expanded_tree_branches, tree,
};
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let strict_theme = theme.clone();
    widget(move |scope| {
        let nodes = tree_detail_nodes();
        let expanded = expanded_tree_branches(&nodes).expect("static tree data");
        let model = TreeChoiceModel::new(
            TreeModel::new(nodes.clone()).expect("static tree data"),
            TreeChoicePolicy::AnyNode,
            true,
        )
        .expect("valid independent policy");
        let choices = scope.cage(TreeChoices::default());
        let checked_model = model.clone();
        let checked = scope.bond(move || {
            checked_model
                .checked(&choices.get())
                .expect("current checked state")
        });
        tree("showcase-tree-strict")
            .theme(strict_theme.clone())
            .name("Independent checked tree")
            .nodes(nodes)
            .default_expanded(expanded)
            .selection_mode(TreeSelectionMode::Multiple)
            .checked(checked)
            .on_selection_intent(move |key| {
                choices.revise(|current| {
                    if let Ok(next) = model.toggle(current, &key) {
                        *current = next;
                    }
                })
            })
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
