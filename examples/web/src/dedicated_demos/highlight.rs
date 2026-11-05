use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    glory::ui::view().fill(
        sapid::highlight()
            .theme(theme.clone())
            .content(TEXT_1)
            .queries(["design".to_owned(), "important".to_owned()]),
    )
}

const TEXT_1: &str = "Consistent design makes important details easy to find. A shared design language helps every team.";
