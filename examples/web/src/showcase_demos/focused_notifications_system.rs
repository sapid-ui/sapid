use glory::prelude::*;

pub fn demo(theme: sapid::Theme) -> impl Widget {
    focused::demo_for("Notifications system", theme)
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

    pub fn demo_for(name: &str, theme: Theme) -> Box<dyn Widget> {
        match name {
            "Notifications system" => Box::new(widget(move |scope| {
                let handle = Rc::new(RefCell::new(None::<NotificationsHandle>));
                let target = handle.clone();
                let clock = NotificationClock::new(|| Duration::ZERO, |_| std::future::pending());
                sapid::stack()
                    .theme(theme.clone())
                    .gap(12.0)
                    .fill((
                        sapid::button()
                            .theme(theme.clone())
                            .label("Show notification")
                            .on_press(move || {
                                if let Some(handle) = target.borrow().clone() {
                                    let _ = handle.show(notification_request(
                                        "saved",
                                        "Changes saved",
                                        "Your changes are stored on this device.",
                                    ));
                                }
                            }),
                        toaster("showcase-notifications")
                            .theme(theme.clone())
                            .capacity(8)
                            .visible_limit(4)
                            .clock(clock)
                            .on_ready(move |value| *handle.borrow_mut() = Some(value)),
                    ))
                    .show_in(scope)
            })),
            _ => unreachable!("unsupported named demo: {name}"),
        }
    }
}
