use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::splitter("docs-nested-outer")
        .theme(theme.clone())
        .name("Resize workspace columns")
        .default_value(40.)
        .min(20.)
        .max(80.)
        .style(Style::new().height(Length::Px(260.)))
        .first({
            let theme = theme.clone();
            move || {
                sapid::box_()
                    .theme(theme.clone())
                    .padding(Edges::all(12.))
                    .fill(sapid::text().theme(theme.clone()).content("Project files"))
            }
        })
        .second(move || {
            sapid::splitter("docs-nested-inner")
                .theme(theme.clone())
                .name("Resize editor and terminal")
                .axis(glory::render::Orientation::Vertical)
                .default_value(65.)
                .min(20.)
                .max(80.)
                .style(Style::new().height(Length::Percent(100.)))
                .first({
                    let theme = theme.clone();
                    move || {
                        sapid::box_()
                            .theme(theme.clone())
                            .padding(Edges::all(12.))
                            .fill(sapid::text().theme(theme.clone()).content("Editor"))
                    }
                })
                .second({
                    let theme = theme.clone();
                    move || {
                        sapid::box_()
                            .theme(theme.clone())
                            .padding(Edges::all(12.))
                            .fill(sapid::text().theme(theme.clone()).content("Terminal"))
                    }
                })
        })
}
