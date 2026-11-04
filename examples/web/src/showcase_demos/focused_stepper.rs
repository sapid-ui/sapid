use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("Stepper", theme)
}

#[allow(dead_code, unused_imports)]
mod focused {
    use glory::prelude::*;
    use glory::render::{FileConfig, Orientation};
    use glory::style::{FlexWrap, ObjectFit};
    use glory::ui::text;
    use sapid::prelude::*;
    use sapid::{
        Icon, ThemeIconVariant, autocomplete_option, background_image, button, file_button,
        file_input, group, image, indicator, progress, ring_progress, select_group, select_option,
        semi_circle_progress, theme_icon,
    };
    use std::{boxed::Box, cell::RefCell, rc::Rc, time::Duration};

    fn stepper_demo(theme: Theme) -> impl Widget {
        widget(move |scope| {
            let active = scope.cage(1usize);
            let selectable = scope.cage(true);
            let skip = scope.cage(false);
            let back_enabled = active.map(|active| *active > 0);
            let next_enabled = active.map(|active| *active < 3);
            let content_theme = theme.clone();
            let account = step("account").label("Account").content({
                let theme = content_theme.clone();
                move || {
                    sapid::text()
                        .theme(theme.clone())
                        .content("Step 1 content: Create an account")
                }
            });
            let details = step("details")
                .label("Details")
                .description("Verify your email")
                .optional(true)
                .content({
                    let theme = content_theme.clone();
                    move || {
                        sapid::text()
                            .theme(theme.clone())
                            .content("Step 2 content: Verify email")
                    }
                });
            let review = step("review").label("Review").content({
                let theme = content_theme.clone();
                move || {
                    sapid::text()
                        .theme(theme.clone())
                        .content("Step 3 content: Confirm your details")
                }
            });
            sapid::stack()
                .theme(theme.clone())
                .gap(24.0)
                .fill((
                    stepper("showcase-workflow")
                        .theme(theme.clone())
                        .class("site-stepper-demo")
                        .active(active)
                        .selectable(selectable)
                        .allow_skip(skip)
                        .on_step_change(move |next| active.revise(|value| *value = next))
                        .steps([account, details, review])
                        .completed({
                            let theme = content_theme.clone();
                            move || {
                                sapid::text()
                                    .theme(theme.clone())
                                    .content("Workflow completed")
                            }
                        }),
                    sapid::group().theme(theme.clone()).fill((
                        sapid::button()
                            .theme(theme.clone())
                            .label("Back")
                            .enabled(back_enabled)
                            .on_press(move || {
                                active.revise(|value| *value = value.saturating_sub(1))
                            }),
                        sapid::button()
                            .theme(theme.clone())
                            .label("Next")
                            .enabled(next_enabled)
                            .on_press(move || active.revise(|value| *value = (*value + 1).min(3))),
                        sapid::checkbox()
                            .theme(theme.clone())
                            .label("Allow step selection")
                            .checked(selectable)
                            .on_checked_change(move |next| {
                                selectable.revise(|value| *value = next)
                            }),
                        sapid::checkbox()
                            .theme(theme.clone())
                            .label("Allow skipping steps")
                            .checked(skip)
                            .on_checked_change(move |next| skip.revise(|value| *value = next)),
                    )),
                    sapid::title(3)
                        .theme(theme.clone())
                        .content("Vertical orientation"),
                    stepper("showcase-vertical")
                        .theme(theme.clone())
                        .name("Read-only progress")
                        .orientation(Orientation::Vertical)
                        .default_active(1)
                        .steps([
                            step("received")
                                .label("Received")
                                .description("Request received"),
                            step("processing")
                                .label("Processing")
                                .description("Waiting for confirmation"),
                            step("delivered")
                                .label("Delivered")
                                .description("Deliver the result"),
                        ]),
                ))
                .show_in(scope)
        })
    }

    pub fn demo_for(name: &str, theme: Theme) -> Box<dyn Widget> {
        match name {
            "Stepper" => Box::new(stepper_demo(theme)),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
