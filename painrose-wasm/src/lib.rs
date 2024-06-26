use painrose_lib::{geometry, language};
use wasm_bindgen::prelude::*;
#[macro_use]
mod macros;

const SVG_NAMESPACE: &'static str = "http://www.w3.org/2000/svg";

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let code = ":\"Hello World!\"I;
a-e:\"Goodbye All?\"C;";

    let program =
        language::LanguageState::<geometry::rhomb::RhombTiling>::new_from_string(code.to_string())
            .unwrap();

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_text_content(Some("Hello!! from Rust!"));

    let svg = svg_element!(
        document,
        "svg",
        {
            "viewBox" => "-5 -5 10 10",
            "width" => "800",
            "height" => "800",
        }
    );
    for shape in program.get_shapes() {
        let path = svg_element!(
            document,
            "path",
            {
                "d" => &shape.get_outline_d(),
                "fill" => shape.fill.unwrap_or("none"),
                "stroke" =>shape.stroke.unwrap_or("none"),
                "stroke-width" => &format!("{}", shape.stroke_width),
            }
        );

        svg.append_child(&path)?;

        let text = svg_element!(
            document,
            "text",
            {
                "x" => &format!("{}", shape.center.x),
                "y" => &format!("{}", shape.center.y),
                "font-size" => "0.5",
                "font-family" => "sans-serif",
                "fill" => shape.text_color,
                "text-anchor" => "middle",
                "dominant-baseline" => "middle",
            }
        );
        text.set_text_content(Some(&shape.label));
        svg.append_child(&text)?;
    }
    body.append_child(&svg)?;

    body.append_child(&val)?;

    Ok(())
}
