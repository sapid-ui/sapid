use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill((sapid::field("cascade-flat-field").theme(theme.clone()).label("Single-column category browser").description("This mount chooses a compact tree instead of multiple columns.").control(|field|sapid::cascader("cascade-flat").field(field).nodes(categories()).with_columns(false).clearable(true)),sapid::field("cascade-rtl-field").theme(theme.clone()).label("Right-to-left category columns").control(|field|sapid::cascader("cascade-rtl").field(field).nodes(categories()).direction(glory::render::Direction::Rtl).default_expanded(std::collections::BTreeSet::from(["electronics".into(),"computers".into()]))),sapid::field("cascade-search-field").theme(theme).label("Search the category tree").description("Search is an editable TreeSelect composition, separate from the select-only column Cascader.").control(|field|sapid::tree_select("cascade-search").field(field).nodes(categories()).searchable(true).nothing_found_message("No matching categories").clearable(true))))
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
