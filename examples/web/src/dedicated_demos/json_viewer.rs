use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::json_viewer("project-json").theme(theme).name("Project configuration").tree_navigation(true).source("{\"project\":{\"name\":\"Sapid\",\"active\":true},\"members\":[\"Ari\",\"Lin\"],\"release\":1}").default_expanded([String::new(),"/project".into(),"/members".into()])
}
