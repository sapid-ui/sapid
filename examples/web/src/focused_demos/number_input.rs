use glory::prelude::*;
use sapid::{Decimal, NumberValue, Theme};

pub fn demo_for(id: &str, theme: Theme) -> Box<dyn Widget> {
    assert_eq!(id, "P7-01");
    Box::new(widget(move |scope| {
        let value = scope.cage(NumberValue::Decimal(
            Decimal::parse("1234.5").expect("valid showcase number"),
        ));
        let variant = scope.cage(Some("default".to_owned()));
        let size = scope.cage(Some("md".to_owned()));
        let radius = scope.cage(4.0);
        let label = scope.cage("Amount".to_owned());
        let description = scope.cage("Enter a value between 0 and 10,000.".to_owned());
        let error = scope.cage(String::new());
        let required = scope.cage(true);
        let grouping = scope.cage(true);
        let controls = scope.cage(true);
        let readonly = scope.cage(false);
        let disabled = scope.cage(false);
        let enabled = scope.bond(move || !disabled.get());
        let revision = scope.bond(move || {
            vec![format!(
                "{}-{}-{}-{}-{}",
                variant.get().unwrap_or_default(),
                size.get().unwrap_or_default(),
                radius.get(),
                grouping.get(),
                controls.get()
            )]
        });
        let preview_theme = theme.clone();
        let preview = each(revision)
            .key(|revision| revision.clone())
            .children(move |_| {
                let size = super::controls::size_value(&size.get().unwrap_or_default());
                let grouping = grouping.get();
                let controls = controls.get();
                let unstyled = variant.get().as_deref() == Some("unstyled");
                let mut preview_theme = preview_theme.clone();
                preview_theme.radius = radius.get();
                if variant.get().as_deref() == Some("filled") {
                    preview_theme.input = preview_theme.disabled;
                }
                sapid::field("focused-number-amount")
                    .theme(preview_theme.clone())
                    .label(label)
                    .description(description)
                    .error(error)
                    .required(required)
                    .readonly(readonly)
                    .enabled(enabled)
                    .control(move |field| {
                        let mut input = sapid::number_input()
                            .field(field)
                            .size(size)
                            .value(value)
                            .min(Decimal::parse("0").ok())
                            .max(Decimal::parse("10000").ok())
                            .step(Decimal::parse("0.5").expect("valid showcase step"))
                            .grouping(grouping)
                            .controls(controls)
                            .on_value_change(move |next| value.revise(|current| *current = next))
                            .on_commit(move |next| value.revise(|current| *current = next));
                        if unstyled {
                            input = input.unstyled();
                        }
                        input
                    })
            });
        let controls = vec![
            super::controls::variant(variant, &theme),
            super::controls::size_slider(size, &theme),
            super::controls::radius_slider(radius, &theme),
            super::controls::text_field("Label", label, &theme),
            super::controls::switch("With asterisk", required, &theme),
            super::controls::text_field("Description", description, &theme),
            super::controls::text_field("Error", error, &theme),
            super::controls::switch("Thousand separator", grouping, &theme),
            super::controls::switch("Step controls", controls, &theme),
            super::controls::switch("Read only", readonly, &theme),
            super::controls::switch("Disabled", disabled, &theme),
        ];
        super::controls::playground(Box::new(preview), controls).show_in(scope)
    }))
}
