use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let body = theme.clone();
    sapid::dialog("full-screen-workspace")
        .theme(theme)
        .label("Open full-screen modal")
        .title("Full-screen workspace")
        .full_screen(true)
        .content(move || {
            sapid::stack().theme(body.clone()).gap(12.).fill((
                sapid::text()
                    .theme(body.clone())
                    .content("Review your project in the available viewport."),
                sapid::text_field()
                    .theme(body.clone())
                    .name("Full-screen project draft")
                    .default_value("Retain focus while editing"),
            ))
        })
}
