use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let current = scope.cage(sapid::PeriodSelection::<sapid::CivilYear>::Single(None));
        let provider = sapid::DatesProvider::new(sapid::CivilDate::new(2026, 11, 4).unwrap());
        sapid::field("graduation-year")
            .theme(theme.clone())
            .label("Graduation year")
            .description("Choose a year")
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(360.)),
            )
            .control(move |field| {
                sapid::year_picker_input("graduation-year", provider)
                    .field(field)
                    .form_name("graduation_year")
                    .placeholder("Pick a year")
                    .clearable(true)
                    .value(current)
                    .on_value_change(move |next| current.revise(|old| *old = next))
            })
            .show_in(scope)
    })
}
