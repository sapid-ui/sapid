use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    let body = theme.clone();
    sapid::dialog("scrolling-modal").theme(theme).label("Review project policies").title("Project policies").description("Read the policy sections below.").content(move||view().class("overlay-doc-scroll").style(Style::new().height(Length::Px(220.)).overflow_y(Overflow::Auto)).fill(sapid::stack().theme(body.clone()).gap(18.).fill((1..=12).map(|i|sapid::text().theme(body.clone()).content(format!("Policy {i}: Workspace members can review their assigned projects and keep shared documents up to date. Changes are recorded in project activity."))).collect::<Vec<_>>())))
}
