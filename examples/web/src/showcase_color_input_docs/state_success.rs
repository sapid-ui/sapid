use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("showcase-color-state-success")
        .theme(theme.clone())
        .label("Success")
        .control(|field| {
            sapid::color_input("showcase-color-state-success")
                .field(field)
                .success(true)
        })
}
