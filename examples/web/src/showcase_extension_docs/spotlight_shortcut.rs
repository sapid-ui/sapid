use glory::prelude::*;
use sapid::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    widget(move |scope| {
        let outcome = scope.cage("No command chosen".to_string());
        let open = scope.cage(false);
        let query = scope.cage(String::new());
        let accept = scope.cage(false);
        let _ = accept;
        let root = scope.cage(None);
        let handle = scope.cage(None::<CommandHandle>);
        let weak = scope.downgrade();
        scope.effect(move || {
            if let (Some(root), Some(handle)) = (root.get(), handle.get())
                && let Err(error) = handle.register_shortcut(root)
                && let Some(owner) = weak.upgrade()
            {
                owner.report_error(error);
            }
        });
        sapid::stack()
            .theme(theme.clone())
            .node_ref(root)
            .fill((
                sapid::button()
                    .theme(theme.clone())
                    .label("Keyboard shortcut target"),
                sapid::command("commands-shortcut")
                    .theme(theme.clone())
                    .label("shortcut palette")
                    .open(open)
                    .search(query)
                    .on_open_change(move |v| open.revise(|o| *o = v))
                    .on_search_change(move |v| query.revise(|o| *o = v))
                    .on_ready(move |value| handle.revise(|o| *o = Some(value)))
                    .group(sapid::command_group("workspace", "Workspace"))
                    .group(sapid::command_group("preferences", "Preferences"))
                    .actions([
                        sapid::command_action("new-note", "Create note")
                            .group("workspace")
                            .description("Start a project note")
                            .keywords("new document fresh"),
                        sapid::command_action("reports", "Open reports")
                            .group("workspace")
                            .description("Review project activity"),
                        sapid::command_action("settings", "Workspace settings")
                            .group("preferences")
                            .keywords("theme language"),
                        sapid::command_action("archive", "Archive workspace")
                            .group("preferences")
                            .disabled(true),
                    ])
                    .on_action(move |key| {
                        outcome.revise(|o| {
                            *o = match key.as_str() {
                                "new-note" => "New note created",
                                "reports" => "Reports opened",
                                _ => "Workspace settings opened",
                            }
                            .into()
                        })
                    }),
                sapid::text().theme(theme.clone()).content(outcome),
            ))
            .show_in(scope)
    })
}
