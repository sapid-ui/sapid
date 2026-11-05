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
                    .label("Toggle year locale")
                    .on_press(move || {
                        provider.revise(|old| {
                            *old = old.locale(if old.locale_value() == sapid::DateLocale::En {
                                sapid::DateLocale::ZhCn
                            } else {
                                sapid::DateLocale::En
                            })
                        })
                    }),
                sapid::year_picker_input("year-localized", provider)
                    .theme(theme.clone())
                    .name("Localized year")
                    .form_name("localized_year")
                    .display_format(sapid::PeriodInputFormat::Localized)
                    .default_value(
                        sapid::PeriodSelection::<sapid::CivilYear>::parse_iso(
                            sapid::DatePickerMode::Single,
                            "2026",
                        )
                        .unwrap(),
                    )
                    .clearable(true),
                sapid::year_picker_input("year-iso", provider)
                    .theme(theme.clone())
                    .name("ISO year")
                    .default_value(
                        sapid::PeriodSelection::<sapid::CivilYear>::parse_iso(
                            sapid::DatePickerMode::Single,
                            "2026",
                        )
                        .unwrap(),
                    ),
            ))
            .show_in(scope)
    })
}
