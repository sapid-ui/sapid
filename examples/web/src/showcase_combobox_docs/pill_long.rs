use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill((
        sapid::pill()
            .theme(theme.clone())
            .content(
                "A very long topic covering architecture, localization, accessibility and testing",
            )
            .removable("Remove architecture topic")
            .style(Style::new().max_width(Limit::Percent(100.))),
        sapid::pills_input("pill-composition")
            .theme(theme)
            .name("Editable topics")
            .placeholder("Add a topic")
            .default_value(vec!["Design".into(), "Testing".into()])
            .clearable(true),
    ))
}
