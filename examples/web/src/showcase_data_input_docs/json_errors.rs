use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(20.).fill((
        sapid::field("json-syntax")
            .theme(theme.clone())
            .label("Editable syntax error")
            .description("The parser reports the line and column; malformed text remains editable.")
            .control(|field| {
                sapid::json_input("json-syntax-editor")
                    .field(field)
                    .rows(4)
                    .default_value("{\n  \"name\":\n}")
            }),
        sapid::field("json-budget")
            .theme(theme)
            .label("Bounded formatting")
            .description("Four-space indentation cannot fit this eight-byte output budget.")
            .control(|field| {
                sapid::json_input("json-budget-editor")
                    .field(field)
                    .rows(3)
                    .default_value("[1,2,3]")
                    .limits(sapid::JsonLimits { bytes: 8, depth: 8 })
                    .format(sapid::JsonFormat::PrettyFour)
            }),
    ))
}
