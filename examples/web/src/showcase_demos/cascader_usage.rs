use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("primary-cascader")
        .theme(theme.clone())
        .label("Product category")
        .control(move |field| {
            sapid::cascader("product-category")
                .field(field)
                .theme(theme.clone())
                .placeholder("Choose a category")
                .clearable(true)
                .nodes(vec![
                    sapid::TreeNode::new("electronics", "Electronics")
                        .kind(sapid::TreeKind::Branch),
                    sapid::TreeNode::new("computers", "Computers")
                        .parent("electronics")
                        .kind(sapid::TreeKind::Branch),
                    sapid::TreeNode::new("laptops", "Laptops").parent("computers"),
                    sapid::TreeNode::new("desktops", "Desktops").parent("computers"),
                    sapid::TreeNode::new("audio", "Audio").parent("electronics"),
                    sapid::TreeNode::new("books", "Books"),
                ])
        })
}
