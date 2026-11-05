use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let date = sapid::CivilDate::parse_iso("2024-02-27").unwrap();
        let enabled = scope.cage(true);
        let readonly = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Enable date strip")
                    .checked(enabled)
                    .on_checked_change(move |next| enabled.revise(|old| *old = next)),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Read only dates")
                    .checked(readonly)
                    .on_checked_change(move |next| readonly.revise(|old| *old = next)),
                sapid::mini_calendar("rtl-dates", sapid::DatesProvider::new(date))
                    .theme(theme.clone())
                    .name("RTL date strip")
                    .direction(glory::render::Direction::Rtl)
                    .enabled(enabled)
                    .readonly(readonly)
                    .default_value(Some(date)),
                sapid::mini_calendar(
                    "edge-dates",
                    sapid::DatesProvider::new(sapid::CivilDate::MAX),
                )
                .theme(theme.clone())
                .name("Final supported date strip")
                .unstyled()
                .number_of_days(3),
            ))
            .show_in(scope)
    })
}
