use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let active = scope.cage(1_usize);
        let content_theme = theme.clone();
        sapid::stack().theme(theme.clone()).gap(16.).fill((
            sapid::stepper("create-workspace").theme(theme.clone()).name("Create a workspace")
                .active(active).selectable(true)
                .on_step_change(move |value| active.revise(|old| *old=value))
                .steps([
                    sapid::step("account").label("Account").description("Create an account")
                        .content({let theme=content_theme.clone();move || sapid::text().theme(theme.clone()).content("Your account is ready. Continue to verify your email address.")}),
                    sapid::step("verify").label("Verify").description("Confirm your email")
                        .content({let theme=content_theme.clone();move || sapid::text().theme(theme.clone()).content("Open the verification link in your inbox, then continue to your workspace.")}),
                    sapid::step("workspace").label("Workspace").description("Start your project")
                        .content({let theme=content_theme.clone();move || sapid::text().theme(theme.clone()).content("Choose the project settings for your new workspace.")}),
                ])
                .completed(move || sapid::text().theme(content_theme.clone()).content("Your workspace is ready.")),
            sapid::group().theme(theme.clone()).gap(8.).fill((
                sapid::button().theme(theme.clone()).label("Previous step").enabled(active.map(|v| *v>0))
                    .on_press(move || active.revise(|v| *v=v.saturating_sub(1))),
                sapid::button().theme(theme.clone()).label("Next step").enabled(active.map(|v| *v<3))
                    .on_press(move || active.revise(|v| *v=(*v+1).min(3))),
            )),
        )).show_in(scope)
    })
}
