use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let provider = scope.cage(sapid::DatesProvider::new(
            sapid::CivilDate::new(2026, 11, 4).unwrap(),
        ));
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .label("Toggle month locale")
                    .on_press(move || {
                        provider.revise(|old| {
                            *old = old.locale(if old.locale_value() == sapid::DateLocale::En {
                                sapid::DateLocale::ZhCn
                            } else {
                                sapid::DateLocale::En
                            })
                        })
                    }),
                sapid::month_picker_input("month-localized", provider)
                    .theme(theme.clone())
                    .name("Localized month")
                    .form_name("localized_month")
                    .display_format(sapid::PeriodInputFormat::Localized)
                    .default_value(
                        sapid::PeriodSelection::<sapid::YearMonth>::parse_iso(
                            sapid::DatePickerMode::Single,
                            "2026-11",
                        )
                        .unwrap(),
                    )
                    .clearable(true),
                sapid::month_picker_input("month-iso", provider)
                    .theme(theme.clone())
                    .name("ISO month")
                    .default_value(
                        sapid::PeriodSelection::<sapid::YearMonth>::parse_iso(
                            sapid::DatePickerMode::Single,
                            "2026-11",
                        )
                        .unwrap(),
                    ),
            ))
            .show_in(scope)
    })
}
