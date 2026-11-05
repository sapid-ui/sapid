use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let body = theme.clone();
    sapid::drawer("drawer-nested")
        .theme(theme)
        .label("Open workspace settings drawer")
        .title("Workspace settings")
        .content(move || {
            sapid::dialog("drawer-details-modal")
                .theme(body.clone())
                .label("Review workspace permissions")
                .title("Workspace permissions")
                .description("Only the topmost layer owns this focus scope.")
        })
}
