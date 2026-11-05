use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let vertical = scope.cage(false);
        let wrap = scope.cage(true);
        let spacious = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::group().theme(theme.clone()).fill((
                    sapid::checkbox()
                        .theme(theme.clone())
                        .label("Vertical direction")
                        .checked(vertical)
                        .on_checked_change(move |v| vertical.revise(|old| *old = v)),
                    sapid::checkbox()
                        .theme(theme.clone())
                        .label("Wrap flex items")
                        .checked(wrap)
                        .on_checked_change(move |v| wrap.revise(|old| *old = v)),
                    sapid::checkbox()
                        .theme(theme.clone())
                        .label("Larger gap")
                        .checked(spacious)
                        .on_checked_change(move |v| spacious.revise(|old| *old = v)),
                )),
                sapid::box_()
                    .theme(theme.clone())
                    .style(Style::new().overflow_x(Overflow::Auto))
                    .fill(
                        sapid::flex()
                            .theme(theme.clone())
                            .class("docs-flex-directions")
                            .style(scope.bond(move || {
                                Style::new()
                                    .flex_direction(if vertical.get() {
                                        FlexDirection::Column
                                    } else {
                                        FlexDirection::Row
                                    })
                                    .flex_wrap(if wrap.get() {
                                        glory::style::FlexWrap::Wrap
                                    } else {
                                        glory::style::FlexWrap::NoWrap
                                    })
                                    .gap(if spacious.get() { 24. } else { 8. })
                            }))
                            .fill(Vec::from(
                                [
                                    "Planning",
                                    "Design review",
                                    "Implementation",
                                    "Quality checks",
                                ]
                                .map(|label| {
                                    sapid::box_()
                                        .theme(theme.clone())
                                        .style(Style {
                                            background: Some(theme.accent_soft),
                                            shrink: Some(0.),
                                            ..Style::new()
                                                .width(Length::Px(132.))
                                                .padding(Edges::all(12.))
                                        })
                                        .fill(sapid::text().theme(theme.clone()).content(label))
                                }),
                            )),
                    ),
            ))
            .show_in(scope)
    })
}
