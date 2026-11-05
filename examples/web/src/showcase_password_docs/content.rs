use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let visible = scope.cage(false);
        let custom_theme = theme.clone();
        let color = theme.foreground;
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::field("password-prefix")
                    .theme(theme.clone())
                    .label("Password with a left section")
                    .control(move |field| {
                        sapid::password_input()
                            .field(field)
                            .left_section(
                                glory::ui::graphic()
                                    .drawing(sapid::Icon::Info.drawing(color))
                                    .style(
                                        Style::new().width(Length::Px(16.)).height(Length::Px(16.)),
                                    ),
                            )
                            .left_section_width(36.)
                            .placeholder("Default eye stays on the right")
                    }),
                sapid::field("password-custom-action")
                    .theme(theme.clone())
                    .label("Password with a custom right section")
                    .control(move |field| {
                        sapid::password_input()
                            .field(field)
                            .visible(visible)
                            .right_section_width(64.)
                            .right_section(
                                sapid::button()
                                    .theme(custom_theme.clone())
                                    .compact(true)
                                    .label(visible.map(|v| {
                                        if *v {
                                            "Hide".to_owned()
                                        } else {
                                            "Show".to_owned()
                                        }
                                    }))
                                    .on_press(move || visible.revise(|v| *v = !*v)),
                            )
                    }),
            ))
            .show_in(scope)
    })
}
