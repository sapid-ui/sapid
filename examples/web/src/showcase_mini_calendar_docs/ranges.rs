use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let date = sapid::CivilDate::parse_iso("2024-02-27").unwrap();
    sapid::stack().theme(theme.clone()).gap(16.).fill((
        sapid::mini_calendar("five-days", sapid::DatesProvider::new(date))
            .theme(theme.clone())
            .name("Five visible dates")
            .number_of_days(5),
        sapid::mini_calendar("ten-days", sapid::DatesProvider::new(date))
            .theme(theme)
            .name("Ten visible dates")
            .number_of_days(10),
    ))
}
