use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let saved = scope.cage(false);
        let text = theme.typography(sapid::TypographyRole::Control, sapid::Size::Medium);
        let control = theme.tokens.control(sapid::Size::Medium);
        let target_style = Style {
            font_size: Some(text.font_size),
            line_height: Some(text.line_height),
            font_weight: text.weight,
            min_height: Some(Length::Px(control.min_height)),
            font_family: Some(vec![glory::style::Family::System]),
            color: Some(theme.foreground),
            background: Some(theme.surface),
            border_color: Some(theme.border),
            ..Style::new()
                .padding(Edges::all(8.))
                .border_width(Edges::all(1.))
                .border_style(BorderStyle::Solid)
                .radius(theme.radius)
        };
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::tooltip("save-draft-help")
                    .theme(theme.clone())
                    .text("Your draft is saved privately until you publish it.")
                    .target(move |target| {
                        target.button(
                            glory::ui::button()
                                .label("Save draft")
                                .style(target_style.clone())
                                .on(glory::render::EventKind::Activate, move |_| {
                                    saved.revise(|v| *v = true)
                                }),
                        )
                    }),
                sapid::text().theme(theme.clone()).content(saved.map(|v| {
                    if *v {
                        "Draft saved privately.".to_owned()
                    } else {
                        "Hover or focus Save draft for an explanation.".to_owned()
                    }
                })),
            ))
            .show_in(scope)
    })
}
