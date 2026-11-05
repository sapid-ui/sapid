use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("primary-pills-input")
        .theme(theme.clone())
        .label("Project topics")
        .control(move |field| {
            sapid::pills_input("project-topics")
                .field(field)
                .theme(theme.clone())
                .placeholder("Add a topic")
                .default_value(vec!["Design".into(), "Accessibility".into()])
                .clearable(true)
        })
}
