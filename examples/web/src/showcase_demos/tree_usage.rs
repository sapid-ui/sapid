use glory::prelude::*;
use sapid::{TreeKind, TreeNode};
use std::collections::BTreeSet;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let selected = scope.cage(BTreeSet::from(["main".to_owned()]));
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::tree("project-files")
                    .theme(theme.clone())
                    .name("Project files")
                    .file_icons(true)
                    .with_lines(true)
                    .height(240.)
                    .nodes(vec![
                        TreeNode::new("src", "src").kind(TreeKind::Branch),
                        TreeNode::new("ui", "ui")
                            .parent("src")
                            .kind(TreeKind::Branch),
                        TreeNode::new("button", "button.rs").parent("ui"),
                        TreeNode::new("input", "input.rs").parent("ui"),
                        TreeNode::new("main", "main.rs").parent("src"),
                        TreeNode::new("tests", "tests").kind(TreeKind::Branch),
                        TreeNode::new("test", "app_test.rs").parent("tests"),
                        TreeNode::new("readme", "README.md"),
                        TreeNode::new("cargo", "Cargo.toml"),
                    ])
                    .default_expanded(BTreeSet::from(["src".into(), "ui".into()]))
                    .selected(selected)
                    .on_selection_change(move |value| selected.revise(|old| *old = value)),
                sapid::text()
                    .theme(theme.clone())
                    .content(selected.map(|value| {
                        format!(
                            "Selected file key: {}",
                            value.iter().next().map(String::as_str).unwrap_or("none")
                        )
                    })),
            ))
            .show_in(scope)
    })
}
