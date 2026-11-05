use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::autocomplete("autocomplete-limited")
        .theme(theme)
        .label("Contact directory")
        .placeholder("Type Contact or a number")
        .filter(sapid::ComboboxFilter::Prefix)
        .limit(5)
        .options(
            (0..100).map(|i| {
                sapid::autocomplete_option(format!("contact-{i}"), format!("Contact {i:02}"))
            }),
        )
        .empty_message("No contacts match this prefix")
}
