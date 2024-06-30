use tokio::sync::mpsc;

use painrose_lib::{
    geometry::{self, draw::DrawableTile, rhomb::Tile, tiling::Tiling},
    language::{self, FollowableDirection},
};
use wasm_bindgen::prelude::*;
#[macro_use]
mod macros;
mod layout;
mod renderer;

use layout::Layout;
use web_sys::Window;

const SVG_NAMESPACE: &'static str = "http://www.w3.org/2000/svg";

struct DivOutput<'a> {
    div: &'a web_sys::Element,
}

impl<'a> std::io::Write for DivOutput<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.div
            .append_with_str_1(&String::from_utf8_lossy(buf))
            .map_err(|e| {
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    e.as_string().unwrap_or(format!("Error")),
                )
            })?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

enum Message {
    ProgramStep,
}

fn start_timer(sender: mpsc::Sender<Message>, window: &Window) -> Result<(), JsValue> {
    let callback: Closure<dyn Fn()> = Closure::new(move || {
        sender.blocking_send(Message::ProgramStep).unwrap();
    });

    window.set_interval_with_callback_and_timeout_and_arguments_0(
        callback.as_ref().unchecked_ref(),
        500,
    )?;
    callback.forget();

    return Ok(());
}

#[wasm_bindgen(start)]
async fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let code = ":\"Hello World!\"I;
a-e:\"Goodbye All?\"C;";

    let mut program =
        language::LanguageState::<geometry::rhomb::RhombTiling>::new_from_string(code.to_string())
            .unwrap();

    let layout = Layout::new(&document)?;

    let (sender, mut receiver) = mpsc::channel::<Message>(24);

    start_timer(sender.clone(), &window)?;

    while let Some(message) = receiver.recv().await {
        match message {
            Message::ProgramStep => {
                program.step(&mut layout.get_output(), &mut std::io::Cursor::new("Help!"));
                layout.update(&program)?;
            }
        }
    }

    Ok(())
}
