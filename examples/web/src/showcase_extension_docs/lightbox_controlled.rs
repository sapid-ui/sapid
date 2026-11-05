use glory::prelude::*;
use sapid::prelude::*;
fn media(name: &str) -> String {
    format!(
        "{}/{name}",
        option_env!("SAPID_SHOWCASE_MEDIA").unwrap_or("/assets")
    )
}
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let images = scope.cage(vec![
            sapid::lightbox_image(
                "lake",
                media("alpine-lake-photo.png"),
                "A fictional alpine village beside a lake",
            )
            .caption("Morning beside an imaginary alpine lake"),
            sapid::lightbox_image(
                "portrait",
                media("team-member-photo.png"),
                "A fictional team member in a workspace",
            )
            .caption("A fictional team portrait"),
        ]);
        let view = scope.cage(LightboxView::default());
        let open = scope.cage(false);
        let accept = scope.cage(true);
        let first_close = scope.cage(true);
        let feedback = scope.cage("Choose a photo".to_string());
        let _ = (accept, first_close);
        let mut cards = Vec::new();
        for (key, file, label) in [
            ("lake", "alpine-lake-photo.png", "Open lake photo"),
            ("portrait", "team-member-photo.png", "Open portrait photo"),
        ] {
            cards.push(
                sapid::stack()
                    .theme(theme.clone())
                    .style(Style::new().width(Length::Px(140.)))
                    .fill((
                        sapid::image(media(file))
                            .theme(theme.clone())
                            .alt(if key == "lake" {
                                "Alpine lake thumbnail"
                            } else {
                                "Team portrait thumbnail"
                            })
                            .fit(glory::style::ObjectFit::Cover)
                            .style(Style::new().width(Length::Px(140.)).height(Length::Px(94.))),
                        sapid::button()
                            .theme(theme.clone())
                            .label(label)
                            .on_press(move || {
                                view.revise(|o| {
                                    *o = LightboxView {
                                        current: Some(key.into()),
                                        zoom: LightboxZoom::Fit,
                                    }
                                });
                                open.revise(|o| *o = true);
                            }),
                    )),
            )
        }
        sapid::stack()
            .theme(theme.clone())
            .fill((
                sapid::group()
                    .theme(theme.clone())
                    .wrap(glory::style::FlexWrap::Wrap)
                    .fill(cards),
                sapid::switch()
                    .theme(theme.clone())
                    .label("Accept image and zoom changes")
                    .checked(accept)
                    .on_checked_change(move |v| accept.revise(|o| *o = v)),
                sapid::lightbox("photos-controlled")
                    .theme(theme.clone())
                    .show_trigger(false)
                    .title("Project photo gallery")
                    .images(images)
                    .view(view)
                    .open(open)
                    .on_view_change(move |next| {
                        if accept.get() {
                            view.revise(|o| *o = next)
                        }
                    })
                    .on_open_change(move |next| {
                        if !next && first_close.get() {
                            first_close.revise(|o| *o = false);
                            feedback.revise(|o| {
                                *o = "First close refused; close again to leave the gallery.".into()
                            });
                        } else {
                            open.revise(|o| *o = next);
                        }
                    }),
                sapid::text().theme(theme.clone()).content(feedback),
            ))
            .show_in(scope)
    })
}
