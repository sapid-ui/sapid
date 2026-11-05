use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let body = theme.clone();
    sapid::popover("popover-nested")
        .theme(theme)
        .label("Open sharing options")
        .title("Sharing options")
        .content(move || {
            sapid::dialog("sharing-details")
                .theme(body.clone())
                .label("Review sharing details")
                .title("Sharing details")
                .description("Close this modal to return to the sharing options.")
        })
}
