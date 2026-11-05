use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let body = theme.clone();
    sapid::dialog("centered-modal")
        .theme(theme)
        .label("Open centered modal")
        .title("Centered modal")
        .description("A focused place to review a message.")
        .content(move || {
            sapid::text()
                .theme(body.clone())
                .content("Close this modal or press Escape to return to its trigger.")
        })
}
