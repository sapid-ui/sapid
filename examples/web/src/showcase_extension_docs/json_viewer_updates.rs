use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let query = scope.cage(String::new());
        let source=scope.cage("{\"project\":{\"name\":\"Sapid\",\"active\":true},\"members\":[\"Ari\",\"Lin\"],\"release\":1}".to_string());
        sapid::stack().theme(theme.clone()).fill((sapid::text_field().theme(theme.clone()).name("Search configuration").placeholder("Search keys and values").value(query).on_value_change(move|v|query.revise(|o|*o=v)),sapid::button().theme(theme.clone()).label("Publish release 2").on_press(move||source.revise(|o|*o="{\"project\":{\"name\":\"Sapid\",\"active\":true},\"members\":[\"Ari\",\"Lin\"],\"release\":2}".into())),sapid::json_viewer("updated-json").theme(theme.clone()).tree_navigation(true).source(source).search(query).default_expanded([String::new(),"/project".into()]))).show_in(scope)
    })
}
