use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::select("framework-popover")
        .theme(theme.clone())
        .name("Choose framework")
        .placeholder("Select framework")
        .style(Style::new().gap(8.))
        .options([
            sapid::select_option("glory", "Glory"),
            sapid::select_option("dioxus", "Dioxus"),
            sapid::select_option("leptos", "Leptos"),
        ])
        .trigger_content(move |context| {
            glory::ui::group().fill((
                glory::ui::graphic()
                    .drawing(sapid::Icon::Info.drawing(theme.accent_text))
                    .style(Style::new().width(Length::Px(16.)).height(Length::Px(16.))),
                text().content(context.label).selectable(false),
            ))
        })
}
