use glory::prelude::*;
use sapid::prelude::*;
fn dt(value: &str) -> LocalDateTime {
    parse_local_date_time(value).unwrap()
}
pub fn demo(theme: sapid::Theme) -> impl Widget {
    {
        let interval =
            ScheduleInterval::new(dt("2026-11-04T09:00"), dt("2026-11-04T10:00")).unwrap();
        let civil = ScheduleContext::civil();
        let fixed = ScheduleContext::fixed(FixedOffset::new(330).unwrap());
        let application = ScheduleContext::application(
            ScheduleKey::new("demo-zone").unwrap(),
            AmbiguityPolicy::Reject,
            |value| {
                LocalResolution::Unique(ResolvedDateTime::from_fixed(
                    value,
                    FixedOffset::new(60).unwrap(),
                ))
            },
        );
        let civil_ok = matches!(civil.resolve(interval).unwrap(), ScheduleResolution::Civil);
        let fixed_ok = matches!(
            fixed.resolve(interval).unwrap(),
            ScheduleResolution::Instants { .. }
        );
        let application_ok = matches!(
            application.resolve(interval).unwrap(),
            ScheduleResolution::Instants { .. }
        );
        sapid::stack().theme(theme.clone()).fill((
            sapid::text()
                .theme(theme.clone())
                .content(format!("Civil has no instant: {civil_ok}")),
            sapid::text().theme(theme.clone()).content(format!(
                "Explicit +05:30 resolves both endpoints: {fixed_ok}"
            )),
            sapid::text().theme(theme).content(format!(
                "Supplied demo-zone resolver resolves both endpoints: {application_ok}"
            )),
        ))
    }
}
