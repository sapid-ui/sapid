use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let search = scope.cage(String::new());
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::field("tree-search-field")
                    .theme(theme.clone())
                    .label("Search project folders")
                    .control(move |field| {
                        sapid::tree_select("tree-search")
                            .field(field)
                            .nodes(folders())
                            .searchable(true)
                            .search(search)
                            .on_search_change(move |v| search.revise(|old| *old = v))
                            .default_expanded(std::collections::BTreeSet::from([
                                "work".into(),
                                "personal".into(),
                            ]))
                            .nothing_found_message("No folders match this search")
                            .clearable(true)
                    }),
                sapid::text()
                    .theme(theme.clone())
                    .content(search.map(|v| format!("Folder search: {v}"))),
            ))
            .show_in(scope)
    })
}
fn folders() -> Vec<sapid::TreeNode> {
    vec![
        sapid::TreeNode::new("work", "Work").kind(sapid::TreeKind::Branch),
        sapid::TreeNode::new("design", "Design").parent("work"),
        sapid::TreeNode::new("engineering", "Engineering").parent("work"),
        sapid::TreeNode::new("archive", "Archive")
            .parent("work")
            .disabled(true),
        sapid::TreeNode::new("personal", "Personal").kind(sapid::TreeKind::Branch),
        sapid::TreeNode::new("notes", "Notes").parent("personal"),
        sapid::TreeNode::new("remote", "Remote").kind(sapid::TreeKind::Lazy),
    ]
}
