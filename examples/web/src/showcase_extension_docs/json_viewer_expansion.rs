use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let accept = scope.cage(false);
        let paths = scope.cage(vec![String::new()]);
        sapid::stack().theme(theme.clone()).fill((sapid::switch().theme(theme.clone()).label("Accept tree expansion").checked(accept).on_checked_change(move|v|accept.revise(|o|*o=v)),sapid::json_viewer("controlled-json").theme(theme.clone()).tree_navigation(true).source("{\"project\":{\"name\":\"Sapid\",\"active\":true},\"members\":[\"Ari\",\"Lin\"],\"release\":1}").expanded(paths).on_expanded_change(move|v|{if accept.get(){paths.revise(|o|*o=v)}}),sapid::text().theme(theme.clone()).content(paths.map(|p|format!("Expanded paths: {p:?}"))))).show_in(scope)
    })
}
