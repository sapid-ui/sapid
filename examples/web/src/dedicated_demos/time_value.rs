use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(14.).fill((
        sapid::group().theme(theme.clone()).gap(10.).fill((
            sapid::text().theme(theme.clone()).content("24-hour time:"),
            sapid::time_value(Some(LocalTime::new(18, 45, 0).unwrap())).theme(theme.clone()),
        )),
        sapid::group().theme(theme.clone()).gap(10.).fill((
            sapid::text().theme(theme.clone()).content("12-hour time:"),
            sapid::time_value(Some(LocalTime::new(18, 45, 0).unwrap()))
                .theme(theme.clone())
                .format(TimeFormat::new(
                    HourCycle::H12,
                    TimePrecision::Minute,
                    DateLocale::En,
                )),
        )),
        sapid::group().theme(theme.clone()).gap(10.).fill((
            sapid::text().theme(theme.clone()).content("With seconds:"),
            sapid::time_value(Some(LocalTime::new(18, 45, 30).unwrap()))
                .theme(theme.clone())
                .format(TimeFormat::new(
                    HourCycle::H24,
                    TimePrecision::Second,
                    DateLocale::En,
                )),
        )),
    ))
}
