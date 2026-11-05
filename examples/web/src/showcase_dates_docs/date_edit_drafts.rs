use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(Some(day("2026-11-04")));
        let message = scope.cage("Edit the appointment date, then press Enter.".to_owned());
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::field("parsed-review-date")
                    .theme(theme.clone())
                    .label("Review date draft")
                    .control(move |field| {
                        sapid::date_input("parsed-review-date", provider())
                            .field(field)
                            .value(value)
                            .on_value_change(move |v| value.revise(|s| *s = v))
                            .on_draft_change(move |draft| {
                                message.revise(|s| {
                                    *s = match draft.status {
                                        DateDraftStatus::Empty => "The date is empty.",
                                        DateDraftStatus::Incomplete => {
                                            "Keep typing the complete date."
                                        }
                                        DateDraftStatus::Valid(_) => "This is a valid date.",
                                        DateDraftStatus::Invalid => "Enter a valid calendar date.",
                                        DateDraftStatus::OutOfBounds(_) => {
                                            "Choose a date within the allowed range."
                                        }
                                        DateDraftStatus::TooLong => "The date draft is too long.",
                                    }
                                    .to_owned()
                                })
                            })
                            .on_commit(move |committed_value| {
                                message.revise(|s| {
                                    *s = if committed_value.is_some() {
                                        "Appointment date confirmed."
                                    } else {
                                        "Appointment date cleared."
                                    }
                                    .to_owned()
                                })
                            })
                    }),
                sapid::text().theme(theme.clone()).content(message),
                sapid::text().theme(theme.clone()).content(value.map(|v| {
                    v.map(|d| format!("Accepted appointment: {d}"))
                        .unwrap_or_else(|| "No appointment is selected.".to_owned())
                })),
                sapid::button()
                    .theme(theme.clone())
                    .variant(ButtonVariant::Outline)
                    .label("Continue reviewing")
                    .on_press(move || {
                        message.revise(|s| {
                            *s = "Continue with the accepted appointment date.".to_owned()
                        })
                    }),
            ))
            .show_in(scope)
    })
}
fn day(value: &str) -> CivilDate {
    CivilDate::parse_iso(value).unwrap()
}
fn provider() -> DatesProvider {
    DatesProvider::new(day("2026-11-04")).today(Some(day("2026-11-04")))
}
