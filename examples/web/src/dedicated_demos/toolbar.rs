use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let bold = scope.cage(false);
        let italic = scope.cage(false);
        let history = scope.cage("Your draft is ready for formatting.".to_owned());
        sapid::stack()
            .theme(theme.clone())
            .gap(14.)
            .fill((
                sapid::toolbar("document-tools")
                    .theme(theme.clone())
                    .size(sapid::Size::Small)
                    .name("Document formatting")
                    .group(
                        sapid::toolbar_group("format")
                            .name("Text formatting")
                            .button(
                                sapid::button()
                                    .label("Bold")
                                    .pressed(bold)
                                    .on_press(move || bold.revise(|v| *v = !*v)),
                            )
                            .button(
                                sapid::button()
                                    .label("Italic")
                                    .pressed(italic)
                                    .on_press(move || italic.revise(|v| *v = !*v)),
                            ),
                    )
                    .group(
                        sapid::toolbar_group("history")
                            .name("Document history")
                            .button(
                                sapid::button()
                                    .name("Undo")
                                    .style(
                                        Style::new()
                                            .width(Length::Px(30.))
                                            .height(Length::Px(30.))
                                            .padding(Edges::all(0.)),
                                    )
                                    .variant(sapid::ButtonVariant::Subtle)
                                    .fill(
                                        glory::ui::graphic()
                                            .drawing(
                                                sapid::Icon::ChevronLeft.drawing(theme.foreground),
                                            )
                                            .style(
                                                Style::new()
                                                    .width(Length::Px(16.))
                                                    .height(Length::Px(16.)),
                                            ),
                                    )
                                    .on_press(move || {
                                        history.revise(|v| *v = "Last change undone.".to_owned())
                                    }),
                            )
                            .button(
                                sapid::button()
                                    .name("Redo")
                                    .style(
                                        Style::new()
                                            .width(Length::Px(30.))
                                            .height(Length::Px(30.))
                                            .padding(Edges::all(0.)),
                                    )
                                    .variant(sapid::ButtonVariant::Subtle)
                                    .fill(
                                        glory::ui::graphic()
                                            .drawing(
                                                sapid::Icon::ChevronRight.drawing(theme.foreground),
                                            )
                                            .style(
                                                Style::new()
                                                    .width(Length::Px(16.))
                                                    .height(Length::Px(16.)),
                                            ),
                                    )
                                    .on_press(move || {
                                        history.revise(|v| *v = "Last change restored.".to_owned())
                                    }),
                            ),
                    ),
                sapid::text().theme(theme.clone()).content(history),
            ))
            .show_in(scope)
    })
}
