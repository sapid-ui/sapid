use glory::prelude::*;
use glory::render::Direction;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill(
        [
            ("Read only times", true, true, Direction::Ltr),
            ("Disabled times", false, false, Direction::Ltr),
            ("Right to left times", true, false, Direction::Rtl),
        ]
        .into_iter()
        .map(|(name, enabled, readonly, direction)| {
            sapid::time_grid(name.replace(' ', "-"))
                .theme(theme.clone())
                .name(name)
                .enabled(enabled)
                .readonly(readonly)
                .direction(direction)
                .spec(
                    sapid::TimeGridSpec::new(
                        sapid::LocalTime::new(9, 0, 0).unwrap(),
                        sapid::LocalTime::new(10, 30, 0).unwrap(),
                        1800,
                        4,
                    )
                    .unwrap(),
                )
                .default_value(Some(sapid::LocalTime::new(9, 30, 0).unwrap()))
        })
        .collect::<Vec<_>>(),
    )
}
