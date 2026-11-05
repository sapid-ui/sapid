use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::splitter("workspace-splitter")
        .theme(theme.clone())
        .name("Resize editor and preview")
        .default_value(50.)
        .min(20.)
        .max(80.)
        .style(Style::new().height(Length::Px(220.)))
        .first({
            let theme = theme.clone();
            move || {
                sapid::box_()
                    .theme(theme.clone())
                    .padding(Edges::all(12.))
                    .fill(
                        sapid::text_field()
                            .theme(theme.clone())
                            .name("Editor draft")
                            .placeholder("Write a draft…")
                            .style(Style::new().width(Length::Percent(100.))),
                    )
            }
        })
        .second(move || {
            sapid::box_()
                .theme(theme.clone())
                .padding(Edges::all(12.))
                .fill(sapid::text().theme(theme.clone()).content("Preview"))
        })
}
