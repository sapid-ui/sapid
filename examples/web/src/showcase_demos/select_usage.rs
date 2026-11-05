use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(None::<String>);
        sapid::field("preferred-framework")
            .theme(theme.clone())
            .label("Your favorite framework")
            .description("Choose one option")
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(360.)),
            )
            .control(move |field| {
                sapid::select("preferred-framework")
                    .field(field)
                    .placeholder("Choose a framework")
                    .clearable(true)
                    .value(value)
                    .options([
                        sapid::select_option("react", "React"),
                        sapid::select_option("vue", "Vue"),
                        sapid::select_option("svelte", "Svelte"),
                        sapid::select_option("angular", "Angular").disabled(true),
                    ])
                    .on_value_change(move |next| value.revise(|old| *old = next))
            })
            .show_in(scope)
    })
}
