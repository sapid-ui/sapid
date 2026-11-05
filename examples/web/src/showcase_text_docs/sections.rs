use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage("design".to_owned());
        let loading = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(16.)
            .fill((
                sapid::field("input-sections")
                    .theme(theme.clone())
                    .label("Workspace URL")
                    .description("The clear action occupies the reserved right section.")
                    .control({
                        let theme = theme.clone();
                        move |field| {
                            sapid::text_field()
                                .field(field)
                                .value(value)
                                .left_section(sapid::text().theme(theme.clone()).content("@"))
                                .right_section(
                                    sapid::close_button()
                                        .theme(theme.clone())
                                        .name("Clear workspace URL")
                                        .style(
                                            Style::new()
                                                .width(Length::Px(24.))
                                                .min_width(Length::Px(24.))
                                                .height(Length::Px(24.))
                                                .min_height(Length::Px(24.)),
                                        )
                                        .on_press(move || value.revise(String::clear)),
                                )
                                .on_value_change(move |next| value.revise(|v| *v = next))
                        }
                    }),
                sapid::field("input-loading")
                    .theme(theme.clone())
                    .label("Search workspace")
                    .control(move |field| {
                        sapid::text_field()
                            .field(field)
                            .loading(loading)
                            .loading_position(sapid::InputSectionPosition::Left)
                            .placeholder("Find a workspace")
                    }),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Show input loading indicator")
                    .checked(loading)
                    .on_checked_change(move |next| loading.revise(|v| *v = next)),
            ))
            .show_in(scope)
    })
}
