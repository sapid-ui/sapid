use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::field("textarea-bio")
        .theme(theme)
        .label("Biography")
        .description("Tell us a little about yourself.")
        .control(|field| {
            sapid::textarea()
                .field(field)
                .rows(3)
                .placeholder("Your experience, interests and current projects")
        })
}
