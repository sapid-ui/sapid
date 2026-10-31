use glory::prelude::*;
use sapid::Theme;

pub fn demo_for(id: &str, theme: Theme) -> Box<dyn Widget> {
    match id {
        "P1-01" => Box::new(
            sapid::field("focused-field-email")
                .theme(theme.clone())
                .label("Email address")
                .description("We will only use this for important account updates.")
                .required(true)
                .control(|field| {
                    sapid::text_field()
                        .field(field)
                        .default_value("alex@studio.design")
                }),
        ),
        "P1-02" => Box::new(
            sapid::field("focused-field-parts")
                .theme(theme.clone())
                .label("Work email")
                .description("Use the address associated with your workspace.")
                .error("Enter a valid email address, including the @ sign.")
                .required(true)
                .control(|field| {
                    sapid::text_field()
                        .field(field)
                        .default_value("alex@studio")
                }),
        ),
        "P1-03" => Box::new(widget(move |scope| {
            let disabled = scope.cage(false);
            sapid::stack()
                .theme(theme.clone())
                .gap(16.0)
                .fill((
                    sapid::fieldset("focused-profile")
                        .theme(theme.clone())
                        .legend("Personal information")
                        .disabled(disabled)
                        .fill((
                            sapid::field("focused-profile-name")
                                .theme(theme.clone())
                                .label("Your name")
                                .control(|field| {
                                    sapid::text_field()
                                        .field(field)
                                        .default_value("Ada Lovelace")
                                }),
                            sapid::field("focused-profile-email")
                                .theme(theme.clone())
                                .label("Email")
                                .control(|field| {
                                    sapid::text_field()
                                        .field(field)
                                        .default_value("ada@example.com")
                                }),
                        )),
                    sapid::group().theme(theme.clone()).fill(
                        sapid::button()
                            .theme(theme.clone())
                            .variant(sapid::ButtonVariant::Secondary)
                            .size(sapid::Size::Small)
                            .label("Toggle fieldset disabled")
                            .on_press(move || disabled.revise(|value| *value = !*value)),
                    ),
                ))
                .show_in(scope)
        })),
        "P1-04" => Box::new(widget(move |scope| {
            let value = scope.cage(
                "Tell us a little about yourself.\nThis textarea grows with its content."
                    .to_owned(),
            );
            let size = scope.cage(Some("md".to_owned()));
            let min_rows = scope.cage(Some("3".to_owned()));
            let max_rows = scope.cage(Some("6".to_owned()));
            let autosize = scope.cage(true);
            let readonly = scope.cage(false);
            let disabled = scope.cage(false);
            let enabled = scope.bond(move || !disabled.get());
            let revision = scope.bond(move || {
                vec![format!(
                    "{}-{}-{}-{}",
                    size.get().unwrap_or_default(),
                    min_rows.get().unwrap_or_default(),
                    max_rows.get().unwrap_or_default(),
                    autosize.get()
                )]
            });
            let preview_theme = theme.clone();
            let preview = each(revision)
                .key(|revision| revision.clone())
                .children(move |entry| {
                    let size = super::controls::size_value(&entry.value.get());
                    let min_rows = min_rows
                        .get()
                        .and_then(|value| value.parse::<u16>().ok())
                        .unwrap_or(3);
                    let max_rows = max_rows
                        .get()
                        .and_then(|value| value.parse::<u16>().ok())
                        .unwrap_or(6);
                    sapid::field("focused-biography")
                        .theme(preview_theme.clone())
                        .label("Biography")
                        .description("Autosizes from three to six lines as you write.")
                        .readonly(readonly)
                        .enabled(enabled)
                        .control({
                            let theme = preview_theme.clone();
                            move |field| {
                                sapid::textarea()
                                    .theme(theme.clone())
                                    .size(size)
                                    .field(field)
                                    .autosize(autosize.get())
                                    .min_rows(min_rows)
                                    .max_rows(max_rows)
                                    .value(value)
                                    .on_value_change(move |next| value.revise(|v| *v = next))
                            }
                        })
                });
            let controls = vec![
                super::controls::size(size, &theme),
                super::controls::select(
                    "min-rows",
                    "Minimum rows",
                    min_rows,
                    &[("2", "2"), ("3", "3"), ("4", "4")],
                    &theme,
                ),
                super::controls::select(
                    "max-rows",
                    "Maximum rows",
                    max_rows,
                    &[("4", "4"), ("5", "5"), ("6", "6")],
                    &theme,
                ),
                super::controls::checkbox("Autosize", autosize, &theme),
                super::controls::checkbox("Read only", readonly, &theme),
                super::controls::checkbox("Disabled", disabled, &theme),
            ];
            super::controls::playground(Box::new(preview), controls).show_in(scope)
        })),
        "P1-05" => Box::new(
            sapid::field("focused-password")
                .theme(theme.clone())
                .label("Account password")
                .description("Use the visibility action without losing your input.")
                .required(true)
                .control(|field| {
                    sapid::password_input()
                        .field(field)
                        .default_value("example-password")
                }),
        ),
        "P1-06" => Box::new(
            sapid::field("focused-website")
                .theme(theme.clone())
                .label("Website")
                .description("Addons and actions stay aligned with the input.")
                .control(move |field| {
                    sapid::input_group()
                        .theme(theme.clone())
                        .start(
                            sapid::input_addon()
                                .theme(theme.clone())
                                .content("https://"),
                        )
                        .input(
                            sapid::text_field()
                                .field(field)
                                .default_value("studio.design"),
                        )
                        .end(
                            sapid::input_section().theme(theme.clone()).fill(
                                sapid::button()
                                    .theme(theme.clone())
                                    .variant(sapid::ButtonVariant::Secondary)
                                    .size(sapid::Size::Small)
                                    .label("Check"),
                            ),
                        )
                }),
        ),
        _ => unreachable!("unsupported fields demo"),
    }
}
