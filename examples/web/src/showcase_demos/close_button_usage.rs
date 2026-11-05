use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::close_button()
        .theme(theme)
        .name("Close notification")
}
