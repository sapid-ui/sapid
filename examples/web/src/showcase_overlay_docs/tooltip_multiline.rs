use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::tooltip("multiline-help").theme(theme).label("Read publishing help").text("Your draft remains private.\nPublish it when the title, summary and required project details are ready. You can keep editing it before publication.").style(Style::new().width(Length::Px(260.)).white_space(glory::style::WhiteSpace::PreWrap))
}
