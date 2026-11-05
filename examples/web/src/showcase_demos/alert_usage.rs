use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let visible = scope.cage(true);
        sapid::stack().theme(theme.clone()).gap(12.).fill((
            sapid::alert("storage-notice").theme(theme.clone()).title("Your storage is nearly full")
                .content("You have used 8 GB of your 10 GB workspace. Remove old files or review your storage plan.")
                .severity(sapid::AlertSeverity::Warning).announcement(sapid::Announcement::Off)
                .icon(glory::ui::graphic().drawing(sapid::Icon::Info.drawing(theme.warning)).style(Style::new().width(Length::Px(20.)).height(Length::Px(20.))))
                .closable(true).close_label("Dismiss storage notice").visible(visible)
                .on_close(move||visible.revise(|v|*v=false)),
            sapid::button().theme(theme.clone()).label("Restore storage notice").enabled(visible.map(|v|!*v)).on_press(move||visible.revise(|v|*v=true)),
        )).show_in(scope)
    })
}
