use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let wrapping = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Allow actions to wrap")
                    .checked(wrapping)
                    .on_checked_change(move |v| wrapping.revise(|old| *old = v)),
                sapid::box_()
                    .theme(theme.clone())
                    .style(Style::new().overflow_x(Overflow::Auto))
                    .fill(
                        sapid::group()
                            .theme(theme.clone())
                            .class("docs-group-wrap")
                            .style(wrapping.map(|v| {
                                Style::new().flex_wrap(if *v {
                                    glory::style::FlexWrap::Wrap
                                } else {
                                    glory::style::FlexWrap::NoWrap
                                })
                            }))
                            .fill(Vec::from(
                                ["Save project", "Share with team", "Export report"].map(|label| {
                                    sapid::button()
                                        .theme(theme.clone())
                                        .label(label)
                                        .style(Style {
                                            shrink: Some(0.),
                                            ..Style::new()
                                        })
                                }),
                            )),
                    ),
            ))
            .show_in(scope)
    })
}
