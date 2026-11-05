use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("tree-geometry-field").theme(theme).label("Compact folder popup").description("A 260-pixel popup with a 160-pixel height limit, placed above its target when space permits.").control(|field|sapid::tree_select("tree-geometry").field(field).nodes(folders()).popup_width(260.).max_dropdown_height(160.).popup_placement(glory::render::AnchorPlacement::new(glory::render::AnchorSide::Top,glory::render::AnchorAlign::End).gap(4.)).with_lines(false).default_expanded(std::collections::BTreeSet::from(["work".into(),"personal".into()])))
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
