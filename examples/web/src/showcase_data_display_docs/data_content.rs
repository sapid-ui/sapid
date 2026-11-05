use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let shown = scope.cage(false);
        sapid::data_list()
            .theme(theme.clone())
            .label_width(86.)
            .name("Shipment details")
            .item(
                sapid::data_list_item()
                    .term("Delivery")
                    .fill(sapid::text().theme(theme.clone()).content(TEXT_1)),
            )
            .item(
                sapid::data_list_item().term("State").fill(
                    sapid::badge()
                        .theme(theme.clone())
                        .left_section(sapid::Icon::Check)
                        .content("Confirmed"),
                ),
            )
            .item(
                sapid::data_list_item().term("Details").fill(
                    sapid::button()
                        .theme(theme.clone())
                        .variant(sapid::ButtonVariant::Outline)
                        .label("Show delivery instructions")
                        .on_press(move || shown.revise(|v| *v = !*v)),
                ),
            )
            .item(sapid::data_list_item().term("Note").fill(
                sapid::text().theme(theme.clone()).content(shown.map(|v| {
                    if *v {
                        "Ask reception to notify Ava when the package arrives.".to_owned()
                    } else {
                        "No extra instructions shown.".to_owned()
                    }
                })),
            ))
            .show_in(scope)
    })
}

const TEXT_1: &str =
    "Leave the package with the reception desk near the main entrance, Monday through Friday.";
