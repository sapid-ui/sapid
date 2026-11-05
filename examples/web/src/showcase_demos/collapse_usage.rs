use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        let status = scope.cage("Add instructions for a smooth delivery.".to_owned());
        sapid::stack()
            .theme(theme.clone())
            .gap(12.)
            .fill((
                sapid::collapsible("delivery-details")
                    .theme(theme.clone())
                    .label("Delivery details")
                    .open(open)
                    .on_open_change(move |v| open.revise(|s| *s = v))
                    .height_transition(240)
                    .content_policy(sapid::ContentPolicy::KeepMounted)
                    .on_transition_end(move |v| {
                        status.revise(|s| {
                            *s = if v {
                                "Delivery details are ready.".to_owned()
                            } else {
                                "Delivery details are closed.".to_owned()
                            }
                        })
                    })
                    .trigger_content({
                        let ink = theme.foreground;
                        move || {
                            glory::ui::group().fill((
                                glory::ui::graphic()
                                    .drawing(open.map(move |v| {
                                        if *v {
                                            sapid::Icon::ChevronDown.drawing(ink)
                                        } else {
                                            sapid::Icon::ChevronRight.drawing(ink)
                                        }
                                    }))
                                    .style(
                                        Style::new()
                                            .width(Length::Px(16.))
                                            .height(Length::Px(16.))
                                            .margin(Edges {
                                                right: 6.,
                                                ..Default::default()
                                            }),
                                    ),
                                glory::ui::text().content("Delivery details"),
                            ))
                        }
                    })
                    .content({
                        let theme = theme.clone();
                        move || {
                            sapid::field("delivery-instructions")
                                .theme(theme.clone())
                                .label("Delivery instructions")
                                .control(|field| {
                                    sapid::text_field()
                                        .field(field)
                                        .placeholder("Where should we leave your package?")
                                })
                        }
                    }),
                sapid::text()
                    .theme(theme.clone())
                    .variant(sapid::TextVariant::Small)
                    .content(status),
            ))
            .show_in(scope)
    })
}
