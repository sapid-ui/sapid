use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill((
        sapid::field("pills-enter-field")
            .theme(theme.clone())
            .label("Enter-only topics")
            .description("Commas remain part of the draft; Enter accepts it.")
            .control(|field| {
                sapid::pills_input("pills-enter")
                    .field(field)
                    .separators(glory::render::TagSeparators::Enter)
                    .default_value(vec!["Design".into()])
            }),
        sapid::field("pills-separators-field")
            .theme(theme)
            .label("Comma and semicolon topics")
            .description("Enter, comma or semicolon accepts a topic. Duplicate topics are ignored.")
            .control(|field| {
                sapid::pills_input("pills-separators")
                    .field(field)
                    .separators(glory::render::TagSeparators::EnterCommaSemicolon)
                    .duplicate_policy(sapid::TagDuplicatePolicy::Ignore)
                    .default_value(vec!["Design".into()])
            }),
    ))
}
