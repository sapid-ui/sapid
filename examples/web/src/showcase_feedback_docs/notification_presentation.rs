use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(16.).fill((
  sapid::notification("notice-success").theme(theme.clone()).severity(sapid::AlertSeverity::Success).title("Import completed").content("The sample records are ready to review.")
   .icon(glory::ui::graphic().drawing(sapid::Icon::Check.drawing(theme.success)).style(Style::new().width(Length::Px(20.)).height(Length::Px(20.)))),
  sapid::notification("notice-outline").theme(theme.clone()).variant(sapid::AlertVariant::Outline).title("A persistent notice").content("This notice has a border and deliberately has no close action.").closable(false),
  sapid::notification("notice-no-title").theme(theme.clone()).content("A notification can omit its title while retaining an independently named close action.").close_label("Dismiss untitled notification"),
  sapid::notification("notice-long").theme(theme).title("The latest workspace configuration has been saved and is available to your collaborators").content("A longer message wraps within the flexible body. The close action stays compact and the notification does not introduce its own queue, transport or timeout."),
 ))
}
