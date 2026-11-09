//! Mount one real widget per probe; enabling a feature alone keeps the text UI.
#![cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "base")]
use glory::prelude::*;
#[cfg(feature = "base")]
thread_local! {
    static APP: std::cell::RefCell<Option<glory::web::WebApp>> = const { std::cell::RefCell::new(None) };
}

#[cfg(feature = "base")]
#[allow(unreachable_code)] // Each measured build selects at most one use case.
fn ui(scope: &glory::Scope) -> Box<dyn Widget> {
    let _ = scope;
    #[cfg(feature = "button")]
    {
        let count = scope.cage(0_u32);
        return Box::new(
            sapid::button()
                .label(count.map(|n| format!("Count {n}")))
                .on_press(move || count.revise(|n| *n += 1)),
        );
    }
    #[cfg(feature = "use-charts")]
    {
        use sapid::charts::*;
        return Box::new(line_chart("size-chart").name("Size chart").data(ChartData {
            categories: vec![ChartCategory::new("a", "A"), ChartCategory::new("b", "B")],
            series: vec![ChartSeries::new("sample", "Sample", [Some(1.), Some(2.)])],
        }));
    }
    #[cfg(feature = "use-code")]
    return Box::new(sapid::code_block().source("fn main() { println!(\"hello\"); }"));
    #[cfg(feature = "use-files")]
    return Box::new(
        sapid::file_button()
            .name("Choose probe file")
            .label("Choose probe file"),
    );
    #[cfg(feature = "use-editor")]
    return Box::new(sapid::rich_text_editor().name("Probe editor"));
    #[cfg(feature = "use-json")]
    return Box::new(
        sapid::json_input("size-json")
            .name("Probe JSON")
            .default_value("{\"ready\":true}"),
    );
    #[cfg(feature = "use-dates")]
    return Box::new(
        sapid::date_picker(
            "size-date",
            sapid::DatesProvider::new(sapid::CivilDate::parse_iso("2026-11-09").unwrap()),
        )
        .name("Probe date"),
    );
    Box::new(text().content("hello"))
}

#[wasm_bindgen]
pub fn mount() -> std::result::Result<(), JsValue> {
    #[cfg(feature = "base")]
    {
        let root = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("app")
            .unwrap();
        let app = glory::web::launch_with(root, ui)
            .map_err(|error| JsValue::from_str(&error.to_string()))?;
        APP.with(|slot| *slot.borrow_mut() = Some(app));
    }
    Ok(())
}

#[wasm_bindgen]
pub fn unmount() {
    #[cfg(feature = "base")]
    APP.with(|slot| drop(slot.borrow_mut().take()));
}
