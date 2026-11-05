use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::switch()
        .theme(theme)
        .label("Notifications")
        .default_checked(true)
        .with_marks(true)
}
