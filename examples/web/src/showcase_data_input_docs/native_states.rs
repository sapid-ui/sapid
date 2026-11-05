use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let value = scope.cage(Some("starter".to_owned()));
        let accept = scope.cage(false);
        sapid::stack()
            .theme(theme.clone())
            .gap(18.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Accept native selection")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|old| *old = v)),
                sapid::field("native-controlled")
                    .theme(theme.clone())
                    .label("Controlled plan")
                    .control(move |field| {
                        sapid::native_select("native-controlled-control")
                            .field(field)
                            .value(value)
                            .option(sapid::select_option("starter", "Starter"))
                            .option(sapid::select_option("pro", "Pro"))
                            .on_value_change(move |next| {
                                if accept.get() {
                                    value.revise(|v| *v = next)
                                }
                            })
                    }),
                sapid::field("native-disabled")
                    .theme(theme.clone())
                    .label("Disabled plan")
                    .enabled(false)
                    .control(|field| {
                        sapid::native_select("native-disabled-control")
                            .field(field)
                            .default_value(Some("starter".into()))
                            .option(sapid::select_option("starter", "Starter"))
                    }),
                sapid::field("native-error")
                    .theme(theme.clone())
                    .label("Required plan")
                    .required(true)
                    .error("Choose an available plan.")
                    .control(|field| {
                        sapid::native_select("native-error-control")
                            .field(field)
                            .placeholder("Choose a plan")
                            .option(sapid::select_option("pro", "Pro"))
                    }),
            ))
            .show_in(scope)
    })
}
