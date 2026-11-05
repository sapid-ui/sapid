//! cargo run --example dsl > target/verification/sapid/dsl.html
#[path = "fixtures/dsl_demo.rs"]
mod fixture;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let html = glory::ssr::render(fixture::demo(sapid::Theme::light()))?;
    println!("{html}");
    Ok(())
}
