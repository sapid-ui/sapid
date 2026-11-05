use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let modes = [
        (sapid::NumberClampBehavior::Blur, "Clamp on blur"),
        (sapid::NumberClampBehavior::Strict, "Strict range"),
        (sapid::NumberClampBehavior::None, "No clamping"),
    ];
    sapid::stack().theme(theme.clone()).gap(18.).fill(
        modes
            .into_iter()
            .enumerate()
            .map(|(i, (policy, label))| {
                sapid::field(format!("number-clamp-{i}"))
                    .theme(theme.clone())
                    .label(label)
                    .description("Range 0–10. Try entering 27, then move focus away.")
                    .control(move |field| {
                        sapid::number_input()
                            .field(field)
                            .default_value(sapid::Decimal::new(2, 0).unwrap())
                            .min(Some(sapid::Decimal::new(0, 0).unwrap()))
                            .max(Some(sapid::Decimal::new(10, 0).unwrap()))
                            .clamp_behavior(policy)
                    })
            })
            .collect::<Vec<_>>(),
    )
}
