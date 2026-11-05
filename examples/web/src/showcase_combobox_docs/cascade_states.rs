use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill((
        sapid::field("cascade-disabled-field")
            .theme(theme.clone())
            .label("Unavailable category")
            .control(|field| {
                sapid::cascader("cascade-disabled")
                    .field(field)
                    .nodes(categories())
                    .default_value(chosen(&["electronics", "audio"]))
                    .enabled(false)
            }),
        sapid::field("cascade-readonly-field")
            .theme(theme.clone())
            .label("Read-only category")
            .control(|field| {
                sapid::cascader("cascade-readonly")
                    .field(field)
                    .nodes(categories())
                    .default_value(chosen(&["electronics", "audio"]))
                    .readonly(true)
            }),
        sapid::field("cascade-error-field")
            .theme(theme)
            .label("Required category")
            .required(true)
            .error("Choose a product category")
            .control(|field| {
                sapid::cascader("cascade-error")
                    .field(field)
                    .nodes(categories())
            }),
    ))
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
