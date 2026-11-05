use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::checkbox()
        .theme(theme)
        .label("I agree to the terms and conditions")
}
