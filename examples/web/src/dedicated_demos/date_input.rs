use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(None::<CivilDate>);
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::field("appointment-editable-date")
                    .theme(theme.clone())
                    .label("Appointment date")
                    .description("Type YYYY-MM-DD or choose a date from the calendar.")
                    .form_name("appointment")
                    .control(move |field| {
                        sapid::date_input("appointment-editable-date", provider())
                            .field(field)
                            .placeholder("YYYY-MM-DD")
                            .picker_label("Choose appointment date")
                            .picker_title("Appointment calendar")
                            .value(value)
                            .on_value_change(move |v| value.revise(|s| *s = v))
                    }),
                sapid::text()
                    .theme(theme.clone())
                    .variant(TextVariant::Small)
                    .content(value.map(|v| {
                        v.map(|d| format!("Your appointment: {d}"))
                            .unwrap_or_else(|| {
                                "Your appointment date has not been chosen.".to_owned()
                            })
                    })),
            ))
            .show_in(scope)
    })
}

fn provider() -> DatesProvider {
    DatesProvider::new(CivilDate::parse_iso("2026-11-04").unwrap())
        .today(Some(CivilDate::parse_iso("2026-11-04").unwrap()))
}
