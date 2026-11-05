use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let body = theme.clone();
    sapid::popover("popover-content").theme(theme).label("Open project notes").title("Project notes").content_policy(sapid::ContentPolicy::KeepMounted).width(sapid::PopoverWidth::Px(320.)).content(move||sapid::stack().theme(body.clone()).gap(12.).fill((sapid::text_field().theme(body.clone()).name("Retained popover note").default_value("Keep this note"),view().class("overlay-doc-scroll").style(Style::new().height(Length::Px(160.)).overflow_y(Overflow::Auto)).fill(sapid::stack().theme(body.clone()).gap(14.).fill((1..=8).map(|i|sapid::text().theme(body.clone()).content(format!("Note {i}: Share a concise update with the rest of your project team."))).collect::<Vec<_>>())))))
}
