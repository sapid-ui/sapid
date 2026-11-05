use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let raw = scope.cage("1234".to_owned());
        let accept = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(18.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept masked proposals")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|old| *old = v)),
                sapid::field("mask-controlled")
                    .theme(theme.clone())
                    .label("Controlled reference")
                    .control(move |field| {
                        sapid::mask_input(sapid::MaskPattern::parse("##-##").unwrap())
                            .field(field)
                            .value(raw)
                            .on_value_change(move |next| {
                                if accept.get() {
                                    raw.revise(|v| *v = next)
                                }
                            })
                    }),
                sapid::field("mask-readonly")
                    .theme(theme.clone())
                    .label("Read-only reference")
                    .readonly(true)
                    .control(|field| {
                        sapid::mask_input(sapid::MaskPattern::parse("##-##").unwrap())
                            .field(field)
                            .default_value("1234")
                    }),
                sapid::field("mask-disabled")
                    .theme(theme.clone())
                    .label("Disabled reference")
                    .enabled(false)
                    .control(|field| {
                        sapid::mask_input(sapid::MaskPattern::parse("##-##").unwrap())
                            .field(field)
                            .default_value("5678")
                    }),
            ))
            .show_in(scope)
    })
}
