use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("primary-tree-select")
        .theme(theme.clone())
        .label("Project folder")
        .control(move |field| {
            sapid::tree_select("project-folder")
                .field(field)
                .theme(theme.clone())
                .placeholder("Choose a folder")
                .searchable(true)
                .clearable(true)
                .nodes(vec![
                    sapid::TreeNode::new("work", "Work").kind(sapid::TreeKind::Branch),
                    sapid::TreeNode::new("design", "Design").parent("work"),
                    sapid::TreeNode::new("engineering", "Engineering").parent("work"),
                    sapid::TreeNode::new("personal", "Personal").kind(sapid::TreeKind::Branch),
                    sapid::TreeNode::new("notes", "Notes").parent("personal"),
                ])
        })
}
