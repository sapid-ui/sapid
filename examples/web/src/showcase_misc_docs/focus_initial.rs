use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let active = scope.cage(false);
        let preferred = scope.cage(None::<glory::render::SemanticId>);
        let configuration = scope.bond(move || glory::render::FocusScopeConfig {
            initial_focus: preferred.get(),
            ..Default::default()
        });
        sapid::stack()
            .theme(theme.clone())
            .gap(14.)
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .label("Edit shipping details")
                    .on_press(move || active.revise(|v| *v = true)),
                sapid::focus_scope()
                    .configuration(configuration)
                    .active(active)
                    .loop_tab(true)
                    .contain_focus(true)
                    .auto_focus(true)
                    .restore_focus(true)
                    .class("shipping-focus-form")
                    .style(Style {
                        border_color: Some(theme.border),
                        background: Some(theme.surface),
                        ..Style::new()
                            .display(glory::style::Display::Flex)
                            .flex_direction(glory::style::FlexDirection::Column)
                            .gap(12.)
                            .padding(Edges::all(16.))
                            .border_width(Edges::all(1.))
                            .border_style(glory::style::BorderStyle::Solid)
                            .radius(8.)
                    })
                    .fill((
                        sapid::field("shipping-initial-name")
                            .theme(theme.clone())
                            .label("Shipping name")
                            .control(|field| {
                                sapid::text_field().field(field).default_value("Ava Patel")
                            }),
                        sapid::field("shipping-initial-address")
                            .theme(theme.clone())
                            .label("Shipping address")
                            .control(move |field| {
                                preferred.revise(|id| *id = Some(field.id.clone()));
                                sapid::text_field()
                                    .field(field)
                                    .default_value("Mountain studio reception")
                            }),
                        sapid::button()
                            .theme(theme.clone())
                            .label("Done editing shipping")
                            .on_press(move || active.revise(|v| *v = false)),
                    )),
                sapid::text()
                    .theme(theme.clone())
                    .variant(sapid::TextVariant::Small)
                    .content(active.map(|v| {
                        if *v {
                            "Tab stays in the shipping form. Choose Done editing to return."
                                .to_owned()
                        } else {
                            "Choose Edit shipping details to focus the address field.".to_owned()
                        }
                    })),
            ))
            .show_in(scope)
    })
}
