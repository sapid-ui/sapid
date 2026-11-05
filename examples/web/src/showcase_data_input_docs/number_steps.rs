use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(18.).fill((
        sapid::field("number-step")
            .theme(theme.clone())
            .label("Quarter steps")
            .description("Arrow keys adjust by 0.25; Page keys adjust by 2.")
            .control(|field| {
                sapid::number_input()
                    .field(field)
                    .default_value(sapid::Decimal::new(1, 0).unwrap())
                    .step(sapid::Decimal::new(25, 2).unwrap())
                    .page_step(sapid::Decimal::new(2, 0).unwrap())
            }),
        sapid::field("number-no-controls")
            .theme(theme)
            .label("Keyboard without controls")
            .control(|field| {
                sapid::number_input()
                    .field(field)
                    .controls(false)
                    .default_value(sapid::Decimal::new(5, 0).unwrap())
            }),
    ))
}
