use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let borders = scope.cage(true);
        let compact = scope.cage(false);
        let details = scope.cage(false);
        let footer = scope.cage(true);
        let overlay = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::group().theme(theme.clone()).fill((
                    sapid::checkbox()
                        .theme(theme.clone())
                        .label("Region borders")
                        .checked(borders)
                        .on_checked_change(move |v| borders.revise(|old| *old = v)),
                    sapid::checkbox()
                        .theme(theme.clone())
                        .label("Compact header")
                        .checked(compact)
                        .on_checked_change(move |v| compact.revise(|old| *old = v)),
                    sapid::checkbox()
                        .theme(theme.clone())
                        .label("Show details panel")
                        .checked(details)
                        .on_checked_change(move |v| details.revise(|old| *old = v)),
                    sapid::checkbox()
                        .theme(theme.clone())
                        .label("Position header above body")
                        .checked(overlay)
                        .on_checked_change(move |v| overlay.revise(|old| *old = v)),
                    sapid::checkbox()
                        .theme(theme.clone())
                        .label("Show footer")
                        .checked(footer)
                        .on_checked_change(move |v| footer.revise(|old| *old = v)),
                )),
                sapid::app_shell("docs-shell-configuration")
                    .theme(theme.clone())
                    .style(Style::new().height(Length::Px(280.)))
                    .with_border(borders)
                    .main_scrollable(true)
                    .header_style(scope.bond(move || {
                        let height = if compact.get() { 44. } else { 68. };
                        let mut style = Style::new().height(Length::Px(height));
                        if overlay.get() {
                            style = style
                                .position(Position::Absolute)
                                .top(Length::Px(0.))
                                .left(Length::Px(0.))
                                .right(Length::Px(0.));
                        }
                        style
                    }))
                    .body_style(scope.bond(move || {
                        Style::new().margin(Edges {
                            top: if overlay.get() {
                                if compact.get() { 44. } else { 68. }
                            } else {
                                0.
                            },
                            ..Edges::all(0.)
                        })
                    }))
                    .header(|context| sapid::text().theme(context.theme).content("Task details"))
                    .aside_width(112.)
                    .aside_visible(details)
                    .aside(|context| {
                        sapid::text()
                            .theme(context.theme)
                            .content("Assigned to you")
                    })
                    .main(|context| {
                        sapid::text_field()
                            .theme(context.theme)
                            .name("Task note")
                            .style(Style::new().width(Length::Percent(100.)))
                            .placeholder("Keep your task note…")
                    })
                    .footer_visible(footer)
                    .footer(|context| sapid::text().theme(context.theme).content("Draft saved")),
            ))
            .show_in(scope)
    })
}
