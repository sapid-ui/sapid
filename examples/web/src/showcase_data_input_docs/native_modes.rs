use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(18.).fill((
        sapid::field("native-single")
            .theme(theme.clone())
            .label("Single language")
            .control(|field| {
                sapid::native_select("native-single-control")
                    .field(field)
                    .default_value(Some("rust".into()))
                    .option(sapid::select_option("rust", "Rust"))
                    .option(sapid::select_option("js", "JavaScript"))
                    .option(sapid::select_option("py", "Python"))
            }),
        sapid::field("native-multiple")
            .theme(theme)
            .label("Several languages")
            .description("Use your platform's native multiple-selection keys.")
            .control(|field| {
                sapid::native_select("native-multiple-control")
                    .field(field)
                    .multiple(true)
                    .rows(4)
                    .default_values(["rust".into(), "py".into()])
                    .option(sapid::select_option("rust", "Rust"))
                    .option(sapid::select_option("js", "JavaScript"))
                    .option(sapid::select_option("py", "Python"))
            }),
    ))
}
