use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::json_viewer("escaped-json")
        .theme(theme)
        .tree_navigation(true)
        .source("{\"a/b~c\":{\"name\":\"Literal <script> 😀\"}}")
        .default_expanded([String::new(), "/a~1b~0c".into()])
}
