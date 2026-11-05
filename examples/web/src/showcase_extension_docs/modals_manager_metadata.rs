use glory::prelude::*;
use sapid::prelude::*;
use std::{cell::RefCell, rc::Rc};
#[derive(Default)]
struct Current {
    handle: Option<ModalsHandle>,
    first: Option<ModalToken>,
}
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let control = Rc::new(RefCell::new(Current::default()));
        let ready = control.clone();
        let draft = scope.cage("Design workspace".to_string());
        let project = scope.cage("Design workspace".to_string());
        let status = scope.cage("Ready".to_string());
        sapid::stack()
            .theme(theme.clone())
            .fill((
                sapid::modals_manager("project-metadata")
                    .theme(theme.clone())
                    .on_ready(move |h| ready.borrow_mut().handle = Some(h)),
                sapid::text()
                    .theme(theme.clone())
                    .content(project.map(|name| format!("Project: {name}"))),
                sapid::group()
                    .theme(theme.clone())
                    .wrap(glory::style::FlexWrap::Wrap)
                    .fill(vec![{
                        let control = control.clone();
                        let palette = theme.clone();
                        let weak = scope.downgrade();
                        sapid::button()
                            .theme(theme.clone())
                            .label("Edit project")
                            .on_press(move || {
                                let handle = control.borrow().handle.clone();
                                let Some(handle) = handle else { return };
                                let result = (|| -> glory::ui::Result {
                                    let content_control = control.clone();
                                    let content_theme = palette.clone();
                                    let token =
                                        handle.open(
                                            sapid::modal_request(
                                                "Edit project",
                                                "Change the name for this example.",
                                            )
                                            .close_label("Cancel editing")
                                            .content(move || {
                                                form(
                                                    content_control.clone(),
                                                    content_theme.clone(),
                                                    draft,
                                                    project,
                                                    status,
                                                )
                                            })
                                            .on_result(move |outcome| {
                                                status
                                                    .revise(|o| *o = format!("Editor: {outcome:?}"))
                                            }),
                                        )?;
                                    control.borrow_mut().first = Some(token);
                                    Ok(())
                                })();
                                if let Err(error) = result
                                    && let Some(owner) = weak.upgrade()
                                {
                                    owner.report_error(error);
                                }
                            })
                    }]),
                sapid::text().theme(theme.clone()).content(status),
            ))
            .show_in(scope)
    })
}
fn form(
    control: Rc<RefCell<Current>>,
    theme: sapid::Theme,
    draft: Cage<String>,
    project: Cage<String>,
    status: Cage<String>,
) -> impl Widget {
    sapid::stack().theme(theme.clone()).fill((
        sapid::text_field()
            .theme(theme.clone())
            .name("Project name")
            .value(draft)
            .on_value_change(move |v| draft.revise(|o| *o = v)),
        {
            let control = control.clone();
            sapid::button()
                .theme(theme.clone())
                .label("Update dialog title")
                .on_press(move || {
                    let command = {
                        let c = control.borrow();
                        c.handle.clone().zip(c.first.clone())
                    };
                    if let Some((handle, token)) = command
                        && let Err(error) = handle.update(
                            &token,
                            "Project details",
                            "The editor and its draft are retained.",
                        )
                    {
                        status.revise(|o| *o = error.to_string());
                    }
                })
        },
        sapid::button()
            .theme(theme.clone())
            .label("Save project")
            .on_press(move || {
                let command = {
                    let c = control.borrow();
                    c.handle.clone().zip(c.first.clone())
                };
                if let Some((handle, token)) = command {
                    match handle.close(&token) {
                        Ok(true) => {
                            project.revise(|o| *o = draft.get());
                            status.revise(|o| *o = "Project saved".into());
                        }
                        Ok(false) => {}
                        Err(error) => status.revise(|o| *o = error.to_string()),
                    }
                }
            }),
    ))
}
