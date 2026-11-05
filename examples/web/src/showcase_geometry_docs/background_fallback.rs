use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let invalid = scope.cage(false);
        let source = scope.bond(move || {
            format!(
                "{}/{}",
                option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets"),
                if invalid.get() {
                    "image-invalid.png"
                } else {
                    "mountain-evening.svg"
                }
            )
        });
        sapid::stack()
            .theme(theme.clone())
            .gap(10.)
            .fill((
                sapid::checkbox()
                    .theme(theme.clone())
                    .label("Use an invalid primary image")
                    .checked(invalid)
                    .on_checked_change(move |next| invalid.revise(|old| *old = next)),
                sapid::background_image(source)
                    .theme(theme.clone())
                    .fallback_src(format!(
                        "{}/mountain-evening.svg",
                        option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets")
                    ))
                    .height(Length::Px(170.))
                    .fill(
                        glory::ui::view()
                            .style(Style {
                                background: Some(Color(0, 12, 24, 130)),
                                ..Style::new()
                                    .height(Length::Px(170.))
                                    .padding(Edges::all(24.))
                            })
                            .fill(
                                sapid::button()
                                    .theme(theme.clone())
                                    .label("Foreground action"),
                            ),
                    ),
            ))
            .show_in(scope)
    })
}
