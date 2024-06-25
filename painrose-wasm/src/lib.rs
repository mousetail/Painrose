use wasm_bindgen::prelude::*;
use painrose_lib::{language, geometry};

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let code = ":\"Hello World!\"I;
a-e:\"Goodbye All?\"C;";

    let mut program =
        language::LanguageState::<geometry::rhomb::RhombTiling>::new_from_string(code.to_string()).unwrap();

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello!! from Rust!");

    body.append_child(&val)?;

    Ok(())
}
