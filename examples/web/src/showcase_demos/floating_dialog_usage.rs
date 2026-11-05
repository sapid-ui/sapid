use glory::prelude::*;
use glory::render::ViewportPlacement;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let note = scope.cage("Continue editing here".to_owned());
        let content_theme = theme.clone();
        sapid::stack().theme(theme.clone()).gap(12.).fill((
   sapid::text_field().theme(theme.clone()).name("Background document note").value(note).on_value_change(move|v|note.revise(|old|*old=v)),
   sapid::floating_dialog("background-sync").theme(theme.clone()).label("Show background sync notice").title("Background sync")
    .description("Your changes have been saved. You can continue editing the document.").close_label("Dismiss sync notice")
    .position(ViewportPlacement{x:24.,y:120.,..Default::default()}).style(Style::new().width(Length::Px(300.)))
    .content(move||sapid::text().theme(content_theme.clone()).content("This nonmodal notice preserves the current focus and leaves the page available.")),
  )).show_in(scope)
    })
}
