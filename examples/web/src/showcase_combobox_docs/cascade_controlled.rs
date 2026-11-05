use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(sapid::TreeChoices::default());
        let accept = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::field("cascade-controlled-field")
                    .theme(theme.clone())
                    .label("Controlled category")
                    .control(move |field| {
                        sapid::cascader("cascade-controlled")
                            .field(field)
                            .nodes(categories())
                            .value(value)
                            .on_value_change(move |v| {
                                if accept.get() {
                                    value.revise(|old| *old = v)
                                }
                            })
                            .change_on_select(true)
                            .clearable(true)
                    }),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept category paths")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|old| *old = v)),
                sapid::text().theme(theme.clone()).content(value.map(|v| {
                    v.paths()
                        .first()
                        .map(|p| format!("Accepted category: {}", p.keys().join(" / ")))
                        .unwrap_or_else(|| "No category accepted".into())
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
