use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::typography().theme(theme.clone()).content(move |cx| {
        (
            cx.title(2)
                .weight(400.)
                .content("Designing a calmer workspace"),
            cx.text()
                .variant(sapid::TextVariant::Lead)
                .content("A practical guide to clear, consistent project work."),
            cx.text().content(TEXT_1),
            cx.title(3)
                .weight(400.)
                .content("Start with a few shared patterns"),
            sapid::text_list()
                .theme(cx.theme.clone())
                .size(cx.size)
                .items(
                    [
                        "Use clear names for projects and actions",
                        "Keep related details together",
                        "Show what changed after an action",
                    ]
                    .into_iter()
                    .map(|label| sapid::text_list_item().fill(cx.text().content(label))),
                ),
            sapid::blockquote()
                .theme(cx.theme.clone())
                .fill(cx.text().content("Make the next step easy to understand."))
                .citation(
                    cx.text()
                        .variant(sapid::TextVariant::Small)
                        .content("— Mountain studio working notes"),
                ),
            cx.title(3)
                .weight(400.)
                .content("Keep the details available"),
            cx.text().content(TEXT_2),
            cx.anchor()
                .label("Read about explicit typography scope")
                .href("#typography-scope"),
        )
    })
}

const TEXT_1: &str = "A useful workspace helps people understand the current project, find their next action and return to an unfinished draft without losing context.";
const TEXT_2: &str = "Long descriptions should wrap naturally. Important values deserve readable labels, and supporting links should describe the destination.";
