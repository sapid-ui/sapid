use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(sapid::NumberValue::Decimal(
            sapid::Decimal::new(2, 0).unwrap(),
        ));
        let accept = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(18.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept number proposals")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|old| *old = v)),
                sapid::field("number-controlled")
                    .theme(theme.clone())
                    .label("Controlled quantity")
                    .control(move |field| {
                        sapid::number_input()
                            .field(field)
                            .value(value)
                            .on_value_change(move |next| {
                                if accept.get() {
                                    value.revise(|old| *old = next)
                                }
                            })
                            .on_commit(move |next| {
                                if accept.get() {
                                    value.revise(|old| *old = next)
                                }
                            })
                    }),
                sapid::field("number-readonly")
                    .theme(theme.clone())
                    .label("Read-only quantity")
                    .readonly(true)
                    .control(|field| {
                        sapid::number_input()
                            .field(field)
                            .default_value(sapid::Decimal::new(3, 0).unwrap())
                    }),
                sapid::field("number-disabled")
                    .theme(theme.clone())
                    .label("Disabled quantity")
                    .enabled(false)
                    .control(|field| {
                        sapid::number_input()
                            .field(field)
                            .default_value(sapid::Decimal::new(4, 0).unwrap())
                    }),
            ))
            .show_in(scope)
    })
}
