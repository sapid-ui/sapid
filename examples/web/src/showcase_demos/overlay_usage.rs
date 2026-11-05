use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let visible = scope.cage(true);
        let actions = scope.cage(0usize);
        sapid::stack().theme(theme.clone()).gap(16.).fill((
   sapid::overlay().theme(theme.clone()).visible(visible).color(Color(0,0,0,255)).opacity(0.35)
    .fill(sapid::card().theme(theme.clone()).fill(sapid::stack().theme(theme.clone()).gap(12.).fill((
     sapid::text().theme(theme.clone()).content("A preview beneath a decorative shade"),
     sapid::text().theme(theme.clone()).content("This layer changes presentation while its content remains operable."),
     sapid::button().theme(theme.clone()).label("Try the preview action").on_press(move||actions.revise(|v|*v+=1)),
    )))),
   sapid::checkbox().theme(theme.clone()).label("Show decorative shade").checked(visible).on_checked_change(move|v|visible.revise(|old|*old=v)),
   sapid::text().theme(theme.clone()).content(actions.map(|n|format!("Preview actions: {n}"))),
  )).show_in(scope)
    })
}
