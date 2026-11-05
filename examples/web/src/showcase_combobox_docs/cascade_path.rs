use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(chosen(&["electronics", "computers", "laptops"]));
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::field("cascade-path-field")
                    .theme(theme.clone())
                    .label("Product category path")
                    .control(move |field| {
                        sapid::cascader("cascade-path")
                            .field(field)
                            .nodes(categories())
                            .value(value)
                            .on_value_change(move |v| value.revise(|old| *old = v))
                            .default_expanded(std::collections::BTreeSet::from([
                                "electronics".into(),
                                "computers".into(),
                            ]))
                            .clearable(true)
                            .clear_label("Clear product category")
                    }),
                sapid::text().theme(theme.clone()).content(value.map(|v| {
                    v.paths()
                        .first()
                        .map(|p| format!("Category path: {}", p.keys().join(" / ")))
                        .unwrap_or_else(|| "No category selected".into())
                })),
            ))
            .show_in(scope)
    })
}
fn categories() -> Vec<sapid::TreeNode> {
    vec![
        sapid::TreeNode::new("electronics", "Electronics").kind(sapid::TreeKind::Branch),
        sapid::TreeNode::new("computers", "Computers")
            .parent("electronics")
            .kind(sapid::TreeKind::Branch),
        sapid::TreeNode::new("laptops", "Laptops").parent("computers"),
        sapid::TreeNode::new("desktops", "Desktops").parent("computers"),
        sapid::TreeNode::new("audio", "Audio").parent("electronics"),
        sapid::TreeNode::new("books", "Books"),
    ]
}
fn chosen(path: &[&str]) -> sapid::TreeChoices {
    sapid::TreeChoices::new([sapid::TreePath::new(path.iter().map(|s| (*s).to_owned())).unwrap()])
        .unwrap()
}
