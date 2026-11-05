use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let error = scope.cage(true);
        sapid::stack()
            .theme(theme.clone())
            .gap(20.)
            .fill((
                sapid::stepper("status-workflow")
                    .theme(theme.clone())
                    .name("Workspace review")
                    .default_active(1)
                    .steps([
                        sapid::step("identity")
                            .label("Identity")
                            .description("Completed"),
                        sapid::step("billing")
                            .label("Billing")
                            .description("Check payment details")
                            .status(error.map(|v| {
                                if *v {
                                    Some(sapid::StepStatus::Error)
                                } else {
                                    None
                                }
                            })),
                        sapid::step("invite")
                            .label("Invite")
                            .description("Add teammates")
                            .optional(true),
                    ]),
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Payment needs attention")
                    .checked(error)
                    .on_checked_change(move |v| error.revise(|old| *old = v)),
                sapid::stack().theme(theme.clone()).gap(16.).fill(
                    [
                        sapid::Size::ExtraSmall,
                        sapid::Size::Small,
                        sapid::Size::Medium,
                        sapid::Size::Large,
                        sapid::Size::ExtraLarge,
                    ]
                    .into_iter()
                    .enumerate()
                    .map(|(i, size)| {
                        sapid::stepper(format!("size-{i}"))
                            .theme(theme.clone())
                            .size(size)
                            .name(format!("Workflow size {}", i + 1))
                            .default_active(1)
                            .steps([
                                sapid::step("first").label("First"),
                                sapid::step("second").label("Second"),
                                sapid::step("third").label("Third"),
                            ])
                    })
                    .collect::<Vec<_>>(),
                ),
            ))
            .show_in(scope)
    })
}
