use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::app_shell("docs-shell-regions")
        .theme(theme)
        .style(Style::new().min_height(Length::Px(240.)))
        .header(|context| sapid::text().theme(context.theme).content("Header"))
        .main(|context| {
            sapid::text()
                .theme(context.theme)
                .content("Main content fills the available space.")
        })
        .footer(|context| sapid::text().theme(context.theme).content("Footer"))
}
