use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let left = scope.cage(false);
        let offset = scope.bond(move || sapid::AffixOffset {
            horizontal: if left.get() {
                sapid::AffixHorizontal::Left(12.)
            } else {
                sapid::AffixHorizontal::Right(12.)
            },
            vertical: sapid::AffixVertical::Bottom(12.),
        });
        let content_theme = theme.clone();
        sapid::stack()
            .theme(theme.clone())
            .gap(8.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Use left corner")
                    .checked(left)
                    .on_checked_change(move |next| left.revise(|old| *old = next)),
                sapid::affix_container("reactive-corner")
                    .theme(theme.clone())
                    .style(Style {
                        background: Some(theme.accent_soft),
                        ..Style::new().height(Length::Px(160.))
                    })
                    .content(move |context| {
                        sapid::affix("corner-action")
                            .within(context)
                            .offset(offset)
                            .fill(
                                sapid::button()
                                    .theme(content_theme.clone())
                                    .label("Positioned action"),
                            )
                    }),
            ))
            .show_in(scope)
    })
}
