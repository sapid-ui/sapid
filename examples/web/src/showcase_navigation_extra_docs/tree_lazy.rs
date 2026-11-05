use glory::prelude::*;
use sapid::{TreeKind, TreeNode, stack, tree};
use std::collections::BTreeSet;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let lazy_theme = theme.clone();
    widget(move |scope| {
        let data = scope.cage(vec![
            TreeNode::new("workspace", "Workspace").kind(TreeKind::Branch),
            TreeNode::new("remote", "Remote package")
                .parent("workspace")
                .kind(TreeKind::Lazy),
        ]);
        let status = scope.cage("Expand Remote package to load children".to_owned());
        stack()
            .theme(lazy_theme.clone())
            .gap(8.0)
            .fill((
                tree("showcase-tree-lazy")
                    .theme(lazy_theme.clone())
                    .name("Lazy loading tree")
                    .nodes(data)
                    .default_expanded(BTreeSet::from(["workspace".into()]))
                    .on_load_children(move |key| {
                        if key != "remote" {
                            return;
                        }
                        data.revise(|nodes| {
                            if nodes[1].kind == TreeKind::Lazy {
                                nodes[1].kind = TreeKind::Branch;
                                nodes.push(
                                    TreeNode::new("remote-index", "index.tsx").parent("remote"),
                                );
                            }
                        });
                        status.revise(|value| *value = "Loaded Remote package".into());
                    }),
                glory::ui::text()
                    .class("showcase-tree-lazy-status")
                    .content(status),
            ))
            .show_in(scope)
    })
}
