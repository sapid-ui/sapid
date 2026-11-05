use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let raw = scope.cage("1234".to_owned());
        sapid::stack()
            .theme(theme.clone())
            .gap(14.)
            .fill((
                sapid::field("mask-raw")
                    .theme(theme.clone())
                    .label("Four-digit reference")
                    .control(move |field| {
                        sapid::mask_input(sapid::MaskPattern::parse("##-##").unwrap())
                            .field(field)
                            .value(raw)
                            .placeholder("00-00")
                            .on_value_change(move |next| raw.revise(|old| *old = next))
                    }),
                sapid::text()
                    .theme(theme.clone())
                    .content(scope.bond(move || format!("Raw application value: {}", raw.get()))),
                sapid::button()
                    .theme(theme.clone())
                    .label("Set reference from application")
                    .on_press(move || raw.revise(|v| *v = "9876".into())),
                sapid::button()
                    .theme(theme.clone())
                    .variant(sapid::ButtonVariant::Secondary)
                    .label("Clear reference")
                    .on_press(move || raw.revise(String::clear)),
            ))
            .show_in(scope)
    })
}
