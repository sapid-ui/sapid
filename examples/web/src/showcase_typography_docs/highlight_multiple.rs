use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    glory::ui::view().fill(
        sapid::highlight()
            .theme(theme.clone())
            .content(TEXT_1)
            .queries([
                "design".to_owned(),
                "design work".to_owned(),
                "system".to_owned(),
            ]),
    )
}

const TEXT_1: &str = "Design systems connect design work, design reviews and system documentation.";
