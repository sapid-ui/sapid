use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let nodes = scope.cage(vec![
            sapid::TreeNode::new("local", "Local category"),
            sapid::TreeNode::new("remote", "Remote category").kind(sapid::TreeKind::Lazy),
        ]);
        sapid::field("cascade-lazy-field")
            .theme(theme.clone())
            .label("Categories with supplied children")
            .description("Expand Remote category to publish the local sample data.")
            .control(move |field| {
                sapid::cascader("cascade-lazy")
                    .field(field)
                    .nodes(nodes)
                    .on_load_children(move |key| {
                        if key == "remote"
                            && nodes.with(|items| {
                                items
                                    .iter()
                                    .any(|n| n.key == "remote" && n.kind == sapid::TreeKind::Lazy)
                            })
                        {
                            nodes.revise(|items| {
                                items.iter_mut().find(|n| n.key == "remote").unwrap().kind =
                                    sapid::TreeKind::Branch;
                                items.push(
                                    sapid::TreeNode::new("imported", "Imported category")
                                        .parent("remote"),
                                );
                            });
                        }
                    })
            })
            .show_in(scope)
    })
}
