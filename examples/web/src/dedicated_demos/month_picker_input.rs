use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let current = scope.cage(sapid::PeriodSelection::<sapid::YearMonth>::Single(None));
        let provider = sapid::DatesProvider::new(sapid::CivilDate::new(2026, 11, 4).unwrap());
        sapid::field("billing-month")
            .theme(theme.clone())
            .label("Billing month")
            .description("Choose a month, without inventing a day")
            .style(
                Style::new()
                    .width(Length::Percent(100.))
                    .max_width(Limit::Px(360.)),
            )
            .control(move |field| {
                sapid::month_picker_input("billing-month", provider)
                    .field(field)
                    .form_name("billing_month")
                    .placeholder("Pick a month")
                    .display_format(sapid::PeriodInputFormat::Localized)
                    .clearable(true)
                    .value(current)
                    .on_value_change(move |next| current.revise(|old| *old = next))
            })
            .show_in(scope)
    })
}
