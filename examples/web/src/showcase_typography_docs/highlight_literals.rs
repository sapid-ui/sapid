use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    glory::ui::view().fill(
        sapid::highlight()
            .theme(theme.clone())
            .content(TEXT_1)
            .queries(["<Button>".to_owned(), ".*".to_owned(), "[label]".to_owned()]),
    )
}

const TEXT_1: &str =
    "Use <Button> for actions. The .* and [label] fragments are literal source text.";
