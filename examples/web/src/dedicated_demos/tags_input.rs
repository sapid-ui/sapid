use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("primary-tags-input")
        .theme(theme.clone())
        .label("Project keywords")
        .control(move |field| {
            sapid::tags_input("project-keywords")
                .field(field)
                .theme(theme.clone())
                .placeholder("Add a keyword")
                .default_value(vec!["Rust".into(), "User interfaces".into()])
                .clearable(true)
        })
}
