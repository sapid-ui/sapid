use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill((
  sapid::alert("no-title").theme(theme.clone()).announcement(sapid::Announcement::Off).name("Saved settings notice").content("Your settings have been saved. This notice deliberately has no title.").closable(true),
  sapid::alert("long-message").theme(theme.clone()).announcement(sapid::Announcement::Off).title("Review the workspace storage settings before inviting the rest of your team")
   .content("Long content wraps in the flexible message column. The icon and compact close action retain their own dimensions, leaving the full message readable at narrow widths.")
   .icon(glory::ui::graphic().drawing(sapid::Icon::Info.drawing(theme.accent_text)).style(Style::new().width(Length::Px(20.)).height(Length::Px(20.)))).closable(true),
 ))
}
